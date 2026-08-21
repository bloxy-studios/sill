# Design: Plugin Architecture

**Status: PROPOSED — far future (Roadmap Phase 7). No implementation, no
committed design.** This document exists to park constraints so nearer-term
work doesn't accidentally foreclose them — and to say clearly what has _not_
been decided.

## Why plugins at all (eventually)

The agent ecosystem churns faster than any core team can track; themes,
statusline widgets, and per-tool adapters are classic community territory. A
plugin surface is how a small core stays small.

## Constraints parked now

1. **Security first**: plugins run in a terminal that sees secrets. Whatever
   the mechanism, plugins get _scoped capabilities_ (which sessions, which
   events), not ambient authority — same philosophy as the agent surface
   (threat model T7/T8 apply verbatim).
2. **Agent adapters must be data before they are code.** L0 detection tables
   ([agent-architecture.md](agent-architecture.md)) should be extensible
   without a plugin runtime at all — that covers the most urgent
   extensibility need years before Phase 7.
3. **The core must not grow a plugin-shaped hole prematurely.** No hook
   points, no "plugin API" placeholders in Phase 1–5 code. APIs that exist
   get used; speculative ones get abused.
4. **Candidate runtimes** (unevaluated, listed to bound the space): WASM
   (Zellij precedent — strong sandboxing story), Lua (WezTerm precedent —
   config-programmability story), external processes over the L2/CLI surface
   (cheapest: reuse Phase 5's public API as _the_ extension mechanism).
   The honest bias: exhaust option three before building a runtime.

## Non-goals

- A plugin marketplace, revenue share, or curation program
- Plugins in the input hot path (keystroke latency is not extensible)

## Decision path

When Phase 7 approaches: RFC comparing the three candidate mechanisms against
the constraints above → ADR. Anything before that is conversation, not
commitment.
