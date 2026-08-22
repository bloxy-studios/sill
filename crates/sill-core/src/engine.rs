//! Terminal emulation state over `alacritty_terminal` (ADR-0006).
//!
//! The engine is a pure state machine: bytes in via [`TermState::feed`],
//! render-ready snapshots out via [`TermState::snapshot`]. No IO, no
//! threads — the session layer owns those. This keeps the parser fuzzable
//! in isolation (threat model T1).

use alacritty_terminal::event::{Event, EventListener};
use alacritty_terminal::grid::{Dimensions, Scroll};
use alacritty_terminal::term::{Config, Term, TermMode};
use alacritty_terminal::vte::ansi::Processor;

use crate::snapshot::{self, Snapshot};

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
        snapshot::from_term(&self.term, self.dims)
    }

    pub fn dims(&self) -> TermDims {
        self.dims
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
}
