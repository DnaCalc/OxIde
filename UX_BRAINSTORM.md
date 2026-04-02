# OxIde UX Brainstorm

Working document for planning the next UX direction of `OxIde`.

Status:
- exploratory
- opinionated
- intended to provoke decisions
- not yet a locked product spec

This document uses Unicode box drawing, layout sketches, and palette tokens on purpose.
Yes, OxIde can use Unicode in docs and in the eventual product where terminal capabilities allow it.
Yes, OxIde can use richer modern color palettes, but the real product must still degrade cleanly to weaker terminals.

---

## 1. What Are We Building?

`OxIde` should not become “VS Code in a terminal”.

It should become:
- a modern terminal-native IDE for `OxVba`
- a project-aware, target-aware, language-service-aware console environment
- a direct host for `OxVba`, not a text editor that happens to shell out
- a tool that feels focused, deliberate, and instrument-like rather than generic

The UX target is:

```text
not web-app-in-a-box
not terminal-themed VS Code clone
not nostalgic retro cosplay only

but:

an intentional modern TUI IDE
with strong keyboard flow
with visible project state
with high information density
with low chrome overhead
with beautiful but disciplined rendering
```

The emotional reference is somewhere between:
- QuickBasic / VBA / Visual Studio shell clarity
- Helix / modern editor intentionality
- lazygit / k9s / modern TUI density and composure

---

## 2. Scope Of This Planning Pass

This pass is about:
- UX usage model
- modality strategy
- default keybinding and compatibility strategy
- screen-space strategy
- tool-surface composition, including Immediate Panel behavior
- interaction model for editing, running, debugging, and project management
- visual language and terminal affordances
- mouse policy
- terminal capability testing and setup UX
- what should feel “editor-like” vs “IDE-like”

This pass is not yet about:
- exact widget implementation
- exact FrankenTui APIs
- final keymap
- final debug engine behavior
- final theme implementation details

---

## 3. Hard Constraints

The UX is constrained by:
- terminal rendering
- keyboard-first interaction
- variable terminal size
- limited true layering compared with desktop GUI
- limited mouse assumptions
- focus clarity requirements
- text-cell layout instead of freeform pixels
- host/editor/runtime separation defined by the OxVba ownership split

Architectural constraints:
- `OxIde` owns UX, shell, flow, editor behavior, session orchestration
- `OxVba` owns project truth, workspace loading, semantic queries, diagnostics, symbols, hover, completions, definitions, references
- `OxIde` should not invent a duplicate project model
- `OxIde` should not route semantic UX through LSP

Practical constraints:
- must work acceptably at 80x25
- should shine at 120x40 and above
- should degrade gracefully without becoming ugly or unusable
- should work fully without mouse input
- should support mouse well when available
- should help users diagnose weak terminal/font/configuration setups

---

## 4. What Makes A TUI Different From A Web-Tech GUI?

### TUI strengths

- constant keyboard presence
- low-latency task switching
- dense information without pointer travel
- stable spatial memory from grid layouts
- text-first content is native, not simulated
- simpler mental model for panes, lists, logs, inspectors
- stronger “instrument panel” feel

### TUI weaknesses

- no real floating pixel-perfect composition
- no rich typography in the GUI sense
- no carefree use of hidden hover-only affordances
- animation budget is tiny
- discoverability can collapse if everything is key-driven and implicit
- too many panes become unreadable quickly

### Implication

The best TUI is not a reduced GUI.

It is a:
- focused console instrument
- visibly stateful workspace
- dense but legible cockpit
- keyboard-centered environment with explicit navigation surfaces

Bad TUI instinct:
- recreate web sidebars, toolbars, tabs, dialogs, and tiny widgets one-for-one

Good TUI instinct:
- use clear regions
- use strong panel identities
- make state explicit
- keep flows shallow
- keep important mode changes visible

---

## 5. Editor-Style vs IDE-Style

### Editor-style approach

Examples:
- VS Code
- Sublime Text
- many modal/non-modal editors with tooling attached

Characteristics:
- the editor is the center of gravity
- files are primary
- projects are often secondary shells around files
- everything tends toward extensibility and generality
- UI often optimizes for many tool types and many languages

Mental model:

```text
open files
attach tools
inspect results
customize endlessly
```

### IDE-style approach

Examples:
- VBA IDE
- Visual Studio
- Delphi
- Xcode, in some respects

Characteristics:
- solution/project/workspace is primary
- documents live inside a larger project model
- run/debug/build configuration is first-class
- structure, orchestration, and state are more explicit
- the shell communicates what the application is doing

Mental model:

```text
open workspace
inspect project structure
edit artifacts in context
build / run / debug with visible state
manage references, targets, and sessions as first-class concepts
```

### OxIde should lean where?

Recommendation:
- `OxIde` should be IDE-style in product identity
- but editor-fast in moment-to-moment editing

That means:
- project/workspace state should always matter
- editing should remain immediate and low-friction
- build/run/debug should not feel bolted on
- project/module/reference management should be visible and legible

Short version:

