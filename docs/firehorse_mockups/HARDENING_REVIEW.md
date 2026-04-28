# Fire Horse Mockup Hardening Review

Status: refined proposal; W039 terminal proof completed
Type: doctrine
Date: 2026-04-27

This review hardens the Fire Horse mockups before converting them into
UX-lab or FrankenTui implementation targets. It does not replace
`docs/DESIGN_TUI_2026_FIRE_HORSE.md`; it records what survived the
first visual pass and what must be checked in real terminal cells.

## Verdict

The style direction is worth carrying forward.

The strongest moves are:

- rails over boxed panes;
- source-centered semantic lenses;
- warm Ember only for activity, dirty state, run/debug, and selection;
- Azure for command and semantic intelligence;
- Project Spine as an instrument rail rather than a file-tree clone;
- Command Lens with preview and disabled-command explanations;
- Run Lane as a staged workflow;
- Debug Cockpit as a distinct posture without losing source continuity;
- Console Fit as a polished product surface rather than a diagnostics dump.

The main risk is not visual taste. The main risk is terminal fidelity:
CSS gives us gradients, shadows, fractional spacing, and smooth sizing
that the terminal renderer will approximate with cells. The next proof
must translate the style into cell-safe rules.

## Canonical Artifacts

Source:

- `docs/firehorse_mockups/index.html`
- `docs/firehorse_mockups/README.md`
- `docs/firehorse_mockups/HARDENING_REVIEW.md`
- `docs/firehorse_mockups/UX_RESET.md`
- `docs/firehorse_mockups/UX_PROJECTION_CONTRACT.md`

Approved overview export:

- `docs/firehorse_mockups/refined_complete.png`

Approved per-screen exports:

- `docs/firehorse_mockups/refined_01_launchpad.png`
- `docs/firehorse_mockups/refined_02_editing_lens.png`
- `docs/firehorse_mockups/refined_03_command_lens.png`
- `docs/firehorse_mockups/refined_04_run_lane.png`
- `docs/firehorse_mockups/refined_05_debug_cockpit.png`
- `docs/firehorse_mockups/refined_06_console_fit.png`
- `docs/firehorse_mockups/refined_07_compact_focus_mode.png`

Non-canonical files may also exist in this directory from earlier
capture attempts. They are retained because repo-local rules forbid file
deletion without an exact user-approved delete command. Treat only the
files listed above as approved.

## North-Star Screenshot

Primary north-star:

- `refined_02_editing_lens.png`

Reason: it shows the product's core identity in one frame: Project
Spine, Code Canvas, source-adjacent semantic lens, Context Dock,
Activity Deck, and Key Rail. This is the screen future work should
compare against first.

Secondary north-star:

- `refined_05_debug_cockpit.png`

Reason: it proves the same visual language can become hotter and more
stateful for debug work without turning into a separate app.

## Style Rules To Preserve

### 1. Rails over heavy boxes

Use full boxes for overlays, focused modal workflows, expanded decks,
Console Fit, and dangerous confirmations. Use rails, separators,
selection bars, and color temperature for ordinary structure.

### 2. Source is the visual center

The Code Canvas should get the calmest background and the most stable
geometry. Semantic, diagnostic, run, and debug state should first touch
the gutter or source-adjacent lens before expanding into a dock or deck.

### 3. Ember is heat, not theme wash

Ember appears when work is active or risky:

- Launchpad accent;
- dirty state;
- active selection edge;
- Run Lane active step;
- Debug paused/hit state;
- important warnings when Gold is insufficient.

Do not make every border or heading orange.

### 4. Azure means command and meaning

Azure is the color of semantic intelligence and command affordance:

- function names and semantic lenses;
- Command Lens outline and active action;
- key affordance highlights;
- goto/refs/help actions.

### 5. Mint and Rose stay literal

Mint means pass, ready, healthy, or clean. Rose means error, stop, or
runtime failure. Do not use them decoratively.

### 6. Dock content is contextual

The Context Dock should not become a generic dashboard. It shows the
thing the user is currently touching: diagnostics, symbol details, run
target, call stack, locals, watches, or selected reference details.

