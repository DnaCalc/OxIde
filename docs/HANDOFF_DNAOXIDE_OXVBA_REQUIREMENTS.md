# Handoff — DNA OxIde OxVba Requirements

Status: `cross_repo_requirements_note`
Date: 2026-05-07
Source lane: DnaOxIde / **DNA OxIde** fast-track standalone host
Target audience: OxVba maintainers and OxIde/DnaOxIde implementers

## Purpose

**DNA OxIde** is intended to reach full standalone Windows desktop IDE scope soon. To do that honestly, OxIde/DnaOxIde needs OxVba to expose and stabilize direct, typed host APIs for project, language-service, build/run, Immediate, debug, watch, breakpoint, and COM/reference workflows.

This note is an OxIde-side requirements handoff. It does not write to the OxVba repo and does not claim that OxIde already consumes these capabilities end-to-end.

## Product Pressure

DnaOxIde is not just another static shell proof. The desired near-term product shape is:

- open real `.basproj` workspaces,
- edit modules with OxVba diagnostics and language features,
- view/edit compile/project options,
- build/check with typed diagnostics,
- select and repair references, including Windows COM/type libraries,
- run projects through a native OxVba runtime session,
- use the Immediate Window against the live session,
- debug with callstack, locals, watches, breakpoints, stepping, and source mapping,
- package as a Windows desktop app with the same shared IDE UI reusable by DnaOneCalc.

The browser/WASM profile remains useful for review and limited workflows, but it must not set the pace for full native Windows IDE capability.

## Ownership Rules

OxVba owns:

- `.basproj` and project semantics,
- module/reference authoring semantics,
- language-service truth,
- compile/build truth,
- runtime/session truth,
- Immediate evaluation truth,
- debug/watch/breakpoint truth,
- COM discovery/reference/runtime truth,
- source span mapping from semantic/runtime locations to project documents.

OxIde/DnaOxIde owns:

- UI layout and command routing,
- editor widgets and pane composition,
- host lifecycle UX,
- save/reload/session policy,
- Tauri/native-shell wiring,
- rendering OxVba DTOs into panels, command palettes, timelines, and dialogs,
- preserving capability/no-claim states when OxVba services are unavailable.

DnaOneCalc should be able to reuse the same OxIde UI/components. Therefore OxVba-facing DTOs should be shared/authoritative rather than duplicated in DnaOxIde-specific code.

## Current OxVba Surface To Align With

OxVba documentation currently identifies these direct-host surfaces as the intended consumption path for OxIde-class hosts:

- `oxvba_languageservice::HostWorkspaceSession`
- `oxvba_languageservice::LanguageService`
- `oxvba_project::load_workspace_target`
- `oxvba_project::inspect_workspace_target`
- `oxvba_project::host_helpers::*`
- `oxvba_project::prepare_host_project_edit_plan`
- `oxvba_project::apply_host_project_edit_plan`
- `oxvba_project::assess_project_com_selections`
- `oxvba_project::com_selection`
- `oxvba_host::Engine`
- `oxvba_host::EmbeddedBuildRunHost`
- `oxvba_host::EmbeddedWorkspaceInput`
- `oxvba_host::EmbeddedWorkspaceSnapshot`
- `oxvba_host::EmbeddedRunSession`
- `oxvba_host::ImmediateSession`
- `oxvba_host::DebugSession`

DnaOxIde should consume direct Rust APIs, not CLI text, and should not route internal semantics through LSP.

## Required Contract Qualities

Every OxVba API intended for DnaOxIde should provide:

1. **Stable identity** — workspace, project, document/module, runtime session, debug session, breakpoint, stack frame, and watch identities must be stable enough for UI state and incremental updates.
2. **Typed status** — command availability and disabled reasons must be typed, not parsed from strings.
3. **Typed errors** — build failure, runtime failure, COM unavailable, debug unavailable, missing service, user/host policy rejection, and invalid project shape must be distinct.
4. **Source mapping** — diagnostics, runtime errors, debug locations, breakpoints, and references must map back to module/document spans.
5. **Overlay policy** — build/run must explicitly choose `DiskOnly` versus workspace/editor overlay state; OxIde must not reconstruct compiler input itself.
6. **No fake data requirement** — if a capability is not implemented, return unavailable states; do not synthesize callstacks, locals, watch values, Immediate responses, or COM runtime results.
7. **Windows capability gates** — COM discovery/runtime and any Windows-only runtime services must report platform/capability status explicitly.
8. **Fixture-grade determinism** — ThinSliceHello and future fixtures must produce deterministic results suitable for GUI-lab/Tauri regression evidence.

