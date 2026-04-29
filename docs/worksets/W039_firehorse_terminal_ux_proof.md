# Workset W039 — Fire Horse Terminal UX Proof

## Ambition

The Fire Horse visual direction becomes a terminal-native, state-mapped
UX proof before it becomes product implementation.

A contributor can run a Fire Horse suite in the UX lab, inspect
Launchpad, Editing Lens, Command Lens, Run Lane, Debug Cockpit,
Console Fit, and Compact Focus Mode as real terminal-cell scenes, and
read exactly how every visible surface maps to OxIde state ownership and
the OxVba seam.

At the end of W039, the team has a large, concrete UX chunk with low
risk: terminal-rendered mockups, command/action contracts, projection
types, fixture data shaped like OxVba responses, WTD goldens, and a
clear handoff into W040 / W060 / W070 / W080 / W090 / W100. No shipping
project behavior changes in this workset.

W039 also makes one strategic reset explicit: **all previous OxIde
front-end/rendering/UI work is superseded for product direction**. The
Fire Horse terminal UX is a complete replacement of the old front-end
shape, not a mixed continuation. Useful inner framework pieces may be
reused only after review, and only when they serve the new projection
and terminal-cell renderer.

## Dependencies

- **W038 Phase 1** — minimum UX-lab substrate:
  - lab scenario registry and viewport contract;
  - non-interactive `oxide-uxlab --once` runner;
  - WTD helper for lab scenario capture.
  W039-B02 and later depend on this initial W038 phase. W039-B00 and
  W039-B01 may run before it because they are doctrine/contract beads.
  Later W038 phases follow W039 and should use W039 captures as real
  review cases.
- **W037** — WTD harness and goldens.
- **W035 / Fire Horse doctrine**:
  - `docs/DESIGN_TUI_2026_FIRE_HORSE.md`
  - `docs/firehorse_mockups/HARDENING_REVIEW.md`
  - `docs/firehorse_mockups/UX_RESET.md`
  - `docs/firehorse_mockups/UX_PROJECTION_CONTRACT.md`
  - `docs/firehorse_mockups/refined_*.png`
- **ARCHITECTURE.md** — ownership boundaries:
  - OxVba owns VBA/project meaning.
  - OxIde owns workflow, presentation, editor state, command policy,
    and when/how OxVba results are surfaced.

Soft dependencies:

- **W090** action registry direction. W039 may define action ids before
  W090 implements the registry, but it must not build a competing
  command system.
- **W100** capability direction. W039 may mock capability results, but
  actual terminal probing remains W100.

## Design

### Scope shape

W039 is a **terminal UX proof**, not feature implementation.

It owns:

- Retirement doctrine for previous front-end/rendering/UI work.
- Fire Horse terminal-cell scene renderings.
- A typed projection model for the visible surfaces.
- A command/action matrix for each scene.
- Mock fixture data shaped like future/current OxVba seam responses.
- A Fire Horse lab scenario provider registered through the W038 Phase
  1 scenario registry.
- A read-only adapter spike from current `ShellState` into the
  projection model for one scene.
- WTD captures/goldens proving the terminal layouts.

It does not own:

- maintenance or incremental polishing of legacy front-end/rendering
  paths;
- real project-management behavior (W040);
- real editor/document lifecycle changes (W050);
- real semantic features beyond existing seams (W060);
- real run/immediate behavior (W070);
- real debug contracts (W080);
- real command registry implementation (W090);
- real capability probing/degradation (W100).
- the generic UX-lab runner, viewport contract, WTD capture helper, VT
  replay/diff/bless commands, or interactive lab browser (W038).

### Low-risk rule

Every W039 output is either:

1. doctrine,
2. a UX-lab-only renderer,
3. a fixture-backed scene,
4. a read-only projection adapter,
5. a WTD golden.

No W039 bead mutates `.basproj` files, saves user buffers, starts real
runs, invokes real debug stepping, or changes default `ox-ide` behavior.

The only allowed integration with current runtime state is a one-way
projection from existing state into Fire Horse surface data. If a seam
is missing, W039 records the dependency; it does not invent OxIde-owned
VBA meaning.

