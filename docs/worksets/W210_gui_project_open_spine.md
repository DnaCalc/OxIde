# Workset W210 — GUI Project-Open Spine

## Ambition

A browser-capable OxIde GUI can open the checked-in thin-slice OxVba fixture and present real project, module, source, and host-capability state.

This is the first executable GUI workset after W200. It should produce the smallest real GUI IDE spine, not a full editor.

## Dependencies

- W200 — GUI pivot runway complete.
- [`GUI_WORKSPACE_LAYOUT.md`](../GUI_WORKSPACE_LAYOUT.md) — crate/layout staging.
- [`GUI_FIXTURES_AND_LAB.md`](../GUI_FIXTURES_AND_LAB.md) — thin-slice fixture and `oxide-guilab` scenario ladder.
- OxVba project/session APIs for loading `examples/thin-slice/ThinSliceHello.basproj`.

## Design

First target scenario:

```text
Open examples/thin-slice/ThinSliceHello.basproj
  -> project spine shows ThinSliceHello
  -> module list shows Module1.bas
  -> source surface shows Module1.bas text read-only
  -> capability/status surface states current host profile
```

Likely implementation lanes:

1. create only the initial crates required by this vertical slice,
2. add minimal domain/view models for project/module/source/capability display,
3. add an OxVba adapter that loads the fixture through authoritative OxVba APIs,
4. add a browser GUI/lab surface that renders the scenario,
5. add contract tests for fixture load and view-model construction.

## Beads

This scaffold is not execution-ready. Before implementation starts, expand this section into concrete beads with Goal / Design / Tests / Evidence / Closure.

Expected bead lanes:

1. W210-B01 — workspace crate shell for the first GUI slice.
2. W210-B02 — thin-slice fixture load view model.
3. W210-B03 — `oxide-guilab` browser scenario shell.
4. W210-B04 — project/module/source read-only GUI rendering.
5. W210-B05 — W210 acceptance smoke and documentation close.

## Out-of-scope

- Editing source text.
- Diagnostics rendering beyond minimal capability/state smoke.
- Save/reload/session restore.
- Run/debug/immediate surfaces.
- DnaOneCalc embedding.
- Windows COM support.
