---
number: 21
title: A repository is drained before it goes, the runner following each place it launched there to its end, and no rename is handled in flight
status: accepted
date: 2026-11-05
decision-makers:
- Chuck Swanberg
---

# A repository is drained before it goes, the runner following each place it launched there to its end, and no rename is handled in flight

Status: accepted by the user on 2026-11-07, on their card. Proposed on 2026-11-05 by the code stage of the fix `the-runner-follows-each-place-it-launched-to-its-end`, and revised on 2026-11-07 on the user's note on its card, quoted below, which rules out handling a rename in flight. How the runner reads a place it no longer keeps the repository of, and that crew's implementation gives such a launch, are the stage's choices, made on this repository's rules and the two tools' statements. Record 0020 is the sibling change `boomdrive-publishes-a-flake-of-the-family-and-the-runners-service`'s, on its branch.

Deciders: the user, 2026-11-07, on this record's card: "We are not going to build special functionality to handle repositories being renamed in flight. Big picture with boomdrive, you add repositories and, when you remove repositories, I don't think there's a rename. I think you would drain the work from it, remove it, do the rename separately, and then add it back"; the design agent, for how a drained repository is kept; the orchestrator's brief and the code stage, for the first draft.
Sources: the orchestrator's brief of the fix `the-runner-follows-each-place-it-launched-to-its-end`, 2026-11-05; Flume's statement, clauses 56, 58 and 62, and `flume status`'s usage at 0.6.4; crew's `plugin/lib/runner.py` (`flume_facts`, `places.rec`'s descriptor) and `plugin/lib/asks.py` (`status_of`); record 0016; rules core.1, core.5, core.9, core.26 and core.38; the work-into-flume and crew-control-plane specs.

## Context and problem statement

On 2026-11-05 a host's runner launched a unit's place for a kit request whose `Repo` named the control plane's repository by its old name, `flywheel-cloud`, in its clone of agentplot/flywheel-cloud. It carried the place's status once, at its construct stage, waiting on readiness. The place then went through construct, code and verify, its submission landed on its bolt, and Flume tidied it, keeping its record as `<location>@<at>` for an end pending for the planner. The runner's branch still holds the status of construct, so crew reads the unit as waiting at construct, its planner cannot see that it landed, and the bolt cannot go on.

The cause, read from the code and the host:

- **No pass ran for most of the place's life.** The runner had no service on that host until later that day: a pass by hand launched the place and another carried a retry of it, and none ran while it built.
- **When its service started, the runner followed it no more.** By then the configuration named the repository by its new name, agentplot/boomdrive-cloud (record 0016), and the old clone was gone.
  - **crew's implementation gave no want for the launch.** A kit's launch naming no `Host` is given only where its `Repo` names a repository the hello names (`Kept::No => continue`). That test ran before the one for a launch the runner already answered, though the crew-control-plane spec says a launch the runner's report answers is given at every later pull. `flywheel-cloud` names no repository the runner keeps now, so the pull gave nothing for the launch.
  - **The pass read a place's status only through a kept clone.** It carried the status of a launch only where the launch's repository was kept with its clone made, to pass that clone as `--repository`. So even a launch given was followed no more.
- **The act after the launch, the place's tidied folder name and the restarts were not the cause.** The act was answered at a pass by hand. The runner already reads a tidied place's record, `<location>@<at>`, as the place's entry (Flume's 62). A restart keeps the report file, which holds the launch.

Flume reads a place by its location whatever repository it is asked in: `flume --json status <location>` answers the place, or its tidied record, from any folder, with or without `--repository`, whose use is to resolve a `<kind>--<name>` (clause 58; its usage at 0.6.4). crew documents what `places.rec` holds as `flume status --json <place>`'s answer.

The orchestrator's brief, 2026-11-05: "Find why the runner stopped following that place. … Fix it here, so that every place the runner launched is followed until Flume reports it over or tidied. Write that state, with its landing, to `places.rec`."

## Considered options

1. **The runner follows each launch the pull gives to its end, asking Flume by the place's location where it no longer keeps the repository; crew's implementation gives each launch the runner answered whatever the hello names.**
2. **The runner follows each launch its report holds, whether the pull gives it or not.**
3. **The renamed repository is matched to its new address**, from the forge's redirect or a table of renames.
4. **Every place is asked by its location alone**, with no `--repository` at all.
5. **No rename is handled, and a repository is drained before it goes** (the user's).

## Decision outcome

Option 5, the user's.

- **A rename is a removal and an addition.** Neither the runner nor crew's implementation matches a repository's old name to its new one. A launch is given, taken and followed only for a repository the runner keeps.
- **A repository is drained before it goes.** Where the configuration drops a repository while a place the runner launched in it is live, the runner launches nothing more there, keeps its clone and view, and carries each such place's status to its end as before. Once none is live, it removes them as it removes any dropped repository's. A rename is then done in four steps: drain the work, remove the repository, rename it on the forge, and add it back.
- **No keeper is told of a landing in a repository the runner does not keep.** The keeper keeps only what the configuration names (core.5).

Option 1 follows a place by its location after its repository is gone, and has crew's implementation give a launch whatever the hello names. That is a rename handled in flight, which the user ruled out. Option 2 would have the runner read work its control plane no longer gives, and which work is the host's is the control plane's (core.1). Option 3 would have the runner guess a forge's rename. Option 4 changes the call for every place the runner keeps, for a case that no longer arises.

### Consequences

- core.5's "Not ruled out" names a dropped repository's clone and view kept, with nothing launched in it, while a place the runner launched there is live, in place of a place followed by its location alone.
- The fix `the-runner-follows-each-place-it-launched-to-its-end`, built on option 1, is taken back by a later change once this record is accepted: the runner asks Flume of a place only through a kept clone, crew's implementation gives a launch only where the runner keeps its repository, draining ones among them, and the work-into-flume and crew-control-plane specs say so.
- A request that names a repository by an old name is given to no runner. Its work is launched again under the repository's name once the repository is added back.