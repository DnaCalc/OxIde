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

## Tier 1 — Current Shape

Specs executors will touch next. Rewritten to the current template:

- `W035_ux_design_pass.md`
- `W037_wtd_harness.md`
- `W038_ux_development_lab.md`
- `W039_firehorse_terminal_ux_proof.md`
- `W040_project_workspace_management.md`
- `W045_wtd_demo_backfill.md`
- `W050_file_document_services.md`
- `W060_full_language_service_ux.md`
- `W070_run_debug_immediate_surfaces.md`
- `W080_debug_surfaces.md`
- `W090_command_system.md`
- `W100_terminal_capability.md`
- `W110_polish_and_recovery.md`

## Tier 2 — Historical

Closed retrospective specs kept for provenance. They predate the
current bead schema and are not reshaped:

- `W010_shell_mockup_scaffold.md`
- `W020_runtime_shell_foundation.md`
- `W030_service_integration.md`

New work against these areas starts with a new workset in the
current template.
