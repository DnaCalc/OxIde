# OxIde Product Direction

Status: `active_product_direction`
Date: 2026-05-08

This document implements the product commitments in [`CHARTER.md`](CHARTER.md).

## 1. Product Direction

`OxIde` is the OxVba IDE for the DNA Calc program.

The active direction is a Rust/WASM-capable GUI IDE surface that supports three real product scenarios:

1. **DnaOneCalc website / browser WASM host** — the DnaOneCalc app is loaded from a website into the client browser; it can open the OxIde interface, edit/load source, compile/check through an OxVba wasm-safe profile, and run/invoke supported compiled or interpreted functions directly inside the DnaOneCalc WASM host.
2. **Standalone Windows DnaOxIde desktop** — the same IDE surface runs in a desktop shell, with native Rust OxVba compiler/runtime/debug/COM capabilities exposed by the desktop host.
3. **DnaOneCalc Windows desktop host** — the DnaOneCalc desktop product shell embeds/opens OxIde and exposes native OxVba capabilities, including Windows COM where supported.

Across all scenarios OxIde must author, inspect, run, and debug OxVba projects without duplicating OxVba truth. Browser/WASM support is a first-class product target, not an exhibition-only proof mode.

The previous FrankenTui direction is parked, retained, and no longer the active product path. It remains valuable design evidence and possible future companion-TUI material. See [`docs/TUI_PARKING_PLAN.md`](docs/TUI_PARKING_PLAN.md).

## 2. What Are We Building?

OxIde should become:

- a modern GUI IDE for OxVba,
- a project-aware and runtime-aware authoring surface,
- a reusable IDE/editor surface for DNA Calc hosts,
- a direct host/consumer of OxVba project, language-service, build/run, immediate, and debug semantics,
- a product that feels closer to a focused VB/VBA-class IDE than a generic editor with a plugin.

OxIde should not become:

- a VS Code clone,
- a generic editor platform first,
- a browser-only toy IDE,
- a duplicate OxVba project or semantic model,
- a compatibility shell that hides host/runtime capability differences,
- a mutation of the existing TUI codebase into a GUI by attrition.

The core product promise is:

```text
Open an OxVba project.
Understand its modules, references, targets, diagnostics, and runtime capability.
Edit source confidently.
Compile/check and run where the current host profile supports it.
Use Immediate/debug/COM surfaces where the current host profile supports them.
Carry the same IDE surface into DNA Calc hosts where that is the right product fit.
```

## 3. Product Ownership Split

The ownership split is non-negotiable:

```text
OxVba owns VBA/project/runtime truth.
OxIde owns IDE experience.
DNA Calc hosts consume, embed, and run where appropriate.
```

### OxVba owns

- `.basproj` and project semantics,
- workspace loading and discovery policy,
- module/reference/project identity,
- parsing, binding, semantic analysis, diagnostics,
- completions, hover, symbols, definitions, references, rename/code-action planning,
- build/run contracts,
- runtime sessions,
- Immediate Window semantics,
- debug session semantics,
- COM reference and COM runtime semantics where supported.

### OxIde owns

- product shell and IDE flow,
- editor UX and source presentation,
- project/session/document orchestration as a host of OxVba truth,
- command model, keybindings, command palette, and action availability,
- layout, panes, focus, overlays, dialogs, and status surfaces,
- buffer lifecycle UX: dirty, save, reload, revert, restore,
- host capability presentation,
- how OxVba results are made visible and actionable.

### DNA Calc hosts own

- their own product shells,
- when and where embedded OxIde surfaces appear,
- host policy and persistence policy,
- host-specific capability exposure,
- retained evidence and integration-specific workflows where applicable.

## 4. DNA Calc Host Framing

"Embedded OxIde" means embedded inside the DNA Calc host suite, not arbitrary third-party host embedding.

`DnaOneCalc` is the first exemplar because it already points toward:

- Rust shared app core,
- Leptos UI,
- browser/WASM capability,
- desktop host capability,
- adapter/service/UI layering,
- evidence-driven testing.

The desired relationship is:

