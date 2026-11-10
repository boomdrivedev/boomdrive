# runner-protocol Specification

## Purpose
The runner's protocol with a hosted control plane, as JSON Schema of draft 2020-12 in the hosted implementation's crate, one directory per version, which the cloud pins by revision. Each end checks every message it is sent against its version. It covers where each version lives; the envelope, its ids and its two directions; the closed objects; the interface's shapes it carries; the copies of Flume's and sluice's answers; the terminal's key exchange and frames; what version 1 leaves out; and the runner's types, tested against it (core.42; record 0015; boomdrive-cloud records 0016, 0049 and 0056).

## Requirements

### Requirement: Each version of the protocol is JSON Schema in the hosted implementation's crate

The runner's protocol with a hosted control plane SHALL be JSON Schema of draft 2020-12, one directory per version under `crates/boomdrive-hosted/protocol/`: `v1/` and `v2/`. Each schema file SHALL name its own address as its `$id`. Every `$ref` SHALL resolve among its version's files, and no check SHALL fetch one. Each version's module SHALL hold in `FILES` every schema file of its version, and nothing else (core.42; record 0037; boomdrive-cloud record 0056).

#### Scenario: Each file is a schema of its draft

- **WHEN** each file of each version is checked against the draft 2020-12 metaschema
- **THEN** each passes, and its `$id` is its own address under its version's directory

#### Scenario: Nothing outside the version is referenced

- **WHEN** validators are built from one version's `FILES` alone, with nothing fetched, for each version
- **THEN** every `$ref` of every file of that version resolves

#### Scenario: Every file is listed

- **WHEN** the files under each version's directory ending `.schema.json` are compared with that version's `FILES`
- **THEN** each is in `FILES`, and each in `FILES` is among them

### Requirement: Every message names its version, carries an id of its own and holds one body

Every message SHALL be an object of `protocol_version`, its version's number, `id`, its sender's id for it, and `body`, which holds exactly one kind. A message's id SHALL be 1 to 128 letters, digits and `._:/-` (boomdrive-cloud core.36, core.38; core.21).

#### Scenario: A message with no id

- **WHEN** a `hello` with no `id`, one with an empty `id`, and one whose `id` holds a space are checked against version 2
- **THEN** each is refused

#### Scenario: A message of another version

- **WHEN** a `hello` naming `protocol_version` 1 is checked against version 2, and one naming `protocol_version` 2 against version 1
- **THEN** each is refused

#### Scenario: A body of two kinds, or of none

- **WHEN** a message whose body holds both `hello` and `report`, one whose body is empty, and one whose body names a kind version 2 does not have are checked against version 2
- **THEN** each is refused

### Requirement: Each direction sends only its own kinds

A runner SHALL send only `enroll`, `hello`, `report`, `terminal_opened`, `terminal_output`, `terminal_closed` and `message_refused`. A control plane SHALL send only `enrolled`, `enroll_refused`, `pulled`, `reported`, `terminal_open`, `terminal_resume`, `terminal_input`, `terminal_close` and `message_refused`. Each direction's envelope SHALL take its own kinds alone (core.42).

#### Scenario: A body of the other direction

- **WHEN** a `pulled` is checked as a runner's message, and a `hello` as a control plane's
- **THEN** each is refused

#### Scenario: Enrollment's kinds in the other direction

- **WHEN** an `enroll` is checked as a control plane's message, and an `enrolled` and an `enroll_refused` as a runner's
- **THEN** each is refused

### Requirement: A message holds only what its version names

Every object a version names SHALL take only the fields it names, at every depth, but a sluice row's `flaky` and a thing a stage's end made, each carried as its tool printed it. Every field the interface's types always write SHALL be required. A time of the runner's or the control plane's SHALL be RFC 3339 in UTC to the second. An integer SHALL be within 2^53 − 1 of zero, and a lease's number SHALL be 1 or more (core.42).

#### Scenario: A field the version does not name

