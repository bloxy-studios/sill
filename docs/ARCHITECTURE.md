# Architecture

This document describes Sill's architecture in two honest layers: **what
exists today** (very little — the project is pre-alpha) and **the target
architecture** the roadmap builds toward (marked PROPOSED). Design detail per
subsystem lives in [docs/design/](design/).

## What exists today (2026-08)

Phase 2 (minimal terminal) is implemented:

- `crates/sill-core/` — the webview-free terminal core (tested headless):
  - `shell`: zsh-first default-shell resolution with honest fallbacks
  - `engine`: emulation state over `alacritty_terminal` (ADR-0006, measured);
    OSC title/bell events, PTY write-backs (DSR/DA), bracketed-paste mode
  - `snapshot`: render-ready style-run DTOs (data across IPC, never bytes)
  - `session`: PTY spawn via `portable-pty`, per-session reader/waiter
    threads, dispatcher answering emulation write-backs, edge-triggered
    dirty notifications, scrollback capped (default 10k, hard cap 200k)
- `src-tauri/` — typed IPC commands (`create_session`, `session_input`,
  `session_resize`, `session_scroll*`, `session_snapshot`, `session_kill`,
  `session_close`, `list_sessions`) + two pump threads: frame-coalesced
  snapshot emits (16ms window) and typed session events. Capabilities stay
  `core:default` + `opener:default`; CSP strict.
- `src/` — canvas grid renderer (imperative draws, zero per-frame React),
  keyboard encoding, bracketed-paste-aware paste, resize → PTY, wheel
  scrollback, exit overlay.

Not yet implemented (design intent below): selection/copy, search, shell
integration/semantic zones, workspaces/tabs/panes, agent awareness, CLI,
MCP. Phase 2's GUI leg still needs verification on real display hardware.

## Target architecture — PROPOSED

```
┌─────────────────────────────────────────────┐
│ Frontend (webview: React + TypeScript)      │
│   grid renderer · workspace UI · input      │
└──────────────────┬──────────────────────────┘
                   │ Tauri IPC (commands + events)
                   │ == the security boundary ==
┌──────────────────▼──────────────────────────┐
│ Rust core (src-tauri)                       │
│  ├─ PTY layer          spawn/resize/kill    │
│  ├─ Terminal state     emulation, grid,     │
│  │                     scrollback, damage   │
│  ├─ Session manager    session lifecycle,   │
│  │                     persistence          │
│  ├─ Workspace model    workspace→project→   │
│  │                     session · worktrees  │
│  ├─ Process awareness  fg process, agent    │
│  │                     detection (neutral)  │
│  ├─ Notifications      command/agent events │
│  └─ (later) CLI + event surface, MCP        │
└──────────────────┬──────────────────────────┘
                   │ syscalls (openpty, fork/exec, signals)
┌──────────────────▼──────────────────────────┐
│ Operating system                            │
└─────────────────────────────────────────────┘
```

### Division of responsibility

- **Rust owns reality**: anything touching the OS — PTYs, child processes,
  file access, signals, environment — lives in the Rust core. The frontend
  never gets raw OS capabilities.
- **Frontend owns presentation**: renders terminal state pushed over IPC,
  captures input, and sends it back as _data_, not commands to execute.
- **IPC is a protocol, not a convenience**: every command is an attack-surface
  entry point and is treated like a public API — typed, validated, least
  privilege via Tauri capabilities. See [design/ipc.md](design/ipc.md) and
  [design/security-model.md](design/security-model.md).

### Data flow (terminal I/O) — PROPOSED

1. Keystroke → frontend encodes → IPC → Rust writes to PTY.
2. PTY output → emulation engine updates grid state → damage regions batched
   (frame-coalesced) → event to frontend → renderer paints.
3. Bulk output is flow-controlled in Rust; the frontend receives bounded,
   render-ready updates, never an unbounded byte firehose.

### Performance boundaries

Frame-rate and memory budgets, and where backpressure is applied, are defined
in [PERFORMANCE.md](PERFORMANCE.md). The IPC hop is the structural risk of
this architecture and gets benchmarked first (see ADR-0001 consequences).

### Security boundaries

Threats and mitigations: [SECURITY_THREAT_MODEL.md](SECURITY_THREAT_MODEL.md).
Summary of stance: terminal output is untrusted input; the webview is
CSP-restricted and capability-limited; spawned shells inherit user privilege
(a terminal's job) but nothing Sill adds may _raise_ privilege; future
agent/MCP surfaces are opt-in and least-privilege by design.

## Design documents

| Area                 | Doc                                                              | Status             |
| -------------------- | ---------------------------------------------------------------- | ------------------ |
| Terminal engine      | [design/terminal-engine.md](design/terminal-engine.md)           | PROPOSED           |
| Workspace model      | [design/workspace-model.md](design/workspace-model.md)           | PROPOSED           |
| Agent architecture   | [design/agent-architecture.md](design/agent-architecture.md)     | PROPOSED           |
| IPC                  | [design/ipc.md](design/ipc.md)                                   | PROPOSED           |
| Security model       | [design/security-model.md](design/security-model.md)             | PROPOSED           |
| Performance          | [design/performance.md](design/performance.md)                   | PROPOSED           |
| Plugins              | [design/plugin-architecture.md](design/plugin-architecture.md)   | PROPOSED           |
| Release architecture | [design/release-architecture.md](design/release-architecture.md) | ACTIVE (CI exists) |
