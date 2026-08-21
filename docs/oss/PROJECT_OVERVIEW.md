# Project Overview

_Factual one-pager. Last updated 2026-08-21._

**Project:** Sill — a native terminal for modern software development.
**Repository:** https://github.com/bloxy-studios/sill
**License:** MIT OR Apache-2.0 · **Status:** pre-alpha (no releases)
**Maintainer:** Abdul Ali ([@bloxy-studios](https://github.com/bloxy-studios)), solo
**Stack:** Rust core + Tauri 2 shell; TypeScript/React frontend; Bun tooling

## Problem

Developers increasingly work across many repositories, git worktrees,
long-running services, and — since 2023–2026, rapidly — multiple autonomous
coding agents (Claude Code, Codex CLI, Cursor, Aider, and successors) running
in parallel terminal sessions. Terminals still model all of this as anonymous
windows/tabs/panes: no notion of project, session identity, or "which of my
nine sessions needs a human right now." Detail: [PROBLEM.md](PROBLEM.md).

## Solution

A terminal whose native model is **workspace → project → session** (with git
worktrees and process kinds as first-class metadata), plus a
**provider-neutral** integration surface through which any coding agent can
surface its status — while remaining a fast, complete terminal with zero AI
involved. Detail: [SOLUTION.md](SOLUTION.md).

## Technology

Rust core owning all OS interaction (PTY, processes, sessions) behind a typed
IPC boundary; webview UI; strict CSP; threat-model-first security posture
(the threat model predates the terminal code deliberately). Detail:
[TECHNICAL_ARCHITECTURE.md](TECHNICAL_ARCHITECTURE.md).

## Target users

Developers who live in terminals: multi-repo maintainers, infra/platform
engineers, and the growing population running agent-parallel workflows.

## Open-source model

Permissive dual license, inbound=outbound contributions (no CLA),
maintainer-led governance with a documented evolution path, public roadmap,
security policy with private disclosure. Detail:
[OPEN_SOURCE_MODEL.md](OPEN_SOURCE_MODEL.md).

## Current maturity — stated plainly

The repository contains complete open-source infrastructure (CI, security
policy, threat model, governance, release automation, architecture/design
documentation) and an application scaffold. **The terminal itself is not yet
implemented; there are no users, releases, or contributors beyond the
maintainer.** Roadmap Phase 1 (PTY + emulation + rendering) is the current
work. This overview is updated as that changes — claims here never outrun
[../evidence/](../evidence/).

## Ecosystem context

The "agent-era terminal" category is real and contested (Warp, cmux — both
copyleft). Sill's distinct position: permissively licensed, provider-neutral,
no bundled agent, no account, workspace-first. Honest comparison:
[../COMPETITIVE_LANDSCAPE.md](../COMPETITIVE_LANDSCAPE.md).
