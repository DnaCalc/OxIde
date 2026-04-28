# Workset W037 — WinTermDriver Test Harness Foundation

## Ambition

Every user-facing bead from this point forward can land a `wtd` journey
scenario as part of its closure (`docs/BEADS.md` §2.1 Tests layer).
That requires the harness — crate, workspace definitions, goldens, and
the test-runner gate — to exist and be ergonomic enough to use per bead.

At the end of W037 a contributor working on any future bead can:
- spawn the release binary in a known-good workspace,
- drive real keystrokes against it,
- capture the visible result,
- assert contract properties of that result,
- re-bless goldens when the contract intentionally changes,

all in five lines of Rust plus one YAML scenario.

## Dependencies

- **W030** — real OxVba integration. Without it the scenarios would
  drive mocks, which defeats the purpose.

## Design

### Harness crate

A dev-only crate at `tests/wtd/` exposes the shape already documented
in [`docs/TESTING_WTD.md`](../TESTING_WTD.md) §2. Summary:

```
OxIdeHarness::spawn(workspace, target) -> Self
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

Polling-based stable-frame detection is in-crate (`wtd` does not
provide it).

### Scenario workspaces

Per-scenario YAMLs under `.wtd/`. W037 ships:

- `.wtd/oxide-smoke.yaml` — launches two panes against the release
  binary: one empty-scene pane, one thin-slice-loaded pane.

Later worksets add their own YAMLs (see
[`docs/TESTING_WTD.md`](../TESTING_WTD.md) §3).

### Goldens

Under `tests/wtd/goldens/<workset>/`. W037 ships the baseline:

- `W037/empty.txt` + `.vt`
- `W037/thin_slice_loaded.txt` + `.vt`

Future worksets deposit goldens under their own workset folder.

### Test-runner gate

`cargo test --features wtd` runs the `wtd` suite. The default
`cargo test` stays fast (unit tests only). Individual journey
scenarios run via `cargo test --features wtd --test <journey>`.

### Snapshot-update flow

`UPDATE_GOLDENS=1 cargo test --features wtd --test <journey>`
regenerates the golden and writes it to the committed path. A future
`cargo xtask snapshots` wrapper (tracked below) makes this less
error-prone.

## Beads

### W037-B01 — Harness crate and baseline goldens

**Feature (infrastructure-flavoured).**

*Already landed in the current tree. Documented here in the new
schema so it closes under the current method once its evidence
checklist is complete.*

- **Goal.** A contributor can run `cargo test --features wtd --test
  wtd_smoke` and see two `wtd` scenarios pass against the committed
  goldens (empty scene, thin-slice loaded).
- **Design.** Crate at `tests/wtd/`, harness module `tests/support/`,
  scenario `.wtd/oxide-smoke.yaml`, goldens under
  `tests/wtd/goldens/W037/`. Feature-gated behind `wtd` in
  `Cargo.toml`.
- **Tests.**
  - Unit contract: `assert_snapshot` correctness (trivial self-test of
    the harness; diffs fail the test when content differs).
  - `wtd` journey: the two baselines (`wtd_smoke.rs` with
    `empty_scene_matches_golden` and `thin_slice_loaded_matches_golden`).
- **Evidence.**
  - The two tests pass against a freshly built release binary.
  - Goldens re-blessable via `UPDATE_GOLDENS=1`.
  - `docs/TESTING_WTD.md` references the harness with accurate paths.
- **Closure.**
  - [x] Crate present and feature-gated.
  - [x] Two baseline goldens present and passing.
  - [x] `docs/TESTING_WTD.md` accurate.
  - [ ] Five-minute user pass against release binary (in Wide, Standard,
    and Narrow terminal widths) confirms the goldens match live
    rendering. *Pending — becomes part of the clean-up pass that
    adds `wtd` journeys for already-shipped features.*

### W037-B02 — Journey-scenario template under `tests/wtd/`

**Infrastructure.**

- **Goal.** A contributor starting a new feature bead can copy a
  single-file template to `tests/wtd/journey_<feature>.rs`, fill in
  the keystrokes and assertions, and run it in under 30 seconds.
- **Design.** A `tests/wtd/template_journey.rs` committed as a stub
  (not compiled into the test run by default) with:
  - harness setup boilerplate (spawn, wait-for-stable-frame),
  - keystroke sending example,
  - capture + assertion example,
  - the `UPDATE_GOLDENS` re-bless pattern.
- A short `tests/wtd/README.md` explains copy-rename usage.
- **Tests.** Unit contract: the template compiles when copied into a
  live test (verified by a dedicated `template_compiles.rs` stub test
  that `mod`-includes the template with a rename).
- **Evidence.** A fresh bead author can land a new journey scenario
  end-to-end (start → commit) in under 30 minutes without reading
  the full harness source.
- **Closure.**
  - [ ] Template file present.
  - [ ] `tests/wtd/README.md` explains the flow.
  - [ ] Author-trial: a real subsequent bead uses the template
    path-free.

### W037-B03 — `cargo xtask snapshots` re-bless wrapper

**Infrastructure.**

- **Goal.** `cargo xtask snapshots` (or a close equivalent shell
  script at `scripts/snapshots.ps1`) runs
  `UPDATE_GOLDENS=1 cargo test --features wtd` after killing any
  lingering `ox-ide.exe` processes and rebuilding release. One
  command, not four.
- **Design.** Either a `xtask` crate (preferred once we introduce it)
  or a simple PowerShell script wrapping `taskkill` +
  `cargo build --release` + `UPDATE_GOLDENS=1 cargo test`. Logs each
  step. Fails loudly if a process lock can't be broken.
- **Tests.** Unit contract: none (tool). Manual run verifies the
  end-to-end flow.
- **Evidence.** A contributor re-blesses a golden in one command
  without racing a lingering process.
- **Closure.**
  - [ ] Tool or script present.
  - [ ] Documented in `docs/TESTING_WTD.md` §4.

### W037-B04 — Width-class golden coverage

**Feature.**

- **Goal.** The two baselines are captured at 100x30 (Narrow), 120x40
  (Standard), and 160x50 (Wide) — so later scenes that change shape by
  width-class can land goldens at the three canonical sizes.
- **Design.** `.wtd/oxide-smoke.yaml` gains three panes per scenario
  at the three sizes. Goldens under
  `tests/wtd/goldens/W037/<scenario>_<width>x<height>.{txt,vt}`.
- **Tests.** The three goldens pass against the release binary at the
  three widths.
- **Evidence.**
  - Three goldens per scenario exist.
  - Narrow golden shows Inspector collapsed (per D12).
  - Wide golden shows the taller Lower Surface (per D10 / D11).
- **Closure.**
  - [ ] Goldens exist for all three widths.
  - [ ] Each width golden exercises its width-specific layout
    contract.

## Out-of-scope

- **UX development lab** (inner loop / scenario replay /
  `oxide-uxlab` TUI). Owned by W038.
- **Journey scenarios for already-shipped features.** The
  harness is built here; the journeys for features that already
  landed (Ctrl+S, Ctrl+Z, F1, F12, Ctrl+N, palette Enter dispatch,
  etc.) are part of the follow-up clean-up pass.
- **Linux / macOS support.** `wtd` is Windows-only. Noted in
  [`docs/TESTING_WTD.md`](../TESTING_WTD.md) §6 as a future porting
  tax.
