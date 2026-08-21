# Roadmap (Application View)

_Canonical roadmap: [ROADMAP.md](../../ROADMAP.md) — horizons (NOW/NEXT/
LATER/EXPLORING), no dates, non-goals versioned. This page frames it for
reviewers._

**Read this first:** Sill is at the _start_ of Phase 1. The roadmap is a
plan, and plans are evidence of thinking, not of delivery. Delivery evidence
accumulates in [../evidence/TECHNICAL_PROGRESS.md](../evidence/TECHNICAL_PROGRESS.md).

| Phase | Content                                                                                                | Horizon   | Exit criterion                                            |
| ----- | ------------------------------------------------------------------------------------------------------ | --------- | --------------------------------------------------------- |
| 1     | Terminal foundation: PTY, emulation (engine per ADR-0006), rendering, input, config                    | **NOW**   | Maintainer daily-drives Sill                              |
| 2     | Developer workflows: shell integration (OSC 133), command status/notifications, search, repo awareness | NEXT      | —                                                         |
| 3     | Workspace system: workspace→project→session model, worktrees, persistence                              | NEXT      | Model restores across restarts; answers "what's running?" |
| 4     | Agent-native, provider-neutral: L0 detection → L1 signals → (RFC-gated) L2 protocol                    | LATER     | Attention routing works with ≥2 real agents, tested       |
| 5     | Programmability: `sill` CLI, event stream                                                              | LATER     | —                                                         |
| 6     | Remote development                                                                                     | EXPLORING | only with a settled security model                        |
| 7     | Ecosystem: plugins, MCP                                                                                | EXPLORING | —                                                         |

**For grant scoping:** the natural fundable units are Phase 1 (a working,
benchmarked terminal — the ante) and Phase 3+4 (the differentiating
workspace/agent model with a published neutral protocol). Each has concrete,
verifiable exit criteria a funder can hold the project to.

**Change control:** roadmap changes land by PR; non-goals (no bundled AI, no
telemetry-by-default, no account) are part of the document and equally
version-controlled.
