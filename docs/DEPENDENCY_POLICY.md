# Dependency Policy

Sill's stance: **every dependency is a liability with a job.** A terminal is
long-lived, security-sensitive software; its dependency graph should be boring
and explainable.

## Accepting a new dependency

A PR adding a dependency answers, in its description:

1. **Why** — what it does that we shouldn't write/maintain ourselves
2. **Alternatives** — including "std / write it" for small utilities
3. **License** — must pass the `deny.toml` allowlist (MIT/Apache-2.0-family,
   BSD, ISC, Zlib, MPL-2.0…). GPL/AGPL/SSPL/source-available: rejected as
   linked dependencies (see [LICENSING.md](LICENSING.md))
4. **Health** — maintained? bus factor? release cadence? open RUSTSEC/CVEs?
5. **Weight** — transitive deps added, compile-time and binary-size impact
   (Rust), bundle-size impact (JS)

Micro-dependencies for trivial code (left-pad-class) are declined on
principle, especially in JS. Prefer std, then a well-established crate, then
vendoring a small piece (with attribution in [THIRD_PARTY.md](THIRD_PARTY.md)),
then a new dependency.

Heightened scrutiny for anything touching: PTY/process spawning, parsing of
untrusted input (escape sequences!), IPC, crypto, or the build/release
pipeline.

## Updates

- **Dependabot** (weekly, grouped): patch/minor grouped per ecosystem; majors
  arrive individually. Nothing auto-merges; updates pass full CI + maintainer
  review. Changelogs of security-relevant deps get read, not skimmed.
- **Security advisories** (Dependabot alerts + `cargo-deny` advisories, PR
  and weekly scheduled runs): triaged within days; exploitable-in-Sill issues
  are prioritized `priority:critical` and may trigger an out-of-band release.
- **Lockfiles are the truth**: `bun.lock` / `Cargo.lock` committed; CI uses
  frozen/locked installs so builds don't drift.

## Abandoned dependencies

A dependency that stops being maintained (no releases/response ~12+ months,
unpatched advisories) gets an issue labeled `area:build` + `type:security`
when relevant, and a plan: replace, vendor, or fork-as-last-resort. RUSTSEC
"unmaintained" advisories surface this automatically via `cargo-deny`.

## Version pinning philosophy

- Applications pin aggressively: exact lockfiles + pinned toolchain
  (`rust-toolchain.toml`).
- Cargo semver ranges stay conventional (`"1"`-style) — the lockfile does the
  pinning; ranges matter only if Sill ever publishes library crates.
- Toolchain bumps (Rust, Bun) are deliberate PRs with a changelog line, not
  side effects.
