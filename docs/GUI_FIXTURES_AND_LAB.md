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

## 12. W270 Accepted Runtime Surfaces

W270 adds three runtime-surface lab scenarios:

```powershell
cargo run --manifest-path crates/Cargo.toml -p oxide-guilab -- render gui-run-timeline-simulated
cargo run --manifest-path crates/Cargo.toml -p oxide-guilab -- render gui-immediate-browser-disabled
cargo run --manifest-path crates/Cargo.toml -p oxide-guilab -- render gui-debug-browser-disabled
```

Observed run-timeline output contains:

- `data-scenario="gui-run-timeline-simulated"`,
- `role="run-timeline"`,
- `data-provider="simulated"`,
- `data-status="completed"`,
- `data-native-execution="false"`,
- `data-com-runtime="false"`,
- `data-index="1"`,
- `data-index="2"`,
- `data-index="3"`,
- `simulated output: Main completed with answer 42`.

Observed Immediate browser-disabled output contains:

- `data-scenario="gui-immediate-browser-disabled"`,
- `role="immediate-panel"`,
- `data-profile="browser-disabled"`,
- `data-enabled="false"`,
- `data-native-runtime-required="true"`,
- `data-com-runtime-required="false"`,
- `data-fake-responses="false"`,
- `Immediate disabled: browser-safe profile has no native OxVba runtime session`,
- `No Immediate responses rendered without runtime session`.

Observed debug browser-disabled output contains:

- `data-scenario="gui-debug-browser-disabled"`,
- `role="debug-panel"`,
- `data-profile="browser-disabled"`,
- `data-enabled="false"`,
- `data-native-runtime-required="true"`,
- `data-com-runtime-required="false"`,
- `data-fake-debug-data="false"`,
- `Debug disabled: browser-safe profile has no OxVba debug session`,
- `unavailable; no fake debug data`.

Implementation notes:

1. `oxide-core` owns the pure timeline/Immediate/debug capability projections.
2. `oxide-guilab` renders runtime surfaces without invoking a real runtime.
3. `docs/HANDOFF_OXVBA_RUNTIME_DEBUG_IMMEDIATE_INTERFACES.md` captures the required OxVba/shared runtime interfaces.
4. W270 acceptance keeps simulated output distinct from native execution and COM-capable execution.

Known W270 limitations:

1. no real OxVba runtime session,
2. no real Immediate request/response path,
3. no real debug adapter/session,
4. no callstack/locals/watch/breakpoint binding,
5. no COM-capable run/debug/Immediate proof,
6. no DnaOneCalc-hosted runtime integration yet.

## 13. W280 Accepted Command, Keyboard, Focus, And Accessibility Polish

W280 adds four GUI polish lab scenarios:

```powershell
cargo run --manifest-path crates/Cargo.toml -p oxide-guilab -- render gui-command-palette-baseline
cargo run --manifest-path crates/Cargo.toml -p oxide-guilab -- render gui-keyboard-contexts-baseline
cargo run --manifest-path crates/Cargo.toml -p oxide-guilab -- render gui-focus-graph-no-mouse
cargo run --manifest-path crates/Cargo.toml -p oxide-guilab -- render gui-accessibility-disabled-reasons
```

Observed command-palette output contains:

- `data-scenario="gui-command-palette-baseline"`,
- `role="command-palette"`,
- `data-source="gui-core command registry"`,
- `data-parked-tui-imported="false"`,
- `data-command-count="10"`,
- `data-command-id="project.open"`,
- `data-command-id="document.save"`,
- `data-command-id="runtime.run"`,
- `data-capability="browser-unsupported"`,
- `native execution provider unavailable`,
- `data-command-id="runtime.immediate"`,
- `no native OxVba runtime session`,
- `data-command-id="runtime.debug"`,
- `no OxVba debug session`,
- `GUI-native command registry; parked TUI command model not imported`.

Observed keyboard-context output contains:

- `data-scenario="gui-keyboard-contexts-baseline"`,
- `role="keyboard-contexts"`,
- `data-source="gui-core keyboard map"`,
- `data-host-specific-overrides-required="false"`,
- `data-context-collisions="0"`,
- `data-cross-context-collisions="0"`,
- `data-context="global-shell"`,
- `data-context="editor"`,
- `data-context="immediate"`,
- `data-context="debug"`,
- `data-command-id="shell.command_palette" data-gesture="Ctrl+Shift+P"`,
- `data-command-id="document.save" data-gesture="Ctrl+S"`,
- `data-command-id="runtime.run" data-gesture="F5"`,
- `data-command-id="runtime.immediate" data-gesture="Enter"`,
- `data-allow-cross-context="true"`,
- `no browser-specific key trap is product truth`.

Observed focus-graph output contains:

- `data-scenario="gui-focus-graph-no-mouse"`,
- `role="focus-graph"`,
- `data-source="gui-core focus graph"`,
- `data-node-count="9"`,
- `data-route-length="10"`,
- `data-node-id="project-tree" data-kind="project-tree"`,
- `data-node-id="source-editor" data-kind="editor"`,
- `data-node-id="run-output" data-kind="run-output" data-focusable="true" data-disabled-reason-visible="true"`,
- `data-node-id="immediate-panel" data-kind="immediate"`,
- `data-node-id="debug-panel" data-kind="debug"`,
- `data-node-id="command-palette" data-kind="command-palette"`,
- `role="focus-restore-target">source-editor`,
- `data-index="1" data-node-id="project-tree"`,
- `data-index="9" data-node-id="command-palette"`,
- `returns to source-editor`,
- `Disabled reason panels remain reachable`.

