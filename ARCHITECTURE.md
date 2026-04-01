# OxIde Architecture Note

## Decision

`OxIde` should be built on the Rust `FrankenTui` stack as its primary shell and runtime foundation, while treating modern `OxVba` as a first-class project/language-service/build/runtime substrate.

The working architecture choice is:

- use `FrankenTui` for the console shell, layout, runtime, widgets, input handling, and presentation primitives
- keep `OxIde` on one primary UI/runtime stack
- use `FrankenTui`'s editor path as the initial implementation behind an `OxIde`-owned seam
- treat `msedit` as a reference implementation and selective donor for editor behavior, algorithms, and tests
- treat `.basproj`, `ProjectManifest`, language services, and target-aware build/runtime flows as first-class OxIde scope

`OxIde` should not start by growing the `msedit` shell outward into the IDE, and it should not invent a parallel VBA parser/project system when `OxVba` already has one.

## Why

The reason is architectural fit.

The Rust `FrankenTui` tree already provides:

- a usable shell/runtime model
- pane and layout infrastructure
- focus and input handling
- multi-surface widget support
- a real editor path

That makes it a better base for a future VBA IDE than `msedit`'s surrounding shell.

At the same time, current `OxVba` now provides much more than a compiler/runtime core:

- a canonical `.basproj` project format
- a `ProjectManifest`-centered project/workspace model
- a language-service crate with document/workspace snapshots
- a lossless syntax tree and parser layer
- build/runtime target surfaces beyond single-file execution

That means OxIde should lean into `OxVba` as a semantic and tooling substrate rather than duplicating those concerns.

The decision should stand on console-first IDE needs alone. Any future non-console target is optional upside, not a phase-1 driver.

## Ownership Boundaries

The split should be defined by ownership of truth.

The good rule is:

- if it defines VBA meaning, project meaning, or canonical `.basproj` semantics, it belongs in `OxVba`
- if it defines IDE behavior, application flow, or presentation, it belongs in `OxIde`

The intended ownership split is:

- `FrankenTui` owns shell/runtime primitives and rendering/input infrastructure
- `OxVba` owns project model truth, canonical `.basproj` semantics, workspace loading/discovery policy, thin external-editor transport such as `oxvba-lsp`, and the public host/service boundary docs
- `OxIde` owns shell/UI/application flow, `ProjectSession` and `DocumentSession` orchestration, command routing, panels, status surfaces, keybindings, and the decision of when to call `OxVba` services and how to present results
- the editor subsystem owns text editing behavior

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

- `ProjectSession`
  - active `.basproj` identity
  - module roster and module-to-document mapping
  - project references and target configuration shown in the UI
  - runtime profile, host policy, and target selections surfaced to the user
  - coordination between editor state and `OxVba` project/language-service/build calls without redefining project semantics

- `DocumentSession`
  - current document identity
  - file path binding
  - dirty state
  - open/save/reload semantics
  - versioning and editor-facing source text
  - byte-offset mapping support needed for OxVba spans and language-service queries

- `OxVbaServices`
  - `.basproj` parsing/loading/generation and `ProjectManifest` workflows
  - canonical project model and workspace discovery/loading policy
  - syntax and semantic analysis
  - diagnostics, symbols, completion, hover, and related language services
  - build target selection and compilation workflows
  - runtime and execution workflows
  - typed host-facing service contracts for embedded consumers such as `OxIde`

## Recommended Technical Direction

Phase 1 should use `FrankenTui`'s editor path first. It should not attempt an immediate hard extraction of `msedit`.

That means:

- start with the existing `FrankenTui` editor implementation path
- adapt it behind an `OxIde`-owned `EditorSurface`
- keep project/session and document/session ownership outside the editor widget
- use `OxVba` language-service and project layers rather than building parallel OxIde-owned semantic infrastructure
- borrow ideas from `msedit` only where `FrankenTui` is weak or incomplete

Concrete `FrankenTui` types are an implementation detail, not the architecture. Concrete `OxVba` syntax/language-service types should also stay behind OxIde-owned adapter boundaries rather than leaking through all UI code.

