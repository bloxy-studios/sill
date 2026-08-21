# Problem Statement

_The technical/product thesis, written for a skeptical reviewer. No hype._

## The shape of modern terminal work

A working developer's terminal load in 2026 commonly includes, simultaneously:

- several repositories (or several git worktrees of one repository),
- long-running services (dev servers, watchers, tunnels),
- build/test runs,
- and increasingly, one or more **autonomous coding agents** — CLI programs
  (Claude Code, Codex CLI, Cursor's CLI, Gemini CLI, Aider, OpenCode…) that
  work for minutes-to-hours, pause to ask questions, and finish silently.

The agent development pattern specifically drives _session multiplication_:
practitioners run agents in parallel, frequently one per worktree. Anthropic,
OpenAI, and the agent-CLI ecosystem all document parallel/worktree workflows
as intended usage.

## The abstraction gap

Terminals model this workload as **windows, tabs, and panes** — anonymous
rectangles. The mapping between "rectangle 7" and "the migration agent on the
`auth-fix` worktree" lives only in the developer's head. Consequences:

1. **Attention routing fails.** Nothing tells you which of nine sessions
   needs input, finished, or failed. Agents idle for an hour because their
   question appeared in an unfocused tab. (This is the acute, new pain.)
2. **State dies on restart.** The workspace-in-your-head evaporates with
   every reboot; multiplexers (tmux) solve persistence but bolt a second
   model _inside_ the terminal's model, with its own UX tax.
3. **No queryable model.** "What's running across my projects?" has no
   answer a tool can give; the terminal doesn't know what a project is.
4. **Context switching is manual search.** Finding "the shell in that
   worktree" is visual scanning, not addressing.

## Why now

The gap is old; agents made it expensive. A human generates one stream of
work; N agents generate N streams that _block on the human_. The terminal —
the place agents actually run — is the natural point to surface that, and
none of the mainstream permissively-licensed terminals model it (see
[../COMPETITIVE_LANDSCAPE.md](../COMPETITIVE_LANDSCAPE.md); the two products
that do target it are copyleft and, in one case, built around a proprietary
agent platform).

## Constraints a solution must respect

- Must remain a **fast, correct terminal first** — the model can't tax the
  90% case of "open a shell and type."
- Must be **provider-neutral** — the agent market churns; betting a terminal
  on one vendor is planned obsolescence.
- Must be **local-first and account-free** — terminals see secrets;
  trust structure matters.
- Must stay useful with **zero AI** — the workspace/session model must carry
  its own weight (services, builds, plain shells).

Sill exists to test whether a terminal built on these constraints, with
work-shaped primitives instead of rectangle-shaped ones, is the right answer.
The proposed answer: [SOLUTION.md](SOLUTION.md).
