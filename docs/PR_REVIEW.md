# Pull Request Review Guide

What a review checks, in priority order. Applies to everyone, including the
maintainer's own PRs (self-review discipline: fill the template honestly, wait
for CI, read your own diff line-by-line).

## 1. Correctness

- Does it do what it claims? Are edge cases handled (empty input, huge input,
  non-UTF-8 bytes — this is a terminal)?
- Tests: behavior changes carry test changes. Absent tests need a stated
  reason (e.g. pure UI polish), not silence.

## 2. Security (gate, not suggestion)

Required scrutiny when a PR touches: escape/OSC handling, paste/clipboard,
IPC commands or capabilities, process spawning/env, file access, local
sockets, dependencies, CI/release workflows.

- New inputs validated? New surface reflected in the
  [threat model](SECURITY_THREAT_MODEL.md)?
- Tauri capability additions: least privilege argued in the PR description
- Secrets: none in code, tests, fixtures, or logs
- Security-sensitive PRs need the template's Security section filled with
  substance ("n/a" is acceptable only when obviously true)

## 3. Performance

For renderer / PTY-output path / IPC / session-model changes: expected impact
stated per [PERFORMANCE.md](PERFORMANCE.md). Reviewer sanity-checks: new
allocations in hot loops, unbounded buffers, per-byte IPC chatter, timers that
tick while idle.

## 4. Architecture

- Logic on the right side of the IPC boundary?
  ([ARCHITECTURE.md](ARCHITECTURE.md) — OS in Rust, presentation in frontend)
- Consistent with accepted ADRs? Contradicting one requires updating it, not
  ignoring it
- Should this have been an RFC? (If reviewing it feels like reviewing a
  design, pause and say so kindly)

## 5. Dependencies

New/updated deps: [DEPENDENCY_POLICY.md](DEPENDENCY_POLICY.md) questions
answered; `cargo-deny`/dependency-review green; lockfile diff actually read.

## 6. Cross-platform

macOS/Linux/Windows all considered? Platform-specific code behind clean
abstractions, `#[cfg]`s justified? CI matrix green is necessary, not
sufficient — reviewers flag "this needs a manual check on X" when it does.

## 7. Docs & polish

User-facing changes update docs/changelog; naming matches repo conventions;
comments explain _why_.

## Review conduct

Critique code, not people; explain _why_; distinguish blocking comments from
nits (`nit:` prefix); approve when it's better than main and sound — not when
it's perfect. First-time contributors get extra patience and pointers, and
maintainers fix trivia themselves rather than ping-ponging a newcomer over
formatting. Response-time honesty: solo maintainer, target days not hours;
ping after a week guilt-free.