- **WHEN** examples of version 2 with a field added at the envelope, in a launch, in a lease, in a place's status copy, in an end's delivery, in a confirm, in a preview, in what the runner answered of a preview, in a preview's copy, in a release's copy and in a key share are checked
- **THEN** each is refused

#### Scenario: A time not in UTC to the second

- **WHEN** a lease expiring at `2026-11-01T12:00:00+02:00`, and one at `2026-11-01T10:00:00.5Z`, are checked against version 2
- **THEN** each is refused

#### Scenario: An integer past what JavaScript reads exactly

- **WHEN** a lease numbered 2^53, and one numbered 0, are checked against version 2
- **THEN** each is refused

### Requirement: The messages carry the interface's shapes

Version 2 SHALL carry the interface's `Id`, `Lease`, `Want`, `ToolVersion`, `Answer`, `Acted`, `Commanded` and `InboxCarried` as `boomdrive-control`'s serde types write them, every want and act sort the interface names among them, and its `Hello` and `Report` but for their copies. Version 1 SHALL carry the same but a preview, a land, a confirm and `previews`. An acted's and a commanded's outcome SHALL be `done` or `refused` (boomdrive-cloud record 0056; record 0037 as amended on 2026-11-09).

#### Scenario: The interface's values pass

- **WHEN** one value of each want, act and command sort version 2 names, a preview not done and one done, a land and a confirm among them, a lease with an expiry and one without, a launch with every field and one with none, each tool version, an acted, a commanded and an inbox carried are written by `boomdrive-control`'s types and checked against their definitions in version 2
- **THEN** each passes

#### Scenario: Every sort the interface names

- **WHEN** a value of every sort of `Want`, `ActSort` and `CommandSort`, each named by a match with no wildcard, is written by `boomdrive-control`'s types and checked against version 2's `want`
- **THEN** the sorts named are every sort the interface has, and each value passes

#### Scenario: An act's outcome

- **WHEN** an acted, and a commanded, whose outcome is `launched` are checked against version 2
- **THEN** each is refused

### Requirement: A place's status is carried as its copy

A place's status SHALL be carried as the copy of `flume status --json`'s place, holding only the fields design Decision 7 names and record 0037 adds, each as Flume printed it. It SHALL hold no agent's last words, screen, printed output, notes, settings or mode; no task's text; no readiness; no group's title; no teardown's words; no end's report; no stall's move, answerer, stop-short or hold; no verdict's words; and no path but the place's own (core.27; record 0037).

#### Scenario: The copy's fields are the list

- **WHEN** the field paths of version 2's `place-status.schema.json` are read
- **THEN** they equal the list design Decision 7 gives with the paths record 0037 adds, which the test holds

#### Scenario: Text the cloud does not keep

- **WHEN** a status copy holding an agent's `last_words`, `screen` or `printed`, a task's `text`, `readiness`, a group's `title`, a teardown's `words`, an end's `report`, or a stall's `waiting` or `stop_short` is checked against version 2
- **THEN** each is refused

#### Scenario: Flume's answers as copies

- **WHEN** the `report` examples of version 2 are checked: one waiting on an answer, one on hold, a role's run delivered, a submission under test, a landing kept by a failed teardown, a tidied place, a live agent with its pane's address, a dropped place held for the runner, a done place held for the runner, an end handed over at a session's start, an end owed, expired and one confirmed, and an end by its delivery alone
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

A control plane's `reported` SHALL name the `report` message it answers by that message's id, with the ids it lets go and the ids it refused with its words. Either end's `message_refused` SHALL name the message that failed its check by its id, with words (boomdrive-cloud core.36; core.42).

#### Scenario: A reported with no report named

- **WHEN** a `reported` with no `report` is checked
- **THEN** it is refused

#### Scenario: A message refused

- **WHEN** a `message_refused` naming a message's id with words is checked as each direction's
- **THEN** it passes

### Requirement: A terminal's frames are sealed end to end and carry their offsets