```text
DnaOneCalc
  product host, proving workbench, first embedded consumer

OxIde
  IDE/editor/project-authoring product and reusable IDE surface

OxVba
  semantic/project/runtime authority
```

DnaOneCalc should not absorb OxIde identity. OxIde should not hard-code DnaOneCalc as its only future host. Shared surfaces and contracts should be clean enough for the DNA Calc host family.

See [`docs/DNA_CALC_HOST_INTEGRATION.md`](docs/DNA_CALC_HOST_INTEGRATION.md).

## 5. Cross-Repo Product Assumption

The DNA Calc repos are one coordinated product family. Human and agentic developers have product-level control over OxIde, OxVba, DnaOneCalc, and sibling repos.

That includes directing OxVba changes when the clean product path requires a wasm-safe compiler/runtime profile, a native compiler/runtime profile, shared DTOs, or clearer host capability contracts. OxIde should capture those needs as handoffs or coordinated work rather than building permanent substitutes.

That changes the design posture:

1. prefer a clean cross-repo interface change over compatibility glue,
2. do not duplicate enums/types from sibling repos when authoritative types can be used,
3. move shared concepts to the repo where they belong,
4. add handoff notes when this repo discovers required sibling-repo changes,
5. preserve long-term layering even while exploiting shared ownership.

Execution boundary for OxIde-hosted agents:

- write only in the OxIde repo/folder and subdirectories,
- read sibling DNA Calc repos freely,
- route sibling repo changes through handoffs or separate repo-scoped runs.

## 6. Runtime Capability Honesty

OxIde must be explicit about what the current host can do.

The product supports multiple host profiles:

1. browser website / DnaOneCalc WASM host profile,
2. standalone DnaOxIde desktop profile,
3. DnaOneCalc Windows desktop embedded-host profile,
4. Windows desktop native profile with COM capability where available,
5. non-Windows desktop profile without Windows COM.

Do not treat "OxVba in WASM" as either impossible or universal. The correct product model is:

- browser/WASM mode supports a **wasm-safe OxVba compiler/runtime profile** that can compile/check and run/invoke supported functions inside the browser host;
- browser/WASM mode must surface native-only capabilities as unavailable instead of faking them;
- desktop native mode can run native OxVba where packaged by the host;
- Windows native mode can support COM through a native runtime layer;
- pure browser/WASM cannot directly call Windows COM.

The UI should surface facts such as:

- `COM reference resolved`,
- `COM reference present but unavailable in this host`,
- `This project requires Windows native runtime`,
- `Run with a Windows desktop host to execute COM calls`.

Capability truth is a product feature, not just an error path.

## 7. Windows COM Product Rule

Windows COM support requires native Windows execution.

In browser/WASM-only mode, COM references may be visible as project facts, but COM discovery and COM runtime calls are unavailable unless a native host service is present.

In Windows desktop mode, the intended product behavior is:

```text
GUI surface
  presents project/reference/runtime state

Native Windows host service
  owns COM-capable OxVba runtime/session
  owns COM apartment/threading policy
  owns registry/type-library discovery
  owns COM calls

OxVba
  owns semantics and runtime truth
```

COM-dependent projects should degrade honestly outside Windows native profiles.

## 8. IDE-Style Product Identity

OxIde should lean IDE-style in product identity and editor-fast in moment-to-moment editing.

That means:

- workspace/project state is primary,
- files are project artifacts, not detached editor tabs,
- run/debug/build configuration is visible,
- references and host capabilities are first-class,
- diagnostics and symbols are near the source,
- Immediate and debug surfaces are tied to the active runtime session,
- command availability and disabled reasons are visible.

Mental model:

```text
open workspace
inspect project structure
edit source in context
understand diagnostics and references
run/debug through a visible target/profile
retain or hand off evidence where the host supports it
```

## 9. Editor And Command UX

The editor should likely be custom and OxVba-aware rather than a wholesale adoption of a generic editor.

Required product behavior:

- source editing with strong keyboard flow,
- syntax/semantic decoration from OxVba-owned truth,
- diagnostics, hover, completion, symbols, references, go-to-definition,
- project-backed document identity,
- multiple visible views where useful,
- open-buffer continuity,
- source continuity during run/debug/Immediate workflows,
- no broad LSP detour for internal semantics.

