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
- see `OPERATIONS.md` and `docs/WORKSET_REGISTER.md` for the current execution model and rewrite workset map

Ownership of truth is split this way:

- `OxVba` owns VBA semantics, canonical `.basproj` meaning, workspace loading/discovery policy, and the public host/service boundary
- `OxIde` owns IDE behavior, shell flow, session orchestration, editor UX, command routing, and result presentation

## Current Prototype Baseline

The current implementation should be treated as a retained spike/reference, not
as the intended product baseline.

It is still useful because it proves some real integration seams, but the repo
is now in an explicit rebuild phase driven by `PRODUCT_DIRECTION.md`,
`ARCHITECTURE.md`, and the active workset register.

What the retained spike currently demonstrates:

- launch `OxIde` as a Rust console app
- open one file at startup or create one bound new file
- edit text in a single buffer
- save with `Ctrl-S` or `:write`
- open another file with `:open <path>`
- load a direct OxVba `HostWorkspaceSession` when a real `.basproj` is active
- push unsaved editor text into the active OxVba host session for project-backed modules
- surface diagnostics and document-symbol counts from the direct host session in the side panel
- switch the main output pane between execution output, diagnostics, symbols, hover, and completions
- revert the current file or reload the active project workspace from disk
- run `:build` and `:run` through `OxVbaServices`
- view structured results in the `OxVba Output` pane

Current prototype command surface:

- `:open <path>`
- `:write [path]`
- `:revert`
- `:build`
- `:diagnostics`
- `:symbols`
- `:hover`
- `:complete`
- `:workspace-reload`
- `:run`
- `:quit`

Why it is not the forward implementation base:

- it is built around one active document/editor path at a time
- it still uses raw `:` command entry as a primary interaction path
- project and semantic surfaces are still shaped like a prototype shell
- build/run still uses the temporary legacy execution seam
- direct language-service views are still output-pane oriented rather than
  rebuilt shell-native surfaces

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

Current implementation status against those seams:

- `DocumentSession` is explicit and implemented
- `ProjectSession` now owns the active direct `HostWorkspaceSession`
- editor semantic updates now flow directly into the OxVba host session
- `OxVbaServices` still owns the temporary CLI-shaped build/run seam
- the current shell already separates command handling, buffer state, footer
  status, and output-pane rendering
- `EditorSurface` is still an architectural seam more than a fully-factored
  runtime object

## Technical Direction

The repo is aligned around these decisions:

- Rust application
- `FrankenTui` as the shell/runtime foundation
- `FrankenTui` editor path behind an `OxIde`-owned editor seam
- the current repo is being rebuilt around FrankenTui-first editing and direct OxVba host integration
- `OxVba` treated as the project/language-service/build/runtime substrate

Planned `OxVba` target surface in scope:

- `HostModule`
- `Library`
- `Exe`
- `Addin`
- `ComServer`
- `ComExe`

## Prototype Workflow

Use the sample project in `examples/thin-slice/`:

```bash
cargo run -- examples/thin-slice/Module1.bas
```

Then in `OxIde`:

1. Edit `Module1.bas`.
2. Save with `Ctrl-S` or `:write`.
3. Switch the active document to the project file with `:open examples/thin-slice/ThinSliceHello.basproj`.
4. Run `:build` to compile the sample project through `OxVba`.
5. Run `:run` to execute the sample project through `OxVba`.

The footer status line shows the immediate result, and the `OxVba Output` pane
shows the action, target, success flag, exit code, and captured stdout/stderr.

For a fuller walkthrough, including a deliberate failing build to prove the
output pane, see `examples/thin-slice/README.md`.

## Prototype Verification

The retained spike has an in-repo smoke test that exercises the implemented
prototype flow:

- launch with a bound startup path
- edit the buffer
- save to disk
- open the sample `.basproj`
- issue `:build`
- issue `:run`
- assert the final structured output state

Run it with:

```bash
cargo test smoke_flow_covers_launch_edit_save_open_build_and_run
```

Run the full unit test suite with:

```bash
cargo test
```

Important note:
- treat current tests as prototype/reference evidence, not as proof that the
  rebuild is already structurally aligned
- full `cargo test` may also be affected by upstream `OxVba` breakage while the
  sibling repo is changing

## Current Rebuild Direction

The repo is now following the rewrite/salvage workset map in
`docs/WORKSET_REGISTER.md`.

Immediate direction:

- classify the retained spike into retain / port / discard buckets
- rebuild the shell and action system around the current product direction
- implement the explicit buffer/view/layout model
- rebuild the editor surface with correct undo/redo ownership
- rebuild file/workspace/project management surfaces on top of the new shell
- port the direct OxVba semantic editing integration into that rebuilt shell
- deliver the early-use empty state and console setup surfaces

That rebuild is still coupled to ongoing OxVba evolution:

- expand typed direct host/build/run contracts where OxIde currently still has
  temporary legacy seams
- continue exposing project helpers so OxIde does not invent project logic

The intended ecosystem shape is:

- `OxVba` = engine, semantics, project truth, transport
- `OxIde` = first-class direct host and showcase
- VS Code extension = alternate host over the same `OxVba` semantics
