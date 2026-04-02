# W030 Buffer View Layout Session Model

Status: `planned`
Date: 2026-04-02

## 1. Purpose
This workset establishes the buffer/view/layout model required by current OxIde
direction.

It exists because the rebuilt IDE needs:
1. open-but-not-visible buffers,
2. multiple visible views,
3. multiple views onto the same buffer,
4. layout composition without tab-first assumptions.

## 2. Closure Condition
This workset closes when:
1. buffers, views, and layouts are explicit first-class concepts,
2. buffer identity is no longer collapsed into one active editor instance,
3. the session model supports non-visible open buffers,
4. the session model supports multiple visible views on one buffer.

## 3. Initial Epic Lanes
1. buffer model
2. view model
3. layout model
4. session restore hooks
