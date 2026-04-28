# OxIde Architecture

## Status

This document is subordinate to `PRODUCT_DIRECTION.md`.

Use it to capture:
- architectural seams
- ownership boundaries
- current integration shape
- implementation direction implied by the current product direction

Do not treat this file as a competing product-vision document.

## Architectural Position

`OxIde` is a standalone terminal-native IDE for `OxVba`.

The architecture should therefore optimize for:
- a FrankenTui-based shell and editing environment
- explicit project, document, and editor seams
- direct embedding of OxVba host/session semantics
- IDE behavior that is project-aware and stateful
- terminal-honest UX rather than LSP-shaped or CLI-shaped indirection

The core rule is:

- `PRODUCT_DIRECTION.md` defines what OxIde is trying to be
- `ARCHITECTURE.md` defines how the codebase should be divided to support that

## Current Stack Choice

The stack direction is:

- `FrankenTui` as the shell, layout, rendering, input, and editor foundation
- `OxVba` as the owner of project truth, workspace loading, document identity, and semantic services
- `OxIde` as the host shell that orchestrates editing, workspace interaction, and presentation

Important clarification:

- do not route editor semantics through LSP inside OxIde

## Ownership Of Truth

The ownership split should remain explicit.

If it defines VBA or project meaning, it belongs in `OxVba`.
If it defines IDE behavior or presentation, it belongs in `OxIde`.

### `OxVba` owns

- canonical `.basproj` semantics
- workspace loading and discovery policy
- project-backed document identity
- semantic queries and analysis
- diagnostics
- document and workspace symbols
- completions
- hover
- go-to-definition
- references
- semantic provenance
- typed host-facing session APIs
- typed build/run contracts when those are available

### `OxIde` owns

- shell/UI/application flow
- panel composition and layout behavior
- focus routing
- keybinding and command invocation policy
- keyboard shortcut profiles, chords, mnemonic sequences, and command aliases
- editor state and presentation
- buffer/view/layout orchestration
- dirty/save/reload/revert UX
- when OxVba services are invoked
- how OxVba results are surfaced in the UI
- project-management and debugging surfaces as UX

## Core Seams

The top-level seams should remain:

- `OxIdeShell`
- `ProjectSession`
- `DocumentSession`
- `EditorSurface`
- `OxVbaServices`

These seams are useful because they keep project semantics, document lifecycle, editing behavior, and OxVba integration from collapsing into one object.

### `OxIdeShell`

Owns:
- pane composition
- focus movement
- command routing
- status surfaces
- output surfaces
- inspector/tool surfaces
- stateful edit/run/debug presentation

### `ProjectSession`

Owns:
- the active OxVba target path or loaded project context
- project-backed document roster as presented to the shell
- mapping between active OxIde documents and OxVba document identity
- target/profile/policy state as presented to the user
- the loaded direct OxVba host session for the active workspace
- orchestration of semantic refreshes for project-backed documents

Important boundary:
- `ProjectSession` does not invent project semantics
- it hosts and coordinates OxVba project truth

### `DocumentSession`

Owns:
- open document path binding
- dirty state
- save/reload/revert lifecycle
- editor-facing source text
- current file identity as OxIde presents it

Important boundary:
- `DocumentSession` owns buffer lifecycle and file UX
- `OxVba` owns project-backed semantic identity

### `EditorSurface`

Owns:
- editing behavior
- cursor movement
- viewport behavior
- selection behavior
- scroll behavior
- text presentation
- editor-local interactions

Important boundary:
- `EditorSurface` should not own project semantics
- `EditorSurface` should not call OxVba directly

### `OxVbaServices`

Owns the OxIde-side integration seam to OxVba.

Today that means:
- direct `HostWorkspaceSession` integration for semantic/project-backed editor flows
- existing build/run path where OxVba still exposes CLI-shaped execution seams

Over time that seam should become more typed, not more shell-shaped.

## Current Direct Host Integration

The current architectural center of gravity is the direct host session from `oxvba_languageservice`.

The key API is:
- `HostWorkspaceSession`

Current intended flow:

1. `ProjectSession` loads one `HostWorkspaceSession` for the active OxVba workspace.
2. `ProjectSession` maps the active project-backed editor document to an OxVba `DocumentId`.
3. `DocumentSession` provides the current editor text.
4. `ProjectSession` pushes that text into the host session with `set_document_text(...)`.
5. `OxIde` queries diagnostics, symbols, hover, completions, and related semantic data from the host session.
6. `OxIdeShell` presents those results in editor-adjacent UX surfaces.

This is the right direction because:
- project truth stays in OxVba
- semantic behavior stays in OxVba
- OxIde stays responsible for host orchestration and UX
- no CLI parsing is needed for language intelligence
- no LSP detour is needed inside OxIde

## Current Build/Run Position

Build/run is still a mixed state.

