# Codex CLI

**Status: Planned. No Codex-specific integration exists.**

Codex CLI is OpenAI's terminal coding agent. Like every agent, it integrates
with Sill through the neutral layers in
[generic-agents.md](generic-agents.md) — L0 process detection and L1
standard-escape signals first (ADR-0007).

## Intended adapter scope (L0/L1)

- L0: recognize the `codex` process → `agent` session kind, labeled "Codex"
- L1: surface waiting/finished states from its actual terminal behavior —
  verified against the real tool during Phase 4, not assumed in advance

## What will be documented here once real

Tested version pairs, interactive vs non-interactive modes, notification
behavior, known limitations. No compatibility claims until then — see the
[matrix](../AGENT_COMPATIBILITY.md).
