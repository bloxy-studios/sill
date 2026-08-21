# Maintainers

Current maintainers:

| Name      | GitHub                                             | Areas                        |
| --------- | -------------------------------------------------- | ---------------------------- |
| Abdul Ali | [@bloxy-studios](https://github.com/bloxy-studios) | Everything (sole maintainer) |

This file doubles as the maintainer runbook so that maintenance knowledge is
not trapped in one person's head. See also [docs/BUS_FACTOR.md](docs/BUS_FACTOR.md).

## Responsibilities

- Triage issues ([docs/ISSUE_TRIAGE.md](docs/ISSUE_TRIAGE.md))
- Review and merge pull requests ([docs/PR_REVIEW.md](docs/PR_REVIEW.md))
- Handle security reports privately ([SECURITY.md](SECURITY.md), aim: acknowledge within 72 hours)
- Cut releases ([docs/RELEASING.md](docs/RELEASING.md))
- Keep dependencies current ([docs/DEPENDENCY_POLICY.md](docs/DEPENDENCY_POLICY.md))
- Enforce the code of conduct ([CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md))
- Keep [docs/PROJECT_STATUS.md](docs/PROJECT_STATUS.md) roughly current
- Administer any project funding transparently ([docs/FUNDING.md](docs/FUNDING.md))

## Repository administration checklist

Settings that live in the GitHub UI (not in this repo) and should be kept true:

- [ ] **Branch protection / ruleset on `main`**: require pull requests, require
      CI status checks (`CI / frontend`, `CI / rust`), block force pushes.
- [ ] **Private vulnerability reporting**: enabled (Settings → Security).
- [ ] **Secret scanning + push protection**: enabled.
- [ ] **Dependabot alerts + security updates**: enabled.
- [ ] **Discussions**: enabled, categories per [docs/COMMUNITY.md](docs/COMMUNITY.md).
- [ ] **Repository metadata**: description, topics, and social preview set
      (suggested topics: `terminal`, `rust`, `tauri`, `developer-tools`,
      `desktop`, `pty`, `coding-agents`).
- [ ] **Actions permissions**: default workflow permissions set to read-only.
- [ ] Labels created per [docs/LABELS.md](docs/LABELS.md).

## Release authority

Only maintainers may push tags and publish releases. Release workflow creates
**draft** releases; a human publishes them after checking artifacts. Signing
and notarization status is tracked in [docs/RELEASE_SECURITY.md](docs/RELEASE_SECURITY.md).

## Adding a maintainer

1. Sustained, high-quality contribution and review history.
2. Existing maintainer(s) propose publicly; invitation on agreement.
3. Add to this file, CODEOWNERS, and GitHub with least-privilege permissions.
4. New maintainers get review access first; release/security authority follows
   after further trust is established.

## Stepping down

Maintainers may step down at any time by PR against this file, transferring or
revoking their access. Emeritus maintainers are listed below.

### Emeritus

None yet.
