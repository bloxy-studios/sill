# ADR-0008: Product naming and crate-publishing policy

- **Status:** Accepted
- **Date:** 2026-08-21

## Context

The 2026-08-21 naming review (audit §4.2) found the obvious namespaces
contested: crates.io `sill` and `sill-adapter` are held by an unrelated
AI-agent credential tool (GitHub org `sill-sh`; adjacent domain, same
registry); npm `sill` is a dormant squat; `sill.social` is an active,
unrelated product holding bare-word search mindshare; "SILL" is a French
government FOSS-catalog acronym. Trademark registers were not queried.

A follow-up availability sweep (same day) found every namespace a terminal
actually distributes through **open**: the Homebrew formula and cask `sill`,
the `sill` binary name (no conflicting popular CLI), and — notably — the
domain **`sill.sh`** (the crates.io project named its org "sill-sh" but never
registered the domain). Also free at decision time: `sill.so`,
`sillterm.com`/`.dev`, crates.io `sillterm`/`sill-term`/`sill-core`/
`sill-protocol`, and npm `sillterm`.

Precedent considered: Warp terminal coexists with the unrelated,
highly-popular `warp` Rust crate and Cloudflare WARP — compound-name search
("warp terminal") proved sufficient. Alacritty and WezTerm both publish
reusable internals under names decoupled from the app brand
(`vte`, `termwiz`, `portable-pty`).

## Decision

1. **The product name stays Sill.** Binary: `sill`. Homebrew formula/cask:
   `sill`. Prose and metadata use the compound "Sill terminal" where
   disambiguation matters.
2. **Bare `sill` and any `sill-*` name are never published to crates.io or
   npm** — even where individual names are free. Publishing them would
   interleave Sill's crates with an unrelated agent-space project's
   `sill`/`sill-adapter` line on the same registry, confusing both projects'
   users.
3. **Ecosystem pieces intended for third-party adoption get standalone,
   descriptive, brand-neutral names**, chosen at extraction time with a
   fresh availability sweep (the `vte`/`termwiz`/`portable-pty` pattern).
   This applies with extra force to the ADR-0007 agent protocol: a
   provider-neutral protocol must not carry one terminal's brand, or other
   terminals won't adopt it. Neutral naming there is strategy, not just
   collision avoidance.
4. **App-branded published internals, if ever needed, use the `sillterm-*`
   prefix** (e.g. `sillterm-core`, `sillterm-cli`); a future JS SDK uses the
   npm scope `@sillterm/*`. Both verified free at decision time.
5. **Canonical domain: `sill.sh`** — available as of 2026-08-21 and the best
   possible TLD for a terminal. Acquisition is a maintainer action to
   complete promptly; this ADR records the designation, not the purchase.

## Alternatives considered

- **Rename now** (the cheapest day for it — zero users/releases). Rejected:
  every distribution-critical namespace is open, the Warp precedent shows
  compound-name terminals thrive, the colliding crates.io project shows no
  activity since its first week, and a taste-driven rename cycle at day zero
  costs the momentum Phase 1 needs. A rename would also inherit its own
  collision sweep.
- **Publish under `sill-*` where free** (e.g. `sill-core`). Rejected per
  point 2 — registry interleaving with an unrelated project is the one
  actively harmful option.
- **Defer the decision.** Rejected: the audit correctly noted the decision
  only gets more expensive, and open naming questions leak into every
  publishing and branding choice.

## Consequences

- ✅ Audit §4.2's open risk is closed with a concrete policy; publishing and
  branding decisions stop re-litigating the name.
- ⚠️ Bare-name identity on crates.io/npm is permanently ceded; first
  mentions carry the "terminal" qualifier; bare-word SEO remains contested
  (accepted, with eyes open).
- Optional, zero-cost: a polite transfer inquiry to the crates.io `sill`
  owner (36 downloads, apparently inactive). Changes nothing if declined.
- Open item regardless of this decision: a trademark register check
  (USPTO/EUIPO/WIPO) for "Sill" in software classes.

## Revisit triggers (binding)

Re-open the rename question — **before `v0.1.0-alpha.1`, never after real
users exist** — if any of:

1. The `sill-sh`/crates.io `sill` project visibly revives and grows;
2. A trademark conflict surfaces;
3. "sill terminal" proves unwinnable in search at first-announcement time.

Absent a trigger, the name question is settled and stays out of the way of
building the terminal.
