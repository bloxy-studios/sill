//! Render-ready grid snapshots — the DTOs that cross the IPC boundary.
//!
//! The frontend receives *data to draw*, never raw bytes or HTML
//! (docs/design/ipc.md rule 4; threat model T4). Cells are merged into
//! style runs so a typical prompt row serializes to a handful of runs
//! rather than N cells.

use alacritty_terminal::index::Point;
use alacritty_terminal::term::cell::Flags;
use alacritty_terminal::term::search::Match;
use alacritty_terminal::term::{Term, TermMode};
use alacritty_terminal::vte::ansi::{Color as AnsiColor, NamedColor};

use crate::engine::{EventProxy, TermDims};

/// Color in snapshot form. `Default`/`DefaultBg` defer to the frontend
/// theme; `Indexed` is the 256-color palette; `Rgb` is truecolor.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(tag = "t", content = "v"))]
pub enum Color {
    Default,
    DefaultBg,
    Indexed(u8),
    Rgb(u8, u8, u8),
}

/// Cell style flags, mirrored as a compact bitfield.
pub mod flag {
    pub const BOLD: u16 = 1 << 0;
    pub const ITALIC: u16 = 1 << 1;
    pub const UNDERLINE: u16 = 1 << 2;
    pub const INVERSE: u16 = 1 << 3;
    pub const DIM: u16 = 1 << 4;
    pub const STRIKEOUT: u16 = 1 << 5;
    pub const HIDDEN: u16 = 1 << 6;
    pub const WIDE: u16 = 1 << 7;
}

/// A run of consecutive cells sharing one style.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Run {
    pub text: String,
    pub fg: Color,
    pub bg: Color,
    pub flags: u16,
}

#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Row {
    pub runs: Vec<Run>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Cursor {
    pub row: i32,
    pub col: u16,
    pub visible: bool,
}

/// A cell range in *viewport* coordinates (row 0 = top of the displayed
/// region), end-inclusive. Ranges are pre-clamped to the viewport: a
/// selection extending into off-screen history arrives already trimmed,
/// so the renderer never needs grid coordinates (docs/design/ipc.md).
///
/// `is_block = false`: a text-stream range — `start_col` applies on the
/// first row, `end_col` on the last, and rows between span the full width.
/// `is_block = true`: a rectangle between the two corners.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct GridRange {
    pub start_row: u16,
    pub start_col: u16,
    pub end_row: u16,
    pub end_col: u16,
    pub is_block: bool,
}

/// Clamp an absolute grid range to the visible viewport; `None` when it is
/// entirely off-screen. Stream ranges that continue past a viewport edge
/// extend to that row's full width, matching what the user actually sees.
fn viewport_range(
    start: Point,
    end: Point,
    is_block: bool,
    display_offset: usize,
    dims: TermDims,
) -> Option<GridRange> {
    let offset = display_offset as i32;
    let rows = dims.rows as i32;
    let last_col = dims.cols.saturating_sub(1);

    let mut start_row = start.line.0 + offset;
    let mut end_row = end.line.0 + offset;
    if end_row < 0 || start_row >= rows {
        return None;
    }

    let mut start_col = (start.column.0 as u16).min(last_col);
    let mut end_col = (end.column.0 as u16).min(last_col);
    if start_row < 0 {
        start_row = 0;
        if !is_block {
            start_col = 0;
        }
    }
    if end_row >= rows {
        end_row = rows - 1;
        if !is_block {
            end_col = last_col;
        }
    }

    Some(GridRange {
        start_row: start_row as u16,
        start_col,
        end_row: end_row as u16,
        end_col,
        is_block,
    })
}

/// Full visible-viewport snapshot.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Snapshot {
    pub cols: u16,
    pub rows: Vec<Row>,
    pub cursor: Cursor,
    /// Lines of history above the current viewport position.
    pub display_offset: usize,
    /// Total scrollback lines available.
    pub history: usize,
    /// Program requested bracketed paste (frontend wraps pastes).
    pub bracketed_paste: bool,
    /// Visible portion of the active selection, if any.
    pub selection: Option<GridRange>,
    /// Visible portion of the focused search match, if any.
    pub search_focus: Option<GridRange>,
}

