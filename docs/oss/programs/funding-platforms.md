# Funding platforms (thanks.dev, Polar, Ko-fi, Patreon, Liberapay, others)

Status: CURRENT (Last verified: 2026-08-21)

## Program

This file covers payment rails, not grant programs. The distinction is the whole point: a grant program has a reviewer, a decision and an award; a platform has a checkout page and gives you exactly as much money as your audience chooses to send. For a project with zero users, every platform in this file pays zero. GitHub's FUNDING.yml supports these natively (see the sponsors-and-hosting notes file).

## Official URL

https://thanks.dev/ | https://polar.sh/ | https://ko-fi.com/ | https://www.patreon.com/ | https://liberapay.com/ | https://www.buymeacoffee.com/ | https://tidelift.com/ | https://issuehunt.io/

## What it funds

Nothing is "funded" — money is routed. Two mechanisms are worth distinguishing:

- Dependency-graph platforms (thanks.dev, Tidelift): companies pay, and the platform allocates automatically across the dependency trees of their codebases. thanks.dev scans sponsor repos three levels deep and distributes proportionally to how often a project appears as a dependency. Corporate donors have included Sentry and Canonical.
- Audience platforms (Ko-fi, Patreon, Liberapay, Buy Me a Coffee, GitHub Sponsors): individuals choose to pay you. Revenue tracks reputation and user count.
- Polar has repositioned as a billing/merchant-of-record stack for usage-based and subscription products (metering, invoicing, tax remittance in 100+ markets), with a startup program offering its Scale plan free for a year to early-stage companies. It is now closer to commercial billing infrastructure than to open source donations, though it remains a supported FUNDING.yml target.

## Eligibility

Generally open: a public repo and an account. thanks.dev requires maintainers to actively register to receive funds — the algorithm may find the project, but unregistered maintainers are not paid. Most platforms require the usual identity, bank and tax onboarding via Stripe or equivalent.

## Typical funding

UNKNOWN and demand-driven. No platform guarantees or ranges anything. Published thanks.dev examples of per-project distributions run to single-digit and low-double-digit dollars per donor per period. Do not model revenue from these.

## Application requirements

Account creation, identity/bank verification, and a FUNDING.yml entry. No review, no proposal.

## Sill fit

LOW across the board, and structurally near-zero for the dependency-graph platforms specifically. thanks.dev and Tidelift pay projects that appear inside other companies' dependency trees; a desktop terminal application is a leaf node that nobody imports, so it will never surface in a dependency scan no matter how popular it becomes. That is a permanent structural mismatch, not a maturity issue. Audience platforms are viable in principle but produce nothing until Sill has users who like it.

## Missing evidence

- Users. That is the entire prerequisite for every audience platform.
- For thanks.dev/Tidelift: a published, importable library — which Sill is not.

## Application strategy

Pick one audience rail and stop. GitHub Sponsors is the right one because it is where developers already are and because other programs (notably the GitHub Secure Open Source Fund) pay through it. Adding Ko-fi, Patreon, Polar and Liberapay alongside it splits attention and looks like a project asking for money before it has shipped anything. Skip thanks.dev registration for Sill; revisit only if a reusable library is ever extracted and published to a package registry.

## Deadline

None — all rolling.
