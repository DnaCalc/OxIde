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

## 8. W230 Acceptance Target

W230 closes against an honest lifecycle/session lab scenario:

```text
Open examples/thin-slice/ThinSliceHello.basproj
  -> project spine still shows ThinSliceHello and Module1.bas
  -> working source shows the W220 deterministic edit
  -> document lifecycle state is dirty
  -> browser-limited save/reload commands explain direct filesystem persistence is unavailable
  -> local revert remains available where it is a pure state transition
  -> in-memory save evidence is explicitly labeled non-filesystem
  -> session restore reconstructs workspace/module/working-source dirty state
  -> capability/status surface still states browser-safe COM-unavailable profile
```

Current W230 evidence command:

```powershell
cargo run --manifest-path crates/Cargo.toml -p oxide-guilab -- render gui-thin-slice-lifecycle
```

Observed W230 output is deterministic text/HTML-like lab output. It contains:

- `data-scenario="gui-thin-slice-lifecycle"`,
- `ThinSliceHello`,
- `Module1.bas`,
- edited source with `answer = 40 + 2` and without `Dim answer`,
- `role="document-lifecycle"`,
- `data-provider="browser-limited"`,
- `data-dirty="true"`,
- `data-command="save" data-enabled="false"`,
- `data-command="reload" data-enabled="false"`,
- `data-command="revert" data-enabled="true"`,
- `browser-safe profile has no direct filesystem persistence`,
- `role="persistence-proof"`,
- `data-provider="in-memory"`,
- `data-filesystem="false"`,
- `no filesystem persistence claimed`,
- `role="session-restore"`,
- `data-profile="browser-limited"`,
- `role="restored-module">Module1.bas`,
- browser-safe host capability text including `COM unavailable`.

Implementation notes:

1. `oxide-core` owns pure lifecycle and session snapshot state.
2. `oxide-guilab` renders lifecycle/session evidence without mutating checked-in fixtures.
3. Browser-limited save/reload are disabled honestly; in-memory persistence is labeled as a proof seam only.
4. No parked TUI session store is imported.

Known W230 limitations:

1. no real DOM/Leptos save controls yet,
2. no real filesystem write/reload fixture yet,
3. no conflict resolution or multi-project restore,
4. no run/output surface yet; W240 owns that next step.

## 9. W240 Acceptance Target

W240 closes against capability-aware run/output lab scenarios:

```text
Browser-safe run/output
  -> project spine shows ThinSliceHello and Module1.bas
  -> run command is disabled
  -> output/activity region records the unsupported run request
  -> disabled reason states native execution provider is unavailable
  -> capability/status surface still states browser-safe COM-unavailable profile

Simulated supported run/output
  -> project spine shows ThinSliceHello and Module1.bas
  -> run command is enabled only by a simulated provider
  -> output/activity region shows structured lifecycle/activity/output events
  -> deterministic output says Main completed with answer 42
  -> scenario explicitly says native execution and COM runtime are false
```

Current W240 evidence commands:

```powershell
cargo run --manifest-path crates/Cargo.toml -p oxide-guilab -- render gui-run-output-browser-disabled
cargo run --manifest-path crates/Cargo.toml -p oxide-guilab -- render gui-run-output-simulated-supported
```

Observed browser-disabled output contains:

- `data-scenario="gui-run-output-browser-disabled"`,
- `ThinSliceHello`,
- `Module1.bas`,
- `role="run-output"`,
- `data-provider="browser-unsupported"`,
- `data-status="disabled"`,
- `ThinSliceHello::Module1.Main`,
- `role="run-command" data-enabled="false"`,
- `native execution provider unavailable`,
- `role="output-activity"`,
- `data-event-kind="lifecycle"`,
- `run requested`,
- `data-event-kind="diagnostic"`,
- `Run disabled`,
- browser-safe host capability text including `COM unavailable`.

Observed simulated-supported output contains:

- `data-scenario="gui-run-output-simulated-supported"`,
- `data-provider="simulated"`,
- `data-status="completed"`,
- `data-native-execution="false"`,
- `data-com-runtime="false"`,
- `role="run-command" data-enabled="true"`,
- `Run enabled by simulated provider`,
- `run started`,
- `simulated provider invoked ThinSliceHello::Module1.Main`,
- `simulated output: Main completed with answer 42`,
- `run completed`,
- browser-safe host capability text including `COM unavailable`.

