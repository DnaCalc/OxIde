# OxIde

`OxIde` is a console-based micro-IDE for `OxVba`.

It is intended to be the focused interactive environment around:

- `OxVba` source editing
- `.basproj` project files and project/workspace management
- `OxVba` language services
- target-aware build and run workflows
- embedded and host-driven `OxVba` scenarios

Current implementation focus:

- Rust application
- `FrankenTui` shell/runtime
- `FrankenTui` editor path behind an `OxIde`-owned `EditorSurface`
- explicit `ProjectSession` and `DocumentSession` seams
- `OxVba` consumed as a project/language-service/build/runtime substrate

Planned `OxVba` target surface in scope:

- `HostModule`
- `Library`
- `Exe`
- `Addin`
- `ComServer`
- `ComExe`

## Thin-Slice Sample Workflow

The current thin slice is a single-document shell with `OxVba` build and run
commands behind the command surface.

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
