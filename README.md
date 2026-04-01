# OxIde

`OxIde` is a console-based micro-IDE for `OxVba`.

It is intended to be the focused interactive environment around:

- `OxVba` source editing
- `.basproj` project files and project/workspace management
- `OxVba` language services
- target-aware build and run workflows
- embedded and host-driven `OxVba` scenarios

Current planning note:

- see `UX_BRAINSTORM.md` for the active UX redesign brainstorming document

Ownership of truth is split this way:

- `OxVba` owns VBA semantics, canonical `.basproj` meaning, workspace loading/discovery policy, and the public host/service boundary
- `OxIde` owns IDE behavior, shell flow, session orchestration, editor UX, command routing, and result presentation

## Current Thin Slice

The current implementation is intentionally narrow. It proves the editor, shell,
document, and service seams without pretending that full project/workspace
 support already exists.

What works now:

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

Current command surface:

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

Current limitations:

- one active `DocumentSession` at a time
- no explicit `ProjectSession` UI yet
- project build/run currently works by making the `.basproj` file the active
  document before `:build` or `:run`
- direct language-service views are currently output-pane oriented; there are no
  inline popups, completion acceptance UX, or navigation surfaces yet

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
- `msedit` as a correctness/behavior reference and selective donor
- `OxVba` treated as the project/language-service/build/runtime substrate

Planned `OxVba` target surface in scope:

- `HostModule`
- `Library`
- `Exe`
- `Addin`
- `ComServer`
- `ComExe`

## Thin-Slice Workflow

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

## Smoke Verification

The current thin slice has an in-repo smoke test that exercises the implemented
flow:

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

## Near-Term Direction

The next layer after this thin slice is the explicit project/workspace surface:

- define `ProjectSession` around `.basproj` and `ProjectManifest`
- add project/workspace UI and module navigation
- add target-aware build and run surfaces
- expose runtime profile and host policy selection
- integrate `oxvba-languageservice` against host-provided document text

That work is coupled to a parallel change in `OxVba`:

- define the first typed `OxIde`-facing session facade
- expand direct project helpers so `OxIde` does not invent project logic
- replace CLI-shaped build/run seams with typed embedded results

The intended ecosystem shape is:

- `OxVba` = engine, semantics, project truth, transport
- `OxIde` = first-class direct host and showcase
- VS Code extension = alternate host over the same `OxVba` semantics