### Use of W038 infrastructure

W039 consumes W038 Phase 1 and does not create a second mockup runner.

The Fire Horse suite registers as a W038 lab scenario provider:

```text
suite: firehorse
scenarios:
  firehorse-launchpad-standard
  firehorse-editing-lens-standard
  firehorse-command-lens-standard
  firehorse-run-lane-standard
  firehorse-debug-cockpit-standard
  firehorse-console-fit-light
  firehorse-focus-compact
```

The non-interactive verification path is:

```text
cargo run --release --bin oxide-uxlab -- \
  --suite firehorse \
  --scenario firehorse-editing-lens-standard \
  --viewport standard \
  --once
```

WTD journeys for W039 use the W038 lab helper to launch that command,
capture the final frame, and assert visible contracts. The W039 renderer
and fixtures may live in lab-only modules, but scenario selection,
viewport naming, process launch, and capture plumbing stay owned by
W038.

Corrective note from W038-B15: the original W039 renderer is now the
plain text **contract renderer**. It proves scenario identity,
projection coverage, and named surface contracts, but it is not the
high-fidelity UX mockup surface. Real terminal UX review uses the
W038-owned mode:

```text
cargo run --release --bin oxide-uxlab -- \
  --suite firehorse \
  --scenario firehorse-editing-lens-standard \
  --viewport studio \
  --once \
  --mockup

cargo run --release --bin oxide-uxlab -- \
  --suite firehorse \
  --scenario firehorse-editing-lens-standard \
  --viewport studio \
  --once \
  --mockup \
  --ansi
```

Downstream UX review should cite the `--mockup` artifacts for layout,
density, and emotional fit against the colourful Fire Horse mockups,
and cite the original W039 text goldens only for contract verification.

In tests, W039 journeys should declare a `LabScenarioJourney` from
`tests/support/mod.rs`, open it with `Harness::open_lab_once`, and
capture with `capture_lab_once_text` / `capture_lab_once_vt`. Fire Horse
goldens then live under `tests/wtd/goldens/W039/` with the scenario id as
the capture name.

### UX reset rule

From W039 onward, agents should treat prior OxIde front-end UI and
rendering work as historical unless a W039 review explicitly blesses a
piece as reusable inner framework.

The authoritative reset note is
`docs/firehorse_mockups/UX_RESET.md`.

This applies to:

- old visual mockup assets;
- old shell/frame terminology that conflicts with Fire Horse;
- old renderer paths whose job is to preserve the previous pane design;
- old docs that position the earlier TUI as current implementation
  truth.

The reset does not require immediate deletion. Repo-local rules forbid
deleting any file or directory unless the user gives the exact delete
command in-session. Instead, W039 starts by classifying old surfaces,
removing their authority from forward plans, and documenting what, if
anything, may be reused as internal framework.

Reuse criteria:

- keeps OxVba/OxIde ownership boundaries intact;
- is rendering-infrastructure quality, not legacy visual design;
- does not preserve old pane/frame UX by inertia;
- can serve the `FireHorseProjection -> FrankenTui render` path;
- has tests or can be covered before reuse.

### Scenario suite

The Fire Horse suite names seven first-class UX-lab scenarios:

| Scenario id | Reference image | Purpose |
| --- | --- | --- |
| `firehorse-launchpad-standard` | `refined_01_launchpad.png` | Empty/recent/start/capability posture. |
| `firehorse-editing-lens-standard` | `refined_02_editing_lens.png` | Primary north-star; full surface grammar. |
| `firehorse-command-lens-standard` | `refined_03_command_lens.png` | Overlay, command preview, disabled reasons. |
| `firehorse-run-lane-standard` | `refined_04_run_lane.png` | Staged run timeline and activity deck. |
| `firehorse-debug-cockpit-standard` | `refined_05_debug_cockpit.png` | Secondary north-star; debug posture. |
| `firehorse-console-fit-light` | `refined_06_console_fit.png` | Light theme and capability UX. |
| `firehorse-focus-compact` | `refined_07_compact_focus_mode.png` | Small terminal source-first mode. |

