# ADR-0006: Terminal emulation engine selection

- **Status:** Proposed (open — no decision made)
- **Date:** 2026-08-21

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

## Status

An evaluation spike per candidate is the first Phase 1 engineering task; the
results will be recorded here and this ADR moved to Accepted. Current crate
landscape research lives in [design/terminal-engine.md](../design/terminal-engine.md).
