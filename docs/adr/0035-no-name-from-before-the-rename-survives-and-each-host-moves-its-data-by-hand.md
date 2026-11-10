---
number: 35
title: No name from before the rename survives, and each host moves its data by hand
status: accepted
date: 2026-11-09
decision-makers:
- Chuck Swanberg
- the design agent
---

# No name from before the rename survives, and each host moves its data by hand

Status: accepted on 2026-11-09. The direction is the user's, in the design agent's pane that day. How it is met is the design agent's own choice, cheap to change. It amends record 0016: its first three points, which keep and read what a runner made under the old name, are replaced by this record. The rest of record 0016 stands.

Deciders: the user, on 2026-11-09, reading a host's draft Boomdrive configuration in the design agent's pane, of its line that put each repository's folder under `~/.local/share/flywheel`: "I don't know why this says flywheel. We've spent all this time moving to Boom Drive. When we cut over everything should be Boom Drive"; and on 2026-11-08, in the same pane: "I want the system to be built the way it should be and then have agents fucking fix the data transition as necessary". The design agent, for the rest.

Sources: record 0016; rules core.10, core.35 and core.36; the README's sections on a pass and "A host from before the rename", and `crates/boomdrive/src/lib.rs` (`BEFORE_RENAME`), `config.rs`, `clone.rs` and `carry.rs`, at the flake bolt's head 295e3bb; agentplot/blueprints `design/principles.md` 12 and 14.

## Context and problem statement

Record 0016 renamed the runner from `flywheel` to `boomdrive` and kept a host working through the rename with code. The runner takes `$XDG_DATA_HOME/flywheel` as its data directory where no `boomdrive` one is there. It carries a clone's `flywheel.address` and `refs/flywheel/view` over to the new names the first time a pass finds them. And it finds a launch's place by `flywheel:<id>` where `boomdrive:<id>` finds none. Rule core.36 asks this of every release: never drop a field the release before reads, and drop a reading only a release later.

So a host that never moves its folder keeps the old name for good. A host of the user's still has its runner keep its clones under `~/.local/share/flywheel`, and the draft of its configuration names that folder for every repository. The user wants no flywheel name left once a host cuts over, and wants each move of data done by hand, by an agent, rather than by code that bridges two shapes.

## Considered options

1. Keep record 0016's readings, and have each host move its folder whenever it chooses.
2. Drop the readings in the next release, and have each host move its data by hand, with the runner stopped, before it runs that release.
3. Add a guard to the runner that refuses to start while a `flywheel` folder sits beside no `boomdrive` one.

## Decision outcome

Option 2.

- **The data directory is Boomdrive's alone.** It is `$XDG_DATA_HOME/boomdrive`, else `~/.local/share/boomdrive`. The runner never reads a folder named `flywheel`.
- **The runner reads only its own names.** In a clone it reads `boomdrive.address` and `refs/boomdrive/view` alone, and it finds a launch's place only by `boomdrive:<id>`. The carry-over of record 0016, `BEFORE_RENAME`, and the README's section "A host from before the rename" go.
- **No flywheel name stays in the runner or the set.** No path, label, ref, git configuration key, mark, constant, option, test name or fixture the runner writes or reads says `flywheel`. A fixture keeps a tool's own words as that tool printed them, except a mark the runner gave the tool, which becomes the runner's own mark.
- **Each host moves its data by hand at its cutover.** This is ops' chore on each host, not code. The runner is stopped and no place is live. Then the chore moves the data directory to `boomdrive`, rewrites each clone's `flywheel` records to `boomdrive`, repairs the link of each worktree the moved clones hold (the views and any place left), and rewrites every file that names the old folder: crew's teams file, the host's own configuration and any other. Only then does the host run a release without the readings. On each of the user's hosts, the chore is the ops agent's.
- **core.36 follows the user's rule.** A release may drop or rename a field the release before it read in a file the runner keeps. Its release notes then name the move each host makes by hand before it runs the release. A file the runner cannot read still makes it refuse to start.

Not taken:

- **Option 1.** A host that never moves keeps the old name, and the code that reads it never goes.
- **Option 3.** A guard that reads the old name is code for the move in another form. The user ruled out both a fallback and a guard. The chore's order keeps the runner from starting on an empty folder.

## Consequences

- Good: once a host has cut over, nothing on it says flywheel. The runner keeps one reading of its folder, its clones and its marks.
- Bad: a host that runs the new release before its folder moves starts a runner with an empty data directory and no report. It clones every repository again, and the runner's record of the acts it already carried out is gone. The chore's order is the only thing that prevents it. A release's notes name the move for that reason.
- Bad: a place still marked `flywheel:<id>` is no longer found by that mark. The chore runs with no place live, so none is left to find.
- This record changes core.36's rule and details, and core.10's details. The change `the-runner-keeps-no-name-from-before-the-rename` builds it.
