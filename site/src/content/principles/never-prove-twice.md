---
order: 6
title: "Never prove twice"
summary: "The task runner's cache, keyed by what each task reads and shared across every worktree, agent and CI run, means a check that passed on these inputs is not run again."
line: "A check that passed on these inputs is not run again."
status: draft
introducedIn: "0.1"
lastReviewed: 2026-10-08
---

## How we do it today

moon's cache, with archives in the git common directory so worktrees of one repository share them. Inputs are declared per project, and a check catches a test that reads outside them.

## Where it's going

A remote cache every machine reads and only the confined executor writes. It alone holds the cache's write credential, and that credential is the boundary, not a setting a runner inspects. A pass proven in a worktree replays when the merge queue tests the candidate it lands in. Testing the combination reruns only what the combination changed.

The cache remembers only passes. So the queue keeps a small record of every run beside it, a fail as faithfully as a pass ([sluice statement](https://github.com/agentplot/blueprints/blob/main/design/sluice/requirements.md), clauses 15, 16 and 19).