The first scenario implemented should be
`firehorse-editing-lens-standard`; it exercises the most reusable shell
grammar and gives later beads a stable reference.

### Projection model

W039 introduces a UI projection layer for the proof. It should be
defined in a UX-lab-only module first, then promoted only if later
worksets adopt it.

Sketch:

```rust
pub struct FireHorseProjection {
    pub identity: IdentityRailProjection,
    pub project_spine: Option<ProjectSpineProjection>,
    pub code_canvas: CodeCanvasProjection,
    pub context_dock: Option<ContextDockProjection>,
    pub activity_deck: ActivityDeckProjection,
    pub key_rail: KeyRailProjection,
    pub overlay: Option<OverlayProjection>,
    pub theme: ThemeProjection,
    pub terminal_fit: Option<TerminalFitProjection>,
}
```

Surface projections:

| Surface | Projection owns | Must not own |
| --- | --- | --- |
| Identity Rail | Project label, scene posture, dirty/run/debug summary, cursor position when relevant. | Project semantics or focus routing. |
| Project Spine | User-facing project tree rows, active/open/dirty/run/debug badges, slim/full/peek shape. | Canonical `.basproj` model. |
| Code Canvas | Visible source lines, gutter markers, source lens, selected spans, execution line. | Text editing semantics or OxVba queries. |
| Context Dock | Current contextual cards: diagnostics, symbol, run status, call stack, locals, watches. | Generic dashboard state. |
| Activity Deck | Rail/compact/expanded content for Problems, Output, Run Timeline, Immediate, References, Watch/Trace. | Raw unstructured dump as the only source of truth. |
| Key Rail | Current live key hints and chord state. | Action dispatch itself. |
| Overlay | Command Lens / picker projection with filter, rows, preview, disabled reasons. | Independent command registry. |
| Terminal Fit | Capability result rows and recommendations. | Real probing logic. |

Projection is intentionally one-way:

```text
Shell / fixture / OxVba-shaped seam data
        -> FireHorseProjection
        -> FrankenTui render
        -> WTD capture / golden
```

There is no reverse path from projection into project or semantic truth.

### Internal state mapping

The proof should map mockup concepts onto current or planned OxIde
owners:

| Fire Horse concept | OxIde owner | Later implementation workset |
| --- | --- | --- |
| Identity Rail | `OxIdeShell` / runtime shell state | W039 proof, W110 polish |
| Project Spine | `ProjectSession` presentation + Explorer state | W040 |
| Code Canvas | `EditorSurface` + `DocumentSession` + semantic projection | W050 / W060 |
| Context Dock | Inspector mode state | W060 / W070 / W080 |
| Activity Deck | Lower Surface mode state | W070 / W080 |
| Key Rail | current status-line hint, then `ActionRegistry` | W090 |
| Command Lens | palette overlay, then `ActionRegistry` view | W090 |
| Run Lane | execution event projection | W070 |
| Debug Cockpit | debug scene/projection | W080 |
| Console Fit | terminal capability state | W100 |
| Compact Focus Mode | layout policy / width adaptation | W100 / W110 |

### OxVba seam mapping

W039 must keep every piece of VBA/project meaning tied to an OxVba-owned
or OxVba-shaped source.

| UX data | Seam source |
| --- | --- |
| Project modules/forms/classes/references/targets | `ProjectSession` over OxVba project truth. |
| Document identity and active source text | `DocumentSession` mapped to OxVba `DocumentId`. |
| Diagnostics | `HostWorkspaceSession::diagnostics` or current equivalent. |
| Hover/source lens | `HostWorkspaceSession::hover`. |
| Definition location | `HostWorkspaceSession::goto_definition` or current equivalent. |
| References | `HostWorkspaceSession::references`. |
| Symbols | document/workspace symbol APIs. |
| Completion rows | completion API. |
| Run timeline | current `WebHostEvent` stream, later typed run event seam. |
| Immediate results | current/future evaluate API from OxVba runtime. |
| Call stack / locals / watches / stepping | W080 audit of OxVba debug contract. |
| Generated-code tagging | OxVba provenance surfaced through OxIde formatting. |

