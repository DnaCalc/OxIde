# Workset W070 — Run Surfaces And Immediate Panel

## Ambition

Running, iterating, and evaluating are first-class in OxIde.

A VBA author presses `F5`, sees the build + run event stream live in
the Lower Surface, reads the run's exit state in the Inspector, and
goes back to Editing cleanly when the run is done. They open the
Immediate panel, type an expression, watch OxVba evaluate it, and
iterate on the running project without leaving the keyboard.

At the end of W070 the edit→run→inspect→iterate loop is tight enough
that an OxVba author prefers OxIde over the VBA IDE for hot work.

W070 does **not** include full debugger UX — breakpoints / stepping /
callstack / locals / watches are W080 proper.

## Dependencies

- **W030** — `WebShellSession` run contracts (exist).
- **W050** — save/dirty already in force; running requires
  consistency between buffer and disk, which Save delivers.
- **W060** — diagnostics already in force; the run path surfaces
  OxVba-reported build failures through the same channel.
- **W039** — Fire Horse Run Lane proof. It supplies the staged run
  timeline and Activity Deck presentation target; W070 owns the real
  execution stream and Immediate evaluation.

### W039 Fire Horse Input

W039 proved Run Lane as a terminal-cell composition from seam-shaped
fixture events: prepare, analyze, build, execute, result, structured
output rows, and run-specific Key Rail. W070 should bind that shape to
real `WebHostEvent`/execution data, save-before-run policy, output
filtering, return-to-edit behavior, and Immediate evaluation. W039 did
not run code or prove an evaluation API.

## Design

### What already landed

- `F5` triggers `Msg::RunProject` → `apply_scene(BuildRun)` with
  live `WebHostEvent` stream to the Lower Surface `Output` pane.
- Inspector in BuildRun carries `Run Status` (build/runtime/profile/
  last exit) + `Target` (entry + active buffer).
- Output lines carry `[workspace]`, `[diagnostic]`, `[stdout]`
  prefixes; the `" with N user slots"` jargon suffix is stripped.
- Dirty edits survive F5 → BuildRun (preservation fix landed
  during the audit pass).

### New surfaces

**Return from Run.** Currently BuildRun is a sticky scene — no
keystroke returns to Editing. W070 adds `Esc` (when no popover /
overlay is active) as the return, restoring the prior Editing focus.
`F5` from BuildRun is a re-run.

**Filter and auto-scroll in Output.** The Lower Surface Output pane
gains an `f` filter (prefix filter on the stream labels), `/` search
(highlight-next), and auto-scroll-to-bottom toggled by `End`. The
status line during BuildRun documents these.

**Immediate panel.** New `LowerSurfaceMode::Immediate` (already
reserved in the enum). Toggled by `Ctrl+I` or selected by the Lower
Surface mode cycler. Renders a single-line prompt below a history
buffer; Enter evaluates via `WebShellSession::evaluate` (or closest
OxVba host API — verify in a rollout bead if needed); results append
to history.

**Run status polish.** Inspector Run Status gets a one-line summary
of time-elapsed + exit code. Failed runs render the red `✗` gutter
mark alongside the diagnostic; successful runs a green `✓`.

### Save-before-run policy

`F5` writes every dirty buffer to disk before running, by default.
This matches IDE convention (VBA IDE compiles the in-memory state;
OxVba's runtime reads from disk). User can override via
`--run-without-save` (dev) or a setting later; default is save.

## Beads

### W070-B01 — `Esc` returns from BuildRun to prior Editing

**Feature.**

- **Goal.** Pressing `Esc` while in BuildRun (no overlay, no popover)
  transitions the scene back to Editing with focus restored to the
  Editor and the prior cursor position.
- **Design.** Extend `Msg::CloseOverlay` handler: if no popover,
  no overlay, and current scene is BuildRun, dispatch
  `apply_scene(previous_scene)` which is Editing (saved by
  `runtime.previous_scene`). Avoid collision with palette-close
  cascade.
- **Tests.** Unit contract: in BuildRun with no overlay, Esc
  transitions to Editing and preserves workspace. `wtd` journey:
  F5 → Esc → assert top bar back to Editing.
- **Evidence.** Five-minute pass: F5, then Esc, confirm
  round-trip preserves edits.

### W070-B02 — Save-before-run on `F5`

**Feature.**

- **Goal.** `F5` writes every dirty buffer before triggering the run,
  so the runtime reads the user's in-memory changes from disk.
- **Design.** `Msg::RunProject` handler calls
  `shell.save_all_dirty_buffers()` first. Any error popup surfaces,
  run aborts with an honest popover.
- **Tests.** Unit: with a dirty buffer, RunProject writes the file
  before launching; with a save failure, run does not launch and a
  popover appears. `wtd` journey: dirty-then-F5 → assert the on-disk
  file matches the in-memory buffer before the run started.

### W070-B03 — Output filter, search, auto-scroll

**Feature.**

- **Goal.** In BuildRun focus=LowerSurface (Output mode): `f` prompts
  a prefix filter; `/` prompts a search (next match highlighted);
  `End` toggles auto-scroll-to-bottom; `Home` scrolls to top.
- **Design.** Extend Lower Surface state with filter / search /
  auto-scroll fields; view layer applies them before render.
- **Tests.** Unit: filter/search/scroll state transitions.
  `wtd` journey: F5, focus LowerSurface, `f diagnostic`, assert only
  `[diagnostic]` rows visible.

### W070-B04 — Immediate panel (`Ctrl+I`)

**Feature.**

- **Goal.** `Ctrl+I` opens/focuses the Immediate panel in the Lower
  Surface. The user types an expression; Enter evaluates; the result
  appends to the history with `>` for input and the result lines
  below. History persists across the session but resets on quit.
- **Design.** New `LowerSurfaceMode::Immediate` render path; new
  `ImmediateState { history: Vec<ImmediateEntry>, input: String }`;
  `Msg::ImmediateInput(char)`, `Msg::ImmediateSubmit`,
  `Msg::ImmediateHistoryUp` / `Down` (recall).
  Evaluation binds to `WebShellSession::evaluate` or equivalent — if
  no such API exists yet, file a dependency bead against the OxVba
  surface before completing this bead.
- **Tests.** Unit: input / submit / recall. `wtd` journey: F5 → run
  to completion → Ctrl+I → type `2 + 2` → Enter → assert `4`
  appears in the history.
- **Evidence.** Requires the OxVba evaluation API. If absent, bead
  blocks on the OxVba dependency bead.

### W070-B05 — Run status visual polish

**Feature.**

- **Goal.** Inspector's `Run Status` adds a time-elapsed row and a
  bolded exit-code row with colour: green `✓` on success, red `✗` on
  failure. Same mark appears in the Editor gutter for the line the
  runtime last reported.
- **Design.** Extend `ExecutionState` with `started_at`,
  `completed_at`, `last_halt_line`. Inspector render updates.
- **Tests.** Unit: elapsed computation, mark colour for each exit
  code. `wtd` journey: run success + failure variants, assert
  Inspector content.

## Out-of-scope

- **Full debugger UX** — breakpoints, step in/out/over, call stack,
  locals, watches, exception surfaces. Owned by **W080**.
- **Hot reload** — changing code mid-run. Not a W070 concern.
- **Run configuration management** — multiple run profiles,
  command-line args per profile, environment. Later worksets
  once OxVba exposes the contracts.
- **Remote execution** — host OxVba runtime on another machine.
  Out of scope.
