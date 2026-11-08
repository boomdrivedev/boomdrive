---
number: 2
title: The runner takes leased work while its Flume stream has room, and carries each tool's record back as given
status: accepted
date: 2026-11-01
decision-makers:
- Chuck Swanberg
---

# The runner takes leased work while its Flume stream has room, and carries each tool's record back as given

Status: accepted by the user on 2026-11-01, in the brief of the construct stage of the change `the-runner-carries-crews-work-into-flume`. Amended on 2026-11-07, below, four times: a pull's launches go ahead of its acts on places; a launch waits on no act of the pass before its own; a bolt is a launch, the runner carrying no branch command, which retires settled point 3 and is proposed; and crew's implementation offers a launch a drop names only where the runner launched it. The brief settled four points, quoted under *Decision outcome*, and folded in the runner's items of the cloud's domain-model ledger: 3.b, c-flume.a, c-sluice.a, c-inbox.a and N1.b. The user had accepted that ledger whole ("yes to all").

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

**Commands** (settled point 3; cloud record 0017): "a bolt's branch is made when the user approves its rail card, by a Command a runner takes (sluice branch create), any host keeping the repository, waiting when none is online, an existing branch counting as done". A Command is a tool task under a lease of its own, not a Flume place. The runner carries it out through the tool's command line, and reports the commit the branch stands at as a fact read from git. The control plane counts the branch made from that, whichever way sluice answered. core.1 holds it. The amendment of 2026-11-07, *a bolt is a launch*, below, retires this point.

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

## Amendment, 2026-11-07: a bolt is a launch, and the runner carries no branch command

Status: proposed on 2026-11-07 by the construct stage of the change `a-bolt-is-a-launch`, until the user sees it as a record. The direction is the user's. On 2026-11-06 they accepted Flume's record 0051 on its card ("Approve: push the statement, start code, launch the follow-ups in order"). Its design names this repository's follow-up B1: "the runner keeps the host's watch, and a bolt is a launch". This amendment is that follow-up's second half, and it retires settled point 3 above, the user's own of 2026-11-01. The stage chose four things, each cheap to change: a bolt is judged by its stream's room as any launch is; its send is an act; a branch command is refused, naming sluice, whatever its repository; and the interface and version 1 of the protocol keep a branch command's shape.

The brief: "A control plane's bolt reaches the runner as a Flume launch of its caller's kind with no stages, and its send as a retry of that place's submission, so sluice alone makes and moves a bolt's branch and the runner keeps no branch command."

Sources: Flume's record 0051 and the design of its change `every-place-waiting-on-the-merge-queue-is-settled-at-every-depth`, its decisions 4 to 11 and its follow-ups B1 and C1; Flume's statement, clauses 202 to 206 and section 9's row for the runner (agentplot/blueprints main at b0df70b); `crates/boomdrive/src/carry.rs` and `sluice.rs` at 3f428cf; boomdrive-cloud's records 0017 and 0051; rules core.1, core.9, core.10, core.14, core.15, core.16, core.36 and core.42; the branch-commands and work-into-flume specs.

### Context

Settled point 3 had the runner make a bolt's branch with `sluice branch create`, as a Command under a lease of its own. crew's plugin made it the same way. The bolt's branch had no place. The units' places on it were Flume's and landed through Flume's settles, while crew and ops submitted the bolt, waited on it and removed its branch by hand. One depth was Flume's and the other crew's and the runner's. The user's note of 2026-11-06, quoted whole in Flume's record 0051, ruled that out:

> The code shouldn't know about bolts and units. [...] I don't want it to be hacky. I want it to be a consistent and obvious design in terms of where the merge queue runs. [...] To me, that isn't symmetric and I don't know if that limits us from having other flexibility in terms of how people might want to do branching.

