# Commit Guidelines

Sill uses [Conventional Commits 1.0.0](https://www.conventionalcommits.org/en/v1.0.0/).
Decision + enforcement stance: [ADR-0004](decisions/0004-commit-convention.md)
— **followed on `main`, never a reason to reject a first-time PR** (maintainers
fix messages on squash-merge).

## Format

```
<type>(<optional scope>)<!>: <imperative summary, lowercase, no period>

[optional body: what & why, wrapped ~72 cols]

[optional footers: BREAKING CHANGE:, Fixes #123, Co-authored-by:]
```

## Types

| Type       | Use for                                           |
| ---------- | ------------------------------------------------- |
| `feat`     | user-visible functionality                        |
| `fix`      | bug fixes                                         |
| `docs`     | documentation only                                |
| `refactor` | code change, no behavior change                   |
| `perf`     | performance improvement (say how it was measured) |
| `test`     | tests only                                        |
| `build`    | build system, dependencies                        |
| `ci`       | workflows, CI config                              |
| `chore`    | maintenance that fits nothing above               |

Scopes (optional, lowercase): `pty`, `term`, `renderer`, `workspace`, `agent`,
`ipc`, `cli`, `ui`, `security`, `release`. Breaking changes: `!` after
type/scope **and** a `BREAKING CHANGE:` footer explaining migration.

## Examples

```
feat(pty): propagate window resize to child via TIOCSWINSZ
fix(term): clamp OSC color-query responses to valid range
docs: correct linux prerequisites for fedora
perf(renderer): coalesce damage regions per frame (30→2 ipc events on flood)
ci: pin tauri-action to commit sha
feat(ipc)!: rename session.write to session.input

BREAKING CHANGE: `session.write` IPC command is now `session.input`.
```

## Practices

- One logical change per commit; keep PRs rebased into reviewable commits
  (or expect a squash).
- The body explains _why_; the diff already shows _what_.
- Reference issues in footers (`Fixes #123`) so automation links them.
- Release tooling reads these types to draft changelog sections — `feat` →
  Added, `fix` → Fixed, `perf` → Performance, `!` → Breaking Changes.
