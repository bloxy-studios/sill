# OSS Readiness Report

_Consolidated verdict on Sill as an open-source project. Written 2026-08-21,
at the completion of the open-source foundation work. Successor audits
supersede this one. Scores are deliberately conservative; inflating them
would defeat the document's purpose._

## Executive summary

Sill is now a **structurally complete, honestly presented open-source
project at day zero of its product**. Everything a reviewer, contributor, or
funder needs in order to understand, build, trust, and evaluate the project
exists and is truthful. What does not exist is the product, users, community,
and track record — and every document in this repository says so rather than
disguising it.

**Verdict on public presentation:** Sill **can** be presented publicly today
as what it is — an infrastructure-first, pre-alpha open-source project with
a clear thesis — and **cannot** honestly be presented as a working terminal,
to users or to funders, until Phase 1 ships. Announce quietly if at all;
announce loudly when there's a build to try.

## Scorecard (0 = missing · 1 = weak · 2 = partial · 3 = strong · 4 = excellent)

| Category                              | Score | One-line justification                                                                                 |
| ------------------------------------- | ----- | ------------------------------------------------------------------------------------------------------ |
| Technical (product)                   | **0** | The terminal does not exist yet; scaffold + verified build pipeline only                               |
| Documentation                         | **4** | Architecture, designs, ADRs, processes, quickstart — complete and honest for this stage                |
| Repository / community infrastructure | **4** | All community-health files, forms, labels policy, triage/review processes                              |
| Governance                            | **3** | Documented, honest, with evolution path; capped below 4 while bus factor = 1                           |
| Security                              | **3** | Policy + design-stage threat model + scanning + pinned CI; capped: no track record, no signed releases |
| Release engineering                   | **2** | Full pipeline defined and lint-validated; zero releases executed                                       |
| Community                             | **0** | One person. Infrastructure ≠ community                                                                 |
| Impact / adoption                     | **0** | None; framework + zero-baseline exist                                                                  |
| Sustainability                        | **2** | Plans, scope discipline, knowledge externalized; no money, no second maintainer                        |
| Evidence / funding readiness          | **2** | Evidence system + researched program matrix; the evidence itself is empty                              |

**Total: 20/40** — an honest picture of a well-built empty vessel.

## Biggest weaknesses, ranked

1. No product (blocks everything downstream)
2. Bus factor 1
3. No release history (pipeline unexercised)
4. Contested category (Warp/cmux) + name collisions (crates.io/npm `sill`
   taken; see audit §4.2) — positioning risk stays open; naming policy is
   now fixed ([ADR-0008](decisions/0008-naming-and-crate-policy.md))
5. Unsigned artifacts once releases start (funding-gated)

## Top 10 actions, in order

1. **Ship Phase 1**: terminal-engine spike → ADR-0006 decision → PTY →
   rendering → daily-drive exit criterion (everything else is downstream)
2. Enable repo settings that files can't set: `main` ruleset (require PR +
   status checks), private vulnerability reporting, secret scanning + push
   protection, Dependabot alerts, Discussions (categories per
   [COMMUNITY.md](COMMUNITY.md)), description/topics
   ([MAINTAINERS.md](../MAINTAINERS.md) checklist)
3. Create the label set from [LABELS.md](LABELS.md); seed 5–10 genuine
   `good first issue`s from real Phase 1 work
4. Verify `bun tauri dev` + the new strict CSP on a real dev machine
   (audit §6 caveat)
5. Complete [MAINTAINER_PROFILE.md](oss/MAINTAINER_PROFILE.md) personally;
   check GitHub Sponsors region eligibility (gates the funding plan) and, if
   German tax residency applies, note the Prototype Fund window
   (2026-10-01 → 11-30) — see [oss/PROGRAM_MATRIX.md](oss/PROGRAM_MATRIX.md)
6. Naming stance: **decided 2026-08-21**
   ([ADR-0008](decisions/0008-naming-and-crate-policy.md) — name kept; bare
   `sill`/`sill-*` never published; `sillterm-*` for branded internals).
   Remaining maintainer actions from it: acquire `sill.sh` (free at decision
   time — time-sensitive), run the trademark register check
7. Cut `v0.1.0-alpha.1` once Phase 1 is daily-driveable — exercising the
   release pipeline is itself a milestone; record it in evidence/
8. Take monthly metrics snapshots (founding baseline exists: 2026-08);
   maintain [PROJECT_STATUS.md](PROJECT_STATUS.md) honestly
9. Add parser fuzzing when the engine lands (threat model T1 commitment);
   read the first Scorecard results into [OPENSSF.md](OPENSSF.md) unedited
10. Revisit funding only per the sequencing in
    [oss/PROGRAM_MATRIX.md](oss/PROGRAM_MATRIX.md) — infrastructure now,
    applications after evidence

## Standing instruction

Every future claim of progress belongs in [evidence/](evidence/) with a
link, or it didn't happen. This report's scores get re-issued — not edited —
as reality changes.
