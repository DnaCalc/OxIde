# Workset W270 — Run, Debug, And Immediate GUI Surfaces

## Ambition

Run, Immediate, and debug surfaces become coherent GUI IDE workflows over real OxVba seams or explicit future/unavailable seams.

## Dependencies

- W240 — capability-aware run/output path.
- W260 — COM capability proof where COM-sensitive runtime state matters.
- OxVba Immediate and debug session APIs.

## Design

The GUI should preserve source continuity while moving through run, Immediate, and debug postures.

Likely implementation lanes:

1. run timeline refinement,
2. Immediate panel tied to active runtime session,
3. debug paused-state projection,
4. callstack/locals/watch/breakpoint surfaces,
5. future/unavailable seam honesty where OxVba support is incomplete.

## Beads

This scaffold is not execution-ready. Expand before implementation.

Expected bead lanes:

1. W270-B01 — run timeline and output refinement.
2. W270-B02 — Immediate panel over runtime session.
3. W270-B03 — debug paused-state projection.
4. W270-B04 — callstack/locals/watch surfaces.
5. W270-B05 — run/debug/immediate acceptance scenarios.

## Out-of-scope

- Full debugger parity if OxVba seams are not ready.
- Fake debug truth.
- General telemetry.
