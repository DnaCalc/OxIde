# DNA OxIde / OxVba Integration Readiness Report

Status: `w349_readiness_report`
Date: 2026-05-07
Source workset: W349 — DnaOxIde While-OxVba Acceptance

## Summary

W341-W348 have built the OxIde-side runway for a standalone **DNA OxIde / DnaOxIde** host while OxVba hardens full-scope host APIs. The runway is ready for direct Rust adapter work, but it does **not** yet claim full native runtime, Immediate, debug, watch, breakpoint, or COM runtime behavior inside OxIde/DnaOxIde.

Current OxIde state:

- `apps/dna-oxide/` owns the standalone host scaffold and static frontend proof path.
- `oxide-ui-leptos` owns reusable shared IDE rendering.
- `oxide-host-bridge` owns host-neutral service categories, command catalog, availability states, disabled reasons, and no-claim defaults.
- DnaOxIde command stubs expose proven project/document/session flows and typed unavailable/subset/fixture-evidenced service packets.
- Compile/options/reference/COM panels exist as honest placeholders ready for OxVba DTO adoption.
- DnaOneCalc reuse is proven inside OxIde only, with no sibling repo writes and no real DnaOneCalc mount claim.

## Evidence Buckets

The active readiness vocabulary remains:

1. `proven-oxide-only` — implemented and tested inside OxIde without claiming OxVba runtime truth.
2. `oxvba-available-subset` — a direct OxVba Rust surface exists, but OxIde adapter or DTO adoption is incomplete.
3. `oxvba-fixture-evidenced` — OxVba ThinSliceHello fixture evidence exists, but OxIde still needs direct adapter tests before full host claims can flip.
4. `pending-oxvba-hardening` — DTO/event/taxonomy/source-span/native-boundary adoption remains pending.
5. `unavailable-no-claim` — no usable provider is configured and UI/commands must stay disabled.

## Minimum Delivery Order Mapping

### W355 — Compile/build UX

OxVba-side target:

- project properties,
- compile options,
- run targets,
- typed build/check request IDs,
- command availability,
- lifecycle events and diagnostics.

OxIde/DnaOxIde readiness:

- `docs/DNAOXIDE_COMPILE_REFERENCE_PANEL_CONTRACT.md` defines the placeholder contract.
- `apps/dna-oxide/src/placeholder-panels.js` renders project properties, compile options, build/check, and run-target panels.
- `apps/dna-oxide/src-tauri/src/commands.rs` has command stubs for compile/build/check surfaces.
- `oxide-host-bridge` maps `compile.options` as `pending-oxvba-hardening` and `compile.check` as `oxvba-fixture-evidenced`.
- `target/w347-acceptance.txt` and `target/w349-regression.txt` prove the panels/checks remain no-claim.

Next OxIde adapter work:

- adopt OxVba `HostProjectCompileOptionsSurface`, settings edit plans, run target DTOs, build request IDs, build lifecycle events, and typed diagnostics into DnaOxIde command packets;
- add OxIde-side direct adapter tests over temp project copies;
- keep checked-in fixtures immutable.

### W360 — COM references and native boundary

OxVba-side target:

- active reference roster,
- COM candidate discovery,
- add/repair/replace/remove/reorder plans,
- COM capability profile,
- COM runtime invocation availability,
- platform-specific unavailable status.

OxIde/DnaOxIde readiness:

- `docs/HOST_BRIDGE_SERVICE_MAP.md` maps `references.show` and `references.com.search` to reference/COM host APIs.
- W347 renders references, COM candidate, repair, and COM runtime boundary panels.
- `apps/dna-oxide/src-tauri/src/commands.rs` returns typed COM/reference unavailable or fixture-evidenced packets.
- `data-com-runtime-invocation="false"` remains required in W347/W349 evidence.

Next OxIde adapter work:

- adopt OxVba `ComSelectionService`, `ComCapabilityProfile`, `ComRuntimeInvocationAvailability`, and reference edit/reorder DTOs;
- test Windows and non-Windows/degraded capability paths separately;
- do not claim COM runtime invocation until DnaOxIde has local native-boundary/runtime evidence.

### W365 — Runtime + Immediate

