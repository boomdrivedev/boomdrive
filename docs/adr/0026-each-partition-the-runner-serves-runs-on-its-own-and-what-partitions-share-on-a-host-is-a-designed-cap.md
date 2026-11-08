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

Status: accepted by the user on 2026-11-07, on their card. Proposed the same day from their own words, carried from swancloud-planner by agentplot-operator. Amended on 2026-11-07, below: how each part of the control plane runs on its own, and what parts share.

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

## Amendment, 2026-11-07: how each part of the control plane runs on its own

Status: the construct stage's own choices, in the change `each-partition-the-runner-serves-runs-on-its-own`, 2026-11-07, within this record as accepted. Each is the runner's own and cheap to change, so none is asked.

The brief: "No partition the runner serves has its launches, acts or reports delayed by another partition's backlog, slow answer, refusal or failure; what partitions share on the host is bounded only by its owner's cap (Flume's agents and streams, sluice's runs) and a repository's one clone is fetched under a synchronization of its own."

### Context

The runner's control pass is one, over every partition its implementation serves, on the loop that also runs the keeping pass. crew's implementation serves a partition for each drive, a branch of its account's state repository. The hosted implementation serves a partition for each account enrolled, with its one connection (record 0028). The hosted protocol's version 1 names no drive on a want, and the runner reads no id.

### Decision outcome

- **A part is an implementation's partition.** The interface serves the control plane as parts, each a label and a control plane of its own, which says which ids are its. Under crew a part is a drive. Under the hosted control plane it is an account and its connection.
- **Each part runs on a thread of its own**, from the end of the first keeping pass, with its own timer, wake, reach and terminals, and its own `due` for core.50. The loop keeps the keeping passes and the landings, and keeps a landing between two repositories of a keeping pass, so a landing waits for no more than the repository in hand.
- **What parts share, and for how long.**
  - A clone's lock, held by the keeper for one repository's keep or let-go, and by a part for one launch into that clone.
  - The report file's lock, held for one save of a part's share. The file keeps its form and gains `parts`, the labels whose share it holds, and each part replaces only its own entries.
  - Flume's room. Each part counts its own launches against its own listing. Where two parts take one room at once, Flume's bound holds the later place waiting for room.
  None of these is held across a pull, a report or a tool's call, but a launch into its own clone.
- **crew's caches are per drive.** Each partition reads its state through a bare cache of its own, so no two drives share a git directory or weir's lock.
- **Under the hosted control plane, an account's drives share its part**, as they share its connection. Telling them apart needs a protocol version that names a want's drive, which is the hosted control plane's to design with boomdrive-cloud.

Not taken:

- **A room counted across parts by the runner.** This record rules out a line of the runner's own in front of a cap a tool owns.
- **One report file per part.** A host rolled back one release would read a whole file left as it stood at the upgrade, and carry out again what was answered since (core.10, core.36).
- **A fixed pool of threads.** A part would wait behind another's backlog for a free thread, a cap across drives no record decided.
- **The runner splitting one pull's wants by id or by repository.** It reads no id, and a hosted account's repositories are the hosted implementation's to read (core.2).

core.49's details name what parts share. core.9's details allow two parts to each take one room read at once, and core.23's allow each part's reach said on its own.
