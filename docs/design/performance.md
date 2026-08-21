# Design: Performance Architecture

**Status: PROPOSED — budgets exist ([PERFORMANCE.md](../PERFORMANCE.md)), no
implementation yet.** This document is _how_ the budgets get met, so the
first implementation isn't designed slow.

## Structural bets

1. **The hot path is: PTY read → emulation → damage → render.** Everything on
   it is budgeted; everything off it (workspace metadata, notifications,
   agent detection) runs at leisure and must never block it.
2. **Coalesce at the source.** PTY output is read in chunks, fed to the
   engine, and damage is flushed to the frontend _at most once per frame_
   (display-refresh aligned). A million lines of `yes` is one grid diff per
   frame, not a million events. Backpressure applies before the IPC hop
   (bounded channel; reader slows when render lags).
3. **Ship deltas, not screens.** IPC carries damage regions + dirty rows in a
   compact binary-ish encoding (exact format decided with ADR-0006's engine
   choice), not full-grid JSON per update.
4. **Renderer draws cells, not DOM.** The grid renders to canvas/WebGL with a
   glyph atlas; DOM is for chrome (tabs, palettes), never per-cell. This is
   the load-bearing assumption of the webview bet (ADR-0001) and the first
   thing Phase 1 must prove or kill.
5. **Idle means zero.** No animation loops, no polling timers while nothing
   changes; PTY readers park on epoll/kqueue/IOCP via the async runtime.
   Idle CPU is a budget line, tested.
6. **Scrollback is a ring, memory-bounded**, configurable; per-session cost
   must stay near the marginal-session budget (<10 MB), which mostly means
   scrollback discipline.

## Multi-session scaling (the thesis workload)

Target shape: tens of sessions, several producing output simultaneously
(agents + builds). Implications: per-session reader tasks are cheap
(async, not thread-per-session); only _visible_ sessions render — background
sessions update state and status, skipping damage transport entirely;
`attention` transitions (the thing users actually need from background
sessions) are events, costing nothing until they fire.

## Measurement discipline

Budgets without a harness are vibes. Phase 1 lands
[benchmarks/](../../benchmarks/) alongside the first PTY: startup, input
latency, flood throughput, idle CPU, per-session memory — reproducible,
methodology-documented, run on the reference machines. Regressions on the hot
path are release blockers once budgets are ratified. No number is published
that the harness didn't produce.
