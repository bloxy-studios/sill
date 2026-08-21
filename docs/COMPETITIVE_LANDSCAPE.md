# Competitive Landscape

Where Sill sits among terminals, honestly. Facts below (licenses, languages,
positioning) verified against project repositories on **2026-08-21**; stars
and activity drift, so treat numbers as a snapshot. This document exists to
keep Sill's positioning grounded — not to attack anyone. Every project here
is good at what it chose.

## The landscape

| Project                                             | License                  | Built with         | Chosen problem                                                                                 |
| --------------------------------------------------- | ------------------------ | ------------------ | ---------------------------------------------------------------------------------------------- |
| [Alacritty](https://github.com/alacritty/alacritty) | Apache-2.0               | Rust               | Deliberately minimal GPU terminal; no tabs/splits by design                                    |
| [kitty](https://github.com/kovidgoyal/kitty)        | GPL-3.0                  | C, Python          | Feature-rich GPU terminal; own graphics/keyboard protocols                                     |
| [WezTerm](https://github.com/wezterm/wezterm)       | MIT                      | Rust               | Terminal + multiplexer in one; Lua-programmable                                                |
| [Ghostty](https://github.com/ghostty-org/ghostty)   | MIT                      | Zig (+ native UIs) | Fast terminal with genuinely platform-native UI (`libghostty` core)                            |
| [iTerm2](https://github.com/gnachman/iTerm2)        | GPL-family               | Objective-C        | The mature macOS terminal; deep tmux integration                                               |
| [tmux](https://github.com/tmux/tmux)                | ISC                      | C                  | The canonical multiplexer; sessions survive detach                                             |
| [Zellij](https://github.com/zellij-org/zellij)      | MIT                      | Rust               | Terminal workspace with layouts and a WASM plugin runtime                                      |
| [Warp](https://github.com/warpdotdev/Warp)          | AGPL-3.0 (UI crates MIT) | Rust               | "Agentic development environment"; block-based UI, built-in agent platform, commercial product |
| [cmux](https://github.com/manaflow-ai/cmux)         | GPL-3.0-or-later         | Swift + Rust       | Ghostty-based macOS terminal with vertical tabs and notifications for AI coding agents         |

Notes that correct common stale beliefs: **Warp's client is no longer
closed-source** (AGPL-3.0 since 2026; server side remains closed and the
product is commercial). **Ghostty is Zig**, not Rust.

## What each teaches Sill

- **Alacritty**: restraint is a feature; also, its `alacritty_terminal`/`vte`
  crates are candidate foundations (ADR-0006) — a competitor and a supplier.
- **tmux/Zellij**: session persistence and workspace thinking already exist in
  multiplexer form; Sill's bet is that these belong in the terminal's native
  model, not a layer duct-taped inside one.
- **Ghostty**: platform-native polish sets the UX bar; its libghostty
  architecture validates "shared core, thin shells."
- **WezTerm**: one coherent Rust codebase can carry terminal + mux + config
  programmability; also supplies `portable-pty`/`termwiz`.
- **kitty/iTerm2**: protocol innovation (graphics, semantic zones) comes from
  terminals willing to extend the medium.
- **Warp/cmux**: the agent-era terminal is a real category with real demand —
  cmux went 0→26k stars in seven months.

## The uncomfortable part, stated plainly

**"Terminal for AI agents" is a contested category, not an open niche.** Warp
(64k★, venture-backed, ships its own agent platform) and cmux (26k★,
fast-growing, macOS) are already there. A generic "agent terminal" pitch adds
nothing. Sill proceeds anyway because its bets are different:

1. **Provider-neutral by constitution** (ADR-0007): no built-in agent, no
   privileged vendor, no account. Warp centers its own agent platform; Sill
   centers _whatever agents the user already runs_.
2. **Permissive license** (MIT OR Apache-2.0): both incumbents chose
   copyleft (AGPL / GPL). A permissively-licensed, embeddable-by-anyone
   workspace/agent model is an ecosystem position neither occupies.
3. **Workspace-first, AI-optional**: the workspace → project → session →
   worktree model (Phase 3) is the product even with zero agents; agent
   awareness (Phase 4) is a layer on top, not the foundation.
4. **Cross-platform + lightweight**: cmux is macOS-only; Warp is a full
   environment. Sill aims to stay a small, fast terminal.

Whether these bets are enough is an open question that only shipped software
answers. This document gets updated when reality disagrees with it.

## Non-competition

Sill happily coexists with tmux (attach from Sill), shell tooling, and
editors. It is a terminal, not an IDE, browser, or agent runtime — see
[ROADMAP.md](../ROADMAP.md) non-goals.
