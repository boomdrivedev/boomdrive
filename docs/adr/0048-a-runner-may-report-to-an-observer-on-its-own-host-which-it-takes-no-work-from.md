---
number: 48
title: A runner may report to an observer on its own host, which it takes no work from
status: rejected
date: 2026-11-16
decision-makers:
- Chuck Swanberg
- boomdrive-design
---

# A runner may report to an observer on its own host, which it takes no work from

Status: withdrawn by the user on 2026-11-19, in the design agent's pane. An observer served only a local copy of Boomdrive Cloud, which no customer runs, so a runner keeps no connection but the one to the control plane it takes work from. The real connection is record 0008's: the operator's own partitions move to the hosted control plane, and crew's implementation then leaves the runner.

Deciders: the user, who chose a read-only mirror of the host's real runner for a local preview of Boomdrive Cloud over a full loop on a separate test runner, and so loosened core.19; boomdrive-design, for how the observer is reached and what it may ask.

Sources: records 0006, 0008, 0013, 0015, 0018 and 0026; rules core.1, core.9, core.10, core.19, core.42; boomdrivedev/boomdrive-cloud record 0069, the local preview this serves.

## Context and problem statement

Boomdrive Cloud is built against the runner's hosted protocol, and a person working on it wants to see the cloud's pages fill with a real host's work: the places Flume runs on deneb, their stages and their ends. The runner on deneb serves crew's control plane, and core.19 lets a runner open one control plane and no other. A local copy of the cloud can show real work only if something real reports to it, and nothing the local copy does may reach the work.

## Considered options

1. **The host's own runner reports to the local cloud as an observer, which it takes no work from** (chosen).
2. **A separate test runner, with its own Flume and scratch repositories, serving the local cloud as its one control plane.**
3. **The local cloud reads crew's state repository, where the runner's reports already land.**

## Decision outcome

1. **An observer, the user's choice.** Beside the one control plane it takes work from, a runner may report a partition's work to an observer: a hosted control plane that gets the runner's hello and its reports of the places it follows, and gives it nothing. The runner offers an observer no room, and takes from it no lease, launch, act, command, preview or terminal; any such message from an observer is refused and logged, and nothing else follows from it. So whatever a person does in the observer, nothing reaches the host's work. Not taken: a separate test runner (option 2), which shows only the scratch work it makes; reading crew's state repository (option 3), a second way into the cloud beside the protocol, which the hosted runner would never use.
2. **On its own host alone.** An observer is reached only at a loopback origin, so nothing an observer gets leaves the host, and what leaves a host stays what records 0013 and 0014 allow. Not taken: an observer anywhere, such as staging, which would send a host's work to a cloud no record has allowed it to reach.
3. **Enrolled as a partition is.** An observer is named in the runner's configuration, by its origin and the partition whose work it observes, and enrolled with the host's key as any hosted partition is (records 0006 and 0018). Its reports are the reports that partition's control plane gets, each place as the protocol's copy of Flume's status gives it (core.27), with no lease where the observer offered none.
4. **The protocol says it.** The hello says the connection observes, and the protocol's version that carries it lets a report name no lease on an observing connection alone (core.42). A control plane that takes an observing hello offers nothing on that connection.
5. **Nothing waits on it.** An observer that cannot be reached is tried again as any control plane is (core.22), and its reach holds up no pass, no launch and no report to the control plane the runner takes work from.

## Consequences

- Good: the cloud can be worked on against a real host's work with no risk to it, and an observer needs nothing on the host but the runner it already runs.
- Bad: a runner now holds more than one socket, and a pass reports twice where an observer is named.
- Neutral: a runner still takes work from one control plane alone; record 0008's point on one control plane per runner holds for work, and core.19 is amended in the commit that writes this record.
