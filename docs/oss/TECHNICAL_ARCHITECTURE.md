# Technical Architecture (Application Summary)

_Condensed for program reviewers; the canonical documents are
[../ARCHITECTURE.md](../ARCHITECTURE.md) and [../design/](../design/).
Honesty marker: the architecture below is implemented only to the scaffold
level; subsystem designs are PROPOSED and labeled as such in the canonical
docs._

## Shape

```
React/TS webview (render + input only)
        │  typed Tauri IPC  ← security boundary, treated as public API
Rust core (all OS access)
  PTY layer · terminal emulation (proven Rust engine, ADR-0006 pending)
  session/workspace model · process awareness · notifications
        │  syscalls
Operating system (macOS / Linux / Windows)
```

## Deliberate properties

- **Memory-safe systems core.** All parsing of hostile input (terminal byte
  streams) happens in Rust, in an engine chosen for fuzz-ability and track
  record — not hand-rolled.
- **Privilege concentration.** The UI layer has no OS capabilities; strict
  CSP is enforced from the first commit (no `csp: null` scaffolding left
  behind); Tauri capabilities are minimal and reviewed per-addition.
- **Threat-model-first.** A design-stage threat model
  ([../SECURITY_THREAT_MODEL.md](../SECURITY_THREAT_MODEL.md)) covering
  escape-sequence abuse, OSC 52 clipboard, paste injection, IPC compromise,
  agent-surface misuse, and supply chain — written _before_ Phase 1 code, so
  security constraints bind the design rather than patching it.
- **Performance as budgets, not adjectives.** Published targets (startup,
  input latency, idle CPU, per-session memory) with a planned reproducible
  harness; no numbers are published that the harness didn't produce
  ([../PERFORMANCE.md](../PERFORMANCE.md)).
- **Small artifacts.** OS webview via Tauri instead of bundled Chromium —
  and the main technical risk of that choice (grid rendering performance in
  a webview) is named in ADR-0001 with a defined revisit trigger, not hidden.

## Engineering infrastructure (implemented now)

Cross-platform CI (fmt/clippy/tests/typecheck/lint/build on 3 OS), dependency
scanning (`cargo-deny`: advisories/licenses/sources), Dependabot, SHA-pinned
least-privilege workflows, OpenSSF Scorecard, tag-driven draft-release
pipeline with checksums, pinned toolchains, ADR/RFC process.
