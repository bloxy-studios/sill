# Solution

_What Sill actually proposes to build. Everything here is design intent
unless marked otherwise; implementation state is tracked in
[../PROJECT_STATUS.md](../PROJECT_STATUS.md)._

## 1. A real terminal (Phase 1 — the ante)

VT/xterm-compatible emulation on a proven Rust engine, native PTY management,
fast rendering, cross-platform (macOS/Linux/Windows). Nothing below matters
if this layer isn't excellent; exit criterion is "the maintainer daily-drives
it."

## 2. Work-shaped primitives (Phases 2–3 — the thesis)

Replace anonymous rectangles with a queryable model:

```
workspace → project (repo/worktree-aware) → session (kind + status + identity)
```

- Sessions are durable identities ("api dev server", "agent on auth-fix"),
  not tab positions; display becomes presentation, not existence.
- Worktrees are first-class — matching how agent-parallel development
  actually organizes.
- The model restores across restarts and can _answer questions_
  ("what's running?", "what needs me?").
- Zero-ceremony path preserved: a bare shell with no project stays one
  keystroke away. The model must earn its keep or stay out of the way.

## 3. Provider-neutral agent awareness (Phase 4 — the differentiator)

A layered, opt-in integration surface any agent can use — none privileged:

- **L0**: passive process awareness (works with zero agent cooperation)
- **L1**: standard terminal signals (OSC 133 semantic zones, OSC 9/777
  notifications) — benefits builds and shells identically
- **L2**: documented opt-in local protocol for structured status
  (inbound-telemetry-only in v1; threat-modeled before built)
- **L3**: scoped MCP surface, last and most carefully

Yielding the concrete payoff: session status (idle / running / **needs
attention**) surfaced across all projects, so N agents block on a human for
seconds, not hours.

## 4. Programmability (Phase 5)

`sill` CLI + event stream addressing the model ("open a session in project X
and run Y"), which is also how third-party tools integrate without plugins.

## What Sill deliberately is not

Not an AI product (no bundled agent, keys, or account); not an IDE or
browser; not telemetry-driven. Non-goals are versioned in
[../../ROADMAP.md](../../ROADMAP.md).

## Why this can win attention honestly

The incumbents in agent-aware terminals are copyleft and (in Warp's case)
anchored to a commercial agent platform. A **permissive (MIT OR Apache-2.0),
vendor-neutral, local-first** implementation of the workspace/agent model is
an unoccupied and ecosystem-friendly position — adoptable by users who can't
run AGPL software and by tools that want to integrate without legal review.
That is a structural differentiation, not a marketing one — and it only
matters if Phase 1 ships an excellent terminal, which is the current work.
