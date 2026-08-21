# Project Status

> Maintained snapshot of where Sill actually is. If this file's date is stale
> by more than ~2 months, treat that as information too.

**Last updated: 2026-08-21**

## Phase

**Pre-alpha, Phase 1 (Terminal foundation) — core landed, GUI verification pending.**

The Rust terminal core exists and is tested headless (`crates/sill-core`:
PTY sessions over portable-pty, emulation over alacritty_terminal, typed
events, capped scrollback), wired through Tauri IPC to a minimal canvas
renderer with keyboard input, paste, resize, and wheel scrollback. The
engine choice is measured and recorded (ADR-0006 Accepted). Still true:
no releases, nothing installable, and the GUI leg has not yet been
exercised on real hardware.

## Current focus

1. Verify the Phase 2 terminal end-to-end on real hardware (macOS first):
   rendering correctness, input feel, CSP behavior
2. Terminal quality (Phase 3): selection/copy, search, application cursor
   mode, shell integration (OSC 133), parser fuzzing (threat model T1)
3. Benchmarks against the [performance budgets](PERFORMANCE.md) on the
   reference machines — no numbers are claimed until then

## What works today

- `crates/sill-core`: spawn real shells over PTYs, feed output through the
  emulation engine, snapshot the grid — 19 tests pass headless, including
  echo roundtrip, stty-verified resize, exit codes, OSC titles, alt screen,
  scrollback capping, and create/close churn
- Tauri IPC layer (commands + coalesced snapshot events) and a canvas
  grid renderer with keyboard/paste/resize/scrollback — **compiles and
  lints; not yet run on a GPU/display** (this sandbox is headless)
- Engine spike benchmark with recorded results
  ([benchmarks/engine-spike/RESULTS.md](../benchmarks/engine-spike/RESULTS.md))
- CI: typecheck, lint, format, `cargo fmt`/`clippy`/`test`, dependency and
  license scanning, cross-platform build checks

## Known limitations

- Everything a terminal does — Sill doesn't do yet
- Release artifacts: none published; signing not configured
  ([RELEASE_SECURITY.md](RELEASE_SECURITY.md))
- Solo maintainer: response times are best-effort ([BUS_FACTOR.md](BUS_FACTOR.md))

## Recent progress

- 2026-08-21 — Terminal core landed: sill-core crate (PTY + emulation +
  sessions, 19 headless tests), measured engine decision (ADR-0006
  Accepted), Tauri wiring, minimal canvas renderer
- 2026-08-21 — Repository created; open-source foundation landed (licenses,
  governance, security policy, threat model, CI, release automation, docs)

## Current blockers

None external — the constraint is maintainer time.

## Upcoming priorities

See [ROADMAP.md](../ROADMAP.md) Phase 1. Next visible milestone: a build that
can replace the maintainer's daily terminal.