### 7. The Activity Deck is a timeline-capable task surface

Problems, Output, Run Timeline, Immediate, References, and Watch/Trace
belong in the deck. The deck rail should remain meaningful even when the
deck is collapsed.

### 8. Command Lens is a product surface

Command Lens rows should show label, binding, availability, and preview.
Disabled commands are visible when useful and explain why they cannot
run.

### 9. Console Fit is calm and practical

The capability surface should be visually polished, but it should avoid
raw terminal protocol trivia unless the user asks for details.

### 10. Compact Focus Mode is intentional

Small terminals do not get a cramped imitation of the full shell. They
get Identity Rail, Code Canvas, Activity Rail, and Key Rail; Project,
Context, and Activity open as temporary docks.

## Refinements Already Applied

- Launchpad in-app copy changed from a marketing-style sentence to a
  product/state mark: `OxIde / VBA work.` The useful Recent, Start, and
  Terminal rows now carry the action value.
- README now names the approved `refined_*.png` set so capture attempts do
  not become accidental truth.

## Terminal Fidelity Checklist

The next implementation proof should answer these questions in actual
terminal cells, not CSS.

### Geometry

- Does `refined_02_editing_lens` translate to a 120x34 standard terminal
  without wrapping the Key Rail?
- Does the Project Spine remain useful at 24, 18, 14, and 10 columns?
- Does the source lens fit within the Code Canvas at 120 columns?
- Does Context Dock content degrade to a deck or overlay below 120
  columns?
- Does Compact Focus Mode work at 92x30?

### Color

- Do Graphite Ember colors survive Windows Terminal truecolor?
- Is the same screen legible in 16-color fallback?
- Can errors, warnings, selections, active run steps, and disabled
  commands be distinguished without color?
- Does Paper Ember have enough contrast for Console Fit?

### Glyphs

- Are all approved markers available as ASCII fallbacks?
- Do gutter markers align in common fonts?
- Do thin separators hold up in Windows Terminal and legacy hosts?
- Does the source lens remain readable without box shadow or gradients?

### Motion

- Can Run Lane and diagnostics change state without flaky WTD goldens?
- Is reduced-motion mode simply a static marker swap?
- Does animation stop while typing source?

### Interaction

- Is every visible key in the Key Rail a live action?
- Does Command Lens preview update without stealing source edits?
- Does `Esc` cascade lens, overlay, deck, and scene state predictably?
- Does mouse focus match keyboard focus without adding mouse-only flows?

## Implementation Mapping

| Mockup concept | OxIde concept to refine | Notes |
| --- | --- | --- |
| Identity Rail | Top bar | Reduce bordered block feel; keep display-only. |
| Project Spine | Explorer | Add full/slim/peek states and run/debug badges. |
| Code Canvas | Editor Area | Add gutter markers, source lenses, and stronger syntax roles. |
| Context Dock | Inspector | Make hidden/shelf/focused states explicit. |
| Activity Deck | Lower Surface | Add rail/compact/expanded modes and Run Timeline. |
| Key Rail | Status Line | Preserve one-row no-wrap live-affordance contract. |
| Command Lens | Command Palette | Add preview, disabled reasons, aliases, and profile-aware bindings. |
| Run Lane | BuildRun lower mode | Replace raw output-first presentation with staged event stream. |
| Debug Cockpit | Debug workset surfaces | Keep source continuity while switching Context Dock and Activity Deck roles. |
| Console Fit | W100 terminal capability | Make capability testing a product surface. |

## W039 Terminal Proof Handoff

The large proof chunk is
`docs/worksets/W039_firehorse_terminal_ux_proof.md`.

W039 converted the refined mockups into terminal-cell UX-lab scenarios,
projection contracts, command/action ids, OxVba seam-shaped fixture data,
WTD journeys, and committed goldens. It remains a proof layer: it does
not replace the shipping `ox-ide` renderer, mutate projects, implement
real semantic features, run/debug real code, or perform real terminal
probing.

Authoritative W039 outputs:

- `docs/firehorse_mockups/UX_RESET.md`
- `docs/firehorse_mockups/UX_PROJECTION_CONTRACT.md`
- `src/shell/uxlab/firehorse/projection.rs`
- `src/shell/uxlab/firehorse/fixtures.rs`
- `src/shell/uxlab/firehorse/renderer.rs`
- `src/shell/uxlab/firehorse/adapter.rs`
- `src/bin/oxide-uxlab.rs`
- `tests/wtd/firehorse.rs`
- `tests/wtd/goldens/W039/`

The first bead was a UX reset: previous OxIde front-end/rendering/UI work
is superseded product direction, not something to maintain in parallel.
Any useful inner framework must be reviewed and explicitly reused for
the Fire Horse projection/renderer path. The reset doctrine is recorded
in `docs/firehorse_mockups/UX_RESET.md`.

The typed projection, ownership, seam, and action-id contract is recorded
in `docs/firehorse_mockups/UX_PROJECTION_CONTRACT.md`.

Downstream ownership remains explicit:

- W040 owns real Project Spine/project-session behavior, MRU, session
  restore, and project mutations.
- W050 owns real document/editor lifecycle and Code Canvas plumbing.
- W060 owns real diagnostics, hover/source lens, references,
  completion, signature help, and rename.
- W070 owns real Run Lane execution events and Immediate evaluation.
- W080 owns real Debug Cockpit contracts: breakpoints, suspend, stack,
  locals, watches, stepping, and halt surfacing.
- W090 owns the real action registry, command dispatch, chords,
  mnemonic menus, and keymap profiles.
- W100 owns live terminal capability probing and fallback rendering.
- W110 owns motion, accessibility, recovery, and final polish.

Open seam gaps are deliberately visible in the proof. The real Editing
adapter renders unavailable rows for seams such as hover/source lens,
references, action registry dispatch, and deeper context cards when the
current `ShellState` cannot provide them. Later worksets should replace
those unavailable projections with real seam data, not invent OxIde-owned
VBA meaning.

## UX-Lab Targets

W039 created UX-lab scenarios in this order:

1. `firehorse-editing-lens-standard` from `refined_02_editing_lens.png`.
2. `firehorse-command-lens-standard` from `refined_03_command_lens.png`.
3. `firehorse-run-lane-standard` from `refined_04_run_lane.png`.
4. `firehorse-debug-cockpit-standard` from `refined_05_debug_cockpit.png`.
5. `firehorse-launchpad-standard` from `refined_01_launchpad.png`.
6. `firehorse-console-fit-light` from `refined_06_console_fit.png`.
7. `firehorse-focus-compact` from `refined_07_compact_focus_mode.png`.

W039 also added `firehorse-real-editing`, a read-only adapter over the
current thin-slice `ShellState`. That adapter is the handoff bridge from
fixture proof to later product work: it populates the same projection
surfaces from real shell state where possible, and marks missing seam
data explicitly where not.

## Open Review Notes

- The HTML mockups intentionally use shadows and large-scale spacing to
  sell the visual direction. FrankenTui should translate those into
  cell-safe contrast, rails, and active-row grounding, not attempt fake
  shadow art.
- The Launchpad still has an expressive first impression, but it should
  remain action-led. Recent work, Start, and Terminal rows are the real
  content.
- Console Fit proves the light theme direction, but Paper Ember should
  get a dedicated contrast pass before being treated as shippable.
- Compact Focus Mode needs the most interaction design. Its visual
  target is clear; the temporary dock choreography still needs a
  separate journey.

## Read-Through Evidence

- [x] Canonical image set named.
- [x] Non-canonical capture files explicitly demoted.
- [x] North-star screenshot chosen.
- [x] Style rules extracted.
- [x] Terminal fidelity risks listed.
- [x] Implementation mapping written.
- [x] UX-lab scenario order proposed.
- [x] UX reset doctrine linked.
- [x] Projection contract linked.
- [x] W039 terminal proof outputs linked.
- [x] Downstream workset owners named.
- [x] Open seam gaps listed rather than hidden.
