# OxIde Product Direction

Primary current-direction planning document for `OxIde`.

Status:
- primary working UX/product-direction document
- opinionated
- intended to provoke decisions
- not yet a locked implementation spec
- supersedes the older `VISION.md` direction doc

This document uses Unicode box drawing, layout sketches, and palette tokens on purpose.
Yes, OxIde can use Unicode in docs and in the eventual product where terminal capabilities allow it.
Yes, OxIde can use richer modern color palettes, but the real product must still degrade cleanly to weaker terminals.

---

## 1. Current Product Direction

`OxIde` is being focused as a standalone terminal-native IDE for `OxVba`.

This document is now the main place where the current product direction is captured.
It starts from the original `VISION.md` intent, but updates it based on what we think now.

That means:
- the standalone IDE is the product focus
- the project/workspace authoring environment is the center of gravity
- debugging remains important, but does not define the product identity
- embedded-host scenarios are not part of the current OxIde plan

What belongs upstream in `OxVba` may still later support embedded hosting, but that is no longer a defining constraint for OxIde UX planning.

## 2. What Are We Building?

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

## 3. Scope Of This Planning Pass

This pass is about:
- UX usage model
- modality strategy
- default keybinding and compatibility strategy
- unified command model across palette, shortcuts, chords, and command aliases
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

## 4. Hard Constraints

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
- require at least `100x30`
- optimize for `120x40` and above
- become excellent around `160x45` and above
- should degrade gracefully without becoming ugly or unusable
- should work fully without mouse input
- should support mouse well when available
- should help users diagnose weak terminal/font/configuration setups

---

## 5. What Makes A TUI Different From A Web-Tech GUI?

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

## 6. Editor-Style vs IDE-Style

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

## 6.5 VBA IDE Compatibility As A UX Principle

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

Layer 3: command palette / command alias
  "Insert Module"
  "Run Project"
```

This is a strong TUI fit because it keeps:
- discoverability
- memorability
- power-user speed

without demanding a giant visible menu system at all times.

### Curation note

Not every idea about command entry should become a primary UX path.

In particular:
- a unified command model is a strong idea
- raw punctuation-triggered command entry from normal text editing is weaker

The system should be shaped around:
- actions
- bindings
- profiles
- discoverability

not around preserving every historical invocation detail literally.

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

## 7. Modality: Modal Or Non-Modal?

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

Command aliases belong here too:
- they are a transient invocation surface
- they should feed the same action namespace as shortcuts and palette commands

---

## 8. Recommended UX Usage Model

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
- many open buffers may exist even when not currently visible
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

## 8.4 Immediate Panel

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

## 8.5 Mouse Policy

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

## 9. Screen Space Strategy

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

## 10. Layout Presets

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

### 9.1b Split edit layout

```text
┌──────────────────────────────────────────────────────────────────────────────┐
│ Workspace: Payroll.basproj   Views: 2   Mode: Edit                         │
├──────────────┬──────────────────────────────┬───────────────────────────────┤
│ Explorer     │ Module1.bas                  │ Module2.bas                   │
│              │                              │                               │
│ > Module1    │ Public Sub Main()            │ Public Sub Helper()           │
│   Module2    │     answer = 40 + 2          │     Call Main                 │
│   Project    │ End Sub                      │ End Sub                       │
│   Refs       │                              │                               │
├──────────────┴──────────────────────────────┴───────────────────────────────┤
│ Problems / Immediate / Output                                               │
└──────────────────────────────────────────────────────────────────────────────┘
```

This is the TUI-friendly answer to “multiple files open at once”:
- compose multiple views
- do not rely on overlapping windows
- do not rely on a thick tab strip as the primary buffer model

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

## 11. What FrankenTui Affordances Should Be Used Best?

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

## 11.5 Hyprland / Helix Influence: What To Borrow, What Not To Borrow

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

## 12. Visual Language

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

## 13. What Makes A Modern TUI IDE Feel Modern?

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

Diagnostics deserve special emphasis because they will be the most frequent semantic feedback during normal editing.

The design should assume:
- problems list and diagnostic navigation are first-class
- inline diagnostics may be lighter than desktop-style red squiggles
- gutter markers, line markers, span highlights, or on-demand overlays may be a better TUI fit
- current-line diagnostic detail in status or inspector surfaces is likely valuable

### 12.5 Great empty and small-screen states

A modern TUI looks good when:
- no project is open
- the terminal is narrow
- diagnostics are empty
- the debug session is idle

This matters more than in a desktop GUI.

The empty state should be designed deliberately.

It should communicate:
- this is an IDE
- you can start productively from here
- the next actions are obvious

Possible welcome-state ingredients:
- recent projects
- create project
- open project
- open file
- terminal diagnostics / console setup
- shortcut/help entry point

---

## 14. Proposed Product Identity

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

## 15. Candidate Interaction Model

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

## 15.5 Unified Command Model

This should be a real architectural UX decision.

OxIde should have one command namespace.

Everything should resolve through named actions.

### Core idea

```text
Action
  -> can appear in command palette
  -> can have 0..N keyboard shortcuts
  -> can have 0..N keyboard chords
  -> can have 0..N mnemonic menu sequences
  -> can have 0..N command aliases