```text
OxIde should feel like:
an IDE shell with a very strong editor core

not:
a general editor that acquired an OxVba plugin
```

---

## 5.5 VBA IDE Compatibility As A UX Principle

This should be elevated from “nice to have” to a core adoption strategy.

Many of the right early users for `OxIde` will not be terminal-first users.
They will be:
- VBA IDE users
- Visual Studio users
- users carrying old muscle memory from project-oriented Windows IDEs

That means `OxIde` should deliberately reduce migration friction.

### Recommendation

Default to a VBA-IDE-compatible keybinding philosophy.

Not because OxIde should imitate the VBA IDE shell literally, but because:
- it lowers onboarding friction
- it respects existing muscle memory
- it reinforces the IDE-first identity
- it keeps OxIde from feeling like “yet another editor that wants retraining”

### Working rule

```text
When a well-known VBA IDE shortcut has a clear OxIde equivalent,
that shortcut should work by default unless there is a strong reason not to.
```

### This should apply to:

- text editing commands
- build/run/debug commands
- project/module management
- navigation between code and tool surfaces
- common project actions

### Important nuance

We do not need to emulate the literal VBA menu bar structure.

We do want to preserve:
- the command vocabulary
- the mnemonic expectations
- the keyboard sequencing style where it helps

Example:

```text
Alt+I, M
```

should be a valid path for:
- Insert
- Module

even if the visual structure is not a classic Windows menu bar.

This suggests a command system that supports:
- direct shortcuts
- mnemonic menu sequences
- palette commands

all mapping to the same underlying actions.

### Proposed command layering

```text
Layer 1: direct shortcut
  F5 = Run
  F8 = Step Into
  Shift+F8 = Step Over

Layer 2: mnemonic menu sequence
  Alt+I, M = Insert Module
  Alt+R, R = Run / Start

Layer 3: command palette / command line
  "Insert Module"
  "Run Project"
```

This is a strong TUI fit because it keeps:
- discoverability
- memorability
- power-user speed

without demanding a giant visible menu system at all times.

### Keybinding policy recommendation

Ship with:
- `VBA IDE Compatible` as the default keymap

Later optionally support:
- `OxIde Native`
- `Visual Studio-ish`
- maybe `Helix/Vim-inspired` only as an opt-in expert profile

The default should not optimize for terminal purists first.

It should optimize for:
- OxVba users
- VBA migrants
- project-oriented IDE users

---

## 6. Modality: Modal Or Non-Modal?

There are really three different modality questions:

1. text editing modality
2. task-mode modality
3. transient interaction modality

### 6.1 Text editing modality

Options:

#### A. Fully non-modal editing

Pros:
- familiar
- low learning barrier
- simple mental model

Cons:
- command density must go elsewhere
- harder to achieve very fast keyboard workflows without modifier overload

#### B. Fully modal editing

Pros:
- maximal keyboard efficiency
- powerful composable commands

Cons:
- high cognitive cost
- wrong default for an IDE-oriented product with many non-text workflows
- risks making the shell feel editor-dictated rather than IDE-dictated

#### C. Hybrid editing mode

Recommended.

Definition:
- text entry is non-modal by default while focused in editor
- command palette / shell command line / jump panels are transient modes
- optional future “command cursor” editing layer may exist, but not as the product’s core identity

Recommendation:
- do not make OxIde a Vim-like modal editor first
- keep editing non-modal by default
- use explicit focus regions and transient overlays instead of a hard modal editing religion

Compatibility note:
- the non-modal default aligns better with VBA IDE expectations
- this is another reason not to make Vim-style editing the identity of OxIde

---

### 6.2 Product task mode: edit vs run vs debug

Question:
Should these be hard modes?

Recommendation:
- not hard global modes
- but yes, strong workspace states

Better framing:

```text
OxIde should be stateful, not mode-confused.
```

Use:
- persistent workspace state
- contextual layout presets
- visible state strips
- debug surfaces that appear when active

Avoid:
- global mode flips that completely rewire the shell without strong cues

Recommended states:
- Editing
- Running
- Debugging
- Reviewing Results

These are not mutually exclusive total modes. They are shell states with different emphasis.

---

### 6.3 Transient interaction modes

These are good and necessary:
- command line
- palette
- quick-open
- symbol search
- reference picker
- completion menu
- debug step controls
- confirmation sheets

This kind of modality is healthy because it is:
- local
- visible
- temporary
- reversible

---

## 7. Recommended UX Usage Model

## Working thesis

`OxIde` should use:
- non-modal editing
- stateful IDE shell
- transient overlays and focused panels
- layout presets that respond to active task

### In one sentence

```text
Edit should feel immediate.
Project state should feel persistent.
Run/debug state should feel explicit.
Everything important should have a place on screen.
```

### Proposed usage model

#### Base behavior

- one active workspace
- one active document editor
- one primary left-side project navigator
- one lower utility surface
- one right contextual inspector surface
- one `Immediate Panel` available as a persistent or summonable tool surface

#### Focus model

