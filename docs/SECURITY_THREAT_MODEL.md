# Security Threat Model

**Status: design-stage.** Sill has no terminal functionality yet; this threat
model exists _before_ the code so that Phase 1+ is built against it, and it is
updated as subsystems land. Threat modeling before implementation is the
cheapest security work this project will ever do.

Scoring is qualitative (Low/Medium/High) and deliberately conservative.
Reports: see [SECURITY.md](../SECURITY.md).

## Assets

- The user's machine: files, processes, credentials in env/keychain/agent
  sockets (SSH keys, cloud tokens), clipboard contents
- The user's shell session integrity: what gets _executed_ vs what was _typed_
- Sill's own release/supply chain

## Trust assumptions

- The local user and OS are trusted; Sill does not defend against a
  compromised OS or malicious root.
- **Everything a PTY emits is untrusted**, even from tools the user invoked —
  `cat`-ing a hostile file or building a hostile repo must not compromise the
  terminal.
- Coding agents are semi-trusted automation: user-authorized but capable of
  executing at machine speed with imperfect judgment.

## Threats

### T1 — Escape-sequence abuse (terminal output → terminal behavior)

|                      |                                                                                                                                                                                                                                 |
| -------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Threat               | Hostile output uses control/OSC sequences to alter titles, spoof UI, resize, exfiltrate via query responses (e.g. answerback, color queries), or exploit parser bugs. Classic terminal CVE territory.                           |
| Impact               | High (UI spoofing → user runs attacker text; parser RCE worst case)                                                                                                                                                             |
| Likelihood           | Medium — hostile bytes are routine (build logs, curl, repos)                                                                                                                                                                    |
| Mitigation (planned) | Use a battle-tested emulation engine (ADR-0006) rather than a hand-rolled parser; allowlist supported sequences; cap/validate query responses; strip or gate title/answerback reflection; fuzz the parser in CI once it exists. |
| Residual risk        | Parser bugs are never zero; mitigated by engine choice + fuzzing + fast advisory process.                                                                                                                                       |

### T2 — Clipboard access via OSC 52

|                      |                                                                                                                                                                                                       |
| -------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Threat               | Output sets clipboard (paste-to-shell attacks) or reads it (data theft) via OSC 52.                                                                                                                   |
| Impact               | High (read = credential theft; write = staged command injection)                                                                                                                                      |
| Likelihood           | Medium                                                                                                                                                                                                |
| Mitigation (planned) | Clipboard **read** via escape sequence: denied by default. Clipboard write: off or size-capped + user-notified by default, configurable. Decisions recorded before Phase 1 ships selection/clipboard. |
| Residual risk        | Low if defaults stay strict.                                                                                                                                                                          |

### T3 — Paste-based injection (paste-jacking)

|                      |                                                                                                                                           |
| -------------------- | ----------------------------------------------------------------------------------------------------------------------------------------- |
| Threat               | Web/HTML copies smuggle hidden newlines/control chars so pasting executes commands immediately.                                           |
| Impact               | High (arbitrary command execution with user privilege)                                                                                    |
| Likelihood           | Medium-High — this attack is documented in the wild                                                                                       |
| Mitigation (planned) | Bracketed paste always; strip control characters from paste by default; warn-on-multiline-paste option; never auto-submit pasted content. |
| Residual risk        | User can still consciously run hostile text; that is a terminal's nature.                                                                 |

### T4 — IPC / webview boundary compromise

|               |                                                                                                                                                                                                                                                                                                                                            |
| ------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Threat        | XSS or injected content in the webview escalates: calls IPC commands to write PTYs (= execute commands) or abuses over-broad Tauri capabilities.                                                                                                                                                                                           |
| Impact        | High (webview compromise → command execution)                                                                                                                                                                                                                                                                                              |
| Likelihood    | Low-Medium (no remote content by design, but terminal output flows near the DOM)                                                                                                                                                                                                                                                           |
| Mitigation    | Strict CSP (already set — no `null` CSP); no `remote` URLs in the webview; capabilities kept minimal (`core:default`, `opener:default` today; every future permission reviewed in PR); terminal output rendered to a grid/canvas as **data**, never injected as HTML/DOM; IPC commands typed + validated, rejecting out-of-session access. |
| Residual risk | Webview engine 0-days (patched upstream via OS/WebView2 updates).                                                                                                                                                                                                                                                                          |

### T5 — Link opening (`tauri-plugin-opener`)

|                      |                                                                                                                                                               |
| -------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Threat               | Hostile output presents `file://`/custom-scheme/lookalike URLs; opener launches unexpected handlers.                                                          |
| Impact               | Medium                                                                                                                                                        |
| Likelihood           | Medium                                                                                                                                                        |
| Mitigation (planned) | Open only user-clicked links; scheme allowlist (`https`, `http`, `mailto`); show destination before opening non-obvious schemes; never auto-open from output. |
| Residual risk        | Low.                                                                                                                                                          |

### T6 — Malicious repositories & shell startup files

