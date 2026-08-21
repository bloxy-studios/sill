# ADR-0001: Application stack — Tauri 2 + Rust core + web frontend

- **Status:** Accepted
- **Date:** 2026-08-21

## Context

Sill needs: native process/PTY control, cross-platform desktop distribution
(macOS, Linux, Windows), a UI layer productive enough to iterate quickly on a
novel workspace/session model, and a small footprint. One maintainer must be
able to carry the whole stack.

## Decision

- **Rust core** for everything that touches the OS: PTY, processes, sessions,
  future agent detection. Compiled, memory-safe, first-class PTY/terminal
  crates exist in the ecosystem.
- **Tauri 2** as the application shell: windowing, IPC, packaging, updater.
  Uses the OS webview rather than bundling Chromium, keeping binaries tens of
  MB rather than hundreds.
- **React + TypeScript (Vite)** frontend in the webview.
- **Bun** for JS tooling — fast installs, single tool for scripts/tests.

## Alternatives considered

- **Fully native per-platform (SwiftUI/AppKit + GTK + WinUI):** best-feeling
  result, three UIs to build; not viable solo.
- **Electron:** mature, but ships Chromium — heavy for a tool whose thesis
  includes low overhead.
- **Pure-Rust GUI (egui/iced/gpui):** attractive long-term; today it trades
  away UI iteration speed the workspace model needs. The renderer question is
  deliberately kept separate (ADR-0006) so a GPU-accelerated grid renderer
  inside the webview — or a future re-platform of the _shell_ — remains
  possible without discarding the Rust core.
- **Flutter:** weak terminal/text-grid story, another language (Dart).

## Consequences

- ✅ One codebase, three platforms; small artifacts; Rust where correctness matters.
- ⚠️ Webview rendering must be _proven_ fast enough for a terminal grid
  (rAF-batched canvas/WebGL rendering; budgets in [PERFORMANCE.md](../PERFORMANCE.md)).
  This is the stack's main risk and is treated as such, not waved away.
- ⚠️ Webview divergence across platforms (WebKit vs WebView2) needs a
  compatibility test matrix.
- ⚠️ IPC boundary becomes the security perimeter — see
  [security-model](../design/security-model.md).

Revisit trigger: if Phase 1 rendering can't hit budgets on reference hardware,
re-open the renderer/shell question before Phase 2.
