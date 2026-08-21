# ADR-0003: Contribution licensing — inbound=outbound, no CLA, no DCO

- **Status:** Accepted
- **Date:** 2026-08-21

## Context

The project must be able to accept external PRs with clear licensing while
keeping first-contribution friction near zero. Options range from nothing to
CLA bureaucracy.

## Decision

**Inbound = outbound**: contributions are accepted under the project's own
terms (MIT OR Apache-2.0), stated in CONTRIBUTING.md using the standard
Rust-ecosystem clause (which mirrors Apache-2.0 §5). **No CLA. No mandatory
DCO sign-off.**

## Alternatives considered

- **CLA:** real friction, signals possible future relicensing, needs tooling
  and an entity to receive rights. Unjustified for this project.
- **DCO (`Signed-off-by`):** lightweight provenance, but every first-time
  contributor trips over it and it mostly certifies what inbound=outbound
  already states. Deferred, not condemned.

## Consequences

- ✅ Lowest-friction legal model that is still explicit and widely understood.
- ⚠️ Provenance relies on the CONTRIBUTING statement + review vigilance for
  copy-pasted incompatible code (checked in review; see PR checklist).
- Revisit trigger: significantly larger contributor base or an ecosystem/legal
  requirement → propose DCO via RFC. A CLA is not on the table.