Observed accessibility output contains:

- `data-scenario="gui-accessibility-disabled-reasons"`,
- `role="accessibility-projection"`,
- `data-source="gui-core accessibility projection"`,
- `data-web-framework-bound="false"`,
- `data-surface-count="10"`,
- `data-surface-id="source-editor" data-role="editor"`,
- `role="accessible-label">Source editor`,
- `data-surface-id="diagnostics-panel" data-role="diagnostics"`,
- `OxVba language-service diagnostics`,
- `data-surface-id="run-output" data-role="run-output" data-has-disabled-reason="true"`,
- `native execution provider unavailable`,
- `data-surface-id="immediate-panel" data-role="immediate" data-has-disabled-reason="true"`,
- `no native OxVba runtime session`,
- `data-surface-id="debug-panel" data-role="debug" data-has-disabled-reason="true"`,
- `no OxVba debug session`,
- `data-surface-id="com-capability" data-role="com-capability" data-has-disabled-reason="true"`,
- `COM discovery unavailable in browser-safe profile`,
- `no web framework accessibility API is chosen in core`.

Implementation notes:

1. `oxide-core` owns pure command, keyboard, focus, and accessibility projections.
2. `oxide-guilab` renders deterministic evidence without a concrete web framework.
3. No parked TUI command/key/focus state is imported.
4. Command availability reuses lifecycle/run/Immediate/debug/COM capability state.
5. W280 does not claim real runtime, debug, Immediate, COM, or filesystem persistence.

Known W280 limitations:

1. no mounted browser/desktop GUI shell yet,
2. no real DOM accessibility audit yet,
3. no host-specific keybinding override layer yet,
4. no visual theme/high-contrast implementation yet,
5. no real runtime/debug/Immediate or COM support beyond capability/unavailable projections.

## 14. W290 Accepted Host-Mounted GUI Shell

