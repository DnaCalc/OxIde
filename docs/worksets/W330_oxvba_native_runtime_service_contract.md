# Workset W330 — OxVba Native Runtime Service Contract

## Ambition

OxIde defines a typed, GUI-consumable contract for a future native OxVba runtime/debug/Immediate service. The GUI can render runtime-service packets for browser-disabled, native-service-missing, and simulated profiles without inventing execution, debug, Immediate, or COM data.

W330 is an OxIde-side contract workset. It does not implement OxVba runtime execution or write to sibling repositories unless the user explicitly authorizes that in a later session.

## Dependencies

- W270 — run/debug/Immediate GUI surfaces and no-fake-data disabled states.
- W320 — native filesystem/session persistence proof and no-runtime/COM policy.
- [`docs/HANDOFF_W330_OXVBA_NATIVE_RUNTIME_SERVICE.md`](../HANDOFF_W330_OXVBA_NATIVE_RUNTIME_SERVICE.md).
- [`docs/HANDOFF_OXVBA_RUNTIME_DEBUG_IMMEDIATE_INTERFACES.md`](../HANDOFF_OXVBA_RUNTIME_DEBUG_IMMEDIATE_INTERFACES.md).
- [`docs/GUI_FIXTURES_AND_LAB.md`](../GUI_FIXTURES_AND_LAB.md).

## Guardrails

1. OxVba remains runtime, debug, Immediate, semantic, language-service, and COM truth owner.
2. Do not route OxIde internal semantics through LSP.
3. Do not duplicate authoritative OxVba DTOs if shared types become available.
4. Do not write to OxVba, DnaOneCalc, or other sibling repositories without explicit user authorization.
5. Do not claim real runtime/debug/Immediate/COM execution without native service tests.
6. Preserve W270 disabled/no-fake-data scenarios and W320 filesystem persistence evidence.
7. Keep browser/WASM limitations explicit.
8. Do not import parked TUI runtime/debug/Immediate state or command handlers.

## Design

W330 should add pure packet/projection types in OxIde's GUI crate stack that can consume a future native service while staying honest today. The packets should distinguish:

- browser unsupported,
- simulated provider,
- native service missing,
- future native service available.

The workset should extend current run/debug/Immediate projections rather than replacing them with fake data. GUI-lab scenarios should make missing-native-service reasons visible and keep real execution flags false.

## Scenario Plan

W330 should add deterministic GUI-lab scenarios:

```text
gui-runtime-service-contract-browser-disabled
gui-runtime-service-contract-native-missing
gui-immediate-service-contract-native-missing
gui-debug-service-contract-native-missing
```

Required no-claim tokens include:

```text
data-native-runtime="false"
data-com-runtime="false"
data-real-execution="false"
data-fake-responses="false"
data-fake-debug-data="false"
```

## Beads

### W330-B00 — Register OxVba native runtime service contract workset

Goal:
  Register W330 as the next OxIde-side runtime service contract workset after W320 acceptance.

Design:
  - Add `docs/worksets/W330_oxvba_native_runtime_service_contract.md`.
  - Update `docs/WORKSET_REGISTER.md` and `docs/worksets/README.md`.
  - Use `HANDOFF_W330_OXVBA_NATIVE_RUNTIME_SERVICE.md` and `HANDOFF_OXVBA_RUNTIME_DEBUG_IMMEDIATE_INTERFACES.md` as inputs.
  - Keep sibling repo writes out of scope unless explicitly authorized.

Tests:
  - Documentation review against W270/W320 handoffs and runtime no-claim guardrails.

Evidence:
  - Registered W330 workset and executable bead list.

Closure:
  - [ ] W330 is in the active sequence.
  - [ ] W330 has concrete beads.
  - [ ] Guardrails prevent untested runtime/debug/Immediate/COM claims.

### W330-B01 — Runtime service contract packet

Goal:
  Add an OxIde-side runtime service packet that can represent browser-disabled, simulated, native-service-missing, and future native-service-available run states without claiming untested execution.

Design:
  - Include runtime session identity as optional until a real native service exists.
  - Include workspace/project/run target correlation.
  - Include provider kind and command availability.
  - Include lifecycle/activity/output/error event rows.
  - Include no-claim flags for real execution, native runtime, and COM runtime.

Tests:
  - Packet round-trips through JSON.
  - Browser-disabled and native-service-missing packets keep real execution false.
  - Simulated packet remains explicitly simulated.
  - No COM runtime claim appears in any current packet.

