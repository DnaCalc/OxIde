# W001 Execution Doctrine And Rewrite Bootstrap

Status: `active`
Date: 2026-04-02

## 1. Purpose
This workset establishes the OxIde-local execution doctrine and the first real
rewrite map for the repo.

It is the packet that:
1. adapts the OxFunc workset/beads pattern to OxIde,
2. defines the local operations and bead method,
3. creates the live workset register,
4. turns the rewrite/reuse recommendation into explicit worksets and initial beads.

## 2. Why This Exists
OxIde now has:
1. an authoritative product-direction document,
2. an aligned architecture document,
3. a review-based recommendation to rebuild the shell while salvaging specific semantic integration value.

What it lacked was:
1. a coherent execution doctrine for that phase,
2. a workset-level ordered map,
3. an explicit bridge from review findings into executable work.

## 3. Closure Condition
This workset closes when:
1. `OPERATIONS.md` exists and is live,
2. `docs/BEADS.md` exists and is live,
3. `docs/WORKSET_REGISTER.md` exists and is live,
4. `docs/worksets/` contains the first active workset set for the rewrite phase,
5. the first execution beads under those worksets exist in `.beads/`.

## 4. Initial Epic Lanes
1. doctrine bootstrap
2. register bootstrap
3. rewrite-map rollout
