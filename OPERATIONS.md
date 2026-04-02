# OPERATIONS.md - OxIde Operations

## 1. Purpose
This document defines the local execution model for OxIde.

It is intentionally lightweight.

Its job is to keep OxIde execution coherent while the repo is being reoriented
around:
- `PRODUCT_DIRECTION.md`
- `ARCHITECTURE.md`
- a workset-driven planning surface
- `.beads/` as the live execution-state surface

## 2. Precedence
Distinguish between:
1. repo guardrails, and
2. plan and execution authority.

Repo guardrails:
1. `AGENTS.md`

Plan and execution authority:
1. `PRODUCT_DIRECTION.md`
2. `ARCHITECTURE.md`
3. this file (`OPERATIONS.md`)
4. `docs/WORKSET_REGISTER.md`
5. `docs/BEADS.md`
6. individual workset packets under `docs/worksets/`

Rule:
- `AGENTS.md` governs repo safety, destructive-action limits, and session discipline
- `PRODUCT_DIRECTION.md` is authoritative for what OxIde is trying to become
- `ARCHITECTURE.md` is authoritative for how the code should be divided
- `.beads/` is authoritative for live execution state

## 3. Operating Principles
1. Prefer a clean architecture over preserving prototype shape.
2. Salvage real value; do not preserve accidental structure.
3. Keep OxIde project/session/editor/UI ownership explicit.
4. Keep OxVba as the owner of project truth and semantics.
5. Prefer direct host/session integration over CLI- or LSP-shaped indirection for editor semantics.
6. Keep process small, but keep planning and execution traceable.
7. Do not let workset packets become a second blocker tracker.
8. Use worksets for scope and sequencing, beads for execution.
9. Prefer reviewable vertical outcomes over broad speculative scaffolding.
10. When prototype code conflicts with current direction, current direction wins.

## 4. Execution Model
Execution in OxIde is:
1. `docs/WORKSET_REGISTER.md`
2. `workset -> epic -> bead`
3. `.beads/` as the detailed execution truth

Interpretation rule:
1. worksets are planning and scope-partition units,
2. epics are the main execution lanes under a chosen workset,
3. beads are the unit of executable progress,
4. worksets do not own ready/in-progress/blocked/closed state,
5. `.beads/` owns live execution state.

## 5. Tool Split
`br` is the mutation tool.

Use it to:
1. inspect ready work,
2. create and update beads,
3. add dependencies,
4. close completed beads.

`bv` is the graph-aware triage tool.

Use it to:
1. inspect graph shape,
2. inspect blockers and ready paths,
3. reason about execution order.

Agent rule:
1. prefer non-interactive calls,
2. do not use `.beads/` files as an editable planning surface.

## 6. Workset Rule
Worksets exist to make rewrite and salvage planning explicit.

They should state:
1. purpose,
2. dependency order,
3. governing truth surfaces,
4. intended execution lanes,
5. closure condition.

They should not carry:
1. day-to-day execution state,
2. blocker ledgers,
3. duplicate bead status.

## 7. Current OxIde Execution Posture
OxIde is in a re-foundation phase.

That means:
1. the existing implementation is treated as a spike/reference, not a fixed base,
2. the shell/application structure is expected to be rebuilt,
3. direct OxVba host-session integration and a subset of behavioral tests are expected salvage targets,
4. stale prototype assumptions should be removed rather than preserved.

## 8. Closure Rule
A bead closes only when:
1. the stated reviewable outcome exists,
2. the required code/docs/tests exist,
3. any newly discovered follow-up work is added back into the graph,
4. touched truth surfaces are updated.

Do not close a bead because “enough progress happened”.

## 9. Relationship To Other Docs
1. `PRODUCT_DIRECTION.md`
   - product and UX authority
2. `ARCHITECTURE.md`
   - seam and implementation-direction authority
3. `docs/WORKSET_REGISTER.md`
   - ordered workset authority
4. `docs/BEADS.md`
   - local bead working method
5. `docs/worksets/*`
   - scope and provenance packets for major execution lanes
