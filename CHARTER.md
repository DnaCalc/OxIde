# OxIde Charter

Status: `active_product_charter`
Date: 2026-05-08

## Mission

OxIde is the OxVba IDE surface for the DNA Calc product family.

The product must let users edit, inspect, compile, run, and debug OxVba projects without duplicating OxVba-owned truth. The same IDE surface must be usable in browser-hosted DNA Calc apps and in Windows desktop hosts with native capability.

## Non-Negotiable Ownership

```text
OxVba owns VBA/project/compiler/runtime/debug/COM truth.
OxIde owns IDE/editor/product experience.
DNA Calc hosts own their product shell, host policy, and when OxIde is embedded.
```

Architecture and worksets must prefer clean coordinated changes to OxVba/DnaOneCalc over compatibility shims inside OxIde. We can direct OxVba work as needed; OxIde should not permanently contort around missing upstream seams.

## Required Product Scenarios

### 1. Browser website / DnaOneCalc WASM host

A user opens DnaOneCalc from a website. The client browser loads the DnaOneCalc WASM app. From that app the user can open the OxIde interface, enter or load local source, compile/check through a wasm-safe OxVba compiler profile, and run/invoke supported functions directly from the DnaOneCalc WASM host.

This scenario must not require native filesystem, native process spawning, Windows COM, or desktop IPC. It must support only the capabilities available to a browser-hosted WASM runtime and must report all unavailable native features honestly.

### 2. Standalone Windows DnaOxIde desktop

A user runs the DnaOxIde Windows desktop app. The app uses the same IDE surface, but its desktop host can expose native Rust capabilities: full native OxVba compiler/runtime where available, native filesystem/project access, wrapped native binary execution where supported, debugging surfaces, and Windows COM support.

### 3. Windows desktop DnaOneCalc host embedding OxIde

A user runs the Windows desktop DnaOneCalc app. DnaOneCalc owns the product shell and opens/hosts OxIde as an embedded IDE surface. Because the host is desktop/native, the embedded OxIde surface can use native OxVba services and Windows COM capabilities exposed by the DnaOneCalc desktop host.

## Stack Commitments

- Rust is the default language for shared OxIde/OxVba/DnaOneCalc core logic.
- The UI direction must remain compatible with browser/WASM and desktop WebView hosting.
- Leptos or a similar Rust/WASM-capable UI framework is the preferred shared UI direction.
- Tauri is the preferred desktop shell candidate for DnaOxIde and DnaOneCalc desktop hosts.
- In Tauri, the default native backend means Rust code linked into the Tauri app crate and called through typed commands; a separate service process is optional only when isolation, COM apartment policy, crash containment, or multi-host sharing requires it.
- OxVba needs distinct capability profiles: wasm-safe compiler/runtime and native Windows compiler/runtime/COM.

## Work Rule

Future work must move along real endpoint seams. A bead is not enough if it only produces a browser-only exhibition artifact unrelated to the intended product stack.

Acceptable proof seams include:

- shared Rust UI/component logic usable in browser WASM and desktop WebView;
- DnaOneCalc browser WASM host invoking OxIde/OxVba wasm-safe compiler/runtime APIs;
- Tauri/WebView UI invoking linked native Rust commands;
- native Rust commands calling OxIde/OxVba adapters;
- host capability profiles surfacing wasm-safe vs native vs Windows COM support honestly;
- automation driving the real hosted app, not a disconnected static HTML snapshot.

Static visual artifacts remain useful review evidence, but they are not the app and must not become the main implementation track.
