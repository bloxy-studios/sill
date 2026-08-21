# Design: IPC

**Status: PROPOSED — the scaffold's single `greet` command is the only IPC
today.** This document sets the rules everything real will follow.

## Role

Tauri IPC is the _only_ channel between webview and Rust core — and therefore
both the app's API and its primary internal attack surface (threat model T4).
It is treated like a public API: typed, versioned-by-discipline, reviewed.

## Rules

1. **Commands are verbs on the model, not plumbing.** `session_input`,
   `workspace_list`, `session_resize` — the frontend never gets generic
   power (`run_command`, `read_file` do not exist).
2. **Validate at the boundary.** Every command validates session/workspace
   ids against the caller's window and rejects out-of-scope access; payloads
   are typed structs (serde), never stringly-typed blobs.
3. **Capabilities stay minimal.** Tauri capability grants are reviewed
   per-addition; the default posture is deny (see `capabilities/default.json`
   — currently `core:default` + `opener:default` only).
4. **Output flows as events, input as commands.** PTY output → batched,
   damage-coalesced events (push). User input → `session_input` command.
   The frontend never polls.
5. **Backpressure lives in Rust** (see [performance](performance.md)):
   frame-coalescing and bounded buffers before the IPC hop, so a `yes`(1)
   flood costs one render per frame, not one event per line.
6. **No secrets over IPC** beyond what the user typed into the terminal
   itself; no command returns environment dumps or raw buffers outside the
   requesting session's scope.

## Sketch (PROPOSED, will change)

```
Commands (webview → core)
  session_create(project_id, kind, cmd?) -> SessionId
  session_input(session_id, bytes)
  session_resize(session_id, cols, rows)
  session_kill(session_id)
  workspace_snapshot() -> WorkspaceState

Events (core → webview)
  session_damage(session_id, grid_delta)
  session_status(session_id, status)      // idle|running|attention|exited
  notification(kind, payload)
```

## Testing intent

IPC handlers get unit tests at the Rust boundary (bad ids, oversized
payloads, wrong-window access). The command list above is also the seed of
the future `sill` CLI surface (Phase 5) — designing them as a coherent API
now is what makes that phase cheap later.