Current rule:
- keep semantic/editor integration on the direct host session
- allow build/run to use a typed OxIde-side service seam even while OxVba execution contracts continue to evolve

That means the architecture should tolerate:
- direct host session for editing semantics
- an execution seam that can adapt as OxVba exposes richer direct build/run contracts

But it should not treat CLI-shaped integration as the architectural center.

## UX-Driven Architectural Constraints

Because `PRODUCT_DIRECTION.md` is authoritative, the architecture has to support:

- standalone IDE operation
- non-modal default editing
- stateful edit/run/debug workspace presentation
- split-based multi-view composition rather than tab-centric assumptions
- open buffers that may not currently be visible
- multiple visible views onto the same buffer
- unified action/command registry behind shortcuts, chords, mnemonics, palette entries, and command aliases
- full mouse support without mouse dependency
- console capability testing and setup guidance as a first-class product concern

Those are not just UX notes; they have architectural implications for:
- focus routing
- buffer/view modeling
- action dispatch
- layout persistence
- session restore

## Shell Surface Model

The shell should explicitly model a small number of persistent regions:

- explorer
- editor area
- inspector
- lower utility surface
- transient overlays

These are architectural regions, not just paint decisions.

That means the shell should own:

- which region is visible
- which region has focus
- which content mode a region is showing
- how width and height pressure collapse or remap regions

## Inspector Model

The inspector should be modeful rather than treated as a generic side bucket.

Expected inspector modes include:

- summary
- diagnostics
- symbols
- hover / documentation
- run status

This keeps semantic-rich editing and execution-rich states inside the same shell
grammar rather than spawning unrelated ad hoc panels.

## Lower Utility Surface Model

The lower utility surface should have explicit architectural standing.

It is the natural home for:

- problems
- output
- immediate
- references
- build log

This should be modeled as a region with its own content mode, focus behavior,
and persistence rather than a last-minute appendage.

## Overlay Model

Overlays should be few, explicit, and policy-driven.

The command palette is the primary overlay class.

Architecturally, overlays need:

- placement rules
- focus capture and release rules
- keyboard routing rules
- width-adaptation rules

This should not be left to individual widgets.

## Width Adaptation

The shell should adapt by region policy, not by local widget improvisation.

At narrower sizes, the system should prefer:

- collapsing inspector content into the lower utility surface
- reducing chrome before reducing structure
- keeping the project-aware shell legible

That implies a layout-policy owner in the shell/runtime layer.

## Buffer / View / Layout Model

The architecture should preserve a three-part model:

- buffers
- views
- layouts

Meaning:
- a buffer may be open without being visible
- a view presents one buffer
- multiple views may present the same buffer
- a layout composes the currently visible views and tool surfaces

This matters because OxIde is intentionally not adopting a tab-first shell model.

Undo/redo implications:
- undo history should attach to buffers, not views
- multiple views onto the same buffer must observe the same edit history
- layout operations should not be part of text undo/redo

## Action Model

The command system should be unified architecturally.

One action namespace should back:
- keybindings
- keyboard chords
- mnemonic menu-like sequences
- command palette entries
- command aliases

This should be reflected in the architecture rather than bolted on as parallel systems.

## What To Avoid

Avoid:
- inventing an OxIde-owned project model
- inventing an OxIde-owned semantic layer
- sending editor semantic behavior through LSP
- parsing CLI output for diagnostics or semantic queries
- burying document lifecycle inside the editor widget
- making the editor widget the owner of project/session state

## Immediate Architectural Priorities

The current priority areas are:

- keep tightening the direct `HostWorkspaceSession` integration
- expand project-management surfaces on top of OxVba-owned helpers
- improve semantic editing surfaces during active editing
- keep the buffer/view/layout model aligned with the product direction
- make persistent shell-region ownership and mode routing explicit
- make undo/redo ownership explicit in the editor and buffer architecture
- prepare for typed direct build/run contracts when OxVba exposes them
- support session restore and persistent shell state where OxIde should own it

## Verification

See [`docs/BEADS.md`](docs/BEADS.md) for the verification method.
This file is for architecture only; the when / why / what closes of
verification live in the bead definition.

## Relationship To Other Docs

- `PRODUCT_DIRECTION.md` — product direction, UX model, scope, design
  intent.
- `ARCHITECTURE.md` — seam boundaries and implementation direction
  (this file).
- `docs/BEADS.md` — working method (worksets, beads, closure).
- `docs/WORKSET_REGISTER.md` — ordered workset sequence.
- `docs/DESIGN_TUI.md` — current detailed TUI shell specification.
- `docs/uxpass/` — addendum UX authority produced by W035; to be
  reconciled back into `PRODUCT_DIRECTION.md` and
  `docs/DESIGN_TUI.md` via `docs/uxpass/60_reconciliation.md`.
- `docs/TESTING_WTD.md` — mechanical reference for the `wtd`
  harness.
- `README.md` — entry point and doc map.
