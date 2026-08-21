# Roadmap

Sill's goal: a fast, native terminal designed for how software is actually
developed now — multiple repositories, worktrees, services, long-running
sessions, and increasingly, autonomous coding agents working alongside humans.

This roadmap is organized by phase and horizon (**NOW / NEXT / LATER /
EXPLORING**), not by dates. Sill has no release schedule to promise yet, and
this document will change as reality intrudes. Items are not commitments.

**Current reality (2026-08):** Sill is pre-alpha. The repository contains the
application scaffold and project infrastructure; the terminal itself is not
yet implemented. Phase 1 is the only active phase.

## Phase 1 — Terminal foundation — NOW

The non-negotiable base: Sill must be a good terminal before anything else.

- PTY management: spawn, resize, kill; login shells; environment handling
- Terminal emulation (VT/xterm-compatible) via a proven engine
  (see [ADR-0006](docs/decisions/0006-terminal-engine.md) — decision open)
- Rendering: correct, fast text grid; selection; scrollback
- Input: keyboard model, paste (bracketed, with safety), IME basics
- Shell support: zsh, bash, fish on macOS and Linux
- Configuration: fonts, colors, keybindings
- Baseline performance budgets ([docs/PERFORMANCE.md](docs/PERFORMANCE.md))

Exit criteria: the maintainer can use Sill as a daily-driver terminal.

## Phase 2 — Developer workflows — NEXT

- Splits and tabs, done after the session model exists (not before)
- Shell integration: semantic prompt/command zones (OSC 133), command status
- Directory and repository awareness (current project, git branch/worktree)
- Command notifications: "long-running command finished/failed"
- Search in scrollback

## Phase 3 — Workspace system — NEXT

The core product thesis: richer primitives than window/tab/pane.

- First-class model: workspace → project → session
- Git worktree awareness as a first-class concept
- Session persistence and restore
- Fast switching between projects/sessions

## Phase 4 — Agent-native capabilities — LATER

Provider-neutral support for coding agents (Claude Code, Codex CLI, Cursor,
Gemini CLI, Aider, OpenCode, and whatever comes next). Sill must remain fully
useful with zero AI involved.

- Detection of agent processes running in sessions
- Session status: idle / running / agent waiting for input
- Notifications when an agent needs attention
- A neutral integration surface (no privileged vendor)
  (see [docs/design/agent-architecture.md](docs/design/agent-architecture.md) — PROPOSED)

## Phase 5 — Programmability — LATER

- `sill` CLI to control the app (open session, run command, query state)
- Event stream for scripting and automation
- Declarative workspace configuration

## Phase 6 — Remote development — EXPLORING

- SSH-backed sessions
- Container / remote dev environment attachment
- Only if it can be done without compromising the local-first security model

## Phase 7 — Ecosystem — EXPLORING

- Plugin system ([docs/design/plugin-architecture.md](docs/design/plugin-architecture.md) — PROPOSED)
- MCP surface for agent interop ([docs/integrations/mcp.md](docs/integrations/mcp.md) — PLANNED)
- Themes, community integrations

## Non-goals

- Being an AI product. AI-adjacent features must degrade to zero gracefully.
- Bundling an LLM, API keys, or a vendor account requirement.
- A built-in browser, email client, or kitchen sink.
- Telemetry-by-default. (Any future opt-in metrics require an RFC.)

## How to influence this roadmap

Open a Discussion, or an RFC for substantial proposals
([docs/rfcs/](docs/rfcs/)). The roadmap is re-evaluated as phases complete.
