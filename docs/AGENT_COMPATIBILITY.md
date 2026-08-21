# Agent Compatibility Matrix

**Current truth: no agent integration exists, because the terminal does not
exist yet.** No compatibility is claimed for anything. This matrix is the
tracking structure that will hold _tested_ results from Phase 4 onward;
until a row says otherwise, the honest status of everything is **Planned**.

Statuses: **Supported** (tested, documented) · **Partial** (works with listed
limitations) · **Experimental** (works sometimes; expect breakage) ·
**Planned** (intended; untested). A status only changes with a documented
test on a named Sill version.

| Agent              | Runs in session (basic PTY) | Detection (L0) | Semantic signals (L1) | Protocol (L2) | MCP (L3)  | Notes                                                |
| ------------------ | --------------------------- | -------------- | --------------------- | ------------- | --------- | ---------------------------------------------------- |
| Claude Code        | Planned                     | Planned        | Planned               | Planned       | Planned   | —                                                    |
| Codex CLI          | Planned                     | Planned        | Planned               | Planned       | Planned   | —                                                    |
| Cursor (CLI)       | Planned                     | Planned        | Planned               | Planned       | Planned   | —                                                    |
| Gemini CLI         | Planned                     | Planned        | Planned               | Planned       | Planned   | —                                                    |
| Aider              | Planned                     | Planned        | Planned               | Planned       | Planned   | —                                                    |
| OpenCode           | Planned                     | Planned        | Planned               | Planned       | Planned   | —                                                    |
| Generic TUI agents | Planned                     | n/a            | Planned               | open spec     | open spec | The layers are provider-neutral by design (ADR-0007) |

Layer definitions (L0–L3): [design/agent-architecture.md](design/agent-architecture.md).
Per-agent notes live in [integrations/](integrations/).

## What "Supported" will require (defined now, so it can't be inflated later)

1. Agent runs correctly in a Sill session for a full real task (TUI renders,
   input works, resize works)
2. L0 detection identifies it with no false positives in normal shells
3. Documented on the agent's page in [integrations/](integrations/) with
   tested versions (Sill + agent) and known limitations
4. A regression check exists where feasible

## Requesting an agent integration

Open an issue with the _Agent Integration Request_ form. Neutrality rule:
requests are implemented as adapters over the common layers — Sill does not
add vendor-specific privileged paths (see ADR-0007).
