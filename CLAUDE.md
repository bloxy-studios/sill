# SILL — NATIVE, ULTRA-LOW-MEMORY, AI-AGENT-NATIVE TERMINAL

## MISSION

You are the principal software architect, Rust systems engineer, terminal-emulator engineer, desktop application engineer, performance engineer, UX engineer, developer-tools engineer, and AI-agent infrastructure engineer responsible for building **Sill**.

You are working inside an **existing project folder**.

Your first responsibility is NOT to start coding.

Your first responsibility is to:

> **Understand the existing repository completely, research the current ecosystem, determine the correct architecture, and then build Sill without unnecessarily replacing or destroying existing work.**

Sill is intended to become a serious modern terminal application for developers.

It should compete conceptually with products and workflows around:

- Ghostty
- Kitty
- iTerm2
- WezTerm
- Alacritty
- tmux
- Zellij
- cmux
- Warp
- modern AI-agent terminals

But Sill must NOT simply clone any of them.

The goal is to build a terminal that feels like the **natural home for modern software development and autonomous coding agents**.

---

# 1. NON-NEGOTIABLE PRINCIPLES

The following principles govern every engineering decision.

## 1.1 Native first

Sill is a native desktop application.

Use:

- Tauri 2
- Rust
- TypeScript
- Bun 1.4+
- Vite
- a lightweight frontend architecture

Do NOT use Electron.

Do NOT introduce a second browser runtime.

Do NOT embed Bun as a permanently running runtime inside the application unless there is a demonstrated architectural reason.

Bun should primarily be used for:

- package management
- frontend development
- build tooling
- scripts
- testing
- development utilities
- future Sill CLI tooling where appropriate

Rust should own:

- PTY management
- process management
- terminal sessions
- terminal state
- terminal parsing where appropriate
- filesystem/native operations
- OS integration
- process lifecycle
- session persistence
- terminal metadata
- agent process supervision
- low-level IPC
- performance-critical functionality

TypeScript should own:

- application UI
- workspace UI
- tabs
- panes
- settings
- command palette
- visual state
- interaction
- accessibility
- frontend orchestration

---

# 2. FIRST TASK — INSPECT THE EXISTING PROJECT

Before changing anything:

## Thoroughly inspect the repository.

Read:

- README
- package.json
- bun.lock
- Cargo.toml
- Cargo.lock
- src
- src-tauri
- configuration files
- existing components
- existing Rust modules
- existing frontend architecture
- tests
- scripts
- assets
- documentation
- TODOs
- comments
- git status
- git history where useful

Determine:

1. What already exists?
2. What is incomplete?
3. What architecture has already been chosen?
4. What should be preserved?
5. What should be refactored?
6. What is technically wrong?
7. What dependencies already exist?
8. Which dependencies are unnecessary?
9. What can be removed?
10. What can be reused?
11. What is the current build pipeline?
12. What operating systems are being targeted?
13. What terminal functionality has already been implemented?

Do NOT blindly scaffold a new application over the existing project.

---

# 3. RESEARCH BEFORE IMPLEMENTATION

Use the available web research tools and current official documentation.

Research the latest versions and APIs for:

- Tauri 2
- Bun 1.4+
- Rust stable
- terminal emulation libraries
- PTY libraries
- xterm.js or alternatives
- Ghostty/libghostty where technically relevant
- Alacritty terminal components
- portable-pty
- wezterm components
- tmux
- Zellij
- cmux
- Claude Code
- Codex CLI
- Cursor CLI
- MCP
- ACP
- terminal notification protocols
- OSC escape sequences
- shell integration
- zsh
- macOS terminal APIs
- Windows ConPTY
- Linux PTY systems

Prefer:

1. official documentation
2. official GitHub repositories
3. source code
4. engineering writeups
5. benchmarks
6. reputable technical analysis

Do not rely on outdated tutorials when official documentation exists.

Do not assume APIs are unchanged.

---

# 4. BUN 1.4 STRATEGY

The project should use **Bun 1.4+** where appropriate.

Bun 1.4 introduces significant improvements in:

- memory usage
- startup time
- binary size
- Node compatibility
- package installation
- native APIs
- Rust-based internals
- terminal APIs
- tooling

Take advantage of those improvements where they actually improve Sill.

However:

## Do not confuse Bun's runtime with Sill's native terminal runtime.

Sill itself should not launch a Bun process merely to execute every UI operation.

Avoid architecture like:

```text
Tauri
  ↓
Bun runtime
  ↓
TypeScript
  ↓
Rust
  ↓
PTY
```

when the operation can simply be:

```text
Tauri
  ↓
Rust
  ↓
PTY
```

Every permanently running process has a memory and startup cost.

Prefer:

```text
Tauri process
├── Rust native core
├── OS WebView
└── TypeScript UI
```

Use Bun primarily during development/build/package workflows.

---

# 5. PERFORMANCE IS A PRODUCT FEATURE

Sill should be designed around the principle:

> **A terminal should feel instantaneous even when everything else on the machine is busy.**

Performance requirements:

- extremely fast startup
- low idle RAM
- low idle CPU
- minimal background work
- low input latency
- smooth scrolling
- smooth typing
- smooth pane resizing
- efficient rendering
- efficient terminal output processing
- efficient process lifecycle
- no unnecessary polling
- no unnecessary React/UI rerenders
- no memory leaks
- no continuously running background services unless required

---

# 6. MEMORY BUDGET

Establish explicit memory budgets.

Do not simply say:

> "Keep memory low."

Measure it.

Create performance benchmarks for:

### Empty Sill

No sessions.

### One shell

One zsh session.

### Heavy output