W290 accepted against the twenty current regression lab commands:

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
cargo run --manifest-path crates/Cargo.toml -p oxide-guilab -- render gui-run-timeline-simulated
cargo run --manifest-path crates/Cargo.toml -p oxide-guilab -- render gui-immediate-browser-disabled
cargo run --manifest-path crates/Cargo.toml -p oxide-guilab -- render gui-debug-browser-disabled
cargo run --manifest-path crates/Cargo.toml -p oxide-guilab -- render gui-command-palette-baseline
cargo run --manifest-path crates/Cargo.toml -p oxide-guilab -- render gui-keyboard-contexts-baseline
cargo run --manifest-path crates/Cargo.toml -p oxide-guilab -- render gui-focus-graph-no-mouse
cargo run --manifest-path crates/Cargo.toml -p oxide-guilab -- render gui-accessibility-disabled-reasons
cargo run --manifest-path crates/Cargo.toml -p oxide-guilab -- render gui-shell-packet-baseline
cargo run --manifest-path crates/Cargo.toml -p oxide-guilab -- render gui-mounted-shell-static
cargo run --manifest-path crates/Cargo.toml -p oxide-guilab -- render gui-mounted-command-palette
cargo run --manifest-path crates/Cargo.toml -p oxide-guilab -- render gui-mounted-no-mouse-accessibility
```

Observed shell-packet output contains:

- `data-scenario="gui-shell-packet-baseline"`,
- `role="shell-packet"`,
- `data-source="oxide-core GuiShellPacket"`,
- `data-project="ThinSliceHello"`,
- `data-active-module="Module1.bas"`,
- `data-native-execution-claimed="false"`,
- `data-com-runtime-claimed="false"`,
- `data-web-framework-bound="false"`,
- `data-parked-tui-imported="false"`,
- `role="shell-packet-command-count">10`,
- `role="shell-packet-keybinding-count">11`,
- `role="shell-packet-focus-node-count">9`,
- `role="shell-packet-accessibility-count">10`.

Observed mounted static output contains:

- `data-scenario="gui-mounted-shell-static"`,
- `role="mounted-shell-static"`,
- `data-source="GuiShellPacket"`,
- `data-static-render="true"`,
- `data-dom-audited="false"`,
- `data-filesystem-persistence="false"`,
- `data-native-runtime="false"`,
- `data-com-runtime="false"`,
- `role="mounted-project-tree"`,
- `role="mounted-editor"`,
- `role="mounted-diagnostics"`,
- `role="mounted-run-output"`,
- `role="mounted-command-palette"`,
- `Static shell render consumes GuiShellPacket`.

Observed mounted command-palette output contains:

- `data-scenario="gui-mounted-command-palette"`,
- `role="mounted-command-palette-detail"`,
- `data-source="GuiShellPacket.command_palette"`,
- `data-parked-tui-imported="false"`,
- `data-command-count="10"`,
- `data-command-id="document.save" data-category="document" data-gesture="Ctrl+S"`,
- `data-command-id="runtime.run" data-category="runtime" data-gesture="F5"`,
- `native execution provider unavailable`,
- `data-command-id="runtime.immediate" data-category="runtime" data-gesture="Enter"`,
- `no native OxVba runtime session`,
- `parked TUI command model not imported`.

Observed mounted no-mouse/accessibility output contains:

- `data-scenario="gui-mounted-no-mouse-accessibility"`,
- `role="mounted-no-mouse-accessibility"`,
- `data-source="GuiShellPacket.focus_graph+accessibility"`,
- `data-web-framework-bound="false"`,
- `data-dom-audited="false"`,
- `data-route-length="10"`,
- `data-accessibility-surface-count="10"`,
- `data-index="1" data-node-id="project-tree"`,
- `data-index="9" data-node-id="command-palette"`,
- `returns to source-editor`,
- `role="mounted-accessible-label">Source editor`,
- `COM discovery unavailable in browser-safe profile`,
- `DOM accessibility audit is not claimed`.

Implementation notes:

1. `oxide-core` owns `GuiShellPacket`; it combines existing pure projections rather than forking them.
2. `oxide-guilab` renders the mounted/static proof from packet state.
3. Command, keyboard, focus, and accessibility mounted slices consume packet fields.
4. No parked TUI command/key/focus/widget/shell state is imported.
5. W290 keeps DnaOneCalc as a consumer/host boundary and does not modify sibling repos.

Known W290 limitations:

1. no real web framework or browser DOM mount yet,
2. no DOM accessibility audit or accessibility compliance claim,
3. no real filesystem persistence,
4. no native runtime/debug/Immediate execution,
5. no native COM discovery or invocation,
6. no DnaOneCalc host integration changes.

## 15. W300 Accepted Mounted Web Shell Adapter

W300 accepted against the twenty-four current regression lab commands. W300 added:

```powershell
cargo run --manifest-path crates/Cargo.toml -p oxide-guilab -- render gui-web-shell-adapter-boundary
cargo run --manifest-path crates/Cargo.toml -p oxide-guilab -- render gui-web-shell-dom-smoke
cargo run --manifest-path crates/Cargo.toml -p oxide-guilab -- render gui-web-command-palette-dom-smoke
cargo run --manifest-path crates/Cargo.toml -p oxide-guilab -- render gui-web-no-mouse-accessibility-dom-smoke
```

Observed web-shell adapter boundary output contains:

- `data-scenario="gui-web-shell-adapter-boundary"`,
- `role="web-shell-boundary-snapshot"`,
- `role="web-shell-adapter" data-source="GuiShellPacket"`,
- `data-web-framework="unselected"`,
- `data-dom-smoke-tested="false"`,
- `data-dom-audited="false"`,
- `data-filesystem-persistence="false"`,
- `data-native-runtime="false"`,
- `data-com-runtime="false"`,
- `data-parked-tui-imported="false"`,
- `role="web-project-tree"`,
- `role="web-source-editor"`,
- `role="web-run-output"`,
- `role="web-com-capability"`,
- `role="web-command-summary"`,
- `role="web-focus-accessibility-summary"`,
- `Web shell adapter consumes GuiShellPacket`.

Observed web-shell DOM smoke output contains:

- `data-scenario="gui-web-shell-dom-smoke"`,
- `role="web-shell-dom-smoke"`,
- `data-source="GuiShellPacket"`,
- `data-smoke-kind="parsed-html-tree"`,
- `data-dom-smoke-tested="true"`,
- `data-browser-runtime="false"`,
- `data-dom-audited="false"`,
- `data-all-passed="true"`,
- `data-check="root consumes GuiShellPacket" data-passed="true"`,
- `data-check="project tree carries project name" data-passed="true"`,
- `ThinSliceHello`,
- `Module1.bas`,
- `data-check="source editor shows module source" data-passed="true"`,
- `Public Sub Main()`,
- `Parsed HTML DOM smoke only; no browser runtime or DOM accessibility audit is claimed`.

Observed web command-palette DOM smoke output contains:

- `data-scenario="gui-web-command-palette-dom-smoke"`,
- `role="web-command-palette-dom-smoke"`,
- `data-smoke-kind="parsed-html-command-palette"`,
- `data-dom-smoke-tested="true"`,
- `data-browser-runtime="false"`,
- `data-dom-audited="false"`,
- `data-all-passed="true"`,
- `project.open gesture survives DOM mounting`,
- `document.save gesture survives DOM mounting`,
- `data-gesture=Ctrl+S`,
- `runtime.run gesture survives DOM mounting`,
- `data-gesture=F5`,
- `runtime.run disabled reason remains visible`,
- `native execution provider unavailable`,
- `runtime.immediate gesture survives DOM mounting`,
- `data-gesture=Enter`,
- `runtime.debug gesture survives DOM mounting`,
- `data-gesture=F10`,
- `shell.command_palette gesture survives DOM mounting`,
- `data-gesture=Ctrl+Shift+P`,
- `parked TUI command model remains isolated`.

Observed web no-mouse/accessibility DOM smoke output contains:

- `data-scenario="gui-web-no-mouse-accessibility-dom-smoke"`,
- `role="web-no-mouse-accessibility-dom-smoke"`,
- `data-smoke-kind="parsed-html-no-mouse-accessibility"`,
- `data-dom-smoke-tested="true"`,
- `data-browser-runtime="false"`,
- `data-dom-audited="false"`,
- `data-all-passed="true"`,
- `focus route starts at project tree`,
- `focus route reaches source editor`,
- `focus route reaches diagnostics`,
- `focus route reaches run output`,
- `focus route reaches Immediate`,
- `focus route reaches debug`,
- `focus route reaches COM capability`,
- `focus route reaches command palette`,
- `command palette restores editor focus`,
- `returns to source-editor`,
- `source editor accessible description survives DOM mounting`,
- `native execution provider unavailable`,
- `no native OxVba runtime session`,
- `no OxVba debug session`,
- `COM discovery unavailable in browser-safe profile`,
- `not a full accessibility audit`.

Implementation notes:

1. `oxide-webshell` is a thin adapter over `oxide-core::GuiShellPacket`.
2. DOM smoke uses parsed HTML via the adapter snapshot; it does not claim a browser runtime.
3. W300 keeps GUI-lab deterministic and keeps W210-W290 scenarios intact.
4. No real filesystem persistence, native runtime/debug/Immediate, or COM runtime is claimed.
5. No DnaOneCalc or OxVba sibling repo files were modified.

Known W300 limitations:

1. no real DnaOneCalc host mount yet,
2. no browser runtime smoke beyond parsed HTML DOM checks,
3. no full accessibility audit/compliance claim,
4. no real filesystem persistence,
5. no native runtime/debug/Immediate execution,
6. no native COM discovery or invocation.

## 16. W310 Accepted DnaOneCalc Web Shell Hosting

W310 accepted against the twenty-six current regression lab commands. W310 added:

```powershell
cargo run --manifest-path crates/Cargo.toml -p oxide-guilab -- render gui-dnaonecalc-web-shell-host-contract
cargo run --manifest-path crates/Cargo.toml -p oxide-guilab -- render gui-dnaonecalc-web-shell-dom-readiness
```

Observed DnaOneCalc web-shell host contract output contains:

- `data-scenario="gui-dnaonecalc-web-shell-host-contract"`,
- `role="dnaonecalc-web-shell-host-contract"`,
- `data-host="DnaOneCalc"`,
- `data-state-contract="GuiShellPacket"`,
- `data-embedding-contract="EmbeddedIdePacket"`,
- `data-web-adapter="oxide-webshell"`,
- `data-sibling-repo-writes="false"`,
- `data-host-mount-claimed="false"`,
- `role="host-ownership-boundary" data-owner="DnaOneCalc"`,
- `role="host-ownership-boundary" data-owner="OxIde"`,
- `role="host-ownership-boundary" data-owner="OxVba"`,
- `role="host-web-shell-summary" data-project="ThinSliceHello" data-active-module="Module1.bas"`,
- `role="host-dom-readiness" data-smoke-kind="parsed-html" data-all-passed="true"`,
- `DnaOneCalc browser host smoke is not claimed`,
- `OxIde-side W310 contract did not modify DnaOneCalc files`.

Observed DnaOneCalc web-shell DOM readiness output contains:

- `data-scenario="gui-dnaonecalc-web-shell-dom-readiness"`,
- `role="dnaonecalc-web-shell-dom-readiness"`,
- `data-source="W300 DOM smoke reports"`,
- `data-static-shell="true"`,
- `data-command-palette="true"`,
- `data-no-mouse-accessibility="true"`,
- `data-browser-runtime="false"`,
- `data-dnaonecalc-host-smoke="false"`,
- `data-dom-audited="false"`,
- `data-filesystem-persistence="false"`,
- `data-native-runtime="false"`,
- `data-com-runtime="false"`,
- `OxIde parsed HTML DOM readiness only`,
- `full accessibility audit are not claimed`.

Implementation notes:

1. `oxide-bridge` owns `DnaOneCalcWebShellHostPacket`, composing `EmbeddedIdePacket`, `GuiShellPacket`, and parsed DOM readiness/no-claim facts.
2. `oxide-guilab` renders DnaOneCalc host contract/readiness evidence without modifying sibling repos.
3. W310 reuses W300 parsed HTML smoke reports; it does not claim a browser runtime or full accessibility audit.
4. DnaOneCalc remains a consumer/host boundary, not the owner of OxIde IDE state.
5. `docs/HANDOFF_DNAONECALC_WEB_SHELL_HOST_API.md` documents the paired DnaOneCalc-side host API expectations.

Known W310 limitations:

1. no real DnaOneCalc browser host mount yet,
2. no sibling DnaOneCalc repository writes,
3. no full DOM accessibility audit/compliance claim,
4. no real filesystem persistence,
5. no native runtime/debug/Immediate execution,
6. no native COM discovery or invocation.

## 17. W320 Accepted Native Filesystem And Session Persistence

W320 accepted against the twenty-nine current regression lab commands. W320 added:

```powershell
cargo run --manifest-path crates/Cargo.toml -p oxide-guilab -- render gui-native-save-reload-disk
cargo run --manifest-path crates/Cargo.toml -p oxide-guilab -- render gui-native-session-restore-disk
cargo run --manifest-path crates/Cargo.toml -p oxide-guilab -- render gui-browser-filesystem-still-disabled
```

Observed native save/reload disk output contains:

- `data-scenario="gui-native-save-reload-disk"`,
- `role="native-save-reload-disk"`,
- `data-provider="native-filesystem"`,
- `data-filesystem-persistence="true"`,
- `data-test-owned-temp-project="true"`,
- `data-checked-in-fixture-mutated="false"`,
- `data-dirty-before-save="true"`,
- `data-dirty-after-save="false"`,
- `data-save-acknowledged="true"`,
- `data-reload-source-matches-disk="true"`,
- `data-native-runtime="false"`,
- `data-com-runtime="false"`,
- `answer = 21 * 2`,
- `Disk-backed save/reload is proven only against a GUI-lab test-owned temp project copy`.

Observed native session restore disk output contains:

- `data-scenario="gui-native-session-restore-disk"`,
- `role="native-session-restore-disk"`,
- `data-provider="native-filesystem"`,
- `data-session-provider="native-filesystem-session"`,
- `data-filesystem-persistence="true"`,
- `data-test-owned-temp-project="true"`,
- `data-session-file-written="true"`,
- `data-checked-in-fixture-mutated="false"`,
- `data-restored-dirty="false"`,
- `data-native-runtime="false"`,
- `data-com-runtime="false"`,
- `role="native-session-module">Module1.bas`,
- `answer = 84 / 2`,
- `OxIde-owned session JSON`,
- `.basproj semantics remain OxVba-owned`.

Observed browser filesystem disabled output contains:

- `data-scenario="gui-browser-filesystem-still-disabled"`,
- `role="browser-filesystem-still-disabled"`,
- `data-provider="browser-limited"`,
- `data-filesystem-persistence="false"`,
- `data-save-enabled="false"`,
- `data-reload-enabled="false"`,
- `data-native-runtime="false"`,
- `data-com-runtime="false"`,
- `browser-safe profile has no direct filesystem persistence`,
- `Browser/WASM direct filesystem persistence remains disabled`.

Implementation notes:

1. `oxide-core` owns `NativeFilesystemDocumentPersistence`, `NativeFilesystemSessionPersistence`, and the native/browser persistence projections.
2. Disk-write evidence uses test-owned temporary project copies and verifies checked-in thin-slice fixture content remains unchanged.
3. `oxide-guilab` renders native save/reload and session persistence evidence by creating temporary project copies; it does not mutate checked-in fixtures.
4. Browser/WASM direct filesystem persistence remains disabled and visible.
5. W320 does not claim native OxVba runtime/debug/Immediate execution or COM runtime.

Known W320 limitations:

1. no DnaOneCalc host implementation,
2. no native OxVba runtime/debug/Immediate execution,
3. no native COM discovery or invocation,
4. no full browser runtime or DOM accessibility audit,
5. no conflict resolution or external-file-change handling.

## 18. W330 Accepted OxVba Native Runtime Service Contract

W330 accepted against the thirty-three current regression lab commands. W330 added:

```powershell
cargo run --manifest-path crates/Cargo.toml -p oxide-guilab -- render gui-runtime-service-contract-browser-disabled
cargo run --manifest-path crates/Cargo.toml -p oxide-guilab -- render gui-runtime-service-contract-native-missing
cargo run --manifest-path crates/Cargo.toml -p oxide-guilab -- render gui-immediate-service-contract-native-missing
cargo run --manifest-path crates/Cargo.toml -p oxide-guilab -- render gui-debug-service-contract-native-missing
```

Observed runtime service browser-disabled output contains:

- `data-scenario="gui-runtime-service-contract-browser-disabled"`,
- `role="runtime-service-contract"`,
- `data-provider="browser-unsupported"`,
- `data-command-enabled="false"`,
- `data-real-execution="false"`,
- `data-native-runtime="false"`,
- `data-com-runtime="false"`,
- `ThinSliceHello::Module1.Main`,
- `native execution provider unavailable`,
- `real OxVba execution, native runtime, and COM runtime are not claimed`.

Observed runtime service native-missing output contains:

- `data-scenario="gui-runtime-service-contract-native-missing"`,
- `role="runtime-service-contract"`,
- `data-provider="native-service-missing"`,
- `data-command-enabled="false"`,
- `data-real-execution="false"`,
- `data-native-runtime="false"`,
- `data-com-runtime="false"`,
- `native OxVba runtime service not configured`,
- `real execution unavailable`.

Observed Immediate service native-missing output contains:

- `data-scenario="gui-immediate-service-contract-native-missing"`,
- `role="immediate-service-contract"`,
- `data-provider="native-service-missing"`,
- `data-command-enabled="false"`,
- `data-response-count="0"`,
- `data-fake-responses="false"`,
- `data-native-runtime="false"`,
- `data-com-runtime="false"`,
- `role="immediate-service-request">?answer`,
- `native OxVba runtime service not configured`,
- `fake responses are not allowed`.

Observed debug service native-missing output contains:

- `data-scenario="gui-debug-service-contract-native-missing"`,
- `role="debug-service-contract"`,
- `data-provider="native-service-missing"`,
- `data-state="unavailable"`,
- `data-command-enabled="false"`,
- `data-command-count="6"`,
- `data-callstack-count="0"`,
- `data-locals-count="0"`,
- `data-watches-count="0"`,
- `data-breakpoints-count="0"`,
- `data-fake-debug-data="false"`,
- `data-native-runtime="false"`,
- `data-com-runtime="false"`,
- `native OxVba runtime/debug service not configured`,
- `fake debug data is not allowed`.

Implementation notes:

1. `oxide-core` owns `RuntimeServicePacket`, `ImmediateServicePacket`, and `DebugServicePacket` as OxIde-side contract packets for future OxVba native service data.
2. Browser-unsupported and native-service-missing states are distinct.
3. Immediate/debug unavailable states render empty response/callstack/locals/watch/breakpoint rows rather than fake data.
4. W330 preserves W270 simulated and disabled scenarios while adding contract-ready service packets.
5. W330 does not write to OxVba or DnaOneCalc and does not claim real runtime/debug/Immediate/COM execution.

Known W330 limitations:

1. no real OxVba native runtime service implementation,
2. no real Immediate execution,
3. no real debug session, callstack, locals, watches, or breakpoints,
4. no native COM discovery or invocation,
5. no DnaOneCalc host implementation.

## 19. W341 DnaOxIde Scaffold Acceptance Target

W341 closes against a deterministic app scaffold verification rather than a GUI-lab scenario:

```powershell
node apps/dna-oxide/scripts/verify-scaffold.mjs
```

Observed W341 scaffold verification covers:

- required app files under `apps/dna-oxide/`,
- `DNA OxIde` product branding,
- `DnaOxIde` internal app naming,
- Tauri-ready `src-tauri/tauri.conf.json` metadata with `productName = DNA OxIde`,
- frontend entry files with no direct Tauri invoke or OxVba coupling,
- dependency-free native scaffold tests under `apps/dna-oxide/src-tauri`,
- command-name placeholders only,
- explicit false/no-claim runtime, native runtime, COM runtime, fake Immediate response, and fake debug data states.

Implementation notes:

1. W341 does not add shared UI components; W342 owns reusable UI.
2. W341 does not implement host bridge traits; W343 owns the shared facade.
3. W341 does not implement Tauri command adapters; W344 owns command wiring.
4. W341 does not claim a live Tauri/WebView runtime; W345/W346 own host UI and interaction proof.
5. W341 does not claim real OxVba runtime/debug/Immediate/COM capability.

Known W341 limitations:

1. no shared UI crate yet,
2. no host bridge crate yet,
3. no real Tauri runtime dependency or WebView smoke yet,
4. no real project open/save through Tauri commands yet,
5. no real OxVba compile/runtime/debug/Immediate/COM adapter evidence yet.

## 20. W342 Shared UI Component Acceptance Target

W342 closes against the shared component crate and a deterministic GUI-lab route:

```powershell
cargo test --manifest-path crates/Cargo.toml -p oxide-ui-leptos
cargo run --manifest-path crates/Cargo.toml -p oxide-guilab -- render gui-shared-ui-shell-component
```

Observed W342 shared UI output contains:

- `data-scenario="gui-shared-ui-shell-component"`,
- `role="shared-ui-component-route"`,
- `data-source="oxide-ui-leptos"`,
- `role="shared-ide-surface"`,
- `data-component-crate="oxide-ui-leptos"`,
- `data-provenance="pending-oxvba-hardening"`,
- `role="shared-project-spine"`,
- `role="shared-editor-boundary"`,
- `role="shared-diagnostics-summary"`,
- `role="shared-lifecycle-summary"`,
- `role="shared-run-output"`,
- `role="shared-command-palette"`,
- `role="shared-focus-accessibility"`,
- `role="shared-runtime-service"`,
- `role="shared-immediate-service"`,
- `role="shared-debug-service"`,
- `role="shared-com-capability"`,
- `ThinSliceHello`,
- `Module1.bas`,
- `data-native-runtime="false"`,
- `data-com-runtime="false"`,
- `data-fake-responses="false"`,
- `data-fake-debug-data="false"`.

Implementation notes:

1. `oxide-ui-leptos` is the shared UI component boundary for DnaOxIde, DnaOneCalc, and GUI-lab review.
2. W342 starts with deterministic HTML-string component output over OxIde packets; real Leptos hydration is not yet claimed.
3. Shared components consume `GuiShellPacket`, `RuntimeServicePacket`, `ImmediateServicePacket`, and `DebugServicePacket` inputs.
4. Shared components carry provenance labels such as `proven-oxide-state`, `oxvba-available-subset`, `oxvba-fixture-evidenced`, `pending-oxvba-hardening`, and `unavailable-no-claim`.
5. `oxide-guilab` preserves existing scenario IDs and adds `gui-shared-ui-shell-component` as the component-backed route.
6. W342 keeps Tauri/app-folder dependencies out of the shared UI crate.
7. W343 owns the next host bridge facade; W344 owns DnaOxIde Tauri command adapters.

Known W342 limitations:

1. no live Tauri/WebView runtime,
2. no real browser hydration,
3. no full DOM accessibility audit,
4. no host bridge crate yet,
5. no real OxVba runtime/debug/Immediate/COM adapter evidence,
6. no DnaOneCalc repo mount.

## 21. W343 Host Bridge Command Dispatch Acceptance Target

W343-B03 closes against host bridge command availability projected into the shared UI and a deterministic GUI-lab route:

```powershell
cargo test --manifest-path crates/Cargo.toml -p oxide-host-bridge
cargo test --manifest-path crates/Cargo.toml -p oxide-ui-leptos
cargo run --manifest-path crates/Cargo.toml -p oxide-guilab -- render gui-host-bridge-command-dispatch
```

Observed W343 command dispatch output contains:

- `data-scenario="gui-host-bridge-command-dispatch"`,
- `role="host-bridge-command-dispatch-route"`,
- `data-source="oxide-host-bridge+oxide-ui-leptos"`,
- `role="shared-command-dispatch"`,
- `data-source="HostBridgeCommandAvailability"`,
- `data-command-id="project.open"` with `data-enabled="true"`,
- `data-command-id="runtime.run"` with `data-state="oxvba-fixture-evidenced"`,
- `data-command-id="runtime.immediate"`,
- `data-command-id="watch.upsert"`,
- `data-command-id="references.com.search"`,
- `ThinSliceHello fixture covers EmbeddedBuildRunHost::run_project`,
- `ComSelectionService reference state and capability_profile`,
- `data-native-runtime="false"`,
- `data-com-runtime="false"`,
- `data-fake-responses="false"`,
- `data-fake-debug-data="false"`.

Implementation notes:

1. `oxide-host-bridge` owns the command catalog and availability projection over `HostBridgeServiceStatus`.
2. `oxide-ui-leptos` renders `HostBridgeCommandAvailability` without depending on Tauri or app-specific host code.
3. `oxide-guilab` provides the deterministic review route.
4. OxVba ThinSliceHello fixture-evidenced commands remain adapter-target labels until OxIde adds direct adapter tests.
5. Real DnaOxIde runtime/debug/Immediate/COM claims remain false.

## 22. W344 DnaOxIde Command Boundary Acceptance Target

W344 closes against Rust-callable DnaOxIde command stubs plus a frontend command client shim:

```powershell
npm --prefix apps/dna-oxide run command-client:check
npm --prefix apps/dna-oxide run scaffold:check
cargo test --manifest-path apps/dna-oxide/src-tauri/Cargo.toml
cargo test --manifest-path crates/Cargo.toml --workspace
```

Observed W344 acceptance evidence is captured in `target/w344-acceptance.txt` and contains:

- `dna_oxide_open_project_path`, `dna_oxide_load_active_module`, `dna_oxide_save_active_module`, and session snapshot commands;
- `dna_oxide_get_host_capabilities` over W343 host bridge availability;
- compile/reference/COM/runtime/Immediate/debug/watch/breakpoint command stubs;
- `proven-oxide-only`, `oxvba-available-subset`, `oxvba-fixture-evidenced`, and `pending-oxvba-hardening` labels;
- `createDnaOxIdeCommandClient` and `createBrowserFixtureCommandClient`;
- `native-service-missing` runtime/Immediate/debug packets;
- checked-in `examples/thin-slice/Module1.bas` mutation guard;
- no direct Tauri import/global in frontend/shared UI/host bridge paths;
- no true runtime/COM/fake-data claim tokens.

Implementation notes:

1. Lifecycle commands operate on explicit/test-owned project paths and temp copies.
2. Runtime, Immediate, and debug commands return empty native-service-missing packets; they do not synthesize responses, callstacks, locals, watches, or breakpoints.
3. COM discovery/capability commands are disabled/subset/fixture labels; COM runtime invocation remains unclaimed.
4. The frontend command client accepts an injected invoke implementation for the future Tauri path and provides a browser fixture client for review.
5. W345 owns live host UI proof and must state whether it drives Tauri/WebView IPC, static frontend output, Rust render, or another bounded proof mode.

## 23. W345 DnaOxIde Host UI Proof Acceptance Target

W345 closes against the selected static frontend host fixture proof mode:

```powershell
npm --prefix apps/dna-oxide run host-ui:check
npm --prefix apps/dna-oxide run host-lifecycle:check
npm --prefix apps/dna-oxide run host-services:check
npm --prefix apps/dna-oxide run command-client:check
npm --prefix apps/dna-oxide run scaffold:check
cargo test --manifest-path crates/Cargo.toml -p oxide-ui-leptos
cargo test --manifest-path crates/Cargo.toml -p oxide-host-bridge
```

Observed W345 acceptance evidence is captured in `target/w345-acceptance.txt` and contains:

- `role="dnaoxide-host-ui-proof"`,
- `data-proof-mode="static-frontend-host-fixture"`,
- `data-shared-ui-crate="oxide-ui-leptos"`,
- `data-host-bridge-crate="oxide-host-bridge"`,
- `DNA OxIde`, `ThinSliceHello`, and `Module1.bas`,
- project, editor, diagnostics, lifecycle, command-palette, runtime, Immediate, debug, and COM panes,
- `proven-oxide-only`, `oxvba-available-subset`, `oxvba-fixture-evidenced`, `pending-oxvba-hardening`, and `native-service-missing` labels,
- runtime/Immediate/debug/COM empty-state markers such as output events `0`, Immediate responses `0`, callstack/locals/watches/breakpoints `0`, and COM runtime invocation `false`,
- checked-in fixture mutation guard,
- no direct Tauri import/global in frontend/shared UI/host bridge paths,
- no true runtime/COM/fake-data/live-proof claim tokens.

Implementation notes:

1. W345 is reviewable static frontend host proof, not live Tauri/WebView IPC.
2. `apps/dna-oxide/src/main.js` mounts the host shell into `#dna-oxide-root`.
3. `apps/dna-oxide/src/host-shell.js` is thin DnaOxIde app glue over W342/W343/W344 contract labels and W344 command-client buckets.
4. Lifecycle proof files live under `target/`; checked-in `examples/thin-slice/Module1.bas` remains unchanged.
5. W346 owns the first interaction/e2e harness and must not overclaim full browser/accessibility/runtime behavior.