A terminal SHALL open on the browser's key share and answer with the runner's, each share holding its suite, its key, its signature and its signing key's certificate. Every frame of a terminal's bytes SHALL carry them only sealed, in base64url, with its offset in its direction's stream. A resume SHALL name the offset its browser has seen, and a frame of the runner's SHALL say whether it is a redraw (core.30; boomdrive-cloud record 0016; research rows S6 and F54).

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

### Requirement: No version names a grant or a credential

No field of any version SHALL be named `grant`, `role`, `permission`, `token`, `credential`, `secret` or `password`. A connection's account, and what it may do, SHALL come from its enrollment, never from a message (core.4; boomdrive-cloud records 0047 and 0065).

#### Scenario: Every name read

- **WHEN** every property name, `properties` key and `$defs` key of every file of each version is read
- **THEN** none is among those names

### Requirement: The runner's types write and read only what the schema takes

`boomdrive_hosted::protocol::v2` SHALL hold a type for every message of version 2, using the interface's own types wherever version 2 carries a shape as the interface writes it, and the runner SHALL speak no other version. Every example of version 2 SHALL read into its direction's type and write back to an equal value that passes the schema, and every kind SHALL have an example (research row F50; record 0037).

#### Scenario: Every example read and written back

- **WHEN** each example under version 2's `examples/from-runner/` and `examples/to-runner/` is read into its direction's `Message`, written back, and the written value checked
- **THEN** the written value equals the example's as JSON and passes its direction's schema

#### Scenario: Every kind has an example

- **WHEN** the kinds of version 2's examples are read
- **THEN** every kind of each direction is among them

### Requirement: An enrollment is one enroll, answered by enrolled or enroll_refused

An `enroll` SHALL hold `key`, 32 bytes, `signature`, 64 bytes, each in base64url, and `machine`, the host's name. An `enrolled` SHALL hold `host`, the control plane's id for the host. An `enroll_refused` SHALL hold `enroll`, the `enroll` message's id, and `words`. None SHALL hold a token: the token is the request's bearer (core.4, core.31; record 0018).

#### Scenario: Enrollment's examples

- **WHEN** the examples of `enroll`, `enrolled` and `enroll_refused` are checked
- **THEN** each passes, and reads into its direction's type and writes back equal

#### Scenario: An enroll unsigned, or holding a token

- **WHEN** an `enroll` with no `signature`, one whose `key` is not 32 bytes, and one holding a `token` are checked
- **THEN** each is refused

### Requirement: The README names the two paths and the signatures' contexts

The protocol's README SHALL describe version 2: `runner/enroll` and `runner/connect` under the control plane's address, the kinds each takes, the subprotocol `boomdrive.v2`, the `BoomdriveHost` header, and the bytes each enrollment's and connection's signature covers, each context starting `boomdrive.v2`, with a worked example of each. It SHALL name what a `pulled` and a `report` hold, and how version 1 differs. The hosted implementation SHALL sign exactly those bytes (records 0018 and 0037).

#### Scenario: The README's worked contexts

- **WHEN** the hosted implementation makes the enrollment's and the connection's contexts from the README's fixed key, token, origin, time and nonce
- **THEN** each equals the README's worked bytes, and each starts with the bytes of `boomdrive.v2`

#### Scenario: The README of version 2

- **WHEN** the protocol's README is read
- **THEN** its envelope names `protocol_version` 2; its paths name the subprotocol `boomdrive.v2`; its files name `preview.schema.json`; its copies name an end's `delivery`, an end's delivery alone and a preview's copy; its messages name a confirm and a land among a `pulled`'s acts, a preview among its wants, and `previews` in a `report`; and a section `Version 1` names `boomdrive.v1` and says it has no land and no preview

### Requirement: Version 1 carries no land

Version 1 names no land, and a release has spoken it, so it SHALL NOT change to carry one. A want whose act is a land SHALL fail version 1's check, and a land SHALL reach the runner from a hosted control plane only in a later version that names it (core.42, core.53).