- focus is always explicit
- active region is visually obvious
- keyboard shortcuts route by focused region first
- global commands stay global
- mouse clicks should change focus predictably when mouse support exists

#### Shell rhythm

```text
Navigate project
Open module/file/reference
Edit
See live diagnostics/semantic cues
Build or run
Inspect output/problems
Enter debug when needed
Return to editing without shell whiplash
```

---

## 7.6 Immediate Panel

The shell should make room for an `Immediate Panel`.

This should be treated as a first-class IDE surface, not an afterthought.

### Product role

The `Immediate Panel` is:
- part REPL
- part statement evaluator
- part debug-time inspection surface

It should exist:
- outside debugging
- during debugging
- as a stable shell concept, not a temporary debug-only console

### Ownership split

`OxVba` should own:
- execution/evaluation semantics
- the typed request/response contract
- debug-context-aware evaluation behavior

`OxIde` should own:
- panel presentation
- history UX
- layout placement
- focus behavior
- keyboard and mouse affordances
- result rendering

That means OxIde should plan the surface now even if the engine support lands later in OxVba.

### What it is not

It is not just:
- stdout/stderr output
- build log output
- a shell command line
- the debug trace stream

It is also not identical to:
- watches
- locals
- hover

Those are related but different.

### Distinction from nearby surfaces

```text
Output panel:
  passive results from build/run

Debug console:
  runtime/debug event stream and control messages

Immediate Panel:
  active user-driven evaluation surface

Watches:
  pinned recurring expressions

Hover:
  quick contextual inspection
```

### Core user stories

- evaluate an expression while editing
- test a statement or small fragment quickly
- inspect a value during a paused debug session
- call helper routines interactively when that becomes supported
- experiment without leaving the IDE shell
- promote a one-off expression into a watch

### Working UX model

The panel should support:
- input prompt
- history
- multiline input where needed
- result list with rich rendering
- keyboard recall of previous entries
- copy/reuse into editor or watch list

### Suggested layout role

The `Immediate Panel` belongs in the lower tool-surface family alongside:
- Output
- Problems
- Search
- References
- Debug Console

But it should also be easy to pop out or maximize when actively used.

### Baseline sketch

```text
┌──────────────────────────────────────────────────────────────────────────────┐
│ Immediate                                                                   │
├──────────────────────────────────────────────────────────────────────────────┤
│ > ?answer                                                                    │
│   Integer = 42                                                               │
│                                                                              │
│ > Print ComputeTax(100)                                                      │
│   21                                                                         │
│                                                                              │
│ >                                                                             │
└──────────────────────────────────────────────────────────────────────────────┘
```

### Debug-time sketch

```text
┌──────────────────────────────────────────────────────────────────────────────┐
│ Debug • Immediate • Frame: Main                                             │
├──────────────────────────────────────────────────────────────────────────────┤
│ > ?answer                                                                    │
│   Integer = 41                                                               │
│                                                                              │
│ > ?items.Count                                                                │
│   Long = 7                                                                   │
│                                                                              │
│ > Watches: Add `answer * 2`                                                  │
│                                                                              │
│ >                                                                             │
└──────────────────────────────────────────────────────────────────────────────┘
```

### Interaction model recommendation

- `Ctrl+G` remains a strong candidate for focus/open if it does not conflict with better compatibility goals
- the panel should be focusable like any other region
- results should be navigable
- pressing Enter should evaluate
- history navigation should be fast and obvious

### Presentation recommendation

The panel should feel:
- active
- interactive
- code-adjacent

not:
- like raw terminal output
- like a generic shell subprocess

### Strategic reason

This surface helps OxIde feel like a true IDE rather than just an editor plus build/run panels.

---

## 7.5 Mouse Policy

Mouse support should be full.

Mouse dependence should be zero.

That should be a hard UX rule.

### Recommended principle

```text
Anything possible with the mouse must also be possible quickly with the keyboard.
Nothing important should require the mouse.
```

### Mouse should support

- focus changes
- pane selection
- cursor placement in editor
- selection in editor
- scroll in lists, editors, output panes, inspectors
- clicking symbols/modules/references
- resizing splitters if FrankenTui supports it reliably
- clicking tabs or bottom-surface selectors
- hover-like reveal interactions where terminals permit it cleanly

### Mouse should not be required for

- opening modules
- creating modules
- running or debugging
- project management actions
- completion acceptance
- navigation between diagnostics/references
- layout changes
- command invocation

### Why this matters

The best TUI tools feel:
- great with the mouse
- never broken without it

This also fits:
- SSH / remote usage
- tmux / nested terminal usage
- accessibility of keyboard-only workflows

### UX consequence

Every mouse-visible affordance should have a visible keyboard counterpart.

For example:

```text
Mouse click     module entry
Keyboard        explorer focus + arrows + Enter

Mouse click     problem item
Keyboard        problems focus + arrows + Enter

Mouse scroll    diagnostics list
Keyboard        PgUp/PgDn, j/k, arrows, search
```

---

## 8. Screen Space Strategy

