# Technical Innovation

_Calibrated claims only. Three buckets: what exists, what is designed, what
is research. Reviewers should be able to test every sentence._

## Implemented today (verifiable in this repository)

Innovation-adjacent _practice_, not product yet: a security-first project
skeleton unusual for day-zero projects — design-stage threat model,
SHA-pinned supply chain, license-enforced dependency graph, evidence system
that refuses fabricated metrics. This is process discipline, claimed as
nothing more.

## Designed, not yet built (PROPOSED docs in ../design/)

1. **Work-shaped terminal model.** Workspace → project → session with
   worktree awareness as the _native_ model of a terminal (not a multiplexer
   bolted inside one). Novelty is in placement and integration: tmux/Zellij
   prove session models; no permissively-licensed terminal makes them — plus
   project/worktree identity — the primary abstraction.
2. **Layered, provider-neutral agent surface (L0–L3).** Passive detection →
   standard escapes (OSC 133/9/777) → opt-in local telemetry protocol →
   scoped MCP; strictly no privileged vendor. The neutral-protocol-ladder
   framing — with a threat model gating each rung — is, to our knowledge,
   not implemented by existing terminals (incumbents integrate their own
   agent or target specific tools). "To our knowledge" is doing honest work
   in that sentence; the landscape moves fast and is re-checked in
   [../COMPETITIVE_LANDSCAPE.md](../COMPETITIVE_LANDSCAPE.md).
3. **Damage-coalesced IPC rendering contract.** Frame-aligned grid deltas
   with Rust-side backpressure as the webview rendering strategy — the
   make-or-break engineering bet of the stack, stated with its kill
   criterion (ADR-0001).

## Research directions (no design commitment)

Semantic terminal events as an ecosystem convention (converging with what
agent CLIs already emit rather than inventing a standard); cross-session
attention scheduling ("what needs a human next?") as a first-class terminal
concept; workspace state as a queryable local API for tooling.

## What Sill does not claim

No performance numbers (no harness yet), no protocol adoption (none
proposed publicly yet), no user validation (no users yet). Each claim above
graduates buckets only via shipped code and [../evidence/](../evidence/).
