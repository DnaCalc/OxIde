# Workset W260 — Windows COM Capability Proof

## Ambition

COM-dependent OxVba projects are handled honestly: browser and non-Windows hosts show unavailable states, while the Windows-native path has an explicit, designed, and testable capability route before any COM runtime invocation is claimed.

W260 is not a full COM implementation. It proves the capability matrix and native-service seam needed before W270 run/debug/Immediate surfaces can safely mention COM-capable execution.

## Dependencies

- W240 — capability-aware run/output path.
- W250 — DnaOneCalc embedding contract and current regression ladder.
- [`PRODUCT_DIRECTION.md`](../../PRODUCT_DIRECTION.md) §6 and §7.
- [`ARCHITECTURE.md`](../../ARCHITECTURE.md) §7 and §8.
- [`docs/HANDOFF_W260_WINDOWS_COM_CAPABILITY_PROOF.md`](../HANDOFF_W260_WINDOWS_COM_CAPABILITY_PROOF.md).
- OxVba COM reference/runtime contracts or handoff when authoritative project/runtime APIs are missing.

## Design

### Capability distinctions

W260 must keep these concepts separate:

1. **COM reference present** — project/source fact owned by OxVba.
2. **COM reference discovery** — host/platform capability, usually native Windows only.
3. **COM runtime invocation** — native Windows runtime/service capability.
4. **Native execution provider** — prerequisite for real run/output, but not identical to COM runtime availability.
5. **Browser/WASM rendering** — may show COM facts and unavailable reasons, but cannot directly call COM.

### First implementation stance

The first implementation should add OxIde-owned capability projections and lab evidence. It should not:

1. parse or invent OxVba project-reference semantics,
2. call Windows COM,
3. claim native execution,
4. mutate sibling repos,
5. treat W240 simulated output as COM evidence.

A real COM-reference fixture should be added only when the consuming test/scenario can state its source of truth. If OxVba does not expose the needed reference facts yet, W260 should create a handoff rather than duplicate them.

### Planned GUI-lab scenarios

W260 should introduce deterministic scenarios:

```text
gui-com-reference-browser-unavailable
gui-com-reference-native-service-missing
```

The browser-unavailable scenario should show:

1. COM reference present as a projected project fact,
2. browser-safe profile,
3. COM discovery unavailable,
4. COM runtime unavailable,
5. Windows native host required,
6. no native execution or COM runtime claim.

The native-service-missing scenario should show:

1. Windows native host profile,
2. COM reference present,
3. native execution profile admitted,
4. COM discovery/runtime blocked because the native COM service is not configured,
5. disabled reasons and handoff pointer.

## Beads

### W260-B00 — Expand Windows COM capability proof workset

Goal:
  Make W260 executable by replacing the scaffold with concrete vertical beads and explicit COM capability distinctions.

Design:
  - Record reference/discovery/runtime/native-execution separation.
  - Name first lab scenarios and limitations.
  - Preserve browser/WASM COM-unavailable honesty.

Tests:
  - Documentation review against product/architecture/W260 handoff docs.

Evidence:
  - Expanded `docs/worksets/W260_windows_com_capability_proof.md`.

Closure:
  - [ ] W260 has concrete beads.
  - [ ] Browser/native/COM distinctions are explicit.
  - [ ] Native execution is not claimed prematurely.

### W260-B01 — Pure COM capability matrix model

Goal:
  Add pure OxIde capability state for COM reference presence, discovery, runtime invocation, and disabled reasons.

Design:
  - Add `ComReferenceFact`, `ComCapabilityProfile`, and status/feature types in `oxide-core` or another OxIde-owned pure crate.
  - Distinguish browser-unavailable, non-Windows-unavailable, Windows-native-service-missing, and future Windows-native-service-available profiles.
  - Keep the model as a UI/capability projection, not an OxVba project semantic model.

Tests:
  - Browser-safe profile reports discovery/runtime unavailable and Windows native required.
  - Non-Windows native profile reports COM unavailable without claiming browser limitations.
  - Windows native service missing reports native host but blocked COM service.
  - Future service available profile is labeled separately and not used as browser proof.

Evidence:
  - `cargo test --manifest-path crates/Cargo.toml -p oxide-core`.

