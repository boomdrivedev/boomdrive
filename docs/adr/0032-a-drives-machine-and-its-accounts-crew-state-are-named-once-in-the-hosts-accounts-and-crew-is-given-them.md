---
number: 32
title: A drive's machine and its account's crew state are named once, in the host's accounts, and crew is given them
status: accepted
date: 2026-11-08
decision-makers:
- the design agent
---

# A drive's machine and its account's crew state are named once, in the host's accounts, and crew is given them

Status: accepted on 2026-11-08 by the design agent, its own choice, made on the records and rules written (`briefs.15`).

Deciders: the design agent, on the planner's question from batch 18 of 2026-11-08. The rest is the user's already: record 0028, accepted by the user on 2026-11-07, where "Under crew, an account names its crew state repository" and a drive names its `machine`; and the user's words there, "Yes the branches in crew correlate to the drives".

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
- **No code for the transition.** No code carries a host between the flake bolt's shape and this one. The user, 2026-11-08, in the design agent's pane: "I want the system to be built the way it should be and then have agents fucking fix the data transition as necessary".

## Amendment, 2026-11-15: the module's forms

Status: decided on 2026-11-15 by the construct stage of the change `a-drives-machine-and-state-are-named-once`, on its brief: "A host names each drive's machine and its account's crew state repository once, in its Boomdrive accounts, and the set passes each drive's machine and its account's state to crew's module, so services.crew.settings names neither, and the runner's crew table takes them from there too." These are choices made on the rules written and on crew's module at the lock's tag, each cheap to change. The change's design gives the path not taken for each.

Gives this record's outcome its forms.

- **The key is `state_repository`.** An account names its crew state repository as `services.boomdrive.accounts.<account>.state_repository`, an address `<owner>/<name>`, unset by default, and no key of the runner's file (core.5). The runner's crew table names the value `state_repository`, and `boomdrive setup` takes it as `--state-repository`. Not taken: `state`, as option 1 above and record 0028's example write it. core.12's details rule out "a key such as `state` that names less than what it holds, a repository", and core.46 names an option as the file names its key.
- **With crew's module on, whether or not the runner is.** crew is given each drive of each account that names its `state_repository`, this host's and others': its `machine`, the drive's or `services.crew.host` where it names none, and its `state`, the account's `state_repository`. Nothing else of a drive is given, and nothing of an account that names no `state_repository`, which is no account of crew's. Not taken: only with the runner, as Flume's streams are given, which would have a host write both in crew's settings again while the runner is off; a drive's machine given where its account names no state repository, which would split one crew drive between two writers.
- **At the options' own priority, placed at `services.boomdrive.accounts`.** crew's own check, which takes a drive's `machine` and `state` "written once", fails a host's second write. The giving module's `_file` is `services.boomdrive.accounts`, so crew's failure lists that option among the places that write it. A host's `lib.mkForce` wins, on purpose (core.57). Not taken: `lib.mkDefault`, under which a host's own value would win with no word; an assertion of the set's over crew's settings, which judges a fact crew already judges.
- **The runner's crew table in the accounts' order.** It holds each drive of this host under an account that names its `state_repository`, accounts and each account's drives in the order of their names, as the runner's file lists them, by the drive's name as `label` with its account's `state_repository`. Not taken: label order across the accounts, a sort that changes nothing the runner's crew implementation reads.
- **Under crew, in the accounts or nowhere.** With the runner, a crew control plane and crew's module on, an account that names a drive and no `state_repository` fails evaluation, naming its option. A drive of crew's settings that no account names fails too, naming it and `services.boomdrive.accounts`. Not taken: either left to stand, which serves a drive with no stream or no partition, the mismatch this record ends; either failure under another control plane, whose accounts are not crew's (core.19).

core.57, and core.18's and core.46's details, are amended in this amendment's commit.

## Amendment, 2026-11-18: crew runs only the drives the host sets up in crew