On the outer branch's place they answered: "No worktree: a branch and a record". Flume's statement now has a branch that work lands on, and that itself lands, as a base place's. A base place is a place whose kind lands and has no stages. Flume asks the merge queue to make its branch at its launch, and to close it at its tidy. It takes no room. It is submitted only on its caller's retry naming the submission (Flume's 202 to 206). Its caller is the runner, on a host that keeps one: "each launch its control plane asks, passed as given, a bolt's base place and its units' places among them; a retry of a base place's submission its control plane asks (203)".

### Considered options

1. Keep the branch command beside base places, so a control plane may make a bolt either way.
2. A bolt is a launch of its caller's kind with no stages, passed as given. Its send is a retry of its place's submission. The runner carries no branch command, and refuses one, naming sluice.
3. As option 2, but a branch command is left untaken rather than refused.

### Decision outcome

Option 2.

- **A bolt is a launch.** The runner takes it as it takes any launch: under its lease, for a repository it keeps, while the stream it runs in has room as Flume's status gives it, as one `flume launch` naming what the launch gives (core.9, core.15). Its units are launches with `--base <its branch>`, as they are now. The runner reads no kind, so it cannot tell a kind with no stages from any other. It counts the bolt against its stream's room within its pass, though Flume counts none. In a full stream, then, a bolt waits for a slot it will not hold, and the next pass's listing shows the slot free again.
- **A bolt's send is an act.** It is a retry naming the submission, under the bolt launch's own lease, carried out as `flume retry <place> --submission`, as every retry is. It acts on a launch's place, and a lease the runner has let go must stop it (core.10), so it is no command. Flume's refusal, while a place on the bolt's branch is live, is the act's answer (core.1, core.11).
- **The runner carries no branch command.** A branch command is refused, naming sluice, as every command of a sort this binary does not carry is refused, naming its tool (core.16). The runner reads neither its repository nor its clone, and runs nothing for it. A branch command an earlier release carried out keeps its answer in the report and runs nothing again (core.10).
- **The shapes stay.** The interface's branch sort and its answer's origin commit stay. Version 1 of the hosted protocol names both and carries the interface's wants as their serde writes them, and a release has spoken it (core.42). `report.json` keeps the origin commit too, since the release before reads it (core.36). Neither is filled for a command this release answers.

Not taken:

- **Option 1.** A bolt made by a command has no place, so one depth stays outside Flume, which the user called not symmetric. And sluice would make a bolt's branch outside any place, which Flume's base place now owns (202).
- **Option 3.** A control plane would wait for good on a branch no host will make, never learning why. A refusal names the tool.
- **A launch judged by whether its kind has stages.** The runner would ask Flume about a kind before the launch (core.15), or take the control plane's word for a fact of Flume's.
- **Dropping the branch sort from the interface**, with the hosted implementation reading version 1's branch command as one of no sort the binary carries. That needs a version 1 type of its own for commands, and a conversion the hosted implementation holds none of, for no change in what the runner answers.

core.52 holds it. The commands and work-into-flume specs gain it through the change `a-bolt-is-a-launch`, which retires the branch-commands spec.

### Consequences

- The runner keeps no code that asks sluice for a branch. What it reads of sluice is each submission's entry (the tool-records spec).
- boomdrive-cloud's records 0017 and 0051 have its bolt made by a branch Command, and end the bolt on a refused one. Until the cloud launches a bolt instead, a hosted bolt's branch command is refused with the runner's words. That is the cloud's to follow, and is told to the orchestrator.
- crew's implementation needs nothing. A bolt's launch is a request like any other, and its send is a retry request naming the submission. crew's own follow-up, C1, makes its bolts base places.
- Flume's guards on a base place hold among the places of its own host (202 to 204). Which host takes a bolt and its units, and when the bolt is sent, are the control plane's (core.1).

## Amendment, 2026-11-07: crew's implementation offers a launch a drop names only where the runner launched it

Status: the construct stage's own choice, on the brief of the change `a-dropped-launch-is-never-offered`, 2026-11-07. It changes what crew's implementation gives, read from crew's own records, and no rule of the runner's, so it is cheap to change and is not asked.

The brief: "crew's implementation of the control plane gives no offer for a launch that a drop in requests.rec names, and lets go one the runner holds unlaunched as the interface's let-go has it, so no runner claims or launches a dropped launch; the runner's own rules are unchanged".

### Context

crew drops a launch by a drop request naming it, written in the commit that takes the launch's work out of its plan. crew reads a launch a drop names as no longer standing: its `plugin/lib/runner.py` at `130ce42b`, `standing` and `launch_of`. crew's implementation here read no drop. It offered and claimed a dropped launch as any other. It gave a request whose work had left the plan only where the runner already answered it, so a drop, written as its work leaves, was given to no runner.

On 2026-11-07 a partition's request 425 dropped launch 398 at 20:34:58Z. A host's runner launched 398 at 20:50:03Z, from a pull made at 19:40, in a pass that carried out some 90 wants one after another, as runners did before core.50. No later pull gave request 425, and 398's place went on working.

### Considered options

1. No offer for any launch a drop names, as the brief's first clause reads alone.
2. No offer and no claim for a dropped launch the runner has not launched, and one it holds unlaunched let go; a launched one still given, and its drop given to the runner that holds it, whatever the plan holds.
3. The drop given ahead of its launch in the pull's wants, so the runner's own rule answers a drop of a launch it holds unlaunched.

### Decision outcome

Option 2.

- **A launch is dropped as crew reads it**: a request `Act: drop` names its number as `Of`, whatever its `Host`, `Work` or place in the file.
- **No offer and no claim** for a dropped launch that neither the runner's report nor its branch answers `launched`. A claim of it on the runner's branch leaves at the next write.
- **Let go**: a dropped launch the runner's report holds answered `held`, `refused` or `done` is named in the report's let-go, whatever the plan holds.
- **The drop reaches the runner that holds its launch**: a drop whose launch the runner's report or branch answers is given whatever the plan holds. The runner carries it out by its own rules, `flume drop` on a launched place or `done` from its report.

Not taken:

- **Option 1.** The runner carries a place's status only for a launch the pull gives (record 0021). A launched one's place would never be seen over or let go, and its drop would still never be given.
- **Option 3.** crew's order is its requests'. A launch naming no host would still be claimed, and a dropped launch no runner held would still be offered.
- **Every act on a launch the runner holds given whatever the plan holds.** crew writes no act but a drop for work that has left its plan, so this would change nothing crew writes and give an old act left unanswered a way back.

### Consequences

- A launch a runner takes from a pull made before its drop is still made. Its place is dropped once the next pull gives the drop. Since core.50 that window is a pass's first round.
- core.1's details already allow it: an implementation gives what it reads from the control plane's own rule, in the control plane's own records. No rule changes. The crew-control-plane spec gains it through the change `a-dropped-launch-is-never-offered`.
