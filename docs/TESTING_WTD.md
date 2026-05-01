# OxIde Testing With WinTermDriver

Mechanical reference for driving and verifying OxIde via
`WinTermDriver` (`wtd`). This document is the *how* of the harness.
The *when / why / what closes a bead* discipline lives in
[`docs/BEADS.md`](BEADS.md) §2.1 (Tests) and §3 (Five-Minute User
Pass).

## 1. Why `wtd`

- **Headless driving.** ConPTY-backed child-process launch with key /
  chord / mouse injection via the `wtd` CLI or named-pipe IPC. Works
  in CI and locally.
- **Queryable screen state.** `wtd capture` returns text, dimensions,
  cursor, mouse mode, scrollback, title, progress info.
- **Replayable VT snapshots.** `wtd capture --vt` emits a cell-
  accurate snapshot that can be replayed to a terminal or diffed as
  a golden.

## 2. Harness Shape

A dev-only crate at `tests/wtd/` exposes:

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

Waits are implemented in-crate (polling with timeout); `wtd` itself
does not provide stable-frame detection.

## 3. Scenario Workspaces

Scenario YAMLs live under `.wtd/`:

- `.wtd/oxide-smoke.yaml` — thin-slice happy path (W037 baseline).
- `.wtd/oxide-uxlab-smoke.yaml` — W038 `oxide-uxlab --once` smoke
  path and reusable lab capture convention.
- `.wtd/oxide-audit-lab-studio.yaml` — W041 interactive Audit Lab in
  Studio.
- `.wtd/oxide-audit-lab-first-class.yaml` — W041 interactive Audit Lab
  in First-class.
- `.wtd/oxide-edit.yaml` — editor scenarios (W050).
- `.wtd/oxide-diagnostics.yaml` — error / fix cycle (W060).
- `.wtd/oxide-run.yaml` — build / run / immediate (W070).
- `.wtd/oxide-debug.yaml` — breakpoint / step (W080).
- `.wtd/oxide-keymap.yaml` — command / profile (W090).
- `.wtd/oxide-caps.yaml` — capability probe variants (W100).

Each workspace launches `cargo run -p ox-ide -- <project>` (or the
release binary directly) in a dedicated pane against a fixture
project. `examples/thin-slice` is the starting fixture; richer
fixtures are introduced as later worksets land.

UX-lab workspaces launch `target/release/oxide-uxlab.exe` directly with
the non-interactive shape:

```text
oxide-uxlab.exe --suite <suite> --scenario <id> --viewport <viewport> --once
```

WTD tests should declare a `LabScenarioJourney` from
`tests/support/mod.rs`, then call `Harness::open_lab_once`,
`capture_lab_once_text`, and `capture_lab_once_vt`. This keeps W039 Fire
Horse journeys on the same suite/id/viewport contract as the W038 smoke
scenario.

Interactive Audit Lab tests use the same `Harness::open` path and drive
the release `oxide-uxlab` cockpit through `tests/wtd/audit_lab.rs`.

## 4. Golden Snapshot Discipline

- Goldens live at `tests/wtd/goldens/<workset>/<scenario>.vt` with a
  parallel `.txt` for human diffs.
- UX-lab goldens use the same layout. The W038 convention example is
  `tests/wtd/goldens/W038/uxlab_once_smoke.{txt,vt}`.
- Updates are reviewed artefacts. The diff is visible in the PR and
  the bead that produced it cites the design change that motivates
  it.
- A dev command regenerates and reports diffs (`UPDATE_GOLDENS=1
  cargo test --features wtd --test wtd_smoke`; a future
  `cargo xtask snapshots` wrapper is tracked under W037).
- Where a scene changes shape by width class, snapshots are stored
  for 100x30, 120x40, and 160x50.

## 5. Replaying VT Captures

Use `ox-vt replay` when the `.txt` golden is too flattened for visual
review and the committed `.vt` stream should be inspected in a real
terminal:

```text
cargo build --release --bin ox-vt
target\release\ox-vt.exe replay tests\wtd\goldens\W039\firehorse_editing_lens_standard.vt
cargo build --release --bin oxide-uxlab
cargo test --features wtd --test wtd_smoke audit_lab
```

`replay` writes the captured VT bytes directly to stdout. Run it in a
terminal pane where alternate-screen controls, colour, and cursor
movement are acceptable. The command does not interpret the capture as
product truth and does not modify goldens.

## 6. Audit Lab Operator Shortcuts

`oxide-uxlab` can generate a temporary WTD workspace for the current
Fire Horse design realization. This is the preferred quick path when a
reviewer wants to see the real ConPTY-rendered surface without remembering
the checked-in `.wtd/oxide-firehorse-*.yaml` files:

```text
target\release\oxide-uxlab.exe --audit --suite firehorse --wtd-open
target\release\oxide-uxlab.exe --audit --suite firehorse --scenario firehorse-editing-lens-standard --viewport studio --wtd-open
```

`--wtd-open` leaves the WTD workspace open and prints the target path plus
the matching `wtd capture` commands. Scenario design panes run the ANSI
mockup stream so the WTD UI shows the same color and border treatment that
the terminal renderer emits.

For durable evidence, use `--wtd-capture <root>`. The root must be under
`target/ux_audit_lab` or `docs/firehorse_mockups/ux_audit_lab`; the
command creates a unique run directory, writes the generated workspace,
waits for a stable frame, and stores both flattened text and byte-exact
VT:

```text
target\release\oxide-uxlab.exe --audit --suite firehorse --scenario firehorse-editing-lens-standard --viewport studio --wtd-capture target/ux_audit_lab/wtd_design --json
```

The command is an operator wrapper over the same WTD mechanism used by
the `cargo test --features wtd` journeys. It does not replace the WTD
goldens; it makes ad-hoc viewing and evidence capture easier.

## 7. Local And CI Loop

- Default: `cargo test` runs unit tests only. The `wtd` suite is
  gated behind a `wtd` cargo feature to keep the default loop fast.
- Per-journey during bead work: `cargo test --features wtd --test
  <journey>`.
- Full sweep before push: `cargo test --features wtd`.
- CI: Windows runner executes both on every PR. `wtd` artefacts
  (captures, diffs) are uploaded so reviewers can inspect failures
  visually.

## 8. Risks And Known Gaps

- `wtd` is Windows-only. Not a problem today (OxIde targets Windows
  first) but a future porting tax.
- No built-in stable-frame detection — the polling helper is the
  workaround.
- Release-binary lock on Windows: a lingering `ox-ide.exe` from a
  prior `wtd` run can prevent the next `cargo build --release`. The
  current workaround is `taskkill /F /IM ox-ide.exe` before rebuild.

## 9. Pointers

- `wtd` source: `C:/Work/WinTermDriver`.
- `wtd` CLI reference: see the project README; `wtd open`, `wtd
  capture`, `wtd keys`, `wtd mouse`, `wtd input`, `wtd follow`.
- FrankenTui: `.external/frankentui` (used by OxIde as the shell
  foundation).
- UX development lab (scene flag, scenario driver, VT replay / diff,
  `oxide-uxlab` TUI): see
  [`worksets/W038_ux_development_lab.md`](worksets/W038_ux_development_lab.md).