Fixture data in W039 should be named after these seam concepts, not
after arbitrary UI strings. Example:

```rust
pub struct MockDiagnostic {
    pub document_id: String,
    pub range: SourceRange,
    pub severity: Severity,
    pub code: String,
    pub message: String,
    pub provenance: DiagnosticProvenance,
}
```

This keeps the mockup honest: when W060 replaces fixture diagnostics
with real diagnostics, the projection layer changes data source, not
meaning.

### Command/action matrix

W039 defines visible commands as stable action ids, even before W090
implements `ActionRegistry`.

Initial ids:

| Visible command | Action id | Default key | Scene(s) |
| --- | --- | --- | --- |
| Open Project | `project.open` | `Ctrl+O` | Launchpad, Command Lens |
| Create Project | `project.create` | `Ctrl+N` | Launchpad, Command Lens |
| Console Fit | `app.console_fit` | `F10` | Global |
| Command Lens | `command.lens.open` | `F6` | Global |
| Save | `editor.save` | `Ctrl+S` | Editing |
| Semantic Lens | `semantic.hover` | `F1` | Editing |
| Go Definition | `semantic.goto_definition` | `F12` | Editing |
| References | `semantic.references` | `Shift+F12` | Editing |
| Run Project | `run.start` | `F5` | Editing, Command Lens |
| Stop Run | `run.stop` | `F8` | Run |
| Return To Edit | `scene.return_edit` | `Esc` | Run, Debug |
| Immediate | `immediate.focus` | `Ctrl+G` | Run, Debug |
| Toggle Breakpoint | `debug.breakpoint.toggle` | `F9` | Debug / Editing |
| Continue | `debug.continue` | `F5` | Debug |
| Step | `debug.step` | `F8` | Debug |
| Step Out | `debug.step_out` | `Shift+F8` | Debug |
| Focus Project | `focus.project` | `Alt+1` | Editing / Focus |
| Focus Code | `focus.code` | `Alt+2` | Editing |
| Focus Context | `focus.context` | `Alt+3` | Editing / Focus |
| Focus Activity | `focus.activity` | `Alt+4` | Editing / Focus |

The W039 renderer consumes these ids only for display and test
contracts. Dispatch remains owned by existing model code until W090.

### Terminal proof levels

Each scenario should be rendered through W038 viewport classes at three
levels:

1. **Standard truecolor** — 120x34 or nearest existing WTD size.
2. **Compact truecolor** — 92x30 for Focus Mode and collapse behavior.
3. **Fallback discipline check** — no advanced glyph/color assumption
   is required to identify errors, warnings, active selection, disabled
   command, run active step, or debug paused line.

W039 does not need full W100 capability probing. It needs proof that the
visual language has cell-safe fallbacks.

## Beads

### W039-B00 — UX reset and legacy front-end retirement doctrine

**Doctrine.**

- **Goal.** A contributor opens
  `docs/firehorse_mockups/UX_RESET.md` and understands that previous
  OxIde front-end/rendering/UI work is superseded by the Fire Horse UX
  proof. The doc identifies legacy UI surfaces as historical or
  candidate inner framework, and states that no agent should maintain
  mixed old/new UI direction.
- **Design.** New doctrine file under `docs/firehorse_mockups/`.
  Sections:
  - product decision: Fire Horse is the current UX direction;
  - legacy surfaces inventory: old mockup app, old visual spec language,
    existing renderer paths, old captures/goldens where applicable;
  - classification: historical UI, non-authoritative docs/assets,
    reusable framework candidates, deletion candidates requiring exact
    user command;
  - agent rule: do not extend old front-end UI; do not cite old mockups
    as current design authority; reuse inner framework only by explicit
    review;
  - transition rule: W039 terminal proof is the next authoritative UX
    artifact.
- **Tests.** Doctrine bead. No unit tests.
- **Evidence.** Read-through checklist:
  - legacy front-end/rendering surfaces are named or scoped;
  - no deletion is performed;
  - useful inner framework reuse criteria are explicit;
  - `HARDENING_REVIEW.md` and W039 both link to the reset doc;
  - downstream agents can tell which UX artifacts are authoritative.
