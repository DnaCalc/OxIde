# OxIde Host Bridge Service Map

Status: `w343_service_map`
Date: 2026-05-07
Planned crate: `oxide-host-bridge`
Primary consumers: `oxide-ui-leptos`, **DnaOxIde / DNA OxIde**, DnaOneCalc host mounts, browser/WASM review fixtures

## Purpose

The OxIde host bridge facade lets shared UI issue host-neutral requests without depending on Tauri, DnaOxIde app code, DnaOneCalc product code, or OxVba implementation details.

The facade separates every service into one of four evidence states:

1. **proven-oxide-only** — implementable now from OxIde state and tests;
2. **oxvba-available-subset** — a current direct OxVba Rust surface exists, but DnaOxIde hardening or OxIde adapter evidence is still partial;
3. **oxvba-fixture-evidenced** — OxVba has ThinSliceHello fixture evidence, but OxIde still needs local adapter tests before UI claims can flip;
4. **pending-oxvba-hardening** — the UI/command exists but must return unavailable/pending states until OxVba closes or OxIde adopts the tracked gap.

## Service Categories

| Service category | Purpose | Current state | Authoritative owner |
| --- | --- | --- | --- |
| `HostProjectApi` | open/list/inspect workspace/project/module roster | `proven-oxide-only` for fixture packets; `oxvba-available-subset` for `HostWorkspaceSession` / `inspect_workspace_target` | OxVba project truth; OxIde host orchestration |
| `HostDocumentApi` | load/save/reload active documents and sessions | `proven-oxide-only` for W320 filesystem/session persistence | OxIde host policy for persistence; OxVba document identity |
| `HostLanguageApi` | diagnostics, hover, symbols, definition, references | `oxvba-available-subset` through `HostWorkspaceSession` and language-service direct APIs | OxVba |
| `HostCompileApi` | compile options, build/check, run target selection | build/check is `oxvba-fixture-evidenced`; compile options/run targets are `pending-oxvba-hardening` | OxVba |
| `HostReferenceApi` | active references, COM candidates, reference plans | COM selection and capability profile are `oxvba-fixture-evidenced`; reorder/native boundary adoption is pending | OxVba |
| `HostRuntimeApi` | run/reset/invoke/stop lifecycle | run/session creation and runtime IDs are `oxvba-fixture-evidenced`; event/source-span/command availability adoption is pending | OxVba |
| `HostImmediateApi` | Immediate request/response against active runtime | attach/evaluation over overlay source is `oxvba-fixture-evidenced`; host UX/taxonomy adoption is pending | OxVba |
| `HostDebugApi` | debug attach, continue/step/stop, locals/callstack/watches/breakpoints | debug attach, watch registry/evaluation, breakpoint binding DTOs, and stable frame/watch/breakpoint IDs are `oxvba-fixture-evidenced`; full host UX/source mapping adoption is pending | OxVba |
| `HostSettingsApi` | host preferences, app settings, UI policy | `proven-oxide-only` placeholders only | Host app policy |
| `HostCapabilityApi` | capability profile and disabled reasons across all services | `proven-oxide-only` for existing OxIde packets; shared stable taxonomy is `pending-oxvba-hardening` | Shared contract/OxVba + OxIde presentation |

## Shared UI Command Map

| Shared UI command | Host API category | W343 state | Notes |
| --- | --- | --- | --- |
| `project.open` | `HostProjectApi` | `proven-oxide-only` / `oxvba-available-subset` | DnaOxIde can open paths; OxVba owns project identity |
| `project.inspect` | `HostProjectApi` | `oxvba-available-subset` | maps to project/session roster surfaces |
| `document.save` | `HostDocumentApi` | `proven-oxide-only` | W320 native filesystem/session proof covers temp project copies |
| `document.reload` | `HostDocumentApi` | `proven-oxide-only` | W320 proof; browser profile remains disabled |
| `document.revert` | `HostDocumentApi` | `proven-oxide-only` | pure state command |
| `language.diagnostics` | `HostLanguageApi` | `oxvba-available-subset` | direct language service; no LSP routing |
| `language.hover` | `HostLanguageApi` | `oxvba-available-subset` | direct language service |
| `language.definition` | `HostLanguageApi` | `oxvba-available-subset` | direct language service |
| `language.references` | `HostLanguageApi` | `oxvba-available-subset` | direct language service |
| `compile.options` | `HostCompileApi` | `pending-oxvba-hardening` | project properties/compile options DTOs pending |
| `compile.check` | `HostCompileApi` | `oxvba-fixture-evidenced` | overlay build through `EmbeddedBuildRunHost::build_workspace` has ThinSliceHello evidence |
| `references.show` | `HostReferenceApi` | `oxvba-fixture-evidenced` | broken COM reference state and capability profile have ThinSliceHello evidence |
| `references.com.search` | `HostReferenceApi` | `oxvba-available-subset` / `oxvba-fixture-evidenced` | `ComSelectionService` subset plus capability-profile evidence; native boundary adoption pending |
| `runtime.run` | `HostRuntimeApi` | `oxvba-fixture-evidenced` | runtime session creation and stable runtime IDs have ThinSliceHello evidence; OxIde adapter tests still pending |
| `runtime.stop` | `HostRuntimeApi` | `pending-oxvba-hardening` | stop/cancel availability pending |
| `runtime.immediate` | `HostImmediateApi` | `oxvba-fixture-evidenced` | `EmbeddedRunSession::into_immediate_session` and `ImmediateSession` evaluation have ThinSliceHello evidence |
| `runtime.debug` | `HostDebugApi` | `oxvba-fixture-evidenced` | `EmbeddedRunSession::into_debug_session` has ThinSliceHello evidence |
| `debug.continue` | `HostDebugApi` | `oxvba-available-subset` | direct debug subset exists; host adapter evidence pending |
| `debug.step_into` | `HostDebugApi` | `oxvba-available-subset` | direct debug subset exists; host adapter evidence pending |
| `debug.step_over` | `HostDebugApi` | `oxvba-available-subset` | direct debug subset exists; host adapter evidence pending |
| `debug.step_out` | `HostDebugApi` | `oxvba-available-subset` | direct debug subset exists; host adapter evidence pending |
| `watch.upsert` | `HostDebugApi` | `oxvba-fixture-evidenced` | `DebugSession::add_watch` and `evaluate_watches` have ThinSliceHello evidence |
| `breakpoint.set` | `HostDebugApi` | `oxvba-fixture-evidenced` | `DebugSession::set_source_breakpoint` binding DTO has ThinSliceHello evidence |
| `settings.open` | `HostSettingsApi` | `proven-oxide-only` placeholder | host policy, no OxVba truth |
| `capability.show` | `HostCapabilityApi` | `proven-oxide-only` / `pending-oxvba-hardening` | existing packets now; stable taxonomy pending |
| `shell.command_palette` | `HostCapabilityApi` | `proven-oxide-only` | presentation command only |

