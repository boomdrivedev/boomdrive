---
order: 7
title: "Never prove twice"
summary: "The task runner's cache, keyed by what each task reads and shared across every worktree, agent and CI run, means a check that passed on these inputs is not run again."
line: "A check that passed on these inputs is not run again."
status: draft
introducedIn: "0.1"
lastReviewed: 2026-10-08
diagram:
  src: /diagrams/never-prove-twice.svg
  alt: "tailrace's cache sits in the middle, and four places run the same three checks around it: sluice's merge queue, a worktree, a second worktree and CI. The queue tests a candidate first, and its checks take their time and pass. sluice records each run, and when the candidate lands, its verdicts fill the cache with one entry for each check. The worktree, the second worktree and CI then replay those checks from the cache in a flash, running nothing. A next candidate changes the Vale configuration, so only the prose check runs again while the other two replay. That run fails. sluice's record keeps the fail; the cache fills only from landings, so it never remembers it. moon declares what each check reads."
---

## How we do it today

moon's cache, with archives in the git common directory so worktrees of one repository share them. Inputs are declared per project, and a check catches a test that reads outside them.

## Where it's going

A remote cache every machine reads and only the confined executor writes. It alone holds the cache's write credential, and that credential is the boundary, not a setting a runner inspects. A pass proven in a worktree replays when the merge queue tests the candidate it lands in. Testing the combination reruns only what the combination changed.

The cache remembers only passes. So the queue keeps a small record of every run beside it, a fail as faithfully as a pass ([sluice statement](https://github.com/agentplot/blueprints/blob/main/design/sluice/requirements.md), clauses 15, 16 and 19).