- **Closure.**
  - [ ] `UX_RESET.md` exists.
  - [ ] It states Fire Horse is a complete front-end UX replacement.
  - [ ] It distinguishes historical UI from reusable framework.
  - [ ] It records that deletion requires an exact user command.
  - [ ] Hardening review links it.

### W039-B01 — Fire Horse projection contract

**Doctrine.**

- **Goal.** A contributor opens
  `docs/firehorse_mockups/UX_PROJECTION_CONTRACT.md` and sees the
  typed projection model, surface ownership rules, state mapping,
  OxVba seam mapping, and action id matrix that W039 scenarios use.
- **Design.** New doctrine file extracted from this workset's Design
  section. It should be concrete enough that a renderer can be written
  without re-reading the HTML mockup. Include Rust-like type sketches,
  fixture naming rules, and a "do not own VBA meaning" rule.
- **Tests.** Doctrine bead. No unit tests.
- **Evidence.** Read-through checklist:
  - every visible Fire Horse surface maps to an OxIde owner;
  - every VBA/project meaning maps to an OxVba seam or named future
    seam;
  - every visible key in the refined mockups has an action id or an
    explicit "display only" reason.
- **Closure.**
  - [ ] Contract doc exists.
  - [ ] `HARDENING_REVIEW.md` links to it.
  - [ ] No command row remains unmapped.

### W039-B02 — Fire Horse fixture suite

**Infrastructure.**

- **Goal.** The UX lab can load named Fire Horse fixture projections
  for the seven scenario ids without touching real projects or real
  OxVba services.
- **Design.** Add a Fire Horse lab scenario provider using the W038
  Phase 1 registry. The provider owns the seven descriptors under
  `suite = "firehorse"` and points each descriptor at a fixture builder.
  Builders live in lab-only Fire Horse modules, for example
  `src/shell/uxlab/firehorse/fixtures.rs`, and return
  `FireHorseProjection` values for Launchpad, EditingLens, CommandLens,
  RunLane, DebugCockpit, ConsoleFit, and FocusCompact.
- **Tests.**
  - Unit contract: every scenario id resolves to a projection.
  - Unit contract: each projection has non-empty Identity Rail,
    Activity Deck, and Key Rail.
  - Unit contract: fixture diagnostics, symbols, run events, and
    debug rows use OxVba-shaped fixture structs, not ad hoc strings as
    their only source.
- **Evidence.** `oxide-uxlab --suite firehorse --list` or the W038
  registry test output shows all seven Fire Horse scenarios with their
  viewport classes.
- **Closure.**
  - [ ] Seven fixture builders exist.
  - [ ] Scenario ids match the hardening review.
  - [ ] Fire Horse provider registers through the W038 lab registry.
  - [ ] Fixture structs are seam-shaped.

### W039-B03 — Terminal renderer for the north-star Editing Lens

**Feature.**

- **Goal.** Running the UX lab scenario
  `firehorse-editing-lens-standard` renders a terminal-cell version of
  `refined_02_editing_lens.png`: Identity Rail, Project Spine, Code
  Canvas with source lens, Context Dock, Activity Deck, and Key Rail
  are all visible and legible at the standard WTD size.
- **Design.** Implement the Fire Horse renderer for the full surface
  set. The original renderer remains the text-contract renderer for
  WTD goldens. W038-B15 adds the real FrankenTui mockup mode for UX
  review: `oxide-uxlab --once --mockup` for plain cell text and
  `--mockup --ansi` for a styled terminal stream. Use FrankenTui
  primitives; no CSS-like shadows. Source lens becomes a bordered/railed
  cell-safe block anchored under the source span. Key Rail is one row
  and no-wrap. Both renderer modes are reached through the W038
  `oxide-uxlab --once` path, not a custom Fire Horse runner.
