# Runner protocol

## Purpose

The runner's protocol with a hosted control plane, as JSON Schema of draft 2020-12 in the hosted implementation's crate, one directory per version, which the cloud pins by revision. Each end checks every message it is sent against its version. It covers where each version lives; the envelope, its ids and its two directions; the closed objects; the interface's shapes it carries; the copies of Flume's and sluice's answers; the terminal's key exchange and frames; what version 1 leaves out; and the runner's types, tested against it (core.42; record 0015; flywheel-cloud records 0016, 0049 and 0056).

## ADDED Requirements

### Requirement: Each version of the protocol is JSON Schema in the hosted implementation's crate

The runner's protocol with a hosted control plane SHALL be JSON Schema of draft 2020-12, one directory per version under `crates/flywheel-hosted/protocol/`, version 1 at `v1/`. Each schema file SHALL name its own address under the version's directory as its `$id`. Every `$ref` SHALL resolve among the version's own files, and no check SHALL fetch a file. The crate's `FILES` SHALL hold every schema file of the version, and nothing else (core.42; flywheel-cloud record 0056).

#### Scenario: Each file is a schema of its draft

- **WHEN** each file of version 1 is checked against the draft 2020-12 metaschema
- **THEN** each passes, and its `$id` is its own address under `v1/`

#### Scenario: Nothing outside the version is referenced

- **WHEN** validators are built from `FILES` alone, with nothing fetched
- **THEN** every `$ref` of every file resolves

#### Scenario: Every file is listed

- **WHEN** the files under `protocol/v1/` ending `.schema.json` are compared with `FILES`
- **THEN** each is in `FILES`, and each in `FILES` is among them

### Requirement: Every message names its version, carries an id of its own and holds one body

Every message SHALL be an object of `protocol_version`, which is 1, `id`, its sender's id for it, and `body`, which holds exactly one kind. A message's id SHALL be 1 to 128 letters, digits and `._:/-` (flywheel-cloud core.36, core.38; core.21).

#### Scenario: A message with no id

- **WHEN** a `hello` with no `id`, one with an empty `id`, and one whose `id` holds a space are checked
- **THEN** each is refused

#### Scenario: A message of another version

- **WHEN** a `hello` naming `protocol_version` 2 is checked
- **THEN** it is refused

#### Scenario: A body of two kinds, or of none

- **WHEN** a message whose body holds both `hello` and `report`, one whose body is empty, and one whose body names a kind version 1 does not have are checked
- **THEN** each is refused

### Requirement: Each direction sends only its own kinds

A runner SHALL send only `hello`, `report`, `terminal_opened`, `terminal_output`, `terminal_closed` and `message_refused`. A control plane SHALL send only `pulled`, `reported`, `terminal_open`, `terminal_resume`, `terminal_input`, `terminal_close` and `message_refused`. Each direction's envelope SHALL take its own kinds alone (core.42).

#### Scenario: A body of the other direction

- **WHEN** a `pulled` is checked as a runner's message, and a `hello` as a control plane's
- **THEN** each is refused

### Requirement: A message holds only what its version names

Every object version 1 names SHALL take only the fields it names, at every depth, but a sluice row's `flaky` and a thing a stage's end made, each carried as its tool printed it. Every field the interface's types always write SHALL be required. A time of the runner's or the control plane's SHALL be RFC 3339 in UTC to the second. An integer SHALL be within 2^53 − 1 of zero, and a lease's number SHALL be 1 or more (core.42).

#### Scenario: A field the version does not name

- **WHEN** examples with a field added at the envelope, in a launch, in a lease, in a place's status copy and in a key share are checked
- **THEN** each is refused

#### Scenario: A time not in UTC to the second

- **WHEN** a lease expiring at `2026-11-01T12:00:00+02:00`, and one at `2026-11-01T10:00:00.5Z`, are checked
- **THEN** each is refused

#### Scenario: An integer past what JavaScript reads exactly

- **WHEN** a lease numbered 2^53, and one numbered 0, are checked
- **THEN** each is refused

### Requirement: The messages carry the interface's shapes

Version 1 SHALL carry the interface's `Id`, `Lease`, `Want`, `ToolVersion`, `Answer`, `Acted`, `Commanded` and `InboxCarried` as `flywheel-control`'s serde types write them, and its `Hello` and `Report` but for their copies. An acted's and a commanded's outcome SHALL be `done` or `refused` (flywheel-cloud record 0056).

#### Scenario: The interface's values pass

- **WHEN** one value of each want, act and command sort, a lease with an expiry and one without, a launch with every field and one with none, each tool version, an acted, a commanded and an inbox carried are written by `flywheel-control`'s types and checked against their definitions in version 1
- **THEN** each passes

#### Scenario: An act's outcome

- **WHEN** an acted, and a commanded, whose outcome is `launched` are checked
- **THEN** each is refused

### Requirement: A place's status is carried as its copy

A place's status SHALL be carried as the copy of `flume status --json`'s place, holding only the fields design Decision 7 names, each as Flume printed it. It SHALL hold no agent's last words, screen, printed output, notes, settings or mode; no task's text; no readiness; no group's title; no teardown's words; no end's report; no verdict's words; and no path but the place's own (core.27; flywheel-cloud core.108, record 0049).

