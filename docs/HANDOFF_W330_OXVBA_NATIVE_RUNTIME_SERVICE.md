# Handoff — W330 OxVba Native Runtime Service Contract

Status: `ready_for_workset_registration`
Date: 2026-05-07
Source workset: W320 — Native Filesystem And Session Persistence

## Decision

W330 should define the OxIde-side contract for a native OxVba runtime/debug/Immediate service before any GUI surface claims real execution.

Paired DnaOneCalc host implementation remains a cross-repo follow-up and requires explicit sibling-repo write authorization. W330 should stay inside OxIde unless the user explicitly authorizes OxVba or other sibling-repo writes.

## Ambition

Replace the current simulated/disabled runtime proof with a contract-ready native runtime seam:

```text
OxIde can render run/debug/Immediate surfaces from a typed runtime-service packet
  -> browser profile remains disabled
  -> native service missing remains disabled
  -> simulated provider remains explicitly simulated
  -> real OxVba execution is not claimed until a native service test exists
```

## Guardrails

1. OxVba remains runtime, debug, Immediate, language, and COM truth owner.
2. Do not route OxIde internal semantics through LSP.
3. Do not duplicate authoritative OxVba DTOs if shared types become available.
4. Do not write to OxVba or DnaOneCalc without explicit user authorization.
5. Do not claim real runtime/debug/Immediate/COM execution without native service tests.
6. Preserve W270 disabled/no-fake-data scenarios and W320 filesystem persistence evidence.
7. Keep browser/WASM limitations explicit.

## Suggested Bead Shape

### W330-B00 — Register runtime service contract workset

- Add `docs/worksets/W330_oxvba_native_runtime_service_contract.md`.
- Update `docs/WORKSET_REGISTER.md` and `docs/worksets/README.md`.
- Use `docs/HANDOFF_OXVBA_RUNTIME_DEBUG_IMMEDIATE_INTERFACES.md` as primary input.

### W330-B01 — Runtime service packet

Define an OxIde-side packet that can represent:

```text
runtime session id
workspace/project correlation
entrypoint/run target list
provider kind: browser-unsupported | simulated | native-service-missing | native-service-available
run command availability and disabled reason
lifecycle/activity/output/error events
native runtime claimed flag
COM runtime claimed flag
```

Tests should prove JSON round-trip and no-claim defaults.

### W330-B02 — Immediate service packet

Define an OxIde-side packet that can represent:

```text
Immediate session id
request text
response/event rows
runtime unavailable/native-service-missing reasons
no fake responses flag
```

Tests should keep browser/native-service-missing profiles disabled with no fake responses.

### W330-B03 — Debug service packet

Define an OxIde-side packet that can represent:

```text
debug session id
running/paused/stopped/unavailable state
continue/break/step availability
callstack frames
locals/watch rows
breakpoint bind status
no fake debug data flag
```

Tests should keep unavailable profiles empty rather than invent callstack/locals/watches.

### W330-B04 — GUI-lab service contract scenarios

Suggested scenario IDs:

```text
gui-runtime-service-contract-browser-disabled
gui-runtime-service-contract-native-missing
gui-immediate-service-contract-native-missing
gui-debug-service-contract-native-missing
```

Required no-claim tokens:

```text
data-native-runtime="false"
data-com-runtime="false"
data-real-execution="false"
data-fake-responses="false"
data-fake-debug-data="false"
```

### W330-B05 — Acceptance and next handoff

- Run full nested workspace tests.
- Render W210-W330 GUI-lab regression scenarios.
- Decide whether W340 should coordinate an OxVba native-service implementation or paired DnaOneCalc host implementation.

## Not Claimed By W330

- real OxVba execution unless a native service test is added,
- COM discovery/invocation,
- DnaOneCalc browser hosting,
- full browser runtime,
- full accessibility audit/compliance.
