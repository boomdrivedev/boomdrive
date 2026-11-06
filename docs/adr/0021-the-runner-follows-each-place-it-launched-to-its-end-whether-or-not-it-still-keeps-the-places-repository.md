---
number: 21
title: The runner follows each place it launched to its end, whether or not it still keeps the place's repository
status: proposed
date: 2026-11-05
decision-makers:
- Chuck Swanberg
---

# The runner follows each place it launched to its end, whether or not it still keeps the place's repository

Status: proposed on 2026-11-05 by the code stage of the fix `the-runner-follows-each-place-it-launched-to-its-end`. The direction is the orchestrator's, in its brief's words below; the user has not ruled on it yet, and rules at the fix's review. How the runner reads a place it no longer keeps the repository of, and that crew's implementation gives such a launch, are the stage's choices, made on this repository's rules and the two tools' statements. Record 0020 is the sibling change `boomdrive-publishes-a-flake-of-the-family-and-the-runners-service`'s, on its branch.

Deciders: the orchestrator's brief, for the direction; the code stage, for how; the user, at the fix's review.
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

## Decision outcome

Option 1.

- **A place is followed to its end whether or not the runner still keeps its repository.** For each launch the pull gives that the report holds launched at a place, the pass carries Flume's status of the place. Where the configuration keeps the launch's repository with its clone made, it asks as before, `flume --json status --repository <container> <place>`. Where it does not, it asks `flume --json status <place>`, by the place's location alone. The status is still read again only when the place's entry in the pass's listing differs from the one carried, and a tidied place's record is still its entry.
- **No keeper is told of a landing in a repository the runner does not keep.** The keeper keeps only what the configuration names (core.5), so a landing there is told to none.
- **crew's implementation gives each launch the runner answered.** A kit's launch the runner's report or its branch already answers is given at every pull, whatever the hello names. Its repository is the kept address its `Repo` names where there is one, else its `Repo` as written. It is neither claimed again nor named in the log. A launch no runner has answered is given, as before, only where the runner keeps its repository. The runner takes no action on a launch it already answered `launched` but to follow its place; one it held and no longer keeps the repository of, it refuses, naming the repository (core.5).

Option 2 would have the runner read work its control plane no longer gives. Which work is the host's is the control plane's (core.1), and what goes over a partition's connection names only the work it gives (core.26). A control plane that stops giving a launch lets it go by its answer to the report. Option 3 would have the runner guess a forge's rename. Which repository a kit is, is crew's to say, by renaming its kit. Requests are never changed, so a request made before the rename names the old name for good, and following its place needs no repository. Option 4 would change the call for every place the runner keeps, at a release that does not yet check Flume's floor (core.38). Keeping `--repository` where the clone is kept changes nothing for those places.

### Consequences

- core.5's "Not ruled out" gains a place the runner launched, followed in Flume's status by its location after the configuration drops its repository, citing this record. No clone or view of that repository is made, fetched or kept for it.
- The work-into-flume spec gains "A place is followed to its end whether or not its repository is kept", and its "Flume's status of each place is carried as Flume gave it" and "A landing is told to the keeper once" are amended. The crew-control-plane spec gains "A launch the runner answered is given whatever the hello names", and its "A launch naming no host is given only where this runner keeps its repository" is amended. That is the OpenSpec change `the-runner-follows-each-place-it-launched-to-its-end`.
- `Flume::status_of` takes the clone's container where the runner keeps one, and none where it does not.
- sluice's entry of a submission is read only in a kept clone, as before. A place whose repository the runner no longer keeps carries its landing in Flume's status alone, which is what crew reads first (`flume_facts`).
- On the host, the first pass of this release carries the tidied place's record, with its landing and its commit, to the runner's branch.
- A request made after the rename that names a repository by its old name is given to no runner until crew renames its kit. That is crew's, told to the orchestrator.
