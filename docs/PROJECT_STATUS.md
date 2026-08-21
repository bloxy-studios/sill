# Project Status

> Maintained snapshot of where Sill actually is. If this file's date is stale
> by more than ~2 months, treat that as information too.

**Last updated: 2026-08-21**

## Phase

**Pre-alpha, Phase 1 (Terminal foundation) — not yet started in code.**

The repository currently contains the application scaffold (Tauri 2 + React
runs and builds) and the project's open-source infrastructure (licensing,
CI, security policy, governance, roadmap, architecture/design docs). There is
no terminal functionality yet. Nothing is installable for real use.

## Current focus

1. Terminal engine evaluation spike — the open decision in
   [ADR-0006](decisions/0006-terminal-engine.md)
2. First PTY: spawn a shell, stream output to the frontend, round-trip input
3. Proving the rendering approach against the
   [performance budgets](PERFORMANCE.md)

## What works today

- `bun tauri dev` launches the scaffold app on macOS/Linux/Windows
- CI: typecheck, lint, format, `cargo fmt`/`clippy`/`test`, dependency and
  license scanning, cross-platform build checks

## Known limitations

- Everything a terminal does — Sill doesn't do yet
- Release artifacts: none published; signing not configured
  ([RELEASE_SECURITY.md](RELEASE_SECURITY.md))
- Solo maintainer: response times are best-effort ([BUS_FACTOR.md](BUS_FACTOR.md))

## Recent progress

- 2026-08-21 — Repository created; open-source foundation landed (licenses,
  governance, security policy, threat model, CI, release automation, docs)

## Current blockers

None external — the constraint is maintainer time.

## Upcoming priorities

See [ROADMAP.md](../ROADMAP.md) Phase 1. Next visible milestone: a build that
can replace the maintainer's daily terminal.
