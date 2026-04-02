# W040 Editor Surface And Undo Foundation

Status: `planned`
Date: 2026-04-02

## 1. Purpose
This workset establishes the rebuilt editor surface on top of FrankenTui and
gets undo/redo ownership right from the start.

It covers:
1. editor integration behind the OxIde-owned seam,
2. cursor and viewport behavior,
3. per-buffer undo/redo,
4. shared edit history across multiple views onto the same buffer.

## 2. Key Rule
Undo history belongs to buffers, not views.

## 3. Closure Condition
This workset closes when:
1. the editor surface is aligned with the rebuilt shell/session model,
2. per-buffer undo/redo is explicit,
3. multiple views onto the same buffer share one edit history,
4. layout operations are cleanly separated from text undo/redo.

## 4. Initial Epic Lanes
1. editor adapter
2. undo/redo model
3. cursor and viewport behavior
4. multi-view editor coordination
