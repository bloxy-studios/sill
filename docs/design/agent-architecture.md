# Design: Agent Architecture

**Status: PROPOSED — nothing implemented.** Governing decision (open):
[ADR-0007](../decisions/0007-agent-integration-model.md). Roadmap Phase 4.
Threat model sections T7/T8 are binding constraints on this design.

## Problem, concretely

A developer runs Claude Code in one session, Codex CLI in another, a build in
a third. Agents work for minutes, then block on a question — silently, in an
unfocused tab. The human context-switches away and the agent idles for an
hour. Today's terminals treat these sessions as indistinguishable rectangles
of text. The waste is real and growing as parallel-agent workflows spread
(often one agent per git worktree — which is why the
[workspace model](workspace-model.md) treats worktrees as first-class).

## Design constraints (from ADR-0007, non-negotiable)

1. **Provider-neutral**: no hard-coded vendor privilege; adapters implement a
   common interface; any agent can integrate via documented mechanisms.
2. **AI-optional**: every mechanism must be useful for non-agent processes
   too (a build, a test watcher) — "agent" is a session `kind`, not a mode.
3. **Least privilege by default**: integration surfaces are opt-in, scoped to
   the requesting session, auditable (T7).

## Layered proposal (weakest → strongest signal)

### L0 — Process awareness (no cooperation needed)

Foreground-process inspection per session: name, cwd, runtime. Detection of
known agent binaries by name/path is an _adapter data file_, not code
branches — adding an agent is a table row.

### L1 — Terminal-native signals (standard escape sequences)

- OSC 133 semantic zones → command boundaries, exit status ("the agent's last
  command failed")
- OSC 9 / OSC 777 notifications → "needs attention", surfaced through the
  session `status` field and OS notifications
  These are vendor-neutral, already emitted by some tools, and benefit shells
  and builds identically. Likely the highest value-per-risk layer.

### L2 — Explicit local protocol (opt-in, cooperating agents)

A documented handshake (env var `SILL_*` pointing at a user-private local
socket) through which a process may _offer_ structured status: task
description, progress, blocked-on-input. Strictly _inbound telemetry_ in v1 —
no control channel, no cross-session reads, no input injection. Requires its
own RFC + threat-model expansion before any code (T7/T8).

### L3 — MCP surface

Sill as an MCP server exposing _scoped_ terminal context to agents the user
authorizes. See [integrations/mcp.md](../integrations/mcp.md). Furthest out,
largest surface, gated on L2 learnings.

## What Sill will not build

- A bundled agent, LLM calls, or API-key management — Sill is the bench, not
  the power tool
- Vendor-exclusive integrations or paid-placement treatment
- Automation that types into sessions "on the agent's behalf" without an
  explicit, per-action user grant (T7)

## Open questions

- Detection false-positive posture: prefer under-detection (quiet) over
  over-detection (noisy) — proposed default: under-detect
- Where adapter definitions live (in-tree data vs user-extensible config)
- Whether L2 status vocabulary can converge with what agent CLIs already
  print (avoid inventing a standard where one is emerging — watch the
  ecosystem before freezing names)
