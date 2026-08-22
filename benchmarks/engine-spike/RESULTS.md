# Engine Spike Results (ADR-0006)

Reproduce: `cargo run --release -- <alacritty|vt100> <plain|ansi|unicode|cursor> 50`
(one process per cell — clean RSS).

## Environment

- 2026-08-21, Linux x86_64 (Amazon Linux 2023), 2× Intel Xeon @ 2.90GHz, 4GB RAM
- rustc 1.98.0, release profile (thin LTO), 120×40 viewport, 10,000-line scrollback
- 50 MB deterministic synthetic corpus per cell, fed in 32KB chunks

## Throughput (MB/s of raw PTY-style bytes parsed into grid state)

| Corpus                               | alacritty_terminal 0.26 | vt100 0.16 |
| ------------------------------------ | ----------------------- | ---------- |
| plain (build-log lines)              | **83.1**                | 68.3       |
| ansi (SGR-heavy, 16/256/truecolor)   | **98.2**                | 90.5       |
| unicode (CJK/emoji/combining)        | **92.7**                | 73.8       |
| cursor (full-screen repaint pattern) | **92.8**                | 90.0       |

## Peak RSS (KB, includes the 50MB in-memory corpus — subtract ~51,200)

| Corpus                      | alacritty_terminal     | vt100                  |
| --------------------------- | ---------------------- | ---------------------- |
| plain (scrollback filled)   | 82,200 (~30MB engine)  | 91,712 (~40MB engine)  |
| cursor (no scrollback fill) | 53,744 (~2.5MB engine) | 53,532 (~2.3MB engine) |

Notable: a **full** 120-col × 10k-line scrollback costs ~30MB in
alacritty_terminal — that is the real memory driver, which is why scrollback
is configurable and hard-capped in sill-core, and why the default stays
10k pending real-usage measurement (docs/PERFORMANCE.md budgets).

## Reading

- alacritty_terminal wins throughput on all four corpora (+9–27%) and uses
  ~25% less memory at full scrollback.
- Both are far above the bar: worst case ~68 MB/s means parsing is not the
  bottleneck on even weak hardware; rendering and IPC are (as designed for
  in docs/design/performance.md).
- termwiz was not benchmarked: its surface model requires substantially more
  assembly for embedding, and the maintained-but-slow release cadence noted
  in docs/design/terminal-engine.md made it the fallback candidate, not the
  primary. Revisit only if alacritty_terminal's API proves limiting.

Decision recorded in ADR-0006 (Accepted): alacritty_terminal for emulation,
portable-pty for the PTY layer.
