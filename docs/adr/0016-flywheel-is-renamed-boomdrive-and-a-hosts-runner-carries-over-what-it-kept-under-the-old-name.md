---
number: 16
title: Flywheel is renamed Boomdrive, and a host's runner carries over what it kept under the old name
status: proposed
date: 2026-11-04
decision-makers:
- Chuck Swanberg
---

# Flywheel is renamed Boomdrive, and a host's runner carries over what it kept under the old name

Status: proposed on 2026-11-04 by the code stage of the chore `flywheel-is-renamed-boomdrive`. The rename and its names are the user's decision, in their words below. How this repository carries it out is the stage's choice, made on the written rules. The record is accepted when the user says yes to it.

Deciders: the user, for the rename and its names; the code stage, for how this repository carries it out.
Sources: the user's words in the brief of the chore `flywheel-is-renamed-boomdrive`, 2026-11-04; the names GitHub gives the repositories on that day; record 0001; rules core.5, core.10, core.14, core.17, core.19, core.31, core.33, core.34, core.36 and core.42; the clones and work-into-flume specs.

## Context and problem statement

The user renamed Flywheel across every repository at once: "the ecosystem, the org, the branding, the cloud and the opensource runner tool are all boomdrive. There is also a "boom" open source tool that does the claim reconciliation." They approved **drive** for a customer's unit of work, "a Flywheel", as ubiquitous language, so that the interface and the code use the same words. They ruled that crew's word for a partition's state branch follows it. They asked that the same pass spell the word enroll, enrolls, enrolled, enrolling and enrollment, never enrol or enrolment.

This repository was agentplot/flywheel-runner, and GitHub now names it agentplot/boomdrive. Its binary, crates, paths, records and rules all said flywheel. A runner of that name already runs on a host, by hand and under crew. It keeps its clones, its views and a report of what it carried out in `~/.local/share/flywheel`, records each clone in the clone's git config under `flywheel`, and marks each place it launches `flywheel:<id>`. A Flume place stands as a worktree of one of those clones.

Renamed outright, the runner would break three rules on that host. It would make a second clone of every repository beside the first (core.14). It would lose its report, and not find its own launches by their marks, so it could carry out an act or a launch twice (core.10). And it would move a record out of a clone's git config without reading the old place (core.36).

## The names

| was | is |
|---|---|
| Flywheel, the product, brand and ecosystem, and the family of open tools on its sites | Boomdrive |
| Flywheel Cloud, the control plane, agentplot/flywheel-cloud | Boomdrive Cloud, agentplot/boomdrive-cloud |
| the runner, agentplot/flywheel-runner | Boomdrive, agentplot/boomdrive |
| the command `flywheel` | `boomdrive` |
| the crates `flywheel`, `flywheel-control`, `flywheel-crew` and `flywheel-hosted` | `boomdrive`, `boomdrive-control`, `boomdrive-crew` and `boomdrive-hosted` |
| agentplot/flywheel-web | agentplot/boomdrive-web |
| tailrace, agentplot/tailrace | timber, agentplot/timber |
| a Flywheel, a customer's unit of work, and `Flywheel` in code | a drive, drives, and `Drive` in code |
| crew's flywheel branch, a partition's state branch | the drive branch |
| the mark `flywheel:<partition>/<n>` | `boomdrive:<partition>/<n>` |
| `~/.config/flywheel/config.toml` and `~/.local/share/flywheel` | `~/.config/boomdrive/config.toml` and `~/.local/share/boomdrive` |
| a clone's `flywheel.address`, `flywheel.view` and `refs/flywheel/view` | `boomdrive.address`, `boomdrive.view` and `refs/boomdrive/view` |
| version 1's addresses under `agentplot/flywheel-runner/main/crates/flywheel-hosted/protocol/v1/`, and its subprotocol `flywheel-runner.v1` | under `agentplot/boomdrive/main/crates/boomdrive-hosted/protocol/v1/`, and `boomdrive.v1` |
| the commits on crew's runner branch, by `flywheel@<host>` | by `boomdrive@<host>` |
| enrol, enrolment | enroll, enrollment |

boom keeps its name, and so does anything third-party that says flywheel. agentplot/flywheel, the original, is archived, and a mention of it as history names it as archived. agentplot/flywheel-next keeps its name: GitHub still gives it that name, and the rename does not change it.

