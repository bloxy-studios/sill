# Claude Code

**Status: Planned. No Claude Code-specific integration exists.**

Claude Code is Anthropic's terminal-based coding agent. It is one of the
agents Sill's Phase 4 work targets — as an _adapter_ over the neutral layers
in [generic-agents.md](generic-agents.md), never a privileged code path
(ADR-0007).

## Intended adapter scope (L0/L1)

- L0 detection: recognize the `claude` process in a session → session kind
  `agent`, labeled "Claude Code"
- L1 signals: surface "waiting for input" and task-completion states via
  whatever standard escapes/notification behavior Claude Code emits —
  **to be verified against the real tool at implementation time**, not
  assumed here

## What will be documented here once real

Tested version pairs (Sill × Claude Code), interactive and non-interactive
(`-p`/print-mode) behavior, notification behavior, known limitations, and any
configuration on either side. Until then this page intentionally makes no
compatibility claims — see the [matrix](../AGENT_COMPATIBILITY.md).
