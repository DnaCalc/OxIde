# Handoff — DNA OxIde OxVba Feedback Alignment

Status: `confirmed_cross_repo_feedback`
Date: 2026-05-07
Source feedback: OxVba processed `docs/HANDOFF_DNAOXIDE_OXVBA_REQUIREMENTS.md` and created `../OxVba/docs/worksets/WORKSET_2026-05-07_DNAOXIDE_FULL_SCOPE_HOST_INTEGRATION_SUPPORT.md`.

## Confirmation

OxIde confirms the OxVba feedback and incorporates it into the DnaOxIde continuation plans.

Read-only verification in `C:/Work/DnaCalc/OxVba` confirmed the new OxVba-side workset and updated public host guidance. OxIde still has no authorization to write to the OxVba repo from this session.

## OxVba Surfaces Available Now

OxVba reports these direct Rust host surfaces as already available or available subsets:

### Workspace/editor

- `oxvba_languageservice::HostWorkspaceSession`
- direct diagnostics, symbols, hover, definition, references, completions, semantic provenance
- editor overlay snapshots
- explicit `DiskOnly` / `WorkspaceOverlay` source policy handoff through embedded workspace snapshots

### Project authoring

- `oxvba_project::inspect_workspace_target`
- `oxvba_project::host_helpers::*`
- validated `.basproj` edit plans
- module/class scaffolding
- module identity and `Attribute VB_Name` reconciliation data through current project surfaces

### COM/reference subset

- `oxvba_project::ComSelectionService`
- registered COM candidates
- ProgID-backed candidates
- file-backed candidates
- active COM reference assessment
- add/repair/replace/remove plans

### Build/run subset

- `oxvba_host::EmbeddedBuildRunHost`
- `oxvba_host::EmbeddedWorkspaceSnapshot`
- explicit `DiskOnly` / `WorkspaceOverlay`
- typed build/run/reset/invoke results
- live `EmbeddedRunSession`

### Immediate/debug subset

- live-runtime-backed `oxvba_host::ImmediateSession`
- VM-backed `oxvba_host::DebugSession`
- step/continue controls
- locals/frame projection
- bounded paused evaluation

## OxVba Gaps Still Tracked There

OxVba is now tracking these remaining gaps in its DNA OxIde full-scope workset:

- stable IDs and shared capability/error taxonomy,
- unified workspace/project/module DTOs with revisions and `Attribute VB_Name` state,
- project properties / compile options / run target DTOs,
- build/run request IDs, lifecycle events, and command availability,
- runtime session IDs and error/source-span mapping,
- Immediate/debug attach from `EmbeddedRunSession`,
- watch and breakpoint DTOs,
- COM capability profile, reference reorder, bitness/apartment/native boundary status,
- ThinSliceHello fixture/evidence ladder.

## Plan Impact For OxIde/DnaOxIde

The OxIde-side worksets should no longer treat every full-scope service as a blank unavailable stub. Instead they should split each service into:

1. **available-subset adapter proof** — consume current direct OxVba surfaces where the OxIde repo can depend on them safely;
2. **pending-hardening state** — keep UI disabled/partial for stable IDs, capability taxonomy, missing DTO fields, event streams, source-span mapping, watch/breakpoint DTOs, and COM native boundary status;
3. **claim gate** — flip OxIde `real_execution_claimed`, `native_runtime_claimed`, `com_runtime_claimed`, `fake_responses`, or `fake_debug_data` only when matching OxVba-backed tests exist in OxIde.

## Workset Adjustments

| OxIde workset | Adjustment from OxVba feedback |
| --- | --- |
| W341 DnaOxIde Tauri app scaffold | unchanged; scaffold stays product-host-only. |
| W342 shared UI component layer | design components to accept both unavailable packets and available-subset OxVba adapter packets. |
| W343 host bridge facade | add direct OxVba adapter map for `HostWorkspaceSession`, `inspect_workspace_target`, `ComSelectionService`, `EmbeddedBuildRunHost`, `EmbeddedRunSession`, `ImmediateSession`, and `DebugSession`. |
| W344 Tauri command boundary | split commands into proven OxIde-only, available-subset OxVba adapter, and pending-hardening unavailable responses. |
| W345 live host UI proof | can aim to show current OxVba subset data if dependency wiring is ready, but labels must distinguish subset-backed from full-scope complete. |
| W346 interaction/e2e harness | include subset-backed build/run/Immediate/debug smoke only after direct adapter tests exist; otherwise keep unavailable-state interactions. |
| W347 compile/options/reference UI | upgrade from pure placeholders to subset-backed project/reference/build panels where current OxVba APIs provide data; compile options/run-target/stable taxonomy remain pending. |
| W348 DnaOneCalc reuse path | reuse the same shared component + host bridge split so DnaOneCalc can consume unavailable, subset-backed, or full OxVba states through the same UI. |
| W349 while-OxVba acceptance | audit both subset-adoption evidence and remaining gap gates. |

## Claim Boundaries

Until OxIde has its own direct-consumption evidence over these OxVba APIs, the following remain unclaimed in OxIde/DnaOxIde:

- full real OxVba runtime execution in the DNA OxIde host,
- full Immediate Window session UX,
- full debug/watch/breakpoint UX,
- COM runtime invocation,
- complete stable capability/error taxonomy,
- complete source-span mapping for runtime/debug/breakpoints,
- real DnaOneCalc host mount.

Subset-backed adapter tests may prove narrower claims. Each narrow claim must name the exact OxVba surface and fixture evidence.

## Next OxIde-Side Planning Rule

When implementing W343-W347, prefer this order:

1. consume workspace/project/language-service surfaces first;
2. consume project/reference/COM selection subset next;
3. consume typed build/check subset before runtime UX;
4. consume runtime + Immediate + debug subset behind explicit partial-capability labels;
5. keep watch/breakpoint/source-span/COM-runtime claims pending until OxVba closes the corresponding gaps.
