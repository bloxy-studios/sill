<!-- Template for release notes. Copy into the draft release, delete empty
     sections, keep the integrity notice honest. See docs/RELEASING.md. -->

## Highlights

<!-- 2-4 bullets a user actually cares about. No adjectives without evidence. -->

## Added

## Changed

## Fixed

## Performance

<!-- Only harness-produced numbers, with links to benchmarks/results/. -->

## Security

<!-- Advisories fixed (link), hardening shipped. -->

## Breaking changes

<!-- What breaks, who's affected, exact migration steps. -->

## Known issues

## Upgrade instructions

## Verifying your download

Checksums for every asset are in `SHA256SUMS.txt` attached to this release:

```sh
sha256sum -c SHA256SUMS.txt --ignore-missing   # Linux
shasum -a 256 -c SHA256SUMS.txt --ignore-missing   # macOS
```

⚠️ Artifacts are currently **unsigned** (no macOS notarization, no Windows
Authenticode) — your OS will warn accordingly. Status and plan:
[docs/RELEASE_SECURITY.md](https://github.com/bloxy-studios/sill/blob/main/docs/RELEASE_SECURITY.md).

## Contributors

<!-- Thank everyone with commits/reviews/repros in this release. -->
