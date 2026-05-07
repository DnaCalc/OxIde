# OxIde GUI Direction

Status: `first_pass_direction`
Date: 2026-05-07

## Purpose

This note records the active product-direction pivot for OxIde.

OxIde is moving toward a Rust/WASM-capable GUI IDE surface for OxVba that can run standalone and can be embedded inside DNA Calc hosts such as DnaOneCalc.

The live product authority is [`../PRODUCT_DIRECTION.md`](../PRODUCT_DIRECTION.md). This document is the short GUI direction anchor used by W200.

## Direction

OxIde should become:

1. a GUI IDE for OxVba project authoring,
2. a shared IDE/editor surface that DNA Calc hosts can consume,
3. browser/WASM-capable for host embedding and preview,
4. desktop-capable through a local host such as Tauri,
5. explicit about host capabilities, especially runtime and COM availability.

## Invariants

1. OxVba owns VBA language, project, semantic, build, runtime, immediate, and debug truth.
2. OxIde owns IDE experience, editor UX, command flow, and project-authoring presentation.
3. DNA Calc hosts consume/embed/run where appropriate.
4. OxIde should consume authoritative cross-repo types instead of duplicating them.
5. Coordinated upstream or sibling-repo changes are preferred over local compatibility bridges when that gives a cleaner final system.

## Current Implementation Posture

The existing FrankenTui implementation is retained as a parked prototype/evidence lane. The GUI implementation should bias strongly toward new implementation rather than rescuing TUI-shaped code.
