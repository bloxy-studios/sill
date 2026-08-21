# Supply Chain Security

How code gets into Sill and how artifacts get out. Companion documents:
[SECURITY_THREAT_MODEL.md](SECURITY_THREAT_MODEL.md) (threat T11),
[DEPENDENCY_POLICY.md](DEPENDENCY_POLICY.md), [RELEASE_SECURITY.md](RELEASE_SECURITY.md),
[OPENSSF.md](OPENSSF.md).

## In place now

| Control                | Implementation                                                                                                                                                                 |
| ---------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Locked dependencies    | `bun.lock` and `src-tauri/Cargo.lock` committed; CI installs with `--frozen-lockfile` / locked resolution                                                                      |
| Pinned toolchain       | `rust-toolchain.toml` pins the Rust version; Bun version declared in CI and docs                                                                                               |
| Dependency review      | New/changed dependencies justified in PRs per [DEPENDENCY_POLICY.md](DEPENDENCY_POLICY.md); `dependency-review-action` flags vulnerable/incompatibly-licensed additions on PRs |
| Vulnerability scanning | `cargo-deny` advisories check in CI (PRs + weekly schedule); Dependabot alerts                                                                                                 |
| License enforcement    | `cargo-deny` license allowlist (`deny.toml`); inventory in [THIRD_PARTY.md](THIRD_PARTY.md)                                                                                    |
| Automated updates      | Dependabot: Cargo, GitHub Actions, JS deps — weekly, grouped; updates run full CI before merge, never auto-merged                                                              |
| Registry sources       | `cargo-deny` restricts crates to crates.io; no git/path dependencies without explicit review                                                                                   |
| Action pinning         | Third-party GitHub Actions pinned to full commit SHAs, version noted in comments                                                                                               |
| Least-privilege CI     | Workflows default `permissions: contents: read`; elevated permissions declared per-job with a comment; PR-triggered workflows never run with write tokens against PR code      |
| Secret hygiene         | No secrets in repo; secret scanning + push protection expected enabled (maintainer checklist); credentials only in Actions secrets                                             |

## Planned (tracked, not claimed)

- **Artifact attestations** (build provenance) on release artifacts
- **SBOM** per release — CycloneDX for the Rust graph (`cargo-cyclonedx`) and
  the JS graph; evaluated once releases exist so SBOMs describe real artifacts
- **OpenSSF Scorecard** in CI with published results ([OPENSSF.md](OPENSSF.md))
- **Code signing / notarization** ([RELEASE_SECURITY.md](RELEASE_SECURITY.md))

## Non-claims

Sill does not currently claim: reproducible builds, SLSA levels, signed
artifacts, or third-party audits. Claims appear here only after they are
implemented and verified.

## Incident response

Suspected supply-chain compromise (malicious dependency version, compromised
action, tampered artifact): treat as a security incident — pull affected
drafts/releases from _Latest_, publish an advisory with affected versions and
checksums, rotate any credentials involved, and document the timeline in the
advisory. Report via [SECURITY.md](../SECURITY.md).
