# Technical Progress Evidence

Shipped technical work worth citing externally: subsystems landed, features
shipped, measured performance improvements, platform support achieved, and
notable fixes. Each entry links the PRs/releases that prove it.

Rules: an entry describes something merged and working, in past tense, with a
link. Roadmap intentions do not appear here — that is what
[ROADMAP.md](../../ROADMAP.md) is for.

## Entries

- **2026-08-21 — Terminal core landed** (Phase 2 of the build plan, via the
  feat/terminal-foundation PR): `crates/sill-core` — PTY sessions,
  alacritty_terminal-backed emulation, typed events, capped scrollback;
  19 headless tests against real shells. Engine chosen via measured spike
  (throughput + RSS, two engines, four corpora):
  [ADR-0006](../decisions/0006-terminal-engine.md),
  [benchmark results](../../benchmarks/engine-spike/RESULTS.md). PUBLIC.
  Caveat recorded honestly: GUI rendering not yet verified on real display
  hardware at entry time.
