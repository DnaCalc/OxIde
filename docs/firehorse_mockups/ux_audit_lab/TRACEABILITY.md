# Fire Horse UX Audit Lab Traceability

This document is the first W041 audit authority map. It lets a reviewer
or coding agent trace each audit target from UX authority to Fire Horse
scenario, reference artifact, terminal capture, projection contract, and
downstream implementation owner.

## Authority Inputs

| Source | Role |
| --- | --- |
| `docs/uxpass/00_principles.md` | General UX principles: source-centered work, honest state, no fake semantics. |
| `docs/uxpass/10_user_journeys.md` | Baseline user journeys and visible state transitions from the earlier UX pass. |
| `docs/uxpass/20_frame_and_regions.md` | Frame, region, and focus model provenance. |
| `docs/uxpass/60_reconciliation.md` | Reconciliation trail back into product direction. |
| `docs/DESIGN_TUI_2026_FIRE_HORSE.md` | Current Fire Horse UX doctrine and high-end terminal ambition. |
| `docs/firehorse_mockups/UX_RESET.md` | Rule that old OxIde front-end UI work is not maintained in parallel. |
| `docs/firehorse_mockups/UX_PROJECTION_CONTRACT.md` | W039 projection, state ownership, action id, and OxVba seam contract. |
| `docs/firehorse_mockups/HARDENING_REVIEW.md` | Mockup hardening and W039 handoff trail. |
| `docs/firehorse_mockups/frankentui_terminal_review/README.md` | Current W038-B15 FrankenTui terminal review pack. |

## Personas

| Persona id | Authority | Primary scenario pressure |
| --- | --- | --- |
| `pricing_maintainer` | Fire Horse editing/run scenarios and W035 source-centered principles. | Maintain pricing VBA quickly with semantic confidence and safe run feedback. |
| `migration_reviewer` | W035 meaning/provenance principles and W039 seam honesty rules. | Review inherited VBA without letting UI copy masquerade as semantic truth. |
| `debug_responder` | Fire Horse Debug Cockpit and W070/W080 direction. | Diagnose a paused failure under pressure with stack, local, watch, and step clarity. |
| `terminal_power_user` | Fire Horse high-end terminal ambition and W100 degradation doctrine. | Use dense terminal performance without accepting a low-end console ceiling. |

## Scenario Traceability

| Audit scenario | Persona | Fire Horse scenario | Reference mockup | Current terminal capture | Projection / seam authority | Downstream owner |
| --- | --- | --- | --- | --- | --- | --- |
| `audit-launchpad-cold-start` | `terminal_power_user` | `firehorse-launchpad-standard` | `docs/firehorse_mockups/refined_01_launchpad.png` | `docs/firehorse_mockups/frankentui_terminal_review/captures/launchpad_studio.txt` | `UX_PROJECTION_CONTRACT.md` Identity Rail, Project Spine absence, Key Rail, Terminal Fit. | W040, W090, W100 |
| `audit-editing-lens-pricing` | `pricing_maintainer` | `firehorse-editing-lens-standard` | `docs/firehorse_mockups/refined_02_editing_lens.png` | `docs/firehorse_mockups/frankentui_terminal_review/captures/editing_lens_studio.txt` | `UX_PROJECTION_CONTRACT.md` Code Canvas, Context Dock, Activity Deck, diagnostics, hover. | W050, W060, W090 |
| `audit-command-lens-run` | `terminal_power_user` | `firehorse-command-lens-standard` | `docs/firehorse_mockups/refined_03_command_lens.png` | `docs/firehorse_mockups/frankentui_terminal_review/captures/command_lens_first-class.txt` | `UX_PROJECTION_CONTRACT.md` Overlay and command/action matrix. | W090 |
| `audit-run-lane-progress` | `pricing_maintainer` | `firehorse-run-lane-standard` | `docs/firehorse_mockups/refined_04_run_lane.png` | `docs/firehorse_mockups/frankentui_terminal_review/captures/run_lane_studio.txt` | `UX_PROJECTION_CONTRACT.md` Run Lane and `WebHostEvent` stream mapping. | W070 |
| `audit-debug-cockpit-paused` | `debug_responder` | `firehorse-debug-cockpit-standard` | `docs/firehorse_mockups/refined_05_debug_cockpit.png` | `docs/firehorse_mockups/frankentui_terminal_review/captures/debug_cockpit_studio.txt` | `UX_PROJECTION_CONTRACT.md` Debug Cockpit and W080 debug-contract audit. | W080 |
| `audit-console-fit-light` | `terminal_power_user` | `firehorse-console-fit-light` | `docs/firehorse_mockups/refined_06_console_fit.png` | `docs/firehorse_mockups/frankentui_terminal_review/captures/console_fit_studio.txt` | `UX_PROJECTION_CONTRACT.md` Terminal Fit and capability posture. | W100 |
| `audit-compact-focus-degradation` | `terminal_power_user` | `firehorse-focus-compact` | `docs/firehorse_mockups/refined_07_compact_focus_mode.png` | `docs/firehorse_mockups/frankentui_terminal_review/captures/compact_focus_default_compact.txt` | `UX_PROJECTION_CONTRACT.md` Compact Focus and degradation discipline. | W100, W110 |
| `audit-real-editing-adapter-honesty` | `migration_reviewer` | `firehorse-real-editing` | `docs/firehorse_mockups/refined_02_editing_lens.png` | `docs/firehorse_mockups/frankentui_terminal_review/captures/real_editing_adapter_studio.txt` | `UX_PROJECTION_CONTRACT.md` read-only adapter and explicit unavailable seam rules. | W050, W060 |

