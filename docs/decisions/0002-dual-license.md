# ADR-0002: Dual license — MIT OR Apache-2.0

- **Status:** Accepted
- **Date:** 2026-08-21

## Context

Sill is developer infrastructure that wants: maximum adoption, contributor
familiarity, explicit patent safety for users, and the option to extract
internals (terminal engine pieces, agent protocol) into reusable crates later.
It also intends to _honestly_ call itself open source.

## Decision

License the project **MIT OR Apache-2.0** (user's choice), the Rust ecosystem
standard. Contributions accepted under the same terms (inbound = outbound).

## Alternatives considered

MIT-only (no patent grant), Apache-2.0-only (GPLv2-incompatible, slightly
higher friction), GPL family (conflicts with the goal of the integration
surface being adopted everywhere), source-available licenses (not open source;
rejected on honesty grounds). Full comparison: [docs/LICENSING.md](../LICENSING.md).

## Consequences

- ✅ Zero-surprise licensing for Rust contributors; patent grant available via
  Apache-2.0; trivially compatible with the dependency ecosystem.
- ⚠️ GitHub's license widget shows dual-licensed repos less cleanly. Accepted.
- Copyright: "Abdul Ali (Bloxy Studios) and Sill contributors"; no CLA means
  no single entity can relicense unilaterally later — this is a feature
  (contributor trust), accepted with eyes open.