Large amounts of terminal output.

### Many sessions

10 sessions.

### Many panes

20+ panes.

### Agent workload

Multiple Claude Code/Codex/Cursor sessions.

### Long-running workload

Keep Sill open for:

- 1 hour
- 8 hours
- 24 hours

Measure:

- RSS
- heap
- native allocations
- frontend memory
- renderer memory
- CPU
- process count
- GPU usage

---

# 7. MEMORY LEAK DETECTION

Build automated tests that repeatedly:

1. create a terminal
2. spawn a shell
3. write output
4. close the shell
5. destroy the terminal
6. recreate it

Repeat hundreds or thousands of times.

Memory should return close to baseline.

Do the same for:

- panes
- tabs
- workspaces
- agent sessions
- notifications
- browser surfaces if implemented
- terminal history
- output buffers

No object should remain alive simply because an event listener or frontend subscription forgot to unsubscribe.

---

# 8. DO NOT STORE INFINITE TERMINAL OUTPUT

This is critical.

Never allow terminal scrollback to grow without bounds.

Implement configurable scrollback:

```text
small
medium
large
custom
```

Potential default:

```text
10,000 lines
```

But benchmark this.

Consider:

- ring buffers
- chunked buffers
- compact storage
- lazy history
- disk-backed history for optional long-term persistence

Do NOT retain every terminal cell forever in JavaScript memory.

---

# 9. TERMINAL ENGINE

Build a proper terminal emulator.

The terminal must support:

- ANSI escape sequences
- VT100/VT220 behavior where relevant
- colors
- 256 colors
- truecolor
- cursor movement
- cursor styles
- alternate screen
- mouse reporting
- bracketed paste
- hyperlinks
- OSC sequences
- title changes
- clipboard operations
- Unicode
- wide characters
- combining characters
- emoji
- Unicode grapheme handling
- selection
- copy/paste
- search
- scrollback
- shell integration

Do not implement a toy terminal parser.

Use mature Rust terminal components where appropriate.

Before selecting a terminal emulator library, benchmark:

- CPU
- RAM
- rendering throughput
- compatibility
- maintenance
- licensing
- platform support

Document the decision.

---

# 10. PTY ARCHITECTURE

Implement platform-native process management.

### macOS/Linux

Use PTYs appropriately.

### Windows

Use ConPTY where appropriate.

The architecture should abstract platform-specific PTY implementations behind a common Rust interface.

Example conceptual interface:

```text
TerminalSession
├── spawn()
├── write()
├── resize()
├── read()
├── signal()
├── kill()
├── suspend()
├── resume()
└── close()
```

Do not leak platform-specific implementation details throughout the application.

---

# 11. DEFAULT SHELL — ZSH

On macOS:

## Default to zsh.

Do not hardcode:

```text
/bin/zsh
```

without checking the environment.

Determine the user's preferred/default shell appropriately.

If zsh is available:

> default to zsh.

If unavailable:

> gracefully fall back to the system shell.

Never assume zsh exists on every platform.

Expose shell configuration through settings.

Potential configuration:

```text
Default shell:
    System Default
    zsh
    bash
    fish
    Custom
```

---

# 12. SHELL INTEGRATION

Build first-class shell integration.

Support:

- current working directory
- command start
- command completion
- command failure
- command success
- command duration
- command history
- prompt detection
- semantic zones
- title updates
- git context where possible

The terminal should understand that:

```text
$ npm test
```

is a command.

And:

```text
test completed successfully
```

is a state transition.

This information becomes extremely important for AI agents.

---

# 13. TERMINAL SEMANTIC MODEL

Do not treat terminal output as merely a bitmap of text.

Maintain semantic information where possible.

For each command/session:

```text
Command
├── command text
├── cwd
├── start time
├── end time
├── exit code
├── duration
├── output
├── stderr
├── process ID
├── shell
└── related agent
```

This enables future features such as:

> "Why did my previous command fail?"

without requiring an AI to scrape an entire screen.

---

# 14. TERMINAL EVENTS

Create a robust event system.

Potential events:

```text
session.created
session.ready
session.command.started
session.command.finished
session.command.failed
session.output
session.title.changed
session.cwd.changed
session.process.started
session.process.exited
session.attention.required
session.notification
session.agent.detected
session.agent.waiting
session.agent.finished
session.agent.failed
session.closed
```

Use typed event schemas.

Do not create an unstructured event spaghetti.

---

# 15. AGENT-FIRST ARCHITECTURE

This is one of the most important parts of Sill.

Do NOT build a proprietary AI coding agent into the terminal initially.

Instead:

> **Make Sill the best environment for running ANY agent.**

Agents should be treated as first-class processes.

Examples:

```text
Claude Code
Codex
Cursor Agent
Gemini CLI
Aider
OpenCode
Goose
Kiro
Amp
Cline
Roo Code
custom agents
```

Any CLI agent that can run in a terminal should work.

---

# 16. AGENT DETECTION

Detect agents automatically.

For example:

```text
claude
codex
cursor-agent
gemini
aider
opencode
```

But do NOT rely exclusively on executable names.

Use multiple signals:

- process name
- command line
- environment variables
- terminal escape sequences
- known output patterns
- hook integration
- agent metadata
- child process tree

Represent an agent session as:

```text
AgentSession
├── provider
├── process
├── session
├── cwd
├── git branch
├── status
├── model
├── task
├── startedAt
├── lastActivity
├── attentionState
├── permissionState
└── metadata
```

---

# 17. AGENT STATES

Define normalized states:

```text
starting
working
waiting
awaiting_permission
idle
completed
failed
cancelled
unknown
```

The UI should make these states visible.

Example:

