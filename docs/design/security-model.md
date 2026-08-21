# Design: Security Model

**Status: PROPOSED as design; the CSP + minimal capabilities are ACTIVE
already.** The full threat catalog lives in
[SECURITY_THREAT_MODEL.md](../SECURITY_THREAT_MODEL.md); this document states
the _architecture_ of trust.

## Trust zones

```
Zone 0  OS + user                     trusted
Zone 1  Rust core                     trusted, minimal, owns all OS access
Zone 2  Webview (UI)                  semi-trusted: CSP-confined, capability-
                                      limited, no direct OS access
Zone 3  PTY byte streams              UNTRUSTED, always — even from tools the
                                      user launched
Zone 4  Cooperating agents (future)   semi-trusted automation: authorized by
                                      the user, treated as fallible
```

## Structural decisions

1. **All OS capability concentrates in Zone 1.** The webview cannot spawn,
   read files, or touch the network by itself; it can only call the typed IPC
   surface ([ipc.md](ipc.md)). Compromising the UI must not equal
   compromising the machine.
2. **Zone 3 bytes are data until proven otherwise.** Terminal output is
   parsed by the emulation engine into _state_; it is never interpreted as
   HTML/DOM, never triggers actions directly, and risky sequences (OSC 52
   clipboard, queries that reflect data back) are policy-gated with strict
   defaults (threat model T1–T3).
3. **CSP is on and restrictive** (`tauri.conf.json`): `default-src 'self'`,
   IPC via explicit `connect-src ipc: http://ipc.localhost` — no remote
   content origins. The webview loads only bundled assets. Any CSP loosening
   is a security-review PR by definition.
4. **Spawned shells are the user's agents** — they inherit user privilege, as
   any terminal's must. Sill adds no privilege: no setuid helpers, no
   privileged daemons, no service running as root.
5. **Local-first**: no account, no telemetry-by-default, no cloud dependency.
   Anything that ever changes this arrives via RFC with this file updated
   first.
6. **Future surfaces (L2 socket / MCP) default-deny**: user-private sockets,
   per-session scoping, visible audit of grants
   ([agent-architecture.md](agent-architecture.md)); shipped only after their
   threat-model sections are written and reviewed.

## What Sill explicitly does not promise

- Sandboxing the user's own shell or the programs they run
- Protection on an already-compromised machine
- That a webview engine 0-day cannot exist (mitigated by zone separation +
  upstream patching, not denied)

## Security development practices (active now)

Dependency scanning (`cargo-deny`), SHA-pinned CI, least-privilege workflow
tokens, secret-scanning expectations, and the release integrity ladder are
covered in [SUPPLY_CHAIN.md](../SUPPLY_CHAIN.md) and
[RELEASE_SECURITY.md](../RELEASE_SECURITY.md). Vulnerability handling:
[SECURITY.md](../../SECURITY.md).
