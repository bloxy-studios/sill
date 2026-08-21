# Architecture

This document describes Sill's architecture in two honest layers: **what
exists today** (very little — the project is pre-alpha) and **the target
architecture** the roadmap builds toward (marked PROPOSED). Design detail per
subsystem lives in [docs/design/](design/).

## What exists today (2026-08)

The repository is a Tauri 2 scaffold plus project infrastructure:

- `src-tauri/` — Rust application shell. `lib.rs` registers a single
  placeholder IPC command (`greet`) and the `tauri-plugin-opener` plugin.
  Sandboxing/capabilities: `capabilities/default.json` grants `core:default`
  and `opener:default` to the main window.
- `src/` — React 19 + TypeScript frontend (template placeholder UI, will be
  replaced in Phase 1).
- Build: Vite 7 frontend, Bun for JS tooling, `tauri build` for packaging.

There is **no terminal, PTY, session, or agent functionality yet**. Everything
below this line is design intent.

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