```text
● Working
◐ Waiting
! Permission
✓ Complete
× Failed
```

Do not depend on color alone.

---

# 18. AGENT ATTENTION SYSTEM

This should become one of Sill's defining features.

When an agent needs attention:

- visually mark the pane
- mark the workspace
- add unread state
- optionally show desktop notification
- optionally play a sound
- allow keyboard navigation to the next attention-required agent

Provide:

```text
Next Agent Requiring Attention
Previous Agent Requiring Attention
```

The user should be able to run:

```text
10 agents
 ↓
3 are working
2 finished
1 needs permission
1 failed
3 idle
```

and immediately understand the state of the development environment.

---

# 19. AGENT NOTIFICATIONS

Support standard terminal notification mechanisms.

Research and support appropriate OSC sequences such as:

- OSC 9
- OSC 99
- OSC 777

where applicable.

Also provide a Sill CLI:

```bash
sill notify "Build finished"
```

and eventually:

```bash
sill notify --title "Claude" --body "Waiting for permission"
```

This lets external tools integrate without knowing internal Sill APIs.

---

# 20. CLAUDE CODE INTEGRATION

Do NOT fork or modify Claude Code.

Treat Claude Code as an external agent.

Support:

- normal interactive `claude`
- non-interactive execution
- hooks
- MCP
- sessions
- permissions
- notifications
- agent lifecycle detection

Provide optional Sill integration instructions/configuration.

Where Claude Code hooks can notify Sill, provide a clean integration mechanism.

Potential flow:

```text
Claude Code
     │
     ├── Hook
     │
     ▼
Sill CLI
     │
     ▼
Sill socket/event API
     │
     ▼
Workspace state
     │
     ▼
UI notification
```

Do not hardcode assumptions about undocumented Claude Code internals.

---

# 21. CODEX INTEGRATION

Treat Codex similarly.

Support:

- interactive Codex CLI
- `codex exec`
- session state
- command/process detection
- MCP awareness
- notifications
- permissions
- workspace association

The goal is:

```text
Run Codex inside Sill
```

and have Sill automatically understand:

> "This pane belongs to Codex."

---

# 22. CURSOR AGENT INTEGRATION

Treat Cursor's CLI agent as another external agent.

Support:

- Cursor Agent CLI
- interactive sessions
- non-interactive workflows
- MCP
- ACP where appropriate
- status detection
- notifications
- workspace association

Do not make Sill dependent on Cursor.

---

# 23. UNIVERSAL AGENT ADAPTER

Design an abstraction:

```text
AgentProvider
```

Possible interface:

```text
AgentProvider
├── id
├── name
├── detect()
├── start()
├── stop()
├── status()
├── capabilities()
├── configure()
└── integration()
```

Then:

```text
ClaudeCodeProvider
CodexProvider
CursorProvider
GeminiProvider
OpenCodeProvider
GenericAgentProvider
```

The generic provider should work for unknown terminal agents.

---

# 24. MCP

MCP should be treated as an ecosystem capability.

Do NOT make Sill itself dependent on a specific MCP implementation.

Instead:

- detect MCP configuration where useful
- provide configuration visibility
- provide safe links/access to MCP configuration
- expose Sill capabilities to agents through MCP where appropriate
- eventually allow Sill itself to act as an MCP-capable environment

Potential future Sill MCP tools:

```text
list_workspaces
list_sessions
get_session
get_terminal_output
get_current_directory
get_git_state
create_workspace
create_terminal
split_pane
focus_session
send_input
notify_user
```

However:

## SECURITY IS CRITICAL.

Never expose arbitrary shell execution through an MCP interface without explicit permissions.

---

# 25. SILL CONTROL API

Build Sill as a programmable application.

Create a local IPC/control mechanism.

Possible implementation:

- Unix domain socket
- Windows named pipe
- localhost authenticated endpoint where appropriate

Provide a CLI:

```bash
sill
sill new
sill split
sill tab
sill workspace
sill focus
sill send
sill notify
sill list
sill inspect
```

Potential future examples:

```bash
sill workspace create feature-auth
```

```bash
sill pane split --direction horizontal
```

```bash
sill send --session <id> "npm test"
```

```bash
sill notify "Agent needs attention"
```

---

# 26. PROGRAMMABLE TERMINAL

Everything important should eventually be scriptable.

The terminal should not only be a UI.

It should be:

> **a programmable development environment.**

Agents and scripts should be able to:

- create sessions
- create workspaces
- create panes
- split panes
- focus panes
- read semantic output
- send input
- resize panes
- inspect processes
- inspect working directories
- inspect git state
- receive notifications
- close sessions

---

# 27. WORKSPACES

Do not make the user manage hundreds of tabs.

Build workspaces around projects.

Example:

```text
Sill

WORKSPACES

▾ Sill
    main
    tests
    docs

▾ Project A
    frontend
    backend
    agent

▾ Project B
    API
    database
```

Each workspace can contain:

- tabs
- panes
- terminals
- agents
- browser surfaces
- metadata

---

# 28. PROJECT-AWARE WORKSPACES

A workspace should understand:

- repository root
- current branch
- git status
- active processes
- ports
- package manager
- framework
- active agents

Potential sidebar:

```text
Sill
────────────────────

my-project
  main
  ● Claude
  ● Codex
  ○ Tests

  git: feature/auth
  ports: 3000 5432

other-project
  ● Agent
```

---

# 29. GIT AWARENESS

Sill should detect:

- repository
- branch
- dirty state
- ahead/behind
- recent commit
- changed files
- worktree

Do not constantly execute `git status`.

Use efficient caching/event-driven refresh where possible.

