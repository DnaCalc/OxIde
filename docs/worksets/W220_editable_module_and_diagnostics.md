# Workset W220 — Editable Module And Diagnostics

## Ambition

The GUI can edit a module from the fixture project and surface OxVba diagnostics from the current document snapshot.

## Dependencies

- W210 — GUI project-open spine.
- OxVba language-service APIs for document text updates and diagnostics.
- `oxide-guilab` scenario support from W210.

## Design

The user should be able to type in the module surface and see document state and diagnostics update through OxVba-owned semantics.

Likely implementation lanes:

1. minimal rendering-independent editor buffer/caret/input model,
2. Leptos editor surface for source text input,
3. document snapshot handoff to OxVba,
4. diagnostics panel and minimal inline markers,
5. browser scenario assertions for edited source and diagnostic state.

## Beads

This scaffold is not execution-ready. Expand before implementation.

Expected bead lanes:

1. W220-B01 — editor-core minimal buffer/caret/input.
2. W220-B02 — GUI editor surface accepts text input.
3. W220-B03 — document snapshot updates OxVba session.
4. W220-B04 — diagnostics surface renders OxVba diagnostics.
5. W220-B05 — browser/lab acceptance for edit + diagnostics.

## Out-of-scope

- Full editor feature parity.
- Save/reload/session restore.
- Completion/hover/reference UX beyond what diagnostics require.
- Runtime execution.
