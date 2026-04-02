# W010 Implementation Reset And Salvage Triage

Status: `planned`
Date: 2026-04-02

## 1. Purpose
This workset turns the current code review into an actionable salvage/rewrite
packet.

Its job is to decide, explicitly:
1. what existing code is worth porting,
2. what existing tests are worth migrating,
3. what prototype structure should be discarded,
4. where the rewrite boundary sits.

## 2. Current Position
The current repo contains a working spike with real direct OxVba host-session
integration value, but its shell structure does not match current product
direction.

The working recommendation is:
1. rebuild the shell/application structure,
2. salvage the direct host-session integration patterns,
3. salvage selected tests,
4. discard prototype assumptions such as raw `:` command entry and the one-buffer
   application model.

## 3. Closure Condition
This workset closes when:
1. current code is classified into retain / port / discard buckets,
2. the direct host-session salvage packet is explicit,
3. the test salvage packet is explicit,
4. the replacement boundary for the current shell is explicit.

## 4. Initial Epic Lanes
1. code inventory
2. test inventory
3. semantic salvage packet
4. prototype removal and replacement plan