#### Scenario: The copy's fields are the list

- **WHEN** the field paths of `place-status.schema.json` are read
- **THEN** they equal the list design Decision 7 gives, which the test holds

#### Scenario: Text the cloud does not keep

- **WHEN** a status copy holding an agent's `last_words`, `screen` or `printed`, a task's `text`, `readiness`, a group's `title`, a teardown's `words` or an end's `report` is checked
- **THEN** each is refused

#### Scenario: Flume's answers as copies

- **WHEN** the `report` examples whose copies are cut from Flume 0.4.1's places are checked: one waiting on an answer, one on hold, a role's run delivered, a submission under test, a landing kept by a failed teardown, a tidied place, and a live agent with its pane's address
- **THEN** each passes

### Requirement: The host's capacity is Flume's host part as values, its streams without their queues

A hello's capacity SHALL be `missing`, with words, or `printed`, holding Flume's `running`, `bound` and `room`, and each stream's `name`, `priority`, `running`, `bound` and `room`, each as Flume printed it. It SHALL hold no stream's queue, and none of Flume's pools, addresses, bases or host queue (core.26, core.27).

#### Scenario: The capacity's fields are the list

- **WHEN** the field paths of the host part's copy are read
- **THEN** they equal the list design Decision 8 gives, which the test holds

#### Scenario: A stream's queue

- **WHEN** a hello whose capacity names a stream with its `queue` is checked
- **THEN** it is refused

### Requirement: sluice's entry is carried as its copy

A submission's entry SHALL be carried as an object of sluice's `id`, `branch`, `head`, `state` and `landed_as`, and its `failed` rows each with only `task`, `hash`, `flaky` and `system`, each as sluice printed it. It SHALL hold no row's `log` (core.11, core.27).

#### Scenario: The entry's fields are the list

- **WHEN** the field paths of `submission-entry.schema.json` are read
- **THEN** they equal the list design Decision 9 gives, which the test holds

#### Scenario: A row's log

- **WHEN** a `report` whose entry's failed row holds `log` is checked
- **THEN** it is refused

### Requirement: A report is answered by its id, and a failing message is refused whole

A control plane's `reported` SHALL name the `report` message it answers by that message's id, with the ids it lets go and the ids it refused with its words. Either end's `message_refused` SHALL name the message that failed its check by its id, with words (flywheel-cloud core.36; core.42).

#### Scenario: A reported with no report named

- **WHEN** a `reported` with no `report` is checked
- **THEN** it is refused

#### Scenario: A message refused

- **WHEN** a `message_refused` naming a message's id with words is checked as each direction's
- **THEN** it passes

### Requirement: A terminal's frames are sealed end to end and carry their offsets

A terminal SHALL open on the browser's key share and answer with the runner's, each share holding its suite, its key, its signature and its signing key's certificate. Every frame of a terminal's bytes SHALL carry them only sealed, in base64url, with its offset in its direction's stream. A resume SHALL name the offset its browser has seen, and a frame of the runner's SHALL say whether it is a redraw (core.30; flywheel-cloud record 0016; research rows S6 and F54).

#### Scenario: A frame with no offset

- **WHEN** a `terminal_output` with no `offset` is checked
- **THEN** it is refused

#### Scenario: A frame in the clear

- **WHEN** a `terminal_output`, and a `terminal_input`, carrying bytes under a field other than `sealed`, and one whose `sealed` is not base64url, are checked
- **THEN** each is refused

#### Scenario: An unsigned share

- **WHEN** a `terminal_open` whose share has no `signature`, and one whose share has no `certificate`, are checked
- **THEN** each is refused

#### Scenario: A terminal's messages

- **WHEN** the examples of `terminal_open`, `terminal_opened`, `terminal_output` with and without a redraw, `terminal_input`, `terminal_resume`, `terminal_close` and `terminal_closed` are checked
- **THEN** each passes

### Requirement: Version 1 names no grant and no credential

No field of version 1 SHALL be named `grant`, `role`, `permission`, `token`, `credential`, `secret` or `password`. A connection's account, and what it may do, SHALL come from its enrolment, never from a message (core.4; flywheel-cloud records 0047 and 0065).

#### Scenario: Every name read

- **WHEN** every property name, `properties` key and `$defs` key of every file of version 1 is read
- **THEN** none is among those names

### Requirement: The runner's types write and read only what the schema takes

`flywheel_hosted::protocol::v1` SHALL hold a type for every message of version 1, using the interface's own types wherever version 1 carries a shape as the interface writes it. Every example of version 1 SHALL read into its direction's type and write back to an equal value that passes the schema. Every kind SHALL have an example (research row F50).

#### Scenario: Every example read and written back

- **WHEN** each example under `examples/from-runner/` and `examples/to-runner/` is read into its direction's `Message`, written back, and the written value checked
- **THEN** the written value equals the example's as JSON and passes its direction's schema

#### Scenario: Every kind has an example

- **WHEN** the kinds of the examples are read
- **THEN** every kind of each direction is among them
