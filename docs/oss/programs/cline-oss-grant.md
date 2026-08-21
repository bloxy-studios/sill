# Cline Open Source Grant

Status: CURRENT (Last verified: 2026-08-21)

## Program

A vendor-run grant from Cline, an open source AI coding agent, announced after the project passed 5 million installs. Cline committed $1 million in Cline credits to fund open source projects. The application is a live Google Form on cline.bot. Cline states the funded project does not need to integrate with Cline or use its stack.

## Official URL

https://cline.bot/oss-grant (announcement: https://cline.bot/blog/5m-installs-1m-open-source-grant-program)

## What it funds

Per the announcement: developer tools that solve real problems, AI infrastructure that enables new workflows, agentic systems, and "anything else that makes building software better." Cline states it is "especially interested in projects from developers who haven't had access to traditional funding. Solo developers, small teams, people building in their spare time."

## Eligibility

Open source project with a recognised license (form offers MIT, Apache-2.0, GPL-3.0, BSD-3-Clause, MPL-2.0, or other). No stated geographic restriction, no maturity floor stated. The form collects GitHub username, project link, GitHub star count, license type, and estimated monthly API costs — the last two are described as inputs to grant tiering.

## Typical funding

$1,000 to $10,000 per project, awarded in Cline credits, not cash. This is important: the grant offsets LLM API spend, it does not pay for maintainer time. The blog states amounts depend on scope and ambition; the form says amounts are tailored to project and team.

## Application requirements

Short Google Form: name, email, GitHub username, X username, project link, star count, license, estimated monthly API costs, and free-text description. Reviewed on a rolling basis; the announcement said the first round of recipients would be named within 60 days.

## Sill fit

LOW now, MEDIUM once there is a repo worth linking. Thematically Sill is a direct hit — developer tooling for the agentic-coding era, built by a solo maintainer with no traditional funding access, which is the stated target profile. The problem is the tiering inputs: GitHub stars (0) and monthly API costs (0, since Sill is provider-neutral and does not itself spend on inference). Credits also have limited value to a project whose costs are developer time, not tokens. Application cost is minutes, so the expected value is still positive — just do not count on it and do not treat credits as funding.

## Missing evidence

- A repo with actual functionality behind the link, so the free-text answer is verifiable.
- Any star count above zero.
- A real answer to "estimated monthly API costs" — which only exists if Sill's own development or CI consumes model APIs.
- A concrete statement of what the credits would be used for.

## Application strategy

Apply once Sill has a working prototype and a nonzero star count — a few months of real development, not now. Frame the ask around credits actually consumed in developing and testing agent-session handling, since that is the only honest use. Vendor credit grants like this are the least durable category on the list; the program could end without notice.

## Deadline

None stated — rolling, applications remain open.
