# Issue Triage

How incoming issues are classified. Goal: every issue reaches a decided state
quickly, even when the decision is "not now." Label taxonomy:
[LABELS.md](LABELS.md).

## Flow

New issue (`status:triage` by default) →

1. **Security?** If it describes a vulnerability: minimize the public issue
   (ask the reporter to move it to private reporting per
   [SECURITY.md](../SECURITY.md)); do not discuss exploit details publicly.
2. **Complete?** Bugs need version/commit, OS, repro steps. Missing info →
   `status:needs-info`; auto-close conversation stalls after ~30 days with an
   invitation to reopen.
3. **Reproducible?** Maintainer (or any contributor — this is welcome help)
   attempts the repro. Can't reproduce → `status:needs-info` with what was
   tried.
4. **Classify**: one `type:*`, relevant `area:*`, one `priority:*` →
   `status:accepted` (or close with reasons: duplicate / by-design /
   out-of-scope / wontfix).

## Priority meanings

| Priority            | Meaning                                                                   | Response posture                                     |
| ------------------- | ------------------------------------------------------------------------- | ---------------------------------------------------- |
| `priority:critical` | Security impact, data loss, crash-on-start, broken build/release pipeline | Drop other work; fix may warrant out-of-band release |
| `priority:high`     | Breaks a core workflow, no reasonable workaround                          | Next release                                         |
| `priority:medium`   | Real problem with workaround                                              | Scheduled by roadmap phase                           |
| `priority:low`      | Cosmetic, edge-case, nice-to-have                                         | When touched, or `help wanted`                       |

## Type-specific handling

- **Performance** (`type:performance`): needs numbers or a reproducible
  workload — "feels slow" → `status:needs-info` with a pointer to
  [PERFORMANCE.md](PERFORMANCE.md) methodology.
- **Compatibility** (`area:term` + program name in title): verify against the
  reference terminals (xterm behavior usually wins); record expected-vs-actual
  sequences in the issue.
- **Feature requests**: check roadmap fit. Fits → `status:accepted` +
  phase-appropriate milestone. Doesn't → say so honestly and close, or convert
  to Discussion if the problem is real but the solution unclear. Large →
  require an [RFC](rfcs/) before acceptance.
- **Agent integration requests**: neutrality rule applies (ADR-0007) — a
  request to privilege one vendor is reshaped into the neutral mechanism that
  serves it.

## Newcomer-friendly issues

When accepting an issue that is self-contained, has clear success criteria,
and doesn't require deep context: add `good first issue` + a comment noting
where to start (files, docs). Only genuinely useful work gets the label —
no manufactured busywork.

## Hygiene

Stale `status:needs-info` closes after ~30 days (reopen anytime). `main`
red-CI issues outrank everything but security. Triage passes happen at least
weekly while the project is small.