#### Scenario: A land is no act of version 1

- **WHEN** a want whose act is a land, as `boomdrive-control`'s types write it, is checked against version 1's `want`
- **THEN** it is refused

### Requirement: Version 1 carries no preview

Version 1 names no preview, and a release has spoken it, so it SHALL NOT change to carry one. A want that is a preview SHALL fail version 1's check, and a preview SHALL reach the runner from a hosted control plane only in a later version that names it. The runner's report SHALL go up in version 1 with no `previews` (core.42, core.55).

#### Scenario: A preview is no want of version 1

- **WHEN** a want that is a preview, as `boomdrive-control`'s types write it, is checked against version 1's `want`
- **THEN** it is refused

### Requirement: Version 1 carries no confirm

Version 1 names no confirm, and its copy of a place's status names no end's delivery key, and a release has spoken it, so it SHALL NOT change to carry either. A want whose act is a confirm SHALL fail version 1's check, and a confirm SHALL reach the runner from a hosted control plane only in a later version that names it (core.42, core.56).

#### Scenario: A confirm is no act of version 1

- **WHEN** a want whose act is a confirm, as `boomdrive-control`'s types write it, is checked against version 1's `want`
- **THEN** it is refused

### Requirement: Version 2 names every end Flume 0.21.0 gives a place

Version 2's copy of a place's status SHALL name four forms of an end: a stage's, a submission's, a stall's, and a drop's or a done's, `{"of":"over",…}`. Each SHALL require `key`, `read_at`, `deliveries`, `pending_for`, `owed`, each reader with whether it expired and is known, and `confirmed`, each reader with when, and take `delivery` and `expires_at`. A hand-over SHALL be `by` `notice`, `wait`, `inbox` or `session_start` (core.27, core.42; record 0037; weir record 0006).

#### Scenario: A drop's and a done's end

- **WHEN** a place's status copy holding an end `{"of":"over","over":"dropped",…}` with its key, delivery and readers, and one holding `"over":"done"`, are checked against version 2
- **THEN** each passes

#### Scenario: A hand-over at a session's start

- **WHEN** a status copy whose end was handed over `by` `session_start` is checked against version 2
- **THEN** it passes

#### Scenario: An end with no key

- **WHEN** a status copy whose stage's end has no `key`, and one whose end has no `owed`, are checked against version 2
- **THEN** each is refused

### Requirement: An end of a form version 2 does not name goes up as its delivery alone

Version 2 SHALL name an end's delivery alone, `end_delivery`, as a form of an end: its `of`, any string as Flume printed it, `key`, `read_at`, `owed`, `confirmed` and `pending_for`, each required, and `delivery` and `expires_at`, and nothing else. No end SHALL pass both `end_delivery` and a named form (core.27, core.42; record 0037).

#### Scenario: An end of a form unnamed

- **WHEN** a status copy whose end is `{"of":"moved"}` with its key, delivery and readers alone is checked against version 2
- **THEN** it passes

#### Scenario: Content beside a delivery alone

- **WHEN** a status copy whose end is `{"of":"moved"}` with its key, delivery and readers, and its `deliveries`, or a `stage`, or `words`, is checked against version 2
- **THEN** it is refused

### Requirement: Version 2 carries a control plane's confirm

Version 2 SHALL name a confirm among the acts a `pulled` holds, `{"confirm":{"recipient":…,"deliveries":[…]}}`, as `boomdrive-control`'s `ActSort::Confirm` writes it, a string recipient and an array of string keys, bounding neither, so that a confirm the runner refuses reaches the runner as an act (core.42, core.56; record 0037).

#### Scenario: A confirm is an act of version 2

- **WHEN** a want whose act is a confirm of two keys, and one of no key, as `boomdrive-control`'s types write them, are checked against version 2's `want`, and a `pulled` holding the first against version 2's `to-runner.schema.json`
- **THEN** each passes

### Requirement: Version 1 stays as a release spoke it