The command model should be unified across:

- visible buttons/menus,
- keyboard shortcuts,
- chords,
- command palette entries,
- contextual actions,
- host-availability and disabled-reason reporting.

VBA/VB-family shortcut familiarity remains important, but the GUI product should not be an imitation of the old VBE shell.

## 10. GUI UX Shape

The GUI should use modern desktop/web affordances while preserving the good discipline from the TUI work:

- clear project spine/navigation,
- dominant code/editor canvas,
- contextual side detail/dock,
- lower activity surfaces for problems/output/immediate/references/build/run timeline,
- overlays/dialogs that preserve backing state,
- command palette / command lens,
- high-density but calm information presentation,
- capability and target truth always visible enough to prevent mistaken runs.

TUI-specific terminal constraints no longer define the product. However, the TUI work's lessons about density, keyboard flow, status honesty, and visible state remain product input.

## 11. Accessibility And Keyboard Discipline

The GUI pivot must not lose the TUI's keyboard discipline.

Product requirements:

- all critical paths work without a mouse,
- focus is visible and predictable,
- command palette is first-class,
- keybinding contexts are explicit,
- screen-reader labels are considered from the start,
- high-contrast and semantic color tokens exist,
- platform/browser shortcut conflicts degrade to equivalent outcomes with honest hints.

## 12. Implementation Posture

The current TUI codebase should not be transformed directly into the GUI product.

The active posture is:

```text
start the GUI implementation cleanly
mine the TUI for evidence and behavior examples
reuse code only when it is clearly pure and not terminal-shaped
prefer rewrite-from-behavior over salvage
```

See [`docs/GUI_PIVOT_CODEBASE_REVIEW.md`](docs/GUI_PIVOT_CODEBASE_REVIEW.md).

## 13. First Product Slices

The first GUI implementation slices should be vertical and reviewable.

### Slice 1 — Fixture project opens in GUI

```text
Open fixture .basproj
  -> list modules
  -> open one module
  -> edit text in custom editor surface
  -> ask OxVba for diagnostics
  -> show diagnostics
  -> persist/save where host capability allows
```

### Slice 2 — Capability-aware run/output

```text
Run project through capability-aware runtime path
  -> browser mode reports unsupported/limited honestly
  -> native mode runs where available
  -> output appears in GUI
```

### Slice 3 — DnaOneCalc consumption

```text
DnaOneCalc consumes bridge/component/artifact
  -> loads artifact
  -> shows embedded editor or read/run surface
```

## 14. Active Planning Surfaces

Use these live docs rather than a monolithic planning note:

- [`docs/GUI_DIRECTION.md`](docs/GUI_DIRECTION.md)
- [`docs/DNA_CALC_HOST_INTEGRATION.md`](docs/DNA_CALC_HOST_INTEGRATION.md)
- [`docs/GUI_PIVOT_CODEBASE_REVIEW.md`](docs/GUI_PIVOT_CODEBASE_REVIEW.md)
- [`docs/GUI_TEST_STRATEGY.md`](docs/GUI_TEST_STRATEGY.md)
- [`docs/EDITOR_SUBSTRATE_RESEARCH.md`](docs/EDITOR_SUBSTRATE_RESEARCH.md)
- [`docs/THIRD_PARTY_RESEARCH_AND_LICENSES.md`](docs/THIRD_PARTY_RESEARCH_AND_LICENSES.md)
- [`docs/TUI_PARKING_PLAN.md`](docs/TUI_PARKING_PLAN.md)
- [`docs/worksets/W200_gui_pivot_foundation.md`](docs/worksets/W200_gui_pivot_foundation.md)

## 15. Success Definition

OxIde is successful when a user can:

1. open a real OxVba project,
2. understand project/module/reference/target state,
3. edit source confidently,
4. trust diagnostics and semantic surfaces because they come from OxVba,
5. run/debug where host capabilities allow,
6. understand when a host cannot run something and why,
7. carry useful IDE surfaces into DNA Calc hosts without duplicating semantics,
8. experience the product as a focused IDE rather than a generic editor shell.
