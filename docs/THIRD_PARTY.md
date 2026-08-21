# Third-Party Licenses

How Sill tracks the licenses of what it ships. Enforcement is automated
(`cargo-deny` in CI against the allowlist in `deny.toml`); this document
explains the state and how to regenerate the inventory. Nothing here is
hand-maintained per-crate — hand-maintained license lists rot.

## Rust dependency graph

_Inventory generated 2026-08-21 from `src-tauri/Cargo.lock` via
`cargo deny list` (all features, all platforms — the resolver includes
Windows/macOS/Linux-only crates, so counts exceed any single build)._

| License                        | Crates (count)                                                                                                       |
| ------------------------------ | -------------------------------------------------------------------------------------------------------------------- |
| MIT                            | 409                                                                                                                  |
| Apache-2.0                     | 294 (nearly all dual MIT OR Apache-2.0)                                                                              |
| Unicode-3.0                    | 19 (ICU/unicode data crates)                                                                                         |
| Zlib                           | 18                                                                                                                   |
| Apache-2.0 WITH LLVM-exception | 6                                                                                                                    |
| BSD-3-Clause                   | 6                                                                                                                    |
| Unlicense                      | 6 (all dual Unlicense OR MIT — satisfied via MIT)                                                                    |
| MPL-2.0                        | 5 (file-level copyleft; unmodified use — cssparser/selectors family)                                                 |
| 0BSD / CC0-1.0 / MIT-0         | 1 each (multi-licensed utility crates)                                                                               |
| LGPL-2.1-or-later              | 2 listings (`r-efi`, tri-licensed MIT OR Apache-2.0 OR LGPL — **consumed under MIT/Apache**, no copyleft obligation) |

Notes a reviewer would ask about:

- **No GPL/AGPL/SSPL/proprietary dependencies.** The `deny.toml` allowlist
  fails CI on anything outside the permissive set + MPL-2.0.
- Multi-licensed crates appear under every license they offer in raw `list`
  output; cargo-deny's _check_ evaluates the SPDX expression and selects an
  allowed route. All 600+ crate expressions currently pass.
- MPL-2.0 is accepted deliberately (file-level copyleft, compatible with
  MIT/Apache distribution when unmodified); modifying an MPL-licensed
  dependency in-tree would trigger its source-sharing terms — policy is
  don't; upstream instead.

## Regenerating

```sh
# summary by license (what produced the table above)
cargo deny --manifest-path src-tauri/Cargo.toml list -l license
# full check as CI runs it
cargo deny --manifest-path src-tauri/Cargo.toml check
```

## JavaScript dependency graph

Runtime JS dependencies are minimal: `react`, `react-dom` (MIT),
`@tauri-apps/api`, `@tauri-apps/plugin-opener` (MIT/Apache-2.0). Everything
else in `package.json` is devDependencies (build/lint tooling) and is not
distributed in artifacts. Inspect with `bun pm ls` against `bun.lock`.
JS additions follow the same [DEPENDENCY_POLICY.md](DEPENDENCY_POLICY.md)
questions; a JS license-audit step joins CI if/when the runtime JS graph
grows beyond this handful.

## Vendored code

None. If code is ever vendored, it keeps its original license header and
gets a row in this file with provenance.

## Acknowledgements

Human-readable thanks live in [ACKNOWLEDGMENTS.md](../ACKNOWLEDGMENTS.md);
this file is the compliance view.
