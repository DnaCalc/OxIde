# Desktop TUI OxVba Workload Calibration

Status: calibration addendum
Type: design authority
Date: 2026-05-01

## Purpose

This document calibrates the Fire Horse UX direction against a more
ambitious contemporary desktop-class TUI IDE reference, while keeping
OxIde grounded in the simpler world it actually fronts: VBA projects,
OxVba project truth, semantic services, run/debug seams, and terminal
workflow.

The reference direction is useful because it proves that a modern TUI can
feel like a full IDE: layered, dense, modal where needed, status-rich,
and emotionally closer to a GUI-class tool than to an old console form.
OxIde should take that level of craft seriously.

OxIde should not copy the breadth of a general agent IDE. The VBA coding
universe is smaller. The richness should come from continuity,
provenance, and fast project work, not from adding unrelated panels.

## Calibration Verdict

The existing Fire Horse doctrine is directionally right:

- source is the stage;
- rails over boxes;
- Project Spine, Code Canvas, Context Dock, Activity Deck, and Key Rail;
- Command Lens as a product surface;
- Run Lane as a staged workflow;
- Debug Cockpit as a distinct posture;
- Studio and First-class as high-end desktop targets.

The gap is enforcement. Current proofs can still pass while feeling like
well-named panels rather than a daily IDE workspace. The Audit Lab must
raise the bar from "does this screen contain the right regions?" to "does
this screen make a real OxVba-backed VBA workflow feel dense, calm, and
credible?"

## What We Borrow From Desktop-Class TUI IDEs

Borrow these qualities:

- persistent navigator/project spine with active, dirty, generated,
  reference, and target state;
- tab or work-strip language for open documents, overlays, and tasks;
- layered overlays/dialogs that preserve backing state;
- right-side contextual activity for semantic details, run/debug state,
  or selected objects;
- bottom status/control rail with model, target, diagnostics, run/debug,
  context, and live key state;
- subtle truecolor hierarchy, not decorative color blocks;
- selected rows, disabled reasons, scroll hints, checkboxes, badges, and
  compact buttons when the task genuinely needs controls;
- crowded but legible high-end screens that look useful for a long
  desktop session.

Do not borrow these as scope:

- provider management;
- multi-repository dashboards;
- agent queue management;
- arbitrary shell-log lanes;
- general-purpose configuration centers;
- huge project trees unrelated to VBA module work.

OxIde's equivalent richness is narrower:

- project/module/reference/target truth;
- open buffers and dirty state;
- source text, diagnostics, hover, symbols, references, completions;
- run target, run timeline, output, Immediate;
- debug paused state, call stack, locals, watches, breakpoints;
- generated-code and unavailable-seam honesty;
- terminal capability and compact degradation.

## OxVba Workload Primitives

Every high-end proof should be built from these primitives, not from
decorative filler.

| Primitive | User value | Primary owner |
| --- | --- | --- |
| Project identity | The user knows which VBA project and target they are touching. | W040 / OxVba project truth |
| Module navigation | Modules, classes, forms, references, generated helpers, and targets are findable. | W040 |
| Open buffer continuity | The active source, dirty state, and open documents survive overlays and run/debug postures. | W050 |
| Source edit loop | Code Canvas remains visually dominant during edit, inspect, run, and debug. | W050 |
| Diagnostics | Problems are visible near source and traceable to OxVba diagnostics. | W060 |
| Source lens | Hover/detail appears where the user's eye already is and names its seam. | W060 |
| Navigation | Definition, references, symbols, and completion are discoverable without becoming a separate app. | W060 / W090 |
| Run target | The configured host/target is visible and not guessed. | W040 / W070 |
| Run timeline | Prepare, analyze, build, execute, and result are staged and replayable. | W070 |
| Immediate | Evaluations are tied to the running/paused project context. | W070 / W080 |
| Debug state | Paused line, stack, locals, watches, breakpoints, and step controls form one posture. | W080 |
| Command truth | Every visible command has action id, binding, availability, consequence, and disabled reason when needed. | W090 |
| Capability honesty | Terminal features and degradation are explicit without making compact mode the design ceiling. | W100 |
| Recovery/polish | Status, errors, empty states, and return paths are clear under pressure. | W110 |

## High-End Workspace Contract

Studio and First-class screens should look like a full working IDE, but
with a VBA-sized information model.

Required high-end signals:

- Project Spine is not just a file list. It shows module groups, active
  document, generated/helper distinction, reference/target groups, dirty
  state, and run/debug affordance when relevant.