fn map_color(c: AnsiColor) -> Color {
    match c {
        AnsiColor::Named(named) => match named {
            NamedColor::Foreground | NamedColor::Cursor => Color::Default,
            NamedColor::Background => Color::DefaultBg,
            NamedColor::Black => Color::Indexed(0),
            NamedColor::Red => Color::Indexed(1),
            NamedColor::Green => Color::Indexed(2),
            NamedColor::Yellow => Color::Indexed(3),
            NamedColor::Blue => Color::Indexed(4),
            NamedColor::Magenta => Color::Indexed(5),
            NamedColor::Cyan => Color::Indexed(6),
            NamedColor::White => Color::Indexed(7),
            NamedColor::BrightBlack => Color::Indexed(8),
            NamedColor::BrightRed => Color::Indexed(9),
            NamedColor::BrightGreen => Color::Indexed(10),
            NamedColor::BrightYellow => Color::Indexed(11),
            NamedColor::BrightBlue => Color::Indexed(12),
            NamedColor::BrightMagenta => Color::Indexed(13),
            NamedColor::BrightCyan => Color::Indexed(14),
            NamedColor::BrightWhite => Color::Indexed(15),
            NamedColor::DimBlack => Color::Indexed(0),
            NamedColor::DimRed => Color::Indexed(1),
            NamedColor::DimGreen => Color::Indexed(2),
            NamedColor::DimYellow => Color::Indexed(3),
            NamedColor::DimBlue => Color::Indexed(4),
            NamedColor::DimMagenta => Color::Indexed(5),
            NamedColor::DimCyan => Color::Indexed(6),
            NamedColor::DimWhite => Color::Indexed(7),
            NamedColor::BrightForeground | NamedColor::DimForeground => Color::Default,
        },
        AnsiColor::Indexed(i) => Color::Indexed(i),
        AnsiColor::Spec(rgb) => Color::Rgb(rgb.r, rgb.g, rgb.b),
    }
}

fn map_flags(f: Flags) -> u16 {
    let mut out = 0;
    if f.contains(Flags::BOLD) {
        out |= flag::BOLD;
    }
    if f.contains(Flags::ITALIC) {
        out |= flag::ITALIC;
    }
    if f.contains(Flags::UNDERLINE)
        || f.contains(Flags::DOUBLE_UNDERLINE)
        || f.contains(Flags::UNDERCURL)
        || f.contains(Flags::DOTTED_UNDERLINE)
        || f.contains(Flags::DASHED_UNDERLINE)
    {
        out |= flag::UNDERLINE;
    }
    if f.contains(Flags::INVERSE) {
        out |= flag::INVERSE;
    }
    if f.contains(Flags::DIM) {
        out |= flag::DIM;
    }
    if f.contains(Flags::STRIKEOUT) {
        out |= flag::STRIKEOUT;
    }
    if f.contains(Flags::HIDDEN) {
        out |= flag::HIDDEN;
    }
    if f.contains(Flags::WIDE_CHAR) {
        out |= flag::WIDE;
    }
    out
}

/// Build a snapshot of the currently displayed viewport (honors the
/// display offset into scrollback).
pub(crate) fn from_term(
    term: &Term<EventProxy>,
    dims: TermDims,
    search_focus: Option<&Match>,
) -> Snapshot {
    use alacritty_terminal::grid::Dimensions as _;

    let grid = term.grid();
    let display_offset = grid.display_offset();
    let history = grid.total_lines().saturating_sub(grid.screen_lines());

    let selection = term
        .selection
        .as_ref()
        .and_then(|selection| selection.to_range(term))
        .and_then(|range| {
            viewport_range(range.start, range.end, range.is_block, display_offset, dims)
        });
    let search_focus = search_focus
        .and_then(|m| viewport_range(*m.start(), *m.end(), false, display_offset, dims));

    let mut rows: Vec<Row> = Vec::with_capacity(dims.rows as usize);
    let mut current = Row::default();
    let mut last_line: Option<i32> = None;

    for indexed in grid.display_iter() {
        let line = indexed.point.line.0;
        if last_line != Some(line) {
            if last_line.is_some() {
                rows.push(std::mem::take(&mut current));
            }
            last_line = Some(line);
        }

        let cell = &*indexed;
        // Wide-char spacers carry no glyph; the WIDE flag on the previous
        // cell tells the renderer to advance two columns.
        if cell.flags.contains(Flags::WIDE_CHAR_SPACER)
            || cell.flags.contains(Flags::LEADING_WIDE_CHAR_SPACER)
        {
            continue;
        }

        let fg = map_color(cell.fg);
        let bg = map_color(cell.bg);
        let flags = map_flags(cell.flags);

        match current.runs.last_mut() {
            Some(run) if run.fg == fg && run.bg == bg && run.flags == flags => {
                run.text.push(cell.c);
            }
            _ => current.runs.push(Run {
                text: cell.c.to_string(),
                fg,
                bg,
                flags,
            }),
        }
    }
    if last_line.is_some() {
        rows.push(current);
    }

    let cursor_point = grid.cursor.point;
    let show_cursor = term.mode().contains(TermMode::SHOW_CURSOR);

    Snapshot {
        cols: dims.cols,
        rows,
        cursor: Cursor {
            row: cursor_point.line.0,
            col: cursor_point.column.0 as u16,
            // Cursor is drawn only on the live screen (offset 0).
            visible: show_cursor && display_offset == 0,
        },
        display_offset,
        history,
        bracketed_paste: term.mode().contains(TermMode::BRACKETED_PASTE),
        selection,
        search_focus,
    }
}
