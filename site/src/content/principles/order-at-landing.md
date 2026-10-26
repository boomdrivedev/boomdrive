---
order: 6
title: "Order at landing, never at start"
summary: "Dependencies are constraints the merge queue honours when work lands, not gates that keep a session from starting; conflicts are found cheaply when work is combined, not planned around."
line: "Work starts at once; the merge queue orders it as it lands."
tools:
  - name: sluice
    href: https://agentplot.github.io/sluice/#why
status: draft
introducedIn: "0.1"
lastReviewed: 2026-10-08
related: [intents-and-bolts, one-agent-one-worktree]
diagram:
  src: /diagrams/order-at-landing.svg
  alt: "Three agents, each in its own worktree, submit their work and never merge. sluice forms candidates line plus A, line plus A plus B, and line plus A plus B plus C, and tests them at once. Line plus A plus B fails, so B is the one culprit and only agent B is told. A lands as a merge commit; C is re-formed on the line without B, mostly replays from the cache, passes and lands. The line moves only to what passed."
---

A unit starts at once. When it depends on another unit, it starts off that unit's branch and names it as work to land after. The queue never lands it first, and waiting on a dependency holds back nobody else.

The queue forms each candidate exactly as it would land: the line, plus every entry ahead of it, in order. A conflict is found when the candidate is formed, before any check runs. It removes only that entry, and the rest of the chain continues.

A failed candidate has exactly one culprit: the newest entry of the first candidate that failed. It is named only once everything ahead of it has a verdict, and only its session hears of it.

Work that touches separate parts of the repository is never tested together and lands in either order.

## How we do it today

Crew orders units in its plan and starts a unit only after the units it depends on have merged into the bolt.

## Where it's going

sluice, a merge queue for agents in worktrees: one queue design with two instances. The bolt line's queue trusts the sessions' own runs and the local cache; the shared line's queue trusts only the confined executor.