```

Examples:

```text
Action: project.insert_module
  Palette label: Insert Module
  Shortcut: none
  Chord: Alt+I, M
  Alias: module-new
  Alias: insert-module

Action: debug.run
  Palette label: Run
  Shortcut: F5
  Chord: Alt+R, R
  Alias: run
```

### Why this is the right model

It gives us:
- consistency
- configurability
- discoverability
- profile support
- the ability to map different user traditions onto the same command surface

It also prevents:
- keyboard handling from becoming an ad hoc pile
- palette-only commands drifting away from shortcut commands
- colon aliases from becoming a second-class command universe

### Command palette relationship

The command palette should be a view over the action registry.

That means:
- every user-visible command should have a named action
- palette filtering should search labels, aliases, and maybe bound keys
- command metadata should be centralized

### Keyboard shortcut relationship

Shortcuts and chords should also bind to the same action registry.

Important requirement:
- one action may have more than one binding
- one action may have both single-stroke and multi-stroke bindings

### Alias relationship

What some tools call “colon commands” should be treated here as command aliases over the same action namespace.

That is a better long-term model than treating them as a special scripting island.

### Recommended terminology

Prefer:
- `command aliases`
- `action aliases`

over making `colon-style commands` the primary conceptual layer.

Reason:
- some aliases may still be typed after a command-entry gesture
- but the underlying system should not depend on a literal colon forever

### Likely bad default to avoid

Do not make entering command mode from text editing as cheap as typing `:`.

Why:
- `:` is legitimate text in source editing
- stealing raw punctuation from the editor is friction in a non-modal IDE
- it overfits editor folklore more than OxIde’s actual product identity

Better options:
- explicit command-entry key
- palette key
- menu mnemonic entry

Current direction:
- do not support raw `:` as a default command-entry gesture

### Recommended invocation split

```text
Palette:
  primary discovery and broad access

Shortcuts / chords:
  primary fast-path for learned actions

Mnemonic menu sequences:
  compatibility path for VBA / VS / Windows IDE muscle memory

Command alias entry:
  expert textual invocation path
```

### Smart alias entry

If textual command entry exists, it should not be a raw dumb prompt.

It should support:
- completion
- fuzzy search
- alias suggestions
- recent commands
- action descriptions

So the user experience is closer to:
- command palette with textual bias

than:
- primitive shell prompt

---

## 15.6 Buffers, Views, And Non-Visible Open Files

OxIde should explicitly separate:
- buffers
- views
- layouts

### Working model

```text
Workspace
  contains buffers

Layout
  contains visible views

View
  mounts one buffer
```

This means:
- a file can be open without being visible
- a view can switch which buffer it displays
- a layout may show one, two, or more views
- the same buffer may appear in more than one view if that proves useful

### Recommendation

Yes, OxIde should allow open-but-not-visible buffers.

That is a better TUI model than:
- forcing every open file into a visible tab strip
- pretending non-visible state does not exist

### What should the user be able to do?

- keep multiple buffers open
- move through recent buffers quickly
- see that additional open buffers exist even if they are not visible
- mount a non-visible buffer into the current focused view
- split the layout and show another open buffer side by side

### Suggested user-facing behaviors

```text
Ctrl+Tab
  cycle recent buffers into the focused view

Ctrl+Shift+Tab
  cycle backward through recent buffers

Show Buffers
  open a roster / switcher of open buffers

Split View
  create another visible view and mount a selected buffer into it
