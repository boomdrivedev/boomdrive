---
order: 2
title: "One agent, one worktree"
summary: "Each agent works in its own worktree with its own ports, emulator and dev servers, and submits its work to a merge queue rather than merging it, so nothing it does can disturb another."
status: draft
introducedIn: "0.1"
lastReviewed: 2026-10-08
---

## How we do it today

Worktrunk, hashed ports, an emulator per worktree, Portless names, servers tethered to the worktree. Each session merges its own work: Worktrunk rebases, runs the gate and fast-forwards if nobody moved the line first.

## Where it's going

Worktree environments declared once and stood up from the declaration.

A session submits its head to a line and does nothing more: it never merges, never lands and never moves a line. The merge queue performs every merge and every landing ([sluice statement](https://github.com/agentplot/blueprints/blob/main/design/sluice/requirements.md), clauses 1 and 2).
