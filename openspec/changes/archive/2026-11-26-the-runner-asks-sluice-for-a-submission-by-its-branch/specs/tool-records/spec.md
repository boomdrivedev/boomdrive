## MODIFIED Requirements

### Requirement: Each submission Flume names is carried from sluice's own answer

A launch's carried status can name a sluice entry at `submission.read.found.submission`, by its `id` and its `target`. Where it does, and the repository's clone stands, named or drained, the runner SHALL run `sluice status <id> --branch <target> --json` in that clone's container. It SHALL carry of sluice's answer, each as printed: `id`, `branch`, `head`, `state`, `failed`, each row with only `task`, `hash`, `flaky` and `system`, and `landed_as`.

#### Scenario: An entry carried

- **WHEN** a launched place's carried status names entry 412 with the target `main`, and the stub `sluice` prints its submission
- **THEN** the stub is run as `sluice status 412 --branch main --json` in the clone's container, and the carried entry holds its `id`, `branch`, `head`, `state` and `landed_as` as printed

#### Scenario: No submission

- **WHEN** a launched place's carried status names no sluice entry
- **THEN** no `sluice` command is run for it

#### Scenario: An entry carried from a repository drained

- **WHEN** the runner drains `example/kit` for launch `a`, and `a`'s carried status names entry 412 with the target `main`
- **THEN** the stub `sluice` is run as `sluice status 412 --branch main --json` in `example/kit`'s clone container, and its entry is carried

#### Scenario: A number on two branches

- **WHEN** a launched place's carried status names entry 18 with the target `bolt/parser`. The stub `sluice` refuses 18 with no branch, and gives an entry of its own for 18 on each of `bolt/parser` and `main`
- **THEN** the stub is run once, as `sluice status 18 --branch bolt/parser --json`, and the carried entry is the one on `bolt/parser`

### Requirement: An entry is read until it is final

The runner SHALL read an entry again at each pass until its `state` is `landed`, `failed`, `conflict` or `withdrawn`. It SHALL carry the entry anew only when its fields change. An entry SHALL be its number on its branch. A final entry SHALL stop the reads only where its `id` and `branch` are the number and the target that Flume's status names.

#### Scenario: A landed entry

- **WHEN** a carried entry's `state` is `landed`
- **THEN** later passes run no `sluice status` for it

#### Scenario: The same number on another branch

- **WHEN** the carried entry 18 on `bolt/parser` is `landed`, and the place's carried status then names entry 18 with the target `main`
- **THEN** the next pass runs `sluice status 18 --branch main --json`, and carries its entry

## ADDED Requirements

### Requirement: A refused entry is asked for no more

Where sluice refuses the entry of a number on a branch, the runner SHALL keep the refusal on the launch and SHALL say it once. The refusal SHALL hold the number, the branch and sluice's words as given. It SHALL NOT ask sluice for that number on that branch again. It SHALL ask for another number or branch that Flume's status names, and an answer SHALL clear the refusal. Where sluice could not be run, the runner SHALL keep nothing and ask again at the next pass (core.11).

#### Scenario: A refused entry is asked for no more

- **WHEN** a launched place's carried status names entry 18 with the target `main`, and the stub `sluice` refuses it at every ask, over five passes
- **THEN** the stub is run once, the launch keeps the refusal with `18`, `main` and sluice's words, and the refusal is said once

#### Scenario: Another entry after a refusal

- **WHEN** sluice refused entry 18 on `main`, and the place's carried status then names entry 19 with the target `main`, which the stub answers
- **THEN** the next pass runs `sluice status 19 --branch main --json`, carries its entry, and the launch keeps no refusal