The biggest TUI mistake would be to copy desktop pane count blindly.

### The shell should prioritize:

1. editor surface
2. project structure
3. problems / output / immediate / debug console
4. contextual inspector

### Suggested persistent regions

```text
┌─────────────────────────────────────────────────────────────────────────────┐
│ Top Bar: workspace • target • config • mode/state • dirty • notifications │
├──────────────┬──────────────────────────────────────────────┬──────────────┤
│ Project      │ Editor                                       │ Inspector    │
│ explorer     │                                              │              │
│ modules      │                                              │ diagnostics  │
│ refs         │                                              │ symbols      │
│ targets      │                                              │ hover        │
│ actions      │                                              │ properties   │
├──────────────┴──────────────────────────────────────────────┴──────────────┤
│ Bottom utility surface: output • problems • references • immediate • debug │
├─────────────────────────────────────────────────────────────────────────────┤
│ Command/status strip                                                       │
└─────────────────────────────────────────────────────────────────────────────┘
```

### Why this shape works

- left side = navigation and structure
- center = primary work
- right side = contextual meaning
- bottom = event/result/task stream

This maps very well to:
- TUI spatial memory
- IDE-style workflows
- OxVba’s direct semantic surfaces

---

## 9. Layout Presets

Do not use one frozen layout for everything.

Use a small number of task-shaped layouts.

### 9.1 Edit layout

```text
┌──────────────────────────────────────────────────────────────────────────┐
│ Workspace: Payroll.basproj   Target: Exe   Profile: win-console         │
├──────────────┬───────────────────────────────────────────┬───────────────┤
│ Explorer     │ Module1.bas                                │ Inspector     │
│              │                                            │               │
│ > Module1    │ Sub Main()                                 │ Diagnostics   │
│   Module2    │     Dim answer As Integer                  │ 0 errors      │
│   Forms      │     answer = 40 + 2                        │ 1 warning     │
│   References │ End Sub                                    │               │
│              │                                            │ Symbols       │
│              │                                            │ Main          │
├──────────────┴───────────────────────────────────────────┴───────────────┤
│ Problems / Output                                                         │
└──────────────────────────────────────────────────────────────────────────┘
```

### 9.2 Run layout

```text
┌──────────────────────────────────────────────────────────────────────────┐
│ Running: Payroll.basproj   Target: Exe   Status: active                 │
├──────────────┬───────────────────────────────────────────┬───────────────┤
│ Explorer     │ Editor / active source                     │ Run status    │
│              │                                            │               │
│              │                                            │ last build    │
│              │                                            │ runtime state │
│              │                                            │ host status   │
├──────────────┴───────────────────────────────────────────┴───────────────┤
│ Output / Immediate / stdout / stderr / events                             │
└──────────────────────────────────────────────────────────────────────────┘
```

### 9.3 Debug layout

```text
┌──────────────────────────────────────────────────────────────────────────┐
│ Debugging: Payroll.basproj   Break: Module1.Main line 42                │
├──────────────┬───────────────────────────────────────────┬───────────────┤
│ Call stack   │ Source                                      │ Watches      │
│ > Main       │ ▌answer = ComputeTotal(items)               │ answer = 42  │
│   Compute    │  nextLine                                   │ items = ...  │
│   Helpers    │                                             │               │
│              │                                             │ Locals        │
│              │                                             │ answer: Int   │
├──────────────┼───────────────────────────────────────────┬─┴──────────────┤
│ Breakpoints  │ Debug console / Immediate / trace / evaluation               │
└──────────────┴──────────────────────────────────────────────────────────────┘
```

### 9.4 Project management layout

```text
┌──────────────────────────────────────────────────────────────────────────┐
│ Project: Payroll.basproj                                                │
├───────────────────┬──────────────────────────────┬──────────────────────┤
│ Project structure │ Details                      │ Actions              │
│                   │                              │                      │
│ Modules           │ selected item metadata       │ add module           │
│ References        │ target info                  │ add reference        │
│ Targets           │ runtime/profile/policy       │ change output type   │
│ Build configs     │ provenance/source of truth   │ validate             │
├───────────────────┴──────────────────────────────┴──────────────────────┤
│ Validation / helper output                                               │
└──────────────────────────────────────────────────────────────────────────┘
```

---

## 10. What FrankenTui Affordances Should Be Used Best?

We should treat FrankenTui as enabling:
- hard-edged regions
- strong layout systems
- stateful widgets
- keyboard focus routing
- explicit overlays/sheets
- text-rich rendering
- line/box/panel composition

We should not treat it as if its job is to imitate a browser.

Best-use affordances:

### A. Strong structural frames

Use:
- bordered panels
- titled regions
- stable placement
- active-region emphasis

Because in a TUI:
- structure is a substitute for freeform visual hierarchy

### B. Layered emphasis, not many tiny widgets

Prefer:
- a few large, legible zones
- internally structured content

Avoid:
- “dashboard confetti”
- five tiny inspectors at once

### C. Bottom surfaces for task streams

