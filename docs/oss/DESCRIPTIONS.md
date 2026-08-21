# Reusable Project Descriptions

_For application forms and directories. Written 2026-08-21 for a pre-alpha
project — these describe intent honestly and must be revised the moment
reality improves on them. Word counts approximate._

## 50 words

Sill is an early-stage open-source terminal (Rust, Tauri; MIT OR Apache-2.0)
designed for modern development: first-class workspaces, projects, and
sessions instead of anonymous tabs, with provider-neutral awareness of coding
agents like Claude Code and Codex. Pre-alpha: infrastructure and design are
public; the terminal is being built.

## 100 words

Sill is an open-source desktop terminal, built in Rust on Tauri and dual-
licensed MIT OR Apache-2.0. Its thesis: developers now work across many
repositories, git worktrees, services, and autonomous coding agents, while
terminals still model everything as windows, tabs, and panes. Sill makes
workspaces, projects, and sessions the native primitives — restorable,
queryable, and aware of which session needs human attention — with a
provider-neutral integration ladder any agent can adopt, and full usefulness
with no AI involved. Currently pre-alpha: governance, security threat model,
CI, and design documents are public; Phase 1 (terminal foundation) is in
progress by a solo maintainer.

## 250 words

Sill is an open-source terminal for modern software development, built with
a Rust core and Tauri shell, dual-licensed MIT OR Apache-2.0.

The problem: developers increasingly run many terminal sessions at once —
multiple repositories and git worktrees, long-running services, and, since
the rise of CLI coding agents (Claude Code, Codex CLI, Cursor, Gemini CLI,
Aider), autonomous processes that work for minutes and then silently block
waiting for a human. Terminals model all of this as anonymous rectangles.
Nothing answers "what's running across my projects?" or "which session needs
me?" — state dies on restart, and attention routing is manual tab-scanning.

Sill's design makes workspace → project → session the terminal's native
model: sessions have identity, kind, and status; worktrees are first-class;
the workspace restores and can be queried. Agent awareness is layered and
strictly provider-neutral — passive detection, standard escape sequences
(OSC 133/9), then an opt-in, threat-modeled local protocol — so any agent
can integrate and none is privileged. Sill remains a complete, fast terminal
with zero AI involved; there is no bundled agent, no account, no telemetry
by default.

Status, honestly: pre-alpha. The repository ships complete open-source
infrastructure from day zero — governance, security policy and threat model,
cross-platform CI, dependency and license scanning, release automation with
checksums, architecture decision records — but the terminal itself is
Phase 1 work in progress by a solo maintainer. In a category where the
incumbents are copyleft or built around one vendor's agent platform, Sill's
bet is a permissive, neutral, local-first alternative.

## 500 words

[Compose per application from PROBLEM.md + SOLUTION.md + OPEN_SOURCE_MODEL.md

- current PROJECT_STATUS.md rather than maintaining a fourth parallel text —
  long descriptions drift fastest. Assemble fresh, cite evidence, have the
  maintainer read it aloud once before submitting.]
