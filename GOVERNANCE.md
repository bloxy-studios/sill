# Governance

## Current model: maintainer-led

Sill is currently developed and maintained by a single maintainer,
[Abdul Ali](https://github.com/bloxy-studios). There is no foundation,
committee, or company behind the project, and this document does not pretend
otherwise.

While the project is maintainer-led:

- **Technical decisions** are made by the maintainer, in the open, informed by
  issues, discussions, and RFCs. Significant decisions are recorded as
  [Architecture Decision Records](docs/decisions/).
- **Code review**: all changes — including the maintainer's own — go through
  pull requests. External contributions require maintainer review and approval.
- **Releases** are cut and published by the maintainer. See
  [docs/RELEASING.md](docs/RELEASING.md).
- **Security reports** are handled privately by the maintainer. See
  [SECURITY.md](SECURITY.md).
- **Code of conduct enforcement** is the maintainer's responsibility. See
  [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md).

## Roles

| Role                | Meaning                                                    | How it is earned                      |
| ------------------- | ---------------------------------------------------------- | ------------------------------------- |
| User                | Uses Sill, files issues, joins discussions                 | —                                     |
| Contributor         | Has a merged pull request                                  | Contribute                            |
| Regular contributor | Multiple quality contributions over time                   | Sustained contribution                |
| Reviewer            | Reviews PRs in an area; opinions carry weight in that area | Demonstrated judgment and reliability |
| Maintainer          | Merge, release, and security authority                     | Invitation, based on sustained trust  |

Roles are earned through demonstrated work and trust. They are never granted
automatically, sold, or exchanged for sponsorship.

## Decision-making

- Small changes: pull request review.
- Significant changes (new subsystems, protocol/API surface, security-relevant
  behavior): open a Discussion or an [RFC](docs/rfcs/) first. See the
  [large feature policy](CONTRIBUTING.md#large-changes).
- Irreversible or architectural decisions get an ADR in
  [docs/decisions/](docs/decisions/).

Disagreements are resolved by discussion; the maintainer has the final call.
That is the honest trade-off of a young project — in exchange, decisions and
their reasoning are documented publicly.

## Sponsorship and influence

Funding sustains development; it does not buy decisions. Sponsors receive no
governance rights, no priority in security handling, and no guaranteed roadmap
placement. See [docs/SPONSORSHIP.md](docs/SPONSORSHIP.md).

## How governance evolves

The intended path as the project grows:

1. **Maintainer-led** (now) — single maintainer, public reasoning.
2. **Maintainer group** — 2–3 maintainers with area ownership (CODEOWNERS),
   two-person review for security-sensitive changes.
3. **Formal governance** — written decision process, elected/appointed
   maintainer group, documented conflict resolution.
4. **Foundation or fiscal host** — only if scale genuinely requires it.

Each transition will be proposed publicly and recorded as an ADR. Nothing in
this document should be read as a claim that later stages exist today.
