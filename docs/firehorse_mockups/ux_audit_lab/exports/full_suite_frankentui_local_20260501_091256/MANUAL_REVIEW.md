# Manual Review Notes

Scope: Fire Horse default audit export generated during the local
FrankenTui skeleton pass.

Evidence root:
`docs/firehorse_mockups/ux_audit_lab/exports/full_suite_frankentui_local_20260501_091256`

Release evidence:

- `target/release/ox-ide.exe --firehorse-skeleton --firehorse-screen firehorse-run-lane-standard`
  rendered a live FrankenTui Run Lane screen in a PTY and exited cleanly
  with `q`.
- `target/release/oxide-uxlab.exe --audit --batch docs/firehorse_mockups/ux_audit_lab/agent_run.json --json`
  produced 14 scorecards for the seven designed screens across Studio
  and First-class.

Screen review:

| Screen | Artifact | Review note |
| --- | --- | --- |
| Launchpad | `firehorse-launchpad-standard_studio/captures/firehorse-launchpad-standard_studio.txt` | Start posture, recent work, fit context, and key rail are visible without relying on project state. |
| Editing Lens | `firehorse-editing-lens-standard_studio/captures/firehorse-editing-lens-standard_studio.txt` | Source remains dominant while project spine, context dock, activity deck, and command rail stay visible. |
| Command Lens | `firehorse-command-lens-standard_first-class/captures/firehorse-command-lens-standard_first-class.txt` | Overlay, command rows, preview, action ids, and disabled reason posture are present at First-class width. |
| Run Lane | `firehorse-run-lane-standard_studio/captures/firehorse-run-lane-standard_studio.txt` | Run timeline, source context, run context, activity deck, and stop/immediate commands are visible. |
| Debug Cockpit | `firehorse-debug-cockpit-standard_studio/captures/firehorse-debug-cockpit-standard_studio.txt` | Paused source line, call stack, locals, watches, and debug command rail are visible in one cockpit. |
| Console Fit | `firehorse-console-fit-light_studio/captures/firehorse-console-fit-light_studio.txt` | Capability rows and recommendations render as text-first terminal evidence, not color-only signals. |
| Compact Focus | `firehorse-focus-compact_compact/captures/firehorse-focus-compact_compact.txt` | Compact posture keeps source, activity rail, and key rail while deliberately hiding side docks. |

Audit result:

- Objective preflight checks report no failures for the exported default
  viewports.
- The all-screen Studio/First-class batch reports no failures and one
  concern per scorecard.
- The remaining concern is the Audit Lab's manual `aesthetic.emotional_fit`
  gate. These notes document the first manual pass, but they do not
  change the scorecard gate; downstream implementation should still cite
  the scorecards and this review note together.
