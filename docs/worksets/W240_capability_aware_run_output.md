# Workset W240 — Capability-Aware Run And Output

## Ambition

The GUI exposes run/output behavior through explicit host capability profiles: unsupported hosts explain why they cannot run, while supported native paths produce structured run/output events.

## Dependencies

- W230 — document lifecycle baseline.
- OxVba build/run host contracts.
- Capability profile model.

## Design

Run should be a visible IDE workflow, not a raw log append.

Likely implementation lanes:

1. runtime capability profile surface,
2. run request/event model,
3. output/activity panel,
4. browser unsupported/limited path,
5. native or simulated-native supported path,
6. deterministic run-output fixture.

## Beads

This scaffold is not execution-ready. Expand before implementation.

Expected bead lanes:

1. W240-B01 — runtime capability model and disabled reasons.
2. W240-B02 — run request/event protocol.
3. W240-B03 — output/activity surface.
4. W240-B04 — browser unsupported run scenario.
5. W240-B05 — supported run-output scenario.

## Out-of-scope

- Debugger surfaces.
- Immediate Window.
- Windows COM invocation.
- DnaOneCalc embedding.
