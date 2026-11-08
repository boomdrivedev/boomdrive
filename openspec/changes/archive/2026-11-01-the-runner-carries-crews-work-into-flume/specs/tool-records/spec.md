# Tool records

## Purpose

What the runner carries back of sluice and of an inbox. For sluice, it carries each submission's entry: its state and verdict rows, read from sluice's own answer, never its logs or heads. For an inbox the control plane names, it carries the inbox's records as references, never raw material. Each is carried as the tool gave it, through the tool's command line or weir's record formats, and nothing of a repository's content is carried (core.11; flywheel-cloud core.4 and core.28; ledger items c-sluice.a and c-inbox.a).

## ADDED Requirements

### Requirement: Each submission Flume names is carried from sluice's own answer

For each launch whose carried status names a sluice entry, at `submission.read.found.submission.id`, the runner SHALL run `sluice status <id> --json` in the launch's clone container. It SHALL carry these fields of sluice's answer, each as sluice printed it: `id`, `branch`, `head`, `state`, `failed`, each row with only its `task`, `hash`, `flaky` and `system`, and `landed_as`.

#### Scenario: An entry carried

- **WHEN** a launched place's carried status names entry 412, and the stub `sluice` prints its submission
- **THEN** the stub is run as `sluice status 412 --json` in the clone's container, and the carried entry holds its `id`, `branch`, `head`, `state` and `landed_as` as printed

#### Scenario: No submission

- **WHEN** a launched place's carried status names no sluice entry
- **THEN** no `sluice` command is run for it

### Requirement: No log of sluice's is carried

The runner SHALL leave out every row's `log`, and SHALL read nothing from `refs/sluice/logs` or `refs/sluice/heads` (core.11).

#### Scenario: A failed row without its log

- **WHEN** the stub `sluice` prints a submission whose `failed` row holds a `log`
- **THEN** the carried entry holds the row's `task` and `hash` as printed, and no `log`

### Requirement: An entry is read until it is final

The runner SHALL read an entry again at each pass until its `state` is `landed`, `failed`, `conflict` or `withdrawn`, and SHALL carry it anew only when its fields change.

#### Scenario: A landed entry

- **WHEN** a carried entry's `state` is `landed`
- **THEN** later passes run no `sluice status` for it

### Requirement: An inbox is read at each new tip of its branch

For an inbox the control plane names on `branch` of `repository`, the runner SHALL read the tip of `origin/<branch>` in its clone at each control pass. Where the tip moved since the last carried, it SHALL read the files under `inbox/` at that tip through git, and check them whole by weir-signals.

#### Scenario: An unchanged tip

- **WHEN** the inbox's branch's tip has not moved since the last pass
- **THEN** no file of it is read again, and the carried inbox is unchanged

### Requirement: An inbox's records are carried as references

Where weir takes an inbox's files, the runner SHALL carry the tip's commit, each record file under `inbox/` by its path with its text as the tip holds it, and each item's `brief.md` as `sha256:<hex>` of its bytes. It SHALL read nothing a capture's pointer names.

#### Scenario: An inbox carried

- **WHEN** an inbox is named on `inbox` in `example/crew`, and the forge's `inbox` branch holds a capture, a signal and an item with a brief written in weir's layout
- **THEN** the carried inbox holds the tip's commit, the capture's and signal's files with their text as the tip holds it, the item's record, and the brief as its digest

### Requirement: An inbox weir refuses keeps what was carried

Where weir refuses an inbox's files, the runner SHALL carry the tip with weir's words, and the files carried before SHALL stand.

#### Scenario: An inbox weir refuses

- **WHEN** the inbox's branch's tip holds a capture file weir cannot read
- **THEN** the carried inbox names the tip with weir's words, and the files carried before stand

### Requirement: An inbox is carried only from a repository the runner keeps

An inbox in a repository the configuration does not name SHALL be answered with words naming it, and nothing read. An inbox whose clone is not yet made SHALL wait. An inbox on a branch origin does not hold SHALL be answered with words naming the branch (core.5).

#### Scenario: A repository not kept

- **WHEN** an inbox is named in a repository the configuration does not name
- **THEN** it is answered with words naming the repository, and no git command reads it

#### Scenario: No such branch

- **WHEN** an inbox is named on `inbox` in a kept repository whose origin has no `inbox` branch
- **THEN** it is answered with words naming `inbox`
