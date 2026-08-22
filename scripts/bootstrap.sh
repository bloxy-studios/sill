#!/usr/bin/env bash
# Sill development environment bootstrap.
# Verifies the toolchain, explains what's missing, installs JS dependencies.
# Usage: bun run setup   (or: bash scripts/bootstrap.sh)
set -euo pipefail

ok()   { printf '  \033[32m✓\033[0m %s\n' "$1"; }
miss() { printf '  \033[31m✗\033[0m %s\n' "$1"; FAILED=1; }
note() { printf '    %s\n' "$1"; }
FAILED=0

echo "Checking Sill prerequisites…"

# Bun ≥ 1.4
if command -v bun >/dev/null 2>&1; then
  BUN_V="$(bun --version)"
  case "$BUN_V" in
    0.*|1.0.*|1.1.*|1.2.*|1.3.*)
      miss "Bun $BUN_V found — Sill needs ≥ 1.4"
      note "Update: bun upgrade" ;;
    *) ok "Bun $BUN_V" ;;
  esac
else
  miss "Bun not found"
  note "Install via the official instructions: https://bun.sh/docs/installation"
  note "(package managers like Homebrew/Scoop are available there too)"
fi

# Rust via rustup (rust-toolchain.toml pins the version). The pin only
# works when cargo resolves through rustup's shim — a Homebrew/system Rust
# earlier in PATH silently ignores it and fails later with
# "rustc X is not supported ... requires rustc Y". Catch that here.
PINNED="$(grep '^channel' rust-toolchain.toml | cut -d'"' -f2)"
if command -v rustup >/dev/null 2>&1; then
  CARGO_PATH="$(command -v cargo || true)"
  case "$CARGO_PATH" in
    "$HOME/.cargo/bin/cargo" | *rustup*)
      ok "rustup $(rustup --version 2>/dev/null | head -1 | awk '{print $2}') (cargo resolves through the rustup shim)"
      note "Toolchain $PINNED auto-installs on first cargo use"
      ;;
    "")
      miss "rustup found but no cargo on PATH"
      note "Add \$HOME/.cargo/bin to PATH (rustup's shims live there)"
      ;;
    *)
      miss "cargo at $CARGO_PATH is NOT rustup's shim — rust-toolchain.toml will be IGNORED"
      note "Another Rust install (often Homebrew) shadows rustup in PATH."
      note "Fix: brew uninstall rust    # or put \$HOME/.cargo/bin first in PATH"
      ;;
  esac
elif command -v cargo >/dev/null 2>&1; then
  RUSTC_V="$(rustc --version 2>/dev/null | awk '{print $2}')"
  case "$RUSTC_V" in
    "$PINNED"*)
      ok "cargo $RUSTC_V (no rustup; version happens to match the pin)"
      note "Installing rustup is still recommended so future pin bumps just work: https://rustup.rs"
      ;;
    *)
      miss "rustc $RUSTC_V found, but this project pins $PINNED (rust-toolchain.toml)"
      note "Without rustup the pin cannot auto-install the right toolchain."
      note "Install rustup: https://rustup.rs  (if Rust came from Homebrew: brew uninstall rust first)"
      ;;
  esac
else
  miss "Rust not found"
  note "Install: https://rustup.rs"
fi

# Platform libraries Tauri needs
case "$(uname -s)" in
  Darwin)
    if xcode-select -p >/dev/null 2>&1; then
      ok "Xcode Command Line Tools"
    else
      miss "Xcode Command Line Tools"
      note "Install: xcode-select --install"
    fi
    ;;
  Linux)
    if command -v pkg-config >/dev/null 2>&1 && pkg-config --exists webkit2gtk-4.1 2>/dev/null; then
      ok "webkit2gtk-4.1 development libraries"
    else
      miss "webkit2gtk-4.1 dev libraries (and friends) not detected"
      note "Debian/Ubuntu: sudo apt-get install -y libwebkit2gtk-4.1-dev build-essential \\"
      note "  curl wget file libxdo-dev libssl-dev libayatana-appindicator3-dev librsvg2-dev"
      note "Other distros: https://tauri.app/start/prerequisites/"
    fi
    ;;
  MINGW*|MSYS*|CYGWIN*)
    note "Windows: ensure Microsoft C++ Build Tools and WebView2 are installed"
    note "See https://tauri.app/start/prerequisites/"
    ;;
esac

if [ "$FAILED" -ne 0 ]; then
  echo
  echo "Fix the items marked ✗ above, then re-run: bun run setup"
  exit 1
fi

echo
echo "Installing JS dependencies…"
bun install

echo
echo "Ready. Start developing with: bun tauri dev"
