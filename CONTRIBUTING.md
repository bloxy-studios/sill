# Contributing to Sill

Thanks for your interest. Sill is pre-alpha: the terminal itself is being
built right now, which means contributions have outsized impact — and that the
ground moves. This document tells you how to build the project, what we expect
in PRs, and where work happens.

**Before a large change, talk to us first** — see [Large changes](#large-changes).

## Ways to contribute

- **Code** — Rust core, TypeScript/React UI
- **Testing** — platforms, shells, terminal programs (vim, tmux, htop, agents)
- **Documentation** — guides, architecture docs, fixing anything stale
- **Design** — UX for the workspace/session model ([open a Discussion first](https://github.com/bloxy-studios/sill/discussions))
- **Triage** — reproducing bug reports is genuinely valuable

Good entry points are labeled
[`good first issue`](https://github.com/bloxy-studios/sill/issues?q=is%3Aissue+is%3Aopen+label%3A%22good+first+issue%22)
and [`help wanted`](https://github.com/bloxy-studios/sill/issues?q=is%3Aissue+is%3Aopen+label%3A%22help+wanted%22).

## Prerequisites

| Tool                      | Version                         | Notes                                           |
| ------------------------- | ------------------------------- | ----------------------------------------------- |
| [Bun](https://bun.sh)     | ≥ 1.4                           | JS runtime + package manager (no Node needed)   |
| [Rust](https://rustup.rs) | pinned by `rust-toolchain.toml` | rustup installs the right version automatically |
| Platform deps             | see below                       | required by Tauri                               |

**macOS:** Xcode Command Line Tools (`xcode-select --install`).

**Linux (Debian/Ubuntu):**

```sh
sudo apt-get update
sudo apt-get install -y libwebkit2gtk-4.1-dev build-essential curl wget file \
  libxdo-dev libssl-dev libayatana-appindicator3-dev librsvg2-dev
```

Other distros: see the [Tauri prerequisites](https://tauri.app/start/prerequisites/).

**Windows:** Microsoft C++ Build Tools and WebView2 (usually preinstalled on
Windows 11); see the Tauri prerequisites page. Also **Git for Windows** —
`bun run setup` and `bun run check` are Bash scripts, so run them from
**Git Bash** (everything else, like `bun tauri dev`, works from any shell).

## Setup

```sh
git clone https://github.com/bloxy-studios/sill.git
cd sill
bun run setup     # verifies toolchain, installs dependencies
bun tauri dev     # launches the app with hot reload
```

`bun run setup` is a thin script (`scripts/bootstrap.sh`) — it checks Bun,
Rust, and platform libraries, then runs `bun install`. If it reports a missing
system library, install it and re-run.

## Repository layout

```
src/          React + TypeScript frontend (webview)
src-tauri/    Rust core: application setup, commands, (future) PTY & sessions
  src/lib.rs  Tauri builder + command handlers
docs/         Architecture, design, policies
.github/      CI, issue forms, templates
scripts/      Developer tooling
```

Where code belongs: **anything that touches the OS (PTY, processes, files,
signals) is Rust**; the frontend renders state and captures input. IPC between
them is the security boundary — read
[docs/ARCHITECTURE.md](docs/ARCHITECTURE.md) before moving logic across it.

## Development loop

| Command                                                  | What it does                                                       |
| -------------------------------------------------------- | ------------------------------------------------------------------ |
| `bun tauri dev`                                          | run the app (hot-reloads frontend, rebuilds Rust on change)        |
| `bun run check`                                          | everything CI runs: typecheck, lint, format, Rust fmt/clippy/tests |
| `bun run typecheck`                                      | TypeScript, no emit                                                |
| `bun run lint`                                           | ESLint                                                             |
| `bun run format`                                         | Prettier (write) / `format:check` to verify                        |
| `cargo fmt && cargo clippy --all-targets -- -D warnings` | in `src-tauri/`                                                    |
| `cargo test`                                             | in `src-tauri/`                                                    |
| `bun tauri build`                                        | full production build (slow; rarely needed locally)                |

`bun run check` is intentionally the same set of checks CI enforces — if it
passes locally, CI should agree. Debugging: `bun tauri dev` gives you webview
devtools (right-click → Inspect) and Rust logs on stderr.

## Submitting changes

1. Fork, then branch from `main`: `git checkout -b fix/paste-crash`.
2. Keep PRs focused and reviewable. Several small PRs beat one big one.
3. Add or update tests where behavior changes (Rust: colocated `#[cfg(test)]`;
   frontend test infra arrives with the first real UI).
4. Update docs touched by your change.
5. Run `bun run check`.
6. Open the PR — the template asks what/why/how-tested; security- or
   performance-relevant changes need a sentence on impact.

Commit messages follow [Conventional Commits](docs/COMMIT_GUIDELINES.md)
(`feat:`, `fix:`, `docs:`…). Don't stress over it for a first PR; we can fix
messages on squash.

Review: the maintainer reviews all PRs — currently usually within a few days,
honestly sometimes longer. Pings after a week are welcome.

## Large changes

For anything architectural — new subsystems, the terminal engine, agent
protocol surface, plugin system, security-model changes — **open a Discussion
or [RFC](docs/rfcs/) before writing serious code.** This is the difference
between a merged design and wasted weekends. Small fixes never need ceremony.

## Issues

- **Bugs**: use the bug form; include OS, version/commit, and reproduction
  steps. A minimal repro is worth more than a perfect description.
- **Security bugs**: never public issues — see [SECURITY.md](SECURITY.md).
- **Features**: use the feature form; explain the problem before the solution.

## Licensing of contributions

Sill is dual-licensed MIT OR Apache-2.0. Unless you explicitly state
otherwise, any contribution intentionally submitted for inclusion in Sill by
you shall be dual-licensed as above, without any additional terms or
conditions. No CLA, no sign-off ritual. Don't submit code you don't have the
right to submit — this includes large verbatim copies from incompatibly
licensed projects.

## Conduct

Participation is governed by the [Code of Conduct](CODE_OF_CONDUCT.md). In
short: be direct about code, decent to people.