Status: decided on 2026-11-18 by the design agent, on the planner's urgent question from afterthought-design, its own choice, cheap to change. The user's own ruling stands under it: Lime Energy's repositories are kept on deneb as the drive `wes`, with no main level and no agents (afterthought/swancloud record 0059 as amended 2026-11-13: "let's call it wes not lime").

The 2026-11-15 amendment gives crew every drive of an account that names a `state_repository`, so every such drive becomes a crew drive, and under crew every account that names a drive must name one. A drive the host never set up in crew, `wes` among them, then becomes a crew drive with no main level, needing crew's own keys and a state repository.

- **crew runs a drive only where the host sets it up in crew.** A drive is crew's when the host's crew settings set up its main level: its own keys under `services.crew.settings.drives.<drive>`, its session, roles and blueprints. Boomdrive's module gives such a drive its `machine` and its account's `state_repository`, as the 2026-11-15 amendment says. A drive the host does not set up in crew is given nothing: crew has no drive of that name, and the drive is a drive like any other, its repositories kept on its machine and a Flume stream given where it runs, with no main level.
- **An account needs a state repository only for a drive crew runs.** Under crew, an account with a drive the host sets up in crew and no `state_repository` fails evaluation, naming its option. An account with no such drive needs none.
- **Not taken:** *a switch on the drive that turns off its stream, its crew drive and its account's state repository.* The user turned it down on the proposal's card: "this is a smell. i don't like it". A switch that turns off everything a drive is says the drive is not one. *Repositories a host keeps outside any drive.* The user named `wes` a drive. *An account that names no state repository is no account of crew's.* That drops the check that catches a forgotten state repository, and one account may hold a drive crew runs and one it does not.

## Amendment, 2026-11-18: the module's forms

Status: decided on 2026-11-18 by the construct stage of the change `crew-runs-only-the-drives-set-up-for-it`, on its brief: "crew runs only the drives a host sets up for it, and an account needs a crew state repository only for one of those, so a drive kept just to read, like Lime Energy's on deneb, needs nothing more." These choices are made on the rules written and on crew's module at the lock's tag, `v0.68.0`, and each is cheap to change. The change's design gives the path not taken for each.

Gives the amendment above its forms.

- **The set adds to crew's drive type, and defines no drive.** Boomdrive's module declares `services.crew.settings` again, and the module system merges that declaration into crew's own. The declaration's `drives` is an `attrsOf` of a submodule that gives each of crew's drives its `machine` and `state` where the accounts give them. So only a drive the host's crew settings name is ever given anything, and the set adds no name to crew's drives. Each given value is placed at `services.boomdrive.accounts`. A host's second write still fails crew's own check, naming that place. Not taken: reading crew's drives' names to choose what to give, which recurses, since giving a drive's value defines the drive. Not taken either: reading the host's own definitions of crew's settings for the drives they name, which would redo the module system's merge of `lib.mkIf`, `lib.mkMerge` and module functions. Nor an option of crew's own that applies the accounts' values, which would leave this tree declaring no option of crew's, but is a crew release for what crew's own type already allows. That declaration is one of crew's options in this tree, so the service-modules requirement "The modules bring each tool's own module" is amended to admit it, on the design agent's answer of 2026-11-18 to the change's code stage: "Take the first way: keep Decision 1 and amend the requirement". Every option stays declared in crew's input, the platform's crew module, re-exported, stays crew's own with nothing added, and this tree declares one of crew's options again only to add, to its submodule type, a module giving facts the set knows, declaring no option crew's input does not and setting no description, default or example.
- **The runner's crew table holds the drives crew runs.** Each partition is a drive of this host that crew's settings set up, under an account that names its `state_repository`, in the accounts' order. Not taken: every drive of this host under such an account, which names partitions crew has no drive for.
- **Under crew, the state repository's failure reads the drives crew runs.** With the runner, a crew control plane and crew's module on, an account with no `state_repository` fails only where it holds a drive crew's settings set up, naming its option. The "no drive" failure names the machines of the drives crew's settings set up.

core.57, and core.18's and core.46's details, are amended with this amendment, and the service-modules requirement "The modules bring each tool's own module" with the change's delta.
