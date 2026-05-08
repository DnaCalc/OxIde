# Planning Reset — W349 to Live DnaOxIde App

Date: 2026-05-07
Status: `superseded_by_2026_05_08_target_stack_alignment`

## Why This Reset Exists

W341-W349 built the DnaOxIde host runway, but the accepted evidence is still static/review-oriented. There is not yet a live app where a user can type into a source editor and save/reload a source file through the UI.

The planning reset makes that gap explicit and changes the next active lane accordingly.

## New Active Sequence

The next active GUI/DnaOxIde sequence is now:

1. **W350 — DnaOxIde live editable source app**
   - first-class visual and DOM-like instrumentation,
   - interaction injection,
   - command/event trace capture,
   - live source text editing,
   - dirty/save/reload over temp project copies.
2. **W352 — DnaOxIde Tauri/WebView product host and automation**
   - desktop product-host lane proving UI -> Tauri/WebView -> linked native Rust command flow.
3. **W355 — OxVba compile/build adapter profiles**
   - OxVba-backed browser/WASM compile/check profile and native desktop compile/build profile, with typed options, run targets, build/check status, request IDs, lifecycle events, and diagnostics.
4. **W360 — DnaOxIde COM/reference adapter**
   - OxVba-backed reference roster, COM candidates, repair/reorder plans, capability profile, and runtime availability status.
5. **W365 — DnaOxIde runtime/Immediate adapter**
   - OxVba-backed runtime sessions, runtime IDs/events, Immediate attach/session IDs, and typed Immediate responses.
6. **W370 — DnaOxIde debug/watch/breakpoint adapter**
   - OxVba-backed debug sessions, command states, callstack/locals, watch registry/evaluation, and breakpoint binding records.

## Instrumentation Requirement

W350 must put instrumentation in place before the live editing loop is accepted. The app must be observable and automatable through:

- a visual artifact for human review,
- a DOM-like snapshot with stable roles/data attributes,
- an ordered event/state log,
- a command log for host calls and disabled reasons,
- interaction injection for focus, typing, save, reload, and command dispatch,
- before/after snapshot capture for every injected interaction,
- no-claim flags in every artifact.

This instrumentation enabled the W350 feedback loop. After the 2026-05-08 target-stack clarification, it is retained as a regression/review harness only. New product work must exercise real endpoint seams: DnaOneCalc browser WASM host integration, Tauri/WebView to linked native Rust commands, or OxVba adapters.

## Parked Legacy Beads

Open legacy TUI/UX-lab beads from W038, W041, W050, W060, and W070 were not closed. They were labeled:

```text
parked-tui
planning-reset-2026-05-07
```

and deferred pending explicit resume/replan. This preserves their history without letting parked lineage appear as the next active execution path.

## No-Claim Boundaries

The reset does not authorize or claim:

- live Tauri/WebView IPC unless W350 proves it,
- browser event-loop/Playwright/WebDriver coverage unless W350 proves it,
- full DOM accessibility audit,
- real/native OxVba runtime execution,
- real Immediate evaluation,
- real debug/watch/breakpoint behavior,
- COM runtime invocation,
- real DnaOneCalc product mount,
- sibling repo writes,
- fake runtime/Immediate/debug/COM data.

## Superseding Target-Stack Alignment

The active plan is now recorded in:

- [`CHARTER.md`](../CHARTER.md),
- [`PRODUCT_DIRECTION.md`](../PRODUCT_DIRECTION.md),
- [`ARCHITECTURE.md`](../ARCHITECTURE.md),
- [`docs/OXIDE_TARGET_STACK_SCENARIOS.md`](OXIDE_TARGET_STACK_SCENARIOS.md).

W350 is complete. The next product-path work is W352 Tauri/WebView product-host bootstrap, followed by W355 compile/build adapter profiles.
