# Workset W200 — GUI Pivot Foundation, Codebase Review, And TUI Parking

## Ambition

OxIde gains a clean execution runway for the Rust/WASM-capable GUI pivot.

W200 does not attempt to build the full GUI product. It prepares the repository, truth surfaces, and implementation doctrine so later GUI work can proceed without being constrained by the current FrankenTui implementation.

The result should be a repo that clearly says:
1. GUI is the active direction,
2. TUI is parked and retained,
3. current code is behavior/evidence, not the GUI substrate,
4. cross-repo DNA Calc sharing should be exploited deliberately,
5. first GUI implementation work will be vertical, testable, and capability-aware.

## Dependencies

- [`PRODUCT_DIRECTION.md`](../../PRODUCT_DIRECTION.md) — active product direction after W200-B02 reconciliation.
- [`ARCHITECTURE.md`](../../ARCHITECTURE.md) — active architecture direction after W200-B02 reconciliation.
- [`GUI_DIRECTION.md`](../GUI_DIRECTION.md), [`DNA_CALC_HOST_INTEGRATION.md`](../DNA_CALC_HOST_INTEGRATION.md), [`GUI_PIVOT_CODEBASE_REVIEW.md`](../GUI_PIVOT_CODEBASE_REVIEW.md), [`GUI_WORKSPACE_LAYOUT.md`](../GUI_WORKSPACE_LAYOUT.md), [`GUI_FIXTURES_AND_LAB.md`](../GUI_FIXTURES_AND_LAB.md), and [`GUI_TEST_STRATEGY.md`](../GUI_TEST_STRATEGY.md) — live split planning docs.
- Existing TUI lineage `W010` through `W110` — retained as parked design/prototype evidence.
- OxVba direct host/session surfaces — semantic/project/runtime truth remains in OxVba.
- DnaOneCalc architecture — first exemplar DNA Calc host for embedded IDE consumption.

## Design

### Authority sweep

W200 updates the workset register and adds first-pass GUI pivot docs:

1. [`GUI_DIRECTION.md`](../GUI_DIRECTION.md)
2. [`TUI_PARKING_PLAN.md`](../TUI_PARKING_PLAN.md)
3. [`GUI_PIVOT_CODEBASE_REVIEW.md`](../GUI_PIVOT_CODEBASE_REVIEW.md)
4. [`DNA_CALC_HOST_INTEGRATION.md`](../DNA_CALC_HOST_INTEGRATION.md)
5. [`GUI_TEST_STRATEGY.md`](../GUI_TEST_STRATEGY.md)
6. [`EDITOR_SUBSTRATE_RESEARCH.md`](../EDITOR_SUBSTRATE_RESEARCH.md)
7. [`THIRD_PARTY_RESEARCH_AND_LICENSES.md`](../THIRD_PARTY_RESEARCH_AND_LICENSES.md)

### Code posture

W200 records a strong greenfield bias:

```text
Do not evolve the FrankenTui implementation directly into the GUI product.
Use it as evidence, behavior reference, and prototype history.
Build GUI-native implementation deliberately.
```

### Cross-repo posture

DNA Calc is one coordinated product family. OxIde should consume authoritative cross-repo types and request upstream/sibling changes when that gives a cleaner final design. This repo-scoped agent may write only inside OxIde; sibling changes are captured as handoffs.

### TUI parking

W010-W110 become parked TUI lineage. TUI code, WTD tests, UX lab artifacts, and Fire Horse mockups remain available but should not be the default GUI development path.

### Testing runway

W200 defines the intended test layers:
1. pure Rust unit tests,
2. OxVba contract tests,
3. WASM/browser tests,
4. browser visual/scenario tests through an `oxide-guilab`,
5. host capability matrix tests,
6. eventual DnaOneCalc integration smoke.

## Beads

### W200-B01 — GUI pivot foundation docs and workset registration

**Doctrine.**

- **Goal.** The repo has an explicit W200 planning surface for the GUI pivot: W200 appears in the workset register as the active GUI-pivot foundation, and supporting docs identify the TUI as parked, the current codebase as behavior evidence rather than GUI substrate, and the first implementation/test strategy for the GUI run.
- **Design.** Add this workset packet; update the workset register; add the supporting first-pass docs listed above. No code movement in this bead.
- **Tests.** Doctrine read-through checklist: referenced docs exist; register links W200; parked and active lineage are distinguishable; no code movement/deletion performed.
- **Evidence.** Read-through of touched docs and shell listing/grep of referenced files.
- **Closure.** W200 packet exists; register names active GUI lineage and parked TUI lineage; supporting docs exist and reflect the greenfield-biased pivot.

### W200-B02 — Authority docs reconciled to GUI pivot

**Doctrine.**

- **Goal.** `PRODUCT_DIRECTION.md` and `ARCHITECTURE.md` stop presenting the terminal-native TUI as the active product direction and instead point to the GUI pivot while preserving the TUI as parked lineage.
- **Design.** Rewrite the top-level authority docs around GUI product direction, OxVba truth ownership, DNA Calc host integration, capability profiles, and TUI parking.
- **Tests.** Doctrine read-through checklist; grep for stale primary-direction claims.
- **Evidence.** Read-through notes and stale-claim grep output.

### W200-B03 — Workspace layout preparation

**Infrastructure.**

- **Goal.** The repo is ready for GUI crate work without requiring future beads to untangle the TUI default crate first.
- **Design.** Add [`GUI_WORKSPACE_LAYOUT.md`](../GUI_WORKSPACE_LAYOUT.md) as the live layout preparation plan; defer code movement to later explicit beads; keep tests available; avoid deletion.
- **Tests.** `cargo test` for unaffected default tests; read-through that layout docs preserve TUI parking and greenfield bias.
- **Evidence.** Build/test output and doc reference checks.

### W200-B04 — GUI fixture and lab seed

**Infrastructure.**

- **Goal.** The first GUI worksets have deterministic fixtures and a planned scenario-lab surface.
- **Design.** Add [`GUI_FIXTURES_AND_LAB.md`](../GUI_FIXTURES_AND_LAB.md) to identify fixture projects, future fixture ladder, and `oxide-guilab` scenario ladder. No GUI implementation code in this bead.
- **Tests.** Shell checks for existing fixture files named in the doc; read-through for scenario ladder and capability cases.
- **Evidence.** Fixture list and doc reference checks.

## Out-of-scope

- Full GUI implementation.
- Moving code into the final crate layout beyond what W200-B03 explicitly scopes.
- DnaOneCalc repo changes; those must be handoffs from this repo-scoped agent run.
- OxVba API changes; those must be handoffs unless performed in a separate OxVba-scoped run.
- Deleting any TUI files or directories.
