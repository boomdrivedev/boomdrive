---
number: 2
title: The runner takes leased work while its Flume stream has room, and carries each tool's record back as given
status: accepted
date: 2026-11-01
decision-makers:
- Chuck Swanberg
---

# The runner takes leased work while its Flume stream has room, and carries each tool's record back as given

Status: accepted by the user on 2026-11-01, in the brief of the construct stage of the change `the-runner-carries-crews-work-into-flume`. Amended on 2026-11-07, below: a pull's launches go ahead of its acts on places. The brief settled four points, quoted under *Decision outcome*, and folded in the runner's items of the cloud's domain-model ledger: 3.b, c-flume.a, c-sluice.a, c-inbox.a and N1.b. The user had accepted that ledger whole ("yes to all").

Deciders: the user, 2026-11-01.
Sources: agentplot/boomdrive-cloud at `24baa4e`: its records 0002, 0003, 0005, 0007, 0009, 0016, 0017 and 0022; its rules core.4, core.13, core.18, core.19, core.28 and core.36 to core.39; and `design/domain-model.html`, with its Launch, Command and Host cards, its copies of a Flume place, a sluice entry, the inbox and a host's capacity (section 8), "One launch, start to finish" (section 10) and its ledger (section 13). Also record 0001, flume's record 0020 and flywheel-next's `definitions/engine/lease.yaml` and `host.yaml`.

## Context and problem statement

The cloud's constitution now says what a runner is to its control plane:

- A host's runner holds one outgoing socket, which ends at the host's Host object (cloud record 0005).
- A launch is carried out under a numbered lease. A report under a replaced number is refused (cloud record 0009).
- Every offer, lease, command and event has an id, and a second delivery changes nothing (cloud record 0022).
- Streams are defined in each host's Flume configuration, and a runner takes work only while its host's stream has room (cloud record 0003).
- A bolt's branch is made by a Command that any host keeping its repository may take (cloud record 0017).
- What goes up is references and results, never content (cloud core.4, core.28). A terminal is relayed and never stored (cloud core.18).

This repository's rules predate it. core.1 knows only `flume launch`. core.5 says the configuration names what a host keeps, but not that it names no stream, or that keeping a repository registers it to nothing. No rule says when the runner takes work, under what, or what it may carry back. What does the runner hold itself to?

## Considered options

1. Leave the runner's side to each change, the cloud's records binding only the cloud.
2. Hold the runner to the cloud's contract from its own side, by rules here: what its configuration names, when it takes work, what it carries out once, and what it carries back.

## Decision outcome

Option 2, the user's, held by core.1 and core.5 as amended in this record's commit, and by core.9 to core.11.

**What the configuration names** (settled point 1; ledger item 3.b): "the runner's configuration names the repositories it keeps and their folders (it clones the bare repositories and refreshes their views) and names NO stream; streams are defined in each host's Flume configuration with the repository mapped there; a launch may name another with --stream". A repository the runner keeps is reported as the host's capacity. Keeping it registers nothing to a drive (cloud record 0007). core.5 holds it.

**Work under a lease, while its stream has room** (settled point 2; cloud records 0003 and 0009): "the runner takes leased work only while its host's stream has room". The runner takes a launch only under the lease the control plane offered, while it keeps the launch's repository, and while the stream the launch runs in has room as Flume's status gives it. It names a stream only where the launch names one, and never chooses one. core.9 holds it.

**Commands** (settled point 3; cloud record 0017): "a bolt's branch is made when the user approves its rail card, by a Command a runner takes (sluice branch create), any host keeping the repository, waiting when none is online, an existing branch counting as done". A Command is a tool task under a lease of its own, not a Flume place. The runner carries it out through the tool's command line, and reports the commit the branch stands at as a fact read from git. The control plane counts the branch made from that, whichever way sluice answered. core.1 holds it.

**Leases, Commands and the Host object, as the cloud's records say** (settled point 4): every report about leased work names the lease's number. Each act and command is carried out once, across restarts. Once the control plane refuses a report for a lease the runner no longer holds, the runner neither acts on that work nor reports it again, and removes nothing of it. The runner reports what the Host object copies: its tools' versions, the repositories it keeps, each Flume stream's room and the leases it holds. core.10 holds it.

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
- This record departs from flywheel-next's design. Its host took a lease itself, by compare-and-swap on the state store, within what its manifest declared. It also removed stray places from its disk (`definitions/engine/lease.yaml`, `host.yaml`; its requirements 55, 128 and 149). Here the control plane offers each lease, and the runner removes nothing.

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

core.49 holds it, and core.1's details allow it. The work-into-flume spec gains it through the chore `a-launch-goes-ahead-of-other-acts`.