Every file under `protocol/v1/`, its schemas and its examples, SHALL stay as release `v0.7.3` holds it, and no file SHALL be added there. Every example of version 1 SHALL still pass its direction's envelope of version 1. `boomdrive_hosted::protocol::v1` SHALL hold version 1's files and no type (core.42; record 0037).

#### Scenario: Version 1 as released

- **WHEN** each file under `protocol/v1/` is read and its SHA-256 taken
- **THEN** the files are exactly those the test lists, each with the digest the test holds for it

#### Scenario: Version 1's examples still pass

- **WHEN** each example under version 1's `examples/` is checked against its direction's envelope of version 1
- **THEN** each passes

### Requirement: Version 2 carries a land

Version 2 SHALL name a land among the acts a `pulled` holds, written as the word `land`, as `boomdrive-control`'s `ActSort::Land` writes it, holding nothing but its act's launch and lease. The retry naming the submission SHALL stay among version 2's acts (core.42, core.53; record 0037 as amended on 2026-11-09).

#### Scenario: A land is an act of version 2

- **WHEN** a want whose act is a land, as `boomdrive-control`'s types write it, is checked against version 2's `want`, and a `pulled` holding it against version 2's `to-runner.schema.json`
- **THEN** each passes, the act's `what` being `"land"`

### Requirement: Version 2 carries a preview and its done

Version 2 SHALL name a preview among the wants a `pulled` holds, `{"preview":{"id","lease","repository","branch","revision","done"}}`, as `boomdrive-control`'s `Want::Preview` writes it, each field required. Its branch and its revision SHALL be any string, so that a preview sluice refuses reaches the runner as a preview, and not the whole `pulled` as a message (core.42, core.55; record 0037 as amended on 2026-11-09).

#### Scenario: A preview is a want of version 2

- **WHEN** a preview not done, one done, and one whose revision is `R1`, as `boomdrive-control`'s types write them, are checked against version 2's `want`, and a `pulled` holding a preview and its done against version 2's `to-runner.schema.json`
- **THEN** each passes

### Requirement: Version 2's report holds what the runner answered of each preview

Every report of version 2 SHALL hold `previews`, by id, each preview the runner answered with its `lease`, `repository`, `revision`, `done`, `outcome` (`held`, `done` or `refused`), `copy`, `words`, `status` and `at`, each required: `copy` the copy of sluice's answer, a preview's or a release's, or `null`; `status` the copy of sluice's listing with its time and whether it was listed, or `null` (core.27, core.42, core.55; record 0037 as amended on 2026-11-09).

#### Scenario: sluice's answers of a preview as copies

- **WHEN** the `report` examples of version 2 are checked: a preview brought up, a preview held, a preview put away, and a preview sluice refused
- **THEN** each passes

#### Scenario: A report with no previews

- **WHEN** a `report` of version 2 with no `previews` is checked
- **THEN** it is refused

#### Scenario: A preview's outcome

- **WHEN** what the runner answered of a preview, its outcome `launched`, is checked against version 2
- **THEN** it is refused

### Requirement: A preview's copy holds no output of its command

Version 2's `preview.schema.json` SHALL hold the copy of a preview, its `name`, required, `branch`, `revision`, `state`, `made` and `addresses`, a map of strings, and the copy of a release, its `preview`, required, and `released`, each as sluice printed it. Neither SHALL hold a preview's `checkout`, `names`, `ports`, `exit`, `output` or `release_failed` (core.11, core.27, core.55; record 0037 as amended on 2026-11-09).

#### Scenario: The copies' fields are the lists

- **WHEN** the field paths of `preview.schema.json`'s `preview` and `release` are read
- **THEN** the first's are `name`, `branch`, `revision`, `state`, `made` and `addresses`, and the second's `preview` and `released`, which the tests hold

#### Scenario: A preview's output

- **WHEN** a report whose preview's copy, or whose preview's status copy, holds `output`, `checkout`, `names`, `ports`, `exit` or `release_failed` is checked against version 2
- **THEN** each is refused