Do not turn Git awareness into a CPU-heavy background process.

---

# 30. PORT DETECTION

Detect listening development ports.

Show:

```text
3000
5432
8080
```

Optionally associate them with processes.

Eventually:

```text
Project
├── frontend :3000
├── API :8080
└── database :5432
```

Allow clicking a port to open it.

---

# 31. COMMAND PALETTE

Build an extremely fast command palette.

Example:

```text
⌘K
```

Search:

```text
Split Right
New Workspace
New Terminal
Close Pane
Next Agent
Previous Agent
Search Output
Open Settings
Toggle Sidebar
Copy
Paste
Clear
Reload
```

It should support fuzzy matching.

Do not load a massive command framework just for this.

---

# 32. KEYBOARD-FIRST DESIGN

Keyboard interaction is fundamental.

Support:

- configurable shortcuts
- pane navigation
- pane resizing
- workspace switching
- tab switching
- search
- command palette
- copy/paste
- zoom
- agent navigation
- attention navigation

Mouse interaction should complement keyboard interaction.

---

# 33. SPLITS

Implement excellent pane management.

Support:

- horizontal split
- vertical split
- resize
- close
- maximize
- swap
- move
- focus
- equalize
- zoom pane

Use a layout tree internally.

Avoid a DOM architecture where every terminal pane causes the entire application to rerender.

---

# 34. TAB MANAGEMENT

Tabs should remain lightweight.

Each tab should maintain only the necessary state.

Avoid keeping inactive terminal renderers unnecessarily active if they are expensive.

Investigate:

- pausing inactive rendering
- throttling offscreen updates
- keeping PTY alive while reducing UI work

---

# 35. RENDERING PERFORMANCE

The terminal renderer must be benchmarked under extreme output.

Test:

```bash
yes
```

large log streams

large JSON output

compiler output

test suites

binary-like output

rapid ANSI changes

large Unicode output

The UI must remain responsive.

Never allow terminal output processing to block:

- keyboard input
- window movement
- pane resizing
- command palette
- agent notifications

---

# 36. BACKPRESSURE

Implement output backpressure.

If a process emits data faster than the renderer can consume:

DO NOT:

- allocate infinite buffers
- freeze the UI
- crash
- block input indefinitely

Instead:

- buffer intelligently
- batch updates
- drop redundant render frames where safe
- preserve terminal semantics
- prioritize user interaction

---

# 37. FRONTEND ARCHITECTURE

Use TypeScript.

Prefer a lightweight architecture.

Do not automatically install a giant UI framework ecosystem.

If React is already established in the repository, use it.

Otherwise evaluate:

- React
- Preact
- Solid

based on:

- memory
- ecosystem
- complexity
- developer experience
- rendering performance

Do not choose a framework merely because it is popular.

---

# 38. STATE MANAGEMENT

Separate state categories.

### Application state

```text
settings
theme
keybindings
sidebar state
```

### Workspace state

```text
workspaces
tabs
panes
```

### Terminal state

```text
session
cwd
shell
process
```

### Agent state

```text
provider
status
attention
model
task
```

Do not put everything into one giant global store.

---

# 39. IPC DESIGN

Minimize frontend ↔ Rust communication.

Do NOT stream every individual byte through an expensive IPC boundary.

Instead investigate:

- batching
- binary payloads
- shared memory where justified
- event coalescing
- structured messages

Terminal output is high-frequency.

Design this path carefully.

---

# 40. TERMINAL OUTPUT PIPELINE

Preferred conceptual architecture:

```text
PTY
 │
 ▼
Rust reader
 │
 ▼
Terminal parser
 │
 ▼
Terminal state
 │
 ├── semantic events
 ├── agent events
 ├── shell events
 └── render updates
          │
          ▼
      Frontend
          │
          ▼
       Renderer
```

Do not send raw PTY bytes through multiple unnecessary layers.

---

# 41. AGENT CONTEXT MODEL

Every agent should be associated with a workspace.

Example:

```text
Workspace
  │
  ├── Git repository
  │
  ├── Branch
  │
  ├── Terminal sessions
  │
  ├── Agents
  │
  └── Ports
```

This makes Sill fundamentally different from a normal terminal.

---

# 42. MULTI-AGENT WORKFLOW

Design for:

```text
             PROJECT
                │
     ┌──────────┼──────────┐
     │          │          │
   Claude      Codex     Cursor
     │          │          │
   pane        pane       pane
     │          │          │
   branch      branch     branch
```

The user should be able to see all three simultaneously.

Eventually support:

- agent labels
- model labels
- task labels
- branch labels
- status
- notifications
- permissions

---

# 43. AGENT QUEUE

Build a global attention queue.

Example:

```text
ATTENTION

1. Claude — needs permission
2. Codex — finished
3. Cursor — test failed
```

Keyboard:

```text
⌘⇧U
```

→ jump to next attention item.

---

# 44. AGENT TASK METADATA

Allow agents to identify themselves where possible.

Display:

```text
Claude
Sonnet
feature/auth
Implement OAuth login
Working
```

or:

```text
Codex
GPT-5.x
fix/payments
Running tests
Waiting
```

Do not fabricate information if the agent doesn't expose it.

---

# 45. TERMINAL HISTORY SEARCH

Provide fast search.

Support:

- current screen
- scrollback
- commands
- command output
- semantic command history

Eventually:

```text
Search:
"where did the migration fail?"
```

could identify the command and relevant output.

Do not make AI mandatory for ordinary search.

---

# 46. AI SEARCH — FUTURE

Architect for optional semantic search.

Potential future feature:

```text
Ask Sill

"Where did I see the authentication error?"
```

