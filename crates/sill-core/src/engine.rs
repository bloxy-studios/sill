//! Terminal emulation state over `alacritty_terminal` (ADR-0006).
//!
//! The engine is a pure state machine: bytes in via [`TermState::feed`],
//! render-ready snapshots out via [`TermState::snapshot`]. No IO, no
//! threads — the session layer owns those. This keeps the parser fuzzable
//! in isolation (threat model T1).

use alacritty_terminal::event::{Event, EventListener};
use alacritty_terminal::grid::{Dimensions, Scroll};
use alacritty_terminal::index::{Boundary, Column, Direction, Point, Side};
use alacritty_terminal::selection::{Selection, SelectionType};
use alacritty_terminal::term::search::{Match, RegexSearch};
use alacritty_terminal::term::{viewport_to_point, Config, Term, TermMode};
use alacritty_terminal::vte::ansi::Processor;

use crate::snapshot::{self, Snapshot};

/// How a selection interprets its anchor points. Mirrors alacritty's
/// `SelectionType` without exposing engine types across the crate boundary
/// (same encapsulation rule as [`crate::snapshot`]).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum SelectionMode {
    /// Plain drag: cell-to-cell stream selection.
    Simple,
    /// Double-click: expand to semantic word boundaries.
    Semantic,
    /// Triple-click: whole lines.
    Lines,
    /// Rectangular (column) selection.
    Block,
}

impl SelectionMode {
    fn to_engine(self) -> SelectionType {
        match self {
            SelectionMode::Simple => SelectionType::Simple,
            SelectionMode::Semantic => SelectionType::Semantic,
            SelectionMode::Lines => SelectionType::Lines,
            SelectionMode::Block => SelectionType::Block,
        }
    }
}

/// Which half of a cell an anchor sits on (drag direction fidelity —
/// selecting left-to-right vs right-to-left includes/excludes edge cells
/// exactly like alacritty).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum CellSide {
    Left,
    Right,
}

impl CellSide {
    fn to_engine(self) -> Side {
        match self {
            CellSide::Left => Side::Left,
            CellSide::Right => Side::Right,
        }
    }
}

/// Escape regex metacharacters so user input searches literally.
/// ~15 lines instead of a regex-syntax dependency
/// (docs/DEPENDENCY_POLICY.md: implement the required 10% ourselves).
fn regex_escape(query: &str) -> String {
    let mut out = String::with_capacity(query.len() + 8);
    for c in query.chars() {
        if matches!(
            c,
            '\\' | '.'
                | '+'
                | '*'
                | '?'
                | '('
                | ')'
                | '|'
                | '['
                | ']'
                | '{'
                | '}'
                | '^'
                | '$'
                | '#'
                | '&'
                | '-'
                | '~'
        ) {
            out.push('\\');
        }
        out.push(c);
    }
    out
}

/// Live scrollback-search state: the compiled query plus the focused match.
/// The focus is a jump target and highlight, not a match list — v1 has no
/// match counter by design (issue #4).
struct ActiveSearch {
    regex: RegexSearch,
    focus: Option<Match>,
}

/// Viewport dimensions handed to the engine.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TermDims {
    pub cols: u16,
    pub rows: u16,
}

impl Dimensions for TermDims {
    fn total_lines(&self) -> usize {
        self.rows as usize
    }

    fn screen_lines(&self) -> usize {
        self.rows as usize
    }

    fn columns(&self) -> usize {
        self.cols as usize
    }
}

/// Engine-originated events surfaced to the session layer.
#[derive(Debug, Clone)]
pub enum EngineEvent {
    /// OSC title change from the running program.
    Title(String),
    /// Title reset to default.
    ResetTitle,
    /// BEL / bell request.
    Bell,
    /// The emulation produced bytes that must be written back to the PTY
    /// (device status reports, identification responses, …).
    PtyWrite(String),
    /// Grid content changed; a redraw is needed.
    Wakeup,
}

/// Listener handed to `alacritty_terminal`; forwards the events Sill cares
/// about into a **bounded** session-owned queue. Everything is try_send:
/// spammable events (bell/title/wakeup) drop under pressure by design, and
/// even response write-backs drop once the queue fills — see the PtyWrite
/// arm for why a blocking send there is a session-freezing deadlock.
/// Clipboard *reads* (OSC 52 query) and color queries are intentionally
/// not answered in Phase 2 — deny-by-default per threat model T2.
pub(crate) struct EventProxy {
    tx: std::sync::mpsc::SyncSender<EngineEvent>,
}