## 24. W346 DnaOxIde Interaction Harness Acceptance Target

W346 closes against the static frontend interaction model plus static DOM/token smoke harness:

```powershell
npm --prefix apps/dna-oxide run interaction-command:check
npm --prefix apps/dna-oxide run interaction-focus:check
npm --prefix apps/dna-oxide run interaction-lifecycle:check
npm --prefix apps/dna-oxide run interaction-services:check
npm --prefix apps/dna-oxide run host-ui:check
npm --prefix apps/dna-oxide run host-lifecycle:check
npm --prefix apps/dna-oxide run host-services:check
npm --prefix apps/dna-oxide run command-client:check
npm --prefix apps/dna-oxide run scaffold:check
cargo test --manifest-path crates/Cargo.toml --workspace
```

Observed W346 acceptance evidence is captured in `target/w346-acceptance.txt` and contains:

- `DNA_OXIDE_INTERACTION_HARNESS`,
- `frontend-interaction-model+static-dom-token-smoke`,
- command palette and keyboard routing checks,
- no-mouse focus route over W345 host pane roles,
- lifecycle command sequence checks over proven OxIde-only commands,
- blocked build/run/Immediate/debug/watch/breakpoint/COM/reference command checks,
- static smoke tokens from W345 rendered host shell outputs,
- checked-in fixture mutation guard,
- no true live/browser/accessibility/runtime/COM/fake-data claim tokens.

