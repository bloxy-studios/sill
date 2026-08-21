# Deprecation Policy

**Current reality:** Sill is pre-alpha with no stable surface. Until 1.0,
anything may change in a MINOR release — flagged in the changelog under
**Breaking Changes** with migration notes, but without a deprecation period.
This document exists now so the policy is a promise made _before_ it's needed.

## What counts as public surface (once stable)

Configuration files, keybinding names, the `sill` CLI, the event/scripting
API, the agent integration protocol, and MCP surface. Internal Rust/TS APIs
are never covered ([API stability tiers](#api-stability-tiers)).

## Policy from 1.0 onward

1. **Deprecate before removing.** A deprecated surface keeps working for at
   least one MINOR release, emitting a visible-but-not-obnoxious warning with
   a link to migration notes.
2. **Document.** Changelog entry under _Deprecated_; migration path in docs.
3. **Remove** only in a MAJOR release (or a MINOR while pre-1.0 norms still
   apply), listed under _Removed / Breaking Changes_.
4. **Security exception.** A surface that is itself a vulnerability may be
   removed immediately, with an advisory explaining why the window was zero.

## API stability tiers

| Tier         | Meaning                                      | Examples                                                      |
| ------------ | -------------------------------------------- | ------------------------------------------------------------- |
| Stable       | Deprecation policy applies                   | (none yet — nothing is stable pre-1.0)                        |
| Experimental | May change/vanish with a changelog note only | future: agent protocol, MCP surface, CLI while incubating     |
| Internal     | No promises, ever                            | Rust crate internals, IPC message shapes, frontend components |

New surfaces launch as **Experimental** by default and are promoted to Stable
deliberately, via RFC when significant.