OxVba-side target:

- live runtime session creation,
- stable runtime session IDs,
- run lifecycle events,
- reset/invoke command availability,
- Immediate attach/session IDs,
- typed Immediate responses without fake data.

OxIde/DnaOxIde readiness:

- `oxide-host-bridge` maps `runtime.run` and `runtime.immediate` as `oxvba-fixture-evidenced` adapter targets.
- W344 service command packets return `RuntimeServicePacket` and `ImmediateServicePacket` native-service-missing states.
- W345/W346 host UI and interaction checks expose runtime/Immediate panes and blocked commands with no fake responses.
- W349 regression verifies `data-native-runtime="false"`, `data-fake-responses="false"`, and disabled command states.

Next OxIde adapter work:

- adopt OxVba `EmbeddedBuildRunHost`, `EmbeddedRunSession`, `EmbeddedRunSessionCommandStatus`, `EmbeddedRunSession::into_immediate_session`, `ImmediateSession`, and typed Immediate result DTOs;
- add direct OxIde adapter tests against temp project copies using the ThinSliceHello ladder;
- only then consider flipping DnaOxIde runtime/Immediate claim flags.

### W370 — Debug, watches, breakpoints

OxVba-side target:

- debug attach/session IDs,
- command states for continue/step/stop,
- callstack/locals projection,
- watch registry and evaluation states,
- source breakpoint bind/unresolved DTOs,
- stable frame/watch/breakpoint IDs,
- source-span mapping breadth.

OxIde/DnaOxIde readiness:

- `oxide-host-bridge` maps debug attach, continue/step, watch, and breakpoint commands to `oxvba-fixture-evidenced` adapter targets.
- W344 debug command packets return typed disabled/unavailable states.
- W345 service panes show zero callstack, locals, watches, and breakpoints without fake data.
- W346 interaction checks verify blocked debug/watch/breakpoint commands.
- W349 regression verifies `data-fake-debug-data="false"` and disabled debug command states.

Next OxIde adapter work:

- adopt OxVba `DebugSession`, `DebugSessionCommandStatus`, watch records/evaluations, breakpoint binding records, pause state, frame/local DTOs, and source-span mapping;
- add OxIde-side adapter tests that prove each pane is backed by OxVba DTOs rather than static placeholder rows;
- leave full debug UX/source-span breadth pending until the adapter tests cover it.

## DnaOneCalc Reuse Gate

W348 proves shared UI reuse inside OxIde with `dnaonecalc-consumer`, `DnaOneCalcWebShellHostPacket`, `oxide-ui-leptos`, and `oxide-host-bridge`.

It does not authorize writing to `C:/Work/DnaCalc/DnaOneCalc`, and it does not claim a real DnaOneCalc product mount. A paired DnaOneCalc implementation requires explicit user authorization and separate evidence.

## Current Blockers Before Full Claims

OxIde/DnaOxIde cannot yet claim:

- live Tauri/WebView IPC execution,
- browser click/key automation or Playwright/WebDriver coverage,
- full DOM accessibility audit,
- real/native OxVba runtime execution in DnaOxIde,
- real Immediate Window evaluation from DnaOxIde,
- real debug/watch/breakpoint panes from DnaOxIde,
- COM runtime invocation,
- real DnaOneCalc mount,
- fake runtime/Immediate/debug rows as substitutes for missing data.

Required next work is a direct OxIde adapter workset over the OxVba direct Rust surfaces. The adapter tests must prove temp-project direct consumption before any claim flags are changed from false.

## Recommended Next Workset

If OxVba direct APIs are considered stable enough for OxIde consumption, start a new OxIde workset for direct adapter adoption in this order:

1. W355 compile/build adapter: project compile options + build/check request/event packets.
2. W360 COM/reference adapter: reference roster + COM capability/profile packets with platform gates.
3. W365 runtime/Immediate adapter: runtime session + Immediate attach/evaluate packets.
4. W370 debug/watch/breakpoint adapter: debug session + watch/breakpoint/source mapping packets.

If OxVba APIs are not ready for local adapter tests, continue only DnaOxIde packaging/polish and keep all runtime/debug/Immediate/COM claim flags false.
