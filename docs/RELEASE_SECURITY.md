# Release Integrity

What a person downloading Sill can verify, today and planned. This document
refuses to claim guarantees that aren't configured yet.

## Current state (pre-first-release)

| Mechanism                                                      | Status                                                                                                                                                                                    |
| -------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Built by CI from a public tag (no laptop builds)               | ✅ defined in `release.yml`                                                                                                                                                               |
| SHA-256 checksums (`SHA256SUMS.txt`) attached to every release | ✅ defined in `release.yml`                                                                                                                                                               |
| Draft-then-human-publish gate                                  | ✅ policy ([RELEASING.md](RELEASING.md))                                                                                                                                                  |
| GitHub artifact attestations (build provenance)                | ⬜ planned next; free for public repos, needs workflow wiring + verification docs                                                                                                         |
| macOS Developer ID signing + notarization                      | ⬜ **not configured** — requires Apple Developer Program membership (paid)                                                                                                                |
| Windows Authenticode signing                                   | ⬜ **not configured** — requires a code-signing certificate (paid)                                                                                                                        |
| Linux artifact GPG signature                                   | ⬜ under consideration; checksums first                                                                                                                                                   |
| Reproducible builds                                            | ⬜ **not claimed.** Toolchain is pinned (`rust-toolchain.toml`, lockfiles), which is a precondition, but bit-for-bit reproducibility is untested — it will not be claimed until verified. |

Consequence users will see: unsigned artifacts trigger macOS Gatekeeper and
Windows SmartScreen warnings. Release notes will say so plainly rather than
instructing users to blindly bypass warnings. Signing is a named use of any
future project funding ([FUNDING.md](FUNDING.md)).

## Verifying a download (once releases exist)

```sh
# 1. Get the checksums file from the same GitHub release
# 2. Verify your artifact:
shasum -a 256 -c SHA256SUMS.txt --ignore-missing   # macOS
sha256sum -c SHA256SUMS.txt --ignore-missing        # Linux
```

Checksums verify _integrity_ (the file wasn't corrupted or swapped after
upload), not _identity_ — identity requires signing/attestations, per the
table above. Documentation here will be updated the moment stronger
verification is actually available, and not before.

## Credential handling (policy)

- Signing keys/certificates, when they exist, live only in GitHub Actions
  secrets (or platform keychains), never in the repository. No exceptions.
- No workflow echoes secrets; release workflows keep `permissions` minimal
  and third-party actions SHA-pinned ([SUPPLY_CHAIN.md](SUPPLY_CHAIN.md)).
- A leaked credential is a security incident: rotate first, investigate
  second, disclose in an advisory if artifacts could be affected.
