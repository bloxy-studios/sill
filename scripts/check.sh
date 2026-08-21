#!/usr/bin/env bash
# Run the same checks CI enforces. Green here ⇒ green there.
# Usage: bun run check   (or: bash scripts/check.sh)
#
# On Windows, run this from Git Bash (installed with Git for Windows).
#
# If Tauri's Linux system libraries are missing, the Rust clippy/test steps
# cannot run. This script then reports a PARTIAL result and exits nonzero —
# it never claims full CI parity for checks it didn't run.
set -euo pipefail

step() { printf '\n\033[1m== %s ==\033[0m\n' "$1"; }

step "TypeScript (tsc --noEmit)"
bun run typecheck

step "ESLint"
bun run lint

step "Prettier (check)"
bun run format:check

step "Frontend production build"
bun run build

step "cargo fmt --check"
cargo fmt --manifest-path src-tauri/Cargo.toml --all -- --check

if [ "$(uname -s)" = "Linux" ] && ! pkg-config --exists webkit2gtk-4.1 2>/dev/null; then
  printf '\n\033[33m== PARTIAL RESULT ==\033[0m\n'
  printf 'Frontend checks and cargo fmt passed, but \033[1mcargo clippy and cargo test\n'
  printf 'were SKIPPED\033[0m: webkit2gtk-4.1 dev libraries are not installed, so the\n'
  printf 'Tauri crate cannot compile on this machine.\n\n'
  printf 'CI will still enforce them. For full local parity install the system\n'
  printf 'libraries (see CONTRIBUTING.md → Prerequisites) and re-run.\n'
  exit 1
fi

step "cargo clippy (-D warnings)"
cargo clippy --manifest-path src-tauri/Cargo.toml --workspace --all-targets -- -D warnings

step "cargo test"
cargo test --manifest-path src-tauri/Cargo.toml --workspace

printf '\n\033[32mAll checks passed.\033[0m\n'
