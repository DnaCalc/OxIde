# OxIde DNA Calc Host Integration

Status: `first_pass_integration_plan`
Date: 2026-05-07

## Purpose

This note records the intended relationship between OxIde and DNA Calc hosts during the GUI pivot.

## Host Framing

Embedded OxIde means embedded inside the DNA Calc host suite, not arbitrary third-party host embedding.

DnaOneCalc is the first exemplar host because it already has a Rust, Leptos, browser/WASM, and desktop-capable architecture.

## Ownership Split

```text
DnaOneCalc
  product host, proving workbench, first embedded consumer

OxIde
  IDE/editor/project-authoring product and reusable IDE surface

OxVba
  VBA semantic/project/runtime authority
```

## Integration Proof Ladder

1. Artifact/runtime proof: DnaOneCalc consumes an OxIde-authored OxVba artifact and runs it through OxVba where host capability allows.
2. Embedded editor proof: DnaOneCalc contains an OxIde editor/project surface backed by OxVba document/session APIs.
3. Shared component proof: standalone OxIde and embedded DnaOneCalc use the same lower-level editor/session/UI components without duplicating semantics.

## Cross-Repo Coordination

The DNA Calc repos are controlled as one coordinated product family. If a clean integration requires changing DnaOneCalc or OxVba interfaces, prefer a handoff and coordinated sibling-repo change over local compatibility sprawl.
