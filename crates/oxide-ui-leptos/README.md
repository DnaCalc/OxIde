# oxide-ui-leptos

Status: `w342_shared_shell_scaffold`

Shared OxIde IDE UI component boundary for DNA OxIde, DnaOneCalc, and GUI-lab review surfaces.

W342 starts with deterministic `html-strings` rendering around `oxide_core::GuiShellPacket`. Real Leptos component exports and hydration are feature placeholders only until later worksets add dependency/toolchain evidence.

Boundary rules:

- consumes OxIde packets/view models,
- emits deterministic component-like markup,
- carries provenance labels,
- keeps Tauri and app-folder dependencies out,
- does not call OxVba or DnaOneCalc directly,
- preserves no-claim runtime/debug/Immediate/COM states.

See [`../../docs/SHARED_UI_COMPONENT_API.md`](../../docs/SHARED_UI_COMPONENT_API.md).
