# Fire Horse UX Audit Lab

This directory holds local UX Audit Lab evidence and agent-facing run
fixtures for the Fire Horse terminal IDE proof. The lab lives in
`oxide-uxlab`; it reviews the Fire Horse mockup renderer and projection
contracts without changing the production `ox-ide` renderer.

## Design Calibration

`DESKTOP_TUI_OXVBA_WORKLOAD_CALIBRATION.md` is the high-end calibration
addendum for this lab. It translates the richer desktop-class TUI IDE
benchmark into OxIde's simpler OxVba workload: project/module truth,
source editing, diagnostics, source lenses, run/debug, Immediate,
generated-code honesty, command truth, and terminal capability.

Use it when reviewing whether Studio and First-class mockups feel like a
real daily VBA IDE workspace rather than a tidy collection of named
panels.

## Agent Loop

```text
target/release/oxide-uxlab.exe --audit
target/release/oxide-uxlab.exe --audit --suite firehorse --list --json
target/release/oxide-uxlab.exe --audit --suite firehorse --matrix --json
target/release/oxide-uxlab.exe --audit --suite firehorse --scenario firehorse-editing-lens-standard --viewport studio --brief --json
target/release/oxide-uxlab.exe --audit --suite firehorse --scenario firehorse-editing-lens-standard --viewport studio --once --json
target/release/oxide-uxlab.exe --audit --suite firehorse --scenario firehorse-editing-lens-standard --viewport studio --evaluate functional,aesthetic --json
target/release/oxide-uxlab.exe --audit --suite firehorse --scenario firehorse-editing-lens-standard --viewport studio --export docs/firehorse_mockups/ux_audit_lab/exports/editing_lens_studio --json
target/release/oxide-uxlab.exe --audit --suite firehorse --export docs/firehorse_mockups/ux_audit_lab/exports/full_suite --json
target/release/oxide-uxlab.exe --audit --batch docs/firehorse_mockups/ux_audit_lab/agent_run.json --json
cargo test --features wtd --test wtd_smoke audit_lab
```

`--audit` without another mode opens the interactive FrankenTui cockpit.
Use `Tab`, `j`/`k`, `v`, `r`, `1`..`5`, `p`/`c`/`f`/`d`, `e`, and
`q` inside that surface.

Exit codes are part of the automation contract:

| Code | Meaning |
| --- | --- |
| `0` | Command succeeded and the selected audit gate is ready. |
| `1` | Command succeeded, but the scorecard gate is concern or blocked. |
| `2` | Command shape, suite, scenario, viewport, JSON, or IO input failed. |
| `3` | Render or capture failed. |

The current scorecard intentionally treats emotional fit as a structured
manual concern. That keeps agents honest: objective checks can pass, but
the high-end Fire Horse feeling still needs cited review evidence before
downstream implementation can treat it as ready.

## Batch Fixture

`agent_run.json` is the default agent smoke run. It covers Editing Lens
and Command Lens across Studio and First-class viewports with functional
and aesthetic checks enabled. It writes batch evidence under
`target/ux_audit_lab/agent_run/batch_runs/<run>/` so repeated automation
runs do not overwrite committed review packs.
