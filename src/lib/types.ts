// DTO types mirroring sill-core's snapshot/event schema (crates/sill-core).
// The frontend receives render-ready data, never raw bytes or HTML.

export type SessionId = number;

export type Color =
  | { t: "Default" }
  | { t: "DefaultBg" }
  | { t: "Indexed"; v: number }
  | { t: "Rgb"; v: [number, number, number] };

export const FLAG = {
  BOLD: 1 << 0,
  ITALIC: 1 << 1,
  UNDERLINE: 1 << 2,
  INVERSE: 1 << 3,
  DIM: 1 << 4,
  STRIKEOUT: 1 << 5,
  HIDDEN: 1 << 6,
  WIDE: 1 << 7,
} as const;

export interface Run {
  text: string;
  fg: Color;
  bg: Color;
  flags: number;
}

export interface Row {
  runs: Run[];
}

export interface Cursor {
  row: number;
  col: number;
  visible: boolean;
}

// A cell range in viewport coordinates, end-inclusive, pre-clamped to the
// visible grid by Rust (crates/sill-core snapshot::GridRange). Stream
// ranges (is_block = false) span full rows between the endpoints; block
// ranges are rectangles.
export interface GridRange {
  start_row: number;
  start_col: number;
  end_row: number;
  end_col: number;
  is_block: boolean;
}

export interface Snapshot {
  cols: number;
  rows: Row[];
  cursor: Cursor;
  display_offset: number;
  history: number;
  bracketed_paste: boolean;
  selection: GridRange | null;
  search_focus: GridRange | null;
}

// Selection gestures, mirroring sill-core's SelectionMode serde form.
export type SelectionMode = "simple" | "semantic" | "lines" | "block";
// Which half of a cell the pointer is on (drag-direction fidelity).
export type CellSide = "left" | "right";

export type SessionEvent =
  | { kind: "created"; id: SessionId }
  | { kind: "title_changed"; id: SessionId; title: string }
  | { kind: "bell"; id: SessionId }
  | { kind: "exited"; id: SessionId; exit_code: number | null }
  | { kind: "closed"; id: SessionId };

export interface SnapshotPayload {
  id: SessionId;
  snapshot: Snapshot;
}

export interface EventPayload {
  event: SessionEvent;
}