## Requirement R1 — Workspace, Project, And Document Identity

DnaOxIde needs OxVba to expose a direct session model that can be held by the desktop host.

Required:

- load/reload a workspace target once,
- stable workspace target id,
- project id/name/path,
- module/document id,
- module file path,
- logical module name,
- `Attribute VB_Name` reconciliation status,
- document open/update/close operations,
- workspace snapshot revision or version,
- current source policy: `DiskOnly` or `WorkspaceOverlay`,
- deterministic relation between editor overlays and build/run snapshots.

Acceptance evidence needed from OxVba:

- host session opens ThinSliceHello,
- updates an unsaved module overlay,
- diagnostics reflect overlay state,
- `DiskOnly` and `WorkspaceOverlay` snapshots diverge when expected,
- checked-in fixtures are not mutated by tests.

## Requirement R2 — Language Service For IDE Editing

DnaOxIde needs a direct language-service session for editor features.

Required features:

- diagnostics,
- document symbols,
- workspace symbols,
- semantic classifications/tokens,
- completions,
- signature help,
- hover,
- go-to-definition,
- references,
- rename preparation,
- safe reference-update/code-action planning where supported,
- source-span DTOs suitable for editor squiggles, panels, and navigation.

Required shape:

- works over real project/workspace identity,
- accepts editor overlays through `HostWorkspaceSession`,
- does not require LSP transport for OxIde-class hosts,
- does not require CLI invocation or CLI output parsing.

## Requirement R3 — Project Authoring And Compile Options

DnaOxIde needs project properties and compile/build configuration panels soon.

Required project authoring:

- inspect project/module roster,
- add/remove/rename modules,
- add/remove class modules where supported,
- reconcile file path, logical name, and `Attribute VB_Name`,
- validated plan/apply flow for `.basproj` edits,
- project edit preview/diff details for UI confirmation,
- deterministic validation errors.

Required compile/build options DTOs:

- project name and identity,
- startup/entrypoint/run target list,
- build/check profile list if profiles exist,
- conditional compilation constants,
- Option Explicit / module/project policy indicators if authoritative,
- warning/error policy if supported,
- reference set used for build,
- source policy for unsaved editor overlays,
- disabled reasons for unavailable options.

Acceptance evidence needed:

- DnaOxIde can render a project properties panel from OxVba DTOs,
- build/check command can run without CLI parsing,
- changed compile/project settings flow through OxVba validated edit plans rather than OxIde-local `.basproj` mutation.

## Requirement R4 — Build/Check Contract

DnaOxIde needs a typed build/check command before runtime/debug can feel real.

Required:

- `EmbeddedBuildRunHost` or equivalent direct build facade,
- `EmbeddedWorkspaceInput` / `EmbeddedWorkspaceSnapshot` handoff,
- explicit source policy (`DiskOnly`, `WorkspaceOverlay`),
- build request id,
- build status,
- build diagnostics,
- output/activity events,
- phase/timing labels where inexpensive,
- warnings distinct from errors,
- invalid workspace/project errors distinct from compile failures,
- no shelling out to CLI for normal IDE build/check.

OxIde packet alignment:

- OxIde can map build output into run/output timelines and diagnostics panels.
- Runtime command availability should remain disabled until build/run support is available.

## Requirement R5 — Runtime And Run Session Contract

DnaOxIde needs real runtime sessions for W365.

Required:

- list run targets/entrypoints,
- create runtime session from a build/run request,
- stable `runtime_session_id`,
- lifecycle events: started, ready, output, completed, failed, reset, stopped/cancelled,
- structured output events,
- runtime error DTOs with source spans when available,
- reset runtime,
- invoke entry point,
- invoke procedure,
- stop/cancel command where supported,
- command availability and disabled reasons,
- explicit COM runtime availability when a run path needs COM.

Required event qualities:

- typed events, not line-parsed text,
- deterministic enough for fixture tests,
- correlation ids for project, request, and runtime session,
- errors separated into build failure, startup failure, invoke failure, host policy rejection, and service unavailable.

OxIde packet alignment:

- `RuntimeServicePacket.runtime_session_id`
- `RuntimeServicePacket.provider_kind`
- `RuntimeServicePacket.command_status`
- `RuntimeServicePacket.events`
- `real_execution_claimed`
- `native_runtime_claimed`
- `com_runtime_claimed`