## Considered options

For what a runner kept on a host under the old name:

1. Rename every name, and have the runner carry over what it kept. It keeps a data directory from before the rename where no new one is there. It carries a clone's records over to the new names the first time a pass finds the clone. And it still finds a launch by its old mark.
2. Rename every name except the runner's own files and records: the data directory, the clone's git keys and ref, and the mark keep `flywheel`.
3. Rename every name, and leave what a host kept to the hosts' change. The host would move the data directory, repair every worktree of its clones and rewrite each clone's records, and the old marks would be lost.

For the configuration file: read it only at the new path, or also at the old path where the new one is not there.

## Decision outcome

Option 1, with the configuration read only at the new path. The names are the table's.

- **The data directory from before is kept, not moved.** The data directory is `boomdrive`, or the `flywheel` beside it where only that one is there. Every place and view made in its clones names the clone by its absolute path, so moving the directory would break them all. And its report is what keeps each act carried out once (core.10, core.14, core.36). A host that wants the new name moves the directory by its own change, with the runner stopped.
- **A clone's records are carried over.** The first pass that finds a clone recording `flywheel.address`, and no `boomdrive.address`, moves `refs/flywheel/view` to `refs/boomdrive/view`. It then renames the clone's git config section from `flywheel` to `boomdrive`, each step through the host's git (core.34). The ref moves first, so a carry-over that is cut short is done again at the next pass. A carry-over that fails is named, and nothing is kept that pass, rather than the clone being taken for a folder the runner did not make. This is core.36's way: write the new names, read the old ones until they are carried over, and drop the reading a release later.
- **A launch's old mark is still found.** A place launched before the rename is marked `flywheel:<id>`. The runner looks for an offer's place by `boomdrive:<id>` first, then by `flywheel:<id>`, so a launch whose answer was lost is never launched twice (core.10).
- **The configuration is the host's.** It is read only at `$XDG_CONFIG_HOME/boomdrive/config.toml`, else `~/.config/boomdrive/config.toml`. The hosts' change writes it there. A runner that finds no file there refuses to start, exiting 2 and naming the path. Reading the old path too would leave the runner configured by a file nobody writes any more (core.17).
- **Version 1 of the protocol is renamed in place.** No release of the runner has spoken it. So its schema files' addresses, its `ADDRESS`, its subprotocol and the context its terminal shares are signed in take the new names within version 1 itself (core.42), and the cloud moves its pin with them.
- **What keeps its text.** These kept their text through the rename: the archived OpenSpec changes; the constitution interview, a dated record of what was asked and said on 2026-11-01; Flume 0.4.1's and sluice 0.61.1's answers under `crates/boomdrive/tests/fixtures/`, as those tools printed them, and the literals in the tests that read them; and the user's own words quoted in record 0009. Record 0014's scrub, the change `boomdrives-tree-holds-nothing-private`, makes their private names and paths neutral, and keeps the constitution interview in the archive, as agentplot/boomdrive-archive's `design/research/the-constitution-interview.md`. A record's or a rule's names change, never its decision.
- **Every crate is renamed.** The brief named `flywheel` and `flywheel-hosted`. `flywheel-control` and `flywheel-crew` take the same new prefix, so no crate in the tree keeps the old name.

Option 2 would keep the host working with no code, but would leave the runner's own files under a name the user has retired, with no way to drop it later. Option 3 would leave the repository with the least code. But the hosts' change would then rewrite the runner's records in its clones, which only the runner may move (core.14), and the old marks would be lost, so a lost answer could launch a place twice.

### Consequences

- `BEFORE_RENAME`, in `crates/boomdrive/src/lib.rs`, holds the old name. The data directory, the clone's carry-over and the old mark read it. A later release drops the carry-over and the old mark once every host's clones are carried over and its launches from before are done. It drops the data directory's fallback once each host has moved its own.
- The hosts' change writes the configuration at `~/.config/boomdrive/config.toml`, and runs `boomdrive` in place of `flywheel`. It moves no data directory.
- boomdrive-cloud moves its pin of version 1 to the revision that renames it, and speaks `boomdrive.v1` and the renamed share context.
- crew's runner branch is written by `boomdrive@<host>` from this change on.
