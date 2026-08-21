# Sill

**A native terminal for modern software development.**

[![CI](https://github.com/bloxy-studios/sill/actions/workflows/ci.yml/badge.svg)](https://github.com/bloxy-studios/sill/actions/workflows/ci.yml)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](docs/LICENSING.md)
[![Status: pre-alpha](https://img.shields.io/badge/status-pre--alpha-orange.svg)](docs/PROJECT_STATUS.md)

Sill is an open-source terminal being built around a simple observation:
developers now work across many repositories, git worktrees, long-running
services, and — increasingly — autonomous coding agents (Claude Code, Codex
CLI, Cursor, Aider, and whatever ships next quarter), while terminals still
model everything as anonymous windows, tabs, and panes.

Sill's bet is that the terminal's native model should be **workspace →
project → session**: sessions with identity and status instead of nameless
rectangles, worktrees as first-class citizens, a workspace that restores on
restart and can answer _"what's running, and what needs me?"_ — with
provider-neutral awareness of coding agents layered on top, and full
usefulness with zero AI involved.

## Status: pre-alpha — not yet usable

Honesty first: **the terminal does not exist yet.** This repository currently
contains the project's complete open-source infrastructure (architecture and
design documents, security threat model, CI, release automation, governance)
and the application scaffold. Phase 1 — PTY, terminal emulation, rendering —
is the work in progress. There are no releases, and nothing to install.

If you're here this early, the useful entry points are the
[roadmap](ROADMAP.md), the [architecture](docs/ARCHITECTURE.md), and the
[open design questions](docs/decisions/). Watch the repo if you want to be
around when it becomes software.

| What                                                    | State                                      |
| ------------------------------------------------------- | ------------------------------------------ |
| Project infrastructure (CI, security, docs, governance) | ✅ done                                    |
| Terminal foundation (PTY, emulation, rendering)         | 🔨 Phase 1 — in progress                   |
| Workspaces, projects, sessions, worktrees               | 📋 designed ([docs/design/](docs/design/)) |
| Provider-neutral agent awareness                        | 📋 designed, RFC-gated                     |
| Releases for macOS / Linux / Windows                    | ⬜ after Phase 1                           |

## Why another terminal?

The terminal space is genuinely good — Ghostty, kitty, WezTerm, Alacritty,
tmux, Zellij each excel at what they chose ([honest comparison](docs/COMPETITIVE_LANDSCAPE.md)).
And the agent-era niche already has incumbents (Warp, cmux — both copyleft).
Sill exists because one position is unoccupied: a **permissively licensed
(MIT OR Apache-2.0), provider-neutral, local-first** terminal whose core
model is work (workspaces/projects/sessions) rather than rectangles — no
bundled agent, no account, no telemetry by default, no privileged vendor.

## Architecture

Rust core owning everything OS-level (PTY, processes, sessions) behind a
typed IPC boundary; webview UI (Tauri 2) under a strict CSP; terminal byte
streams treated as hostile input by design. The
[threat model](docs/SECURITY_THREAT_MODEL.md) predates the terminal code on
purpose. Details: [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md).

```
React/TS frontend  ──typed IPC──  Rust core (PTY · emulation · workspace
                                  model · agent detection)  ──  OS
```

## Development

Prerequisites: [Bun](https://bun.sh) ≥ 1.4, [Rust](https://rustup.rs)
(version auto-pinned by `rust-toolchain.toml`), and platform libraries
([details](CONTRIBUTING.md#prerequisites)).

```sh
git clone https://github.com/bloxy-studios/sill.git
cd sill
bun run setup     # verifies toolchain, installs dependencies
bun tauri dev     # launches the (currently placeholder) app
bun run check     # everything CI runs: typecheck, lint, format, build, fmt, clippy, tests
```

New contributor? The [30-minute quickstart](docs/CONTRIBUTOR_QUICKSTART.md)
goes from clone to first PR.

## Supported platforms

Development targets macOS, Linux, and Windows (CI builds all three; the
release matrix includes ARM64 for macOS and Linux). Support claims will be
made per-platform only when releases exist and are tested.

## Coding agents

Sill intends to work well with **any** terminal-based coding agent —
Claude Code, Codex CLI, Cursor, Gemini CLI, Aider, OpenCode, and future ones
— through neutral mechanisms, with no privileged vendor
([ADR-0007](docs/decisions/0007-agent-integration-model.md)). Current tested
compatibility: **none yet** — tracked honestly in the
[compatibility matrix](docs/AGENT_COMPATIBILITY.md).

## Contributing

Contributions are welcome now — docs, design review, and (soon) Phase 1
code. Start with [CONTRIBUTING.md](CONTRIBUTING.md); large ideas start as
[Discussions or RFCs](docs/rfcs/). Participation is governed by the
[Code of Conduct](CODE_OF_CONDUCT.md).

## Security

Sill is a terminal — security reports matter even pre-alpha. **Never use
public issues**; see [SECURITY.md](SECURITY.md) for private reporting.

## Governance, funding, project health

Maintainer-led, documented in [GOVERNANCE.md](GOVERNANCE.md) with a real
evolution path. Currently unfunded; the funding philosophy (money buys time,
never influence) is in [docs/FUNDING.md](docs/FUNDING.md). Current state and
focus: [docs/PROJECT_STATUS.md](docs/PROJECT_STATUS.md).

## License

Dual-licensed under [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE), at
your option — the Rust ecosystem standard. Contributions land under the same
terms ([why](docs/LICENSING.md)).

## Acknowledgements

Built with Tauri, Rust, React, Vite, and Bun; standing on decades of
terminal prior art. Honest credits: [ACKNOWLEDGMENTS.md](ACKNOWLEDGMENTS.md).
