# AGENTS.md — OxIde

Repo-local safety rules for agent work.

For the live execution model, workset doctrine, and bead method, use:
- [`OPERATIONS.md`](/C:/Work/DnaCalc/OxIde/OPERATIONS.md)
- [`docs/BEADS.md`](/C:/Work/DnaCalc/OxIde/docs/BEADS.md)
- [`docs/WORKSET_REGISTER.md`](/C:/Work/DnaCalc/OxIde/docs/WORKSET_REGISTER.md)

For product and architectural authority, use:
- [`PRODUCT_DIRECTION.md`](/C:/Work/DnaCalc/OxIde/PRODUCT_DIRECTION.md)
- [`ARCHITECTURE.md`](/C:/Work/DnaCalc/OxIde/ARCHITECTURE.md)

## 1. Absolute File Deletion Rule

You may NOT delete any file or directory unless the user gives the
exact delete command in this session.

This is a hard invariant.

Includes:
- files you just created,
- generated files,
- temporary files,
- directories you believe are safe to remove.

If removal seems appropriate:
1. stop,
2. ask for the exact command,
3. do not run or propose a destructive command without that explicit
   approval.

## 2. Irreversible Actions

Absolutely forbidden unless the user gives the exact command and
explicit approval in the same message:
- `git reset --hard`
- `git clean -fd`
- `rm -rf`
- any destructive overwrite or deletion command

If destructive approval is given, record:
1. the exact user authorization text,
2. the exact command run,
3. when it was run.

## 3. Public Posting Rule

Any public post on GitHub or elsewhere must be individually approved
by the user before posting.

Every approved public post must begin with:

*Posted by Codex on behalf of @govert*

## 4. Development Method

All execution follows the workset + bead model defined in
[`OPERATIONS.md`](/C:/Work/DnaCalc/OxIde/OPERATIONS.md) and
[`docs/BEADS.md`](/C:/Work/DnaCalc/OxIde/docs/BEADS.md).

Worksets partition ambition, scope, and sequence. Beads are the
atomic execution unit (goal / design / tests / evidence / closure)
and close only when every item on their closure checklist ticks.
Commit messages describe only behaviours the author has personally
seen on the running release binary.

Use `br` to mutate bead state. Use `bv` only in non-interactive /
robot-style ways from agent sessions. Do not edit `.beads/` files
directly, and do not create a parallel TODO or blocker system.

Do not treat workset docs as live blocker trackers. Workset specs own
scope and design intent; `.beads/` owns live execution state.

For product and architectural authority see
[`PRODUCT_DIRECTION.md`](/C:/Work/DnaCalc/OxIde/PRODUCT_DIRECTION.md)
and [`ARCHITECTURE.md`](/C:/Work/DnaCalc/OxIde/ARCHITECTURE.md).
