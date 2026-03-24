# OxIde Vision

## Summary

`OxIde` is intended to be a small console-based micro-IDE for `OxVba`, in the spirit of QuickBasic or Visual Basic for MS-DOS.

It is not meant to be a general multi-language IDE. Its job is to be the focused interactive development environment around the `OxVba` compiler, runtime, hosting model, and future language services.

## Core Direction

- Implement `OxIde` in Rust.
- Use FrankenTui as the console shell.
- Use `msedit` or an extracted/adapted derivative of it for the editing control.
- Start as a text editor with a command surface, then add capabilities incrementally.

## Early Product Shape

The first useful shape of `OxIde` should be modest:

- edit VBA source text
- expose a small command set
- host and drive `OxVba` workflows
- provide a concrete place to test project and runtime integration

The project should avoid pretending to be a complete IDE too early. The path is to grow from editor-first to richer development environment one layer at a time.

## Strategic Role

`OxIde` is also a pathfinder project.

It should serve as a proving ground for:

- console UI technology choices
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
- project management surfaces
- integration around an embedded `OxVba` engine

This embedded scenario is a first-class design influence, not an afterthought.

## Platform Direction

Current target platforms:

- Windows
- Linux
- Wasm

Planned later:

- macOS

Windows remains especially important because the long-term scope includes full COM-related hosting support where appropriate.

## Scope Boundaries

`OxIde` is not intended to become a general-purpose multi-language IDE.

Its scope is centered on:

- `OxVba`
- `OxVba` hosting
- `OxVba` language services
- development workflows that directly support the DnaCalc ecosystem

## Working Principle

Use `OxIde` as both a product and a laboratory:

- build something genuinely useful
- use it to force clarity in `OxVba` design
- keep the architecture open enough for future language services and debugging layers
- grow capability in small, testable increments
