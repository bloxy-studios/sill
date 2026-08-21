# Open Source Model

_How Sill is open source — license, contributions, governance, money.
Application-oriented summary; canonical documents linked throughout._

## License

**MIT OR Apache-2.0** (user's choice), the Rust ecosystem standard — OSI-
approved, permissive, with Apache-2.0 providing an explicit patent grant.
No source-available hedging, no "open core" carve-outs in the license, no
CLA enabling future relicensing. Rationale: [../LICENSING.md](../LICENSING.md),
[ADR-0002](../decisions/0002-dual-license.md).

## Contributions

Inbound = outbound (contributions land under the project licenses), no CLA,
no DCO ritual ([ADR-0003](../decisions/0003-contribution-licensing.md)).
Contributor experience is invested in ahead of contributors:
[CONTRIBUTING.md](../../CONTRIBUTING.md), a 30-minute
[quickstart](../CONTRIBUTOR_QUICKSTART.md), one-command setup and check
scripts, CI parity with local checks, issue forms, and a documented
[triage](../ISSUE_TRIAGE.md)/[review](../PR_REVIEW.md) process.

## Governance

Maintainer-led, stated plainly, with public decision records (ADRs), an RFC
process for substantial changes, and a documented evolution path toward
multi-maintainer governance — no fictional committees.
[GOVERNANCE.md](../../GOVERNANCE.md).

## Security

Private vulnerability reporting, response targets, design-stage threat
model, scanning and supply-chain controls in CI.
[SECURITY.md](../../SECURITY.md), [SECURITY summary](SECURITY.md).

## Money

Currently: none, and none claimed. Policy set in advance: funding buys
maintainer time and infrastructure, never influence — sponsors get
recognition, not governance, security priority, or roadmap placement
([../SPONSORSHIP.md](../SPONSORSHIP.md)). Any future commercial services
would sit _beside_ the complete open-source core, never as a license
carve-out ([../SUSTAINABILITY.md](../SUSTAINABILITY.md)). Funding state and
plan: [../FUNDING.md](../FUNDING.md), [FUNDING_PLAN.md](FUNDING_PLAN.md).

## Honesty as policy

The repository maintains an evidence system ([../evidence/](../evidence/))
whose rules prohibit fabricated metrics, backdated entries, and claims
exceeding evidence — the same discipline this application package follows.
