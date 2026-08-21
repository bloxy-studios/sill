# Changelog

All notable changes to Sill will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

Sill has not yet published a release. Entries below describe changes on `main`
since the repository was created.

## [Unreleased]

### Added

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
