## ADDED Requirements

### Requirement: Each part saves its share of the one report file

The report file SHALL name in `parts` the labels of the parts whose share it holds. A part's share SHALL be the entries its control plane owns. To save its share, a part SHALL take the file's own lock, held for that save alone, read the file, replace the part's entries, keep every other entry as it stands, add its label to `parts`, and write the whole by rename (record 0026; core.36).

#### Scenario: A part's save keeps every other part's share

- **WHEN** the report file holds `home`'s launch `home/1` and `work`'s act `work/2`, and `home` saves its share holding `home/1` and `home/3`
- **THEN** the file holds `home/1`, `home/3` and `work/2`, and names `home` and `work` in `parts`

### Requirement: A part the report file does not hold takes its share from its pull

A report file with no `parts`, as a release before wrote it, SHALL hold every part's share. A part SHALL read its share from the file where the file names it in `parts` or names no `parts`. Where the file is missing, or names others in `parts` and not the part, the part SHALL take its share from the report its pull gives back (core.10).

#### Scenario: A file from the release before

- **WHEN** the report file has no `parts` and holds `work`'s act `work/2` settled, and `work` pulls `work/2` again
- **THEN** `work/2` is answered from the file, and no `flume` command is run for it

#### Scenario: A part the file does not hold

- **WHEN** the report file names only `home` in `parts`, and `work`'s pull gives back a report holding `work/2` settled
- **THEN** `work` takes `work/2` from its pull's report, answers it from there, and saves it with `work` named in `parts`

## MODIFIED Requirements

### Requirement: Room is read from Flume's status and counted down within a pass

The runner SHALL read room from the pass's listing, the `flume --json status` it reads before its pull. The host SHALL have room while its `room` is `null` or above zero. A stream SHALL have room while the host has room, its own `room` is `null` or above zero, and its `queue` is empty. Each launch a part's pass makes SHALL count one against its stream's room and the host's, in that pass alone: the runner SHALL keep no count across parts (record 0026; core.9).

#### Scenario: Room for one

- **WHEN** two offers name stream `work`, whose `room` is 1 and whose `queue` is empty
- **THEN** the first is launched, and the second is left unanswered

#### Scenario: A stream with a queue

- **WHEN** an offer names stream `work`, whose `room` is 2 and whose `queue` holds a place
- **THEN** it is left unanswered, and no `flume` command is run for it

#### Scenario: A host with no room

- **WHEN** an offer names stream `work`, whose `room` is `null`, and the host's `room` is 0
- **THEN** it is left unanswered

#### Scenario: Unbounded

- **WHEN** an offer names stream `default`, whose `room` and the host's are both `null`
- **THEN** it is launched

#### Scenario: Each part counts its own room

- **WHEN** the runner carries `home` and `work`, each offering one launch naming no stream, and every listing Flume gives says the host's `room` is 1
- **THEN** each part launches its own offer, each pass counting only its own launch

### Requirement: Each launch, act and command is carried out once

The runner SHALL keep its report in `<data>/control/report.json`, the whole report of every part, each part saving its share after every act and command and before the next. A want a part's share holds settled SHALL be answered from it and not carried out again. An offer with no answer, whose mark `boomdrive:<id>` Flume's listing holds on a place, SHALL be answered `launched` at that place and not launched again.

#### Scenario: A retry not made twice

- **WHEN** a pass retries a place, its report then fails, and the runner starts again from its saved report with the same act pulled
- **THEN** no second `flume retry` is run, and the act is reported `done`

#### Scenario: A launch found by its mark

- **WHEN** the report file is missing, and an offer `home/12` is pulled whose mark `boomdrive:home/12` Flume's listing holds on place `P`
- **THEN** it is answered `launched` at `P`, and no `flume launch` is run
