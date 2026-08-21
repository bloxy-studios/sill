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

export interface Snapshot {
  cols: number;
  rows: Row[];
  cursor: Cursor;
  display_offset: number;
  history: number;
  bracketed_paste: boolean;
}

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