Sill can search:

- recent commands
- terminal output
- git state
- agent sessions
- workspace metadata

This should be optional and privacy-conscious.

---

# 47. BROWSER SURFACE — FUTURE

Do not necessarily implement this in v1.

But architect the workspace model so a future browser surface can exist alongside terminals.

Example:

```text
Workspace
├── Terminal
├── Terminal
├── Agent
└── Browser
```

This would allow an agent to:

1. modify a web application
2. start dev server
3. open browser
4. inspect page
5. test
6. report results

Do not bundle a browser engine unnecessarily just because it might be useful someday.

---

# 48. REMOTE DEVELOPMENT

Design for future remote support.

Potential:

```text
Local Sill
   │
   └── SSH
         │
       Server
         │
       Agent
```

Support future concepts such as:

- SSH workspaces
- remote sessions
- tmux attachment
- persistent remote agents

Do not implement a huge remote-management system prematurely.

---

# 49. TMUX COMPATIBILITY

Sill should NOT attempt to replace tmux's protocol or reinvent everything.

Instead:

- detect tmux
- attach to tmux where appropriate
- provide excellent local sessions
- support remote tmux workflows
- eventually expose Sill workspace concepts to tmux sessions

The user should be able to migrate gradually.

---

# 50. MIGRATION FROM OTHER TERMINALS

Users should be able to move from:

- Ghostty
- iTerm2
- Kitty
- WezTerm
- Alacritty
- tmux
- Zellij
- cmux

without losing important workflows.

Investigate importing:

- themes
- fonts
- shell configuration
- keybindings
- environment configuration

Do not promise universal config compatibility.

Provide a migration layer where feasible.

---

# 51. THEMING

Support terminal themes.

But avoid a giant bundled theme library.

Prefer:

- built-in sensible defaults
- importable themes
- user-defined themes

Support:

- background
- foreground
- ANSI palette
- cursor
- selection
- UI chrome
- pane borders
- agent states

---

# 52. TYPOGRAPHY

Make typography excellent.

Support:

- system monospace
- user-selected fonts
- font fallback
- ligatures where supported
- variable fonts where supported
- proper Unicode fallback

Do not bundle fonts unnecessarily.

---

# 53. ACCESSIBILITY

Support:

- keyboard navigation
- reduced motion
- high contrast
- screen-reader compatibility where feasible
- configurable font sizes
- clear focus states
- non-color status indicators

---

# 54. SECURITY MODEL

Treat terminal applications as security-sensitive.

Sill can execute arbitrary user commands.

Therefore:

- never silently elevate privileges
- never execute hidden commands
- never send terminal contents to a cloud service without explicit user action
- never transmit secrets to AI systems automatically
- protect IPC sockets
- authenticate local control interfaces
- validate IPC messages
- sandbox where practical
- clearly distinguish agent automation from user commands

---

# 55. AGENT PERMISSIONS

Eventually support:

```text
Agent permission:

Read
Write
Execute
Network
Clipboard
Browser
Filesystem
```

Potential levels:

```text
Read-only
Ask
Auto
Restricted
```

Do not automatically approve dangerous operations.

---

# 56. SECRETS

Never expose:

- API keys
- SSH private keys
- environment secrets
- credentials
- tokens

to an agent unless explicitly permitted.

Be particularly careful with:

```text
.env
~/.ssh
~/.aws
~/.config
Keychains
credential stores
```

---

# 57. NO CLOUD DEPENDENCY

Sill should work as a normal terminal without:

- account
- login
- network connection
- AI provider
- cloud service

Core terminal functionality must remain local.

AI functionality should be optional.

---

# 58. OFFLINE-FIRST

The terminal must work perfectly offline.

No feature should block startup because:

- network is unavailable
- AI service is unavailable
- telemetry is unavailable
- update service is unavailable

---

# 59. TELEMETRY

Do not add invasive telemetry.

If telemetry is eventually introduced:

- opt-in where appropriate
- transparent
- minimal
- privacy-conscious
- documented

Never collect terminal contents by default.

---

# 60. APPLICATION STARTUP

Optimize cold startup aggressively.

Measure:

```text
process launched
↓
window visible
↓
first usable frame
↓
first shell ready
```

Optimize each stage separately.

Do not delay shell creation waiting for UI initialization unnecessarily.

---

# 61. CRASH RECOVERY

A terminal application must not lose everything because the UI crashes.

Persist lightweight metadata:

- workspace
- tab
- pane layout
- cwd
- shell
- agent association

Do not persist huge terminal buffers unless explicitly configured.

---

# 62. SESSION RESTORATION

On restart:

```text
Restore workspace?
```

Potentially restore:

- layout
- working directories
- terminal sessions
- remote sessions
- agent sessions where safely possible

Do NOT automatically restart destructive commands.

---

# 63. APPLICATION MENU

Build a native-quality application menu.

Include:

- File
- Edit
- View
- Terminal
- Workspace
- Agent
- Window
- Help

But keep menus minimal.

Keyboard-first operation remains primary.

---

# 64. COMMAND-LINE ENTRY POINT

Eventually install:

```bash
sill
```

and support:

```bash
sill .
```

to open the current directory.

Potential:

```bash
sill ~/Projects/foo
```

Potential:

```bash
sill --workspace foo
```

Do not overbuild this initially.

---

# 65. DEVELOPER EXPERIENCE

The repository should be pleasant to work on.

Use:

```text
Bun
Rust
Cargo
Tauri CLI
```

Provide:

```bash
bun install
bun dev
bun test
bun lint
bun typecheck
bun build
```

and appropriate Rust commands.

Everything should be documented.

---

