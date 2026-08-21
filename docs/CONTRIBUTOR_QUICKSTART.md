# Contributor Quickstart

Goal: from zero to a submitted PR in ~30 minutes, assuming prerequisites are
installed. The long-form version is [CONTRIBUTING.md](../CONTRIBUTING.md).

**Honest note:** Sill is pre-alpha. What launches today is a placeholder UI on
top of the real build pipeline — the terminal is being built in
[Phase 1](../ROADMAP.md). That makes this the easiest moment the codebase will
ever have to learn.

## 1. Prerequisites (one-time)

- [Bun](https://bun.sh) ≥ 1.4 — install via the
  [official instructions](https://bun.sh/docs/installation) (package-manager
  routes like Homebrew/Scoop are listed there; prefer those if you'd rather
  not pipe an installer script)
- [rustup](https://rustup.rs) — installs the Rust version pinned by
  `rust-toolchain.toml` automatically on first build
- Platform libraries — macOS: `xcode-select --install`; Linux/Windows: see
  [CONTRIBUTING.md → Prerequisites](../CONTRIBUTING.md#prerequisites)
- Windows only: run `bun run setup` / `bun run check` from **Git Bash**
  (ships with Git for Windows) — they are Bash scripts

## 2. Clone, verify, run (~5 min + first compile)

```sh
git clone https://github.com/bloxy-studios/sill.git
cd sill
bun run setup        # checks toolchain, installs JS deps
bun tauri dev        # first Rust compile is the slow one; later runs are fast
```

A window opens. Frontend edits hot-reload; Rust edits trigger a rebuild.

## 3. Orient yourself (~10 min)

| Path                                  | What it is                                                                                                    |
| ------------------------------------- | ------------------------------------------------------------------------------------------------------------- |
| `crates/sill-core/`                   | The terminal core: PTY sessions, emulation engine, snapshots, events. Builds and tests headless — start here. |
| `src/App.tsx` + `src/lib/`            | Canvas grid renderer, keyboard encoding, session view. Draws Rust snapshots imperatively.                     |
| `src-tauri/src/lib.rs`                | Typed IPC commands over sill-core + snapshot/event pump threads.                                              |
| `src-tauri/capabilities/default.json` | What the webview is allowed to do. Deliberately minimal.                                                      |
| `docs/ARCHITECTURE.md`                | Current vs target architecture — read the diagram.                                                            |
| `ROADMAP.md`                          | What's being built and in what order.                                                                         |

The one architectural rule: **OS things (PTY, processes, files) happen in
Rust; the frontend renders state and sends input as data.** The IPC boundary
is a security boundary.

## 4. Make a change (~5 min)

Try the full loop with something visible — e.g. tweak the default theme in
`src/lib/renderer.ts` (hot-reloads), or adjust a snapshot detail in
`crates/sill-core/src/snapshot.rs` and run its tests
(`cargo test -p sill-core`) to see the contract enforced.

## 5. Check like CI does (~3 min)

```sh
bun run check
```

Runs: TypeScript typecheck, ESLint, Prettier check, frontend production
build, `cargo fmt --check`,
`cargo clippy -D warnings`, `cargo test`. Green locally ⇒ green in CI.

## 6. Submit (~5 min)

```sh
git checkout -b fix/my-change
git commit -am "fix: describe the change"    # conventional commits, best effort
git push -u origin fix/my-change
```

Open the PR; the template asks what/why/how-tested. First PRs: small is
beautiful — docs fixes and test additions are genuinely welcome.

## Where to ask

Stuck on setup → [Discussions Q&A](https://github.com/bloxy-studios/sill/discussions).
Found a docs lie while onboarding → that's a bug; PR the fix. This file
especially.
