---
number: 32
title: A drive's machine and its account's crew state are named once, in the host's accounts, and crew is given them
status: accepted
date: 2026-11-08
decision-makers:
- agentplot-design
---

# A drive's machine and its account's crew state are named once, in the host's accounts, and crew is given them

Status: accepted on 2026-11-08 by agentplot-design, the design agent's own choice, made on the records and rules written (`briefs.15`).

Deciders: agentplot-design, on the planner's question from batch 18 of 2026-11-08. The rest is the user's already: record 0028, accepted by the user on 2026-11-07, where "Under crew, an account names its crew state repository" and a drive names its `machine`; and the user's words there, "Yes the branches in crew correlate to the drives".

Sources: record 0028 and its amendment of 2026-11-08 (the module's options); record 0023; rules core.2, core.18 and core.46; agentplot/crew's records 0054 (crew's "partition" becomes "drive"), 0058 and 0061, and its rules `flake.1` and `flake.2`, under which crew's module writes crew's file whole from `services.crew.settings`, which Boomdrive's module may set; the changes `each-repository-is-named-once-by-its-owner` and `crews-partitions-give-the-runner-its-crew-table` on the bolt `the-boomdrive-flake-configures-the-whole-host`.

## Context and problem statement

On the flake's bolt, a drive's `machine` is written in `services.boomdrive.accounts`, which decides whether this host gives the drive a Flume stream. The runner's crew table is read from crew's partitions in `services.crew.settings`, each with its own `machine` and `state`. A crew partition is a drive (record 0028; crew record 0054). So one fact, which machine a drive runs on, is written twice, and nothing keeps the two the same: a drive this host gives a stream but whose crew partition names another machine gets agents nobody serves, and the reverse serves a drive with no stream. Record 0028 puts an account's crew state repository on the account, and neither change does.

## Considered options

1. **The host's accounts name them, and Boomdrive's module gives them to crew (chosen).** `services.boomdrive.accounts.<account>.state` names the account's crew state repository and each drive's `machine` its machine. Boomdrive's module gives crew each drive's name, machine and state through crew's own module options, and builds the runner's crew table from the same accounts. crew's settings keep only what is crew's own for a drive, named by the drive.
2. **crew's partitions name them, and Boomdrive reads them.** Under crew, a drive writes no `machine` and an account no `state`, and the module reads both from `services.crew.settings`. Not taken: it goes against record 0028's decision, which the user accepted, and the same drive would name its machine in Boomdrive's accounts under a hosted control plane and in crew's settings under crew.
3. **Both stay, checked equal.** Not taken: the host still writes the fact twice, which the flake's bolt exists to end, and a check that only carries the host from one shape to the next is code for a transition, which the user rules out (below).

## Decision outcome

Option 1.

- **One place.** A host names a drive's machine and its account's crew state repository once, in `services.boomdrive.accounts`. A drive naming no machine is this host's.
- **crew is given them.** Where crew's module is on, Boomdrive's module gives crew each drive of every account, this host's and others', with its machine and its account's state repository, through crew's module options (core.46; crew `flake.2`). crew's own keys for a drive, its session, roles, blueprints and review, stay in crew's settings, named by the drive. A host that also writes a drive's machine or state in crew's settings fails its switch, naming `services.boomdrive.accounts`.
- **The runner's crew table** is built from the same accounts: crew's host and each drive of this host under an account served by crew, with its account's state repository. A table the host writes out still wins whole (record 0023).
- **crew's names stay crew's.** Boomdrive's module sets crew's options as crew's file names them, so crew's rename of "partition" to "drive" (crew record 0054) moves in the change that moves crew's version.
- **What builds it.** A crew change lets another module give each drive its machine and its state repository. Then a Boomdrive change gives them from the accounts and adds `state` to an account. No code carries a host between the flake bolt's shape and this one: whoever writes a host's configuration moves its data by hand. The user, 2026-11-08, in agentplot-design's pane: "I want the system to be built the way it should be and then have agents fucking fix the data transition as necessary".

## Consequences

- One crew unit, then one Boomdrive unit, which retire it.
- afterthought/swancloud writes deneb's machines in both places, the same, until both changes land, then once.
