# Cursor (CLI)

**Status: Planned. No Cursor-specific integration exists.**

Cursor is primarily an IDE; its CLI agent runs in terminals, which is the
surface relevant to Sill. Integration follows the neutral layers in
[generic-agents.md](generic-agents.md) (ADR-0007) — same rules as every
other agent.

## Intended adapter scope (L0/L1)

- L0: recognize Cursor's CLI agent process → `agent` session kind
- L1: waiting/finished signals from its actual terminal behavior, verified
  against the real tool during Phase 4

## What will be documented here once real

Tested version pairs, supported modes, notification behavior, known
limitations. No compatibility claims until then — see the
[matrix](../AGENT_COMPATIBILITY.md).
