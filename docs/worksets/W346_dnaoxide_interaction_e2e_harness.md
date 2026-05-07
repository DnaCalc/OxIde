# Workset W346 — DnaOxIde Interaction And E2E Harness

## Ambition

Add the first deterministic interaction/e2e harness for **DnaOxIde**, starting with command/focus/save-reload flows that are already proven in pure state and host-command tests.

This workset moves beyond static render checks without claiming full accessibility audit, real browser runtime semantics, or real OxVba runtime/debug/COM capabilities.

## Dependencies

- W280 — command, keyboard, focus, and accessibility polish.
- W300 — parsed HTML DOM smoke.
- W344 — Tauri command boundary stubs.
- W345 — DnaOxIde live host UI proof.
- [`docs/HANDOFF_DNAOXIDE_OXVBA_FEEDBACK_ALIGNMENT.md`](../HANDOFF_DNAOXIDE_OXVBA_FEEDBACK_ALIGNMENT.md).

## Design

W346 should choose the lightest reliable driver available in the repo/toolchain. Acceptable layers, in order of maturity:

1. Pure component event tests.
2. Static DOM smoke over rendered HTML.
3. Tauri command integration tests.
4. Tauri/WebView smoke.
5. Later Playwright/WebDriver-style click/key automation.

Initial flows:

- command palette opens/closes,
- keyboard shortcut dispatch maps to commands,
- focus graph/no-mouse route remains valid,
- project open command reaches host bridge fixture/stub,
- save/reload/session commands operate on temp projects,
- blocked runtime/Immediate/debug/COM commands show disabled reasons,
- available-subset OxVba adapter commands can be exercised only after direct adapter tests exist and must show subset labels.

## Beads

### W346-B00 — Harness layer decision

Goal:
  Choose the first interaction harness layer based on available toolchain and determinism.

Design:
  - Prefer a layer that can run in CI/local without flaky dependencies.
  - Document what the harness drives and what it does not drive.

Tests:
  - Documentation grep for selected harness layer and claim boundaries.

Evidence:
  - Harness decision note.

Closure:
  - [ ] Harness layer is selected.
  - [ ] Claim boundaries are explicit.
  - [ ] First flows are listed.

### W346-B01 — Command palette and keyboard interaction tests

Goal:
  Prove command palette and keyboard command routing in the host/UI layer.

Design:
  - Reuse W280 command/keyboard maps.
  - Test command availability and disabled reasons.
  - Avoid parked TUI command substrate.

Tests:
  - Component/host interaction tests for command palette and shortcuts.

Evidence:
  - `target/w346-command-keyboard-tests.txt`.

Closure:
  - [ ] Command palette interaction is tested.
  - [ ] Keyboard routing is tested.
  - [ ] Disabled runtime/debug/COM commands remain disabled.

### W346-B02 — Focus/no-mouse interaction tests

Goal:
  Prove no-mouse navigation/focus route in the host/UI layer.

Design:
  - Reuse W280 focus graph.
  - Test route through project, editor, diagnostics, lifecycle, command palette, and capability panels.

Tests:
  - Focus route tests.
  - Static DOM/focus markers if available.

Evidence:
  - `target/w346-focus-route-tests.txt`.

Closure:
  - [ ] No-mouse route is tested.
  - [ ] Focus labels are visible.
  - [ ] No full accessibility audit is claimed.

### W346-B03 — Host lifecycle interaction tests

Goal:
  Exercise open/save/reload/session interactions through the host command path.

Design:
  - Use temp project copies.
  - Drive the same commands a user would invoke.
  - Verify UI state changes dirty/clean/restored.

Tests:
  - Host lifecycle e2e/integration tests.
  - Fixture mutation guard.

Evidence:
  - `target/w346-host-lifecycle-interaction-tests.txt`.

Closure:
  - [ ] Open/save/reload/session interactions are tested.
  - [ ] Fixtures are unchanged.
  - [ ] Results are deterministic.

### W346-B04 — Blocked and subset-backed service interaction tests

Goal:
  Prove blocked service commands and available-subset adapter commands behave honestly in the interactive host path.

Design:
  - Trigger run, Immediate, debug, COM/reference commands.
  - Assert disabled/unavailable reasons for pending-hardening gaps.
  - Assert subset labels for any adapter-backed results.
  - Assert empty fake-data surfaces where full data is unavailable.

Tests:
  - Interaction tests for blocked commands.
  - Interaction tests for subset-backed commands where direct adapter evidence exists.
  - Anti-overclaim scan.

Evidence:
  - `target/w346-blocked-service-interaction-tests.txt`.

Closure:
  - [ ] Runtime/Immediate/debug/COM commands show unavailable or subset-backed states.
  - [ ] No fake data is produced.
  - [ ] Full capability claim flags remain false until matching evidence exists.

### W346-B05 — W346 acceptance

Goal:
  Accept the interaction harness as the host regression base.

Design:
  - Update GUI test strategy docs.
  - Link W347 placeholder panel work.

Tests:
  - Interaction harness suite.
  - Workspace tests.

Evidence:
  - W346 acceptance outputs.

Closure:
  - [ ] Harness is documented.
  - [ ] First interactions are covered.
  - [ ] No unsupported capability is claimed.

## Out-of-scope

- Full accessibility compliance audit.
- Full browser runtime claims beyond what the harness drives.
- Real OxVba runtime/debug/Immediate/COM behavior.
- Installer smoke.
