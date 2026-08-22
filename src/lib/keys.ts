// KeyboardEvent → terminal input bytes (as a JS string; the Rust side
// writes UTF-8). Phase 2 scope: normal-mode encoding; application cursor
// mode and the kitty keyboard protocol are Phase 3 items and deliberately
// not faked.

const CSI = "\x1b[";

const SPECIAL: Record<string, string> = {
  Enter: "\r",
  Backspace: "\x7f",
  Tab: "\t",
  Escape: "\x1b",
  ArrowUp: `${CSI}A`,
  ArrowDown: `${CSI}B`,
  ArrowRight: `${CSI}C`,
  ArrowLeft: `${CSI}D`,
  Home: `${CSI}H`,
  End: `${CSI}F`,
  Delete: `${CSI}3~`,
  Insert: `${CSI}2~`,
  PageUp: `${CSI}5~`,
  PageDown: `${CSI}6~`,
};

/**
 * Encode a keydown into PTY input, or null when the event isn't terminal
 * input (app shortcuts with Cmd, bare modifier presses, …).
 */
export function encodeKey(e: KeyboardEvent): string | null {
  // Cmd (macOS) / Win key: application chrome territory, never sent to the
  // shell. Copy/paste shortcuts are handled by dedicated listeners.
  if (e.metaKey) return null;

  const special = SPECIAL[e.key];
  if (special !== undefined) {
    // Shift+Tab → backtab.
    if (e.key === "Tab" && e.shiftKey) return `${CSI}Z`;
    return special;
  }

  if (e.ctrlKey) {
    if (e.key.length === 1) {
      const ch = e.key.toLowerCase();
      const code = ch.charCodeAt(0);
      // Ctrl+A..Z → 0x01..0x1a (Ctrl+C = SIGINT path, etc.)
      if (code >= 97 && code <= 122) return String.fromCharCode(code - 96);
      if (ch === "[") return "\x1b";
      if (ch === "\\") return "\x1c";
      if (ch === "]") return "\x1d";
      if (ch === " ") return "\x00";
    }
    return null;
  }

  // Printable characters (includes Alt/Option-composed input on macOS,
  // which arrives already composed in e.key).
  if (e.key.length === 1) return e.key;

  return null;
}

// Paste normalization and bracketed-paste wrapping live in Rust
// (sill-core session::paste) — protocol state is never cached out here.