- **Tests.**
  - Unit contract: projection renders required named regions.
  - Unit contract: Key Rail text fits the configured standard width or
    applies the documented truncation policy.
  - `wtd` journey:
    `tests/wtd/journey_firehorse_editing_lens.rs` uses the W038 lab
    helper to launch `oxide-uxlab --suite firehorse --scenario
    firehorse-editing-lens-standard --viewport standard --once` and
    asserts the visible contracts: `Project Spine`, `Code Canvas`,
    `PriceFor`, `Context Dock`, `Activity: Problems`, `F6 Command Lens`.
- **Evidence.**
  - WTD golden committed for standard truecolor.
  - Five-minute pass compares the terminal capture against
    `refined_02_editing_lens.png` using the hardening review questions.
- **Closure.**
  - [ ] Scenario renders in UX lab.
  - [ ] WTD journey green.
  - [ ] Capture reads as the north-star scene, not the old pane frame.

### W039-B04 — Command Lens terminal proof

**Feature.**

- **Goal.** `firehorse-command-lens-standard` renders a modal Command
  Lens with filter text, action rows, bindings, disabled reason, and
  preview. The backing shell is visibly present but inactive.
- **Design.** Add `OverlayProjection::CommandLens`. Rows carry
  `action_id`, label, binding, enabled/disabled state, and preview
  model. Renderer shows disabled reasons and preview consequences.
  No action dispatch beyond fixture row selection is required. The
  scenario is selected and captured through W038's lab provider path.
- **Tests.**
  - Unit contract: disabled command rows must expose a reason.
  - Unit contract: every row has an action id.
  - `wtd` journey:
    `tests/wtd/journey_firehorse_command_lens.rs` uses the W038 lab
    helper and asserts `Run Project`, `Stop Run`, `no active run`, and
    `Enter run` are visible.
- **Evidence.** Five-minute pass: compare to `refined_03_command_lens.png`;
  verify the overlay is opaque and the Key Rail is overlay-specific.
- **Closure.**
  - [ ] Command Lens projection rendered.
  - [ ] Disabled reason visible.
  - [ ] WTD journey green.

### W039-B05 — Run Lane and Debug Cockpit terminal proof

**Feature.**

- **Goal.** The UX lab renders `firehorse-run-lane-standard` and
  `firehorse-debug-cockpit-standard` from seam-shaped run/debug
  fixture data. Run Lane shows staged event state; Debug Cockpit shows
  paused line, call stack, locals, watches, Immediate deck, and debug
  key rail.
- **Design.** Add projection structs:
  - `RunTimelineProjection`
  - `RunStepProjection`
  - `DebugStateProjection`
  - `StackFrameProjection`
  - `LocalValueProjection`
  - `WatchProjection`
  These are fixture-backed only. Debug fixture names must reference the
  W080 audit dependency for real contracts. Both scenarios are exposed
  as W038 lab registry entries under the Fire Horse provider.
- **Tests.**
  - Unit contract: Run Lane active step must be exactly one of
    prepare/analyze/build/execute/result.
  - Unit contract: Debug Cockpit paused state must include current
    document + line, at least one call-stack frame, and a Key Rail
    with continue/step/return.
  - `wtd` journeys for both scenarios use the W038 lab helper and assert
    key visible tokens.
- **Evidence.** Five-minute pass compares against
  `refined_04_run_lane.png` and `refined_05_debug_cockpit.png`.
- **Closure.**
  - [ ] Run Lane renders from event-shaped fixture data.
  - [ ] Debug Cockpit renders from debug-shaped fixture data.
  - [ ] Both WTD journeys green.

### W039-B06 — Launchpad, Console Fit, and Compact Focus terminal proof

**Feature.**

- **Goal.** UX lab renders:
  - `firehorse-launchpad-standard`
  - `firehorse-console-fit-light`
  - `firehorse-focus-compact`
  with the intended state posture and no dependency on real session
  files or terminal capability probes.
- **Design.** Launchpad fixture uses MRU-shaped rows but reads no
  session store. Console Fit fixture uses capability-shaped rows but
  reads no terminal probe. Focus Compact uses the same Code Canvas
  projection with Project/Context/Activity hidden into temporary dock
  affordances. Each scenario declares its W038 viewport class:
  Launchpad and Console Fit use `Standard`; Focus Compact uses
  `Compact`.
