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