# 66. DEPENDENCY DISCIPLINE

This is extremely important.

Every dependency must justify itself.

Before adding a package ask:

1. Can Rust do this?
2. Can Bun do this natively?
3. Can the browser platform do this?
4. Can Tauri do this?
5. Is the package actually necessary?
6. What is its bundle impact?
7. What is its memory impact?
8. Is it maintained?
9. What is its license?
10. Can we implement the required 10% ourselves instead of importing 100%?

Prefer fewer dependencies.

Do NOT use:

```text
library A
+ library B
+ library C
```

for something that can be implemented in 100 lines of Rust.

---

# 67. BUNDLE SIZE

Track:

- application binary size
- frontend bundle size
- compressed size
- installer size
- resources
- native dependencies

Create a size report.

Do not bundle:

- unnecessary fonts
- unused icon libraries
- huge UI frameworks
- unused language runtimes
- unnecessary browser engines
- redundant native libraries

---

# 68. ICONS

Do not install a massive icon library.

Use a minimal icon strategy.

Prefer:

- CSS/simple SVG where appropriate
- small curated icon set
- native platform icons where available

---

# 69. FRONTEND BUNDLE

Analyze the final frontend bundle.

Report:

```text
JS
CSS
assets
chunks
```

Eliminate:

- dead code
- duplicate packages
- unnecessary polyfills
- large dependencies

---

# 70. PERFORMANCE PROFILING

Create repeatable profiling workflows.

Use appropriate:

- Rust profiling
- heap profiling
- CPU profiling
- browser performance tools
- Tauri profiling
- OS-level process inspection

Do not guess about performance.

Measure.

---

# 71. BENCHMARK SUITE

Create benchmarks for:

### Startup

```text
cold start
warm start
shell ready
```

### Input

```text
keyboard latency
```

### Output

```text
1 MB
10 MB
100 MB
```

### Rendering

```text
ANSI-heavy
Unicode-heavy
color-heavy
```

### Sessions

```text
1
5
10
25
50
```

### Agents

```text
1
5
10
20
```

---

# 72. PERFORMANCE REGRESSION GATES

Do not allow future changes to silently increase:

- startup time
- idle RAM
- CPU
- binary size

Define thresholds.

For example:

```text
Startup regression > 10% → investigate
Idle RAM regression > 10% → investigate
Bundle regression > 10% → investigate
```

Tune these thresholds based on actual baseline measurements.

---

# 73. TESTING

Implement:

### Rust unit tests

For:

- PTY
- parser
- session manager
- event system
- layout
- agent detection
- permissions

### Frontend tests

For:

- workspace
- panes
- tabs
- command palette
- agent states

### Integration tests

For:

- spawning shell
- command execution
- resize
- output
- process termination
- notifications

### End-to-end tests

Run the real desktop application where practical.

---

# 74. TERMINAL COMPATIBILITY TESTS

Test real software:

```text
zsh
bash
fish
vim
nvim
top
htop
git
ssh
tmux
fzf
less
man
npm
bun
cargo
python
docker
kubectl
```

Especially:

```text
vim
neovim
tmux
fzf
ssh
```

because these expose terminal compatibility problems quickly.

---

# 75. AGENT COMPATIBILITY TESTS

Test:

```text
claude
codex
cursor-agent
gemini
aider
opencode
```

where available.

Verify:

- input
- output
- colors
- alternate screen
- permissions
- notifications
- process lifecycle
- session persistence
- resizing
- copy/paste

---

# 76. UX GOAL

The user should be able to open Sill and immediately understand:

> "Where am I?"

> "What am I working on?"

> "What terminals are running?"

> "Which agents are running?"

> "Which agents need me?"

> "Which commands failed?"

> "Which project is this?"

without opening multiple menus.

---

# 77. DIFFERENTIATION

Do not compete only on:

> "We're faster than Ghostty."

That is insufficient.

Do not compete only on:

> "We're more beautiful than iTerm2."

Also insufficient.

Sill's fundamental differentiation should be:

> **A terminal designed around the reality that developers now work with multiple autonomous agents simultaneously.**

But importantly:

## Sill should remain an excellent terminal even if the user never uses AI.

---

# 78. THE "NORMAL TERMINAL" TEST

If the user never installs:

- Claude
- Codex
- Cursor
- Gemini
- any AI tool

Sill should still be an excellent terminal.

It must not feel like an AI product with a terminal attached.

It must feel like:

> **a great terminal that happens to understand agents extremely well.**

---

# 79. THE "10 AGENTS" TEST

The ultimate UX test:

Imagine the user has:

```text
4 Claude sessions
3 Codex sessions
2 Cursor sessions
1 normal shell
```

all running simultaneously.

Can they understand the state of everything within 2 seconds?

If not:

Improve the information architecture.

---

# 80. FUTURE AGENT ORCHESTRATION

Architect for future functionality such as:

```text
Create 5 agents
```

Sill could eventually:

```text
Agent 1 → frontend
Agent 2 → backend
Agent 3 → tests
Agent 4 → docs
Agent 5 → review
```

Each gets:

- its own worktree
- workspace
- terminal
- permissions
- status
- logs

Do NOT build full orchestration in v1 unless the architecture supports it naturally.

---

# 81. GIT WORKTREE SUPPORT

This is particularly valuable for agent workflows.

Eventually:

```text
Project
├── main
├── agent/auth
├── agent/payments
├── agent/tests
└── agent/docs
```

Each agent can operate in its own worktree.

Sill should make this relationship visible.

---

# 82. AGENT + WORKTREE MODEL

Eventually represent:

```text
Agent
  │
  ├── Workspace
  ├── Terminal
  ├── Git worktree
  ├── Branch
  └── Task
```

