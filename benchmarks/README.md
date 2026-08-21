# Benchmarks

**Status: no harness and no results exist yet.** This directory is
deliberately created _before_ the terminal so that the first performance
claim Sill ever makes is reproducible. Until code lands here, Sill publishes
no numbers — see [docs/PERFORMANCE.md](../docs/PERFORMANCE.md) for the
budgets that will be tested.

## Planned benchmark suite (arrives with Phase 1)

| Benchmark        | Measures                         | Sketch of method                                                                                              |
| ---------------- | -------------------------------- | ------------------------------------------------------------------------------------------------------------- |
| `startup`        | cold launch → interactive prompt | timestamped process spawn → first PTY write accepted; N=20 runs, report median + p95                          |
| `input-latency`  | keypress → glyph visible         | synthetic input events + frame capture; p50/p99                                                               |
| `flood`          | sustained output handling        | `cat` fixed corpus (100MB mixed text/escape file) through a session; wall time, peak RSS, dropped-frame count |
| `idle`           | background cost                  | 10-minute idle session; CPU samples, wakeups/sec                                                              |
| `sessions`       | marginal session cost            | RSS at 1, 5, 10, 20 idle sessions; delta per session                                                          |
| `agent-workload` | thesis scenario                  | N sessions with scripted bursty writers (agent-like output patterns); UI responsiveness probe                 |

## Rules (binding, from docs/PERFORMANCE.md)

1. Every published result records: hardware, OS + version, Sill version/commit,
   configuration, methodology, and raw data location.
2. Results are produced by the harness in this directory — never assembled by
   hand, never cherry-picked across runs.
3. Comparisons against other terminals, if ever published, use identical
   hardware/method, current versions, their recommended configs — and get
   sanity-checked against those projects' own documentation first.

## Layout (planned)

```
benchmarks/
  README.md        this file
  corpus/          fixed input files (checked in or generated deterministically)
  harness/         benchmark runner
  results/         raw results, committed per reference machine + version
```
