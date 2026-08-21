# ADR-0007: Provider-neutral agent integration model

- **Status:** Proposed (open — no decision made; nothing implemented)
- **Date:** 2026-08-21

## Context

A defining Sill thesis: terminals now host autonomous coding agents (Claude
Code, Codex CLI, Cursor's CLI, Gemini CLI, Aider, OpenCode…) that run long,
pause for input, and finish silently. Users lose track of which session needs
them. Sill wants to surface agent state — without hard-coding any vendor.

## Constraint (non-negotiable)

**No provider gets privileged treatment.** Integration must work through
neutral mechanisms any agent can adopt, and Sill must remain a first-class
terminal with zero agents involved.

## Candidate mechanisms (to evaluate)

1. **Passive detection**: process-tree inspection (known binary names) +
   heuristics on output. Zero agent cooperation needed; brittle; read-only.
2. **Standard escape sequences**: OSC 133 (semantic zones), OSC 9/777
   (notifications) — agents already emit some of these; terminal-native and
   vendor-neutral.
3. **Explicit local protocol**: documented, opt-in channel (e.g. env var
   handshake + local socket) any agent can implement. Most capable; requires
   adoption; largest attack surface — needs its own threat-model section
   before any implementation.
4. **MCP surface**: expose terminal context to agents via MCP
   ([integrations/mcp.md](../integrations/mcp.md)). Complements, not replaces,
   the above.

Likely shape: 1+2 first (read-only, no cooperation required), 3/4 only via RFC
with the security model settled. Not decided.

## Consequences of the neutrality constraint

- ✅ Ecosystem credibility; no vendor lock; survives agent-market churn.
- ⚠️ Slower than hard-coding one vendor's conventions; per-agent adapters must
  live behind a common interface.

Status: design exploration in
[design/agent-architecture.md](../design/agent-architecture.md) (PROPOSED).
This ADR will be split into concrete accepted decisions as Phase 4 approaches.
