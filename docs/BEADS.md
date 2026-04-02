# OxIde Beads Working Method

## 1. Purpose
This file defines the local bead method for OxIde.

It covers:
1. the local execution model,
2. the `br` / `bv` split,
3. the mutation rule,
4. OxIde-specific bead quality expectations,
5. the `workset -> epic -> bead` rollout rule.

## 2. Core Model
Execution in OxIde is:
1. [WORKSET_REGISTER.md](WORKSET_REGISTER.md)
2. `workset -> epic -> bead`
3. `.beads/` as the detailed execution truth

Interpretation rule:
1. worksets are planning and scope-partition units,
2. epics are the main execution lanes under a chosen workset,
3. beads are the unit of executable progress,
4. worksets do not own ready/in-progress/blocked/closed state,
5. `.beads/` is the live execution-state surface.

## 3. Tool Split
`br` is the mutation tool.

Use it to:
1. inspect ready work,
2. create beads,
3. update status,
4. add dependencies,
5. close completed work.

Typical commands:

```powershell
br ready
br show <id>
br create --title "..." --type task --priority 2
br update <id> --status in_progress
br close <id> --reason "Completed"
br dep add <issue> <depends-on>
```

`bv` is the graph-aware inspection tool.

Use it to:
1. inspect ready paths,
2. inspect blockers,
3. inspect graph shape and pressure.

## 4. Mutation Rule
Do not edit `.beads/` files directly.

When `.beads/` exists:
1. use `br` for mutations,
2. use `bv` or read-only `br` for inspection,
3. keep execution-state truth out of ad hoc notes.

## 5. OxIde Bead Quality Bar
Every executable OxIde bead should state:
1. one reviewable outcome,
2. the truth surfaces touched,
3. real dependency relationships,
4. closure evidence that fits the work.

For OxIde, closure evidence usually means some combination of:
1. Rust code,
2. tests,
3. updated architecture or product-direction docs,
4. updated workset/register truth when planning changes,
5. verification against FrankenTui or direct OxVba host-session behavior.

Bad beads:
1. vague activity without a reviewable outcome,
2. broad cleanup themes disguised as one executable issue,
3. mini-worksets hidden inside one bead,
4. documentation-only closure for code-bearing scope unless the bead is explicitly doctrinal.

## 6. Typical OxIde Epic Shapes
Typical OxIde epics:
1. shell/action-system lane,
2. buffer/view/layout lane,
3. editor-surface lane,
4. project/workspace/file-management lane,
5. direct OxVba semantic-integration lane,
6. console UX and first-run lane,
7. cleanup/migration lane.

## 7. Rollout Rule
Any workset chosen for execution should roll out into one or more epics.

Rollout pattern:
1. some epics should expand directly into execution beads,
2. some may begin with a rollout bead when the child set still needs to be formed,
3. obvious implementation work should be expanded directly,
4. the graph should stay explicit.

## 8. Closure Rule
A bead closes only when:
1. the stated outcome exists,
2. the stated evidence exists,
3. any newly discovered necessary work is added back into the graph,
4. touched truth surfaces are updated.

Do not close a bead because the prototype moved forward “enough”.
