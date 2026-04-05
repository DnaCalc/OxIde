# TUI Shell Spec

This document is the current detailed TUI specification for OxIde.

It is the terminal-native realization of the current design work represented by:

- `docs/DESIGN_MOCKUP_WEB.md`
- `docs/DesignMockup/`

It is no longer just a note about how translation should happen. It is the
translation.

Use it to:

- build FrankenTui mockups
- compare terminal mockups against the web-presented design studies
- lock the first shell implementation slice

Use `PRODUCT_DIRECTION.md` for product and UX authority.
Use `ARCHITECTURE.md` for runtime seams and ownership boundaries.

## 1. Scope Of This Spec

This spec defines:

- the canonical shell frame
- region roles
- region modes
- state-specific compositions
- width-adaptation behavior
- focus and navigation expectations
- overlay behavior
- baseline visual tokens
- the first FrankenTui mockup target

This spec does not redefine:

- product identity
- OxVba ownership boundaries
- the buffer / view / layout doctrine

## 2. Canonical Shell Frame

The canonical OxIde shell is:

```text
┌──────────────────────────────────────────────────────────────────────────┐
│ Top Status / Identity Bar                                               │
├──────────────┬───────────────────────────────────────────┬───────────────┤
│ Explorer     │ Editor Area                               │ Inspector     │
├──────────────┴───────────────────────────────────────────┴───────────────┤
│ Lower Utility Surface                                                   │
└──────────────────────────────────────────────────────────────────────────┘
```

Region meaning:

- top = project identity, target, mode, state, and commandability hints
- left = project structure and navigation
- center = primary work
- right = contextual meaning
- bottom = eventful, list-shaped, or stream-shaped work

This frame should persist across the core states so the product keeps its
identity under editing, semantic inspection, and execution activity.

## 3. Region Specifications

### 3.1 Top Status / Identity Bar

The top bar is mandatory.

It should show:

- workspace or project name
- active target
- active shell mode
- dirty / running / debugging status
- terminal capability status if relevant

It may also show:

- active view count
- current inspector mode
- current lower-surface mode

It should not become a menu bar clone or a toolbar strip.

### 3.2 Explorer

The explorer is project-aware, not file-tree-only.

Primary content:

- modules
- forms
- references
- project-level nodes
- targets or build configurations when appropriate

The explorer should show:

- active item
- selected item
- dirty markers
- expandable project structure

The explorer should remain visible in normal-width layouts.

### 3.3 Editor Area

The editor area is the primary work surface.

It must support:

- one visible editor view
- split views across multiple buffers
- split views onto the same buffer

The editor area owns:

- cursor and selection visibility
- active-view emphasis
- line and gutter semantics
- execution-point and diagnostic marks

The shell should not depend on tab strips as the primary explanation of open
work.

### 3.4 Inspector

The inspector is compact, contextual, and modeful.

Canonical inspector modes:

- `Summary`
- `Diagnostics`
- `Symbols`
- `Hover`
- `RunStatus`

The inspector should not turn into a fourth workspace column of unrelated
widgets. It is the right-side semantic lens.

### 3.5 Lower Utility Surface

The lower utility surface is first-class.

Canonical lower-surface modes:

- `Problems`
- `Output`
- `Immediate`
- `References`
- `BuildLog`

This region hosts:

- long-running output
- streamed results
- dense secondary lists
- evaluation surfaces

It should support collapsed, compact, and expanded heights.

### 3.6 Overlays

The main overlay class is the command palette.

Allowed overlay uses:

- command palette
- quick open
- go to symbol
- completion menu
- focused picker
- confirmation surface

Overlay rules:

- one primary overlay at a time
- overlay focus is explicit
- overlays anchor to the shell grid
- overlays should not require backdrop theatrics to read clearly

## 4. State Compositions

### 4.1 Empty / First-Run

Goal:

- make OxIde feel capable and welcoming without feeling toy-like

Composition:

- top bar present
- explorer minimized or replaced by workspace launcher content
- editor area becomes welcome surface
- inspector may show environment or capability summary
- lower surface hidden by default unless capability checks need detail

Content:

- open project
- create project
- recent projects
- terminal capability status
- command palette hint

Render these as selectable command rows, not card UI.

### 4.2 Main Editing

Goal:

- default project-open editing shell

Composition:

- explorer visible
- editor area visible
- inspector visible in `Summary` or `Diagnostics`
- lower utility present in compact or collapsed state

Default lower-surface mode:

- `Problems` if diagnostics exist
- otherwise last active utility mode

### 4.3 Semantic-Rich Editing

Goal:

- show semantic depth without fragmenting the shell

Composition:

- explorer unchanged
- editor unchanged
- inspector switches to `Hover`, `Symbols`, or `Diagnostics`
- lower utility may switch to `Immediate` or `References`

Rules:

- prefer inspector reuse over floating cards
- prefer lower-surface references over ad hoc popouts
- keep the editor visually central

### 4.4 Build / Run

