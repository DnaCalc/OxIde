# OxIde GUI Fixtures And Lab Seed

Status: `active_fixture_lab_plan`
Date: 2026-05-07

## 1. Purpose

This note seeds the deterministic fixture and GUI lab strategy for the first Rust/WASM-capable OxIde implementation worksets.

It does not implement the GUI lab. It defines the initial scenarios and fixture requirements so W210 can start with reviewable evidence rather than ad hoc demos.

## 2. Existing Fixture Inventory

Current checked-in OxIde fixture:

```text
examples/thin-slice/ThinSliceHello.basproj
examples/thin-slice/Module1.bas
examples/thin-slice/README.md
```

Use this as the first W210 project-open fixture because it is small, already tracked, and already used by the current TUI/WTD lane.

## 3. Required Fixture Ladder

The GUI line should grow a small fixture suite in this order:

1. `thin-slice`
   - opens a minimal project,
   - lists one module,
   - shows source,
   - supports basic diagnostics query.

2. `diagnostics-demo`
   - contains intentional, stable diagnostics,
   - proves diagnostic panel and inline marker behavior.

3. `references-demo`
   - contains at least two modules and a resolvable symbol reference,
   - proves hover, definition, references, and symbol navigation surfaces.

4. `run-output-demo`
   - has a deterministic entry point and output,
   - proves capability-aware run and output surfaces.

5. `com-reference-present-demo`
   - contains a COM reference as project truth,
   - proves browser/non-Windows unavailable states without needing COM execution.

6. `windows-com-demo`
   - Windows-native only,
   - proves COM discovery/invocation when the native capability service exists.

New fixtures should be added only when the workset that consumes them can also test them.

## 4. Oxide Guilab Scenario Ladder

`oxide-guilab` should become the fast browser review surface for active GUI work.

Initial scenario IDs should be stable and descriptive:

```text
gui-empty-welcome
gui-thin-slice-loaded
gui-module-readonly
gui-module-editing
gui-diagnostics-visible
gui-hover-visible
gui-run-output
gui-com-unavailable-browser
gui-com-available-windows-profile
gui-dnaonecalc-embedded-frame
```

The lab should support:

1. deterministic fixture-backed state,
2. viewport variants useful for browser and desktop review,
3. DOM text snapshots,
4. screenshot snapshots where stable,
5. accessibility checks where practical,
6. capability-profile switching.

## 5. First W210 Acceptance Target

W210 closes against a GUI/lab scenario equivalent to:

```text
Open examples/thin-slice/ThinSliceHello.basproj
  -> project spine shows ThinSliceHello
  -> module list shows Module1.bas
  -> editor surface shows Module1.bas source
  -> capability/status surface states current host profile
```

Current W210 evidence command:

```powershell
cargo run --manifest-path crates/Cargo.toml -p oxide-guilab -- render gui-thin-slice-loaded
```

Observed W210 output is deterministic text/HTML-like lab output rather
than a full browser mount. It contains:

- `data-scenario="gui-thin-slice-loaded"`,
- `ThinSliceHello`,
- `Module1.bas`,
- `Public Sub Main()`,
- browser-safe host capability text including `COM unavailable`.

This is the accepted W210 rendering substitute before full Leptos/browser
mounting. Editing belongs in W220.

## 6. Test Expectations

Before W210 implementation:

- this doc names the fixture and scenario ladder,
- `GUI_TEST_STRATEGY.md` points here,
- the existing thin-slice fixture files are present.

During W210 and later:

- fixture existence should be asserted by tests or lab boot checks,
- scenario IDs should be tested by name, not list position,
- snapshots should assert product contracts rather than fragile prose where possible.

## 7. W220 Acceptance Target

W220 closes against a deterministic editable/diagnostic lab scenario:

```text
Open examples/thin-slice/ThinSliceHello.basproj
  -> project spine still shows ThinSliceHello and Module1.bas
  -> editor/source region shows Module1.bas with a deterministic in-memory edit
  -> OxVba diagnostics are queried from the edited document snapshot
  -> diagnostics surface shows at least one OxVba-backed diagnostic row
  -> capability/status surface still states browser-safe COM-unavailable profile
```

Current W220 evidence command:

```powershell
cargo run --manifest-path crates/Cargo.toml -p oxide-guilab -- render gui-thin-slice-edited-diagnostics
```

Observed W220 output is deterministic text/HTML-like lab output. It contains:

- `data-scenario="gui-thin-slice-edited-diagnostics"`,
- `ThinSliceHello`,
- `Module1.bas`,
- edited source with `answer = 40 + 2` and without `Dim answer`,
- `role="diagnostics"`,
- `data-severity="error"`,
- `use of undeclared variable: answer`,
- `OxVba language service`,
- browser-safe host capability text including `COM unavailable`.

Implementation notes:

1. `gui-thin-slice-loaded` remains the W210 read-only baseline.
2. `oxide-editor-core` owns the rendering-independent deterministic edit operation.
3. `oxide-oxvba` uses `HostWorkspaceSession` directly; no LSP path or OxIde parser is introduced.
4. Diagnostic severity is projected as presentation text in `DiagnosticRow`, avoiding local duplication of OxVba diagnostic enums.

Known W220 limitations:

1. no real DOM/Leptos text input yet,
2. no save/reload/session restore yet,
3. no inline markers yet,
4. no standalone `diagnostics-demo` fixture yet because the thin-slice in-memory edit is stable enough for this acceptance step.

## 8. W230 Handoff

W230 should start from both lab commands:

```powershell
cargo run --manifest-path crates/Cargo.toml -p oxide-guilab -- render gui-thin-slice-loaded
cargo run --manifest-path crates/Cargo.toml -p oxide-guilab -- render gui-thin-slice-edited-diagnostics
```

W230 prerequisites:

1. preserve disk source until explicit save is requested,
2. introduce an OxIde-owned dirty/persisted document state model,
3. make save/reload/session restore visible in `oxide-guilab`,
4. define how `HostWorkspaceSession` overlays survive or reset after save/reload,
5. keep browser-safe capability text visible while persistence is unavailable or simulated.

## 9. Cross-Repo Fixture Policy

If a fixture belongs better in OxVba or DnaOneCalc, create a handoff and consume it from the authoritative repo after coordination. Do not duplicate project semantics locally just to make a short-term OxIde demo easier.
