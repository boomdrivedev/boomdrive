---
number: 26
title: Each drive the runner serves runs on its own, and what drives share on a host is a designed cap
status: accepted
date: 2026-11-07
decision-makers:
- Chuck Swanberg
- agentplot-design
---

# Each drive the runner serves runs on its own, and what drives share on a host is a designed cap

Status: accepted by the user on 2026-11-07, on their card. Proposed the same day from their own words, carried from swancloud-planner by agentplot-operator.

Amended by record 0028, accepted on 2026-11-07: it reads "drive" where it said "partition" for a body of work, and "account" where it meant what a runner enrolls, and is so written below. The user's words keep their own. The file keeps its name.

Deciders: the user, 2026-11-07, after one drive's backlog on deneb held another's launches: "We might want to improve Boomdrive's design so that each account partition operates independently unless there's some sort of host-level cap and synchronization that needs to be designed." The design's words are agentplot-design's.

Sources: records 0002, 0008 and 0010; rules core.19 and core.26; signals 2026-11-07-agentplot-design-f293029e/01 on afterthought/crew-state agentplot/main and 2026-11-07-swancloud-planner-55d3681f/01 on its swancloud branch; the runner's log on deneb, 2026-11-07 (requests 181, 182 and 184 at 14:05:40, 14:06:13 and 14:06:44Z, and swancloud's launches 63 and 64 of 13:48:54Z claimed only at about 14:03Z while swancloud's stream ran none of its four).

## Context and problem statement

A runner serves several drives of one control plane, of one account or of several, each account an enrollment of its own (core.19), and keeps each account's data apart (core.26). It does not keep the drives' time apart. On deneb the runner took every drive's requests in one line, about 30 seconds each. One drive's backlog of acts on places that no longer stood held another drive's launches for a quarter of an hour, while that drive's stream had room for four agents and ran none.

Some things on a host are shared by every drive and are no drive's own: the host's bound on agents running (Flume's I6 and 70), the host's bound on check runs (sluice's), and the one clone of each repository (record 0003). Each of these already has an owner who bounds it.

## Considered options

1. **Each drive runs on its own.** A drive's work is served apart from every other's. What drives share on a host is a cap or a synchronization its owner designs.
2. **One line, launches first.** The runner keeps one line for every drive but takes launches ahead of other acts. That is the chore `a-launch-waits-on-no-other-act`, which cures this morning's case but leaves a slow or failing drive able to slow the rest.
3. **One runner per drive.** Each drive gets a runner process of its own. That gives independence, at the cost of a process per drive, and two runners fetching the one clone (record 0003) would need a lock designed anyway.

## Decision outcome

Option 1, the user's direction.

- **A drive's work is served apart.** No drive's backlog, slow answer, refusal or failure delays another drive's launches, acts or reports, whether the two are of one account or of two. Each drive's work, from its pull to its answers, moves at its own pace, and each account's connection at its own (core.26 keeps each account's data apart; this keeps every drive's time apart).
- **Within a drive, launches come first.** A launch waits on no act of its own drive's other places (the chore `a-launch-waits-on-no-other-act`).
- **What is shared is a designed cap.** A resource every drive on the host draws on is bounded by the tool that owns it, and the runner adds no line of its own in front of it. The host's agents are Flume's bound and its streams' room; the host's check runs are sluice's; a repository's one clone is fetched by the runner under a synchronization of its own, never one drive waiting for another's whole pass.
- **Any new shared cap is designed, not found.** A cap or synchronization across drives that no tool owns yet is decided in a record before it is built.
- **Still one runner.** The runner stays one process on the host (record 0008), serving its accounts and their drives side by side.

### Consequences

- Good: one drive's trouble stays its own. A burst of acts in one drive costs that drive alone.
- Good: the chore `a-launch-waits-on-no-other-act` goes ahead as it stands, as the first step within a drive.
- Bad: the runner's pass becomes several, one per drive, which its log and status show each on its own.
- A rule beside core.26 says it: core.49.
- Record 0028 names the two words: an account is what the runner enrolls, and a drive is a body of work of one account. The independence here holds between any two drives, so between accounts too.