The bottom area is ideal for:
- build output
- run output
- problem list
- search results
- references
- immediate evaluation
- debug console

Because these are naturally:
- chronological
- list-shaped
- secondary to source editing

### D. Right-side contextual intelligence

The right side should host:
- hover detail
- symbol outline
- diagnostics summary
- watch values
- item properties

This matches the “meaning, not navigation” role.

### E. Overlays and sheets

Use for:
- quick open
- command palette
- go to symbol
- completion menus
- confirmations
- reference picker

This is where “modern TUI” can feel polished rather than cramped.

---

## 10.5 Hyprland / Helix Influence: What To Borrow, What Not To Borrow

The desired feel is closer to:
- Hyprland composure
- Helix clarity
- modern terminal instrument panels

Not:
- literal tiling-window-manager behavior inside the IDE
- editor modal ideology as product identity

### What to borrow from Hyprland-like aesthetics

- sharp contrast between active and inactive regions
- elegant dark surfaces
- quiet but vivid accent colors
- compositional confidence
- strong sense that the workspace is arranged intentionally

### What to borrow from Helix-like aesthetics

- clear active panel emphasis
- calm typography and spacing
- modern text UI minimalism
- strong selection/focus treatment
- immediate, low-chrome editing feel

### What not to borrow blindly

- extreme modality
- sparse feature discoverability
- aesthetics that depend on perfect terminal support without fallback

### Important practical note

Many beautiful modern TUI screenshots rely on:
- truecolor
- Nerd Fonts or equivalent patched fonts
- strong Unicode coverage
- well-configured terminal emulators
- high DPI and large terminal sizes

OxIde should benefit from those conditions.
OxIde should not assume them silently.

That leads directly to a product requirement:
- a console capability and setup experience must exist inside the product

---

## 11. Visual Language

The UI should not be monochrome-by-default unless the terminal forces it.

The visual character should be:
- clean
- dark-leaning but not muddy
- high-contrast
- restrained
- slightly luxurious
- modern, not gamer-neon

### Palette direction A: Slate + Teal + Amber

```text
Background      █  #0F172A
Panel           █  #111827
Panel Alt       █  #1F2937
Border          █  #334155
Text            █  #E5E7EB
Muted           █  #94A3B8
Primary         █  #2DD4BF
Accent          █  #60A5FA
Warn            █  #F59E0B
Error           █  #F87171
Success         █  #34D399
Selection       █  #1D4ED8
```

### Palette direction B: Graphite + Jade + Copper

```text
Background      █  #111111
Panel           █  #171717
Panel Alt       █  #262626
Border          █  #404040
Text            █  #F5F5F5
Muted           █  #A3A3A3
Primary         █  #10B981
Accent          █  #22D3EE
Warn            █  #FB923C
Error           █  #F43F5E
Success         █  #4ADE80
Selection       █  #14532D
```

### Palette direction C: Midnight + Orchid-free electric blue

Avoid purple bias.

```text
Background      █  #0B1020
Panel           █  #121A2B
Panel Alt       █  #172036
Border          █  #2A3550
Text            █  #ECF2FF
Muted           █  #94A3B8
Primary         █  #38BDF8
Accent          █  #14B8A6
Warn            █  #FBBF24
Error           █  #FB7185
Success         █  #22C55E
Selection       █  #1D4ED8
```

### UX note

A “modern TUI” palette should use color for:
- active focus
- diagnostic severity
- state transitions
- current mode emphasis
- semantic grouping

Not for:
- decorating every border
- rainbow noise

### Palette implementation note

OxIde should separate:
- design palette
- terminal capability reality

Meaning:
- prefer full truecolor themes when available
- degrade to 256-color safely
- degrade further to minimal ANSI when necessary
- keep layout, hierarchy, and focus legible even when color quality is poor

---

## 12. What Makes A Modern TUI IDE Feel Modern?

Not:
- rounded fake buttons everywhere
- excessive badges
- terminal gimmicks

But:

### 12.1 Clear statefulness

The shell should always answer:
- what workspace is active?
- what document is active?
- what target/profile/policy is active?
- is the buffer dirty?
- are we editing, running, or debugging?
- where is focus?

### 12.2 Intentional composition

Modern means:
- fewer but better regions
- explicit hierarchy
- quiet background
- strong accent use
- motion only where meaningful, if any

### 12.3 Fast command access

A palette or command strip should make advanced flows easy without making everything a memorized colon command forever.

### 12.4 Rich text-side intelligence

Hover, completion, diagnostics, references, symbols, watches, project metadata:
- these should feel integrated into one shell
- not like separate command outputs pretending to be UX

Immediate evaluation belongs in the same category:
- it should feel like a native IDE interaction surface
- not like a subprocess terminal pane

### 12.5 Great empty and small-screen states

A modern TUI looks good when:
- no project is open
- the terminal is narrow
- diagnostics are empty
- the debug session is idle

This matters more than in a desktop GUI.

---

## 13. Proposed Product Identity

From a UX perspective, OxIde should be:

