# Workset W220 — Editable Module And Diagnostics

## Ambition

The GUI can edit a module from the fixture project and surface OxVba diagnostics from the current document snapshot.

## Dependencies

- W210 — GUI project-open spine.
- OxVba language-service APIs for document text updates and diagnostics.
- `oxide-guilab` scenario support from W210.

## Design

The user should be able to type in the module surface and see document state and diagnostics update through OxVba-owned semantics.

Implementation lanes for the first pass:

1. minimal rendering-independent editor buffer and edit operation model,
2. deterministic lab edit scenario over the W210 thin-slice source,
3. document snapshot handoff to OxVba through `HostWorkspaceSession` inside `oxide-oxvba`,
4. diagnostic view-model projection without duplicating OxVba diagnostic enums,
5. lab renderer assertions for edited source and diagnostic state.

W220 does not need to introduce a full Leptos/DOM editor yet. A deterministic edit operation and lab render is the accepted first step; browser/DOM input can follow once the editor-core and diagnostic seams are stable.

## Beads

### W220-B01 — Editor-core minimal source snapshot and edit operation

**Infrastructure.**

- **Goal.** `oxide-editor-core` can load source text into a rendering-independent editor snapshot and apply one deterministic edit operation that produces updated source text without importing parked TUI editor state.
- **Design.** Add `oxide-editor-core` to the nested GUI workspace. Implement minimal types for source text, cursor/offset as needed, and a named edit operation suitable for the first diagnostics scenario. Keep this independent of Leptos and OxVba.
- **Tests.** Unit tests for source snapshot construction, edit application by named operation, and unchanged W210 source preservation when no edit is applied.
- **Evidence.** `cargo test --manifest-path crates/Cargo.toml --workspace` output.
- **Closure.** Editor-core exists; no parked TUI editor code imported; edit operation is deterministic and tested.

### W220-B02 — Edited thin-slice scenario source projection

**Infrastructure.**

- **Goal.** `oxide-guilab` can produce an edited-source scenario from the W210 thin-slice fixture while keeping `gui-thin-slice-loaded` read-only and unchanged.
- **Design.** Add a new stable scenario ID, likely `gui-thin-slice-edited-diagnostics`. Use `oxide-editor-core` to apply the deterministic edit to `Module1.bas`. The edited source should intentionally create or reveal a stable OxVba diagnostic.
- **Tests.** Scenario lookup by ID; rendered edited source contains the expected edit token; W210 read-only scenario output remains unchanged.
- **Evidence.** GUI workspace test output.
- **Closure.** New edited scenario exists by ID; W210 baseline remains stable.

### W220-B03 — OxVba diagnostics over edited document snapshot

**Infrastructure.**

- **Goal.** `oxide-oxvba` can ask OxVba for diagnostics over the edited active document snapshot without saving the edit to disk and without routing through LSP.
- **Design.** Use `HostWorkspaceSession` directly inside `oxide-oxvba` for this first pass. Load the fixture workspace, map the active module to OxVba document identity, apply the edited source as an in-memory document snapshot, and project diagnostics into OxIde-owned diagnostic rows that preserve OxVba severity/message/provenance enough for UI display.
- **Tests.** Contract tests for edited thin-slice diagnostics: session loads, active document is found by module name, edited source is submitted in memory, and at least one expected diagnostic row is produced. If the first chosen edit does not produce a stable diagnostic, adjust the edit or add a `diagnostics-demo` fixture in this bead.
- **Evidence.** GUI workspace test output and diagnostic row text from the fixture.
- **Closure.** Diagnostics come from OxVba direct session APIs; no LSP; no duplicate OxVba diagnostic enums.

### W220-B04 — Diagnostics lab rendering

**Feature.**

- **Goal.** Running the GUI lab render command for the edited diagnostics scenario shows the edited source and an OxVba-backed diagnostics surface.
- **Design.** Extend the deterministic text/HTML-like renderer with a diagnostics region. Include scenario ID, project name, module row, edited source, diagnostic rows, and browser-safe capability status.
- **Tests.** Render test asserts semantic tokens: scenario ID, edit token, diagnostic role/region, OxVba diagnostic message/severity, and capability text. Assertions must not depend on row position.
- **Evidence.** `cargo run --manifest-path crates/Cargo.toml -p oxide-guilab -- render <scenario-id>` output.
- **Closure.** User-visible lab command renders edited source and diagnostics; no save/reload/run behavior is claimed.

### W220-B05 — W220 acceptance and W230 handoff

**Doctrine.**

- **Goal.** W220 closes with evidence that editable-source plus diagnostics behavior is real and with a precise handoff to W230 for save/reload/session restore.
- **Design.** Update `GUI_FIXTURES_AND_LAB.md`, this workset, and a W230 handoff note with observed W220 behavior, known limitations, and persistence prerequisites.
- **Tests.** Rerun GUI workspace tests and the W220 lab render command.
- **Evidence.** Test output, scenario render output, and W230 handoff note.
- **Closure.** W220 acceptance target is satisfied; W230 has concrete prerequisites and no hidden fixture/lab gaps.

## Acceptance Evidence

W220 was accepted with:

```powershell
cargo test --manifest-path crates/Cargo.toml --workspace
cargo run --manifest-path crates/Cargo.toml -p oxide-guilab -- render gui-thin-slice-edited-diagnostics
```

The accepted lab output contains `gui-thin-slice-edited-diagnostics`,
`ThinSliceHello`, `Module1.bas`, edited source with `answer = 40 + 2`
and without `Dim answer`, a diagnostics region, `data-severity="error"`,
`use of undeclared variable: answer`, `OxVba language service`, and
browser-safe `COM unavailable` capability text.

## Out-of-scope

- Full editor feature parity.
- Real DOM/Leptos text input.
- Save/reload/session restore.
- Completion/hover/reference UX beyond what diagnostics require.
- Runtime execution.
