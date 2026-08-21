# Design: Release Architecture

**Status: ACTIVE — the CI/release pipeline described here exists in
`.github/workflows/`; signing and attestations do not yet.** Process view:
[RELEASING.md](../RELEASING.md); integrity honesty: [RELEASE_SECURITY.md](../RELEASE_SECURITY.md).

## Pipeline

```
git tag v0.X.Y
   │
   ▼
release.yml (GitHub Actions, permissions: contents:write only)
   │
   ├─ matrix build (tauri-action, SHA-pinned)
   │    macos-latest  --target aarch64-apple-darwin   → .app/.dmg
   │    macos-latest  --target x86_64-apple-darwin    → .app/.dmg
   │    ubuntu-22.04                                  → .deb/.rpm/.AppImage
   │    ubuntu-22.04-arm                              → .deb/.rpm/.AppImage (arm64)
   │    windows-latest                                → .msi/.exe
   │
   ├─ all artifacts → ONE draft GitHub Release (tauri-action)
   │
   ├─ checksums job: download draft assets → SHA256SUMS.txt → attach
   │
   ▼
human: verify draft per RELEASING.md checklist → publish
```

Design choices and why:

- **Draft-by-default** (`releaseDraft: true`): CI is allowed to build,
  never to publish. The publish click is the human integrity gate.
- **Checksums as a separate job** after all matrix legs finish: one
  `SHA256SUMS.txt` covering every asset, generated from what is _actually
  attached_ to the release, not from per-leg local files — so the checksum
  file can't drift from the artifacts.
- **SHA-pinned actions, minimal permissions**: the release workflow is the
  most attractive target in the repo ([SUPPLY_CHAIN.md](../SUPPLY_CHAIN.md)).
- **Linux ARM64 via `ubuntu-22.04-arm`**: documented by Tauri for public
  repos; treated as best-effort until exercised by a real release.
- **No auto-updater yet**: the Tauri updater ships only with signing
  (update integrity without signatures is theater). It joins the pipeline
  when [RELEASE_SECURITY.md](../RELEASE_SECURITY.md)'s signing rows flip on.

## Planned evolution (in order)

1. Artifact attestations (`actions/attest-build-provenance`) once first
   releases exercise the pipeline — public-repo free, verifiable via `gh
attestation verify`
2. SBOM generation per release (CycloneDX; see [SUPPLY_CHAIN.md](../SUPPLY_CHAIN.md))
3. macOS Developer ID signing + notarization; Windows Authenticode (funding-
   gated; [FUNDING.md](../FUNDING.md) names them as first uses of money)
4. Tauri updater artifacts + signature files (after 3)

Each step lands as its own reviewed PR with docs updated in the same change —
the release pipeline never claims more than it does.