## OxVba Available-Subset Adapter Targets

Confirmed current OxVba direct Rust surfaces that W343 should prepare adapters for:

- `HostWorkspaceSession`,
- `inspect_workspace_target`,
- `ComSelectionService`,
- `EmbeddedBuildRunHost`,
- `EmbeddedRunSession`,
- `ImmediateSession`,
- `DebugSession`.

These names identify adapter targets only. `oxide-host-bridge` should not duplicate final OxVba DTO ownership.

## OxVba Fixture-Evidenced Adapter Targets

OxVba published `../OxVba/docs/evidence/DNAOXIDE_THIN_SLICE_HELLO_FIXTURE_2026-05-07.md` for bead `bd-avdu.6.1`.

Fixture-evidenced direct-host seams now include:

- `HostWorkspaceSession::load_workspace_path`,
- `HostWorkspaceSession::set_document_text`,
- `workspace_roster`,
- `EmbeddedBuildRunHost::build_workspace`,
- `EmbeddedBuildRunHost::run_project`,
- `EmbeddedRunSession::into_immediate_session`,
- `ImmediateSession` overlay evaluation,
- `EmbeddedRunSession::into_debug_session`,
- `DebugSession::add_watch`,
- `DebugSession::evaluate_watches`,
- `DebugSession::set_source_breakpoint`,
- stable frame/watch/breakpoint/runtime IDs in returned DTOs,
- `ComSelectionService::inspect_workspace_project_state`,
- `ComSelectionService::capability_profile`.

OxIde must still add local adapter tests before any full DnaOxIde runtime/debug/Immediate/COM claim is flipped.

## Pending-Hardening Gates

The host bridge must keep these as unavailable/partial until OxVba closes the corresponding work or OxIde adapter tests verify adoption:

- OxIde adapter adoption for stable workspace/project/module/runtime/debug/watch/breakpoint IDs,
- shared capability/error taxonomy,
- unified workspace/project/module DTO with revisions and `Attribute VB_Name` state,
- project properties / compile options / run target DTOs,
- build/run request IDs,
- build/run lifecycle event stream,
- command availability DTOs,
- runtime error source-span mapping,
- Immediate/debug attach UX over `EmbeddedRunSession`,
- watch registry DTO adoption,
- breakpoint bind/unbind/source-remap DTO adoption,
- COM capability profile adoption,
- reference reorder plans,
- COM bitness/apartment/native boundary status,
- COM runtime invocation evidence,
- OxIde adapter evidence over the ThinSliceHello fixture ladder for each claim.

## Ownership Boundaries

- **OxIde** owns IDE UI state, shared component rendering, command presentation, no-claim labels, fixture/test harnesses, and DnaOxIde host orchestration.
- **DnaOxIde** owns the standalone Windows desktop product shell, Tauri packaging/configuration, and host policy.
- **DnaOneCalc** owns its product shell, placement, and persistence policy when it consumes the shared IDE surface.
- **OxVba** owns project, language, compile/build, runtime, Immediate, debug, watch, breakpoint, COM/reference, and source mapping truth.

## No-Claim Rules

The host bridge must not turn subset, fixture-evidenced, or pending responses into full capability claims.

Required defaults until proven otherwise:

- real execution claimed: `false`,
- native runtime claimed: `false`,
- COM runtime claimed: `false`,
- fake Immediate responses: `false`,
- fake debug data: `false`,
- DnaOneCalc real host mount: `false`.

## Verification

W343-B00 verification checks should include:

```powershell
rg -n "HostProjectApi|HostDocumentApi|HostLanguageApi|HostCompileApi|HostReferenceApi|HostRuntimeApi|HostImmediateApi|HostDebugApi|HostSettingsApi|HostCapabilityApi" docs/HOST_BRIDGE_SERVICE_MAP.md
rg -n "proven-oxide-only|oxvba-available-subset|oxvba-fixture-evidenced|pending-oxvba-hardening|HostWorkspaceSession|inspect_workspace_target|ComSelectionService|EmbeddedBuildRunHost|EmbeddedRunSession|ImmediateSession|DebugSession|DNAOXIDE_THIN_SLICE_HELLO_FIXTURE" docs/HOST_BRIDGE_SERVICE_MAP.md
rg -n "apps/dna-oxide|@tauri-apps|window.__TAURI__|invoke\(" crates
```

The final command is expected to find no implementation dependency from shared crates to app/Tauri host APIs.
