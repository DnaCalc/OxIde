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

`W210` through `W290` are scaffolded future worksets. Expand each
Beads section into concrete executable packets before starting code
against that workset.

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
