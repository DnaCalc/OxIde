# OxIde Worksets

Per-workset design documents. See [`BEADS.md`](../BEADS.md) for the
working method and the workset template; see
[`WORKSET_REGISTER.md`](../WORKSET_REGISTER.md) for the ordered
sequence.

## Spec Shape

Every workset spec at `W<NNN>_<slug>.md` follows the template from
`BEADS.md` §4.1:

```
# Workset W<NNN> — <Ambition phrase>

## Ambition
## Dependencies
## Design
## Beads
## Out-of-scope
```

No `Progress` section. No `Status:` preamble. Progress lives in git
log and in `.beads/` closure records.

## Active GUI Pivot

Specs executors will touch next for the Rust/WASM-capable GUI pivot:

- `W200_gui_pivot_foundation.md`
- `W210_gui_project_open_spine.md`
- `W220_editable_module_and_diagnostics.md`
- `W230_save_reload_session_restore.md`
- `W240_capability_aware_run_output.md`
- `W250_dnaonecalc_embedding_proof.md`
- `W260_windows_com_capability_proof.md`
- `W270_run_debug_immediate_gui_surfaces.md`
- `W280_command_keyboard_accessibility_polish.md`
- `W290_host_mounted_gui_shell.md`
- `W300_mounted_web_shell_adapter.md`
- `W310_dnaonecalc_web_shell_hosting.md`
- `W320_native_filesystem_session_persistence.md`
- `W330_oxvba_native_runtime_service_contract.md`
- `W340_dnaoxide_standalone_host_foundation.md`
- `W341_dnaoxide_tauri_app_scaffold.md`
- `W342_shared_ide_ui_component_layer.md`
- `W343_oxide_host_bridge_facade.md`
- `W344_dnaoxide_tauri_command_boundary_stubs.md`
- `W345_dnaoxide_live_host_ui_proof.md`
- `W346_dnaoxide_interaction_e2e_harness.md`
- `W347_compile_options_reference_placeholders.md`
- `W348_dnaonecalc_shared_ui_reuse_path.md`
- `W349_dnaoxide_while_oxvba_acceptance.md`
- `W350_dnaoxide_live_editable_source_app.md`
- `W352_dnaoxide_tauri_webview_automation.md` — Tauri/WebView product host and automation
- `W355_dnaoxide_compile_build_adapter.md` — OxVba compile/build adapter profiles
- `W360_dnaoxide_com_reference_adapter.md`
- `W365_dnaoxide_runtime_immediate_adapter.md`
- `W370_dnaoxide_debug_watch_breakpoint_adapter.md`

`W341`-`W349` cover the OxIde-side continuation runway while OxVba executes its DNA OxIde full-scope host integration support workset. OxVba feedback now identifies available-subset direct Rust surfaces plus a ThinSliceHello fixture-evidenced ladder for build/run/Immediate/debug/watch/breakpoint/COM capability seams.

After W350, browser DOM evidence is a regression harness and review artifact, not the product substrate. `W352` is now the desktop product-host lane: DnaOxIde must run in Tauri/WebView and prove UI -> linked native Rust command flow. `W355` then maps and proves compile/build adapter profiles for browser website/DnaOneCalc WASM and native desktop hosts without freezing a rigid cross-repo contract. `W360`/`W365`/`W370` connect OxVba direct Rust adapters for reference/COM, runtime/Immediate, and debug/watch/breakpoint surfaces with host-profile-specific availability. Future beads must exercise real endpoint seams: shared Rust/WASM UI, DnaOneCalc browser WASM host integration, Tauri/native Rust command integration, or OxVba adapter integration.

## Parked TUI Lineage

Specs retained for provenance, design evidence, and possible future
companion-TUI work. They no longer define the active implementation
sequence:

- `W010_shell_mockup_scaffold.md`
- `W020_runtime_shell_foundation.md`
- `W030_service_integration.md`
- `W035_ux_design_pass.md`
- `W037_wtd_harness.md`
- `W038_ux_development_lab.md`
- `W039_firehorse_terminal_ux_proof.md`
- `W040_project_workspace_management.md`
- `W041_firehorse_ux_audit_lab.md`
- `W045_wtd_demo_backfill.md`
- `W050_file_document_services.md`
- `W060_full_language_service_ux.md`
- `W070_run_debug_immediate_surfaces.md`
- `W080_debug_surfaces.md`
- `W090_command_system.md`
- `W100_terminal_capability.md`
- `W110_polish_and_recovery.md`

New work against parked TUI areas should start with an explicit new
workset rather than silently resuming the old sequence.
