# ADR-0004: Conventional Commits, gently enforced

- **Status:** Accepted
- **Date:** 2026-08-21

## Context

Commit history should support changelog generation and searchability without
becoming a contribution barrier.

## Decision

Use [Conventional Commits 1.0.0](https://www.conventionalcommits.org/en/v1.0.0/)
(`feat:`, `fix:`, `docs:`, `refactor:`, `perf:`, `test:`, `build:`, `ci:`,
`chore:`) for commits on `main`. **Enforcement is social, not mechanical**: no
commit-lint CI gate; maintainers normalize messages via squash-merge when
needed. Details: [docs/COMMIT_GUIDELINES.md](../COMMIT_GUIDELINES.md).

## Alternatives considered

Free-form (harder changelogs), CI-enforced commitlint (fails first-time
contributors on formalities — exactly the wrong first impression).

## Consequences

- ✅ Clean history and greppable intent without contributor hazing.
- ⚠️ Requires squash-merge discipline from maintainers on messy PRs.
