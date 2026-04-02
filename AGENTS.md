# AGENTS.md — OxIde

This file defines the repo-local guardrails for OxIde agent work.

For the live execution model, workset doctrine, and bead method, use:
- [OPERATIONS.md](/C:/Work/DnaCalc/OxIde/OPERATIONS.md)
- [docs/BEADS.md](/C:/Work/DnaCalc/OxIde/docs/BEADS.md)
- [docs/WORKSET_REGISTER.md](/C:/Work/DnaCalc/OxIde/docs/WORKSET_REGISTER.md)

For product and architecture authority, use:
- [PRODUCT_DIRECTION.md](/C:/Work/DnaCalc/OxIde/PRODUCT_DIRECTION.md)
- [ARCHITECTURE.md](/C:/Work/DnaCalc/OxIde/ARCHITECTURE.md)

## 1. Absolute File Deletion Rule

You may NOT delete any file or directory unless the user gives the exact delete
command in this session.

This is a hard invariant.

That includes:
- files you just created,
- generated files,
- temporary files,
- directories you believe are safe to remove.

If removal seems appropriate:
1. stop,
2. ask for the exact command,
3. do not run or propose a destructive command without that explicit approval.

## 2. Irreversible Actions

Absolutely forbidden unless the user gives the exact command and explicit
approval in the same message:
- `git reset --hard`
- `git clean -fd`
- `rm -rf`
- any destructive overwrite or deletion command

If destructive approval is given, record:
1. the exact user authorization text,
2. the exact command run,
3. when it was run.

## 3. Public Posting Rule

Any public post on GitHub or elsewhere must be individually approved by the
user before posting.

Every approved public post must begin with:

*Posted by Codex on behalf of @govert*

## 4. Current Repo Direction

OxIde is currently being rebuilt as a standalone terminal-native IDE for OxVba.

Important current direction:
- FrankenTui is the shell and editor foundation
- OxIde owns UX, shell flow, buffers/views/layouts, and editor orchestration
- OxVba owns project truth and semantic meaning
- direct host/session integration is preferred over CLI- or LSP-shaped editor semantics
- prototype structure is not sacred; current direction is authoritative

Do not use older notes about embedded-host-first design or `msedit` as an
architectural reference point.

## 5. Development Rules

- Primary implementation language: Rust.
- Primary console shell and editing foundation: FrankenTui.
- This is not a general JS/TS repo. Introduce JS/TS only for a specific,
  documented need.
- Prefer small, explicit edits over bulk rewrite scripts.
- Do not run ad hoc codemods or large regex refactors.
- Prefer a clean architecture over backward-compatibility shims.
- Do not proliferate new files unless they represent a real architectural split.

## 6. Beads Rule

All execution-state tracking goes through `.beads/`.

Rules:
- do not edit `.beads/` files directly,
- use `br` to mutate bead state,
- use `bv` only in non-interactive/robot-style ways from agent sessions,
- do not create parallel TODO systems or ad hoc blocker notes.

## 7. Landing The Plane

When landing a session that should persist remotely:
1. ensure follow-up work is in beads,
2. run appropriate quality gates for changed code,
3. commit code and `.beads/`,
4. push,
5. verify the branch is up to date with `origin/main`.

Use `OPERATIONS.md` and `docs/BEADS.md` for the detailed execution model.

## 8. Multi-Agent Coordination

If Agent Mail is available, use it when parallel work would otherwise risk
conflicting edits.

Basic rule:
- reserve files before editing when multiple agents may touch the same area.

## 9. Current Source Of Truth Map

Use:
- `PRODUCT_DIRECTION.md` for product and UX authority
- `ARCHITECTURE.md` for implementation seams and architectural direction
- `OPERATIONS.md` for execution doctrine
- `docs/WORKSET_REGISTER.md` for ordered workset truth
- `.beads/` for live execution state

Do not treat:
- workset docs as live blocker trackers,
- old prototype code as architectural authority,
- older retired docs as current direction
