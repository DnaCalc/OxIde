# OxIde GUI Pivot First-Pass Plan

Status: `first_pass_planning_note`
Date: 2026-05-07
Scope: Rust/WASM-capable GUI pivot for OxIde, TUI parking, DNA Calc host integration, Windows COM capability implications, and implementation-run preparation

## 1. Purpose

This note captures the current planning discussion around pivoting `OxIde` from its active terminal-native `FrankenTui` direction toward a Rust-based GUI implementation that can run in browser/WASM-capable DNA Calc hosts and in a standalone cross-platform desktop host.

It is intentionally a first-pass planning document, not a locked implementation spec.

It exists to:
1. record the proposed product and architecture reframing,
2. identify the repo and documentation preparation needed before substantial GUI implementation,
3. park the TUI direction without deleting it,
4. define how `DnaOneCalc` should act as the first exemplar host,
5. capture the Windows COM constraint for OxVba runtime execution,
6. establish a research stance for editor/toolkit references and licensing,
7. review how the current codebase should be approached,
8. define testing, feedback, and work-structure practices that set up a smooth implementation run.

## 2. Revised Product Framing

The proposed new direction is:

`OxIde` should become the shared Rust/WASM-capable OxVba IDE surface for standalone use and for embedding inside DNA Calc hosts.

Interpretation:
1. `OxIde` remains the IDE/editor/project-authoring product.
2. `OxVba` remains the owner of VBA language, project, semantic, build, runtime, immediate, and debug truth.
3. DNA Calc hosts, starting with `DnaOneCalc`, may embed or consume OxIde IDE/editor surfaces.
4. Browser/WASM is a real target, not a mock-only target.
5. A local desktop shell, likely `Tauri`, is also a real target for standalone operation.
6. The target environment is the DNA Calc suite of hosts, not arbitrary third-party host embedding.

This reframing replaces the current repo-authoritative product statement that `OxIde` is primarily a standalone terminal-native IDE.

## 3. DNA Calc Cross-Repo Assumption

The planning assumption is that `OxIde`, `OxVba`, `DnaOneCalc`, and the other sibling DNA Calc repositories are part of one coordinated cross-repo product family.

Human and agentic developers for the DNA Calc project have product-level control over the whole suite. That means the best final layout should be pursued directly:
1. interfaces can be changed in upstream or sibling repos when that produces a cleaner system,
2. implementations can be moved between repos when ownership is clearer there,
3. shared crates/types should be introduced or consumed where they reduce duplication,
4. compatibility bridges should not be built merely to avoid making a simple coordinated change elsewhere,
5. long-term layering still matters, so sharing should be intentional rather than a shortcut to tight coupling.

Important execution rule for this repo-scoped agent session:
1. the OxIde-hosted coding agent may write only inside this `OxIde` repo/folder and subdirectories,
2. the agent may read sibling repos under the shared `DnaCalc` parent,
3. changes needed in sibling repos must be captured as handoff notes, work requests, or externally coordinated changes,
4. the plan may recommend cross-repo interface changes, but this repo run must not directly edit those sibling repos.

## 4. Architectural Invariants

The key invariant remains unchanged:

```text
OxVba owns VBA/project/runtime truth.
OxIde owns IDE experience.
DNA Calc hosts consume, embed, and run where appropriate.
```

Consequences:
1. `OxIde` must not invent a duplicate `.basproj` project model.
2. `OxIde` must not invent duplicate VBA semantic truth.
3. `OxIde` should not duplicate enums/types that properly belong to another DNA Calc repo; it should consume the authoritative types where layering permits.
4. Internal OxIde semantics should prefer direct typed OxVba APIs over LSP-shaped indirection.
5. LSP remains useful for external editor integrations, not as OxIde's internal architecture.
6. GUI code should be host-aware through explicit capabilities, not through hidden platform assumptions.
7. Where a needed OxVba or DnaOneCalc interface is almost right but awkward, prefer a coordinated upstream improvement over local adapter sprawl.

## 5. Current Codebase Review

The current codebase is valuable, but it is strongly shaped by the terminal-native implementation path.

