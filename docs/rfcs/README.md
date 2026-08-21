# RFCs

An RFC (request for comments) is how substantial changes to Sill get designed
in the open before implementation. It protects contributors from building the
wrong thing and protects the project from accidental architecture.

## When an RFC is required

- New subsystems (terminal engine swap, plugin system, remote development)
- Any public API, protocol, or configuration surface (CLI, events, MCP, agent
  integration)
- Security-model changes, new permission/capability grants
- Anything that would be painful to reverse

Not required for: bug fixes, refactors, docs, UI polish, dependency updates —
i.e. most PRs. When unsure, open a Discussion and ask; that costs nothing.

## Process

1. Open a GitHub Discussion (Ideas) to sanity-check the direction. Cheap.
2. Copy the template below into `docs/rfcs/NNNN-short-name.md` and open a PR.
3. Discussion happens on the PR. The maintainer accepts, rejects, or asks for
   revision — with reasons on the record.
4. Accepted RFCs are merged with status `Accepted`; the decision is summarized
   as an [ADR](../decisions/) if it is architectural. Implementation follows
   as normal PRs referencing the RFC.

## Template

```markdown
# RFC-NNNN: Title

- Status: Draft | Accepted | Rejected | Withdrawn
- Author(s):
- Date:

## Problem

What hurts, for whom, today.

## Motivation

Why solve it in Sill, and why now.

## Proposal

The design. Concrete enough to argue with.

## Alternatives

What else was considered, and why not.

## Security implications

Attack surface, trust boundaries, new inputs. "None" must be argued, not asserted.

## Performance implications

Memory, latency, startup, binary size.

## Compatibility & migration

What breaks, who migrates, how.

## Open questions
```

No RFCs have been submitted yet. Numbering starts at 0001.
