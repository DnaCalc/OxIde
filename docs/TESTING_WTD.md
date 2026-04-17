# OxIde Testing With WinTermDriver

Status: `active`
Owned by: `W037` and `W038` and onward

## 1. Purpose
This document is the durable reference for how OxIde is driven and verified
via `WinTermDriver` (wtd). Every workset from W037 onward closes against a
WTD scenario rather than only against internal assertions, so this is where
the test shape lives.

Related:
- [WORKSET_REGISTER.md](/C:/Work/DnaCalc/OxIde/docs/WORKSET_REGISTER.md)
- [worksets/W037_wtd_harness.md](/C:/Work/DnaCalc/OxIde/docs/worksets/W037_wtd_harness.md)
- [worksets/W038_ux_development_lab.md](/C:/Work/DnaCalc/OxIde/docs/worksets/W038_ux_development_lab.md)

## 2. Why WTD
- **Headless driving.** ConPTY-backed child process launch with key / chord / mouse injection via the `wtd` CLI or named-pipe IPC. Works in CI and locally.
- **Queryable screen state.** `wtd capture` returns text, dimensions, cursor, mouse mode, scrollback, title, progress info.
- **Replayable VT snapshots.** `wtd capture --vt` emits a cell-accurate snapshot that can be replayed to a terminal or diffed as a golden.

## 3. Harness Shape
A dev-only crate lives at `tests/wtd/` and exposes:

```
OxIdeHarness::spawn(workspace, target) -> Self
  // `wtd open <workspace>`, attaches to <target>
fn send_keys(keys: &[&str])
fn send_text(text: &str)
fn mouse(col, row, button)
fn capture() -> Capture
fn capture_vt() -> VtSnapshot
fn wait_for_text(pattern: &str, timeout: Duration)
fn wait_for_stable_frame(window: Duration)
fn assert_snapshot(name: &str)
fn update_snapshot(name: &str)   // gated on UPDATE_SNAPSHOTS=1
```

Waits are implemented in-crate (polling with timeout); WTD itself does not
provide stable-frame detection.

## 4. Scenario Workspaces
Scenario YAMLs live under `.wtd/`:

- `.wtd/oxide-smoke.yaml` - thin-slice happy path (W037 baseline)
- `.wtd/oxide-edit.yaml` - editor scenarios (W050)
- `.wtd/oxide-diagnostics.yaml` - error / fix cycle (W060)
- `.wtd/oxide-run.yaml` - build / run / immediate (W070)
- `.wtd/oxide-debug.yaml` - breakpoint / step (W080)
- `.wtd/oxide-keymap.yaml` - command / profile (W090)
- `.wtd/oxide-caps.yaml` - capability probe variants (W100)

Each workspace launches `cargo run -p ox-ide -- <project>` in a dedicated
pane against a fixture project. The thin-slice in `examples/thin-slice` is
the starting fixture; richer fixtures are introduced as later worksets land.

## 5. Golden Snapshot Discipline
- Goldens live at `tests/wtd/goldens/<workset>/<scenario>.vt` with a parallel `.txt` for human diffs.
- Updates are reviewed artifacts: the diff is visible in the PR, and the driving workset cites the uxpass decision that motivates any intentional change.
- A dev command (`cargo xtask snapshots` or `just snapshots-update`, TBD in W037) regenerates and reports diffs.
- Where a scene changes shape by width class, snapshots are stored for 100x30, 120x40, and 160x50.

## 6. Local And CI Loop
- Local default: `cargo test` - unit tests only; WTD suite is gated behind a `wtd` cargo feature to keep the default loop fast.
- Local WTD: `cargo test --features wtd` - full headless scenario sweep.
- CI: Windows runner executes both on every PR. WTD artifacts (captures, diffs) are uploaded so reviewers can inspect failures visually.
- The WTD suite is the living definition of "the IDE still looks right." Every workset closes by adding at least one scenario to it.

## 7. Risks And Known Gaps
- WTD is Windows-only. Not a problem today (OxIde targets Windows first) but a future porting tax.
- No built-in stable-frame detection - the polling helper is the workaround.
- Snapshot churn risk during early worksets - mitigated by keeping the W037 golden set minimal and expanding only as scenes stabilise under the uxpass revision.

## 8. Checkable UX Loop (W038)
The WTD suite is the outer regression loop. The UX development lab (W038)
adds an inner and middle loop so UX work is cheap and the captures can be
*read* by a developer, not just diffed by CI.

**Inner loop - "see the scene" (seconds).**
`oxide --scene <id>` boots OxIde directly into a named scene. A thin
scenario loader applies a known-good `ShellState` (e.g. `empty`,
`editing`, `build-success`, `build-failure`). Intended for hand use while
iterating on a scene.

**Middle loop - "verify one scenario" (tens of seconds).**
`oxide --scenario <yaml>` plays a deterministic script of `Msg` values
into `ShellModel::update`. The same scenario definitions power both the
automated WTD suite and the Lab. `oxide-vt replay <file.vt>` pipes any
committed VT snapshot to the current terminal so a developer can see
exactly what a test saw; `oxide-vt diff <a.vt> <b.vt>` prints a
human-legible diff.

**Outer loop - "full regression" (minutes).**
`cargo test --features wtd` runs the full scenario sweep against
committed goldens, per §4-§6 above.

**UX Lab TUI.**
`oxide-uxlab` is a separate FrankenTui binary that embeds `ShellModel`
and presents three panes: scenario list from `.wtd/`, live-rendered
scenario, and golden + diff view. Hotkeys: `Enter` run, `c` capture,
`g` toggle golden, `d` toggle diff, `b` bless, `r` reset. This is the
day-to-day surface for reading the UI while it is being built.

**Diff policy (initial).**
VT diff starts text-only. The focus is readability first, diff strictness
second. A strict cell-exact mode is opt-in. The permanent diff policy is
decided in uxpass §50, not in W037 or W038.

## 9. Pointers
- WTD source: `C:/Work/WinTermDriver`
- WTD CLI reference: see the project README; `wtd open`, `wtd capture`, `wtd keys`, `wtd mouse`, `wtd input`, `wtd follow`.
- FrankenTui: `.external/frankentui` (used by OxIde as the shell foundation).
