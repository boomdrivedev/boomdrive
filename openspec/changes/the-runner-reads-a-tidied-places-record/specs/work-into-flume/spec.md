# Work into Flume

## MODIFIED Requirements

### Requirement: Every pull carries Flume's host part as Flume printed it

The hello's capacity SHALL be Flume's host part, `running`, `bound`, `room` and `streams`, each as Flume printed it in the pass's listing, the `flume --json status` it reads before its pull. It SHALL leave out Flume's `pools`, `addresses`, `bases` and `terminal_kept`.

#### Scenario: Streams as Flume printed them

- **WHEN** the stub `flume` prints a listing whose host part holds two streams and a pool named `madswan`
- **THEN** the hello's capacity holds the two streams' text as the stub printed it, and the word `madswan` appears nowhere in the hello

### Requirement: Room is read from Flume's status and counted down within a pass

The runner SHALL read room from the pass's listing, the `flume --json status` it reads before its pull. The host SHALL have room while its `room` is `null` or above zero. A stream SHALL have room while the host has room, its own `room` is `null` or above zero, and its `queue` is empty. Each launch the pass makes SHALL count one against its stream's room and the host's.

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

### Requirement: Flume's status of each place is carried as Flume gave it

For each launch with a place, the runner SHALL carry Flume's status of the place as Flume printed it, on one line, under the launch's lease number: where it keeps the launch's repository with its clone made, `flume --json status --repository <container> <place>`'s whole answer. It SHALL ask for that answer again, and move its time, only when the place's entry in the statuses' listing differs, as a JSON value, from the entry inside the carried answer.

#### Scenario: The status byte for byte

- **WHEN** a launched place's entry in the listing changes, and the stub `flume` prints a fixture for `status --json <place>`
- **THEN** the carried status is the fixture's text, byte for byte, on one line, and its time is the pass's

#### Scenario: Only the host changed

- **WHEN** a pass's listing differs from the last only in its host part
- **THEN** no `flume status <place>` is run, and the report is unchanged

### Requirement: A place the listing no longer holds keeps its last status

A place the statuses' listing no longer holds SHALL be asked for once a pass by its location. Where Flume answers, its answer SHALL be carried. Where Flume refuses, the last carried status SHALL stand, marked as not listed.

#### Scenario: A place tidied

- **WHEN** the listing no longer holds a carried place, and the stub `flume` refuses `status <place>`
- **THEN** its last carried status stands, marked as not listed

## ADDED Requirements

### Requirement: The statuses are carried against Flume's listing as it stands once the wants are answered

The statuses' listing SHALL be Flume's listing as it stands once the pull's wants are answered. Where answering them ran a `flume launch`, an act of Flume's or a `sluice branch create`, it SHALL be a `flume --json status` read then. Where it ran none, it SHALL be the pass's listing (record 0027).

#### Scenario: A place tidied while the pass answers its wants

- **WHEN** a carried place `P` stands live in the pass's listing, as carried, and while the pass launches another offer Flume tidies `P` landed, keeping its record `P@<at>`
- **THEN** Flume's listing is read again after the launch, and that pass carries Flume's answer for `P` as its status, naming `P@<at>`, tidied and landed

#### Scenario: A quiet pass reads the listing once

- **WHEN** a pass's wants run no `flume` or `sluice` command
- **THEN** it runs `flume --json status` once, before its pull
