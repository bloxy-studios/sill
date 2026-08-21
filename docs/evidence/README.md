# Evidence

This directory is Sill's **evidence accumulation system**: a disciplined,
truthful record of what the project has actually done — releases, community
activity, adoption signals, security work, performance results. Its purpose
is that future claims (in funding applications, in the README, anywhere) can
point at primary evidence instead of adjectives.

**Founding state, 2026-08-21: the evidence base is empty.** That is recorded
rather than hidden. Files below contain structure and rules, not entries,
until reality provides entries.

## Rules (binding)

1. **Nothing is fabricated, estimated-as-fact, or backdated.** An entry
   exists because the thing happened and can be verified.
2. **Prefer publicly verifiable sources**: GitHub PRs/issues/releases/graphs,
   registries, advisories, published benchmark data. Preserve exact URLs.
3. **Classify maturity** per entry: `UNVERIFIED` → `INTERNAL` →
   `PUBLIC` → `THIRD-PARTY VERIFIED`. Internal evidence is never presented
   as independently verified.
4. **Claims map to evidence.** A claim stronger than its evidence gets
   weakened until it matches (see [IMPACT.md](IMPACT.md)).
5. **No vanity-metric optimization.** Metrics are recorded, not farmed;
   nothing here justifies star-begging or download inflation.

## Structure

| File                                           | Holds                                                       |
| ---------------------------------------------- | ----------------------------------------------------------- |
| [EVIDENCE_LOG.md](EVIDENCE_LOG.md)             | Chronological log of significant evidence items (the spine) |
| [METRICS.md](METRICS.md)                       | What is measured, how, and where snapshots live             |
| [snapshots/](snapshots/)                       | Point-in-time metric records (`YYYY-MM.md`)                 |
| [TECHNICAL_PROGRESS.md](TECHNICAL_PROGRESS.md) | Shipped technical work worth citing                         |
| [COMMUNITY.md](COMMUNITY.md)                   | Contributor/community evidence                              |
| [ADOPTION.md](ADOPTION.md)                     | Objectively measurable adoption signals                     |
| [SECURITY.md](SECURITY.md)                     | Security work: advisories handled, hardening shipped        |
| [RELEASE_HISTORY.md](RELEASE_HISTORY.md)       | Every release, with links                                   |
| [IMPACT.md](IMPACT.md)                         | Claim ↔ evidence mapping for external use                   |

The curated index for funding applications is
[docs/oss/EVIDENCE_PACKET.md](../oss/EVIDENCE_PACKET.md), which links here
rather than duplicating.
