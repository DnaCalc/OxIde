# Handoff — DNA OxIde OxVba Feedback Alignment

Status: `confirmed_cross_repo_feedback_with_fixture_evidence`
Date: 2026-05-07
Source feedback: OxVba processed `docs/HANDOFF_DNAOXIDE_OXVBA_REQUIREMENTS.md` and created `../OxVba/docs/worksets/WORKSET_2026-05-07_DNAOXIDE_FULL_SCOPE_HOST_INTEGRATION_SUPPORT.md`.
Follow-up evidence: `../OxVba/docs/evidence/DNAOXIDE_THIN_SLICE_HELLO_FIXTURE_2026-05-07.md`.

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

## OxVba ThinSliceHello Fixture Evidence Now Available

OxVba published `docs/evidence/DNAOXIDE_THIN_SLICE_HELLO_FIXTURE_2026-05-07.md` for bead `bd-avdu.6.1`.

Read-only verification in the sibling repo confirms the evidence states that `crates/oxvba-languageservice/tests/dnaoxide_thin_slice_hello.rs` now covers temp `.basproj` fixture copies, not repository fixture mutation.

Covered OxVba-side direct-host seams:

- workspace load through `HostWorkspaceSession::load_workspace_path`,
- editor overlay through `HostWorkspaceSession::set_document_text`,
- roster overlay/version signal through `workspace_roster`,
- overlay build through `EmbeddedBuildRunHost::build_workspace`,
- runtime session creation through `EmbeddedBuildRunHost::run_project`,
- Immediate attach through `EmbeddedRunSession::into_immediate_session`,
- Immediate evaluation over overlay source through `ImmediateSession`,
- debug attach through `EmbeddedRunSession::into_debug_session`,
- debug watch registry/evaluation through `DebugSession::add_watch` and `evaluate_watches`,
- debug breakpoint binding DTO through `DebugSession::set_source_breakpoint`,
- stable frame/watch/breakpoint/runtime IDs in returned DTOs,
- broken COM reference state through `ComSelectionService::inspect_workspace_project_state`,
- COM runtime availability/capability DTOs through `ComSelectionService::capability_profile`.

OxVba validation reported:

```text
cargo test -p oxvba-languageservice dnaoxide_thin_slice_hello --quiet
```

Result: pass, 2 fixture tests passed.

## OxVba Gaps / OxIde Gates After Fixture Evidence

The follow-up evidence moves several items from "pending in OxVba" to **OxVba-fixture-evidenced / OxIde-adapter-pending**.

Still pending or not fully claimable in OxIde until adapter tests exist:

- OxIde direct consumption of the new ThinSliceHello fixture ladder,
- shared capability/error taxonomy adoption in OxIde,
- unified workspace/project/module DTO adoption with revisions and `Attribute VB_Name` state,
- project properties / compile options / run target DTO adoption,
- build/run lifecycle event and command availability adoption,
- runtime error/source-span mapping adoption,
- Immediate/debug attach UX over `EmbeddedRunSession`,
- watch and breakpoint pane wiring over OxVba DTOs,
- COM capability profile/native boundary UI wiring,
- COM runtime invocation claim evidence inside OxIde/DnaOxIde,
- real DnaOneCalc host mount.

## Plan Impact For OxIde/DnaOxIde

The OxIde-side worksets should no longer treat every full-scope service as a blank unavailable stub. Instead they should split each service into:

1. **available-subset adapter proof** — consume current direct OxVba surfaces where the OxIde repo can depend on them safely;
2. **oxvba-fixture-evidenced adapter target** — OxVba has fixture evidence, but OxIde still needs local adapter tests before UI claims can flip;
3. **pending-hardening state** — keep UI disabled/partial for DTO fields, event streams, source-span mapping, taxonomy, native boundary details, or host UX not yet adopted in OxIde;
4. **claim gate** — flip OxIde `real_execution_claimed`, `native_runtime_claimed`, `com_runtime_claimed`, `fake_responses`, or `fake_debug_data` only when matching OxVba-backed tests exist in OxIde.

## Workset Adjustments

| OxIde workset | Adjustment from OxVba feedback |
| --- | --- |
| W341 DnaOxIde Tauri app scaffold | unchanged; scaffold stays product-host-only. |
| W342 shared UI component layer | design components to accept both unavailable packets and available-subset OxVba adapter packets. |
| W343 host bridge facade | add direct OxVba adapter map for `HostWorkspaceSession`, `inspect_workspace_target`, `ComSelectionService`, `EmbeddedBuildRunHost`, `EmbeddedRunSession`, `ImmediateSession`, and `DebugSession`; add an `oxvba-fixture-evidenced` state for the ThinSliceHello fixture ladder. |
| W344 Tauri command boundary | split commands into proven OxIde-only, available-subset OxVba adapter, oxvba-fixture-evidenced adapter target, and pending-hardening unavailable responses. |
| W345 live host UI proof | can aim to show current OxVba subset/fixture-evidenced data if dependency wiring is ready, but labels must distinguish fixture-backed adapter evidence from full host capability. |
| W346 interaction/e2e harness | include subset-backed or fixture-evidenced build/run/Immediate/debug smoke only after direct OxIde adapter tests exist; otherwise keep unavailable-state interactions. |
| W347 compile/options/reference UI | upgrade from pure placeholders to subset/fixture-backed project/reference/build/runtime/debug panels where current OxVba APIs provide data; compile options/run-target/shared taxonomy may remain pending. |
| W348 DnaOneCalc reuse path | reuse the same shared component + host bridge split so DnaOneCalc can consume unavailable, subset-backed, or full OxVba states through the same UI. |
| W349 while-OxVba acceptance | audit both subset-adoption evidence and remaining gap gates. |

## Claim Boundaries

Until OxIde has its own direct-consumption evidence over these OxVba APIs, the following remain unclaimed in OxIde/DnaOxIde:

- full real OxVba runtime execution in the DNA OxIde host,
- full Immediate Window session UX,
- full debug/watch/breakpoint UX in DnaOxIde,
- COM runtime invocation in DnaOxIde,
- complete stable capability/error taxonomy adoption in OxIde,
- complete source-span mapping for runtime/debug/breakpoints in OxIde,
- real DnaOneCalc host mount.

Subset-backed or OxVba-fixture-evidenced adapter tests may prove narrower claims. Each narrow claim must name the exact OxVba surface, fixture evidence, and OxIde adapter test that proves consumption.

## Next OxIde-Side Planning Rule

When implementing W343-W347, prefer this order:

1. consume workspace/project/language-service surfaces first;
2. consume project/reference/COM selection subset next;
3. consume typed build/check subset before runtime UX;
4. consume runtime + Immediate + debug fixture-evidenced seams behind explicit partial-capability labels;
5. consume watch/breakpoint and COM capability-profile fixture-evidenced seams only after OxIde adapter tests exist;
6. keep COM runtime invocation and full host UX claims pending until OxIde/DnaOxIde proves them directly.
