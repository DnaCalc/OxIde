# OxIde Target Stack Scenarios

Status: `active_target_stack`
Date: 2026-05-08

This document records the target stack clarified after W350 acceptance. It is subordinate to `CHARTER.md`, `PRODUCT_DIRECTION.md`, and `ARCHITECTURE.md`.

## Summary

OxIde must support one shared IDE surface across three product scenarios:

```text
Shared Rust IDE/editor/UI core
  ├─ Browser website: DnaOneCalc WASM + embedded OxIde + OxVba wasm-safe compiler/runtime
  ├─ Standalone Windows desktop: DnaOxIde Tauri/WebView + linked native Rust OxVba backend
  └─ DnaOneCalc Windows desktop: DnaOneCalc shell embeds OxIde + linked/exposed native OxVba backend
```

The browser scenario is a real product target, not a toy mode. It must support compile/run for the wasm-safe OxVba profile. The desktop scenarios add native and Windows COM capabilities.

## Scenario A — Browser Website / DnaOneCalc WASM Host

Shape:

```text
Browser tab
  └─ DnaOneCalc WASM app
      ├─ DnaOneCalc product UI
      ├─ embedded OxIde IDE surface
      └─ OxVba wasm-safe compiler/runtime profile
```

Required capabilities:

- open OxIde from the DnaOneCalc browser app;
- edit code in the embedded IDE;
- load/save source through browser-supported mechanisms and DnaOneCalc host policy;
- compile/check through OxVba wasm-safe compiler APIs;
- run/invoke supported functions directly inside the DnaOneCalc WASM app;
- surface native-only features as unavailable with clear disabled reasons.

Forbidden assumptions:

- Windows COM;
- native process spawning;
- native filesystem as a requirement;
- desktop IPC/Tauri as a requirement;
- fake runtime, Immediate, debug, or COM data.

## Scenario B — Standalone Windows DnaOxIde Desktop

Shape:

```text
DnaOxIde desktop app
  ├─ WebView UI using the shared OxIde surface
  └─ native Rust command layer linked into the Tauri app
      ├─ OxIde adapters
      ├─ OxVba native compiler/runtime/debug APIs
      ├─ native filesystem/project services
      └─ Windows COM-capable services where supported
```

Default meaning of native backend:

> Rust code compiled into and linked with the Tauri app crate, invoked from the WebView through typed Tauri commands.

A separate process is not the default. It becomes an explicit design choice only if COM apartment policy, crash isolation, multi-host sharing, or long-lived runtime isolation demands it.

## Scenario C — DnaOneCalc Windows Desktop Host Embedding OxIde

Shape:

```text
DnaOneCalc desktop app
  ├─ DnaOneCalc product shell
  ├─ embedded OxIde IDE surface
  └─ native Rust backend exposing DnaOneCalc + OxVba services
```

DnaOneCalc owns host policy and when/where OxIde is shown. OxIde supplies the IDE/editor surface and commands. OxVba supplies compiler/runtime/debug/COM truth.

This scenario should reuse the same shared UI and command/capability vocabulary as DnaOxIde desktop, with host-specific policy supplied by DnaOneCalc.

## OxVba Capability Split Required

OxVba needs explicit, testable profiles:

### wasm-safe profile

- parser/binder/semantic services;
- compiler/check APIs usable on `wasm32-unknown-unknown` or equivalent browser-compatible target;
- interpreter/bytecode/runtime path capable of invoking supported functions in the browser host;
- no native filesystem requirement;
- no COM/native interop;
- deterministic typed unavailable states for native-only APIs.

### native desktop profile

- native filesystem/project services;
- native compiler/runtime/debug services;
- wrapped native binary execution where supported;
- Windows COM discovery/invocation where supported;
- capability-gated unavailable states on non-Windows or restricted hosts.

OxIde may request coordinated OxVba changes to establish or harden these profiles.

## Technology Choices

Preferred current choices:

- Rust for shared core and adapter code;
- Leptos or equivalent Rust/WASM-capable UI for shared UI direction;
- `wasm-bindgen`/browser-compatible bindings where the website host needs JS/WASM interop;
- Tauri for desktop host shells;
- Tauri typed commands for WebView UI to linked native Rust calls;
- Playwright for browser-host automation and, where possible, desktop/WebView regression automation.

## Course-Correction Rule

Browser automation is allowed only when it exercises a real product seam:

- browser WASM OxIde/OxVba integration;
- DnaOneCalc host embedding;
- shared UI behavior intended for both browser and desktop;
- or regression automation around the real hosted app.

Browser-only static snapshots are review artifacts, not implementation milestones. Worksets after W350 must not extend browser-only substitutes when the real endpoint requires browser WASM host integration or Tauri/native command integration.
