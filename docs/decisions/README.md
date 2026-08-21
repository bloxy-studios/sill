# Architecture Decision Records

Significant, hard-to-reverse decisions are recorded here so future
contributors (and future maintainers) can understand _why_, not just _what_.

Format per record: **Context → Decision → Alternatives considered →
Consequences → Status → Date.** Statuses: `Proposed`, `Accepted`,
`Superseded by ADR-XXXX`, `Rejected`. An `Open` marker inside a Proposed ADR
means the question is real but undecided.

Records are only created for real decisions. Numbering is chronological.

| ADR                                     | Title                                                 | Status          |
| --------------------------------------- | ----------------------------------------------------- | --------------- |
| [0001](0001-application-stack.md)       | Application stack: Tauri 2 + Rust core + web frontend | Accepted        |
| [0002](0002-dual-license.md)            | Dual license: MIT OR Apache-2.0                       | Accepted        |
| [0003](0003-contribution-licensing.md)  | Contribution licensing: inbound=outbound, no CLA/DCO  | Accepted        |
| [0004](0004-commit-convention.md)       | Conventional Commits, gently enforced                 | Accepted        |
| [0005](0005-release-process.md)         | Tag-driven draft releases via CI                      | Accepted        |
| [0006](0006-terminal-engine.md)         | Terminal emulation engine selection                   | Proposed (open) |
| [0007](0007-agent-integration-model.md) | Provider-neutral agent integration model              | Proposed (open) |

To add one: copy the section structure from any accepted ADR, number it next,
and open a PR. Large proposals should start life as an [RFC](../rfcs/) and
graduate to an ADR once decided.
