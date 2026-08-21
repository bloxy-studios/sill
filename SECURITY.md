# Security Policy

Sill is a terminal: it spawns shells, executes commands, renders untrusted
byte streams from arbitrary programs, and will eventually sit between
developers and autonomous coding agents. Security reports are taken seriously
even at this early stage, and the earlier a design flaw is found, the cheaper
it is to fix.

## Reporting a vulnerability

**Do not report security vulnerabilities through public GitHub issues,
discussions, or pull requests.**

Preferred: **GitHub private vulnerability reporting** —
[Report a vulnerability](https://github.com/bloxy-studios/sill/security/advisories/new).
This keeps the report private between you and the maintainer and supports
coordinated disclosure.

Fallback: email **abdulsdevworkspace@gmail.com** with the subject
`[SECURITY] Sill: <short summary>`.

A useful report includes: affected version/commit, platform, a description of
the issue and its impact, and reproduction steps or a proof of concept.
Partial reports are still welcome.

## What to expect

Sill has a single volunteer maintainer; the following are good-faith targets,
not a contract:

| Stage                  | Target                                                          |
| ---------------------- | --------------------------------------------------------------- |
| Acknowledgement        | within 72 hours                                                 |
| Initial assessment     | within 7 days                                                   |
| Fix or mitigation plan | depends on severity; communicated in the advisory               |
| Public disclosure      | coordinated with the reporter, after a fix or mitigation exists |

Please allow reasonable time for a fix before public disclosure. Good-faith
security research on your own installations is welcome; testing against other
people's systems is not.

Credit is given in the advisory and release notes unless you prefer otherwise.
There is currently no bug bounty; there is no budget for one.

## Supported versions

Sill is pre-alpha and has **no released versions yet**. Until a first release
exists, security fixes land on `main` only.

| Version             | Supported |
| ------------------- | --------- |
| `main` (unreleased) | ✅        |

Once releases begin, this table will state which release lines receive fixes
(expected policy: latest release only, while pre-1.0).

## In scope

- Sill application code (`src/`, `src-tauri/`)
- Sill's build, release, and update infrastructure in this repository
- Vulnerability classes we especially care about, given what a terminal is:
  escape-sequence handling, paste handling, clipboard access (OSC 52),
  IPC/webview boundary issues, command/argument injection, privilege of
  spawned processes, and anything that lets terminal _output_ cause
  unintended _input_ or code execution.

See the [threat model](docs/SECURITY_THREAT_MODEL.md) for how we think about
this space.

## Out of scope

- Vulnerabilities in dependencies (report upstream; tell us too if Sill's use
  is exploitable)
- Social engineering, physical attacks
- Issues requiring an already-compromised machine or malicious local root

## Security advisories

Fixed vulnerabilities are published as
[GitHub Security Advisories](https://github.com/bloxy-studios/sill/security/advisories)
with CVE requests where appropriate, and noted in the changelog under
`Security`.