Observed shape:
1. `src/main.rs` is a TUI entrypoint over `ftui`.
2. `src/shell/model.rs` mixes input routing, shell orchestration, project actions, scene state, and editor behavior.
3. `src/shell/view.rs` is terminal-frame rendering code.
4. `src/shell/state.rs` contains useful behavior and state ideas, but much of it is intertwined with TUI scenes, panels, focus regions, lower surfaces, and width classes.
5. `src/shell/oxvba.rs` proves direct OxVba integration and web-host command/event usage, but should be redesigned behind a GUI/session adapter boundary.
6. `src/shell/project_actions.rs` contains valuable project-authoring behavior, including module creation and COM-reference helper flows, but currently performs direct filesystem and platform work.
7. `src/shell/uxlab/*` and `tests/wtd/*` are strong design/evidence assets, but they are TUI/terminal-specific.

Conclusion:

```text
Do not evolve the current FrankenTui code directly into the GUI product.
Use it as requirements, evidence, behavior reference, and design history.
Build the GUI core cleanly.
```

This should bias strongly toward new implementation rather than rescuing, copying, or reworking existing fragments. Existing code should be reused only when it is clearly pure, well-layered, and not carrying terminal assumptions. The default should be to rewrite behavior deliberately in the new architecture.

## 6. Current Code Salvage Policy

A deliberate salvage map should be created before implementation. Its purpose is not to maximize code reuse. Its purpose is to prevent accidental reuse of TUI-shaped code while preserving hard-won product and seam learning.

Suggested categories:

```text
reuse as-is
extract after refactor
rewrite from behavior
park as evidence only
```

First-pass classification:

| Area | Recommended treatment | Notes |
|---|---|---|
| `src/shell/view.rs` | park as evidence only | terminal-frame renderer; not GUI substrate |
| `src/main.rs` | park with TUI crate | TUI entrypoint only |
| `src/bin/oxide-uxlab.rs` | park / inspire GUI lab | useful scenario-lab pattern, TUI-specific runtime |
| `src/shell/firehorse_design.rs` | park / inspire GUI lab | design-screen selector pattern is useful |
| `src/shell/state.rs` | rewrite from behavior | contains useful editor/session ideas but too TUI-shaped |
| `src/shell/model.rs` | rewrite from behavior | command/update concepts useful; monolithic TUI model should not carry forward |
| `src/shell/session.rs` | rewrite/extract cautiously | workspace/document projection ideas useful; new code should use better OxVba/session layering |
| `src/shell/oxvba.rs` | rewrite into `oxide-oxvba` | strong proof of seams; current shape is not final adapter architecture |
| `src/shell/project_actions.rs` | rewrite into services/adapters | useful behavior; avoid UI-level filesystem/platform coupling |
| `src/shell/session_store.rs` | rewrite | APPDATA-only and desktop-local assumptions need capability-aware persistence |
| `src/shell/highlight.rs` | research/reference | may inform syntax rendering but OxVba should remain truth |
| `src/shell/uxlab/*` | park and mine for scenarios | useful scenario catalogue and audit approach |
| `tests/wtd/*` | parked TUI regression suite | should remain opt-in; browser GUI needs separate test loop |

New doc likely needed:

```text
docs/GUI_PIVOT_CODEBASE_REVIEW.md
```

That doc should be created during `W200` and should explicitly record why code is being rewritten rather than salvaged where applicable.

## 7. DnaOneCalc Role

`DnaOneCalc` is the first architectural exemplar host for this direction.

Current useful facts from `DnaOneCalc`:
1. it is already a Rust workspace,
2. its active app direction is shared Rust application core,
3. its UI direction is `Leptos`,
4. it already has browser/WASM testing and preview scripts,
5. it has a host/adapters/services/UI layout that maps well to OxIde's needs,
6. its charter already lists `OxVba` as a staged later dependency.

Recommended role split:

```text
DnaOneCalc
  product host, proving workbench, first embedded consumer

OxIde
  IDE/editor/project-authoring product and reusable IDE surface

OxVba
  semantic/project/runtime authority
```

`DnaOneCalc` should not absorb OxIde's identity. It should act as the first host that proves embedded IDE surfaces and seamless OxVba artifact execution inside a DNA Calc product shell.

Because the project suite is jointly controlled, any DnaOneCalc interface or layout issue discovered during OxIde work should be handled by a cross-repo handoff and coordinated change rather than by excessive local compatibility code in OxIde.