## Functional Criteria Coverage

| Criterion group | Covered by | Notes |
| --- | --- | --- |
| Persona fit | All audit scenarios | The scenario must state who it serves and the pressure it addresses. |
| Journey fit | All audit scenarios | Each scenario has named steps and expected visible surfaces/actions. |
| Command clarity | Launchpad, Command Lens, Editing Lens, Run Lane, Debug Cockpit | Visible commands must carry action ids and disabled reasons where applicable. |
| State ownership | All audit scenarios | Surface expectations map to projection paths and downstream worksets. |
| Seam honesty | Editing Lens, Run Lane, Debug Cockpit, Real Editing Adapter | VBA/project facts must map to OxVba seams or explicit future/unavailable seams. |
| Degradation | Console Fit, Compact Focus, plus high-end cross-checks | Compact is a degradation target, not the design ceiling. |

## Aesthetic Criteria Coverage

| Criterion group | Covered by | Notes |
| --- | --- | --- |
| Hierarchy | Studio and First-class captures | Source, context, activity, and commands must be immediately scannable. |
| Density | Studio and First-class captures | High-end viewports should use space richly without filler. |
| Balance | All three-column and cockpit layouts | Rails, docks, canvas, and lower surfaces must carry intentional visual weight. |
| Tone and color | ANSI captures | Truecolor review uses `docs/firehorse_mockups/frankentui_terminal_review/ansi/`. |
| Terminal craft | Text and ANSI captures | Box drawing, padding, clipping, and fixed-width rhythm are reviewed as terminal-native craft. |
| Reference fidelity | Refined PNGs plus terminal captures | Audit results cite what the terminal keeps, changes, or rejects from the colourful mockups. |
| Text fit | All target viewports | No coherent target may require overlapping text or hidden command labels. |
| Emotional fit | High-end Studio/First-class review | The design must feel modern, capable, calm under pressure, and not retro-console nostalgic. |

## Known Gaps

| Gap | Impact | Planned owner |
| --- | --- | --- |
| Interactive review is lab-only. | `oxide-uxlab --audit` opens the Audit Lab cockpit, but it is not production `ox-ide` UI and does not imply product behavior. | W041-B04 through W041-B08 |
| Export packs are local evidence. | `--export` writes scenario or suite packs, but refuses to overwrite existing export files. | W041-B09 |
| Aesthetic emotional fit remains manual. | Scorecards can flag the concern, but human or agent rationale must cite reference artifacts before claiming readiness. | W041-B07, W041-B08, W041-B10 |
| No production semantic/run/debug behavior implied by fixtures. | Fire Horse scenes remain proof surfaces, not product behavior. | W060, W070, W080 |

## Closure Check

- All W039 Fire Horse scenario ids appear in the traceability table.
- Every row has a persona, reference mockup, terminal capture, projection
  authority, and downstream owner.
- Gaps are explicit and assigned to future W041 or downstream beads.
