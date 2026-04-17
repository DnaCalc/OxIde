# W090 - Command System And Keymap Profiles

Status: `planned`
Sequence: `9`
Depends on: `W035`

## 1. Purpose
Replace the ad-hoc F-key and Ctrl/Alt routing in `model.rs` with a unified
action registry that backs the command palette, shortcut bindings, chords,
mnemonic menus, and mouse actions through a single namespace, and that
supports swappable keymap profiles including a VBA-IDE-compatible profile.

## 2. Governing Truth
1. [PRODUCT_DIRECTION.md](/C:/Work/DnaCalc/OxIde/PRODUCT_DIRECTION.md)
2. [ARCHITECTURE.md](/C:/Work/DnaCalc/OxIde/ARCHITECTURE.md)
3. [docs/uxpass/40_command_model.md](/C:/Work/DnaCalc/OxIde/docs/uxpass/40_command_model.md)

## 3. Intended Execution Lanes
1. `ActionRegistry` with metadata (id, label, context, default binding, VBA-IDE binding)
2. keymap profile loader: default, VBA-IDE-compatible, and user overrides under `%APPDATA%/OxIde/keymap.json`
3. chord handling (Ctrl+K Ctrl+O style) as an explicit state machine in `model.rs::update`
4. mnemonic menu sequences (Alt+I,M etc.) as a first-class subsystem
5. `model.rs` refactor so input resolution delegates to the registry

## 4. Rollout Intention
This workset depends on W035 having ruled on the command model. Existing
hardcoded keys migrate into the registry as a single bead; profile loader
and chord/mnemonic subsystems land as separate beads.

## 5. Closure Condition
This workset closes when a WTD scenario can switch keymap profiles at
runtime and confirm that a VBA-IDE-native key combination triggers the
expected action, and when every previously hardcoded F-key and
Ctrl/Alt binding resolves through the registry.