## 8. Initial DnaOneCalc Integration Proofs

A staged proof path should be used.

### 8.1 Artifact/runtime proof

Goal:
1. create or load an OxVba artifact authored through OxIde concepts,
2. consume it from `DnaOneCalc`,
3. run it through OxVba runtime/session APIs where host capabilities allow,
4. show output/result/evidence in the DnaOneCalc shell.

Shape:

```text
OxIde-authored artifact
  -> DnaOneCalc host
    -> OxVba runtime/session
      -> visible result, output, diagnostics, evidence
```

### 8.2 Embedded editor proof

Goal:
1. expose an OxIde editor/project surface inside `DnaOneCalc`,
2. keep document/project/language truth in OxVba,
3. keep the `DnaOneCalc` shell as host/product owner.

Shape:

```text
DnaOneCalc mode or panel
  contains OxIde editor component
  backed by OxVba document/session APIs
```

### 8.3 Shared component proof

Goal:
1. use the same lower-level OxIde editor/session/UI components in standalone OxIde and embedded DnaOneCalc,
2. avoid duplicating editor semantics in the host.

Shape:

```text
oxide-editor + oxide-oxvba + oxide-ui components
  used by:
    OxIde standalone
    DnaOneCalc embedded IDE surface
```

## 9. Runtime Capability Model

The GUI pivot must distinguish between:
1. pure browser/WASM execution,
2. native desktop execution,
3. Windows-native COM-enabled execution.

The phrase "OxVba runs in WASM" must not be used as a universal claim. A better framing is:

1. browser/WASM mode supports a WASM-safe OxVba capability profile,
2. desktop native mode can run native OxVba where packaged by the host,
3. Windows desktop native mode can support COM when OxVba runs in a native Windows host layer,
4. browser-only mode cannot directly call Windows COM.

Capability profiles should become first-class product state, not just runtime error messages.

Possible conceptual shape:

```rust
HostCapabilityProfile {
    host_kind,
    platform,
    runtime_location,
    filesystem_access,
    oxvba_semantics,
    oxvba_execution,
    com_reference_discovery,
    com_runtime_invocation,
    persistence_mode,
}
```

The actual implementation should prefer authoritative shared types from existing DNA Calc repos if such types exist or should be added there.

## 10. Windows COM Constraint

OxVba supports COM references and calls on Windows. This creates an important capability boundary.

COM cannot be supported directly from pure browser/WASM. When COM is enabled, OxVba must execute through a native Windows runtime layer.

For a Windows desktop host, the intended split is:

```text
Leptos / WASM UI
  owns editor UI, panes, commands, presentation

Native host layer on Windows
  owns COM-capable OxVba runtime/session
  owns COM apartment/threading policy
  owns registry/type-library discovery
  owns actual COM calls

OxVba
  owns project, language, runtime, immediate/debug semantics
```

A Windows COM-capable host should likely own a service with responsibilities such as:
1. initialize and own the COM apartment deliberately,
2. perform typelib/reference discovery,
3. create COM objects,
4. marshal calls, values, errors, and diagnostics,
5. enforce host trust and capability policy,
6. expose results back to the GUI through typed DTOs/events.

COM should not be casually invoked from arbitrary async UI tasks. A dedicated native service or controlled runtime thread is the safer architecture.

If OxVba already owns or should own part of this service, the correct action is a coordinated OxVba handoff rather than duplicating COM runtime concepts in OxIde.

## 11. Capability Matrix

First-pass host capability matrix:

| Host mode | OxVba location | COM reference discovery | COM runtime calls | Expected behavior |
|---|---|---:|---:|---|
| Browser/WASM only | WASM-safe profile | No | No | Edit/project/diagnostic features where supported; COM execution blocked or metadata-only |
| Desktop on Windows | Native host layer available | Yes | Yes | Full Windows COM-capable runtime path where trust policy allows |
| Desktop on macOS/Linux | Native non-Windows | No Windows COM | No Windows COM | Project can show unsupported COM references; execution requiring COM is blocked |
| DnaOneCalc browser | WASM-safe profile | No | No | Embedded IDE/editor can work; COM-dependent run paths are unavailable |
| DnaOneCalc desktop on Windows | Native host layer available | Yes | Yes | Exemplar COM-capable DNA Calc host path |

