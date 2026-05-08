# Planning Reset — W349 to Live DnaOxIde App

Date: 2026-05-07
Status: `active_reset_applied`

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
2. **W352 — DnaOxIde Tauri/WebView automation**
   - planned desktop-host automation lane reusing W350 instrumentation once Tauri/WebView tooling is ready.
3. **W355 — DnaOxIde compile/build adapter**
   - OxVba-backed compile options, run targets, typed build/check status, request IDs, lifecycle events, and diagnostics.
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

This instrumentation is what enables a fast feedback loop: automate the app, observe the effect, adjust, and rerun.

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

## Ready Bead After Reset

The intended next ready bead is:

```text
W350-B00 — Live editable proof mode decision
```

It must choose the proof/driver stack and document the instrumentation contract before implementation proceeds.