```text
Project-first
Editor-fast
Inspector-rich
Keyboard-native
State-explicit
Semantics-visible
Run/debug-integrated
```

Not:

```text
file-juggling-first
extension-first
chrome-heavy
mouse-dependent
mode-obscure
```

---

## 14. Candidate Interaction Model

### Primary regions

```text
[1] top identity bar
[2] left project rail
[3] center editor
[4] right inspector
[5] bottom task/output surface
[6] command/status strip
```

### Suggested keyboard worldview

```text
Global navigation:
  workspace, palette, open, search, build, run, debug, focus-next

Editor-local:
  typing, movement, selection, editing, completion accept, hover inspect

Pane-local:
  list navigation, expand/collapse, actions, confirm/open
```

### Focus example

```text
Top Bar      passive
Explorer     active
Editor       passive
Inspector    passive
Bottom       passive
Status       passive
```

When focus changes, the shell should show it loudly.

Possible active styling:
- brighter border
- colored title
- stronger background
- command hint swap

---

## 15. Modal Questions: Specific Recommendations

### Recommendation 1

Editing should be non-modal by default.

### Recommendation 2

The shell should be stateful around:
- editing
- running
- debugging

But those should be visible workspace states, not mysterious mode traps.

### Recommendation 3

Transient overlays should be embraced:
- completion
- hover detail
- command palette
- quick open
- symbol picker
- reference picker

### Recommendation 4

Debugging should feel like a stronger shell state, not just another bottom pane.

When debug is active:
- stack
- watches
- locals
- breakpoints
- source focus
should all become first-class.

---

## 16. Specific UX Tensions To Decide

These are planning questions that need real decisions.

### Tension A: Command line vs command palette

Options:
- keep colon command line central
- add palette and demote colon
- support both

Working recommendation:
- keep both
- palette for discovery and broad command access
- colon line for power-user exact commands and script-like flows

### Tension B: Single editor vs tabs vs buffers list

Options:
- one visible editor only
- top tab strip
- hidden buffers with quick switch

Working recommendation:
- one visible editor
- lightweight tab/buffer strip if needed
- avoid browser-style tab overload

### Tension C: Explorer permanence

Options:
- always visible
- collapsible rail
- overlay-only

Working recommendation:
- visible by default
- collapsible in narrow terminals

### Tension D: Inspector permanence

Working recommendation:
- visible when wide enough
- collapsible / replaceable when narrow
- host context-sensitive content rather than many dedicated panes

### Tension E: Completion UX

Options:
- bottom pane only
- inline popup
- right inspector takeover

Working recommendation:
- inline or near-editor overlay first
- fallback to inspector/bottom view when constrained

### Tension F: Hover UX

Working recommendation:
- quick inline/adjacent overlay for immediate detail
- deeper semantic info can open in inspector

---

## 16.5 Keybinding Compatibility Strategy

This needs its own design pass, but the default direction should be clear now.

### Default profile

`VBA IDE Compatible`

### Scope of compatibility

- editor movement and editing where mappings are sensible
- run/build/debug actions
- stepping actions
- breakpoints
- navigation to project/module/symbol actions
- project insertion and management actions

### Example compatibility targets

```text
F5          Run
Ctrl+Break  Stop / interrupt if supported
F8          Step Into
Shift+F8    Step Over
Ctrl+Shift+F8 or equivalent  Step Out if mapped in product
F9          Toggle breakpoint
Ctrl+G      Immediate / command console analogue if we keep one
Alt+I, M    Insert Module
Alt+F, ...  File-family actions where useful
Alt+R, ...  Run-family actions where useful
Alt+D, ...  Debug-family actions where useful
```

Exact mapping details still need research and finalization.

The important planning decision is:
- the default should optimize for familiarity to VBA IDE users
- the command architecture should explicitly support mnemonic sequences

### UX consequence

OxIde should probably have a hidden or transient mnemonic menu model even if it does not draw a classic Windows menu bar permanently.

Possible realization:

```text
Press Alt
Top strip reveals mnemonic families

F  File
E  Edit
V  View
I  Insert
R  Run
D  Debug
T  Tools
H  Help

Then second key narrows to action
```

This gives us:
- VBA familiarity
- zero requirement to copy old menu chrome literally
- a TUI-native and keyboard-native solution

---

## 17. Small, Medium, Large Terminal Strategies

### Small: 80x25

Goal:
- one dominant editor
- one compact explorer or none
- one compact status line
- one transient bottom panel only when needed

Sketch:

```text
┌──────────────────────────────────────────────────────────────────────────┐
│ Top bar                                                                  │
├──────────────┬───────────────────────────────────────────────────────────┤
│ Explorer     │ Editor                                                    │
├──────────────┴───────────────────────────────────────────────────────────┤
│ Status / command / transient output                                      │
└──────────────────────────────────────────────────────────────────────────┘
```

### Medium: 120x35

Goal:
- full three-column editing shell
- bottom utility panel available

### Large: 160x50+

Goal:
- stable full IDE composition
- debug layouts become excellent
- project management surfaces become comfortable

---

