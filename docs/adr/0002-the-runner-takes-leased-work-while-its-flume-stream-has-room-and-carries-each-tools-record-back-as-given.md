---
number: 2
title: The runner takes leased work while its Flume stream has room, and carries each tool's record back as given
status: accepted
date: 2026-11-01
decision-makers:
- Chuck Swanberg
---

# The runner takes leased work while its Flume stream has room, and carries each tool's record back as given

Status: accepted by the user on 2026-11-01, in the brief of the construct stage of the change `the-runner-carries-crews-work-into-flume`. Amended on 2026-11-07, below, twice: a pull's launches go ahead of its acts on places, and a launch waits on no act of the pass before its own. The brief settled four points, quoted under *Decision outcome*, and folded in the runner's items of the cloud's domain-model ledger: 3.b, c-flume.a, c-sluice.a, c-inbox.a and N1.b. The user had accepted that ledger whole ("yes to all").

Deciders: the user, 2026-11-01.
Sources: agentplot/boomdrive-cloud's records 0002, 0003, 0005, 0007, 0009, 0016, 0017 and 0022, its rules core.4, core.13, core.18, core.19, core.28 and core.36 to core.39, and its `design/domain-model.html` with its ledger items 3.b, c-flume.a, c-sluice.a, c-inbox.a and N1.b. Also record 0001, flume's record 0020 and flywheel-next's `definitions/engine/lease.yaml` and `host.yaml`.

## Context and problem statement

The cloud's records 0003, 0005, 0009, 0017 and 0022 and its rules core.4, core.18 and core.28 now set what a runner is to its control plane: how it connects, the leases its work comes under, how a second delivery is read, where streams are defined, how a bolt's branch is made, and that what goes up is references and results.

This repository's rules predate them. core.1 knows only `flume launch`. core.5 says the configuration names what a host keeps, but not that it names no stream, or that keeping a repository registers it to nothing. No rule says when the runner takes work, under what, or what it may carry back. What does the runner hold itself to?

## Considered options

1. Leave the runner's side to each change, the cloud's records binding only the cloud.
2. Hold the runner to the cloud's contract from its own side, by rules here: what its configuration names, when it takes work, what it carries out once, and what it carries back.

## Decision outcome

Option 2, the user's, held by core.1 and core.5 as amended in this record's commit, and by core.9 to core.11.

**What the configuration names** (settled point 1; ledger item 3.b): "the runner's configuration names the repositories it keeps and their folders (it clones the bare repositories and refreshes their views) and names NO stream; streams are defined in each host's Flume configuration with the repository mapped there; a launch may name another with --stream". A repository the runner keeps is reported as the host's capacity. Keeping it registers nothing to a drive (cloud record 0007). core.5 holds it.

**Work under a lease, while its stream has room** (settled point 2; cloud records 0003 and 0009): "the runner takes leased work only while its host's stream has room". The runner takes a launch only under the lease the control plane offered, while it keeps the launch's repository, and while the stream the launch runs in has room as Flume's status gives it. It names a stream only where the launch names one, and never chooses one. core.9 holds it.

**Commands** (settled point 3; cloud record 0017): "a bolt's branch is made when the user approves its rail card, by a Command a runner takes (sluice branch create), any host keeping the repository, waiting when none is online, an existing branch counting as done". A Command is a tool task under a lease of its own, not a Flume place. The runner carries it out through the tool's command line, and reports the commit the branch stands at as a fact read from git. The control plane counts the branch made from that, whichever way sluice answered. core.1 holds it.

**Leases, Commands and the Host object** (settled point 4): every report about leased work names the lease's number. Each act and command is carried out once, across restarts. Once the control plane refuses a report for a lease the runner no longer holds, the runner neither acts on that work nor reports it again, and removes nothing of it. The runner reports what the Host object copies: its tools' versions, the repositories it keeps, each Flume stream's room and the leases it holds. core.10 holds it.

**What goes back** (ledger items c-flume.a, c-sluice.a, c-inbox.a and N1.b):

- each place's status, as Flume gave it, under the launch's lease number;
- each submission's state and verdict rows, as sluice gave them, never its logs or heads;
- an inbox's records as references, never raw material;
- a terminal relayed between a browser and its pane over the runner's one outgoing connection, with nothing kept.

core.11 holds it.

### Consequences

- The runner's control-plane interface carries offers, leases, acts on a launch, commands, the inboxes to carry and terminals. So a hosted control plane's implementation can carry the cloud's contract whole (core.2).
- crew's implementation gives every launch the lease number 1 and no expiry: crew addresses each request to one host and never moves it.
- A launch that names no stream runs where Flume puts it, and Flume's status does not say where before the launch. Until Flume says so, the runner judges such a launch by its host's room.
- This record departs from flywheel-next's `definitions/engine/lease.yaml` and `host.yaml` and its requirements 55, 128 and 149: here the control plane offers each lease, and the runner removes nothing.

## Amendment, 2026-11-07: a pull's launches go ahead of its acts on places