Goal:

- make execution state visible while keeping source context alive

Composition:

- explorer remains visible
- editor area stays central
- inspector switches to `RunStatus`
- lower utility switches to `Output` or `BuildLog`

The editor may show:

- execution point
- active module emphasis
- build-related marks

### 4.5 Command Palette

Goal:

- provide one strong command entry surface

Composition:

- centered or strongly anchored overlay
- shell remains visible behind it
- active focus entirely owned by the palette

Content structure:

- filter input
- grouped command list
- keybinding hints
- optional command aliases or mnemonics

## 5. Region Mode Matrix

```text
State           Explorer      Editor          Inspector       Lower Surface
Empty           launcher/min  welcome         summary/setup   hidden/optional
Editing         project       source          summary/diag    problems/last
Semantic        project       source          hover/symbols   immediate/refs
BuildRun        project       source          run-status      output/buildlog
Palette         frozen        dimmed/frozen   frozen          frozen
```

`Palette` does not mean visual dimming is mandatory. It means background regions
stop owning active focus.

## 6. Width Adaptation

### 6.1 Wide Layout: `160x45+`

Use full composition:

- explorer
- editor
- inspector
- lower utility

Suggested proportions:

- explorer: `18%` to `22%`
- editor: `56%` to `64%`
- inspector: `16%` to `22%`
- lower surface height: `20%` to `28%`

### 6.2 Standard Layout: `120x40`

Keep the same shell grammar.

Adjust by:

- tightening inspector density
- reducing decorative labels
- shortening secondary text
- shrinking lower-surface default height

Suggested proportions:

- explorer: `18%`
- editor: `62%`
- inspector: `20%`
- lower surface height: `18%` to `22%`

### 6.3 Narrow Layout: `100x30`

Preserve shell identity by collapsing regions, not by improvising a new shell.

Rules:

- explorer stays visible if possible
- inspector content collapses into the lower utility surface when necessary
- lower surface becomes the secondary-context owner
- overlays become shorter and more aggressively trimmed

Suggested composition:

- explorer: `20%`
- editor: `80%`
- inspector: collapsed
- lower surface: context-dependent secondary region

## 7. Focus And Navigation

The shell is keyboard-first.

Focus must always be obvious.

The shell should make it clear:

- which region is active
- which editor view is active
- when an overlay owns focus
- whether a list surface is selection-active or merely informative

Core navigation expectations:

- directional or explicit region focus movement
- quick jump into explorer, editor, inspector, and lower surface
- deterministic return from overlay to prior focus owner
- selection movement inside list-shaped regions without ambiguity

This spec does not lock exact keys yet, but it does lock the behavior model.

## 8. Visual Token Baseline

The current first-choice palette is:

```text
Background      █  #0A0E14
Panel           █  #0D1117
Panel Alt       █  #111827
Border          █  #1F2937
Text            █  #E6E6E8
Muted           █  #6C7680
Primary         █  #39BAE6
Warn            █  #FFB454
Error / Hot     █  #F97E72
Success         █  #50FA7B
Selection       █  #214D66
```

Color use:

- `Primary` for active focus and command emphasis
- `Warn` for cautionary state and non-fatal issues
- `Error / Hot` for errors, dirty urgency, and execution stop/error states
- `Success` for passing or active-ready states
- `Selection` for focused selection background or region-active grounding

The shell should use:

- strong border discipline
- restrained accents
- panel stratification
- clear active/inactive distinction

The shell should avoid:

- rainbow borders
- fake button styling
- ornamental glow logic

## 9. FrankenTui Mockup Target

The first FrankenTui mockup should render static but believable versions of:

1. empty / first-run
2. main editing
3. semantic-rich editing
4. build / run
5. command palette

The mockup should prove:

- the shell frame works in terminal space
- the palette and border logic read correctly
- inspector and lower-surface mode changes are understandable
- wide and narrow compositions both retain OxIde identity

The mockup should not yet require:

- real project loading
- real semantic services
- real build/run integration

It should be a shell-and-state mockup on FrankenTui, not a fake web screenshot.

## 10. Comparison Rule Against The Web Mockup

The FrankenTui mockup should be compared against the web-presented design using
these questions:

- does the TUI preserve the same shell identity?
- does it feel more honest, not less ambitious?
- is semantic/contextual information clearer in the terminal version?
- does the terminal version feel like a flagship IDE shell rather than a
  downgraded translation?
- do narrow layouts still feel intentional?

If the answer is no, the TUI spec should be tightened before implementation
continues.

## 11. Implementation Readiness

This spec is intended to be detailed enough to start the shell-foundation
implementation slice.

It is sufficient to begin:

- shell frame implementation
- region ownership
- inspector mode routing
- lower-surface mode routing
- overlay plumbing
- width-adaptation behavior
- theme/token implementation
- static FrankenTui mockup states

It is not yet the full implementation spec for all IDE behavior. That can
follow after the shell mockup proves the design in terminal form.
