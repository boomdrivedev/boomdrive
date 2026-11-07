# Work into Flume

## MODIFIED Requirements

### Requirement: An offer is taken only for a repository the runner keeps

The runner SHALL answer `refused` an offer for a repository it neither keeps nor drains, with words naming the repository and saying this host's configuration does not keep it, whatever Flume's listing holds under the offer's mark. It SHALL leave unanswered an offer for a repository the configuration names whose clone is not yet made. It SHALL run no command for either.

#### Scenario: A repository not kept

- **WHEN** an offer of a launch in `agentplot/sluice` is pulled and the configuration does not name it
- **THEN** it is answered `refused`, its words naming `agentplot/sluice`, and no `flume` command is run

#### Scenario: A clone not yet made

- **WHEN** an offer of a launch in `agentplot/flume` is pulled, the configuration names it, and its clone is not yet made
- **THEN** it is left unanswered, and is launched at the first pass after its clone is made

#### Scenario: A mark in a repository not kept

- **WHEN** an offer `a` in `example/old-kit` is pulled, the runner neither keeps nor drains `example/old-kit`, and Flume's listing holds a place marked `boomdrive:a`
- **THEN** `a` is answered `refused`, its words naming `example/old-kit`, and no `flume status <place>` is run

### Requirement: Flume's status of each place is carried as Flume gave it

For each launch with a place whose repository's clone stands, whether the configuration names the repository or the runner drains it, the runner SHALL carry Flume's status of the place as Flume printed it, on one line, under the launch's lease number: `flume --json status --repository <container> <place>`'s whole answer. It SHALL ask for that answer again, and move its time, only when the place's entry in the statuses' listing differs, as a JSON value, from the entry inside the carried answer.

#### Scenario: The status byte for byte

- **WHEN** a launched place's entry in the listing changes, and the stub `flume` prints a fixture for `status --json <place>`
- **THEN** the carried status is the fixture's text, byte for byte, on one line, and its time is the pass's

#### Scenario: Only the host changed

- **WHEN** a pass's listing differs from the last only in its host part
- **THEN** no `flume status <place>` is run, and the report is unchanged

### Requirement: A landing is told to the keeper once

The first pass that reads a carried place landed SHALL tell the keeper of a landing on its repository, so the clone and its view are fetched at once. A landing in a repository the configuration does not name, one the runner drains among them, or whose clone is not yet made, SHALL be told to no keeper.

#### Scenario: A landing told once

- **WHEN** a carried place's status first reads landed
- **THEN** the keeper is told of a landing on its repository once, and not at later passes

#### Scenario: A landing in a repository not kept

- **WHEN** a carried place whose repository the runner drains first reads landed
- **THEN** its status is carried, and the keeper is told nothing

## ADDED Requirements

### Requirement: The hello names the repositories drained apart from those kept

Apart from its repositories, the hello SHALL name the address of each repository the runner drains, as the clones spec has it, and SHALL not name it among its repositories.

#### Scenario: A repository draining in the hello

- **WHEN** the configuration no longer names `example/kit`, its clone is made, and the report holds launch `a` launched at `P` in `example/kit`, whose carried status reads `P` live
- **THEN** the hello names `example/kit` as drained, and not among its repositories

### Requirement: Nothing more is launched in a repository drained

Where the runner drains a repository, an offer there whose place Flume's listing holds under its mark SHALL be answered `launched` at that place, and any other SHALL be refused as one in a repository not kept. It SHALL run no command for either.

#### Scenario: A repository drained

- **WHEN** the runner drains `example/kit` for launch `a`, and an offer `b` of a launch in `example/kit` is pulled, with room on the host
- **THEN** `b` is answered `refused`, its words naming `example/kit`, and no `flume launch` is run

### Requirement: A place whose repository has no clone is asked of Flume no more

For a launch whose repository's clone does not stand, the runner SHALL ask Flume nothing of its place, and the status last carried SHALL stand as it was.

#### Scenario: A place in a repository with no clone

- **WHEN** the pull gives launch `a`, launched at `P` in `example/old-kit`, the runner has no clone of `example/old-kit`, and `P`'s entry in the listing changes
- **THEN** no `flume status P` is run, and `a`'s carried status is as it was

### Requirement: A place in a repository drained is followed to its end through its clone

Where the configuration drops a repository while a place the runner launched there has not ended, the runner SHALL carry that place's status, as each pull gives its launch, through the repository's clone, which it drains, until the place has ended, as the clones spec has it. No old name of a repository SHALL be matched to a new one: a launch is followed only through the clone of the repository it names.

#### Scenario: A place in a repository drained

- **WHEN** the report holds launch `a` launched at `P` in `example/kit`, the configuration no longer names `example/kit`, its clone stands, the pull still gives `a`, and `P` lands, then is tidied while an end of it is pending
- **THEN** each time, `flume --json status --repository <container> P` is run with `example/kit`'s container, and its answer is carried: first `P` landed, then its record `P@<at>` tidied and landed

### Requirement: Each launch's repository is kept with its answer

The runner's report SHALL hold, with each launch it answers, the repository the offer named, as the offer wrote it, from the pass that first answers it, and SHALL never change it. A launch the report holds with no repository SHALL gain the offer's at the first pull that gives it.

#### Scenario: A launch's repository kept with its answer

- **WHEN** offer `a` of a launch in `agentplot/flume` is launched, and a report file of before holds launch `b` launched with no repository
- **THEN** the report file's `a` names `agentplot/flume`, and its `b` names the repository of `b`'s offer once a pull gives `b`

## REMOVED Requirements

### Requirement: A place is followed to its end whether or not its repository is kept

**Reason**: Record 0021, as revised and accepted by the user on 2026-11-07, rules out handling a repository renamed in flight. A place is followed only through its repository's clone, which the runner drains while the place has not ended, and never by its location alone once the clone is gone.

**Migration**: A repository dropped while a place the runner launched there has not ended is drained, as the clones spec's "A dropped repository is drained while a place the runner launched there has not ended" has it, and its place followed through its clone, as "A place in a repository drained is followed to its end through its clone" has it.