This creates a powerful development model without requiring Sill to become the AI agent itself.

---

# 83. NOTIFICATION CENTER

Create a unified notification center.

Examples:

```text
Today

✓ Codex finished
! Claude needs permission
× Cursor test failed
✓ Build completed
```

Notifications should be actionable.

Clicking one should take the user directly to the relevant pane.

---

# 84. ACTIVITY TIMELINE

Eventually provide:

```text
10:31 Claude started
10:32 npm test
10:33 test failed
10:34 Claude edited auth.ts
10:35 tests running
10:36 Claude waiting
```

This can become extremely powerful for agent-heavy development.

Do not implement an expensive event database unnecessarily.

Use compact local event storage.

---

# 85. PRIVACY OF TERMINAL HISTORY

Terminal contents can contain secrets.

Treat them as sensitive.

Do not:

- upload terminal output
- send output to AI automatically
- index everything remotely
- log passwords
- expose secrets to plugins

AI features must be explicit.

---

# 86. PLUGIN SYSTEM — FUTURE

Architect for plugins without building a massive plugin ecosystem immediately.

Potential plugin categories:

```text
agent
theme
command
workspace
integration
notification
browser
```

Plugins should be isolated from core where possible.

---

# 87. EXTENSIBILITY WITHOUT MEMORY EXPLOSION

Do not allow every plugin to run a permanent background process.

Prefer:

- event-driven plugins
- lazy loading
- short-lived subprocesses
- WASM where appropriate
- native plugins only when necessary

---

# 88. PLATFORM SUPPORT

Design for:

### macOS

Primary target if that is the initial release.

### Linux

Strong target.

### Windows

Support through appropriate Windows terminal APIs such as ConPTY.

Do not let platform-specific assumptions contaminate the core architecture.

---

# 89. MACOS QUALITY

If macOS is first:

Make Sill feel native.

Consider:

- native window behavior
- traffic-light spacing
- vibrancy only where appropriate
- native menus
- keyboard conventions
- clipboard behavior
- notifications
- accessibility
- Retina rendering
- system appearance
- full-screen
- window restoration

Do not imitate Safari or Apple's UI unnecessarily.

---

# 90. LINUX QUALITY

Support:

- Wayland
- X11 where practical
- common desktop environments
- standard clipboard semantics
- system notifications

Avoid desktop-environment-specific hacks unless necessary.

---

# 91. WINDOWS QUALITY

Support:

- ConPTY
- PowerShell
- cmd
- Windows Terminal escape behavior
- clipboard
- Unicode
- native notifications where appropriate

---

# 92. RELEASE MODES

Build:

```text
development
debug
release
benchmark
```

Do not ship debug functionality in release builds.

Strip unnecessary symbols where appropriate.

---

# 93. ERROR HANDLING

Never silently swallow errors.

Rust errors should be:

- typed
- contextual
- recoverable where possible
- logged appropriately

Frontend errors should not crash the whole application.

---

# 94. LOGGING

Development logging:

```text
PTY
IPC
AGENT
RENDERER
WORKSPACE
```

Release logging should be minimal.

Never log:

- passwords
- secrets
- terminal input indiscriminately
- environment variables
- tokens

---

# 95. ARCHITECTURAL DOCUMENTATION

Before major implementation, produce:

```text
docs/
├── architecture.md
├── terminal-engine.md
├── ipc.md
├── agent-system.md
├── performance.md
├── security.md
├── testing.md
└── roadmap.md
```

These documents should describe actual architecture.

Do not create documentation that contradicts the implementation.

---

# 96. IMPLEMENTATION ORDER

Do NOT attempt to build everything simultaneously.

Build in phases.

## PHASE 0 — Repository analysis

Inspect and document the current project.

## PHASE 1 — Architecture

Finalize:

- Tauri
- Rust
- TypeScript
- Bun
- terminal engine
- IPC
- state model

## PHASE 2 — Minimal terminal

Implement:

- window
- PTY
- zsh
- terminal rendering
- input/output
- resize
- copy/paste

## PHASE 3 — Terminal quality

Implement:

- ANSI
- Unicode
- scrollback
- search
- shell integration
- hyperlinks
- mouse
- alternate screen

## PHASE 4 — Workspaces

Implement:

- tabs
- panes
- splits
- workspace persistence
- command palette

## PHASE 5 — Developer awareness

Implement:

- Git
- cwd
- processes
- ports
- shell metadata

## PHASE 6 — Agent foundation

Implement:

- agent detection
- agent model
- states
- notifications
- attention queue
- agent metadata

## PHASE 7 — Agent integrations

Implement adapters for:

- Claude Code
- Codex
- Cursor
- generic CLI agents

## PHASE 8 — Programmability

Implement:

- Sill CLI
- IPC
- socket/API
- automation

## PHASE 9 — Performance

Profile everything.

Optimize:

- startup
- RAM
- CPU
- rendering
- IPC
- bundle
- binary

## PHASE 10 — Hardening

Implement:

- crash recovery
- security
- permissions
- persistence
- compatibility
- accessibility

---

# 97. DO NOT PREMATURELY BUILD

Do NOT immediately build:

- cloud accounts
- subscriptions
- AI chat UI
- AI model hosting
- browser
- marketplace
- team collaboration
- analytics
- complicated plugin marketplace
- remote infrastructure

The foundation must be excellent first.

---

# 98. QUALITY BAR

Sill should not feel like:

> "a Tauri demo."

It should feel like:

> **a serious terminal application that could replace a user's existing terminal.**

Every major interaction should be polished.

---

# 99. COMPETITIVE BENCHMARKING