impl EventProxy {
    pub(crate) fn new(tx: std::sync::mpsc::SyncSender<EngineEvent>) -> Self {
        Self { tx }
    }
}

impl EventListener for EventProxy {
    fn send_event(&self, event: Event) {
        match event {
            // Response write-backs are best-effort WITH a drop bound: a
            // hostile child can flood query requests (ESC[6n …) without
            // ever reading the replies, filling the PTY buffer, blocking
            // the dispatcher's write, filling this queue — and a blocking
            // send here would then wedge the reader WHILE IT HOLDS THE
            // ENGINE LOCK, freezing the whole session. Dropping replies to
            // a child that isn't reading them is the correct defense
            // (threat model T1: cap/validate query responses).
            Event::PtyWrite(text) => {
                let _ = self.tx.try_send(EngineEvent::PtyWrite(text));
            }
            // Droppable under pressure: losing a bell/title/wakeup burst is
            // the designed behavior, not a bug.
            Event::Title(title) => {
                let _ = self.tx.try_send(EngineEvent::Title(title));
            }
            Event::ResetTitle => {
                let _ = self.tx.try_send(EngineEvent::ResetTitle);
            }
            Event::Bell => {
                let _ = self.tx.try_send(EngineEvent::Bell);
            }
            Event::Wakeup => {
                let _ = self.tx.try_send(EngineEvent::Wakeup);
            }
            // ClipboardStore/ClipboardLoad/ColorRequest/…: deliberately
            // dropped for now (threat model T2: clipboard via escape
            // sequences is deny-by-default until the policy surface exists).
            _ => {}
        }
    }
}

/// Terminal emulation state for one session.
pub struct TermState {
    term: Term<EventProxy>,
    parser: Processor,
    dims: TermDims,
    search: Option<ActiveSearch>,
}

impl TermState {
    pub fn new(
        dims: TermDims,
        scrollback_lines: usize,
        events: std::sync::mpsc::SyncSender<EngineEvent>,
    ) -> Self {
        let config = Config {
            scrolling_history: scrollback_lines,
            ..Config::default()
        };
        let term = Term::new(config, &dims, EventProxy::new(events));
        Self {
            term,
            parser: Processor::new(),
            dims,
            search: None,
        }
    }

    /// Feed raw PTY output bytes into the emulation.
    pub fn feed(&mut self, bytes: &[u8]) {
        self.parser.advance(&mut self.term, bytes);
    }

    /// Resize the viewport.
    pub fn resize(&mut self, dims: TermDims) {
        if dims != self.dims {
            self.dims = dims;
            // Reflow rewrites absolute grid coordinates, so a stored match
            // range would point at the wrong cells. Drop the focus (the
            // compiled query survives; the next step re-searches).
            // `Term::resize` invalidates/re-anchors the selection itself.
            if let Some(active) = self.search.as_mut() {
                active.focus = None;
            }
            self.term.resize(dims);
        }
    }

    /// Scroll the *display* (viewport into scrollback); emulation state is
    /// unaffected. Positive = toward history.
    pub fn scroll_display(&mut self, delta_lines: i32) {
        self.term
            .grid_mut()
            .scroll_display(Scroll::Delta(delta_lines));
    }

    /// Jump the viewport back to the live screen bottom.
    pub fn scroll_to_bottom(&mut self) {
        self.term.grid_mut().scroll_display(Scroll::Bottom);
    }

    // NOTE: alacritty's per-line damage API always reports the cursor line
    // as damaged (renderers must repaint it), so it cannot serve as a
    // "content changed?" boolean. Phase 2 dirty-tracking is therefore
    // edge-triggered at the session layer (one notification per read burst),
    // and full visible snapshots are shipped on notify. Per-region delta
    // transport is the documented follow-up (docs/design/performance.md)
    // and will consume `Term::damage()` directly with cursor-aware logic.

    /// Render-ready snapshot of the visible viewport.
    pub fn snapshot(&self) -> Snapshot {
        let focus = self.search.as_ref().and_then(|s| s.focus.as_ref());
        snapshot::from_term(&self.term, self.dims, focus)
    }

    pub fn dims(&self) -> TermDims {
        self.dims
    }

