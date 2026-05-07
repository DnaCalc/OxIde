# OxIde Editor Substrate Research

Status: `first_pass_research_plan`
Date: 2026-05-07

## Purpose

This note records the research stance for building a Rust/WASM-friendly OxVba editor surface.

## Direction

The editor should likely be custom and OxVba-aware rather than a wholesale adoption of a generic editor.

Reasons:
1. OxIde needs project-aware VBA document identity.
2. OxIde needs tight OxVba diagnostics, completions, hover, references, immediate, run, and debug integration.
3. OxIde should not route its internal semantics through LSP.
4. OxIde should consume authoritative OxVba types and APIs where layering permits.

## References To Study

1. CodeMirror 6 — browser editor architecture, transactions, decorations, extension model.
2. Monaco — IDE-scale editor UX and language-feature presentation.
3. Zed — panes, command palette, keymap contexts, responsiveness, product feel.
4. Lapce / Floem — Rust GUI/editor patterns.
5. Leptos / Dioxus examples — Rust/WASM UI patterns.
6. Ropey / Crop — text-buffer implementations.
7. Tree-sitter integrations — only if an incremental syntax projection layer becomes useful.
8. Helix / Kakoune — command and selection models.
9. VBA / VB6 IDEs — project-first product behavior.

## License Posture

Prefer MIT/Apache-compatible dependencies for production code. GPL/AGPL projects can be studied as references but should not be copied into MIT-bound code.