## 17.5 Terminal Capability Testing And Setup

This should be a first-class area or page in the product.

Reason:
- modern TUI quality depends heavily on terminal capabilities
- users often blame the application for terminal, font, or emulator problems
- Windows in particular benefits from guided setup

OxIde should help the user answer:
- does this terminal support truecolor well?
- does Unicode render properly?
- does box drawing align?
- does the font support required glyphs?
- does mouse reporting work?
- does scrolling work in embedded panes?
- is the terminal emulator a good fit?

### Product requirement

Provide a dedicated:
- `Console Test`
- `Terminal Diagnostics`
- or `Display & Input Setup`

surface in the product.

This should not feel like a crash dump.
It should feel like a polished setup lab.

### Possible shell entry points

- Help > Console Setup
- Tools > Terminal Diagnostics
- command palette: `Console Test`
- mnemonic sequence such as `Alt+T, C`

### What the screen should test

#### 1. Geometry and borders

```text
This box should look perfectly square:

┌──────────────┐
│              │
│              │
│              │
└──────────────┘
```

#### 2. Line drawing alignment

```text
These lines should connect cleanly:

├────┬────┬────┤
│    │    │    │
└────┴────┴────┘
```

#### 3. Unicode sample rendering

```text
These should render correctly:

Duck:      🦆
Check:     ✓
Cross:     ✗
Arrow:     → ⇒
Blocks:    ░▒▓█
Braille:   ⠋⠗⠁⠝⠅⠑⠝
Box draw:  ╭─╮ ╰─╯
```

#### 4. Width behavior

```text
These columns should align:

ASCII     | hello |
Accented  | café  |
CJK-ish   | 表     |
Emoji     | 🦆     |
```

#### 5. Color capability

```text
16-color
256-color
truecolor gradient
severity colors
selection colors
```

#### 6. Mouse support

```text
- click this list
- drag selection in editor sample
- scroll this panel with mouse wheel
- verify focus follows click
```

#### 7. Keyboard capture

```text
Press:
F5
F8
Shift+F8
Alt+I, M
Ctrl+S
Alt

The screen should report exactly what OxIde sees.
```

### Sample diagnostics page sketch

```text
┌──────────────────────────────────────────────────────────────────────────────┐
│ Console Test • Terminal Diagnostics                                         │
├──────────────────────────────┬───────────────────────────────────────────────┤
│ Capability summary           │ Live test area                               │
│                              │                                               │
│ Emulator: Windows Terminal   │ square box test                              │
│ Colors: truecolor            │ unicode duck test                            │
│ Mouse: supported             │ border alignment test                        │
│ Font: unknown / detected     │ scroll region test                           │
│ Unicode width: warning       │ key capture test                             │
│                              │                                               │
├──────────────────────────────┴───────────────────────────────────────────────┤
│ Guidance / fixes                                                           │
│ - Install Windows Terminal                                                 │
│ - Use Cascadia Mono / Cascadia Code / suitable Nerd Font                   │
│ - Enable UTF-8 and truecolor-capable terminal settings                     │
│ - If duck is missing, your font lacks glyph support                        │
└──────────────────────────────────────────────────────────────────────────────┘
```

### Guidance philosophy

Do not merely report failure.

Report:
- what OxIde expected
- what likely failed
- what the user should do next

Example:

```text
Problem:
The duck glyph did not render.

Likely cause:
Current terminal font lacks the glyph.

Suggested fix:
Use Windows Terminal with Cascadia Mono, Cascadia Code, or another font with broad Unicode coverage.
```

### Windows-specific setup guidance

OxIde should explicitly recommend, where appropriate:
- Windows Terminal over legacy console hosts
- UTF-8 capable environment
- fonts with strong Unicode coverage
- truecolor-capable terminal settings
- sensible line-height and font-size defaults

This should be presented as:
- a guided setup checklist
- not a vague troubleshooting paragraph

### Why this matters strategically

If OxIde aims for a beautiful modern TUI identity, it must own part of the environment validation story.

Otherwise:
- users will see broken borders
- bad glyph fallback
- muddy colors
- mouse weirdness

and conclude that OxIde itself is poor.

---

## 18. Anti-Goals

Do not build:
- a terminal recreation of Electron chrome
- a pane farm with six permanently tiny panels
- a Vim clone disguised as an IDE
- a desktop-style property-sheet application awkwardly squeezed into text cells
- a nostalgic fake-1980s UI unless intentionally themeable as an optional skin
- a UX that assumes perfect terminal setup without helping the user verify it

Do build:
- something sharper
- something more instrument-like
- something confident about being terminal-native
- something that welcomes VBA IDE users instead of forcing total relearning

---

## 19. Suggested Near-Term Planning Tracks

This document should branch into follow-up design work:

### Track 1: UX principles

- define the shell principles formally
- define active/focused region language
- define visual hierarchy rules

### Track 2: layout system

- define layout presets
- define narrow/medium/wide breakpoints
- define inspector and bottom-surface roles

### Track 3: command model

