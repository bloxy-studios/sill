# Performance

## Why a terminal must be boringly fast

A terminal is held to a harsher standard than most software: it sits open all
day (idle cost matters), it's on the critical path of every keystroke (latency
is felt physically), and it must survive hostile workloads (a `cat` of a 500MB
log). For Sill there is an extra reason: the roadmap points at _many
simultaneous sessions_ with agents producing output in several at once.
Per-session overhead multiplies.

Performance is therefore a feature with budgets, not an adjective.

## Budgets (targets, not measurements)

**No benchmark numbers exist yet, because the terminal doesn't.** These are
the acceptance targets Phase 1 will be measured against on reference hardware
(to be defined alongside the harness — one Apple Silicon Mac, one x86_64
Linux machine, one Windows machine).

| Metric                              | Budget                                              |
| ----------------------------------- | --------------------------------------------------- |
| Cold start → interactive prompt     | < 500 ms                                            |
| Keypress → glyph (p99, idle system) | < 25 ms                                             |
| Idle CPU (open, no output)          | ~0% (no polling loops)                              |
| Memory, 1 idle session              | < 150 MB (webview tax included — measured honestly) |
| Memory, each additional session     | < 10 MB marginal                                    |
| Flood output (`cat` large file)     | no UI freeze; bounded memory; coalesced rendering   |
| 20 concurrent active sessions       | interactive UI throughout                           |

Budgets are revisable with evidence — but publicly, in this file, with
reasoning. The webview-stack risk (ADR-0001) makes the latency and memory
rows the ones to watch; if they can't be met, the renderer strategy changes
(that's the ADR's revisit trigger).

## Ground rules

- **Never fabricate or cherry-pick numbers.** Every published measurement
  carries hardware, OS, Sill version, configuration, and methodology, and must
  be reproducible via [benchmarks/](../benchmarks/).
- Idle means idle: no timers ticking for rendering when nothing changed.
- Backpressure lives in Rust: the frontend receives bounded, render-ready
  state, never an unbounded byte stream (see [ARCHITECTURE.md](ARCHITECTURE.md)).
- Binary/artifact size is tracked per release once releases exist.

## PR expectations

Changes touching the renderer, PTY/output path, IPC, or session model state
their expected performance impact in the PR description (template prompts for
it). "Probably none, because X" is an acceptable answer; silence is not.
Meaningful regressions need either justification or a fix before merge —
see [PR_REVIEW.md](PR_REVIEW.md).

## Current status

- Harness: not built (arrives with Phase 1; see [benchmarks/README.md](../benchmarks/README.md))
- Published results: none — this file will link them when they are real