Closure:
  - [ ] COM capability matrix is pure and deterministic.
  - [ ] Reference/discovery/runtime distinctions are tested.
  - [ ] No OxVba COM enums or DTOs are copied.

### W260-B02 — Browser/non-Windows COM unavailable lab scenarios

Goal:
  Render COM-reference-present unavailable states in the GUI lab without claiming native support.

Design:
  - Add `gui-com-reference-browser-unavailable`.
  - Optionally add or reserve `gui-com-reference-nonwindows-unavailable` if the model supports it cleanly in the same bead.
  - Reuse current lab style and capability footer.
  - Render COM reference fact, discovery status, runtime status, disabled reasons, and `Windows native host required`.

Tests:
  - Scenario registry finds new scenario(s).
  - Rendered output contains COM reference present, browser-safe profile, discovery/runtime unavailable, Windows native required, and no COM available/native execution claim.
  - Existing W210-W250 scenario tests remain green.

Evidence:
  - `cargo test --manifest-path crates/Cargo.toml -p oxide-guilab`.
  - `cargo run --manifest-path crates/Cargo.toml -p oxide-guilab -- render gui-com-reference-browser-unavailable`.

Closure:
  - [ ] Browser COM unavailable scenario is registered.
  - [ ] No native/COM support is claimed in browser mode.
  - [ ] Regression scenarios remain intact.

### W260-B03 — Windows native COM service-missing contract

Goal:
  Model and render the Windows-native path as an explicit service seam that can be missing/blocked before any real COM call.

Design:
  - Add a native-service-missing capability profile.
  - Render `gui-com-reference-native-service-missing`.
  - Show Windows native host admitted, native COM service not configured, COM discovery/runtime blocked, and a handoff-required reason.
  - Keep runtime invocation disabled until a tested native service exists.

Tests:
  - Pure model test for Windows native service missing.
  - GUI-lab render test for service-missing scenario.
  - Render command for `gui-com-reference-native-service-missing`.

Evidence:
  - `cargo test --manifest-path crates/Cargo.toml --workspace`.
  - `cargo run --manifest-path crates/Cargo.toml -p oxide-guilab -- render gui-com-reference-native-service-missing`.

Closure:
  - [ ] Windows native service seam is visible.
  - [ ] Missing service does not become a generic failure.
  - [ ] No COM invocation is claimed.

### W260-B04 — OxVba/native COM handoff for authoritative runtime gaps

Goal:
  Capture any required OxVba/native-service interface work instead of duplicating COM project/runtime truth locally.

Design:
  - Document authoritative inputs needed from OxVba: COM reference facts, discovery status, service/run contracts, and error taxonomy.
  - State what OxIde can render today versus what it cannot prove without native service tests.
  - Add handoff for native Windows service ownership decision if needed.

Tests:
  - Documentation review.
  - Full nested workspace tests remain green.

Evidence:
  - Handoff note.
  - `cargo test --manifest-path crates/Cargo.toml --workspace`.

Closure:
  - [ ] Required OxVba/native interfaces are specific.
  - [ ] OxIde limitations are explicit.
  - [ ] No local duplicate COM runtime model is introduced.

### W260-B05 — W260 acceptance and W270 handoff

Goal:
  Accept W260 with regression evidence and prepare W270 run/debug/Immediate GUI surfaces.

Design:
  - Update GUI fixture/lab docs with W260 scenarios and expected tokens.
  - Update this workset with acceptance evidence.
  - Add W270 handoff note clarifying which run/debug/Immediate surfaces may mention COM capability.

Tests:
  - `cargo test --manifest-path crates/Cargo.toml --workspace`.
  - Render W210-W260 GUI-lab scenarios.

Evidence:
  - Test output and rendered scenario token checks.
  - W270 handoff note.

Closure:
  - [ ] W260 accepted or explicitly blocked with evidence.
  - [ ] W270 prerequisites documented.
  - [ ] Browser/native/COM limitations remain explicit.

## Out-of-scope

- Full COM runtime parity.
- Actual Windows COM invocation without a tested native service.
- Non-Windows COM substitutes.
- Browser-only COM execution.
- Making OxIde own OxVba COM project/runtime truth.
- Debugger and Immediate Window GUI surfaces; these belong to W270.