Implementation notes:

1. W346 is the first host regression base, not a live browser/WebView driver.
2. Interaction scripts live under `apps/dna-oxide/scripts/verify-interaction-*.mjs`.
3. `apps/dna-oxide/src/interaction-harness.js` owns only DnaOxIde static interaction proof logic; reusable UI/host truth remains in shared crates and W343/W344 boundaries.
4. W347 owns compile/options/reference placeholder panels on this host UI and interaction base.

## 25. W347 Compile / Reference Placeholder Acceptance Target

W347 closes against DnaOxIde host-shell placeholder panels for compile/options/reference/COM surfaces:

```powershell
npm --prefix apps/dna-oxide run compile-panels:check
npm --prefix apps/dna-oxide run reference-panels:check
npm --prefix apps/dna-oxide run placeholder-commands:check
npm --prefix apps/dna-oxide run host-ui:check
npm --prefix apps/dna-oxide run interaction-services:check
npm --prefix apps/dna-oxide run scaffold:check
cargo test --manifest-path crates/Cargo.toml --workspace
```

Observed W347 acceptance evidence is captured in `target/w347-acceptance.txt` and contains:

- project properties, compile options, build/check, and run target panels,
- references, COM candidate, reference repair/apply, and COM runtime boundary panels,
- `proven-oxide-only`, `pending-oxvba-hardening`, `oxvba-fixture-evidenced`, and `unavailable-no-claim` labels,
- `EmbeddedBuildRunHost::build_workspace`,
- `ComSelectionService direct Rust surface`,
- `data-final-oxvba-dtos-owned-here="false"`,
- zero output/roster/candidate/preview rows,
- `data-com-runtime-invocation="false"`,
- false runtime/COM/fake-data claim attributes.