Those claim flags may flip only after tests prove the corresponding OxVba-backed behavior.

## Requirement R6 — Immediate Window Contract

DnaOxIde needs Immediate Window support attached to the active runtime session.

Required:

- create/attach `ImmediateSession` to an `EmbeddedRunSession`,
- stable `immediate_session_id`,
- current `runtime_session_id`,
- evaluate request text,
- typed responses: output, value, diagnostic, runtime error,
- command availability in stopped/running/paused/no-session states,
- deterministic error taxonomy for evaluation failure,
- optional history can remain OxIde-owned, but request/result semantics belong in OxVba.

OxIde packet alignment:

- `ImmediateServicePacket.immediate_session_id`
- `ImmediateServicePacket.runtime_session_id`
- `ImmediateServicePacket.request_text`
- `ImmediateServicePacket.responses`
- `fake_responses = false`
- native/COM claim flags only after evidence.

## Requirement R7 — Debug, Watches, And Breakpoints

DnaOxIde needs a real debug vertical slice soon after runtime sessions exist.

Required debug session:

- attach/create `DebugSession` from an active runtime session,
- stable `debug_session_id`,
- state: unavailable, running, paused, stopped,
- command availability for continue, break, step into, step over, step out, stop/restart where supported,
- current execution source span,
- runtime error pause state.

Required callstack/locals:

- stable stack frame id,
- display name/procedure/module,
- source span per frame when available,
- locals names,
- values,
- type labels,
- expandable/opaque value status if needed.

Required watches:

- add/update/remove watch expressions,
- evaluate watches against selected frame/current pause context,
- watch result values, type labels, diagnostics/errors,
- stale/unavailable status when not paused.

Required breakpoints:

- set/clear/enable/disable breakpoint by module/source span,
- stable breakpoint id,
- bind/unbind status,
- unresolved reason,
- hit count or hit state if supported,
- source remapping after module edits,
- deterministic behavior when breakpoint line is invalid.

OxIde packet alignment:

- `DebugServicePacket.debug_session_id`
- `DebugServicePacket.runtime_session_id`
- `DebugServicePacket.state`
- `DebugServicePacket.debug_commands`
- `DebugServicePacket.callstack`
- `DebugServicePacket.locals`
- `DebugServicePacket.watches`
- `DebugServicePacket.breakpoints`
- `fake_debug_data = false`
- native/COM claim flags only after evidence.

## Requirement R8 — References And Windows COM

DnaOxIde needs COM/reference UX early because it affects build, run, and debug behavior.

Required reference/project state:

- active reference roster,
- missing/broken reference status,
- reference identity: name, GUID/typelib id if applicable, version, LCID, path, source kind,
- project reference add/remove/repair/reorder plans where supported,
- validated apply flow through OxVba project semantics.

Required COM discovery:

- Windows-native service/API for registered type libraries,
- ProgID-backed discovery,
- file-backed type library discovery,
- display name,
- typelib GUID,
- version,
- LCID,
- path,
- bitness/compatibility status where knowable,
- availability status on browser/non-Windows/native-service-missing profiles,
- deterministic errors for registry access failure, unsupported platform, and missing native service.

Required COM runtime posture:

- separate reference discovery from runtime invocation,
- report whether runtime COM invocation is available for the active host/session,
- do not imply COM runtime support just because reference discovery works,
- define apartment/threading/bitness constraints if OxVba owns or brokers COM calls.

OxIde packet alignment:

- `ComCapabilityProfile.reference_discovery`
- `ComCapabilityProfile.runtime_invocation`
- `ComHostProfileKind::WindowsNativeServiceAvailable` only after tested Windows-native service evidence.

## Requirement R9 — Capability And Error Taxonomy

DnaOxIde needs a shared taxonomy so UI disabled states are stable and not string guessed.

Required categories:

- browser unsupported,
- non-Windows unsupported,
- native service missing,
- native service unhealthy,
- host policy denied,
- workspace invalid,
- project invalid,
- compile options invalid,
- build failed,
- run target missing,
- runtime startup failed,
- runtime session unavailable,
- Immediate unavailable,
- Immediate evaluation failed,
- debug unavailable,
- not paused,
- watch evaluation failed,
- breakpoint unresolved,
- COM discovery unavailable,
- COM reference missing/broken,
- COM runtime unavailable,
- COM bitness/apartment incompatible.

Each category should carry:

