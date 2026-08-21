# OpenSSF Scorecard Posture

How Sill measures against OpenSSF Scorecard's 20 checks. **No score is
claimed here** — the Scorecard workflow (`.github/workflows/scorecard.yml`,
weekly + on push to main) produces the real number once it runs; this
document records posture and plan. Optimization target is real security, not
the score.

_Assessed 2026-08-21 against the current check list (Binary-Artifacts,
Branch-Protection, CI-Tests, CII-Best-Practices, Code-Review, Contributors,
Dangerous-Workflow, Dependency-Update-Tool, Fuzzing, License, Maintained,
Packaging, Pinned-Dependencies, SAST, SBOM, Security-Policy, Signed-Releases,
Token-Permissions, Vulnerabilities, Webhooks)._

## Expected-strong (implemented in-repo)

| Check                  | Basis                                                                                                                |
| ---------------------- | -------------------------------------------------------------------------------------------------------------------- |
| License                | MIT OR Apache-2.0, license files present                                                                             |
| Security-Policy        | SECURITY.md with private reporting                                                                                   |
| Dependency-Update-Tool | Dependabot: cargo, bun, github-actions                                                                               |
| Pinned-Dependencies    | Lockfiles committed; actions SHA-pinned; toolchain pinned                                                            |
| Token-Permissions      | All workflows default `contents: read`; elevations per-job, commented                                                |
| Dangerous-Workflow     | No `pull_request_target`, no PR-code execution with write tokens                                                     |
| CI-Tests               | CI runs tests/lints/builds on PRs                                                                                    |
| Vulnerabilities        | cargo-deny advisories green (16 documented ignores for Tauri's known unmaintained transitive deps — see `deny.toml`) |
| Binary-Artifacts       | None in repo                                                                                                         |

## Requires repository settings (maintainer checklist in MAINTAINERS.md)

- **Branch-Protection**: enable a ruleset on `main` (require PRs + status
  checks, block force-push). Scorecard reads rulesets with the default
  token — use rulesets, not classic protection.
- Secret scanning + push protection, private vulnerability reporting,
  Dependabot alerts: enable in Settings (all free on public repos).

## Honest weak spots and their real answers

| Check                      | State                                                   | Plan                                                                                                                      |
| -------------------------- | ------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------- |
| Code-Review / Contributors | Solo maintainer: self-merged PRs are structural for now | Governance path to reviewer #2 ([BUS_FACTOR.md](BUS_FACTOR.md)); until then, PR discipline + full CI on every change      |
| Signed-Releases            | No releases; signing unfunded                           | Checksums now, attestations next, signing when funded ([RELEASE_SECURITY.md](RELEASE_SECURITY.md))                        |
| Fuzzing                    | None (no parser exists yet)                             | Parser fuzzing lands with the terminal engine (ADR-0006 requirement)                                                      |
| SAST                       | Only clippy today                                       | Evaluate CodeQL once real code exists; clippy `-D warnings` is the current gate                                           |
| SBOM                       | Not generated                                           | Per-release CycloneDX planned with first releases ([SUPPLY_CHAIN.md](SUPPLY_CHAIN.md))                                    |
| CII-Best-Practices         | No badge                                                | Pursue OpenSSF Best Practices badge after Phase 1 (most criteria already met on paper; the badge wants a working project) |
| Maintained                 | Day-zero repo; the check wants sustained activity       | Only time fixes this — that's the point of the check                                                                      |
| Packaging / Webhooks       | N/A today                                               | Revisit with releases                                                                                                     |

## Reading the score when it arrives

Early scores will be middling because Contributors/Code-Review/Maintained
structurally punish young solo projects — correctly. The number goes in
[evidence/](evidence/) snapshots unedited, alongside this context.