Implementation notes:

1. W347 does not define final OxVba DTOs.
2. Compile options, run targets, reference repair/apply, and COM native boundary remain pending-hardening or unavailable.
3. COM runtime invocation remains unclaimed.
4. W348 owns DnaOneCalc shared UI reuse proof without sibling repo writes.

## 26. W348 DnaOneCalc Shared UI Reuse Acceptance Target

W348 closes against an OxIde-only DnaOneCalc consumer profile and two deterministic GUI-lab renders:

```powershell
node tools/verify-dnaonecalc-profile.mjs
node tools/verify-dnaonecalc-reuse.mjs
cargo run --manifest-path crates/Cargo.toml -p oxide-guilab -- render gui-dnaonecalc-web-shell-host-contract
cargo run --manifest-path crates/Cargo.toml -p oxide-guilab -- render gui-shared-ui-shell-component
```

Observed W348 acceptance evidence is captured in `target/w348-acceptance.txt` and contains:

- `docs/fixtures/dnaonecalc-consumer-profile.json`,
- `DnaOneCalcWebShellHostPacket`,
- `GuiShellPacket`, `RuntimeServicePacket`, `ImmediateServicePacket`, and `DebugServicePacket`,
- `oxide-ui-leptos`, `oxide-host-bridge`, `oxide-core`, `oxide-webshell`, and `oxide-guilab`,
- `data-host="DnaOneCalc"`,
- `data-sibling-repo-writes="false"`,
- `data-host-mount-claimed="false"`,
- `data-component-crate="oxide-ui-leptos"`,
- `role="shared-ide-surface"`,
- `ThinSliceHello` and `Module1.bas`,
- false native runtime, COM runtime, fake response, fake debug data, and DOM-audit claims.