- stable code,
- user-displayable summary,
- technical detail where useful,
- source span/path/session id when applicable,
- retryability if known.

## Requirement R10 — Native Service / Sidecar Boundary

DnaOxIde is likely Tauri-first. OxVba can expose direct Rust APIs in-process, a sidecar native service, or both. For full Windows desktop capability, the boundary needs to be explicit.

Required if a native service/sidecar is used:

- service start/stop/health check,
- version handshake,
- capability handshake,
- workspace open/close lifecycle,
- runtime session lifecycle,
- log/event channel,
- crash/exit reporting,
- cancellation model,
- COM apartment/thread policy,
- 32-bit/64-bit compatibility policy,
- serialization schema/versioning,
- deterministic fixture mode for tests.

Required if in-process APIs are used first:

- document which APIs are safe to call from Tauri command handlers,
- define threading constraints,
- define how long-running build/run/debug work reports progress without blocking UI commands,
- define how runtime sessions are cancelled/dropped.

## Requirement R11 — Test Fixtures And Evidence

DnaOxIde needs fast confidence from OxVba-owned tests before flipping UI claims.

Minimum fixture ladder:

1. ThinSliceHello build/check succeeds.
2. ThinSliceHello overlay edit changes diagnostics/build result without mutating checked-in files.
3. ThinSliceHello run produces typed lifecycle/output events.
4. Immediate evaluates at least one deterministic request against a live session.
5. Debug can pause/step or otherwise expose a deterministic bounded debug state.
6. Watch evaluation reports value or typed unavailable/error state.
7. Breakpoint bind/unbind status is deterministic.
8. COM reference discovery has Windows-only gated tests.
9. COM unavailable paths are tested on browser/non-Windows/native-service-missing profiles.
10. Broken/missing reference fixture reports deterministic repair/diagnostic status.

Test rules:

- use test-owned temp project copies,
- do not mutate checked-in fixtures,
- gate Windows/COM tests explicitly,
- keep browser/WASM unavailable states visible,
- no fake Immediate/debug data in acceptance tests.

## Requirement R12 — Minimal Unblock Sequence For DNA OxIde

To reach full scope soon, OxVba should prioritize the smallest vertical slices that unlock DnaOxIde worksets:

| DnaOxIde workset | OxVba requirement | Minimum acceptable output |
| --- | --- | --- |
| W355 compile/build UX | project/compile options + typed build/check | project properties DTO, run target list, build result with diagnostics |
| W360 COM references | COM/reference discovery and reference plan/apply | active references, candidate search, missing/repair status, apply plan |
| W365 runtime + Immediate | runtime session + Immediate session | run events, stable runtime id, Immediate request/responses |
| W370 debug/watch/breakpoints | debug session attached to runtime | state, commands, callstack, locals, watches, breakpoints |

The fastest useful path is not a perfect final protocol. It is a tested direct-host vertical slice for ThinSliceHello, then incremental broadening.

## OxIde/DnaOxIde Consumption Commitments

When OxVba exposes the required surfaces, OxIde/DnaOxIde should:

- consume direct Rust APIs rather than CLI text,
- keep `HostWorkspaceSession` as the editor overlay/language-service anchor,
- use `EmbeddedBuildRunHost` for build/run,
- retain one active `EmbeddedRunSession` for runtime/Immediate/debug composition,
- map OxVba DTOs into `GuiShellPacket`, `RuntimeServicePacket`, `ImmediateServicePacket`, `DebugServicePacket`, and future shared UI components,
- preserve no-claim flags until tests prove real capability,
- keep Tauri-specific code outside shared UI crates so DnaOneCalc can reuse the interface.

## Non-Goals

This requirements note does not ask OxVba to own:

- DnaOxIde visual layout,
- DnaOneCalc product shell placement,
- OxIde command palette/keybindings/focus graph,
- Tauri packaging,
- browser/WASM COM execution,
- LSP as the internal OxIde transport,
- parked TUI substrate changes.

## Authorization Gate

This note is written inside the OxIde repo only. Any implementation work in `C:/Work/DnaCalc/OxVba` still requires explicit user authorization naming that repo, for example:

```text
Authorize writes to C:/Work/DnaCalc/OxVba for DNA OxIde full-scope OxVba host integration work.
```

Until then, OxIde can continue scaffold/shared-UI/Tauri-host work and can document this handoff, but must not claim real OxVba runtime/debug/Immediate/COM capability without tested OxVba-backed evidence.
