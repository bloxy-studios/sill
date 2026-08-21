# Security (Application Summary)

_Canonical documents: [SECURITY.md](../../SECURITY.md),
[../SECURITY_THREAT_MODEL.md](../SECURITY_THREAT_MODEL.md),
[../SUPPLY_CHAIN.md](../SUPPLY_CHAIN.md), [../OPENSSF.md](../OPENSSF.md)._

Why it matters here: a terminal parses hostile bytes all day, sits next to
credentials, and — in Sill's thesis — will interact with autonomous agents.
Security posture is a core competency claim for this project, so it is backed
by artifacts, not adjectives:

**In place at day zero (verifiable in-repo):**

- Security policy with private vulnerability reporting and response targets
- **Design-stage threat model** (12 threat classes: escape-sequence abuse,
  OSC 52 clipboard, paste injection, IPC/webview boundary, agent-surface
  misuse, local sockets, secrets-in-scrollback, supply chain…) written
  _before_ the terminal code, binding on the design
- Strict CSP from first commit; minimal Tauri capabilities
- Supply chain: lockfiles + pinned toolchain, `cargo-deny`
  (advisories/licenses/sources — all currently green, with 16 explicitly
  documented ignores for Tauri's known unmaintained transitive GTK3/unic
  bindings), Dependabot, SHA-pinned actions, least-privilege workflow tokens,
  OpenSSF Scorecard in CI
- Release integrity ladder documented honestly: CI-built draft releases +
  checksums now; attestations planned; **signing/notarization not yet
  configured and openly stated** ([../RELEASE_SECURITY.md](../RELEASE_SECURITY.md))

**Track record:** none yet — no advisories, because no shipped attack
surface. The claim available today is _posture_, not _history_; history
accumulates in [../evidence/SECURITY.md](../evidence/SECURITY.md).
