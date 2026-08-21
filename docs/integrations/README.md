# Integrations

Per-agent and per-protocol integration notes. **Nothing in this directory is
implemented today** — Sill is pre-alpha and integration work is Roadmap
Phase 4+. Each page states its own status; the summary view is the
[Agent Compatibility Matrix](../AGENT_COMPATIBILITY.md), and the architecture
is [design/agent-architecture.md](../design/agent-architecture.md).

Status vocabulary used on every page: **Current** (implemented and tested) ·
**Experimental** (implemented, unstable) · **Planned** (documented intent
only).

| Page                                   | Status                             |
| -------------------------------------- | ---------------------------------- |
| [claude-code.md](claude-code.md)       | Planned                            |
| [codex.md](codex.md)                   | Planned                            |
| [cursor.md](cursor.md)                 | Planned                            |
| [generic-agents.md](generic-agents.md) | Planned (this is the primary spec) |
| [mcp.md](mcp.md)                       | Planned                            |

Design rule worth repeating: `generic-agents.md` is the real integration
surface — named-agent pages document _adapter data_ over those neutral
mechanisms, never bespoke code paths (ADR-0007).
