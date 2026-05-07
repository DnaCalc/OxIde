# Handoff — OxVba / Native Windows COM Capability Interfaces

Status: `cross_repo_handoff`
Date: 2026-05-07

## Source Workset

OxIde W260 added capability projections and GUI-lab evidence for COM reference states without claiming real COM execution.

Current OxIde evidence:

```powershell
cargo run --manifest-path crates/Cargo.toml -p oxide-guilab -- render gui-com-reference-browser-unavailable
cargo run --manifest-path crates/Cargo.toml -p oxide-guilab -- render gui-com-reference-nonwindows-unavailable
cargo run --manifest-path crates/Cargo.toml -p oxide-guilab -- render gui-com-reference-native-service-missing
```

These scenarios currently use an OxIde capability projection over a demo COM fact (`Scripting.Dictionary`). They do not claim that OxIde has parsed an authoritative OxVba project reference or invoked COM.

## What OxIde Can Render Now

OxIde can render:

1. COM reference present as a projected fact,
2. browser-safe COM discovery/runtime unavailable,
3. non-Windows native COM discovery/runtime unavailable,
4. Windows native host admitted but native COM service missing,
5. disabled reasons for discovery and runtime invocation,
6. explicit no-COM-runtime-support claims for unavailable profiles.

OxIde cannot yet prove:

1. authoritative `.basproj` COM reference parsing,
2. real COM type-library discovery,
3. real COM runtime invocation,
4. native COM service lifecycle/apartment policy,
5. OxVba run/debug/Immediate integration over a COM-capable runtime.

## Required Authoritative Inputs

Prefer OxVba or a shared DNA Calc crate to own these interfaces rather than duplicating them in OxIde:

1. **Project/reference facts**
   - stable representation of COM references in an OxVba project,
   - display name / identifier / source span or project item provenance,
   - distinction between declared reference, discovered type library, and runtime object.
2. **Discovery capability**
   - host profile or request packet for reference/type-library discovery,
   - result shape for available, unavailable, blocked, not configured, and failed states,
   - registry/file-backed type-library provenance where applicable.
3. **Runtime invocation capability**
   - native Windows service contract for COM runtime invocation,
   - service configured / not configured / denied / failed states,
   - threading/apartment policy ownership,
   - trust/safety policy inputs.
4. **Run/debug/Immediate integration**
   - how a COM-capable runtime session is selected,
   - how runtime events/errors are surfaced,
   - what debug and Immediate operations remain unavailable if COM service is missing.
5. **Error taxonomy**
   - deterministic labels for unavailable, blocked, not configured, discovery failure, activation failure, invocation failure, and marshalling failure.

## Native Service Ownership Decision

Open decision for coordinated work:

```text
Option A: OxVba owns native COM service contracts and implementation.
Option B: OxIde host owns native service shell and OxVba owns semantic/runtime APIs.
Option C: shared DNA Calc native service crate owns platform service, consumed by OxVba/OxIde.
```

W260 evidence suggests the service must be explicit and capability-aware whichever option wins.

## Constraints For Future Work

1. Pure browser/WASM must continue to report COM discovery/runtime unavailable.
2. Simulated run output must not be treated as native or COM evidence.
3. OxIde should not create long-lived duplicate COM project/runtime DTOs if OxVba owns them.
4. Native COM runtime support must be tested on Windows before any `COM available` UI claim.
5. DnaOneCalc embedding remains a consumer path, not the owner of COM semantics.

## Suggested Next Cross-Repo Bead

In an OxVba-scoped run, define or expose a minimal COM reference/capability packet:

```text
load project with COM reference
  -> expose declared COM reference fact
  -> expose discovery unavailable/available status by host profile
  -> expose runtime invocation unavailable/available status by native service profile
  -> round-trip packet through serde if it crosses host boundaries
```

OxIde can then replace its demo `ComReferenceFact::scripting_dictionary_demo()` with authoritative OxVba project/reference facts.
