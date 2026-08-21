# Bus Factor

Sill's bus factor is **1**. One maintainer holds all knowledge, access, and
authority. Pretending otherwise would be silly; planning around it is this
document's job.

## Critical knowledge areas & their mitigation

| Area                          | Where knowledge lives outside the maintainer's head                                                                                                           |
| ----------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Why the stack looks like this | [ADRs](decisions/), [ARCHITECTURE.md](ARCHITECTURE.md)                                                                                                        |
| How to build/develop          | [CONTRIBUTING.md](../CONTRIBUTING.md), [CONTRIBUTOR_QUICKSTART.md](CONTRIBUTOR_QUICKSTART.md), `scripts/bootstrap.sh` — periodically tested on clean machines |
| How releases are cut          | [RELEASING.md](RELEASING.md) + automated `release.yml` (process is executable, not tribal)                                                                    |
| Security handling             | [SECURITY.md](../SECURITY.md), [threat model](SECURITY_THREAT_MODEL.md)                                                                                       |
| Project direction             | [ROADMAP.md](../ROADMAP.md), [PROJECT_STATUS.md](PROJECT_STATUS.md)                                                                                           |
| Operational settings          | [MAINTAINERS.md](../MAINTAINERS.md) admin checklist                                                                                                           |

The rule that makes this work: **if doing something required knowledge that
wasn't written down, writing it down is part of finishing the task.**

## Access single-points-of-failure (open risks)

- GitHub account `bloxy-studios` controls repo, releases, future funding
  accounts. Mitigations available now: hardware-key 2FA, recovery codes
  stored offline. Mitigation that needs people: a second owner — see below.
- Future signing certificates/keys: stored so that loss ≠ project death
  (documented storage + revocation path required before acquisition).

## Path to bus factor ≥ 2

1. Cultivate regular contributors (real `good first issue`s, fast review,
   credited work).
2. Reviewer role in one area (e.g. docs or frontend) after sustained quality.
3. Second maintainer with merge rights; security/release authority follows
   trust ([MAINTAINERS.md](../MAINTAINERS.md) describes the mechanics).
4. At that point: two-maintainer review for security-sensitive paths, shared
   release duty, documented emergency access.

## Continuity worst case

Everything needed to fork and continue Sill is public by design: code,
licenses (MIT OR Apache-2.0 — no CLA hostage-taking), decisions, processes,
and this file. A dead upstream would be a loss, not a lock-in. That is
intentional and is the honest floor under every sustainability promise this
project makes.
