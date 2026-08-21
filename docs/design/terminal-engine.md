# Design: Terminal Engine

**Status: PROPOSED — nothing implemented.** Decision tracked in
[ADR-0006](../decisions/0006-terminal-engine.md). This document records the
design space and current crate facts (verified 2026-08-21) so the Phase 1
evaluation spike starts informed.

## Requirements

1. VT/xterm-compatible emulation correct enough for vim, tmux, htop, fzf, and
   agent TUIs (Claude Code, Codex CLI render heavy TUI output)
2. Embeddable as a library: Sill owns the event loop, rendering, and IPC
3. Damage tracking / dirty-region output — the renderer must know _what
   changed_, not re-read the world
4. Bounded scrollback with fast access
5. Hostile-input robustness (see threat model T1) — parser fuzzing must be
   possible
6. License: MIT/Apache-compatible (rules out GPL engines)

## The two stacks + one lightweight path (Rust, 2026)

| Stack            | Crates                                                 | Facts (2026-08)                                                                                                                                              |
| ---------------- | ------------------------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Alacritty stack  | `vte` (parser) + `alacritty_terminal` (grid/state)     | Most recently released emulation core (`alacritty_terminal` 0.26.0, 2026-04); Apache-2.0; deliberately rendering-agnostic — attractive for webview rendering |
| WezTerm stack    | `termwiz` (parse+surface model) + `portable-pty` (PTY) | Heaviest real-world usage (`termwiz` ~9.4M dl/90d); slow release cadence but actively developed parent repo; MIT                                             |
| Lightweight path | `vt100` (+ `pty-process`)                              | Simpler diffing API, less battle-tested against exotic sequences; maintained, low activity                                                                   |

PTY layer note: `portable-pty` (wraps ConPTY on Windows — the hard part) is
the de-facto default and can be adopted _independently_ of the emulation
choice. `vte` underpins much of the ecosystem (69M lifetime downloads) if
Sill ever needs to own its grid model.

## Evaluation plan (the Phase 1 spike)

Per candidate: embed behind a common `TerminalEngine` trait; run a correctness
battery (vttest subset + real-program screenshots); flood-output benchmark;
measure state-snapshot/diff ergonomics for IPC transport; review scrollback
memory model. Output: filled-in comparison table + ADR-0006 moved to Accepted.

## Interface sketch (PROPOSED)

```
trait TerminalEngine {
    fn feed(&mut self, bytes: &[u8]);          // PTY output in
    fn resize(&mut self, cols: u16, rows: u16);
    fn take_damage(&mut self) -> Damage;        // dirty regions since last take
    fn snapshot(&self, viewport: Range) -> GridSlice; // render-ready cells
    fn scrollback_len(&self) -> usize;
}
```

Design intent: the engine is a pure state machine (bytes in, grid out) so it
can be fuzzed in isolation and swapped if the decision proves wrong.