Implementation notes:

1. W348 proves reusable shared UI/host-bridge packets inside OxIde only.
2. W348 does not write to `C:/Work/DnaCalc/DnaOneCalc`.
3. W348 does not claim a real DnaOneCalc product mount.
4. Real DnaOneCalc mount evidence remains gated on explicit sibling-repo authorization and paired DnaOneCalc tests.
5. W349 owns the W341-W348 composition/readiness audit.

## 27. W349 DnaOxIde While-OxVba Acceptance Target

W349 closes the W341-W348 continuation runway with evidence audit, full regression, and OxVba readiness reporting:

```powershell
cargo test --manifest-path crates/Cargo.toml --workspace
cargo test --manifest-path apps/dna-oxide/src-tauri/Cargo.toml
node tools/verify-dnaonecalc-profile.mjs
node tools/verify-dnaonecalc-reuse.mjs
```

Observed W349 acceptance evidence is captured in:

- `target/w349-evidence-audit.txt`,
- `target/w349-regression.txt`,
- `target/w349-readiness-report.txt`,
- `target/w349-acceptance.txt`,
- `docs/DNAOXIDE_OXVBA_INTEGRATION_READINESS.md`,
- `docs/HANDOFF_W349_DNAOXIDE_WHILE_OXVBA_ACCEPTANCE.md`.

Implementation notes:

1. W349 accepts the OxIde-side DnaOxIde runway, not full OxVba runtime/debug/Immediate/COM behavior.
2. W349 keeps true native runtime, COM runtime, COM runtime invocation, fake response, fake debug data, DOM audit, DnaOneCalc mount, and sibling write claims absent.
3. The next meaningful capability work is direct OxIde adapter adoption over OxVba DTOs/APIs for W355/W360/W365/W370, with local temp-project tests before any claim flags flip.
4. If adapter work is not ready, DnaOxIde can continue packaging/polish only while preserving no-claim states.

## 28. Cross-Repo Fixture Policy

If a fixture belongs better in OxVba or DnaOneCalc, create a handoff and consume it from the authoritative repo after coordination. Do not duplicate project semantics locally just to make a short-term OxIde demo easier.
