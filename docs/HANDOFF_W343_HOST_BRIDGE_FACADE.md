# Handoff — W343 OxIde Host Bridge Facade

Status: `accepted_oxide_host_bridge_facade`
Date: 2026-05-07
Workset: [`W343_oxide_host_bridge_facade.md`](worksets/W343_oxide_host_bridge_facade.md)

## Summary

W343 establishes `oxide-host-bridge` as the host-neutral service facade between shared OxIde UI and concrete host implementations such as **DnaOxIde / DNA OxIde**, browser review fixtures, and future DnaOneCalc host mounts.

The bridge is intentionally not a Tauri API, not a DnaOxIde app API, and not an OxVba DTO owner. It names service categories, command intents, evidence states, and no-claim defaults so shared UI can render honest capability and disabled-reason states while real adapters are added.

## Delivered Surface

Crate: `crates/oxide-host-bridge`

Key public pieces:

- `OxideHostBridgeRole`,
- `HostBridgeCapabilityState`, including `oxvba-fixture-evidenced`,
- `HostBridgeServiceCategory`,
- `HostBridgeServiceStatus`,
- `HostBridgeConsumerKind`,
- `HostCommandIntent`,
- `HostBridgeCommandSpec`,
- `HostBridgeCommandAvailability`,
- `HostBridgeResponse`,
- `HostProjectApi`, `HostDocumentApi`, `HostLanguageApi`, `HostCompileApi`, `HostReferenceApi`, `HostRuntimeApi`, `HostImmediateApi`, `HostDebugApi`, `HostSettingsApi`, `HostCapabilityApi`,
- `HostDnaOneCalcWebShellApi`,
- `host_bridge_command_catalog`,
- `command_availability_for_statuses`,
- `BrowserReviewFixtureHost`.

Shared UI integration:

- `oxide-ui-leptos::render_host_bridge_command_panel`,
- `UiDataProvenance::OxVbaFixtureEvidenced`,
- deterministic command availability render over `HostBridgeCommandAvailability`.

GUI-lab evidence route:

```powershell
cargo run --manifest-path crates/Cargo.toml -p oxide-guilab -- render gui-host-bridge-command-dispatch
```

## Evidence States

W343 uses four bridge states:

1. `proven-oxide-only` — OxIde has local evidence and the fixture command can be enabled.
2. `oxvba-available-subset` — a direct OxVba Rust surface exists, but the current OxIde host command remains disabled until adapter proof exists.
3. `oxvba-fixture-evidenced` — OxVba has ThinSliceHello fixture evidence, but the current OxIde command remains disabled until direct adapter proof exists.
4. `pending-oxvba-hardening` — DTOs, event streams, command availability, source mapping, native boundary, or host UX adoption remain pending.

The follow-up OxVba evidence file read for W343 was:

- `../OxVba/docs/evidence/DNAOXIDE_THIN_SLICE_HELLO_FIXTURE_2026-05-07.md`.

That evidence upgrades these seams to `oxvba-fixture-evidenced` adapter targets, not full OxIde claims:

- workspace load and editor overlay,
- overlay roster/version signal,
- overlay build/check,
- runtime session creation and stable runtime IDs,
- Immediate attach/evaluation over overlay source,
- debug attach,
- watch registry/evaluation,
- breakpoint binding DTO,
- stable frame/watch/breakpoint IDs,
- broken COM reference state,
- COM capability profile.

## Command Dispatch Boundary

The W343 command catalog maps shared UI command IDs to host service categories without Tauri coupling.

Examples:

- `project.open` → `HostProjectApi` → enabled in the browser-review fixture as `proven-oxide-only`.
- `compile.options` → `HostCompileApi` → disabled as `pending-oxvba-hardening`.
- `compile.check` → `HostCompileApi` → disabled as `oxvba-fixture-evidenced` until OxIde adapter proof exists.
- `runtime.run` → `HostRuntimeApi` → disabled as `oxvba-fixture-evidenced` until OxIde adapter proof exists.
- `runtime.stop` → `HostRuntimeApi` → disabled as `pending-oxvba-hardening`.
- `runtime.immediate` → `HostImmediateApi` → disabled as `oxvba-fixture-evidenced` until OxIde adapter proof exists.
- `watch.upsert` and `breakpoint.set` → `HostDebugApi` → disabled as `oxvba-fixture-evidenced` until OxIde adapter proof exists.
- `references.com.search` → `HostReferenceApi` → disabled as `oxvba-fixture-evidenced`; COM runtime invocation remains unclaimed.

## Claim Boundaries

W343 does **not** claim:

- live Tauri/WebView command execution,
- real DnaOxIde runtime execution,
- real Immediate Window UX,
- real debug/watch/breakpoint UX,
- COM runtime invocation,
- full source-span mapping,
- full shared capability/error taxonomy,
- real DnaOneCalc host mount.

Required no-claim defaults remain false in the bridge and shared UI renders:

- `real_execution_claimed`,
- `native_runtime_claimed`,
- `com_runtime_claimed`,
- `fake_immediate_responses`,
- `fake_debug_data`.

## Acceptance Evidence

Acceptance evidence is captured in `target/w343-acceptance.txt`.

Required commands:

```powershell
cargo test --manifest-path crates/Cargo.toml --workspace
cargo run --manifest-path crates/Cargo.toml -p oxide-guilab -- render gui-host-bridge-command-dispatch
```

Observed non-blocking warning class:

- frozen OxVba `unexpected cfg condition name: kani` / dead-code warnings.

## Next Workset

W344 should build DnaOxIde Tauri-ready command stubs over this facade. It should keep the same state split:

1. proven OxIde-only,
2. OxVba available-subset,
3. OxVba fixture-evidenced adapter target,
4. pending-hardening unavailable.

W344 must not flip runtime/debug/Immediate/COM claim flags until direct OxIde adapter tests prove the specific command path.