    /// Convert a viewport cell (row 0 = top of the *displayed* region,
    /// which honors scrollback offset) into an absolute grid point. Inputs
    /// are clamped to the viewport — boundary validation lives here, not in
    /// the frontend (docs/design/ipc.md rule 2).
    fn viewport_point(&self, row: usize, col: usize) -> Point {
        let row = row.min(self.dims.rows.saturating_sub(1) as usize);
        let col = col.min(self.dims.cols.saturating_sub(1) as usize);
        let display_offset = self.term.grid().display_offset();
        viewport_to_point(display_offset, Point::new(row, Column(col)))
    }

    /// Begin a selection at a viewport cell. Replaces any existing
    /// selection (a plain click therefore clears: an un-dragged Simple
    /// selection has no extent and yields no range).
    pub fn selection_start(&mut self, mode: SelectionMode, row: usize, col: usize, side: CellSide) {
        let point = self.viewport_point(row, col);
        self.term.selection = Some(Selection::new(mode.to_engine(), point, side.to_engine()));
    }

    /// Extend the active selection to a viewport cell. No-op without one.
    pub fn selection_update(&mut self, row: usize, col: usize, side: CellSide) {
        let point = self.viewport_point(row, col);
        if let Some(selection) = self.term.selection.as_mut() {
            selection.update(point, side.to_engine());
        }
    }

    /// Drop the selection; reports whether one existed (callers use this to
    /// skip redraw notifications when nothing changed).
    pub fn selection_clear(&mut self) -> bool {
        self.term.selection.take().is_some()
    }

    /// Extract the selected text (grid truth, headless-testable). `None`
    /// when there is no selection or it has no extent.
    pub fn selection_text(&self) -> Option<String> {
        self.term
            .selection_to_string()
            .filter(|text| !text.is_empty())
    }

    /// Start (or restart) a scrollback search. The query is LITERAL — user
    /// input is regex-escaped (regex syntax is a later, explicit opt-in) —
    /// and always case-insensitive: the engine's smart-case default would
    /// turn case-sensitive on any uppercase query, so `(?i)` pins it.
    /// Searches upward (into history) from the live screen bottom; jumps
    /// the viewport to the match. Returns whether a match was found.
    pub fn search_start(&mut self, query: &str) -> bool {
        self.search = None;
        if query.is_empty() {
            return false;
        }
        let pattern = format!("(?i){}", regex_escape(query));
        let mut regex = match RegexSearch::new(&pattern) {
            Ok(regex) => regex,
            // Escaped literals should always compile; treat a failure as
            // not-found rather than poisoning the session.
            Err(_) => return false,
        };
        let origin = Point::new(self.term.bottommost_line(), self.term.last_column());
        let focus = self
            .term
            .search_next(&mut regex, origin, Direction::Left, Side::Right, None);
        let found = focus.is_some();
        if let Some(m) = &focus {
            self.jump_to_match(m);
        }
        self.search = Some(ActiveSearch { regex, focus });
        found
    }

    /// Step to the adjacent match: `forward = false` → older (into
    /// history), `forward = true` → newer. Wraps at scrollback boundaries
    /// (the engine's `search_next` falls back to the first match, and the
    /// one-cell origin step uses `Boundary::None`, which wraps too).
    /// Returns whether a match is focused afterwards.
    pub fn search_step(&mut self, forward: bool) -> bool {
        let Some(mut active) = self.search.take() else {
            return false;
        };
        let (origin, direction, side) = match (&active.focus, forward) {
            // Next older: continue leftward from just before the focus.
            (Some(m), false) => (
                m.start().sub(&self.term, Boundary::None, 1),
                Direction::Left,
                Side::Right,
            ),
            // Next newer: continue rightward from just after the focus.
            (Some(m), true) => (
                m.end().add(&self.term, Boundary::None, 1),
                Direction::Right,
                Side::Left,
            ),
            // Nothing focused (e.g. content appeared after a miss): retry
            // the initial upward-from-bottom search.
            (None, _) => (
                Point::new(self.term.bottommost_line(), self.term.last_column()),
                Direction::Left,
                Side::Right,
            ),
        };
        let focus = self
            .term
            .search_next(&mut active.regex, origin, direction, side, None);
        let found = focus.is_some();
        if let Some(m) = &focus {
            self.jump_to_match(m);
        }
        active.focus = focus;
        self.search = Some(active);
        found
    }

    /// End the search: drops the query and the focused-match highlight.
    /// The viewport stays where it is. Reports whether state existed.
    pub fn search_clear(&mut self) -> bool {
        self.search.take().is_some()
    }