That direction implies a specific next step in `OxVba`: define the first typed `OxIde`-facing session facade instead of keeping the seam shaped like CLI invocation.

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
- burying file/document ownership inside the editor widget
- inventing a duplicate OxIde-owned VBA parser or project system
- inventing OxIde-owned workspace discovery or `.basproj` interpretation rules
- assuming filesystem reloads are the authoritative path for IDE features when OxVba language services expect host-provided source text
- optimizing phase 1 for speculative non-console targets
- blocking progress on a perfect editor extraction before the first usable shell exists

## Proposed Seams

The editor should sit behind an `OxIde`-owned seam, and project/document semantics should be explicit rather than implicit.

Conceptually:

- `OxIdeShell`
  - owns panes, commands, status, focus routing, and workflow orchestration

- `ProjectSession`
  - owns active project/workspace state, `.basproj` identity, target selection, and the shell-facing project model

- `DocumentSession`
  - owns current document identity, path binding, dirty state, versioning, and open/save/reload semantics

- `EditorSurface`
  - owns the active text editing model, viewport, and editing behavior

- `OxVbaServices`
  - owns OxVba-side project, language-service, build, and execution contracts consumed by OxIde

The interaction model should be:

- `OxIdeShell` talks to `ProjectSession` for project/workspace semantics
- `ProjectSession` talks to `DocumentSession` for open module/document state
- `OxIdeShell` talks to `EditorSurface` for editing operations
- `ProjectSession` talks to `OxVbaServices` for project loading, language-service, build, and runtime operations
- `EditorSurface` should not know about `OxVba`
- `OxVbaServices` should not own UI/editor state

The service seam should move toward a direct embedded host API:

- `OxVba` should expose a typed session facade for host applications
- `OxVba` should return typed build/run results rather than CLI-shaped payloads
- `OxIde` should consume that direct API as the first-class host and showcase consumer
- external editors such as VS Code should use the same semantic/project substrate, with only transport and editor-integration glue living outside `OxVba`

That seam matters because it lets the project:

- start with the current `FrankenTui` editor path
- refine editor behavior over time
- absorb `.basproj` and project/workspace concerns honestly
- consume OxVba language services without tangling them into widget code
- harden or replace internals later without rewriting the whole shell

## Thin Slice And Near-Term Scope

The first thin slice should still prove the shell, document, editor, and service seams, not the whole IDE.

Recommended first slice:

- launch `OxIde` as a Rust console app
- show one editor surface
- open one file at startup or create one bound new file
- edit text in one buffer
- save the current file with visible dirty-state feedback
- expose a small command/status area
- perform one real minimal `OxVba` action through a narrow service seam
- report the result in a status or output region

That slice is enough to prove:

- `FrankenTui` can host the UI shape
- the editor is viable as the central surface
- document/file state is cleanly separated from editor behavior
- OxVba integration can sit beside the editor rather than being tangled into it

Immediately after that thin slice, the architecture should expand into explicit project/workspace scope:

- `.basproj` project open/create/save flows
- project/workspace session state in the shell
- target-aware build and run workflows
- runtime/profile/policy selections surfaced in the UI
- language-service wiring against host-provided document text

The next concrete split of work should be:

- in `OxVba`
  - define the first typed `OxIde`-facing session facade
  - expand direct project helper coverage where `OxIde` would otherwise invent logic
  - add typed embedded build/run results instead of CLI-shaped seams
  - keep improving docs around the public host interface

- in `OxIde`
  - replace the current CLI-oriented `OxVbaServices` seam with the direct API
  - wire diagnostics, symbols, completion, and hover into the UI
  - build project/module/reference management surfaces on top of `OxVba` helpers
  - use `OxIde` as the showcase consumer of the direct host API

## Conclusion

The current architecture choice is:

- `FrankenTui`-first shell and runtime
- `OxIde`-owned seams for shell, project/session, document/session, and editor concerns
- `OxVbaServices` as the semantic/project/build/runtime substrate consumed by OxIde
- `FrankenTui` editor path first for implementation
- `msedit` as a reference and selective donor

This gives the project the best balance of momentum, architectural fit, and future flexibility while bringing `.basproj`, project/workspace behavior, and OxVba target-aware workflows into explicit OxIde scope.

The intended ecosystem shape is:

- `OxVba` = engine, semantics, project truth, and transport
- `OxIde` = first-class direct host and showcase
- VS Code extension = alternate host over the same `OxVba` semantics
