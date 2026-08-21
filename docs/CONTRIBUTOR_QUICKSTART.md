# Contributor Quickstart

Goal: from zero to a submitted PR in ~30 minutes, assuming prerequisites are
installed. The long-form version is [CONTRIBUTING.md](../CONTRIBUTING.md).

**Honest note:** Sill is pre-alpha. What launches today is a placeholder UI on
top of the real build pipeline — the terminal is being built in
[Phase 1](../ROADMAP.md). That makes this the easiest moment the codebase will
ever have to learn.

## 1. Prerequisites (one-time)

- [Bun](https://bun.sh) ≥ 1.4 — `curl -fsSL https://bun.sh/install | bash`
- [rustup](https://rustup.rs) — installs the Rust version pinned by
  `rust-toolchain.toml` automatically on first build
- Platform libraries — macOS: `xcode-select --install`; Linux/Windows: see
  [CONTRIBUTING.md → Prerequisites](../CONTRIBUTING.md#prerequisites)

## 2. Clone, verify, run (~5 min + first compile)

```sh
git clone https://github.com/bloxy-studios/sill.git
cd sill
bun run setup        # checks toolchain, installs JS deps
bun tauri dev        # first Rust compile is the slow one; later runs are fast
```

A window opens. Frontend edits hot-reload; Rust edits trigger a rebuild.

## 3. Orient yourself (~10 min)

| Path                                  | What it is                                                                                                                                     |
| ------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------- |
| `src/App.tsx`                         | Frontend entry UI (placeholder). Calls Rust via `invoke("greet", …)` — that round-trip **is** Tauri IPC, the pattern everything real will use. |
| `src-tauri/src/lib.rs`                | Rust side: `#[tauri::command] fn greet` + app builder. New commands get registered here.                                                       |
| `src-tauri/capabilities/default.json` | What the webview is allowed to do. Deliberately minimal.                                                                                       |
| `docs/ARCHITECTURE.md`                | Current vs target architecture — read the diagram.                                                                                             |
| `ROADMAP.md`                          | What's being built and in what order.                                                                                                          |

The one architectural rule: **OS things (PTY, processes, files) happen in
Rust; the frontend renders state and sends input as data.** The IPC boundary
is a security boundary.

## 4. Make a change (~5 min)

Try the full loop with something trivial — e.g. change the greeting format in
`src-tauri/src/lib.rs`, watch the rebuild, see it in the UI. Update the
matching unit test at the bottom of `lib.rs` (`cargo test` in `src-tauri/`).

## 5. Check like CI does (~3 min)

```sh
bun run check
```

Runs: TypeScript typecheck, ESLint, Prettier check, `cargo fmt --check`,
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