    /// Scroll the display so the match is visible. Already-visible matches
    /// cause no movement; otherwise the match line is centered (clamped to
    /// available history).
    fn jump_to_match(&mut self, m: &Match) {
        let grid = self.term.grid();
        let display_offset = grid.display_offset() as i32;
        let rows = self.dims.rows as i32;
        let line = m.start().line.0;
        let viewport_row = line + display_offset;
        if viewport_row >= 0 && viewport_row < rows {
            return;
        }
        let history = (grid.total_lines() - grid.screen_lines()) as i32;
        let target = (rows / 2 - line).clamp(0, history);
        self.term
            .grid_mut()
            .scroll_display(Scroll::Delta(target - display_offset));
    }

    /// True when the program enabled bracketed paste — the frontend wraps
    /// pastes in ESC[200~ / ESC[201~ accordingly (threat model T3).
    pub fn bracketed_paste(&self) -> bool {
        self.term.mode().contains(TermMode::BRACKETED_PASTE)
    }

    /// Test-only access to the underlying term (grid inspection).
    #[cfg(test)]
    pub(crate) fn term(&self) -> &Term<EventProxy> {
        &self.term
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc;

    fn state(cols: u16, rows: u16) -> (TermState, mpsc::Receiver<EngineEvent>) {
        let (tx, rx) = mpsc::sync_channel(64);
        (TermState::new(TermDims { cols, rows }, 1000, tx), rx)
    }

    fn visible_text(s: &TermState) -> String {
        let snap = s.snapshot();
        snap.rows
            .iter()
            .map(|r| {
                r.runs
                    .iter()
                    .map(|run| run.text.as_str())
                    .collect::<String>()
                    .trim_end()
                    .to_string()
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    #[test]
    fn plain_text_lands_in_grid() {
        let (mut s, _rx) = state(20, 4);
        s.feed(b"hello");
        assert!(visible_text(&s).starts_with("hello"));
    }

    #[test]
    fn sgr_color_is_captured() {
        let (mut s, _rx) = state(20, 4);
        s.feed(b"\x1b[31mred\x1b[0m");
        let snap = s.snapshot();
        let first = &snap.rows[0].runs[0];
        assert_eq!(first.text, "red");
        assert_eq!(first.fg, crate::snapshot::Color::Indexed(1));
    }

    #[test]
    fn title_event_is_forwarded() {
        let (mut s, rx) = state(20, 4);
        s.feed(b"\x1b]0;my-title\x07");
        let got = rx.try_iter().any(|e| {
            matches!(
                e,
                EngineEvent::Title(ref t) if t == "my-title"
            )
        });
        assert!(got, "expected a Title event");
    }

    #[test]
    fn alt_screen_roundtrip_restores_primary_content() {
        let (mut s, _rx) = state(20, 4);
        s.feed(b"primary");
        // Enter alternate screen (as vim/htop do), draw, then leave.
        s.feed(b"\x1b[?1049h");
        s.feed(b"alt-content");
        assert!(visible_text(&s).contains("alt-content"));
        s.feed(b"\x1b[?1049l");
        let restored = visible_text(&s);
        assert!(
            restored.contains("primary"),
            "primary screen restored: {restored}"
        );
        assert!(!restored.contains("alt-content"));
    }

    #[test]
    fn resize_changes_dimensions() {
        let (mut s, _rx) = state(20, 4);
        s.resize(TermDims { cols: 40, rows: 10 });
        let snap = s.snapshot();
        assert_eq!(snap.cols, 40);
        assert_eq!(snap.rows.len(), 10);
    }

    #[test]
    fn bracketed_paste_mode_is_reported() {
        let (mut s, _rx) = state(20, 4);
        assert!(!s.bracketed_paste());
        s.feed(b"\x1b[?2004h");
        assert!(s.bracketed_paste());
    }

    #[test]
    fn bell_flood_never_blocks_the_parser() {
        // Tiny queue, no consumer: a bell flood must complete promptly by
        // dropping events, not deadlock the feed path.
        let (tx, rx) = mpsc::sync_channel(4);
        let mut s = TermState::new(TermDims { cols: 20, rows: 4 }, 100, tx);
        let start = std::time::Instant::now();
        for _ in 0..10_000 {
            s.feed(b"\x07");
        }
        assert!(
            start.elapsed() < std::time::Duration::from_secs(5),
            "bell flood blocked the parser"
        );
        // Queue holds at most its capacity.
        assert!(rx.try_iter().count() <= 4);
    }

    #[test]
    fn scrollback_is_bounded_by_config() {
        let (tx, _rx) = mpsc::sync_channel(64);
        let mut s = TermState::new(TermDims { cols: 10, rows: 4 }, 100, tx);
        let mut input = String::new();
        for i in 0..1000 {
            input.push_str(&format!("line{i}\r\n"));
        }
        s.feed(input.as_bytes());
        use alacritty_terminal::grid::Dimensions as _;
        let total = s.term().grid().total_lines();
        // total = history (capped) + screen lines.
        assert!(total <= 100 + 4, "scrollback exceeded cap: {total}");
    }

    #[test]
    fn selection_extracts_exact_text() {
        let (mut s, _rx) = state(20, 4);
        s.feed(b"hello world");
        s.selection_start(SelectionMode::Simple, 0, 0, CellSide::Left);
        s.selection_update(0, 4, CellSide::Right);
        assert_eq!(s.selection_text().as_deref(), Some("hello"));
    }

    #[test]
    fn semantic_selection_snaps_to_word() {
        let (mut s, _rx) = state(20, 4);
        s.feed(b"foo bar-baz qux");
        // Single point inside "bar-baz" ('-' is not a semantic separator).
        s.selection_start(SelectionMode::Semantic, 0, 6, CellSide::Left);
        assert_eq!(s.selection_text().as_deref(), Some("bar-baz"));
    }

    #[test]
    fn click_without_drag_selects_nothing() {
        let (mut s, _rx) = state(20, 4);
        s.feed(b"hello");
        s.selection_start(SelectionMode::Simple, 0, 2, CellSide::Left);
        assert_eq!(s.selection_text(), None);
        assert!(s.snapshot().selection.is_none());
    }

    #[test]
    fn selection_range_clamps_to_viewport() {
        let (mut s, _rx) = state(10, 4);
        let mut input = String::new();
        for i in 0..20 {
            input.push_str(&format!("l{i}\r\n"));
        }
        s.feed(input.as_bytes());
        // Select the whole visible screen, then scroll 2 lines back: the
        // selection now sticks out below the viewport and must clamp to
        // the last visible row at full width.
        s.selection_start(SelectionMode::Simple, 0, 0, CellSide::Left);
        s.selection_update(3, 9, CellSide::Right);
        s.scroll_display(2);
        let sel = s.snapshot().selection.expect("selection visible");
        assert_eq!((sel.start_row, sel.start_col), (2, 0));
        assert_eq!((sel.end_row, sel.end_col), (3, 9));
        assert!(!sel.is_block);
    }

    #[test]
    fn search_is_literal_and_case_insensitive() {
        let (mut s, _rx) = state(20, 4);
        s.feed(b"A.B  axb");
        // Searching "a.b": as a literal (case-insensitive) it matches only
        // "A.B" at columns 0..=2. If the dot leaked through as a regex
        // wildcard, the upward-from-bottom search would find "axb" (the
        // rightmost candidate) first instead.
        assert!(s.search_start("a.b"));
        let focus = s.snapshot().search_focus.expect("focused match");
        assert_eq!((focus.start_row, focus.start_col), (0, 0));
        assert_eq!((focus.end_row, focus.end_col), (0, 2));

        assert!(!s.search_start("q.z"));
        assert!(s.snapshot().search_focus.is_none());
    }

    #[test]
    fn search_jumps_into_scrollback_and_wraps() {
        let (mut s, _rx) = state(10, 4);
        let mut input = String::new();
        for i in 0..30 {
            input.push_str(&format!("line-{i}\r\n"));
        }
        s.feed(input.as_bytes());

        assert!(s.search_start("line-3"));
        let snap = s.snapshot();
        assert!(snap.display_offset > 0, "viewport jumped into history");
        assert!(snap.search_focus.is_some());
        assert!(
            visible_text(&s).contains("line-3"),
            "match line visible after jump"
        );

        // Single match: stepping in either direction wraps back to it.
        assert!(s.search_step(false));
        assert!(s.search_step(true));
    }

    #[test]
    fn resize_drops_search_focus() {
        let (mut s, _rx) = state(20, 4);
        s.feed(b"needle");
        assert!(s.search_start("needle"));
        assert!(s.snapshot().search_focus.is_some());
        // Reflow invalidates absolute match coordinates.
        s.resize(TermDims { cols: 30, rows: 6 });
        assert!(s.snapshot().search_focus.is_none());
    }
}
