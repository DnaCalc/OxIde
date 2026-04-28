# Workset W080 — Debug Surfaces

## Ambition

A VBA author can debug a running project from inside OxIde — set a
breakpoint in the Editor gutter, launch, observe the runtime suspend
on the marked line, inspect locals and the call stack, step through
statements, evaluate expressions in Immediate against the suspended
frame, continue, and see the frame resume.

At the end of W080 OxIde provides a genuine debugger UX over the
OxVba runtime's stepping contracts. If OxVba does not yet expose a
particular contract, the dependency is surfaced as a blocking bead
against OxVba, not invented in OxIde.

## Dependencies

- **W070** — run surfaces (output / Run Inspector / Immediate) — a
  breakpoint-suspended state extends those surfaces rather than
  displacing them.
- **W035 §30** — scene catalogue has ruled on debug-as-scene vs
  debug-as-overlay (decision forthcoming; this workset assumes
  debug-as-scene per the uxpass direction).
- **OxVba runtime contract** — stepping and suspend contracts must
  exist on the OxVba side. A preparatory bead audits what is
  available today.
- **W039** — Fire Horse Debug Cockpit proof. It supplies the terminal
  posture, paused-line composition, Debug Dock/Activity Deck roles, and
  key hints; W080 owns the real debug contracts.

### W039 Fire Horse Input

W039 proved Debug Cockpit from fixture data and intentionally named the
W080 audit as the source of real truth. W080 must first audit OxVba
debug contracts, then replace fixture call stack, locals, watches,
breakpoints, stepping, and halt data with real runtime seams. The W039
golden is a visual and state-mapping target, not evidence that debug
behavior exists.

## Design

### Debug scene

New `ShellScene::Debug` entered on breakpoint hit (or `F9` + `F5`
combination once a breakpoint is set). Body decomposition:

```
[ Explorer | Editor (source with suspend arrow) | Debug Inspector ]
[ Lower Surface: Callstack / Locals / Watches / Immediate cycle ]
[ Status line: F10 step-over  F11 step-in  Shift+F11 step-out
             F5 continue  Shift+F5 stop  F6 palette  Ctrl+Q quit ]
```

The Debug Inspector carries three sub-panes: `Callstack`, `Locals`,
`Watches`. The Lower Surface mode set becomes
`Callstack / Locals / Watches / Immediate` with `Callstack` default.

### Breakpoints in the editor gutter

The gutter already carries line numbers (landed during W050 pre-
landings). Breakpoints render as a `●` glyph in the column just
before the line number. Toggle via `F9` on the cursor's line, or via
a left-click on the breakpoint cell.

Breakpoints persist per `DocumentSession` and, once session restore
is in force (W040 / W050), across restarts.

### Stepping / continue / stop

Key bindings:
- `F10` step over
- `F11` step in
- `Shift+F11` step out
- `F5` continue
- `Shift+F5` stop

Each dispatches to `WebShellSession` or whatever OxVba debug contract
W080-B01 audits in.

### Exception / halt handling

On exception or runtime halt, Debug scene enters with the offending
line visible in the Editor, a popover surfaces the exception message,
and the Inspector `Callstack` shows the frame.

## Beads

### W080-B01 — Audit OxVba debug contract

**Infrastructure (rollout bead).**

- **Goal.** A short document or commit message records which OxVba
  runtime APIs exist today for setting breakpoints, stepping, and
  suspending — and what must be added before the subsequent W080
  beads can run.
- **Design.** Grep `.external/oxvba-frozen/crates/oxvba-*/` for
  step / breakpoint / halt / suspend APIs. Summarise findings.
  Either proceed with existing contracts or file a dependency bead
  against the OxVba side with the exact function signatures needed.
- **Tests.** None; doctrine.
- **Evidence.** Findings committed as a note in this workset file,
  or a linked dependency bead.

### W080-B02 — Editor gutter breakpoints (`F9` toggle, persisted)

**Feature.**

- **Goal.** `F9` on a line in the Editor toggles a breakpoint marker
  in the gutter cell preceding the line number. Breakpoints persist
  per `DocumentSession` for the session.
- **Design.** New `Breakpoint { document_id, line }` set on
  `ProjectSession`. Gutter render appends a `●` cell when the line
  is marked. `F9` dispatches `Msg::ToggleBreakpoint`.
- **Tests.** Unit: toggle idempotent, breakpoint list survives a
  view switch. `wtd` journey:
  `tests/wtd/journey_toggle_breakpoint.rs` toggles a breakpoint on
  line 5, asserts the marker visible in the capture.

### W080-B03 — Debug scene + suspend UX

**Feature.**

- **Goal.** Running thin-slice with a breakpoint at line 5 causes
  the shell to enter `ShellScene::Debug` with the cursor (or a
  distinct arrow) on line 5, Debug Inspector showing callstack /
  locals / watches, and the status line advertising the stepping
  bindings.
- **Design.** New `ShellScene::Debug` in the enum. `apply_scene`
  adds its branch. Debug Inspector mode draws the three sub-panes.
  Suspend arrow `➤` renders in the gutter over the current
  statement line.
- **Tests.** Unit: entering Debug scene sets the expected Inspector
  / Lower Surface modes. `wtd` journey:
  `tests/wtd/journey_debug_suspend.rs` sets a breakpoint, `F5`,
  waits for suspend, captures the Debug scene shape.

### W080-B04 — Stepping controls

**Feature.**

- **Goal.** `F10` step-over, `F11` step-in, `Shift+F11` step-out,
  `F5` continue, `Shift+F5` stop. Each dispatches to the debug
  contract identified in W080-B01 and updates the Editor arrow,
  Inspector panes, and status line accordingly.
- **Design.** New `Msg::DebugStepOver / StepIn / StepOut / Continue
  / Stop`. Handler calls the OxVba debug contract.
- **Tests.** Unit: each Msg transitions the debug state machine.
  `wtd` journey:
  `tests/wtd/journey_debug_step_over.rs` suspends, `F10`, asserts
  the suspend arrow advanced.

### W080-B05 — Immediate against suspended frame

**Feature.**

- **Goal.** In Debug scene with a suspended frame, `Ctrl+I` focuses
  Immediate; typed expressions evaluate against the suspended
  frame's scope; results append to history with a `[frame <N>]`
  prefix.
- **Design.** Extend the Immediate panel from W070-B04 to route
  evaluation to the suspended frame when one exists. Prefix the
  history entry with frame id.
- **Tests.** Unit: with a suspended frame, evaluate uses frame
  scope; without, evaluates at global scope. `wtd` journey:
  `tests/wtd/journey_debug_immediate.rs` runs, suspends, types
  `answer`, asserts the value from the frame appears.

### W080-B06 — Exception / halt surfacing

**Feature.**

- **Goal.** When OxVba reports an unhandled exception or runtime
  halt, the shell enters Debug scene with the offending line visible,
  a popover surfaces the exception message, Callstack shows the
  frame, and `F5` / `Esc` resumes or aborts honestly.
- **Design.** New `ExecutionEvent::Halt { line, message, stack }`.
  Popover dismiss installs `Stop` as the default action.
- **Tests.** Unit: halt event transitions to Debug with popover.
  `wtd` journey: trigger a runtime error in a thin-slice-variant
  fixture, assert Debug scene with popover visible.

## Out-of-scope

- **Conditional breakpoints / hit counts.** Deferred.
- **Remote debug.** Out.
- **Time-travel debugging.** Out.
