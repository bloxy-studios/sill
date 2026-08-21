# Releasing

How a Sill release is produced. Authority: maintainers only
([MAINTAINERS.md](../MAINTAINERS.md)). Decision record:
[ADR-0005](decisions/0005-release-process.md).

**Status: no release has been published yet.** This process is defined ahead
of the first tag so that release #1 is already disciplined.

## Versioning

Semantic versioning, pre-1.0:

- `0.MINOR.PATCH` — MINOR for features/breaking changes (allowed pre-1.0),
  PATCH for fixes
- Pre-releases: `v0.2.0-alpha.1`, `-beta.1`, `-rc.1`
- 1.0 happens when the config/API surface is stable enough to promise —
  not for marketing

Version lives in three files that must agree: `package.json`,
`src-tauri/Cargo.toml`, `src-tauri/tauri.conf.json`.

## Release procedure

1. **Prepare**
   - `main` green in CI; no unresolved `priority:critical` issues
   - Update the three version fields; move `CHANGELOG.md` _Unreleased_ items
     under the new version heading (Added/Changed/Fixed/Removed/Security/
     Performance/Breaking Changes — omit empty sections)
   - PR titled `release: vX.Y.Z`; merge after CI
2. **Tag**
   ```sh
   git checkout main && git pull
   git tag -a vX.Y.Z -m "Sill vX.Y.Z"
   git push origin vX.Y.Z
   ```
3. **CI builds** (`release.yml`): cross-platform matrix → artifacts + SHA-256
   checksums → **draft** GitHub Release. Nothing publishes automatically.
4. **Verify the draft** — the human gate:
   - [ ] All platform artifacts present, checksums file attached
   - [ ] Install/launch smoke test on at least macOS + one other platform
   - [ ] Release notes written from the template
         ([.github/RELEASE_TEMPLATE.md](../.github/RELEASE_TEMPLATE.md)) — include
         known issues and the current signing status honestly
5. **Publish** the release. Announce in Discussions → Announcements.
6. **Record**: add the release to
   [docs/evidence/RELEASE_HISTORY.md](evidence/RELEASE_HISTORY.md).

## Artifacts

Naming (produced by Tauri bundler): `sill_<version>_<platform-arch>.<ext>` —
`.dmg`/`.app.tar.gz` (macOS aarch64 + x86_64), `.AppImage`/`.deb` (Linux
x86_64), `.msi`/`.exe` (Windows x86_64). Plus `SHA256SUMS.txt` covering every
asset. Integrity/signing status: [RELEASE_SECURITY.md](RELEASE_SECURITY.md).

## Hotfixes

Pre-1.0: fix on `main`, release `0.Y.Z+1` from `main`. No release branches
until there are users whose older versions need parallel support — revisit
at 1.0.

## Rollback

Releases are never deleted (checksums must stay verifiable). A bad release is
marked in its notes ("superseded — do not use, see vX.Y.Z+1"), un-marked as
_Latest_, and followed by a fixed release. If the flaw is a vulnerability, a
security advisory accompanies it ([SECURITY.md](../SECURITY.md)).