|                      |                                                                                                                                                                                                                                                |
| -------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Threat               | Cloned repo or directory trap runs code via shell hooks the _shell_ honors (direnv, `.zshrc` sourcing, VCS prompt helpers) — or via future Sill features that auto-inspect projects (worktree/branch detection).                               |
| Impact               | High (code execution) — but largely the shell ecosystem's existing surface                                                                                                                                                                     |
| Likelihood           | Medium                                                                                                                                                                                                                                         |
| Mitigation (planned) | Sill's own project inspection (Phase 2/3) must be read-only, parse files with hardened libraries, and **never execute** project-provided code/hooks without explicit per-repo consent. Anything Sill runs automatically is enumerated in docs. |
| Residual risk        | The shell's own behavior remains the user's configuration choice.                                                                                                                                                                              |

### T7 — Agent automation misuse (Phase 4+)

|                                                      |                                                                                                                                                                                                                                                                                                                             |
| ---------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Threat                                               | An agent (or something impersonating one) uses Sill's future integration surface to read other sessions' screens/scrollback (secrets) or inject input into sessions the user didn't authorize. Prompt-injected agents make this concrete: hostile _content_ steers a trusted _tool_.                                        |
| Impact                                               | High (cross-session data theft; unauthorized execution)                                                                                                                                                                                                                                                                     |
| Likelihood                                           | Medium once the surface exists; N/A today                                                                                                                                                                                                                                                                                   |
| Mitigation (design constraints, binding on ADR-0007) | Integration is opt-in per workspace; default scope = the agent's **own** session; cross-session read/write requires explicit grant; input injection into foreign sessions is denied by default; all agent-surface actions are logged visibly. The surface ships only after this section is expanded into a reviewed design. |
| Residual risk                                        | Users can over-grant; mitigated by loud UX and safe defaults.                                                                                                                                                                                                                                                               |

### T8 — Local IPC/socket endpoints (future CLI, agent protocol, MCP)

|                      |                                                                                                                                                                           |
| -------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Threat               | Another local process (other user, or sandboxed app) connects to Sill's control socket and drives sessions.                                                               |
| Impact               | High                                                                                                                                                                      |
| Likelihood           | Medium on multi-user/shared machines                                                                                                                                      |
| Mitigation (planned) | Sockets in user-private dirs with 0600 perms; peer-credential checks where the OS supports them; capability tokens per client; no TCP listeners, ever, for local control. |
| Residual risk        | Same-user malware can already do worse; documented honestly.                                                                                                              |

### T9 — Secrets in scrollback, logs, and session persistence

|                      |                                                                                                                                                                                                    |
| -------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Threat               | Tokens/keys printed to terminals persist in Sill's scrollback buffers, crash logs, or restored sessions; future agent surface could read them (see T7).                                            |
| Impact               | Medium-High                                                                                                                                                                                        |
| Likelihood           | High that secrets _appear_; lower that they leak                                                                                                                                                   |
| Mitigation (planned) | Scrollback in memory by default; any on-disk persistence is opt-in, documented, and excluded from diagnostics; logs never include raw terminal buffers; agent-surface reads governed by T7 scopes. |
| Residual risk        | Screen-visible secrets are inherently visible to screen readers of any kind.                                                                                                                       |

### T10 — Environment variables & child-process hygiene

|                      |                                                                                                                                       |
| -------------------- | ------------------------------------------------------------------------------------------------------------------------------------- |
| Threat               | Sill leaks its own internal variables into user shells, or mishandles PATH/env such that spawned processes behave unexpectedly.       |
| Impact               | Medium                                                                                                                                |
| Likelihood           | Medium                                                                                                                                |
| Mitigation (planned) | Explicit, documented env contract for spawned shells; internal control variables namespaced (`SILL_*`) and minimal; no PATH mutation. |
| Residual risk        | Low.                                                                                                                                  |

### T11 — Supply chain (dependencies, CI, releases)

|                     |                                                                                                                                                                                                                                                                                                                                                      |
| ------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Threat              | Malicious/compromised dependency (crates.io, npm), compromised GitHub Action, or tampered release artifact reaches users.                                                                                                                                                                                                                            |
| Impact              | High                                                                                                                                                                                                                                                                                                                                                 |
| Likelihood          | Low-Medium (ecosystem-wide reality)                                                                                                                                                                                                                                                                                                                  |
| Mitigation (active) | Lockfiles committed; `cargo-deny` (advisories/licenses/sources) in CI; Dependabot; third-party Actions pinned to commit SHAs; workflow permissions default to `contents: read`; draft releases with SHA-256 checksums, human-published. Planned: artifact attestations, then code signing/notarization ([RELEASE_SECURITY.md](RELEASE_SECURITY.md)). |
| Residual risk       | Registry-level compromises; reduced by small dependency policy ([DEPENDENCY_POLICY.md](DEPENDENCY_POLICY.md)).                                                                                                                                                                                                                                       |

### T12 — SSH / remote development (Phase 6, EXPLORING)

Deliberately not designed yet. Constraint recorded now: Sill will not store
remote credentials itself; it defers to the system SSH agent/config. A full
threat model section is a prerequisite for any Phase 6 implementation.

## Non-threats (explicitly out of scope)

- A terminal executes what users tell it to; Sill does not try to sandbox the
  user's own shell.
- Defense against a hostile OS, hostile root, or physical access.

## Standing review triggers

Any PR that touches: escape-sequence handling, clipboard, paste, IPC surface,
capabilities, spawned-process env, local sockets, or the agent surface —
requires a "Security impact" section in its description (enforced by PR
template) and updates this document when behavior changes.
