# Governance (Application Summary)

_Canonical document: [GOVERNANCE.md](../../GOVERNANCE.md)._

**Model:** maintainer-led; one maintainer (Abdul Ali, @bloxy-studios) holds
decision, release, and security authority. Stated without decoration because
reviewers can count contributors themselves.

**What makes it accountable despite being solo:**

- All changes via PR; decisions and reasoning public
- Architecture Decision Records for consequential choices
  ([../decisions/](../decisions/)) and an RFC process for substantial ones
  ([../rfcs/](../rfcs/))
- Written role ladder (user → contributor → reviewer → maintainer), earned
  not granted; maintainer runbook + admin checklist
  ([MAINTAINERS.md](../../MAINTAINERS.md))
- Sponsor influence explicitly barred from technical/security decisions
  ([../SPONSORSHIP.md](../SPONSORSHIP.md))
- Bus-factor risk documented with a concrete mitigation path
  ([../BUS_FACTOR.md](../BUS_FACTOR.md)); permissive license + public process
  means the community can always fork — lock-in is structurally impossible

**Evolution path** (each step a public ADR): maintainer-led → small
maintainer group with area ownership → formalized governance → foundation/
fiscal host only if scale demands. No stage is claimed before it exists.
