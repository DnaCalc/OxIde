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
