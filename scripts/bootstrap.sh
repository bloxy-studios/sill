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

# Rust via rustup (rust-toolchain.toml pins the version)
if command -v rustup >/dev/null 2>&1; then
  ok "rustup $(rustup --version 2>/dev/null | head -1 | awk '{print $2}')"
  note "Toolchain $(grep '^channel' rust-toolchain.toml | cut -d'"' -f2) auto-installs on first cargo use"
elif command -v cargo >/dev/null 2>&1; then
  ok "cargo $(cargo --version | awk '{print $2}') (no rustup — ensure it matches rust-toolchain.toml)"
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
