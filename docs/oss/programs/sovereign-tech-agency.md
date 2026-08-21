# Sovereign Tech Agency (Sovereign Tech Fund)

Status: CURRENT (Last verified: 2026-08-21)

## Program

German government-backed agency (formerly Sovereign Tech Fund) that invests in open digital base technologies underpinning Germany's and Europe's digital infrastructure. It funds globally, not just in the EU. It runs several programs: the Fund (open applications), Resilience (vulnerability management services), Fellowship (paying maintainers of critical components directly), Standards (pilot, applications were May 2026, pilot runs June 2026 to June 2027), and Challenge (closed since 2023).

## Official URL

https://www.sovereign.tech/programs/fund (programs index: https://www.sovereign.tech/programs)

## What it funds

Development and maintenance of "open digital base technologies" that are vital to building other software or enabling digital networking. Stated examples: programming language libraries, package managers, open implementations of communication protocols, administration tools for developers, encryption technologies. Security audits and community events can be included if necessary to the main work.

## Eligibility

Two hard exclusions apply directly to Sill:

- "We do not finance the development of prototypes."
- "We are currently not looking for user-facing applications, such as messaging apps or file storage services."
  Other requirements: submission only via the online portal; German or English; project cost must exceed €50,000 (current minimum); no double-funding from other public entities for the same activities; all code and docs under OSI-approved or FSF Free/Libre licenses, with documentation licenses free of non-commercial and no-derivatives clauses.
  Review criteria: prevalence (how widely other technologies depend on it), relevance to societal sectors, vulnerability (underfunded critical work), public interest, feasibility of planned activities, and demonstrated expertise plus standing in the technology's community.

## Typical funding

Minimum project cost €50,000 (lowered from €150,000 in 2024). No stated maximum on the pages reviewed. Sovereign Tech Challenge (closed) had up to €300,000 per four-month round; that is a different program.

## Application requirements

Account on the application platform, then a full written application. Multi-stage: basic-requirements screen, criteria review, scoping phase with a program manager, external expert consultation, legal compliance and contracting. Response within ~10 weeks; roughly 6 months from submission to potential contract start. The agency explicitly will not pre-advise on whether a project is in scope.

## Sill fit

NOT ELIGIBLE. Sill fails on two independent, explicitly stated grounds: it is a prototype, and it is a user-facing application. Neither is fixed by gaining users — a desktop terminal application stays a user-facing application. The only realistic future path is to extract a genuinely infrastructural component from Sill (a protocol, a library, a session/PTY layer that other terminals and tools depend on), ship it separately, get real downstream adopters, and apply for that component — not for Sill.

## Missing evidence

- A separable, reusable base-technology component with independent downstream users.
- Evidence of prevalence: named projects or organizations that depend on it.
- Standing in the relevant community, or endorsement from its maintainers.
- A ≥€50,000 scope of maintenance/development work with credible costing.

## Application strategy

Do not apply for Sill as a product, now or later — it will be screened out. Revisit only if a spun-out infrastructure component gains real dependents. Watch the Sovereign Tech Fellowship as a separate, possibly better-shaped path if the maintainer ever becomes a maintainer of a critical component.

## Deadline

Fund applications accepted on a rolling basis via the portal. Standards pilot intake was May 2026 (closed).