The UI should surface capability truth explicitly. Example statuses:
1. `COM reference resolved`,
2. `COM reference present but unavailable in this host`,
3. `This project requires Windows native runtime`,
4. `Run with a Windows desktop host to execute COM calls`.

## 12. Host Protocol And Bridge Priority

The first-pass plan named `oxide-bridge`; this should be treated as central rather than incidental.

The same IDE surface must be able to operate in:
1. browser/WASM-only mode,
2. DnaOneCalc browser mode,
3. DnaOneCalc desktop mode,
4. standalone desktop mode,
5. Windows native COM-enabled mode.

A typed request/event protocol should exist early enough to prevent UI code from depending on accidental local host details.

Likely protocol families:

```text
IdeRequest / IdeEvent
ProjectRequest / ProjectEvent
DocumentRequest / DocumentEvent
RuntimeRequest / RuntimeEvent
ImmediateRequest / ImmediateEvent
DebugRequest / DebugEvent
CapabilitySnapshot
DiagnosticPacket
CompletionPacket
RunPacket
```

The protocol should avoid duplicating upstream types when direct use is appropriate. If shared serializable DTOs belong in `OxVba` or another common crate, create a handoff for that upstream/shared move instead of creating long-lived local copies in OxIde.

## 13. Proposed OxIde Crate Arrangement

A possible target workspace layout:

```text
OxIde/
  Cargo.toml
  README.md
  PRODUCT_DIRECTION.md
  ARCHITECTURE.md

  crates/
    oxide-domain/
      # pure IDs, view models, capability model, host-independent product vocabulary

    oxide-core/
      # app state, command registry, reducers, session orchestration

    oxide-editor-core/
      # text buffer, selections, caret, decorations, editor commands

    oxide-oxvba/
      # direct OxVba adapter over HostWorkspaceSession, project helpers,
      # build/run, immediate, debug, and capability-sensitive runtime paths

    oxide-bridge/
      # serde request/event DTOs for WASM/native host boundaries

    oxide-ui-leptos/
      # shared GUI UI model, Leptos components, design system

    oxide-guilab/
      # browser scenario catalogue and visual feedback harness

    oxide-host-browser/
      # browser/WASM entrypoint

    oxide-host-tauri/
      # standalone desktop entrypoint

    oxide-tui-frankentui/
      # parked TUI implementation, feature-gated and isolated

  docs/
    GUI_DIRECTION.md
    DNA_CALC_HOST_INTEGRATION.md
    EDITOR_SUBSTRATE_RESEARCH.md
    TUI_PARKING_PLAN.md
    GUI_PIVOT_CODEBASE_REVIEW.md
    GUI_TEST_STRATEGY.md
```

The exact crate split may change. The principle is more important than the names:
1. separate host-independent IDE state from UI rendering,
2. separate OxVba adapter code from GUI components,
3. keep bridge/protocol DTOs explicit where needed,
4. isolate the parked TUI so it no longer drives active architecture,
5. consume authoritative cross-repo types instead of duplicating them,
6. use handoffs to move common concepts to the right DNA Calc repo when needed.

A dedicated `oxide-host-dnaonecalc` crate should not be assumed initially. The better first move may be shared bridge/components consumed from the DnaOneCalc side, avoiding premature coupling or circular repo pressure.

## 14. TUI Parking Plan

The TUI direction should be parked, retained, and isolated. It should not be deleted.

Parking means:
1. the current `FrankenTui` implementation remains available,
2. TUI docs and WTD tests remain available as historical and possible future assets,
3. TUI dependencies become isolated or feature-gated,
4. active product and architecture docs no longer describe TUI as the primary direction,
5. active workset sequencing moves to GUI pivot worksets,
6. TUI code is moved or otherwise isolated so it does not shape new GUI architecture by accident.

Parking does not mean:
1. deleting TUI code,
2. erasing TUI design learning,
3. claiming the TUI work was wrong,
4. preventing a future companion TUI.

The TUI should be treated as a parked prototype/evidence lane, not as the implementation base for the GUI.

## 15. Required Documentation Sweep

The pivot requires a full doc sweep because current authoritative docs strongly encode the terminal-native product direction.

Files requiring review and likely revision:

```text
README.md
PRODUCT_DIRECTION.md
ARCHITECTURE.md
docs/WORKSET_REGISTER.md
docs/DESIGN_TUI.md
docs/DESIGN_TUI_2026_FIRE_HORSE.md
docs/DESIGN_MOCKUP_WEB.md
docs/TESTING_WTD.md
docs/worksets/*
Cargo.toml
```

New docs likely needed:

```text
docs/GUI_DIRECTION.md
docs/DNA_CALC_HOST_INTEGRATION.md
docs/EDITOR_SUBSTRATE_RESEARCH.md
docs/THIRD_PARTY_RESEARCH_AND_LICENSES.md
docs/TUI_PARKING_PLAN.md
docs/GUI_PIVOT_CODEBASE_REVIEW.md
docs/GUI_TEST_STRATEGY.md
docs/worksets/W200_gui_pivot_foundation.md
docs/worksets/W210_fixture_project_opens_in_gui.md
docs/worksets/W220_editable_module_and_diagnostics.md
docs/worksets/W230_save_reload_session_restore.md
docs/worksets/W240_capability_aware_run_output.md
docs/worksets/W250_dnaonecalc_embedding_proof.md
docs/worksets/W260_windows_com_capability_proof.md
```

The documentation sweep should record any required sibling-repo changes as handoff notes rather than implementing those changes from this repo-scoped agent run.

## 16. Revised Workset Shape

First-pass workset register interpretation:

```text
Parked TUI lineage:
  W010-W110

Active GUI lineage:
  W200 — GUI pivot foundation, codebase review, and TUI parking
  W210 — fixture project opens in GUI
  W220 — editable module and diagnostics
  W230 — save, reload, and session restore
  W240 — capability-aware run/output path
  W250 — DnaOneCalc embedded IDE/runtime proof
  W260 — Windows COM capability proof
  W270 — run/debug/immediate GUI surfaces
  W280 — polish, accessibility, cross-platform packaging
```

`W200` should happen before large GUI implementation. Its closure should include doc authority updates, workspace layout preparation, TUI isolation, codebase salvage mapping, capability model design, bridge protocol sketch, GUI test strategy, first fixture set, and DnaOneCalc dependency-direction guidance.

Worksets after `W200` should be vertical and reviewable rather than broad infrastructure buckets. Each should close against a visible or executable product outcome.

## 17. Editor And Toolkit Research Stance

The editor should likely be custom and OxVba-aware rather than a wholesale adoption of a generic editor.

Rationale:
1. OxIde needs project-aware VBA document identity,
2. OxIde needs tight OxVba diagnostics/completions/hover/references integration,
3. OxIde needs immediate/debug/run surfaces over the same runtime session,
4. OxIde should not insert a broad LSP or generic parser abstraction between itself and OxVba.

Useful references to study include:
1. `CodeMirror 6` for browser editor architecture, transactions, decorations, and extension model,
2. `Monaco` for large-scale IDE editor UX and language feature surfacing,
3. `Zed` for panes, command palette, keymap/context models, responsiveness, and editor/product feel,
4. `Lapce` / `Floem` for Rust GUI/editor patterns,
5. `Dioxus` and `Leptos` examples for Rust/WASM UI patterns,
6. `Ropey`, `Crop`, or related Rust text-buffer crates,
7. `Tree-sitter` integrations if an incremental syntax projection layer becomes useful,
8. `Helix` / `Kakoune` for command and selection models,
9. classic `VBA` / `VB6` IDEs for project-first product behavior.

Licensing stance:
1. prefer MIT/Apache-compatible dependencies for production code,
2. track dependency licenses deliberately,
3. GPL/AGPL systems can be studied as references but should not be copied into MIT-bound code,
4. avoid licensing fuss by documenting what is a dependency, what is a reference, and what is only inspiration,
5. prefer shared DNA Calc implementations over third-party dependencies when that produces a cleaner long-term product.

## 18. Accessibility And Keyboard Discipline

The TUI direction naturally preserved keyboard-first discipline. The GUI pivot must retain that intentionally.

Early GUI architecture should include:
1. command registry,
2. keybinding contexts,
3. command palette,
4. focus order and focus visibility,
5. no-mouse critical paths,
6. screen-reader labels where applicable,
7. high-contrast and semantic color tokens,
8. platform shortcut policy for browser and desktop differences.

