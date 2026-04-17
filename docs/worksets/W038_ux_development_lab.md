# W038 - UX Development Lab

Status: `planned`
Sequence: `3.8`
Depends on: `W037`

## 1. Purpose
Turn the WTD harness from W037 into an interactive, human-drivable loop so
that UX work on OxIde is cheap, legible, and repeatable. The first aim is
that a developer can *read* any shell scene clearly - live, from a golden,
or from a recorded capture - without leaving the terminal.

## 2. Governing Truth
1. [PRODUCT_DIRECTION.md](/C:/Work/DnaCalc/OxIde/PRODUCT_DIRECTION.md)
2. [ARCHITECTURE.md](/C:/Work/DnaCalc/OxIde/ARCHITECTURE.md)
3. [TESTING_WTD.md](/C:/Work/DnaCalc/OxIde/docs/TESTING_WTD.md)
4. [worksets/W037_wtd_harness.md](/C:/Work/DnaCalc/OxIde/docs/worksets/W037_wtd_harness.md)

## 3. Intended Execution Lanes
1. **Scene flag** - `oxide --scene <id>` boots OxIde directly into a named scene (`empty`, `editing`, `build-success`, `build-failure`, ...). Thin scenario loader applies known-good `ShellState`.
2. **Scenario driver** - `oxide --scenario <yaml>` plays a deterministic script of `Msg` values into `ShellModel::update` without going through keystrokes. Single source for both "boot to scene X" and the automated WTD suite.
3. **VT replay and diff CLI** - a small binary (`oxide-vt`) with `replay <file.vt>` (pipes a captured snapshot to the current terminal), `diff <a.vt> <b.vt>` (text-first side-by-side diff, strict cell-exact mode opt-in), and `bless <name>` (promote a capture to a golden). Focus is readability first, diff strictness second.
4. **UX Lab TUI** - a separate FrankenTui binary `oxide-uxlab` with three panes (scenario list, live-rendered scenario, golden + diff). Hotkeys: `Enter` run, `c` capture, `g` toggle golden, `d` toggle diff, `b` bless, `r` reset.

## 4. Rollout Intention
Lanes 1-3 are small, land inside W037's tail or immediately after, and
unblock the Lab. Lane 4 ships as its own rollout bead once the scene
catalogue from W035 is stable enough to name scenarios.

`oxide-uxlab` is a separate binary (not a debug mode of OxIde), built on
FrankenTui, so that it can embed and drive `ShellModel` the same way the
WTD harness drives it externally. This keeps the production binary clean
and gives the Lab freedom to grow (multiple scenarios in tabs, side-by-side
width-class views, capture history).

The VT diff tool starts text-only. Strict cell-exact diff is an opt-in
mode. The permanent diff policy is an output of the uxpass §50 visual
language decision, not a W038 decision.

## 5. Closure Condition
This workset closes when:
1. `oxide --scene <id>` and `oxide --scenario <yaml>` work for every scene in the current catalogue,
2. `oxide-vt replay` reproduces any committed `.vt` golden to the current terminal,
3. `oxide-vt diff` produces a human-legible diff between two captures,
4. `oxide-uxlab` opens, lists the scenarios from `.wtd/`, and lets a developer pick a scenario, see it rendered live, see the golden, toggle the diff, and bless a new golden in one session,
5. `docs/TESTING_WTD.md` documents the inner / middle / outer development loop.
