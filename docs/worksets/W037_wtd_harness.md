# W037 - WinTermDriver Test Harness

Status: `planned`
Sequence: `3.7`
Depends on: `W030`

## 1. Purpose
Establish `WinTermDriver` (wtd) as the headless driver and capture tool for
the OxIde TUI, and ship the minimum test harness that lets every later
workset close against a real on-screen scenario rather than an internal
assertion.

## 2. Governing Truth
1. [PRODUCT_DIRECTION.md](/C:/Work/DnaCalc/OxIde/PRODUCT_DIRECTION.md)
2. [ARCHITECTURE.md](/C:/Work/DnaCalc/OxIde/ARCHITECTURE.md)
3. [TESTING_WTD.md](/C:/Work/DnaCalc/OxIde/docs/TESTING_WTD.md)

## 3. Intended Execution Lanes
1. dev-only harness crate under `tests/wtd/` exposing `OxIdeHarness::spawn`, `send_keys`, `send_text`, `mouse`, `capture`, `capture_vt`, `wait_for_text`, `wait_for_stable_frame`, `assert_snapshot`
2. scenario workspace definitions under `.wtd/` starting with `oxide-smoke.yaml`
3. golden `.vt` snapshots under `tests/wtd/goldens/` for the smoke baseline (empty scene, thin-slice loaded)
4. snapshot-diff tooling and `cargo test --features wtd` gating so the default test loop stays fast

## 4. Rollout Intention
This workset should start narrow: only the smoke-flow scenario and one or
two baseline goldens. Broader golden coverage is added by later worksets as
scenes stabilise.

## 5. Closure Condition
This workset closes when `cargo test --features wtd` drives the existing
thin-slice flow via wtd, captures baseline VT snapshots, and asserts them
against committed goldens. `docs/TESTING_WTD.md` documents the harness
shape, workspace layout, golden discipline, and CI expectations.