```

### How should OxIde indicate open-but-not-visible buffers?

Not with a primary browser-style tab row.

Better TUI-friendly options:
- a `Buffers` roster panel
- a recent-buffers switcher overlay
- a compact top-bar count and current/recent indicators
- per-view title bars that show which buffer is mounted

### Suggested compact indication

```text
Top bar:
Workspace: Payroll.basproj   Views: 2   Buffers: 7 open   Focus: Module1.bas
```

### Buffer roster sketch

```text
┌──────────────────────────────────────────────┐
│ Buffers                                      │
├──────────────────────────────────────────────┤
│ > Module1.bas                      visible   │
│   Module2.bas                      visible   │
│   Payroll.basproj                  hidden    │
│   TaxHelpers.bas                   hidden    │
│   References.txt                   hidden    │
└──────────────────────────────────────────────┘
```

### Why this is better than tabs

- tabs consume scarce horizontal space
- tabs scale badly in text grids
- tabs are weaker than split composition for side-by-side work
- tabs are less important than fast switching and clear mounted-view titles

### Design consequence

The product should think in terms of:
- buffer roster
- recent buffer history
- mounted views
- split composition

not:
- tab strip first

---

## 15.7 Session Restore And Persistence

To feel like an IDE, OxIde should remember useful session state.

This should likely include:
- open buffers
- mounted views and split composition
- focused buffer/view
- cursor positions
- breakpoints
- recent buffer history

This should probably not attempt to restore:
- a live debug session
- transient completion popups
- transient command-entry state

Persistence should align with OxVba capabilities where semantic or debug state is involved, but the UX expectation should be clear now:
- reopening OxIde should feel like returning to a workspace
- not like starting from scratch every time

---

## 16. Modal Questions: Specific Recommendations

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

## 17. Specific UX Tensions To Decide

These are planning questions that need real decisions.

### Tension A: Command line vs command palette

Options:
- keep textual alias entry central
- add palette and demote textual alias entry
- support both

Working recommendation:
- keep both
- palette for discovery and broad command access
- textual alias entry for power-user exact commands and script-like flows

Important caution:
- textual alias entry should not be entered by typing raw `:` by default
- that punctuation-first pattern is a poor fit for a non-modal IDE

### Tension B: Single editor vs tabs vs buffers list

Options:
- one visible editor only
- split / multi-view composition
- hidden buffers with quick switch
- lightweight buffer roster

Working recommendation:
- do not center the UX on tabs
- use a buffer roster plus split/multi-view composition
- allow multiple visible files or multiple views of the same file through non-overlapping panel composition
- keep fast buffer switching available without requiring tabs

Why:
- tabs are a desktop/browser habit more than a TUI strength
- split composition fits terminal space and focus rules better
- multiple visible views are more useful than a long decorative tab row in a text grid

Working mental model:

```text
Buffers exist in a roster.
Views are composed into the layout.
One buffer may appear in one or more views.
```

This supports:
- two modules side by side
- code plus project file
- source plus alternate view of same source
- debug source plus another source/document
- open buffers that are currently not visible
- recent-buffer cycling into the focused view

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

## 17.5 Keybinding Compatibility Strategy

This needs its own design pass, but the default direction should be clear now.

### Initial profiles

- `VBA IDE Compatible`
- `Visual Studio Compatible`
- `VS Code Compatible`

These should be:
- real first-class profiles
- not tiny partial presets
- based on the same unified action registry

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
- profile selection should swap bindings, not fork the command universe

### Configurability requirement

Users should be able to:
- bind more than one shortcut to the same action
- bind more than one chord to the same action
- mix shortcuts and chords on the same action
- override profile defaults locally
- add or remove aliases

### Example shape

```text
Action: project.insert_module
  VBA profile:
    Alt+I, M
  Visual Studio profile:
    Alt+P, A?   (illustrative only; exact mapping still needs research)
  VS Code profile:
    no default chord
  User custom:
    Ctrl+Shift+N, M
    alias: insert-module
