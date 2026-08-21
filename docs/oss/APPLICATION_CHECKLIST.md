# Application Checklist

## Structural checklist (repository state)

- [x] Public repository
- [x] OSI-approved license (MIT OR Apache-2.0)
- [x] README with honest status
- [x] Contribution guide + contributor quickstart
- [x] Code of conduct (Contributor Covenant 3.0, working report path)
- [x] Security policy + threat model
- [x] Governance documented (maintainer-led, evolution path)
- [x] Public roadmap with non-goals
- [x] CI (lint/typecheck/fmt/clippy/tests/build, 3 platforms)
- [x] Security scanning (cargo-deny, Dependabot, dependency review, Scorecard workflow)
- [x] Dependency management policy + license enforcement
- [x] Architecture + design documentation (ADRs, RFC process)
- [x] Community processes (triage, review, labels, discussions plan)
- [x] Funding policy + budget template + program research
- [x] Maintainer profile skeleton
- [x] Impact metrics framework + founding baseline snapshot
- [x] Evidence log
- [x] Reproducible dev environment (pinned toolchains, bootstrap/check scripts)
- [x] Release process + integrity documentation
- [ ] **Releases exist** ← blocking
- [ ] **Working product (Phase 1)** ← blocking
- [ ] External contributors ← blocking for community-weighted programs
- [ ] Funding infrastructure active (Sponsors/fiscal host) — after region eligibility check
- [ ] Maintainer profile completed by maintainer
- [ ] 3+ months of metrics snapshots

## Update process — run before EVERY submission

1. Refresh [APPLICATION_DATA.md](APPLICATION_DATA.md) volatile fields from
   their sources of truth; date the refresh.
2. Take a fresh metrics snapshot into
   [../evidence/snapshots/](../evidence/snapshots/).
3. Re-read [EVIDENCE_PACKET.md](EVIDENCE_PACKET.md); strike any claim whose
   evidence you cannot click through to in 30 seconds.
4. Re-verify the target program's official page (deadline, eligibility,
   requirements) — treat [programs/](programs/) research as a map, not truth.
5. Confirm [PROGRAM_READINESS.md](PROGRAM_READINESS.md) status honestly;
   if the program's merit screen hits a MISSING row, reconsider applying.
6. Complete/refresh [MAINTAINER_PROFILE.md](MAINTAINER_PROFILE.md) sections.
7. Record the application (program, date, materials used) in
   [../evidence/EVIDENCE_LOG.md](../evidence/EVIDENCE_LOG.md).
