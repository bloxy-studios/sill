# Open Source Audit

_Audit of the Sill repository, performed 2026-08-21 — the day the repository
was created — before and after the open-source foundation work landed. This
document is kept as the baseline record; future audits append._

## 1. State found (pre-foundation)

A pristine `create-tauri-app` scaffold, one commit ("Initial Commit"),
`main` only, no tags:

- Tauri 2 + React 19 + TypeScript 5.8 + Vite 7, Bun-managed (`bun.lock`)
- Rust side: template `greet` command only; `tauri-plugin-opener`;
  capabilities `core:default` + `opener:default`
- **No** README content (template stub), license, contributing guide, code
  of conduct, security policy, governance, CI, tests, issue/PR templates,
  dependency management, release process, or documentation of any kind
- Template defects: identifier `com.codewithabdul.sill` (wrong org),
  `csp: null`, `authors = ["you"]`, template `index.html` title
- No secrets found in tree or history (checked); `.gitignore` sane

**Maturity: day zero.** Not "early" — zero. Every statement in this
repository is calibrated to that fact.

## 2. Strengths (found)

- Sensible modern stack with committed lockfiles
- Clean history; no accumulated cruft to unwind
- Public repo from day one — everything below is built in the open

## 3. Weaknesses & risks (found → disposition)

| Finding                                               | Disposition                                                                                                                                              |
| ----------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------- |
| No license → legally unusable by anyone               | Fixed: MIT OR Apache-2.0 ([LICENSING.md](LICENSING.md), ADR-0002)                                                                                        |
| `csp: null` — webview unconfined                      | Fixed: strict CSP with Tauri IPC directives. _Caveat: verify rendering on a packaged build; set on documented values but not yet exercised by a GUI run_ |
| Wrong bundle identifier                               | Fixed: `com.bloxy-studios.sill` (before any release/signing made it expensive)                                                                           |
| No CI — nothing enforced                              | Fixed: CI (3-OS), security scanning, docs checks, scorecard, release pipeline — all SHA-pinned, least-privilege                                          |
| No security process for a security-sensitive category | Fixed: SECURITY.md + design-stage threat model + supply-chain controls                                                                                   |
| Zero tests                                            | Baseline unit tests added (Rust); real test surface arrives with real code — no test theater over a placeholder UI                                       |
| Solo maintainer / bus factor 1                        | Cannot be fixed by documents; documented honestly with mitigation path ([BUS_FACTOR.md](BUS_FACTOR.md))                                                  |

## 4. Risks that remain open (register)

1. **The product doesn't exist.** Every other gap is secondary. (Roadmap
   Phase 1.)
2. **Name collisions** (researched 2026-08-21): crates.io `sill` is taken —
   by an _AI-agent credential tool_ (sill-sh org; adjacent domain, same
   registry); npm `sill` taken (dormant); `sill.social` is an active,
   unrelated product holding search mindshare; "SILL" is a French-government
   FOSS-catalog acronym. Homebrew formula/cask names are free; no terminal
   named sill found; trademark registers not queried.
   **Disposition — decided 2026-08-21:** the name stays. Bare `sill` and
   `sill-*` are never published to any registry; ecosystem crates get
   standalone neutral names; branded internals use `sillterm-*`; canonical
   domain designated `sill.sh` (free at decision time). Policy, rationale,
   and binding rename-revisit triggers (before `v0.1.0-alpha.1` only):
   [ADR-0008](decisions/0008-naming-and-crate-policy.md). Still open inside
   that decision: acquire `sill.sh`; trademark register check.
3. **Category competition:** Warp (AGPL, 64k★) and cmux (GPL, 26k★) occupy
   "agent terminal" mindshare. Differentiation position recorded in
   [COMPETITIVE_LANDSCAPE.md](COMPETITIVE_LANDSCAPE.md); it is a bet, not a
   fact.
4. **Webview rendering risk** for a terminal grid — named with a kill
   criterion in ADR-0001.
5. **Unsigned releases** until funded ([RELEASE_SECURITY.md](RELEASE_SECURITY.md)).
6. **Unexercised pipelines:** release workflow and Linux-ARM leg are
   CI-validated but have never produced a real release.

## 5. Infrastructure delivered (post-foundation state)

Community health: README (honest hero + status), LICENSE-MIT/LICENSE-APACHE,
CONTRIBUTING, CODE_OF_CONDUCT (CC 3.0 with working report path), SECURITY,
SUPPORT, GOVERNANCE, MAINTAINERS, AUTHORS, ACKNOWLEDGMENTS, TRADEMARKS,
ROADMAP, CHANGELOG. GitHub: 6 issue forms + config (security redirected to
private reporting), PR template, release-notes template, CODEOWNERS, inert
FUNDING.yml, Dependabot (cargo/bun/actions, grouped weekly).

Engineering: CI (typecheck/lint/format/fmt/clippy/tests + 3-OS build),
security workflow (cargo-deny + dependency review + weekly schedule),
Scorecard workflow, docs link checks, tag-driven draft-release workflow with
SHA256SUMS; pinned Rust toolchain; `deny.toml` license allowlist; bootstrap +
check scripts giving CI/local parity; ESLint/Prettier/tsconfig enforced.

Documentation: architecture (current-vs-PROPOSED), 8 design docs, 7 ADRs +
RFC process, threat model (12 threat classes), performance budgets +
benchmark rules, releasing + release security, supply chain, dependency
policy, triage/review/labels, community plan, bus factor, sustainability,
competitive landscape, agent compatibility matrix (all-Planned, honest),
integration pages, contributor quickstart.

Funding/evidence: funding policy + plan, 17 researched program dossiers +
matrix (verified 2026-08-21), application package (overview, problem,
solution, innovation, impact, readiness, checklist, fact sheet, descriptions,
maintainer-profile skeleton), evidence system with founding zero-baseline
snapshot.

## 6. Verification performed (2026-08-21, in a clean Linux environment)

| Check                                                   | Result                                                                                                                                            |
| ------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------- |
| `bun install --frozen-lockfile` equivalent              | ✅                                                                                                                                                |
| `bun run typecheck` / `lint` / `format:check` / `build` | ✅ all pass                                                                                                                                       |
| `cargo fmt --check`                                     | ✅                                                                                                                                                |
| `cargo deny check` (advisories/licenses/bans/sources)   | ✅ green, with 16 documented ignores (Tauri's known unmaintained transitive GTK3/unic bindings — see `deny.toml`)                                 |
| `actionlint` on all workflows                           | ✅                                                                                                                                                |
| `cargo clippy` / `cargo test`                           | ⚠️ **not run locally** — audit environment (Amazon Linux) lacks webkit2gtk system libraries; these run in CI on Ubuntu. Verify on the PR's checks |
| GUI launch / CSP behavior                               | ⚠️ not runnable headless — verify `bun tauri dev` on a dev machine                                                                                |

## 7. Gap summary (what documents cannot fix)

Product (Phase 1) → releases → users → contributors → track record. In that
order. Detailed per-category assessment:
[FUNDING_READINESS.md](FUNDING_READINESS.md),
[oss/PROGRAM_READINESS.md](oss/PROGRAM_READINESS.md); consolidated verdict
and action list: [OSS_READINESS_REPORT.md](OSS_READINESS_REPORT.md).