Evidence:
  - `cargo test --manifest-path crates/Cargo.toml -p oxide-core`.

Closure:
  - [ ] Runtime packet is serializable.
  - [ ] Missing native service is distinct from browser unsupported.
  - [ ] Simulated output remains labeled simulated.

### W330-B02 — Immediate service contract packet

Goal:
  Add an OxIde-side Immediate service packet that can represent unavailable/native-service-missing states and future request/response rows without fake responses.

Design:
  - Include optional Immediate session identity.
  - Include request text and response/event rows for future service data.
  - Include command availability and disabled reasons.
  - Include no-fake-responses and no-native-runtime/COM flags for current profiles.

Tests:
  - Browser-disabled packet has no responses.
  - Native-service-missing packet has no responses and a service-missing reason.
  - JSON round-trip preserves no-fake-response flags.

Evidence:
  - `cargo test --manifest-path crates/Cargo.toml -p oxide-core`.

Closure:
  - [ ] Immediate packet is serializable.
  - [ ] Unavailable profiles do not invent responses.
  - [ ] Runtime/COM claims remain false.

### W330-B03 — Debug service contract packet

Goal:
  Add an OxIde-side debug service packet that can represent unavailable/native-service-missing states and future debug rows without fake callstack/locals/watches/breakpoints.

Design:
  - Include optional debug session identity.
  - Include running/paused/stopped/unavailable state.
  - Include command availability for continue/break/step/stop where a future service provides it.
  - Include callstack, locals, watch, and breakpoint rows as empty current-profile vectors.
  - Include no-fake-debug-data and no-native-runtime/COM flags.

Tests:
  - Browser-disabled and native-service-missing packets have empty debug data.
  - JSON round-trip preserves debug state and no-fake-data flags.
  - No runtime or COM claim appears.

Evidence:
  - `cargo test --manifest-path crates/Cargo.toml -p oxide-core`.

Closure:
  - [ ] Debug packet is serializable.
  - [ ] Unavailable profiles do not invent debug data.
  - [ ] Runtime/COM claims remain false.

### W330-B04 — Runtime service contract GUI-lab scenarios

Goal:
  Render deterministic GUI-lab scenarios for runtime, Immediate, and debug service contract packets.

Design:
  - Add `gui-runtime-service-contract-browser-disabled`.
  - Add `gui-runtime-service-contract-native-missing`.
  - Add `gui-immediate-service-contract-native-missing`.
  - Add `gui-debug-service-contract-native-missing`.
  - Render provider kind, service availability, no-claim flags, and no-fake-data states.

Tests:
  - Scenario registry finds all W330 scenario IDs.
  - Runtime browser-disabled and native-missing scenarios keep real execution false.
  - Immediate native-missing scenario keeps fake responses false.
  - Debug native-missing scenario keeps fake debug data false.

Evidence:
  - `cargo test --manifest-path crates/Cargo.toml -p oxide-guilab`.
  - Render commands for the W330 scenarios.

Closure:
  - [ ] GUI-lab can review runtime-service contract states.
  - [ ] Browser and native-service-missing limitations are visible.
  - [ ] Runtime/COM and fake-data claims remain false.

### W330-B05 — W330 acceptance and next handoff

Goal:
  Accept W330 and decide whether W340 should coordinate an OxVba native-service implementation or paired DnaOneCalc host implementation.

Design:
  - Update `docs/GUI_FIXTURES_AND_LAB.md` with W330 scenario tokens.
  - Add a next-workset handoff.
  - Preserve W210-W330 regression renders.

Tests:
  - `cargo test --manifest-path crates/Cargo.toml --workspace`.
  - Render W210-W330 GUI-lab scenarios.
  - Grep runtime-service, Immediate, debug, no-real-execution, no-fake-data, no-COM tokens.

Evidence:
  - Full nested workspace tests.
  - Rendered GUI-lab outputs.
  - Handoff note.

Closure:
  - [ ] W330 accepted or explicitly blocked with evidence.
  - [ ] W210-W330 regression scenarios pass.
  - [ ] Next workset prerequisites are documented.

## Out-of-scope

- Writing to the OxVba repository without explicit authorization.
- Real OxVba runtime/debug/Immediate execution.
- Native COM discovery or invocation.
- DnaOneCalc host implementation.
- Full browser runtime/DOM accessibility audit.
- Parked TUI runtime/debug/Immediate substrate changes.
