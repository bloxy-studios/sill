# Changelog

All notable changes to Sill will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

Sill has not yet published a release. Entries below describe changes on `main`
since the repository was created.

## [Unreleased]

### Added

- Text selection with copy: mouse drag, double-click (word), triple-click
  (line), and Alt+drag (block) selections, rendered as canvas highlights.
  Selection state lives in Rust; copy (Cmd+C on macOS, Ctrl+Shift+C
  everywhere; plain Ctrl+C remains SIGINT) extracts in `sill-core` and
  writes the OS clipboard via `arboard`. Typing or pasting clears the
  selection.
- Scrollback search (Cmd/Ctrl+F): literal, case-insensitive queries over
  the full scrollback; Enter walks older matches, Shift+Enter newer, with
  wraparound at buffer boundaries; the viewport jumps to the focused
  match. Misses show a not-found state (no match counter yet).
- Terminal foundation (Phase 2): `sill-core` crate — PTY sessions
  (portable-pty), terminal emulation (alacritty_terminal, chosen via
  measured spike — ADR-0006), typed session events, capped scrollback
  (default 10,000 lines, never unbounded) — plus Tauri IPC commands with
  frame-coalesced snapshot events and a canvas grid renderer with keyboard
  input, bracketed-paste-aware paste, resize, and wheel scrollback.
- Engine benchmark harness with recorded results
  (`benchmarks/engine-spike/`).
- Open-source project foundation: licensing (MIT OR Apache-2.0), contribution
  guidelines, code of conduct, security policy, governance, roadmap,
  architecture and design documentation, CI, dependency management, and
  release infrastructure.
- Initial Rust/TypeScript project scaffold (Tauri 2 + React + Vite + Bun).

### Removed

- Template `greet` demo command and placeholder UI.

### Changed

- Application identifier set to `com.bloxy-studios.sill`.
- A restrictive Content Security Policy replaces the template default (`null`).
