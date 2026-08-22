# ADR-0006: Terminal emulation engine selection

- **Status:** Accepted
- **Date:** 2026-08-21 (proposed and accepted same day, with measurements)

## Context

Phase 1 needs VT/xterm-compatible terminal emulation: parsing escape
sequences, maintaining grid state, scrollback, and modes. Writing a correct
emulator from scratch is a multi-year project; several proven Rust
implementations exist. This is the single most consequential Phase 1 decision.

## Candidates (to be evaluated, not yet chosen)

| Candidate                                    | Notes to validate                                                                                                                    |
| -------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------ |
| `alacritty_terminal`                         | Battle-tested grid/parser extracted from Alacritty; check API fit for embedding + scrollback model + license (Apache-2.0/MIT status) |
| `wezterm` crates (`termwiz`, `portable-pty`) | Rich, actively maintained, `portable-pty` likely wanted regardless for PTY layer; check modularity/weight                            |
| `vt100`/other parser-only crates             | Lighter; more assembly required around them                                                                                          |
| Custom engine                                | Rejected as a starting point; revisit only with strong evidence the above can't serve                                                |

## Decision criteria

1. Correctness against vttest + real programs (vim, tmux, htop, fzf, agents' TUIs)
2. Embeddability: clean state snapshot/diff API for rendering over IPC
3. Performance under flood output (`cat` a large file, build logs)
4. Maintenance health and license compatibility (must be MIT/Apache-compatible)
5. Scrollback + damage-tracking design

## Decision

**`alacritty_terminal` 0.26 for emulation; `portable-pty` 0.9 for the PTY
layer.** Implemented in `crates/sill-core`.

Measured basis (full matrix + methodology:
[benchmarks/engine-spike/RESULTS.md](../../benchmarks/engine-spike/RESULTS.md),
50MB synthetic corpora, isolated processes, Linux x86_64):

| Corpus       | alacritty_terminal | vt100     |
| ------------ | ------------------ | --------- |
| plain        | **83.1 MB/s**      | 68.3 MB/s |
| ansi-heavy   | **98.2 MB/s**      | 90.5 MB/s |
| unicode      | **92.7 MB/s**      | 73.8 MB/s |
| cursor-heavy | **92.8 MB/s**      | 90.0 MB/s |

alacritty_terminal also used ~25% less memory at full 10k-line scrollback
(~30MB vs ~40MB engine-only), is the most recently released emulation core,
is rendering-agnostic (fits the webview split), and is Apache-2.0. vt100
remains a respectable lighter alternative; termwiz was not benchmarked —
its broader surface model needs substantially more embedding assembly and
its release cadence made it the fallback, not the primary (crate landscape:
[design/terminal-engine.md](../design/terminal-engine.md)).

Correctness spot-checks in sill-core's tests: SGR color, OSC title, alt
screen roundtrip, bracketed-paste mode, scrollback capping, wide chars in
snapshot mapping. Full vttest-style battery and parser fuzzing (threat
model T1) remain Phase 3 commitments.

Revisit trigger: if the damage/snapshot API proves limiting for delta
rendering, or upstream maintenance stalls, re-evaluate against the WezTerm
stack with this same harness.
