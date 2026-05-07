# Handoff — W220 Editable Module And Diagnostics

Status: `handoff_note`
Date: 2026-05-07

## W210 Baseline

W210 produced a deterministic GUI lab command:

```powershell
cargo run --manifest-path crates/Cargo.toml -p oxide-guilab -- render gui-thin-slice-loaded
```

The rendered scenario is read-only and fixture-backed. It proves:

1. `examples/thin-slice/ThinSliceHello.basproj` loads through OxVba project APIs,
2. `ProjectOpenSpineView` carries `ThinSliceHello`, `Module1.bas`, source text, and browser-safe capability state,
3. `oxide-guilab` can render the project-open spine by stable scenario ID.

## W220 Starting Point

W220 should add editable module and diagnostics behavior without erasing the W210 baseline.

Recommended first W220 steps:

1. add editor/document snapshot types outside the TUI code path,
2. add a minimal edit operation over source text,
3. add an OxVba language-service session adapter for diagnostics over the edited snapshot,
4. add a new lab scenario for diagnostics-visible/editing state,
5. keep `gui-thin-slice-loaded` as a read-only project-open regression.

## Constraints

1. Do not route internal OxIde semantics through LSP.
2. Do not duplicate OxVba diagnostic enums if authoritative types can be consumed or projected directly.
3. Do not import parked TUI editor state as the GUI editor substrate.
4. Keep browser-safe capability status visible.

## Open Questions For W220 Expansion

1. Should diagnostics use `HostWorkspaceSession` directly in `oxide-oxvba`, or should a narrower adapter trait be introduced in `oxide-core` first?
2. Should the first editable scenario use the current thin-slice source with an injected edit, or a new `diagnostics-demo` fixture?
3. Should W220 introduce Leptos/DOM now, or continue one more bead with deterministic renderer tests before browser mount?
