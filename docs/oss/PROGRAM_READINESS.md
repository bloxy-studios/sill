# Program Readiness Assessment

_Self-assessment against what OSS programs actually evaluate. Scale:
READY / PARTIAL / MISSING / NOT APPLICABLE. Assessed 2026-08-21 — re-assess
before any application and after each roadmap phase._

| Category                   | Status                    | Why — and what changes it                                                                                                                                                |
| -------------------------- | ------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Repository                 | **READY**                 | Community-health complete: README, licenses, contributing, CoC, security policy, support, governance, templates, CODEOWNERS. GitHub community-profile items all present. |
| Documentation              | **READY**                 | Architecture + 8 design docs + ADR/RFC process + quickstart; kept honest with PROPOSED markers. Docs _website_: MISSING (planned, not needed yet).                       |
| Governance                 | **READY** (for its stage) | Maintainer-led, documented, with evolution path. Programs wanting multi-maintainer governance: MISSING until bus factor ≥ 2.                                             |
| Security                   | **PARTIAL**               | Policy, threat model, scanning, pinned CI: strong for day zero. Missing: track record, Scorecard history, signed releases, any external review.                          |
| Release engineering        | **PARTIAL**               | Pipeline + checksums + draft-gate defined and CI-validated; but **zero releases executed** — unexercised pipeline ≠ release history.                                     |
| Technical maturity         | **MISSING**               | The product doesn't exist yet. This is the blocking gap for every competitive program. Changes with Phase 1 shipped + benchmarks.                                        |
| Community                  | **MISSING**               | One person. Changes with first external contributors and sustained activity.                                                                                             |
| Impact / adoption evidence | **MISSING**               | Zero users, zero downloads. Changes with releases + verifiable usage.                                                                                                    |
| Sustainability             | **PARTIAL**               | Plans and policies written; no funding infrastructure active (Sponsors not set up), bus factor 1.                                                                        |
| Funding model              | **PARTIAL**               | Policy + budget template + program research done; no accounts/fiscal host until eligibility facts (maintainer region) are confirmed.                                     |

## Bottom line

**Structurally ready; substantively not.** The repository would pass an
infrastructure screen today and fail every merit screen. Sequencing per the
[program research](PROGRAM_MATRIX.md): build Phase 1 → release → accumulate
2–3 months of snapshots → set up Sponsors (infrastructure, not competitive)
→ then apply to the small number of genuinely fitting programs. Applying
earlier spends credibility that pre-alpha projects don't have to spare.