- command line vs palette
- global commands vs focused commands
- keybinding philosophy
- VBA IDE shortcut research and mapping
- mnemonic Alt-sequence model

### Track 4: editing intelligence UX

- hover UX
- completion UX
- diagnostics UX
- references and definitions UX
- symbol navigation UX
- immediate evaluation surface UX

### Track 5: project/workspace management UX

- project explorer model
- module/reference management
- target/profile/policy surfaces
- workspace switching

### Track 6: run/debug UX

- run status model
- output/console model
- Immediate Panel model
- debug layout
- watch/locals/stack presentation

### Track 7: visual design system

- palette
- border/title treatments
- severity/status color system
- empty states
- active/focus visuals

### Track 8: console capability and setup experience

- capability detection model
- diagnostics/test page UX
- Windows Terminal and font guidance
- fallback behavior for weak terminals

---

## 20. Provisional Recommendation

If a single direction must be chosen now:

```text
OxIde should become a project-first, non-modal, keyboard-native TUI IDE
with a stable left/center/right/bottom shell,
with transient overlays for commands and semantic interactions,
with explicit edit/run/debug workspace states,
with VBA-IDE-compatible default keybindings,
with full mouse support but zero mouse dependency,
and with a visually modern but terminal-honest design language plus an explicit console setup story.
```

That is the best fit for:
- the OxVba ownership split
- a modern terminal implementation
- FrankenTui’s likely strengths
- the difference between a true IDE and a text editor with plugins

---

## 21. Sketches For Discussion

### A. “Balanced IDE” shell

```text
┌──────────────────────────────────────────────────────────────────────────────┐
│ OxIde • Payroll.basproj • Exe • win-console • clean • Edit                 │
├──────────────┬─────────────────────────────────────────────┬────────────────┤
│ Explorer     │ Editor                                      │ Inspector      │
│              │                                             │                │
│ Modules      │ Attribute VB_Name = "Module1"               │ Diagnostics    │
│ References   │                                             │ Symbols        │
│ Targets      │ Public Sub Main()                           │ Hover          │
│              │     Dim answer As Integer                   │ Properties     │
│              │     answer = 40 + 2                         │                │
│              │ End Sub                                     │                │
├──────────────┴─────────────────────────────────────────────┴────────────────┤
│ Problems • Output • Console • References • Search                            │
├──────────────────────────────────────────────────────────────────────────────┤
│ F1 Help  Ctrl-S Save  Ctrl-P Open  Ctrl-Shift-P Palette  F5 Run  F9 Break   │
└──────────────────────────────────────────────────────────────────────────────┘
```

### B. “Debug cockpit” shell

```text
┌──────────────────────────────────────────────────────────────────────────────┐
│ OxIde • Payroll.basproj • Debug • Paused at Module1.Main:42                │
├──────────────┬─────────────────────────────────────────────┬────────────────┤
│ Call Stack   │ Source                                      │ Watches        │
│ > Main       │  40 │ answer = 40 + 1                      │ answer = 41    │
│   Compute    │  41 │ If answer > 40 Then                  │ items = 7      │
│   Format     │▶ 42 │     Stop                              │                │
│              │  43 │ End If                                │ Locals         │
│ Breakpoints  │  44 │                                       │ answer: Integer│
├──────────────┼─────────────────────────────────────────────┴────────────────┤
│ Debug Console / Trace / Evaluate                                             │
└──────────────────────────────────────────────────────────────────────────────┘
```

### C. “Project management” shell

```text
┌──────────────────────────────────────────────────────────────────────────────┐
│ OxIde • Project Settings • Payroll.basproj                                  │
├──────────────────┬──────────────────────────────────────┬───────────────────┤
│ Project tree     │ Details                              │ Actions           │
│                  │                                      │                   │
│ > Modules        │ OutputType: Exe                      │ Add Module        │
│   References     │ ProjectName: Payroll                 │ Add Class         │
│   Targets        │ EntryPoint: Module1.Main             │ Add Reference     │
│   Runtime        │ RuntimeProfile: win-console          │ Change Target     │
│                  │ HostPolicy: default                  │ Validate Project  │
├──────────────────┴──────────────────────────────────────┴───────────────────┤
│ Validation / helper guidance                                                 │
└──────────────────────────────────────────────────────────────────────────────┘
```

---

## 22. Open Questions For The Next Design Pass

- Should the project explorer be tree-heavy or roster-heavy?
- Should command palette become primary over colon commands?
- Should tabs exist at all, or should buffer switching be list-driven?
- How much inline semantic UI is worth doing in a TUI versus inspector-driven detail?
- How aggressively should the shell re-layout itself when entering debug?
- Should there be a dedicated project-management layout or an inspector-first model?
- How strong should the visual style be by default versus conservative?
- What is the minimum acceptable 80x25 experience?

---

## 23. Closing Position

The right future for `OxIde` is not “terminal VS Code”.

It is:
- a terminal-native IDE
- visually modern but structurally honest
- deeply project-aware
- semantically rich
- editor-fast
- explicit about state

That is the design space worth exploring.
