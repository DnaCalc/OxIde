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

### W210-B01 — Workspace crate shell for the first GUI slice

**Infrastructure.**

- **Goal.** `cargo test` can still build the parked current `ox-ide` crate, and `cargo test --manifest-path crates/Cargo.toml --workspace` can build the first empty GUI foundation crates needed for W210, without changing TUI behavior.
- **Design.** Keep the root `Cargo.toml` as the current parked TUI package so its frozen OxVba path dependencies keep their existing workspace behavior. Add a nested GUI workspace at `crates/Cargo.toml` with only the first project-open spine crates: `oxide-domain`, `oxide-core`, `oxide-oxvba`, and `oxide-guilab`. Keep `oxide-ui-leptos` out until the first rendering bead if that gives a smaller review surface. No TUI code movement in this bead.
- **Tests.** `cargo test` for the root/current crate and `cargo test --manifest-path crates/Cargo.toml --workspace` for the GUI crate shell. Unit smoke in each new crate proving it builds and exposes its intended top-level role.
- **Evidence.** Build/test output and directory listing of new crates.
- **Closure.** Root TUI tests still pass; new GUI workspace crates build; no TUI files are moved or deleted; WTD remains opt-in.

### W210-B02 — Thin-slice fixture project view model

**Infrastructure.**

- **Goal.** A pure Rust test can load `examples/thin-slice/ThinSliceHello.basproj` through OxVba-owned project APIs and produce a GUI-neutral view model containing project name, module list, active module, source text, and baseline host capability status.
- **Design.** Add the smallest `oxide-domain` view-model types that are genuinely OxIde-owned. Add an `oxide-oxvba` fixture loader/adapter that consumes OxVba project APIs rather than duplicating project enums. Add orchestration in `oxide-core` only if needed to keep UI-neutral behavior out of the adapter.
- **Tests.** Contract tests by name: thin-slice project name is `ThinSliceHello`, module list contains `Module1.bas`, source contains `Public Sub Main()`, capability profile is present and browser-safe by default.
- **Evidence.** `cargo test --workspace` output and test names.
- **Closure.** View model is produced from real fixture/OxVba APIs; no copied OxVba project enums; no GUI rendering dependency required.

### W210-B03 — `oxide-guilab` scenario shell

**Infrastructure.**

- **Goal.** A deterministic `oxide-guilab` entrypoint can enumerate and render the `gui-thin-slice-loaded` scenario as text/HTML-like output suitable for early browser/lab review, even before full visual polish.
- **Design.** Add a minimal scenario registry with stable scenario IDs from `GUI_FIXTURES_AND_LAB.md`. The first scenario consumes the W210-B02 view model. If Leptos is introduced here, follow DnaOneCalc's dependency pattern (`leptos` 0.8.x with SSR/CSR split) unless a better current decision is recorded. Otherwise start with a pure renderer and defer browser mount to W210-B04.
- **Tests.** Scenario lookup by ID, duplicate scenario rejection, and rendered output containing `ThinSliceHello`, `Module1.bas`, and host capability text.
- **Evidence.** Test output and, if a binary exists, command output for listing/rendering the scenario.
- **Closure.** Scenario is deterministic, named by ID rather than list position, and consumes the real thin-slice view model.

### W210-B04 — Read-only GUI project/module/source rendering

**Feature.**

- **Goal.** Running the W210 GUI/lab surface shows a read-only project-open spine: project name `ThinSliceHello`, module `Module1.bas`, source text from `Module1.bas`, and current host capability/status text.
- **Design.** Add the first GUI surface needed to render the W210 view model. Keep it read-only. Use browser/WASM-capable architecture and keep product behavior in core/view models rather than widget code. Preserve the planned future path toward Leptos/browser rendering even if the first lab render is still a deterministic text/SSR artifact.
- **Tests.** Browser/lab scenario or renderer test asserting visible contract tokens by semantic role/label rather than fragile row position. If browser automation is not available yet, use deterministic HTML/text snapshot as the W210 acceptance substitute and record the limitation.
- **Evidence.** Render output or browser/lab capture showing the required tokens.
- **Closure.** The user-visible project-open spine exists in the GUI/lab surface; no editing, diagnostics, or run behavior is claimed.

### W210-B05 — W210 acceptance and next-workset handoff

**Doctrine.**

- **Goal.** W210 closes with evidence that the GUI project-open spine is real and with a precise handoff to W220 for editable module and diagnostics work.
- **Design.** Update `GUI_FIXTURES_AND_LAB.md`, this workset, and any relevant handoff notes with observed W210 behavior and known gaps. Keep progress state in beads, not in the workset body.
- **Tests.** Read-through checklist plus rerun W210 acceptance tests.
- **Evidence.** Test output, scenario capture/output, and W220 handoff notes.
- **Closure.** W210 acceptance target is satisfied; W220 has concrete prerequisites and no hidden fixture/lab gaps.

## Out-of-scope

- Editing source text.
- Diagnostics rendering beyond minimal capability/state smoke.
- Save/reload/session restore.
- Run/debug/immediate surfaces.
- DnaOneCalc embedding.
- Windows COM support.
