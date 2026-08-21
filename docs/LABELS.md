# Label Taxonomy

Deliberately small. One `type:*`, any relevant `area:*`, one `priority:*`, and
a `status:*` reflecting where the issue stands. Labels below are the complete
set — resist inventing more until these demonstrably fail.

> Maintainer note: labels are created in the GitHub UI/API; this file is the
> source of truth. Suggested colors are cosmetic.

## type:*

| Label              | Meaning                                                      |
| ------------------ | ------------------------------------------------------------ |
| `type:bug`         | Something behaves incorrectly                                |
| `type:feature`     | New capability                                               |
| `type:performance` | Speed/memory/latency problem or improvement                  |
| `type:security`    | Security hardening (never exploit details — see SECURITY.md) |
| `type:docs`        | Documentation                                                |
| `type:question`    | Should usually become a Discussion                           |

## area:*

`area:pty` · `area:term` (emulation/compat) · `area:renderer` ·
`area:workspace` (sessions/projects/worktrees) · `area:agent` ·
`area:ipc` · `area:cli` · `area:ui` · `area:security` · `area:build`
(build/CI/release) · `area:docs`

## priority:*

`priority:critical` · `priority:high` · `priority:medium` · `priority:low` —
meanings defined in [ISSUE_TRIAGE.md](ISSUE_TRIAGE.md).

## status:*

| Label               | Meaning                                            |
| ------------------- | -------------------------------------------------- |
| `status:triage`     | Awaiting classification (default on new issues)    |
| `status:needs-info` | Blocked on reporter                                |
| `status:accepted`   | Will be worked on                                  |
| `status:blocked`    | Blocked on something else (say what, in a comment) |

## Community

| Label              | Meaning                                                                |
| ------------------ | ---------------------------------------------------------------------- |
| `good first issue` | Self-contained, clear success criteria, starting pointers in a comment |
| `help wanted`      | Accepted, maintainer won't get to it soon, guidance available          |

Both labels are promises of support, not just tags — apply only when someone
will actually help a newcomer through the issue.