- **Tests.**
  - Unit contract: Launchpad recent rows include project, target,
    health, recency.
  - Unit contract: Console Fit rows include result + recommendation
    fields and all signals have text labels, not color alone.
  - Unit contract: Focus Compact omits Project Spine and Context Dock
    but keeps Code Canvas, Activity Rail, and Key Rail.
  - WTD journeys for each scenario use the W038 lab helper and assert
    visible contracts.
- **Evidence.** Five-minute pass compares against refined screenshots.
- **Closure.**
  - [ ] Three scenarios render.
  - [ ] Compact mode works at the configured compact WTD size.
  - [ ] WTD journeys green.

### W039-B07 — Read-only adapter from current `ShellState`

**Infrastructure.**

- **Goal.** A real loaded Editing scene can be projected into the
  Fire Horse model far enough to populate Identity Rail, Project Spine,
  Code Canvas, Activity Deck, and Key Rail without changing product
  behavior.
- **Design.** Add a one-way adapter:

  ```rust
  impl FireHorseProjection {
      pub fn from_shell_state(state: &ShellState) -> Self { ... }
  }
  ```

  Keep it behind a UX-lab feature flag or lab-only module and expose it
  as a W038 lab provider source such as `firehorse-real-editing`. It
  must not replace the production renderer. Missing data is represented
  as explicit `Unavailable { reason }` projection fields, not guessed.
- **Tests.**
  - Unit contract: adapter from current thin-slice Editing state fills
    the required rails and does not panic when optional semantic data
    is absent.
  - Unit contract: adapter does not mutate `ShellState`.
  - `wtd` journey: launch the real editing projection through the W038
    lab helper and assert the same project/module tokens visible in the
    existing Editing scene are present in Fire Horse projection.
- **Evidence.** Five-minute pass: compare fixture Editing Lens vs real
  Editing projection and list the remaining data gaps.
- **Closure.**
  - [ ] Adapter is read-only.
  - [ ] Missing seam data is explicit.
  - [ ] No production render path changes.

### W039-B08 — Handoff reconciliation

**Doctrine.**

- **Goal.** The downstream worksets know exactly what W039 proved and
  what they should implement. `docs/firehorse_mockups/HARDENING_REVIEW.md`,
  `docs/DESIGN_TUI_2026_FIRE_HORSE.md`, and affected workset specs are
  updated with links to W039 outputs.
- **Design.** Add a "W039 handoff" section to the hardening review and
  concise references in W040 / W060 / W070 / W080 / W090 / W100 where
  relevant. Do not rewrite those worksets' bead lists unless W039
  changes their scope; cite the projection contract and scenario
  captures as input.
- **Tests.** Doctrine bead. No unit tests.
- **Evidence.** Read-through checklist:
  - each downstream workset that consumes a Fire Horse surface has a
    citation;
  - no downstream workset is assigned behavior W039 did not prove;
  - open seam gaps are listed rather than hidden.
- **Closure.**
  - [ ] Hardening review links W039 outputs.
  - [ ] Downstream specs cite W039 where useful.
  - [ ] Workset register still reflects correct ordering.

## Out-of-scope

- **Shipping `ox-ide` visual replacement.** W039 proves the design in
  terminal cells; it does not switch the product renderer.
- **Real project mutation.** Add module/class/reference, MRU writes,
  session restore, and project actions remain W040.
- **Real semantic feature expansion.** Inline diagnostics, completion,
  references, rename, and real semantic source lenses remain W060.
- **Real run/immediate implementation.** W039 may fixture the Run Lane
  and Immediate rows; W070 owns real execution.
- **Real debugging.** W039 may fixture Debug Cockpit; W080 owns
  breakpoints, stepping, locals, watches, and OxVba debug contract
  audit.
- **Real command registry.** W039 names action ids; W090 owns command
  resolution, keymap profiles, chords, and user overrides.
- **Real terminal capability probing.** W039 fixtures Console Fit; W100
  owns probes and fallback behavior.
- **Deleting older mockup captures.** Non-canonical PNGs stay until the
  user gives an exact delete command in-session.
