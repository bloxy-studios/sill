# Evidence Packet

_Curated index of the strongest available evidence, by application section.
Links into [../evidence/](../evidence/); duplicates nothing. Refresh before
every application._

**State of the packet (2026-08-21): nearly empty, honestly.** The project is
day-zero; the only evidence that exists is the repository itself. Sections
below name what belongs in them so accumulation is deliberate.

| #   | Section              | Strongest current evidence                                                                                     | What would upgrade it                                                     |
| --- | -------------------- | -------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------- |
| 1   | Project              | The repository: complete OSS infrastructure at day zero (this PR/commit history)                               | Sustained commit history through Phase 1                                  |
| 2   | Technical innovation | Design docs + ADRs (design-stage only — cite as design)                                                        | Working Phase 1 terminal; published engine evaluation (ADR-0006 accepted) |
| 3   | Open-source activity | Founding snapshot ([snapshots/2026-08.md](../evidence/snapshots/2026-08.md))                                   | Monthly snapshots showing motion; merged external PRs                     |
| 4   | Community            | None (community = 1)                                                                                           | First external contributors; discussion activity                          |
| 5   | Adoption             | None (no releases)                                                                                             | Release download counts; packaging (Homebrew formula name verified free)  |
| 6   | Security             | Posture artifacts: threat model, cargo-deny green, pinned CI ([evidence/SECURITY.md](../evidence/SECURITY.md)) | Scorecard results over time; handled advisories; external security review |
| 7   | Performance          | Budgets + planned harness only — **no numbers exist; cite none**                                               | Reproducible benchmark results in benchmarks/results/                     |
| 8   | Ecosystem            | None                                                                                                           | Tested agent compatibility matrix entries; third-party integrations       |
| 9   | Impact               | None beyond design rationale                                                                                   | Documented real-workflow usage; protocol adoption                         |
| 10  | Sustainability       | Written plans (SUSTAINABILITY, BUS_FACTOR, FUNDING_PLAN)                                                       | Sponsors/fiscal host active; second maintainer                            |

Rule of use: a section with "None" above contributes **nothing** to an
application — do not pad it with adjacent material. The gaps are the to-do
list ([PROGRAM_READINESS.md](PROGRAM_READINESS.md) tracks them).
