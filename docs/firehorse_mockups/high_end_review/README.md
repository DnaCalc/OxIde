# Fire Horse High-End Terminal Review Pack

Status: refined high-end UX-lab review pack
Type: review artifact
Date: 2026-04-28

This pack compares the approved colourful Fire Horse mockups with the
current W039 terminal-cell renderer at desktop-oriented targets.

The pack keeps both the baseline and refined captures. The baseline
captures show that the W039 proof established surface mapping but did not
yet deliver the high-end density, rhythm, and emotional force expected
from a desktop-first TUI IDE. The refined captures are the current
high-end evaluation target.

## Viewport Targets

| Target | Cells | Meaning |
| --- | --- | --- |
| First-class | 160x42 | Primary desktop review target. |
| Studio | 190x48 | Premium near-GUI terminal IDE target. |

Compact and fallback hosts remain required, but they are not the design
ceiling. The first-class and studio targets should be judged against the
colourful mockups first, then scaled down.

## Refined Captures

The refined captures were generated after the W038-B14 high-end renderer
pass:

| Scene | Mockup | First-class refined | Studio refined | Baseline first-class |
| --- | --- | --- | --- | --- |
| Editing Lens | `../refined_02_editing_lens.png` | `captures/editing_lens_first_class_refined.txt` | `captures/editing_lens_studio_refined.txt` | `captures/editing_lens_first_class_baseline.txt` |
| Command Lens | `../refined_03_command_lens.png` | `captures/command_lens_first_class_refined.txt` | `captures/command_lens_studio_refined.txt` | `captures/command_lens_first_class_baseline.txt` |
| Run Lane | `../refined_04_run_lane.png` | `captures/run_lane_first_class_refined.txt` | `captures/run_lane_studio_refined.txt` | `captures/run_lane_first_class_baseline.txt` |
| Debug Cockpit | `../refined_05_debug_cockpit.png` | `captures/debug_cockpit_first_class_refined.txt` | `captures/debug_cockpit_studio_refined.txt` | `captures/debug_cockpit_first_class_baseline.txt` |

Live render commands:

```text
target\release\oxide-uxlab.exe --suite firehorse --scenario firehorse-editing-lens-standard --viewport first-class --once
target\release\oxide-uxlab.exe --suite firehorse --scenario firehorse-debug-cockpit-standard --viewport studio --once
```

## Baseline Captures

The baseline captures were generated with:

```text
target\release\oxide-uxlab.exe --suite firehorse --scenario <id> --viewport first-class --once
target\release\oxide-uxlab.exe --suite firehorse --scenario <id> --viewport studio --once
```

| Scene | Mockup | First-class baseline | Studio baseline | W039 text golden |
| --- | --- | --- | --- | --- |
| Editing Lens | `../refined_02_editing_lens.png` | `captures/editing_lens_first_class_baseline.txt` | `captures/editing_lens_studio_baseline.txt` | `../../../tests/wtd/goldens/W039/firehorse_editing_lens_standard.txt` |
| Command Lens | `../refined_03_command_lens.png` | `captures/command_lens_first_class_baseline.txt` | `captures/command_lens_studio_baseline.txt` | `../../../tests/wtd/goldens/W039/firehorse_command_lens_standard.txt` |
| Run Lane | `../refined_04_run_lane.png` | `captures/run_lane_first_class_baseline.txt` | `captures/run_lane_studio_baseline.txt` | `../../../tests/wtd/goldens/W039/firehorse_run_lane_standard.txt` |
| Debug Cockpit | `../refined_05_debug_cockpit.png` | `captures/debug_cockpit_first_class_baseline.txt` | `captures/debug_cockpit_studio_baseline.txt` | `../../../tests/wtd/goldens/W039/firehorse_debug_cockpit_standard.txt` |

## Scene Review

### Editing Lens

Kept:

- Core surface grammar is present: Identity Rail, Project Spine, Code
  Canvas, Context Dock, source lens, Activity Deck, and Key Rail.
- OxVba seam labels remain visible and reviewable.

Lost:

- At first-class/studio widths, the layout does not use the extra space
  to make source feel premium or calmer.
- The Project Spine, Context Dock, and Activity Deck read as table cells
  instead of rails/docks.
- The source lens lacks the visual anchoring and hierarchy of the
  mockup.

Needs redesign:

- Add actual terminal styling/colour once the renderer path supports it.
- Improve source lens anchoring beyond text rails.
- Add richer Context Dock cards from real W060 seams.

Recovered in the refined capture:

- First-class/studio-specific column geometry.
- Stronger Code Canvas center with source-adjacent semantic lens rhythm.
- Denser Activity Deck summary.

### Command Lens

Kept:

- Command rows carry action ids, bindings, enabled state, disabled
  reason, and preview content.
- The backing scene is identified as inactive.

Lost:

- The overlay does not feel like a premium modal surface at high-end
  sizes.
- Preview content is too sparse for the available width.
- The command list lacks grouping and consequence detail.

Needs redesign:

- Add terminal styling/colour and a stronger modal frame.
- Populate richer consequence preview rows when W090 owns the real
  registry.

Recovered in the refined capture:

- High-end overlay header and search/filter posture.
- Denser action rows with binding/action/state in one scan line.
- A dedicated preview column instead of stretched table cells.

### Run Lane

Kept:

- The staged run model is visible: prepare, analyze, build, execute,
  result.
- Activity rows preserve event-shaped fixture data.

Lost:

- The run state is detached from source continuity.
- Timeline rows are table-like rather than a workflow lane.
- First-class/studio space is not used for output/context side channels.

Needs redesign:

- Add real terminal colour/heat for active build step.
- Replace fixture event messages with typed W070 run events.

Recovered in the refined capture:

- Run Lane as a horizontal workflow rail.
- Source continuity beside the run timeline.
- Activity Deck expresses Output/Immediate as part of the posture.

### Debug Cockpit

Kept:

- Paused source line, call stack, locals, watches, and debug key rail
  are all present.
- The W080 debug-contract dependency remains explicit.

Lost:

- Debug output is the weakest high-end baseline. It flattens code,
  stack, locals, and watches into repetitive rows.
- The source line is truncated too aggressively.
- The composition does not yet feel like the colourful Debug Cockpit
  mockup.

Needs redesign:

- Add real terminal colour/heat for paused line and breakpoint state.
- Replace W080 audit fixture rows with real debug contract data when
  available.

Recovered in the refined capture:

- Studio debug layout with source continuity.
- Debug Dock hierarchy for call stack, locals, and watches.
- Immediate/Watch activity row at the bottom.

## Refined Verdict

The refined captures are ready for high-end UX evaluation of density,
layout hierarchy, and scene posture. They are not final visual fidelity:
the lab renderer still lacks real styling/colour, and the fixture data is
not product behavior. The next review question should be whether these
first-class/studio compositions now recover enough of the colourful
mockups' feel to become downstream product targets.
