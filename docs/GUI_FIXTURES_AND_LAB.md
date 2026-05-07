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

## 7. W220 Handoff

W220 should start from the W210 view model and lab command, then add the
smallest editable module surface and diagnostics path.

Known prerequisites:

1. Keep `gui-thin-slice-loaded` stable as the read-only baseline.
2. Add a new scenario ID for editable/diagnostic state rather than mutating the W210 baseline beyond recognition.
3. Reuse `ProjectOpenSpineView` only where it remains true; introduce editor/document snapshot types deliberately.
4. Use OxVba document/session APIs for diagnostics; do not parse VBA in OxIde.
5. Keep browser-safe capability text visible while editing.

Known gaps:

1. no real DOM/Leptos mount yet,
2. no editor buffer in GUI crates yet,
3. no document snapshot update path to OxVba yet,
4. no diagnostic fixture beyond current thin-slice source yet.

## 8. Cross-Repo Fixture Policy

If a fixture belongs better in OxVba or DnaOneCalc, create a handoff and consume it from the authoritative repo after coordination. Do not duplicate project semantics locally just to make a short-term OxIde demo easier.
