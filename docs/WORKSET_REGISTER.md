# OxIde Workset Register

Ordered worksets for OxIde's current implementation sequence.

The active direction is now the Rust/WASM-capable GUI pivot. The
previous FrankenTui sequence is retained as parked TUI lineage rather
than removed.

This file owns **workset truth**: which worksets exist, in what order,
and what ambition each carries. It does not own bead state; that lives
in `.beads/` (see [`BEADS.md`](BEADS.md)).

## Workset Rule

A workset partitions ambition. Each workset spec at
`docs/worksets/W<NNN>_<slug>.md` is a design document, not a progress
log. It reads:

- **Ambition** — the user capability delivered end-to-end.
- **Dependencies** — upstream worksets / beads.
- **Design** — the shape of the end state (UX, architecture,
  contracts, interactions) in enough detail to review without code.
- **Beads** — the full ordered bead list. Each bead uses the schema
  from [`BEADS.md`](BEADS.md) §2.1 (Goal / Design / Tests / Evidence /
  Closure).
- **Out-of-scope** — explicit deferrals, with pointers to the future
  worksets that pick them up.

No `Progress` section. Progress lives in git log and `.beads/`
closures.

Worksets are **ambitious in scope** (a real user capability) and
**comprehensive in design** (every bead in the list knows its tests
and evidence shape before work starts).

## Parked TUI Lineage

These worksets are retained as design/prototype/evidence history for
the FrankenTui direction. They do not define the active implementation
sequence for the GUI pivot.

1. `W010` — shell mockup scaffold and design proof in FrankenTui
   *(historical — predates the current bead schema)*
2. `W020` — runtime shell foundation on top of the proven mockup
   *(historical)*
3. `W030` — project/document/OxVba service integration
   *(historical)*
4. `W035` — fresh UX design pass, reconciled back into
   `PRODUCT_DIRECTION.md` and `DESIGN_TUI.md`
5. `W037` — WinTermDriver test harness foundation
6. `W038` — UX development lab (scene flag, scenario driver, VT
   replay / diff, `oxide-uxlab` TUI)
7. `W039` — Fire Horse terminal UX proof (terminal-cell mockups,
   projection contracts, command/action matrix, OxVba seam mapping)
8. `W040` — project and workspace management shell over OxVba-owned
   truth
9. `W041` — Fire Horse UX Audit Lab (`oxide-uxlab` audit cockpit over
   personas, scenarios, mockups, state/action mappings, and OxVba seams)
10. `W045` — WTD demo backfill and affordance wiring audit
11. `W050` — file and document lifecycle services in the TUI shell
12. `W060` — full language-service UX over direct OxVba semantics
13. `W070` — run / debug / immediate shell surfaces over OxVba
    execution contracts
14. `W080` — debug surfaces (callstack / locals / watches /
    breakpoints, step control)
15. `W090` — command system and keymap profiles
16. `W100` — terminal capability and onboarding (probe, degradation,
    status-line hints)
17. `W110` — polish, accessibility, and recovery; WTD regression
    suite locked

## Active GUI Pivot Sequence

1. `W200` — GUI pivot foundation, codebase review, and TUI parking
2. `W210` — GUI project-open spine
3. `W220` — editable module and diagnostics
4. `W230` — save, reload, and session restore
5. `W240` — capability-aware run and output
6. `W250` — DnaOneCalc embedded IDE and runtime proof
7. `W260` — Windows COM capability proof
8. `W270` — run, debug, and Immediate GUI surfaces
9. `W280` — command, keyboard, accessibility, and polish
10. `W290` — host-mounted GUI shell
11. `W300` — mounted web shell adapter
12. `W310` — DnaOneCalc web shell hosting
13. `W320` — native filesystem and session persistence
14. `W330` — OxVba native runtime service contract
15. `W340` — DnaOxIde standalone host foundation
16. `W341` — DnaOxIde Tauri app scaffold
17. `W342` — shared IDE UI component layer
18. `W343` — OxIde host bridge facade
19. `W344` — DnaOxIde Tauri command boundary stubs
20. `W345` — DnaOxIde live host UI proof
21. `W346` — DnaOxIde interaction and e2e harness
22. `W347` — compile options and reference UI placeholders/subset panels
23. `W348` — DnaOneCalc shared UI reuse path
24. `W349` — DnaOxIde while-OxVba acceptance

`W341`-`W349` cover the OxIde-side continuation runway while OxVba executes its DNA OxIde full-scope host integration support workset. The runway proceeds with DnaOxIde scaffold, shared UI, host bridge, Tauri command boundaries, live host proof, interaction harness, compile/options/reference panels, and DnaOneCalc reuse path. OxVba feedback now identifies several available-subset direct Rust surfaces, so these worksets should prefer subset-backed adapter evidence where possible while keeping stable IDs/taxonomy/watch/breakpoint/COM-runtime/full-debug claims gated on OxVba evidence and explicit sibling-repo authorization.

