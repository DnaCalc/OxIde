# W040 - Project And Workspace Management

Status: `planned`
Sequence: `4`
Depends on: `W030`

## 1. Purpose
Make the workspace itself first-class in the shell by presenting real project
structure, project actions, and workspace/session state over OxVba-owned truth.

## 2. Governing Truth
1. [PRODUCT_DIRECTION.md](/C:/Work/DnaCalc/OxIde/PRODUCT_DIRECTION.md)
2. [ARCHITECTURE.md](/C:/Work/DnaCalc/OxIde/ARCHITECTURE.md)
3. [DESIGN_TUI.md](/C:/Work/DnaCalc/OxIde/docs/DESIGN_TUI.md)

## 3. Intended Execution Lanes
1. project explorer over real OxVba project structure
2. project-management actions through OxVba helpers
3. workspace switching, recent workspaces, and session-restore policy
4. dedicated project-management layouts and inspector/action surfaces

## 4. Rollout Intention
This workset should expand directly into execution beads for explorer,
project-action, workspace-state, and layout outcomes.

## 5. Closure Condition
This workset closes when OxIde can open a workspace, inspect real project
structure, run project-management actions, and restore meaningful workspace
shell state without inventing local project semantics.