Implementation notes:

1. `oxide-core` owns pure run capability, request, transcript, and output event state.
2. Browser-safe mode remains unsupported for execution.
3. The supported proof is explicitly simulated and does not claim native execution or COM.
4. No OxVba execution path or parked TUI run code is used.

Known W240 limitations:

1. no real native execution provider yet,
2. no real OxVba build/run wiring yet,
3. no dedicated `run-output-demo` fixture yet,
4. no debugger or Immediate Window surface yet.

## 10. W250 Acceptance Target

W250 closes against a deterministic DnaOneCalc embedding contract lab scenario:

```text
DnaOneCalc embedding contract
  -> project spine shows ThinSliceHello and Module1.bas
  -> DnaOneCalc is identified as the consuming host
  -> OxIde-owned embedded surface slots are listed
  -> ownership boundaries keep DnaOneCalc, OxIde, and OxVba distinct
  -> run capability remains browser-disabled with native execution unavailable
  -> native execution and COM runtime are not claimed
  -> the scenario states no DnaOneCalc repo files were modified
  -> capability/status surface still states browser-safe COM-unavailable profile
```

Current W250 evidence command:

```powershell
cargo run --manifest-path crates/Cargo.toml -p oxide-guilab -- render gui-dnaonecalc-embedding-contract
```

Observed W250 output contains:

- `data-scenario="gui-dnaonecalc-embedding-contract"`,
- `ThinSliceHello`,
- `Module1.bas`,
- `role="embedded-host-contract" data-host="DnaOneCalc"`,
- `data-sibling-repo-writes="false"`,
- `role="embedded-surface" data-slot="project-spine"`,
- `data-slot="source-editor"`,
- `data-slot="diagnostics"`,
- `data-slot="document-lifecycle"`,
- `data-slot="run-output"`,
- `data-slot="capability-footer"`,
- `role="ownership-boundary" data-owner="DnaOneCalc"`,
- `role="ownership-boundary" data-owner="OxIde"`,
- `role="ownership-boundary" data-owner="OxVba"`,
- `role="embedded-run-capability"`,
- `data-provider="browser-unsupported"`,
- `data-status="disabled"`,
- `data-native-execution="false"`,
- `data-com-runtime="false"`,
- `ThinSliceHello::Module1.Main`,
- `native execution provider unavailable`,
- `did not modify DnaOneCalc files`,
- browser-safe host capability text including `COM unavailable`.

Implementation notes:

1. `oxide-bridge` owns the serializable embedding packet boundary.
2. The packet consumes `oxide-core` session/run state rather than duplicating lifecycle/run/session models.
3. DnaOneCalc remains a read-only sibling repo for this OxIde-scoped run.
4. W250 proves a contract and lab scenario, not a real DnaOneCalc Leptos mount.

Known W250 limitations:

1. no DnaOneCalc repo changes were made,
2. no paired DnaOneCalc smoke exists yet,
3. no native execution provider yet,
4. no Windows COM capability yet,
5. no package/versioning decision for `oxide-bridge` yet.

## 11. W260 Acceptance Target

W260 closes against deterministic COM capability lab scenarios:

```text
Browser COM unavailable
  -> COM reference fact is visible
  -> browser-safe profile shows no native execution
  -> COM discovery and runtime invocation are unavailable
  -> Windows native host is required
  -> no COM runtime support is claimed

Non-Windows COM unavailable
  -> native execution capability is distinct from COM capability
  -> COM discovery and runtime invocation remain unavailable
  -> Windows native host is required

Windows native service missing
  -> Windows native host profile is admitted
  -> native COM service is not configured
  -> COM discovery and runtime invocation are blocked with service-specific reasons
  -> no COM invocation is claimed
```

Current W260 evidence commands:

```powershell
cargo run --manifest-path crates/Cargo.toml -p oxide-guilab -- render gui-com-reference-browser-unavailable
cargo run --manifest-path crates/Cargo.toml -p oxide-guilab -- render gui-com-reference-nonwindows-unavailable
cargo run --manifest-path crates/Cargo.toml -p oxide-guilab -- render gui-com-reference-native-service-missing
```