Status: the code stage's own choice, on the brief of the chore `a-launch-goes-ahead-of-other-acts`, 2026-11-07. It moves when within a pass an act on a place runs, and changes no answer the runner gives, so it is cheap to change and is not asked.

The brief: "Boomdrive's runner takes a pull's launches ahead of its other acts, and an act Flume refuses because its place no longer stands costs no more than its answer, as records 0002 and 0010 say, so a launch never waits behind acts on places that are over."

### Context

A pass answered each want in the control plane's order. crew's implementation gives its wants in `Request` order, so acts asked before a launch come ahead of it. On 2026-11-07 a host's pull gave some twenty priority acts on launches whose places Flume no longer stood, ahead of new offers. Flume refused each, "no place <location> stands on this host", and each refusal took Flume about half a minute on that host. The launches waited some ten minutes behind them.

### Considered options

1. Keep the control plane's order, and leave the order to each control plane.
2. Answer an act on a place the pass's listing does not hold from that listing, running no Flume.
3. Take every act after every other want.
4. Two rounds over the pull, each in the control plane's order: the first answers every want but an act that runs Flume's command on a place, which it leaves for the second.

### Decision outcome

Option 4.

- **The first round** takes each offer, inbox and command, and answers each act that runs no command: one on a launch the runner holds unlaunched, on no launch of the runner's, or on a launch Flume named no place for.
- **The second round** carries out each act left, as Flume's command on its place, and each act on a launch the first round took. So no launch waits behind an act on a place.
- **No answer changes.** Room and holds are read from the pass's one listing, and an act on a place moves neither within the pass. So the launches made are the ones the control plane's order makes, and each act on a place gets its answer from Flume as before, later in the pass. An act ahead of its own launch is now carried out on the place that launch makes in the same pass, where before it waited for the next.

Not taken:

- **Option 1.** Each control plane would order its wants for a cost the runner pays, and crew's order is its requests'.
- **Option 2.** An act's answer is Flume's (core.1, core.11). The runner would judge, from Flume's status, what Flume decides of its own command.
- **Option 3.** A drop of a held launch is answered from the report. Where the pull gives it ahead of the launch, and the launch's After lands that pass, the launch would be made and then dropped, where the control plane's order never makes it. Which work runs is the control plane's (core.1).

core.50 holds it, and core.1's details allow it. The work-into-flume spec gains it through the chore `a-launch-goes-ahead-of-other-acts`.

## Amendment, 2026-11-07: a launch waits on no act of the pass before its own

Status: the code stage's own choice, on the brief of the chore `a-launch-waits-on-no-other-act`, 2026-11-07. It moves when an act on a place runs, across passes, and changes no answer the runner gives, so it is cheap to change and is not asked.

The brief: "Boomdrive's runner answers a pull's acts in seconds: no partition's acts hold another partition's launches, within a partition launches go ahead of other acts, and an act Flume refuses because its place no longer stands costs no more than reading the refusal, as records 0002 and 0010 say."

### Context

The amendment above keeps a pull's launches ahead of its own acts on places. A launch the next pull offers still waits for every act of the pass under way. On a host on 2026-11-07, one pass carried out some 180 acts of one partition's on places, most of them refused, at about 30 seconds each. Another partition's launches 63 and 64, asked at 13:48:54Z, waited through that pass and most of the next, while that partition's stream ran none of its four agents. Flume settles every place on the host before it acts, where no caller keeps watch (Flume's clause 119), so even an act Flume carries out takes it that long there.

Record 0026, on main, gives each partition's work time of its own, and names this chore its first step within a partition.

### Considered options

1. Carry out every act of the pull in its pass, as the amendment above does.
2. Once the next control pass is due, by the control timer or by the control plane's wake, run no further act's command in the pass but its first, and leave the rest to that pass, whose launches go first.
3. Carry out acts on places apart from the pass, on a thread of their own.
4. Run each partition's pass apart from the others' (record 0026).

### Decision outcome

Option 2.

- **Once the next pass is due**, the second round runs no act's command but its first, and leaves each act after it to the next pass, whose pull gives it again. Every pass runs at least one, so the acts get through.
- **The next pass's launches go first**, so a launch the next pull offers, in any partition, waits on no act of the pass before but the one in hand.
- **Due is the loop's own.** The next pass is due `poll_interval` after this one began, or once the control plane has woken the runner since it began. `boomdrive run --once` makes no next pass, so its pass carries out every act.
- **No answer changes.** Each act is carried out once, the acts in the control plane's order among themselves, each answered by Flume. A pass reports what it carried out at its end, so its answers reach the control plane sooner than a pass of every act would bring them.

Not taken:

- **Option 1.** One pass of many acts on places holds every partition's next pull, and the answers of all of them, until its end.
- **Option 3.** Two lines of work would share the report and the pass's room, and an act carried out once across restarts would need a record of its own of being under way (core.10, core.25).
- **Option 4.** That is record 0026's own change. While the partitions share one pass, option 2 keeps a backlog of acts from holding any partition's launches.

core.50 holds it. The work-into-flume spec gains it through the chore `a-launch-waits-on-no-other-act`.
