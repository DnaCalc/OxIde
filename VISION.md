# OxIde Vision

## Summary

`OxIde` is intended to be a small console-based micro-IDE for `OxVba`, in the spirit of QuickBasic or Visual Basic for MS-DOS.

It is not meant to be a general multi-language IDE. Its job is to be the focused interactive development environment around the `OxVba` project model, compiler, runtime, hosting model, build targets, and language services.

## Core Direction

- Implement `OxIde` in Rust.
- Use `FrankenTui` as the shell and runtime foundation.
- Keep `OxIde` on one primary UI/runtime stack.
- Use the `FrankenTui` editor path as the initial implementation behind an `OxIde`-owned `EditorSurface`.
- Treat `msedit` as a correctness and behavior reference, plus a selective donor for editor algorithms and tests.
- Start as a text editor with a command surface, then add project/workspace, language-service, and target-aware build/runtime capabilities incrementally.

## Early Product Shape

The first useful shape of `OxIde` should be modest:

- edit VBA source text
- bind one active document to a `DocumentSession`
- make open/save/dirty-state behavior visible and explicit
- expose a small command set
- host and drive one real `OxVba` workflow
- provide a concrete place to test project and runtime integration

The project should avoid pretending to be a complete IDE too early. The path is to grow from editor-first to project-aware development environment one layer at a time.

## Project And Target Scope

`OxIde` now needs to treat the `OxVba` project and tooling substrate as first-class scope, not as distant future work.

That means the intended product scope includes:

- `.basproj` as the canonical OxVba project format
- project/workspace loading and generation around `ProjectManifest`
- project references, COM references, and native references as part of the editor-facing model
- target-aware build and run workflows
- runtime profile and host policy selection where OxVba exposes them
- host-driven and embedded execution paths, not just standalone script execution

The relevant OxVba target/output surfaces that must be in OxIde scope are:

- `HostModule`
- `Library`
- `Exe`
- `Addin`
- `ComServer`
- `ComExe`

Not every target needs to be implemented in the first thin slice, but the architecture should assume they exist.

## Strategic Role

`OxIde` is also a pathfinder project.

It should serve as a proving ground for:

- console UI technology choices
- clean seams between shell, project/session, document/session, editor, and OxVba service concerns
- VBA project hosting and execution
- cross-platform host behavior
- language services for `OxVba`
- debugging and inspection layers
- future external-editor integration paths such as an LSP

Even when these areas are not fully factored into separate services yet, the design should make experimentation in those areas practical.

## Embedded IDE Direction

`OxIde` should be designed from early on so it can also function as an embeddable VBA IDE, analogous to VBA inside Excel.

In that mode, `OxIde` would run in-process inside a console-based host application and provide:

- editing
- project and document/session management
- project management surfaces
- integration around embedded `OxVba` project, language-service, and runtime layers

This embedded scenario is a first-class design influence, not an afterthought.

## Platform Direction

Current target platforms:

- Windows
- Linux

Exploratory or optional later directions:

- Wasm

Planned later:

- macOS

Windows remains especially important because the long-term scope includes COM-related hosting and server targets where appropriate.

## Scope Boundaries

`OxIde` is not intended to become a general-purpose multi-language IDE.

Its scope is centered on:

- `OxVba`
- `OxVba` projects and project artifacts
- `OxVba` build and runtime targets
- `OxVba` hosting and language services
- development workflows that directly support the DnaCalc ecosystem

## Working Principle

Use `OxIde` as both a product and a laboratory:

- build something genuinely useful
- use it to force clarity in `OxVba` design
- keep the architecture open enough for project/workspace, language-service, and debugging layers
- keep file/document ownership explicit rather than burying it inside the editor widget
- keep concrete `FrankenTui` editor types behind `OxIde` seams
- treat OxVba source text as host-provided editor state for language-service purposes rather than assuming filesystem reloads
- grow capability in small, testable increments

## Architectural Shape

The intended top-level seams are:

- `OxIdeShell`
  - owns panes, commands, status, focus routing, and workflow orchestration

- `ProjectSession`
  - owns the active project/workspace, `.basproj` identity, module roster, project references, target selection, and runtime/profile/policy selections presented to the user

- `DocumentSession`
  - owns current document identity, file path binding, dirty state, open/save/reload semantics, versioning, and editor-facing source text

- `EditorSurface`
  - owns text editing behavior, viewport behavior, and editor interaction

- `OxVbaServices`
  - owns the OxVba-side project, language-service, build, and execution service boundaries consumed by OxIde

This matters because `OxIde` is meant to grow without tangling file semantics, project semantics, editing behavior, and `OxVba` execution into one object model.