Observed browser-unavailable output contains:

- `data-scenario="gui-com-reference-browser-unavailable"`,
- `role="com-capability" data-profile="browser-safe"`,
- `data-native-execution="false"`,
- `data-com-service-configured="false"`,
- `data-windows-native-host-required="true"`,
- `COM reference present: Scripting.Dictionary`,
- `data-feature="reference-discovery" data-available="false"`,
- `COM discovery unavailable in browser-safe profile`,
- `data-feature="runtime-invocation" data-available="false"`,
- `pure browser/WASM cannot directly call Windows COM`,
- `Windows native host required`,
- `No COM runtime support is claimed`,
- browser-safe host capability text including `COM unavailable`.

Observed non-Windows output contains:

- `data-scenario="gui-com-reference-nonwindows-unavailable"`,
- `data-profile="non-windows-native"`,
- `data-native-execution="true"`,
- `data-com-service-configured="false"`,
- `COM discovery unavailable on non-Windows native host`,
- `COM runtime unavailable on non-Windows native host`,
- `Windows native host required`,
- `No COM runtime support is claimed`,
- `Non-Windows native profile`,
- `COM unavailable`.

Observed native-service-missing output contains:

- `data-scenario="gui-com-reference-native-service-missing"`,
- `data-profile="windows-native-service-missing"`,
- `data-native-execution="true"`,
- `data-com-service-configured="false"`,
- `data-windows-native-host-required="false"`,
- `COM reference present: Scripting.Dictionary`,
- `native COM service not configured`,
- `COM discovery blocked until service handoff is implemented`,
- `COM runtime invocation disabled`,
- `No COM runtime support is claimed`,
- `Windows native profile`,
- `COM runtime disabled`.

Implementation notes:

1. `oxide-core` owns pure COM capability projection state.
2. `oxide-guilab` renders capability evidence without calling COM.
3. Current COM reference fact is a demo projection, not an authoritative OxVba project parse.
4. `docs/HANDOFF_OXVBA_NATIVE_COM_CAPABILITY.md` captures required OxVba/native interfaces.

Known W260 limitations:

1. no real COM type-library discovery,
2. no real COM runtime invocation,
3. no native Windows service implementation,
4. no authoritative OxVba COM-reference packet consumed yet,
5. no run/debug/Immediate COM-capable session proof yet.

## 12. W270 Handoff

W270 should start from the nine current regression lab commands:

```powershell
cargo run --manifest-path crates/Cargo.toml -p oxide-guilab -- render gui-thin-slice-loaded
cargo run --manifest-path crates/Cargo.toml -p oxide-guilab -- render gui-thin-slice-edited-diagnostics
cargo run --manifest-path crates/Cargo.toml -p oxide-guilab -- render gui-thin-slice-lifecycle
cargo run --manifest-path crates/Cargo.toml -p oxide-guilab -- render gui-run-output-browser-disabled
cargo run --manifest-path crates/Cargo.toml -p oxide-guilab -- render gui-run-output-simulated-supported
cargo run --manifest-path crates/Cargo.toml -p oxide-guilab -- render gui-dnaonecalc-embedding-contract
cargo run --manifest-path crates/Cargo.toml -p oxide-guilab -- render gui-com-reference-browser-unavailable
cargo run --manifest-path crates/Cargo.toml -p oxide-guilab -- render gui-com-reference-nonwindows-unavailable
cargo run --manifest-path crates/Cargo.toml -p oxide-guilab -- render gui-com-reference-native-service-missing
```

W270 prerequisites:

1. keep run/debug/Immediate surfaces capability-gated,
2. do not claim COM-capable run/debug/Immediate until a tested native service exists,
3. preserve W240 simulated run as simulated-only evidence,
4. preserve W260 native-service-missing disabled reasons,
5. route OxVba runtime/debug/Immediate interface gaps through handoffs rather than local duplicates.

## 13. Cross-Repo Fixture Policy

If a fixture belongs better in OxVba or DnaOneCalc, create a handoff and consume it from the authoritative repo after coordination. Do not duplicate project semantics locally just to make a short-term OxIde demo easier.
