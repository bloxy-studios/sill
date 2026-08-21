# Design: Workspace Model

**Status: PROPOSED — nothing implemented.** This is the product thesis's core
design (Roadmap Phase 3, informing Phase 1 data structures).

## The problem with window/tab/pane

Traditional terminal abstractions describe _rectangles_, not _work_. A
developer's actual mental model is: "the api repo, with a dev server running,
a test watcher, an agent fixing a bug on a worktree, and a scratch shell."
Today that maps to N anonymous tabs whose meaning lives in the developer's
head. Multiply by three projects and two machines, and state is lost on every
restart, and nothing can _answer_ "what's running where?"

## Proposed primitives

```
Workspace            e.g. "work", "oss"
└── Project          a repository or directory context
    ├── Worktree     git worktrees as first-class variants of a project
    └── Session      one PTY + its emulation state + metadata
        ├── kind:    shell | service | agent | task
        └── status:  idle | running | attention | exited
```

- **Session** is the atom (a PTY with identity and lifecycle) — not a pane.
  Where a session is _displayed_ (tab, split, background) is presentation,
  decoupled from existence. tmux proved this decoupling; Sill's bet is that
  it belongs in the terminal's native model, with project/worktree context
  attached.
- **Project** binds sessions to a directory/repo, enabling: "open a shell
  _here_", branch/worktree display, per-project env and defaults.
- **Workspace** groups projects for humans (work/personal/client), scoping
  switching, layout, and restore.
- **kind/status** are what make agents (Phase 4) and services legible later
  without privileging them now — an `agent` session is just a session whose
  status transitions matter more.

## Behaviors this unlocks (the point of the model)

1. Restart Sill → workspaces restore: projects, sessions (respawned, not
   resurrected — honest about what a PTY restart means), layout.
2. "Show me everything running across projects" — a real answer, from state,
   not from squinting at tab titles.
3. Worktree-parallel development (increasingly common _because_ of agents):
   same project, N worktrees, sessions grouped accordingly.
4. Future CLI/automation (Phase 5): `sill open --project api --kind service
-- npm run dev` addresses the model, not screen coordinates.

## Open questions (to resolve via RFC before Phase 3 implementation)

- Persistence format and location; what session metadata is stored vs
  ephemeral (see threat model T9 — scrollback persistence is opt-in)
- Whether "service" sessions get restart policies (supervisor-lite) or stay
  dumb PTYs (bias: dumb until proven otherwise)
- How much structure to impose on single-directory quick shells (a bare
  session with no project must stay zero-ceremony)
- Naming: these four nouns must survive contact with real users

## Constraints

- The model must never make plain "open a terminal, type things" slower or
  heavier. Zero-config default path is sacred.
- All state local; no cloud, no account (see security model).
