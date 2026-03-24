# OxIde Architecture Note

## Decision

`OxIde` should be built on the Rust `FrankenTui` stack as its primary shell and runtime foundation.

The working architecture choice is:

- use `FrankenTui` for the console shell, layout, runtime, widgets, input handling, and presentation primitives
- keep `OxIde` on one primary UI/runtime stack
- use `FrankenTui`'s editor path as the initial implementation behind an `OxIde`-owned seam
- treat `msedit` as a reference implementation and selective donor for editor behavior, algorithms, and tests

`OxIde` should not start by growing the `msedit` shell outward into the IDE.

## Why

The reason is architectural fit.

The Rust `FrankenTui` tree already provides:

- a usable shell/runtime model
- pane and layout infrastructure
- focus and input handling
- multi-surface widget support
- a real editor path

That makes it a better base for a future VBA IDE than `msedit`'s surrounding shell.

`msedit` is strongest as an editor-centered system:

- strong text buffer design
- strong cursor, layout, and navigation behavior
- strong integrated textarea behavior

But its surrounding UI/runtime is less obviously the right foundation for:

- multi-pane IDE surfaces
- broader project-management UI
- embedded-host shells
- richer non-editor surfaces

The decision should stand on console-first IDE needs alone. Any future non-console target is optional upside, not a phase-1 driver.

## Ownership Boundaries

The intended ownership split is:

- `FrankenTui` owns shell/runtime primitives and rendering/input infrastructure
- `OxIde` owns IDE behavior, document/session orchestration, and `OxVba` workflow integration
- the editor subsystem owns text editing behavior
- `OxVba` integration owns compile/run/host/project operations

More concretely:

- shell/runtime primitives
  - app loop
  - screen composition primitives
  - pane and layout primitives
  - focus model
  - input dispatch
  - terminal presentation

- editor subsystem
  - text storage
  - cursor movement
  - selection
  - scrolling and viewport
  - undo/redo
  - search/replace behavior

- `OxIde` application layer
  - command surface
  - status and output areas
  - command routing
  - project/workflow concepts
  - coordination of editor, document, and host operations

- document/session model
  - current document identity
  - file path binding
  - dirty state
  - open/save/reload semantics
  - active buffer lifecycle

- `OxVba` seam
  - compile
  - run
  - host/project loading
  - diagnostics
  - later language-service and debugger integration

## Recommended Technical Direction

Phase 1 should use `FrankenTui`'s editor path first. It should not attempt an immediate hard extraction of `msedit`.

That means:

- start with the existing `FrankenTui` editor implementation path
- adapt it behind an `OxIde`-owned `EditorSurface`
- identify behavior gaps relative to `msedit`
- borrow ideas from `msedit` only where `FrankenTui` is weak or incomplete

Concrete `FrankenTui` types are an implementation detail, not the architecture. `OxIde` should avoid letting those concrete types leak across application boundaries.

This is a narrower and safer hybrid than trying to merge both UI stacks.

## What `msedit` Is For

`msedit` should be treated as:

- a correctness reference
- a behavior reference
- a source of useful editor design ideas
- a possible source of selective algorithm and test borrowing

Useful areas to compare against `msedit` include:

- Unicode/grapheme cursor rules
- visual vs logical position handling
- scrolling and cursor-visibility behavior
- selection semantics
- word movement semantics
- search/replace behavior
- editing performance expectations

The default assumption should be:

- keep `OxIde` on one primary UI/runtime stack
- import concepts or small components from `msedit` only when there is clear payoff

## What To Avoid

Avoid these early mistakes:

- trying to fuse two whole rendering/widget systems together
- treating `msedit` as if it were already the right full IDE shell
- letting concrete `FrankenTui` editor types leak into the `OxIde` application seam
- splitting file/document ownership ambiguously between the command surface and the editor widget
- optimizing phase 1 for speculative non-console targets
- blocking progress on a perfect editor extraction before the first usable shell exists

## Proposed Seam

The editor should sit behind an `OxIde`-owned seam, and file/document semantics should be explicit rather than implicit.

Conceptually:

- `OxIdeShell`
  - owns panes, commands, status, focus routing, and workflow orchestration

- `DocumentSession`
  - owns current document identity, path binding, dirty state, and open/save/reload semantics

- `EditorSurface`
  - owns the active text editing model, viewport, and editing behavior

- `OxVbaHost`
  - owns compile/run/host/project operations

The interaction model should be:

- `OxIdeShell` talks to `DocumentSession` for file semantics
- `OxIdeShell` talks to `EditorSurface` for editing operations
- `OxIdeShell` talks to `OxVbaHost` for compile/run/project operations
- `EditorSurface` should not know about `OxVba`
- `OxVbaHost` should not own editor state

That seam matters because it lets the project:

- start with the current `FrankenTui` editor path
- refine editor behavior over time
- compare against `msedit`
- harden or replace internals later without rewriting the whole shell

## First Thin Slice

The first thin slice should prove the shell, document, editor, and host seams, not the whole IDE.

Recommended first slice:

- launch `OxIde` as a Rust console app
- show one editor surface
- open one file at startup or create one bound new file
- edit text in one buffer
- save the current file with visible dirty-state feedback
- expose a small command/status area
- perform one real minimal `OxVba` action through a narrow seam
- report the result in a status or output region

That slice is enough to prove:

- `FrankenTui` can host the UI shape
- the editor is viable as the central surface
- document/file state is cleanly separated from editor behavior
- `OxVba` integration can sit beside the editor rather than being tangled into it

## Conclusion

The current architecture choice is:

- `FrankenTui`-first shell and runtime
- `OxIde`-owned seams for shell, document/session, editor, and `OxVba` hosting
- `FrankenTui` editor path first for implementation
- `msedit` as a reference and selective donor

This gives the project the best balance of momentum, architectural fit, and future flexibility without widening phase 1 beyond the console-first goal.