## Workset Specs

- [W010_shell_mockup_scaffold.md](worksets/W010_shell_mockup_scaffold.md)
- [W020_runtime_shell_foundation.md](worksets/W020_runtime_shell_foundation.md)
- [W030_service_integration.md](worksets/W030_service_integration.md)
- [W035_ux_design_pass.md](worksets/W035_ux_design_pass.md)
- [W037_wtd_harness.md](worksets/W037_wtd_harness.md)
- [W038_ux_development_lab.md](worksets/W038_ux_development_lab.md)
- [W039_firehorse_terminal_ux_proof.md](worksets/W039_firehorse_terminal_ux_proof.md)
- [W040_project_workspace_management.md](worksets/W040_project_workspace_management.md)
- [W041_firehorse_ux_audit_lab.md](worksets/W041_firehorse_ux_audit_lab.md)
- [W045_wtd_demo_backfill.md](worksets/W045_wtd_demo_backfill.md)
- [W050_file_document_services.md](worksets/W050_file_document_services.md)
- [W060_full_language_service_ux.md](worksets/W060_full_language_service_ux.md)
- [W070_run_debug_immediate_surfaces.md](worksets/W070_run_debug_immediate_surfaces.md)
- [W080_debug_surfaces.md](worksets/W080_debug_surfaces.md)
- [W090_command_system.md](worksets/W090_command_system.md)
- [W100_terminal_capability.md](worksets/W100_terminal_capability.md)
- [W110_polish_and_recovery.md](worksets/W110_polish_and_recovery.md)
- [W200_gui_pivot_foundation.md](worksets/W200_gui_pivot_foundation.md)
- [W210_gui_project_open_spine.md](worksets/W210_gui_project_open_spine.md)
- [W220_editable_module_and_diagnostics.md](worksets/W220_editable_module_and_diagnostics.md)
- [W230_save_reload_session_restore.md](worksets/W230_save_reload_session_restore.md)
- [W240_capability_aware_run_output.md](worksets/W240_capability_aware_run_output.md)
- [W250_dnaonecalc_embedding_proof.md](worksets/W250_dnaonecalc_embedding_proof.md)
- [W260_windows_com_capability_proof.md](worksets/W260_windows_com_capability_proof.md)
- [W270_run_debug_immediate_gui_surfaces.md](worksets/W270_run_debug_immediate_gui_surfaces.md)
- [W280_command_keyboard_accessibility_polish.md](worksets/W280_command_keyboard_accessibility_polish.md)
- [W290_host_mounted_gui_shell.md](worksets/W290_host_mounted_gui_shell.md)
- [W300_mounted_web_shell_adapter.md](worksets/W300_mounted_web_shell_adapter.md)
- [W310_dnaonecalc_web_shell_hosting.md](worksets/W310_dnaonecalc_web_shell_hosting.md)
- [W320_native_filesystem_session_persistence.md](worksets/W320_native_filesystem_session_persistence.md)
- [W330_oxvba_native_runtime_service_contract.md](worksets/W330_oxvba_native_runtime_service_contract.md)
- [W340_dnaoxide_standalone_host_foundation.md](worksets/W340_dnaoxide_standalone_host_foundation.md)
- [W341_dnaoxide_tauri_app_scaffold.md](worksets/W341_dnaoxide_tauri_app_scaffold.md)
- [W342_shared_ide_ui_component_layer.md](worksets/W342_shared_ide_ui_component_layer.md)
- [W343_oxide_host_bridge_facade.md](worksets/W343_oxide_host_bridge_facade.md)
- [W344_dnaoxide_tauri_command_boundary_stubs.md](worksets/W344_dnaoxide_tauri_command_boundary_stubs.md)
- [W345_dnaoxide_live_host_ui_proof.md](worksets/W345_dnaoxide_live_host_ui_proof.md)
- [W346_dnaoxide_interaction_e2e_harness.md](worksets/W346_dnaoxide_interaction_e2e_harness.md)
- [W347_compile_options_reference_placeholders.md](worksets/W347_compile_options_reference_placeholders.md)
- [W348_dnaonecalc_shared_ui_reuse_path.md](worksets/W348_dnaonecalc_shared_ui_reuse_path.md)
- [W349_dnaoxide_while_oxvba_acceptance.md](worksets/W349_dnaoxide_while_oxvba_acceptance.md)

## Use Rule

Use this document as:
1. the repo-local workset authority,
2. the place that records which worksets exist and in what order,
3. the entry point into per-workset design documents.

Do not use this document as:
1. a blocker tracker,
2. a status board,
3. a replacement for `.beads/`.