This should be designed before the GUI grows many independent widgets.

## 19. Testing Strategy

A smooth implementation run requires the test structure to be created early.

### 19.1 Pure Rust unit tests

Cover:
1. editor buffer behavior,
2. cursor and selection movement,
3. undo/redo,
4. command reducer behavior,
5. capability model behavior,
6. DTO serialization,
7. project/session view-model construction.

These tests should not require browser, Tauri, OxVba, or filesystem unless specifically necessary.

### 19.2 OxVba contract tests

Cover `oxide-oxvba` behavior against fixture projects:
1. load workspace,
2. map documents,
3. diagnostics,
4. hover,
5. references,
6. build/run,
7. immediate,
8. unsupported COM profile behavior.

Where OxVba APIs need improvement, create handoff notes and avoid duplicating OxVba-owned types locally.

### 19.3 WASM/browser tests

Use the DnaOneCalc browser-test approach as an exemplar:
1. `wasm-bindgen-test`,
2. browser runner scripts,
3. component smoke tests,
4. deterministic fixtures.

### 19.4 Browser visual/scenario tests

Create a GUI scenario catalogue early, tentatively `oxide-guilab`.

Initial scenarios:
1. empty/welcome,
2. project loaded,
3. module editor,
4. diagnostics visible,
5. completion/hover visible,
6. run output,
7. COM reference unavailable,
8. COM reference available on Windows profile,
9. DnaOneCalc embedded host frame.

Possible test techniques:
1. Playwright screenshots,
2. DOM text snapshots,
3. accessibility snapshots,
4. deterministic scenario fixtures.

### 19.5 Host capability matrix tests

Explicitly test:

```text
browser wasm: COM unavailable
desktop non-Windows: COM unavailable
desktop Windows native: COM available if service present
```

Windows COM tests may start simulated, but user-visible product behavior should be covered from the start.

### 19.6 DnaOneCalc integration smoke

Eventually prove:

```text
DnaOneCalc can consume OxIde bridge/component/artifact without owning OxIde semantics.
```

Because this requires sibling-repo work, the OxIde repo should first produce fixtures, shared contracts, and handoff notes.

## 20. Feedback Loop

Set up three feedback surfaces early:

1. `oxide-guilab`
   - fast browser scenario catalogue for design and component review,
   - successor to the TUI UX lab for GUI work.

2. fixture projects
   - empty project,
   - module/class project,
   - diagnostics project,
   - references project,
   - run-output project,
   - COM-reference-present-but-unavailable project,
   - COM-reference-available-on-Windows project.

3. capability badges and status surfaces
   - the app should always know and display what host profile it is in,
   - unsupported runtime paths should be visible and explanatory rather than surprising.

This avoids building a visually attractive browser IDE that later discovers it cannot honestly run important projects.

## 21. Adjacent Opportunities

The GUI pivot creates opportunities beyond OxIde alone:

1. Shared DNA Calc design system
   - visual tokens, layout primitives, accessibility conventions, and component vocabulary could be shared between OxIde and DnaOneCalc where appropriate.

2. Shared host capability ledger
   - OxIde, DnaOneCalc, OxFml, OxReplay, and OxVba can all benefit from explicit host/capability truth.

3. Reusable artifact model
   - define what an OxIde-authored OxVba artifact is so DnaOneCalc and future hosts can run it seamlessly.

4. Unified evidence/handoff format
   - DnaOneCalc already thinks in retained evidence; OxIde run/debug sessions could emit compatible evidence packets.

5. Editor substrate shared lessons
   - DnaOneCalc's formula editor and OxIde's VBA editor should not be identical, but they can share testing discipline, input architecture, and design-token primitives.

6. Native runtime service boundary
   - Windows COM support may become a reusable DNA Calc native capability, not just an OxIde implementation detail.

7. Cross-repo type cleanup
   - avoid duplicate enums/types by moving common host, capability, artifact, and protocol concepts to the repo where they belong.

These opportunities should be exploited through coordinated handoffs and cross-repo work, while respecting the current OxIde-agent write boundary.

## 22. First Implementation Recommendation

The next concrete scope should be:

```text
W200 — GUI pivot foundation, codebase review, and TUI parking
```

