# ADR-0005: Tag-driven draft releases via CI

- **Status:** Accepted
- **Date:** 2026-08-21

## Context

Releases must be reproducible by CI (not hand-built on a laptop), reviewable
before publication, and honest about their integrity guarantees (code signing
is not yet configured).

## Decision

- Semantic versioning; pre-1.0 with `-alpha.N` / `-beta.N` pre-release tags.
  No 1.0 until the API/config surface has stability worth promising.
- Pushing a `v*` tag triggers the release workflow: cross-platform build
  matrix → **draft** GitHub Release with artifacts + SHA-256 checksums.
- A maintainer inspects the draft and publishes manually. Nothing publishes
  itself.
- macOS notarization / Windows Authenticode signing are **not yet configured**;
  their absence is documented rather than papered over
  ([RELEASE_SECURITY.md](../RELEASE_SECURITY.md)).

Full procedure: [docs/RELEASING.md](../RELEASING.md).

## Alternatives considered

Manual local builds (not reproducible, single-machine trust), fully automatic
publish-on-tag (no human gate on a security-sensitive artifact), release
branches (overkill pre-1.0).

## Consequences

- ✅ Every artifact traceable to a tag + CI run; human gate before publication.
- ⚠️ Unsigned artifacts trigger OS warnings (Gatekeeper/SmartScreen) until
  signing is funded and configured — stated openly in release notes.
