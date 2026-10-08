---
order: 5
title: "Order at landing, never at start"
summary: "Dependencies are constraints the merge queue honours when work lands, not gates that keep a session from starting; conflicts are found cheaply when work is combined, not planned around."
status: draft
introducedIn: "0.1"
lastReviewed: 2026-10-08
related: [intents-and-bolts, one-agent-one-worktree]
---

A unit starts at once. When it depends on another unit, it starts off that unit's branch and names it as work to land after. The queue never lands it first ([sluice statement](https://github.com/agentplot/blueprints/blob/main/design/sluice/requirements.md), clauses 3 and 6), and waiting on a dependency holds back nobody else (clause 7).

The queue forms each candidate exactly as it would land: the line, plus every entry ahead of it, in order. A conflict is found when the candidate is formed, before any check runs. It removes only that entry, and the rest of the chain continues (clauses 10 to 12).

A failed candidate has exactly one culprit: the newest entry of the first candidate that failed. It is named only once everything ahead of it has a verdict, and only its session hears of it (clauses 17, 18 and 20).

Work that touches separate parts of the repository is never tested together and lands in either order (clause 8).

## How we do it today

Crew orders units in its plan and starts a unit only after the units it depends on have merged into the bolt.

## Where it's going

sluice, a merge queue for agents in worktrees: one queue design with two instances. The bolt line's queue trusts the sessions' own runs and the local cache; the shared line's queue trusts only the confined executor.
