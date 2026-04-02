# W020 Shell And Action System Rebuild

Status: `planned`
Date: 2026-04-02

## 1. Purpose
This workset rebuilds the shell around the current product direction instead of
the prototype shell.

Its center is:
1. unified action registry,
2. explicit command-entry model,
3. focus routing,
4. panel composition,
5. shell state for edit/run/debug.

## 2. Key Constraint
The rebuilt shell should not inherit:
1. raw `:` as the primary command gesture,
2. a one-buffer worldview,
3. one bottom output area as the only tool-surface model.

## 3. Closure Condition
This workset closes when the rebuilt shell has:
1. an explicit action namespace,
2. action invocation layers that can support profiles, chords, mnemonics, palette, and aliases,
3. a panel-oriented shell frame,
4. a viable command-entry path consistent with product direction.

## 4. Initial Epic Lanes
1. action registry
2. input routing and focus
3. shell frame and panel surfaces
4. command-entry and palette surfaces
