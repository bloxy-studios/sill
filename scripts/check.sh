#!/usr/bin/env bash
# Run the same checks CI enforces. Green here ⇒ green there.
# Usage: bun run check   (or: bash scripts/check.sh)
set -euo pipefail

step() { printf '\n\033[1m== %s ==\033[0m\n' "$1"; }

step "TypeScript (tsc --noEmit)"
bun run typecheck

step "ESLint"
bun run lint

step "Prettier (check)"
bun run format:check

step "cargo fmt --check"
cargo fmt --manifest-path src-tauri/Cargo.toml --all -- --check

# Clippy and tests need Tauri's system libraries on Linux; degrade with a
# clear message instead of a cryptic sys-crate build error.
if [ "$(uname -s)" = "Linux" ] && ! pkg-config --exists webkit2gtk-4.1 2>/dev/null; then
  printf '\n\033[33mSkipping clippy + tests: webkit2gtk-4.1 dev libraries not installed.\033[0m\n'
  printf 'Install them (see CONTRIBUTING.md) to run the full suite locally; CI always runs it.\n'
else
  step "cargo clippy (-D warnings)"
  cargo clippy --manifest-path src-tauri/Cargo.toml --workspace --all-targets -- -D warnings

  step "cargo test"
  cargo test --manifest-path src-tauri/Cargo.toml --workspace
fi

printf '\n\033[32mAll checks passed.\033[0m\n'