Suggested closure outcomes:
1. `PRODUCT_DIRECTION.md` revised around the Rust/WASM-capable GUI direction,
2. `ARCHITECTURE.md` revised around shared IDE core, GUI hosts, OxVba adapter seams, and capability profiles,
3. `docs/TUI_PARKING_PLAN.md` added,
4. `docs/GUI_DIRECTION.md` added,
5. `docs/DNA_CALC_HOST_INTEGRATION.md` added,
6. `docs/EDITOR_SUBSTRATE_RESEARCH.md` added,
7. `docs/THIRD_PARTY_RESEARCH_AND_LICENSES.md` added,
8. `docs/GUI_PIVOT_CODEBASE_REVIEW.md` added,
9. `docs/GUI_TEST_STRATEGY.md` added,
10. repo converted or prepared for a workspace crate layout,
11. current TUI code isolated under a parked crate/path,
12. TUI/WTD tests made opt-in rather than default blockers for GUI work,
13. workset register updated to show parked TUI lineage and active GUI lineage,
14. first GUI fixture projects selected or created,
15. capability model and host protocol sketched,
16. DnaOneCalc integration proof captured as a later workset with clear dependency boundaries,
17. sibling-repo changes captured as handoff notes where needed.

## 23. Implementation Run Guidance

For the smoothest implementation run:

1. start the GUI implementation greenfield,
2. use current TUI code as evidence and behavior reference, not as the GUI foundation,
3. create the capability model and bridge/protocol before rich UI polish,
4. create the browser GUI lab immediately,
5. keep worksets vertical and reviewable,
6. close each workset against visible/runnable evidence,
7. prefer coordinated upstream/shared changes over local compatibility layers,
8. avoid duplicate cross-repo types,
9. preserve keyboard-first and accessibility discipline from day one,
10. keep TUI/WTD available but out of the default GUI development path.

A good first vertical GUI slice is:

```text
Open fixture .basproj
  -> list modules
  -> open one module
  -> edit text in custom editor surface
  -> ask OxVba for diagnostics
  -> show diagnostics
  -> persist/save where host capability allows
```

A good second vertical slice is:

```text
Run project through capability-aware runtime path
  -> browser mode reports unsupported/limited honestly
  -> native mode runs where available
  -> output appears in GUI
```

A good third vertical slice is:

```text
DnaOneCalc consumes the bridge/component/artifact
  -> loads artifact
  -> shows embedded editor or read/run surface
```

## 24. Open Questions

1. Which host/capability/protocol types already exist in sibling repos and should be consumed rather than duplicated?
2. Which shared concepts should move to OxVba, DnaOneCalc, Foundation, or another common DNA Calc repo?
3. How should DnaOneCalc consume OxIde components without creating circular workspace or repo dependencies?
4. What is the minimal browser/WASM OxVba capability profile for first GUI proof?
5. What is the minimal Windows-native COM-capable runtime service for first desktop proof?
6. Should the first GUI editor use a native DOM editing surface, canvas-like rendering, or a hybrid input-backed custom renderer?
7. Which editor reference systems should be studied first, and how should licensing notes be recorded?
8. What fixture projects best exercise project load, diagnostics, run output, and COM capability boundaries?
9. What exact repository move isolates the TUI while obeying the no-delete rule and preserving reviewable history?

## 25. Summary

The proposed direction is to pivot `OxIde` toward a Rust/WASM-capable GUI IDE architecture for OxVba, suitable for standalone use and for embedding inside DNA Calc hosts such as `DnaOneCalc`.

The TUI direction should be parked carefully, not removed. A full repo and doc sweep is required so the old TUI-first authority does not interfere with the new GUI direction.

The GUI implementation should start cleanly. The current codebase should be mined for requirements, tests, seam evidence, UX learning, and behavior examples, but the default bias should be new implementation rather than salvaging TUI-shaped code.

The key technical caveat is Windows COM: pure browser/WASM cannot support COM directly. COM-capable OxVba execution requires a native Windows runtime layer, while browser/WASM mode should expose an honest reduced capability profile.

The wider DNA Calc project is under coordinated control, so the best final cross-repo architecture should be pursued directly. OxIde should consume shared authoritative types, request upstream/sibling changes through handoffs when needed, and avoid compatibility bridges or duplicate local models where a simple coordinated cross-repo improvement would be cleaner.