- Code Canvas dominates the screen and keeps source geometry stable.
- Context Dock shows the thing the user is touching: diagnostic, symbol,
  run target, call stack, local value, watch, or selected reference.
- Activity Deck is not a generic log. It has mode, count, origin,
  severity, time/order, and actionability.
- Key Rail and status controls are dense but live. No visible key should
  be a silent lie.
- Overlay/dialog surfaces preserve the backing workspace and explain
  what will change.
- High-end width is used for useful state, not whitespace that makes the
  terminal proof look safer than the product should feel.

Non-goals:

- show every possible pane all the time;
- make Compact the baseline;
- copy a GUI toolbar into text;
- fill space with labels that do not come from project, source, command,
  run/debug, or terminal capability state.

## Required Workload Journeys

These journeys are the calibration set. The Audit Lab should be able to
review all of them in Studio and First-class. Compact is reviewed as
degradation, not as the emotional target.

### J1 - Loaded Pricing Edit Loop

Persona: `pricing_maintainer`

Pressure: maintain business-critical pricing code quickly without losing
semantic confidence.

Starting state:

- project loaded;
- active module `PriceFor.bas`;
- one warning or error from OxVba diagnostics;
- active source cursor on or near a meaningful identifier;
- Project Spine shows active module, related module, target, reference,
  and generated/helper distinction if present.

Expected visible continuity:

- Identity Rail shows project, target, dirty/clean state, cursor, and
  analysis/run posture.
- Code Canvas shows source, line numbers, gutter marker, nearest
  diagnostic, and optional source lens.
- Context Dock shows diagnostic or symbol detail, not a generic
  dashboard.
- Activity Deck shows Problems as a task surface with count and selected
  row.
- Key Rail exposes save, Command Lens, semantic lens, run, quick fix or
  references as live or honestly disabled commands.

Audit failure examples:

- source becomes a backdrop behind panels;
- diagnostic card does not name an OxVba seam;
- high-end screen wastes width while hiding useful project or problem
  state;
- Command Lens/run affordance is visible but cannot explain
  availability.

### J2 - Run Loop With Source Continuity

Persona: `pricing_maintainer`

Pressure: run the current project/target, understand progress, inspect
output, and return to source safely.

Starting state:

- same project/source context as J1;
- run target configured or unavailable with explicit reason;
- previous run state may exist.

Expected visible continuity:

- Identity Rail warms to run posture and shows target.
- Code Canvas remains visible with entry point or active source marker.
- Run Lane shows prepare, analyze, build, execute, result.
- Context Dock shows target/current step details.
- Activity Deck shows Run Timeline and Output as structured event rows.
- Key Rail shows stop/rerun/return/Immediate/Command Lens as appropriate.

Audit failure examples:

- run is just raw output in a bottom pane;
- source disappears during run;
- target is implied rather than explicit;
- stopped/failed state does not offer recovery or return path.

### J3 - Debug And Immediate Loop

Persona: `debug_responder`

Pressure: diagnose a paused macro failure under time pressure.

Starting state:

- project loaded;
- source visible at paused line;
- debug contract may be fixture/future until W080, but its seam status is
  explicit.

Expected visible continuity:

- Identity Rail shows Debug, paused location, and reason.
- Code Canvas highlights execution line and breakpoints without dimming
  source into a backdrop.
- Project Spine exposes breakpoint/watch grouping where useful.
- Context Dock becomes Debug Dock with call stack, locals, watches, or
  explicit unavailable seam rows.
- Activity Deck defaults to Immediate or Watch/Trace.
- Key Rail shows continue, step, step out, breakpoint, Immediate, and
  return.

Audit failure examples:

- debug looks like normal editing with a different side panel;
- locals/watches look real without a real or named future seam;
- Immediate is a raw console not tied to paused context;
- step controls are visible without context-specific action ids.

### J4 - Migration Review And Seam Honesty

Persona: `migration_reviewer`

Pressure: review inherited VBA and generated helpers without mistaking UI
copy for semantic truth.

Starting state:

- project loaded from current adapter or fixture;
- one or more unavailable/future seams;
- generated or helper code visible in project or diagnostics.

Expected visible continuity:

- Project Spine distinguishes user modules, references, generated
  helpers, and unavailable project facts.
- Code Canvas can show source even when semantic state is partial.
- Context Dock surfaces unavailable seams explicitly with owner/workset.
- Activity Deck records diagnostics or references without pretending to
  have unsupported truth.
