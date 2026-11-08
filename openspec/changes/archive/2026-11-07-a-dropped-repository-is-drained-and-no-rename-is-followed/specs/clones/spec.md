# Clones

## MODIFIED Requirements

### Requirement: A dropped repository is removed once it holds nothing the forge lacks

At every pass, a dropped repository's clone and view SHALL be removed, the view first and then the container, once neither holds work the forge lacks and the runner drains the repository no more. Until then both SHALL be left untouched and named with each thing that holds them.

#### Scenario: Dropped with nothing of its own

- **WHEN** `agentplot/flume` is dropped from the configuration, its view clean and detached at `origin/main`, its clone holding only what origin has
- **THEN** after the next pass its view's folder and its container are gone, and it is named as removed

#### Scenario: Emptied later

- **WHEN** a dropped repository is held only by its branch `wip`, and `wip` is then deleted
- **THEN** the next pass removes its view and its container

#### Scenario: The view's folder is not the view any more

- **WHEN** `agentplot/flume` is dropped and its view's folder has been replaced by a folder that is not a worktree of the clone
- **THEN** the folder is left untouched, and the container is removed once the clone holds nothing the forge lacks

## ADDED Requirements

### Requirement: A dropped repository is drained while a place the runner launched there has not ended

The runner SHALL drain a dropped repository while its report holds a launch answered `launched` at a place, naming that repository as the one it was answered in, whose place has not ended. While it drains a repository, its clone and view SHALL be kept as they are, neither fetched nor moved, and named at every pass as held by each such place, with its launch.

#### Scenario: Dropped while a place is live

- **WHEN** `agentplot/flume` is dropped holding nothing the forge lacks, and the report holds launch `a` launched at `P` in `agentplot/flume`, whose carried status reads `P` live
- **THEN** after the next pass its view and its container are as they were, and it is named as held by `P` of `a`

### Requirement: A place has ended once the status carried of it says Flume holds it no more

A place SHALL have ended once the status the report carries of it stands not listed, or reads the place's phase `tidied` or `gone`. A place with no status carried SHALL not have ended.

#### Scenario: The place tidied

- **WHEN** the runner drains `agentplot/flume` for `P` alone, and the status the report carries of `P` then reads its record `P@<at>` tidied and landed
- **THEN** the next pass removes its view and its container

#### Scenario: The place no longer listed

- **WHEN** the runner drains `agentplot/flume` for `P` alone, and the status the report carries of `P` then stands not listed
- **THEN** the next pass removes its view and its container

### Requirement: A report file that cannot be read lets no dropped repository go

Where there is no report file, no repository SHALL be drained. Where the report file cannot be read, no dropped repository SHALL be removed that pass, and each SHALL be named with the report file's fault.

#### Scenario: A report that cannot be read

- **WHEN** `agentplot/flume` is dropped holding nothing the forge lacks, and the report file does not read as a report
- **THEN** after the next pass its view and its container are as they were, and it is named with the report file's path
