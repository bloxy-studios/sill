# GitHub Secure Open Source Fund

Status: CURRENT (Last verified: 2026-08-21)

## Program

A GitHub-run, sponsor-funded program that pays maintainers to do concrete security work on their projects, delivered as a three-week security education sprint plus 6- and 12-month check-ins. It is explicitly the successor to the security experiment GitHub ran inside GitHub Accelerator. Sessions run in cohorts; Session 4 invested more than $500,000 across 50 projects.

## Official URL

https://resources.github.com/github-secure-open-source-fund/

## What it funds

Maintainer time spent on measurable security improvements: threat modelling, dependency hygiene, secrets management, CI hardening, security policy, incident response planning. Funding is tied to verified security outcomes, not to feature development.

## Eligibility

- Current maintainer of an open source project; may apply as a team of up to 3.
- Must be able to commit ~15 hours across the 3-week sprint plus 2.5 hours at each of the 6- and 12-month check-ins (20 hours total). Meetings scheduled in Pacific time.
- Paid through GitHub Sponsors, so GitHub Sponsors region eligibility applies transitively.
- Selection is explicitly weighted toward "important, fast growing projects" and fast-growing dependencies that larger projects rely on. Funders and GitHub can refer projects in.

## Typical funding

$10,000 per project, split $6,000 during the sprint, $2,000 at the 6-month check-in, $2,000 at the 12-month check-in. Plus $10,000 in Azure credits; the page states eligible projects have potential to receive up to $150,000 in Azure credits via Microsoft for Startups.

## Application requirements

Online application form on the program page. Applications are open on a rolling basis and are considered for all program sessions. Selected applicants get a virtual interview.

## Sill fit

NOT YET. The selection logic is "projects other projects depend on" — Sill is a leaf-node desktop application with no dependents, no users, and no shipped code, so there is no security surface to fund. Apply after Sill has real releases, a real user base, and a plausible answer to "what breaks downstream if this is compromised". A GitHub blog post dated 2026-08-13 advertised a Session 5 deadline of August 24; ignore it, the rolling intake means nothing is lost by waiting.

## Missing evidence

- Shipped releases and a non-trivial install base.
- A dependency/supply-chain story: what Sill ships, signs, and auto-updates.
- Existing SECURITY.md, disclosure process, and at least a baseline of CI security hygiene to build on.
- Evidence that a compromise of Sill would matter to someone other than the maintainer.

## Application strategy

Realistic 12-24 months out. Because Sill is a terminal that will execute untrusted agent output and hold credentials, the security narrative is genuinely strong once the product exists — this is probably the single best-matched cash program on the list, just not yet. Track session cadence and re-check the page quarterly.

## Deadline

Rolling; applications considered for all future sessions. Individual sessions have their own cut-offs.
