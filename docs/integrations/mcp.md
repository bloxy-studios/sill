# MCP (Model Context Protocol)

**Status: Planned. No MCP functionality exists in Sill today.** This page
records intent and constraints so expectations stay calibrated.

## What MCP could mean for Sill

MCP is an open protocol through which AI applications and agents consume
tools and context. For a terminal, the plausible shape is **Sill as an MCP
server**, exposing _scoped, user-authorized_ terminal context to agents the
user already runs — e.g.:

- List sessions in the current workspace (names, kinds, statuses — not
  contents)
- Read the scrollback of _the session the user explicitly shared_
- Observe command results in a shared session (exit status, semantic zones)

## What Sill will not expose

Bindingly, from threat model T7/T8 and the
[security model](../design/security-model.md):

- No default access to anything — every scope is opt-in, per workspace/session
- No cross-session reads without an explicit grant per session
- No input injection / command execution surface in the initial design —
  telemetry out, nothing in
- No network-reachable endpoint — local, user-private transport only

## Sequencing

MCP is **L3** — last in the integration ladder
([design/agent-architecture.md](../design/agent-architecture.md)), gated on:
Phase 1–3 existing, the L2 local protocol's threat-model review, and an RFC
for the MCP surface itself. Anything MCP-related you see before that is
discussion, not software.
