# Generic Agent Integration

**Status: Planned. Nothing here is implemented.** This page will become the
canonical spec any agent (or any long-running tool) can implement to be a
good citizen in Sill. It is the neutral mechanism every named-agent page
resolves to.

## Intended integration layers

| Layer | Mechanism                                                                                | What an agent gets                                       |
| ----- | ---------------------------------------------------------------------------------------- | -------------------------------------------------------- |
| L0    | Nothing — Sill observes process name/cwd                                                 | Shown as an `agent`-kind session                         |
| L1    | Standard escapes: OSC 133 zones, OSC 9/777 notifications                                 | Command status surfaced; "needs attention" notifications |
| L2    | Opt-in local protocol (`SILL_*` env → user-private socket), inbound telemetry only in v1 | Structured status: task, progress, blocked-on-input      |
| L3    | MCP ([mcp.md](mcp.md))                                                                   | Scoped, user-authorized terminal context                 |

The practical promise to agent authors, once real: **L1 alone will get a good
experience** — emit standard escapes and Sill (and other modern terminals)
will do the right thing. L2/L3 are enhancements, never requirements.

## Compatibility baseline that costs agents nothing

Any agent that works in a VT/xterm-compatible terminal will work in Sill's
sessions — that's a Phase 1 correctness requirement, not an integration
feature.

## When this becomes real

Spec text lands here via the Phase 4 RFC process
([design/agent-architecture.md](../design/agent-architecture.md), ADR-0007),
after the L2 threat-model review. Version compatibility tables will live on
this page from the first tested implementation.
