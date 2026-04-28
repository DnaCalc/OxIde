# Workset W045 — WTD Demo Backfill And Affordance Wiring Audit

## Ambition

Every user-visible affordance advertised by the shipping shell is either
wired end-to-end or removed, and every already-shipped high-value binding
has an interactive `wtd` journey on the release binary.

At workset close, `tests/wtd/backfill.rs` is the canonical regression set for
the UX claims promoted during W035/W040/W050.

## Dependencies

- **W037** — WinTermDriver harness foundation.
- **W035** — UX pass decisions D1..D18 and their narrative docs.
- **W040 / W050** — project/open/create/save/undo/run interactions whose UX
  claims are now being backfilled with interactive journeys.

## Design

W045 adds one release-binary scenario per already-shipped feature with a
missing interactive demo, and hardens test style so regressions fail on
behavioral contracts rather than on row order/prose order.

### Harness Shape

- Extend `tests/support/mod.rs` with the missing input methods from
  `docs/TESTING_WTD.md` §2:
  - `send_keys`
  - `send_text`
  - `mouse_click`
- Add a dedicated workspace file `.wtd/oxide-backfill.yaml` rooted at
  `target/test-workspaces/wtd-backfill` so write-path scenarios (`Ctrl+N`,
  `Ctrl+S`, `Ctrl+Shift+S`) never touch checked-in fixtures.

### Scenario Set

`tests/wtd/backfill.rs` contains one journey per feature:

1. `Ctrl+O` open from Empty.
2. `Ctrl+N` scaffold and mount.
3. `Ctrl+S` save.
4. `Ctrl+Shift+S` save-all.
5. `Ctrl+Z`/`Ctrl+Y` undo/redo.
6. `F1` hover fallback + resolvable.
7. `F12` goto-definition.
8. `F5` run + `Esc` return.
9. `F6` palette Enter-dispatch.
10. Empty -> `F6` overlay backing.
11. Overlay/open + `F5` transition preserving dirty state.

### Contract Hardening

- Replace positional action tests (row/index assumptions) with label-driven
  selection/assertion in `src/shell/model.rs` and `src/shell/state.rs`.
- Replace inspector/lower-surface section index assertions with
  title-addressed assertions.

## Beads

### W045-B01 — Harness Input Completion (Infrastructure)

**Goal**
`wtd` journeys can drive keys/text/mouse from Rust tests without ad-hoc shell
calls.

**Design**

- File: `tests/support/mod.rs`
- Add `send_keys`, `send_text`, `mouse_click` methods on `Harness`.

**Tests**

- Unit contract: compile-time + call-site usage from `tests/wtd/backfill.rs`.
- Interactive: exercised by every W045 journey.

**Evidence**

- Backfill journeys compile and pass against release-binary workspace targets.
- Harness methods are exercised by `tests/wtd/backfill.rs`.

**Closure**

- [x] Harness methods implemented.
- [x] Methods used by the journey suite.

### W045-B02 — Advertised Affordance Wiring Sweep (Feature)

**Goal**
No advertised affordance in status line / palette / Welcome Start silently
no-ops.

**Design**

- Add `wtd` journeys covering advertised bindings.
- Add `Esc` return behavior from BuildRun to Editing so the BuildRun status
  hint is honest.
- Keep Start list to wired actions only (`Ctrl+O`, `Ctrl+N`).

**Tests**

- Unit contract:
  - BuildRun status-line hint contains `Esc return`.
  - `Msg::CloseOverlay` in BuildRun returns to Editing.
- Interactive journey:
  - BuildRun `F5` + `Esc` path in `tests/wtd/backfill.rs`.

**Evidence**

- Release-binary WTD journeys verify visible dispatch for advertised bindings.
- `goto-definition` now surfaces an explicit fallback popover on unresolved
  positions (no silent no-op).
- Five-minute user pass (`2026-04-18`) completed on
  `target/release/ox-ide.exe`.

**Closure**

- [x] BuildRun affordance is wired and documented.
- [x] No silent no-op for covered advertised bindings.

### W045-B03 — Brittle Positional Test Rewrite (Infrastructure)

**Goal**
Tests fail on behavioral contract changes, not list-position churn.

**Design**

- Label-based palette selection in model/state tests.
- Title-addressed inspector/lower-surface section lookup helpers.

**Tests**

- Unit contract: updated tests in `src/shell/model.rs` and `src/shell/state.rs`.
- Interactive: unchanged.

**Evidence**

- `cargo test` on affected modules passes.

**Closure**

- [x] No row-index assertions remain for palette action dispatch tests.
- [x] Scene-content assertions use section titles, not section indexes.

### W045-B04 — Interactive Demo Backfill Set (Feature)

**Goal**
Each listed shipped feature has a release-binary `wtd` journey under
`tests/wtd/`.

**Design**

- File: `tests/wtd/backfill.rs`.
- Module is loaded by `tests/wtd_smoke.rs` (same `wtd`-gated test target).

**Tests**

- Interactive journeys listed in the Scenario Set.

**Evidence**

- `target/release/ox-ide.exe` rebuilt (`cargo build --release --bin ox-ide`).
- All `tests/wtd_smoke` scenarios pass against the release binary via
  deterministic per-scenario execution:
  `cargo test --features wtd --test wtd_smoke <scenario> -- --exact --nocapture --test-threads=1`.
- Five-minute user pass command script reported `FIVE_MINUTE_USER_PASS_OK`.

**Closure**

- [x] All 11 scenarios present.
- [x] Journeys run against `target/release/ox-ide.exe`.
- [x] Journey names/readability are suitable as canonical regression set.

## Out-of-scope

- New UX surface invention outside the listed shipped features.
- Visual redesign beyond keeping advertised affordances honest.
- Replacing W037 goldens; W045 adds scenario assertions rather than another
  full-golden baseline.
