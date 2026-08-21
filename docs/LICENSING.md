# Licensing

Sill is dual-licensed under **MIT OR Apache-2.0**, at your option. This is a
deliberate decision, recorded in
[ADR-0002](decisions/0002-dual-license.md); the reasoning is summarized here.

## What "MIT OR Apache-2.0" means

You may use, modify, and redistribute Sill under the terms of _either_ the
[MIT license](../LICENSE-MIT) _or_ the
[Apache License 2.0](../LICENSE-APACHE) — your choice. This is the standard
licensing model of the Rust ecosystem (used by Rust itself, Tauri, and most
foundational crates).

## Why this license

Evaluated options and the reasoning:

| Option                                                | Assessment                                                                                                                                                                                                                                          |
| ----------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| MIT only                                              | Maximally familiar and simple (used by WezTerm, Zellij, Ghostty), but carries no explicit patent grant.                                                                                                                                             |
| Apache-2.0 only                                       | Explicit patent grant and contribution definition (used by Alacritty), but GPLv2-incompatible and less universally "frictionless" than MIT.                                                                                                         |
| **MIT OR Apache-2.0**                                 | Users who need simplicity take MIT; users who need explicit patent protection take Apache-2.0. Matches Rust ecosystem norms, so Rust contributors already understand it. Keeps future extraction of Sill internals into crates trivial. **Chosen.** |
| GPL/AGPL family                                       | Legitimate model, but wrong fit: Sill wants its session/agent integration surfaces adopted by other tools, including permissively-licensed and commercial ones.                                                                                     |
| Custom / source-available (BUSL, SSPL, "fair source") | Rejected. Sill claims to be open source; those licenses are not OSI-approved open source, and claiming otherwise would be dishonest.                                                                                                                |

Trade-off accepted knowingly: GitHub's license detector displays multi-license
repositories less cleanly than a single `LICENSE` file. Correct licensing wins
over a tidy badge.

## Contributions

Contributions are accepted under the same terms (inbound = outbound):

> Unless you explicitly state otherwise, any contribution intentionally
> submitted for inclusion in Sill by you shall be dual-licensed as MIT OR
> Apache-2.0, without any additional terms or conditions.

This is the Apache-2.0 §5 mechanism plus the standard Rust-ecosystem clause.
**No CLA** and **no DCO sign-off** are required — see
[ADR-0003](decisions/0003-contribution-licensing.md). If the project ever
needs stronger provenance (e.g. at significantly larger scale), DCO would be
proposed via RFC before adoption; a CLA is not planned.

## Per-file headers

Not required. The repository-level license files plus `Cargo.toml` /
`package.json` metadata are authoritative. Files vendored from elsewhere must
retain their original headers and be recorded in
[docs/THIRD_PARTY.md](THIRD_PARTY.md).

## Dependency licensing

Dependency licenses are enforced in CI with `cargo-deny` (see `deny.toml`) and
inventoried in [docs/THIRD_PARTY.md](THIRD_PARTY.md). Copyleft licenses that
would change Sill's effective terms (GPL, AGPL, SSPL) are not accepted as
dependencies; see [docs/DEPENDENCY_POLICY.md](DEPENDENCY_POLICY.md).