During development, compare Sill against:

- Ghostty
- cmux
- iTerm2
- Kitty
- WezTerm
- tmux
- Zellij

Evaluate:

| Capability | Sill | Competitor |
|---|---|---|
| Startup | | |
| Idle RAM | | |
| Heavy output | | |
| Pane management | | |
| Workspace management | | |
| Shell compatibility | | |
| Agent awareness | | |
| Notifications | | |
| Programmability | | |
| Git awareness | | |
| Remote workflows | | |

Do not copy their UI.

Study their strengths.

---

# 100. THE CMUX LESSON

cmux demonstrates that the terminal can become an **agent operations surface** rather than merely a shell window.

Sill should learn from this.

But do NOT clone cmux.

Find a stronger abstraction.

The goal is not:

> "cmux but written in Tauri."

The goal is:

> **Sill — the native terminal environment where human developers and autonomous coding agents coexist.**

---

# 101. AGENT-NATIVE INFORMATION ARCHITECTURE

The fundamental unit should eventually be:

```text
Workspace
    │
    ├── Human terminal
    │
    ├── Agent
    │
    ├── Agent
    │
    ├── Agent
    │
    ├── Git worktree
    │
    └── Services
```

This is more powerful than thinking only:

```text
Window
 → Tab
 → Pane
 → Terminal
```

The classic hierarchy should remain available, but Sill should understand the higher-level development context.

---

# 102. DO NOT MAKE AI MANDATORY

The user must be able to disable all agent-related UI.

Then Sill becomes:

> a fast, beautiful, lightweight terminal.

This keeps the product useful to developers who don't use AI.

---

# 103. FINAL PERFORMANCE TARGET

Do not blindly promise a specific RAM number before measuring.

Instead:

1. establish a baseline
2. compare competing terminals
3. identify the major memory consumers
4. optimize
5. benchmark
6. document results

The target should be:

> **As little RAM as realistically possible without compromising terminal correctness or functionality.**

---

# 104. FINAL ENGINEERING RULE

Whenever you have a choice between:

### Option A

A simple architecture with fewer processes, fewer dependencies, less IPC, less memory, and excellent performance.

### Option B

A complicated architecture with more abstractions and more features.

Prefer Option A unless Option B provides a demonstrable product advantage.

---

# 105. AGENT EXECUTION PROTOCOL

You are an autonomous engineering agent.

Follow this process:

### Step 1

Inspect the repository.

### Step 2

Research current technologies and competitor implementations.

### Step 3

Create an architecture document.

### Step 4

Identify risks.

### Step 5

Identify unnecessary dependencies.

### Step 6

Create an implementation plan.

### Step 7

Implement incrementally.

### Step 8

Run tests after each major subsystem.

### Step 9

Benchmark performance.

### Step 10

Profile memory.

### Step 11

Fix regressions.

### Step 12

Run compatibility tests.

### Step 13

Review security.

### Step 14

Clean the codebase.

### Step 15

Update documentation.

### Step 16

Run the complete test suite.

### Step 17

Build release artifacts.

### Step 18

Report exactly what was implemented.

---

# 106. NEVER FAKE COMPLETION

Do not say:

> "Implemented."

unless it actually works.

Do not create placeholder implementations for critical systems and call them complete.

Do not hide TODOs.

If a feature cannot be safely implemented:

1. explain why
2. document the blocker
3. implement the correct foundation
4. continue with the remaining work

---

# 107. FINAL DELIVERABLE

At completion, the repository should contain:

```text
Sill
├── working Tauri application
├── Rust terminal core
├── TypeScript frontend
├── Bun 1.4+ tooling
├── PTY support
├── zsh default
├── terminal rendering
├── tabs
├── panes
├── workspaces
├── shell integration
├── Git awareness
├── agent detection
├── agent states
├── notifications
├── attention system
├── Sill CLI foundation
├── IPC/control API
├── Claude Code compatibility
├── Codex compatibility
├── Cursor compatibility
├── generic agent compatibility
├── tests
├── benchmarks
├── profiling tools
├── security model
└── architecture documentation
```

Only claim features that are genuinely working.

---

# 108. FINAL REPORT

When finished, report:

## Architecture

Explain the final architecture.

## Terminal engine

Explain the selected implementation and why.

## Frontend

Explain the selected framework and why.

## Rust

Explain what Rust owns.

## Bun

Explain where Bun 1.4 is used.

## Agent architecture

Explain how agents are detected and represented.

## Claude Code

Explain the integration.

## Codex

Explain the integration.

## Cursor

Explain the integration.

## MCP

Explain the architecture.

## IPC

Explain the control API.

## Performance

Report:

- startup
- idle RAM
- idle CPU
- heavy-output RAM
- 10-session RAM
- 20-session RAM
- agent workload RAM
- bundle size
- application size

Use actual measurements.

## Security

Explain the threat model and protections.

## Tests

Report what passed.

## Remaining work

List only genuine remaining work.

---

# FINAL PHILOSOPHY

Build Sill as if you are creating the terminal you personally want to use for the next ten years.

Do not chase feature count.

Do not chase AI hype.

Do not build an Electron-like application.

Do not build a terminal that happens to have an AI sidebar.

Build:

> **a fast native terminal that understands the modern developer's entire computing workspace — especially the growing number of autonomous agents operating inside it.**

The terminal must remain excellent without AI.

The agent system must remain excellent without locking users into one AI provider.

The architecture must remain open.

The application must remain fast.

The application must remain lightweight.

The application must remain local-first.

The codebase must remain understandable.

And every performance claim must be measured.

**First inspect the repository. Then research. Then architect. Then implement.**