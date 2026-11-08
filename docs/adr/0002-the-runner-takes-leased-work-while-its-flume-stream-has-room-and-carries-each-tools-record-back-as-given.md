---
number: 2
title: The runner takes leased work while its Flume stream has room, and carries each tool's record back as given
status: accepted
date: 2026-11-01
decision-makers:
- Chuck Swanberg
---

# The runner takes leased work while its Flume stream has room, and carries each tool's record back as given

Status: accepted by the user on 2026-11-01, in the brief of the construct stage of the change `the-runner-carries-crews-work-into-flume`. The brief settled four points, quoted under *Decision outcome*, and folded in the runner's items of the cloud's domain-model ledger: 3.b, c-flume.a, c-sluice.a, c-inbox.a and N1.b. The user had accepted that ledger whole ("yes to all").

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
