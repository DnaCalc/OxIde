# Handoff W352 — Tauri/WebView Desktop Product Host And Automation

Date: 2026-05-08
Workset: W352 — DnaOxIde Tauri/WebView Product Host And Automation

## Acceptance Summary

W352 accepts Tauri/WebView2 as the DnaOxIde desktop product-host regression lane.

Desktop product evidence now means evidence from the release Tauri executable, the embedded WebView2 surface, and linked Rust Tauri commands. Browser/W350 DOM evidence remains useful as regression/reference material, but it is not a substitute for product-host evidence.

## Accepted Desktop Evidence Lane

```text
DnaOxIde release executable
  ├─ Tauri shell
  ├─ WebView2 UI surface
  ├─ WebView2 CDP automation for inspection/injection
  └─ linked Rust Tauri commands
      ├─ dna_oxide_desktop_host_capabilities_probe
      ├─ dna_oxide_save_active_module
      └─ dna_oxide_reload_active_module
```

## Evidence Artifacts

- `target/w352-b01-tauri-shell-start.txt`
- `target/w352-b02-webview-automation.txt`
- `target/w352-b02-performance-size-baseline.txt`
- `target/w352-b02-webview-state.json`
- `target/w352-b02-webview-after-probe.png`
- `target/w352-b03-tauri-edit-save-reload.txt`
- `target/w352-b03-tauri-edit-save-reload-state.json`
- `target/w352-b03-tauri-edit-save-reload.png`
- `target/w352-acceptance.txt`

## Regression Commands

Run from `apps/dna-oxide` unless noted:

```text
npm run tauri:build
npm run tauri-command-spine:check
npm run webview-automation:check
npm run tauri-edit-save-reload:check
npm run scaffold:check
```

Run from `apps/dna-oxide/src-tauri`:

```text
cargo test
```

## Browser Harness Boundary

W350 browser DOM artifacts remain reference/regression aids for shared UI behavior and instrumentation shape.

They must not be used to claim desktop-native product behavior. W355+ desktop-native adapter acceptance must use the Tauri/WebView2 product lane and linked Rust commands.

## Native Capability Boundary

W352 proves:

- the release Tauri desktop app starts;
- WebView2 automation can observe and interact with the real desktop UI;
- a UI command reaches linked Rust through Tauri;
- edit/save/reload can be driven through the real WebView;
- save/reload are backed by linked native Rust commands over temp project copies;
- checked-in fixtures remain unchanged.

W352 does not prove:

- real/native OxVba compile/build execution;
- runtime execution;
- Immediate evaluation;
- debug/watch/breakpoint behavior;
- COM runtime invocation;
- DnaOneCalc mount;
- production installer/package quality.

Those remain gated by W355-W370.

## Performance And Size Baseline

The current baseline is intentionally early and should be carried into downstream work as a visible budget surface.

Recent W352 evidence includes:

- release executable size: about 8.9 MB;
- frontend dist size: about 74 KB;
- cold start to first observable UI: about 1.0–1.6 seconds in observed checks;
- native host-probe round trip: about 59 ms in the latest B02 rerun;
- native save round trip: about 88 ms in the B03 acceptance run;
- native reload round trip: about 52 ms in the B03 acceptance run;
- process-tree working set: about 376–402 MB, mostly WebView2 child processes.

The process-tree working set is not the same as private application memory. Downstream performance work should split at least:

- Rust host process working set/private bytes;
- WebView2 child process aggregate working set/private bytes;
- cold-start timing over multiple samples;
- command round-trip timings over multiple samples.

Follow-up beads should be filed for sluggishness, bloat, startup regressions, or interaction latency regressions.

## W355+ Handoff

W355 may now use W352 as the desktop-native product seam:

```text
WebView UI -> Tauri command -> linked Rust adapter -> typed result
```

No CLI/LSP/browser-only fallback should be accepted for desktop-native compile/build/runtime/debug/COM product claims unless a later workset explicitly changes the architecture.
