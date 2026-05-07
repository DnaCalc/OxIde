# Workset W250 — DnaOneCalc Embedded IDE And Runtime Proof

## Ambition

DnaOneCalc can consume OxIde IDE surface/contract or OxIde-authored artifacts without owning OxIde semantics.

## Dependencies

- W240 — capability-aware run/output path.
- DnaOneCalc coordination through handoff or separate repo-scoped work.
- Clean dependency-direction decision.

## Design

The integration should prove the DNA Calc host model without making OxIde a DnaOneCalc submodule by accident.

Proof ladder:

1. artifact/runtime proof,
2. embedded editor proof,
3. shared component proof.

Likely implementation lanes:

1. decide dependency direction,
2. identify shared component/bridge boundary,
3. produce OxIde-side contract and fixtures,
4. create DnaOneCalc handoff or paired integration smoke,
5. verify OxVba remains semantic/runtime owner.

## Beads

This scaffold is not execution-ready. Expand before implementation.

Expected bead lanes:

1. W250-B01 — dependency direction and contract decision.
2. W250-B02 — OxIde artifact/runtime proof packet.
3. W250-B03 — embedded editor component boundary.
4. W250-B04 — DnaOneCalc handoff or paired smoke plan.
5. W250-B05 — integration evidence close.

## Out-of-scope

- General third-party host embedding.
- Making DnaOneCalc own OxIde semantics.
- Editing sibling repos from an OxIde-scoped agent run.
