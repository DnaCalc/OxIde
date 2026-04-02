# W050 File Workspace And Project Management Surfaces

Status: `planned`
Date: 2026-04-02

## 1. Purpose
This workset rebuilds file handling, workspace handling, and project-oriented UX
on top of the new shell and session model.

It covers:
1. open/save/reload/revert,
2. workspace load/reload,
3. project-backed module navigation,
4. project management surfaces and commands.

## 2. Ownership Rule
OxIde owns the UX and orchestration.
OxVba owns project truth and helper semantics.

## 3. Closure Condition
This workset closes when:
1. file lifecycle behavior is rebuilt cleanly,
2. workspace and project lifecycle behavior is rebuilt cleanly,
3. project-backed navigation works through explicit surfaces,
4. project-management UX exists without inventing OxIde-local project truth.

## 4. Initial Epic Lanes
1. file lifecycle
2. workspace lifecycle
3. project rail and project-management surfaces
4. project command set
