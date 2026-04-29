# Fire Horse UX Audit Handoff

This document defines how downstream OxIde work uses UX Audit Lab
evidence. The lab is local evidence only. It does not create public
posts, GitHub issues, or product behavior.

## Gate States

| Gate | Meaning | Downstream use |
| --- | --- | --- |
| `ready` | Selected functional and aesthetic criteria pass, or only explicitly deferred criteria remain. | A downstream bead may cite the scorecard, then still verify its own release-binary behavior before closing. |
| `ready_with_dependency` | The design is acceptable if the named downstream owner implements a known future seam or behavior. | The downstream bead must cite the dependency and keep the UI honest until the seam is real. |
| `concern` | The design is usable for review but has unresolved aesthetic, seam, density, command, or evidence concerns. | Implementation may prototype against it, but cannot claim the design is final. |
| `blocked_missing_seam` | A visible VBA/project fact lacks a real, future, unavailable, or not-required seam. | Product implementation is blocked until the seam is named or removed from the UI claim. |
| `rejected_scope` | The finding asks W041 to implement product behavior owned by W040/W050/W060/W070/W080/W090/W100. | File or update the downstream bead instead of expanding the audit lab. |

`oxide-uxlab` currently reports `ready`, `concern`, and `blocked` in
scorecards. The richer names above are the handoff vocabulary used in
downstream bead designs and review notes.

## Downstream Owners

| Area | Owner | Audit evidence to cite |
| --- | --- | --- |
| Project mounting and launch posture | W040 | Launchpad, Real Editing Adapter, Project Spine mappings. |
| Document lifecycle and source canvas | W050 | Editing Lens, Compact Focus, Real Editing Adapter source rows. |
| Diagnostics, hover, references, symbols | W060 | Editing Lens context dock and semantic seam rows. |
| Run, Immediate, and output activity | W070 | Run Lane timeline and `WebHostEvent` mapping. |
| Debug state | W080 | Debug Cockpit paused line, stack, locals, watches, and future debug seam. |
| Command system | W090 | Command Lens action ids, disabled reasons, and key rail mappings. |
| Terminal capability and degradation | W100 | Console Fit and Compact Focus viewport evidence. |
| Polish, recovery, and onboarding | W110 | Launchpad posture, status explanations, recovery labels. |

## Citation Format

Downstream beads should cite audit evidence in this form:

```text
Audit scenario: audit-editing-lens-pricing
Fire Horse scenario: firehorse-editing-lens-standard
Viewport: studio
Scorecard: target/release/oxide-uxlab.exe --audit --suite firehorse --scenario firehorse-editing-lens-standard --viewport studio --evaluate functional,aesthetic --json
Gate: concern
Export pack: docs/firehorse_mockups/ux_audit_lab/exports/<pack>
Finding: aesthetic.emotional_fit concern, or named functional criterion
Observed downstream behavior: <release-binary behavior seen by the bead author>
```

Agent-generated scorecards can select and explain design work, but they
do not replace the downstream author's release-binary observation duty.
If a bead claims product behavior, the author must have personally seen
that behavior in the running release binary.

## Ready-To-Use Commands

```text
target/release/oxide-uxlab.exe --audit --suite firehorse --matrix --json
target/release/oxide-uxlab.exe --audit --suite firehorse --scenario firehorse-editing-lens-standard --viewport studio --brief --json
target/release/oxide-uxlab.exe --audit --suite firehorse --scenario firehorse-editing-lens-standard --viewport studio --evaluate functional,aesthetic --json
target/release/oxide-uxlab.exe --audit --suite firehorse --scenario firehorse-editing-lens-standard --viewport studio --export docs/firehorse_mockups/ux_audit_lab/exports/editing_lens_studio --json
target/release/oxide-uxlab.exe --audit --batch docs/firehorse_mockups/ux_audit_lab/agent_run.json --json
```

Exit code `1` means the command succeeded but the audit gate is concern
or blocked. Automation should parse stdout before deciding whether to
iterate, file follow-up work, or hand the evidence to a reviewer.

Batch automation writes scorecards and agent briefs under the
`output_root` named by the batch fixture, using a unique `batch_runs`
subdirectory for each run.