```

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

## 17.6 Advanced Boundary Notes

These are not first-read material, but they should be explicit somewhere in the planning document.

### Watches

Recommended split:
- OxIde owns watch-list presentation, pinning workflow, and layout placement
- OxVba owns evaluation semantics and any typed evaluation/debug contract

### Project authoring actions

Actions such as:
- Add Module
- Add Reference
- Change Target

should be understood as:
- OxIde commands and UX flows
- invoking typed OxVba helper APIs
- not OxIde inventing project semantics locally

### Target / profile / policy selection

These should appear as shell-presented state, but the underlying truth should remain in OxVba-side project/runtime contracts.

This is the pattern:
- OxIde presents and orchestrates
- OxVba defines meaning

---

## 18. Small, Medium, Large Terminal Strategies

### Small: supported minimum

Examples:
- `100x30`
- heavily zoomed desktop terminals that still meet product minimum
- constrained but still modern terminal windows

Goal:
- one dominant editor
- one compact explorer or none
- one compact status line
- one transient bottom panel only when needed

This should be considered:
- minimum supported
- intentionally reduced
- not the design center

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

### Medium: primary design center

Examples:
- `120x40`
- `132x40`
- similar visible cell counts after user zooming

Goal:
- full three-column editing shell
- bottom utility panel available

This should be the primary design target for normal desktop use.

### Large: premium experience

Examples:
- 160x45
- 160x50
- ultrawide / full-height terminal layouts

Goal:
- stable full IDE composition
- debug layouts become excellent
- project management surfaces become comfortable

### Zooming and reflow

Users will zoom terminal text.

OxIde should therefore think in terms of:
- visible cell geometry
- not pixels

That means when the user zooms:
- text gets bigger
- visible columns/rows shrink
- the shell should relayout from the new cell geometry

Recommendation:
- do not implement “zoom” inside OxIde as a separate scale system first
- respond to terminal resize/reflow correctly
- design breakpoints around cell counts

Working rule:

```text
Terminal zoom changes viewport geometry.
OxIde responds by relayout, not by trying to scale text itself.
```

---

## 18.5 Terminal Capability Testing And Setup

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
- does scrolling work correctly inside nested panes?
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

## 19. Anti-Goals

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

## 20. Suggested Near-Term Planning Tracks

This document should branch into follow-up design work:

### Track 1: UX principles

- define the shell principles formally
- define active/focused region language
- define visual hierarchy rules

### Track 2: layout system

- define layout presets
- define narrow/medium/wide breakpoints
- define inspector and bottom-surface roles
- define split-view composition rules
- define view creation, closing, and reassignment rules

### Track 3: command model

- command alias entry vs palette
- global commands vs focused commands
- unified action registry
- keybinding philosophy
- shortcut and chord schema
- VBA IDE shortcut research and mapping
- Visual Studio shortcut research and mapping
- VS Code shortcut research and mapping
- mnemonic Alt-sequence model
- alias/completion UX

### Track 4: editing intelligence UX

- hover UX
- completion UX
- diagnostics UX
- inline diagnostic treatment
- references and definitions UX
- symbol navigation UX
- immediate evaluation surface UX

### Track 5: project/workspace management UX

- project explorer model
- buffer roster vs view composition model
- module/reference management
- target/profile/policy surfaces
- workspace switching
- session restore policy

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

## 21. Provisional Recommendation

If a single direction must be chosen now:

```text
OxIde should become a project-first, non-modal, keyboard-native TUI IDE
with a stable left/center/right/bottom shell,
with transient overlays for commands and semantic interactions,
with explicit edit/run/debug workspace states,
with VBA-IDE-compatible default keybindings,
with full mouse support but zero mouse dependency,
with split-based multi-view composition instead of tab-centric UX,
and with a visually modern but terminal-honest design language plus an explicit console setup story.
```

That is the best fit for:
- the OxVba ownership split
- a modern terminal implementation
- FrankenTui’s likely strengths
- the difference between a true IDE and a text editor with plugins

---

## 22. Sketches For Discussion

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

## 23. Open Questions For The Next Design Pass

- Should the project explorer be tree-heavy or roster-heavy?
- Should command palette become primary over textual command-alias entry?
- How much inline semantic UI is worth doing in a TUI versus inspector-driven detail?
- How aggressively should the shell re-layout itself when entering debug?
- Should there be a dedicated project-management layout or an inspector-first model?
- How strong should the visual style be by default versus conservative?
- What exact inline diagnostic treatment best fits the TUI surface?
- How much session state should be restored by default versus opt-in?

---

## 24. Closing Position

The right future for `OxIde` is not “terminal VS Code”.

It is:
- a terminal-native IDE
- visually modern but structurally honest
- deeply project-aware
- semantically rich
- editor-fast
- explicit about state

That is the design space worth exploring.
