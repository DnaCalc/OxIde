# OxIde Workset Register

Ordered worksets for OxIde's current green-field implementation
sequence.

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

## Current Sequence

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
9. `W045` — WTD demo backfill and affordance wiring audit
10. `W050` — file and document lifecycle services in the TUI shell
11. `W060` — full language-service UX over direct OxVba semantics
12. `W070` — run / debug / immediate shell surfaces over OxVba
    execution contracts
13. `W080` — debug surfaces (callstack / locals / watches /
    breakpoints, step control)
14. `W090` — command system and keymap profiles
15. `W100` — terminal capability and onboarding (probe, degradation,
    status-line hints)
16. `W110` — polish, accessibility, and recovery; WTD regression
    suite locked

## Workset Specs

- [W010_shell_mockup_scaffold.md](worksets/W010_shell_mockup_scaffold.md)
- [W020_runtime_shell_foundation.md](worksets/W020_runtime_shell_foundation.md)
- [W030_service_integration.md](worksets/W030_service_integration.md)
- [W035_ux_design_pass.md](worksets/W035_ux_design_pass.md)
- [W037_wtd_harness.md](worksets/W037_wtd_harness.md)
- [W038_ux_development_lab.md](worksets/W038_ux_development_lab.md)
- [W039_firehorse_terminal_ux_proof.md](worksets/W039_firehorse_terminal_ux_proof.md)
- [W040_project_workspace_management.md](worksets/W040_project_workspace_management.md)
- [W045_wtd_demo_backfill.md](worksets/W045_wtd_demo_backfill.md)
- [W050_file_document_services.md](worksets/W050_file_document_services.md)
- [W060_full_language_service_ux.md](worksets/W060_full_language_service_ux.md)
- [W070_run_debug_immediate_surfaces.md](worksets/W070_run_debug_immediate_surfaces.md)
- [W080_debug_surfaces.md](worksets/W080_debug_surfaces.md)
- [W090_command_system.md](worksets/W090_command_system.md)
- [W100_terminal_capability.md](worksets/W100_terminal_capability.md)
- [W110_polish_and_recovery.md](worksets/W110_polish_and_recovery.md)

## Use Rule

Use this document as:
1. the repo-local workset authority,
2. the place that records which worksets exist and in what order,
3. the entry point into per-workset design documents.

Do not use this document as:
1. a blocker tracker,
2. a status board,
3. a replacement for `.beads/`.
