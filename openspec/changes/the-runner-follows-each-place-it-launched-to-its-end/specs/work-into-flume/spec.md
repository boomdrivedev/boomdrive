# Work into Flume

## MODIFIED Requirements

### Requirement: Flume's status of each place is carried as Flume gave it

For each launch the pull gives that the report holds launched at a place, the runner SHALL carry Flume's status of the place, as Flume printed it, on one line, under the launch's lease number. Where the configuration keeps the launch's repository with its clone made, it SHALL be `flume --json status --repository <container> <place>`'s whole answer. Where it does not, it SHALL be `flume --json status <place>`'s, asked by the place's location alone. A place SHALL be followed so through its end, over or tidied, whether or not the configuration still names its repository. The runner SHALL ask for that answer again, and move its time, only when the place's entry in the pass's listing differs, as a JSON value, from the entry inside the carried answer. A place's entry SHALL be the one standing at its location, else Flume's record of it tidied, `<location>@<at>`.

#### Scenario: The status byte for byte

- **WHEN** a launched place's entry in the listing changes, and the stub `flume` prints a fixture for `status --json <place>`
- **THEN** the carried status is the fixture's text, byte for byte, on one line, and its time is the pass's

#### Scenario: Only the host changed

- **WHEN** a pass's listing differs from the last only in its host part
- **THEN** no `flume status <place>` is run, and the report is unchanged

#### Scenario: A place whose repository the runner keeps no more

- **WHEN** the report holds launch `a` launched at `P` in `example/kit`, the configuration no longer names `example/kit`, the pull still gives `a`, and `P` lands, then is tidied while an end of it is pending
- **THEN** each time, `flume --json status P` is run with no `--repository`, and its answer is carried: first `P` landed, then its record `P@<at>` tidied and landed

#### Scenario: A place found tidied by its mark

- **WHEN** an offer `a` in a repository the configuration does not name is pulled, and Flume's listing holds the record `P@<at>`, tidied and landed, marked `flywheel:a`
- **THEN** `a` is answered `launched` at `P`, and the carried status is `flume --json status P`'s answer, naming `P@<at>`

### Requirement: A landing is told to the keeper once

The first pass that reads a carried place landed SHALL tell the keeper of a landing on its repository, so the clone and its view are fetched at once. A landing in a repository the configuration does not name, or whose clone is not yet made, SHALL be told to no keeper.

#### Scenario: A landing told once

- **WHEN** a carried place's status first reads landed
- **THEN** the keeper is told of a landing on its repository once, and not at later passes

#### Scenario: A landing in a repository not kept

- **WHEN** a carried place whose repository the configuration no longer names first reads landed
- **THEN** its status is carried, and the keeper is told nothing
