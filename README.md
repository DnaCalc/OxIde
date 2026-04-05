# OxIde

`OxIde` is a console-based micro-IDE for `OxVba`.

It is intended to be the focused interactive environment around:

- `OxVba` source editing
- `.basproj` project files and project/workspace management
- `OxVba` language services
- target-aware build and run workflows
- standalone `OxVba` project authoring, editing, and debugging workflows

Current planning note:

- see `PRODUCT_DIRECTION.md` for the active UX and product-direction document
- see `docs/DESIGN_TUI.md` for the current detailed TUI shell spec
- see `OPERATIONS.md` and `docs/WORKSET_REGISTER.md` for the current execution model and workset state

Ownership of truth is split this way:

- `OxVba` owns VBA semantics, canonical `.basproj` meaning, workspace loading/discovery policy, and the public host/service boundary
- `OxIde` owns IDE behavior, shell flow, session orchestration, editor UX, command routing, and result presentation

## Project Direction

OxIde is being designed as:

- a standalone terminal-native IDE for `OxVba`
- a project-aware editing and authoring environment
- a direct host for OxVba project and semantic services
- a keyboard-first, high-density IDE shell built on `FrankenTui`

## Architecture Seams

The intended top-level seams are:

- `OxIdeShell`
  owns panes, commands, status, focus routing, and workflow orchestration
- `ProjectSession`
  owns `.basproj` identity, project/workspace state, target selection, and
  runtime/profile/policy selections
- `DocumentSession`
  owns document identity, file path binding, dirty state, and open/save/reload
  semantics
- `EditorSurface`
  owns text editing behavior and viewport behavior
- `OxVbaServices`
  owns project, language-service, build, and execution contracts consumed by
  `OxIde`

These seams describe the intended codebase division.

## Technical Direction

The repo is aligned around these decisions:

- Rust application
- `FrankenTui` as the shell/runtime foundation
- `FrankenTui` editor path behind an `OxIde`-owned editor seam
- direct OxVba host integration for project and semantic workflows
- `OxVba` treated as the project/language-service/build/runtime substrate
- current runtime service integration links directly against the sibling `../OxVba` workspace crates

Planned `OxVba` target surface in scope:

- `HostModule`
- `Library`
- `Exe`
- `Addin`
- `ComServer`
- `ComExe`

## Example Workflow

Use the sample project in `examples/thin-slice/`:

```bash
cargo run -- examples/thin-slice/Module1.bas
```

Then in `OxIde`:

1. Edit `Module1.bas`.
2. Save with the active save command.
3. Open the project file `examples/thin-slice/ThinSliceHello.basproj`.
4. Run the project build through `OxVba`.
5. Run the sample project through `OxVba`.

The footer status line shows the immediate result, and the `OxVba Output` pane
shows the action, target, success flag, exit code, and captured stdout/stderr.

For a fuller walkthrough, including a deliberate failing build to prove the
output pane, see `examples/thin-slice/README.md`.

## Verification

The repo includes tests and sample assets that support development.

Run it with:

```bash
cargo test smoke_flow_covers_launch_edit_save_open_build_and_run
```

Run the full unit test suite with:

```bash
cargo test
```

Important note:
- full `cargo test` also depends on the state of the sibling `OxVba` workspace

## Current Direction

The repo follows the product and architecture direction in:

- `PRODUCT_DIRECTION.md`
- `ARCHITECTURE.md`
- `OPERATIONS.md`
- `docs/WORKSET_REGISTER.md`

The intended ecosystem shape is:

- `OxVba` = engine, semantics, project truth, transport
- `OxIde` = first-class direct host and showcase
- VS Code extension = alternate host over the same `OxVba` semantics