- Command Lens keeps future actions visible only when disabled reasons
  are concrete.

Audit failure examples:

- UI copy implies semantic knowledge that OxVba does not provide;
- generated helper rows pollute the normal module model without
  provenance;
- unavailable data is hidden to keep the mockup pretty;
- future action ids are not tied to downstream worksets.

## New Audit Gates

The Audit Lab should add or emulate these gates when reviewing high-end
mockups and FrankenTui screens.

| Gate id | Mode | Question |
| --- | --- | --- |
| `functional.oxvba_workload_fidelity` | Functional | Does every visible project/VBA claim map to a real, future, unavailable, or not-required OxVba seam? |
| `functional.session_continuity` | Functional | Does project/source/dirty/diagnostic/run/debug state remain coherent while moving through overlays and postures? |
| `functional.command_surface_truth` | Functional | Does every command-like row/key expose action id, binding, availability, consequence, and disabled reason when needed? |
| `aesthetic.desktop_workspace_density` | Aesthetic | Does Studio/First-class use space for real IDE state rather than filler or timid empty layout? |
| `aesthetic.layered_terminal_chrome` | Aesthetic | Do rails, tabs/work strips, overlays, docks, decks, and status rows feel modern and terminal-native without becoming boxed forms? |
| `aesthetic.oxvba_specificity` | Aesthetic | Does the screen feel like a VBA/OxVba IDE rather than a generic code dashboard? |

For now these may be documented manual checks. Once W041 is extended,
they should become criteria in `oxide-uxlab --audit --evaluate`.

## Scenario Coverage Matrix

| Journey | Existing Fire Horse scenario | Needed calibration emphasis |
| --- | --- | --- |
| J1 Loaded Pricing Edit Loop | `firehorse-editing-lens-standard`, `firehorse-real-editing` | Add open-buffer/dirty/diagnostic/source-lens continuity and generated-helper honesty. |
| J2 Run Loop With Source Continuity | `firehorse-run-lane-standard`, `firehorse-command-lens-standard` | Make target, staged events, output, Immediate, and return-to-source visible in one flow. |
| J3 Debug And Immediate Loop | `firehorse-debug-cockpit-standard` | Treat debug as a distinct posture with explicit W080 seam status where future. |
| J4 Migration Review And Seam Honesty | `firehorse-real-editing`, `firehorse-editing-lens-standard` | Show unavailable/future seams and generated provenance as first-class review facts. |

Launchpad, Console Fit, and Compact Focus remain important, but they are
supporting journeys. They should not consume the high-end emotional
target.

## FrankenTui Proof Requirements

The next FrankenTui-backed mockup pass should produce at least these
reviewable screens:

1. J1 Studio and First-class crowded edit loop.
2. J2 Studio run loop with structured timeline and source continuity.
3. J3 Studio debug/immediate posture with explicit future seam labels.
4. J4 First-class migration review with unavailable seam and generated
   provenance labels.
5. One Compact degradation capture proving the same work can continue
   without making Compact the design baseline.

Each capture must include:

- plain text capture;
- ANSI capture;
- selected persona and journey id;
- reference mockup path where applicable;
- projection/seam mapping rows;
- audit scorecard or manual review note.

## Agent Review Instructions

When a coding agent uses the Audit Lab for design work:

1. Pick one workload journey, not just one screen.
2. Run `--brief --json` for the closest Fire Horse scenario.
3. Preserve the named OxVba seams and action ids.
4. Increase density only with project/source/semantic/run/debug/status
   facts.
5. Run `--once --json`, `--evaluate functional,aesthetic --json`, and a
   high-end capture.
6. Treat `aesthetic.emotional_fit` and
   `aesthetic.desktop_workspace_density` as manual/cited review gates
   until automated criteria exist.
7. File follow-up beads for missing real seams rather than hiding them.

## Reviewer Checklist

Use this checklist before approving a high-end Fire Horse or FrankenTui
screen for downstream implementation:

- The screen is recognizably OxIde for VBA/OxVba, not a generic code UI.
- The source remains the visual center unless the user deliberately
  opened a focused overlay or max deck.
- High-end width is carrying real state: project, source, diagnostics,
  command, run/debug, activity, target, or terminal capability.
- Every VBA/project fact has seam status.
- Every visible command-like affordance has an action id and honest
  availability.
- Overlays/dialogs preserve backing state and make close/commit/cancel
  obvious.
- The same work has an honest compact degradation path.
- The screen can be reviewed from terminal-cell text and ANSI capture,
  not only from a polished bitmap.

