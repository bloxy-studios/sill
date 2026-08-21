# Funding Plan (Budget Template)

_Every number below is an ESTIMATE for planning, marked as such. Nothing is
committed, promised, or currently funded. Before quoting any figure in an
application: re-price it and fill the [UPDATE] fields._

## Current financial state (2026-08-21)

Income: $0. Expenses: $0 cash (GitHub free tier for public repos covers CI;
development on the maintainer's existing hardware). Cost today is unpaid
maintainer time.

## What money would buy, in priority order

| #   | Item                                                                  | Estimate (annual unless noted)                             | Why it's this high on the list                                                                                                           |
| --- | --------------------------------------------------------------------- | ---------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------- |
| 1   | macOS signing: Apple Developer Program                                | ~$99/yr [ESTIMATE]                                         | Unsigned artifacts = Gatekeeper warnings; single biggest trust improvement per dollar ([../RELEASE_SECURITY.md](../RELEASE_SECURITY.md)) |
| 2   | Windows Authenticode certificate                                      | ~$200–500/yr [ESTIMATE — varies widely by CA/OV type]      | Same, for SmartScreen                                                                                                                    |
| 3   | Cross-platform test hardware (one Windows x64 machine; ARM Linux SBC) | ~$800–1,500 one-time [ESTIMATE]                            | Maintainer develops on macOS; real hardware beats CI-only testing for a terminal (input latency, rendering)                              |
| 4   | Maintainer development time                                           | rate/hours [UPDATE BEFORE APPLICATION — program-dependent] | The actual constraint on every roadmap phase; grants that fund time fund the roadmap                                                     |
| 5   | Security response reserve                                             | modest fixed sum [ESTIMATE]                                | Out-of-band advisory releases cost focused days                                                                                          |
| 6   | Infrastructure margin (CI overage, domain, docs hosting)              | ~$100–300/yr [ESTIMATE]                                    | Public-repo CI is free today; this covers growth and a docs site                                                                         |
| 7   | Design (icon/brand, once)                                             | ~$500–1,500 one-time [ESTIMATE]                            | After the terminal works, not before                                                                                                     |
| 8   | Contributor support (bounties/recognition)                            | only under a documented program [NOT PLANNED YET]          | Requires governance care; see [../SPONSORSHIP.md](../SPONSORSHIP.md)                                                                     |

Deliberately absent: paid marketing, growth tooling, conference sponsorships
— not until users exist and ask for representation.

## Allocation principles

- Time > things: past items 1–3, funding buys development/security/review
  time on the public roadmap.
- Transparency: if money arrives, received/allocated summaries are published
  per [../FUNDING.md](../FUNDING.md); private financial detail is not.
- No dependency: the project must survive any single source disappearing
  ([../SUSTAINABILITY.md](../SUSTAINABILITY.md)).

## Grant-sizing guidance (for future applications)

Small (≤ $10k): items 1–3 + a defined slice of Phase 1/2 time.
Medium ($10–50k): above + sustained maintainer time across a full roadmap
phase with named deliverables. Larger: not credible for a solo pre-alpha
project; don't ask.
