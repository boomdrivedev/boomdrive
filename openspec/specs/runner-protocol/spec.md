# runner-protocol Specification

## Purpose
The runner's protocol with a hosted control plane, as JSON Schema of draft 2020-12 in the hosted implementation's crate, one directory per version, which the cloud pins by revision. Each end checks every message it is sent against its version. It covers where each version lives; the envelope, its ids and its two directions; the closed objects; the interface's shapes it carries; the copies of Flume's and sluice's answers; the terminal's key exchange and frames; what version 1 leaves out; and the runner's types, tested against it (core.42; record 0015; boomdrive-cloud records 0016, 0049 and 0056).

## Requirements

### Requirement: Each version of the protocol is JSON Schema in the hosted implementation's crate

The runner's protocol with a hosted control plane SHALL be JSON Schema of draft 2020-12, one directory per version under `crates/boomdrive-hosted/protocol/`: `v1/`, `v2/`, `v3/` and `v4/`. Each schema file SHALL name its own address as its `$id`. Every `$ref` SHALL resolve among its version's files, and no check SHALL fetch one. Each version's module SHALL hold in `FILES` every schema file of its version, and nothing else (core.42; records 0037, 0052 and 0053; boomdrive-cloud record 0056).

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

- **WHEN** a `hello` with no `id`, one with an empty `id`, and one whose `id` holds a space are checked against version 3
- **THEN** each is refused

#### Scenario: A message of another version

- **WHEN** a `hello` naming `protocol_version` 2 is checked against version 3, one naming `protocol_version` 3 against version 2, and one naming `protocol_version` 1 against version 3
- **THEN** each is refused

#### Scenario: A body of two kinds, or of none

- **WHEN** a message whose body holds both `hello` and `report`, one whose body is empty, and one whose body names a kind version 3 does not have are checked against version 3
- **THEN** each is refused

### Requirement: Each direction sends only its own kinds

A runner SHALL send only `enroll`, `hello`, `report`, `terminal_opened`, `terminal_output`, `terminal_closed`, `message_refused` and, from version 4, `forge_token`. A control plane SHALL send only `enrolled`, `enroll_refused`, `pulled`, `reported`, the four `terminal_` kinds and `message_refused`, from version 4 `forge_token_given` and `forge_token_refused`, and from version 6 `newest_release`. Each envelope SHALL take its own kinds alone (core.42).

#### Scenario: A body of the other direction

- **WHEN** a `pulled` is checked as a runner's message, and a `hello` as a control plane's
- **THEN** each is refused

#### Scenario: Enrollment's kinds in the other direction

- **WHEN** an `enroll` is checked as a control plane's message, and an `enrolled` and an `enroll_refused` as a runner's
- **THEN** each is refused

#### Scenario: A forge token's kinds in the other direction

- **WHEN** version 4's `forge_token` is checked as a control plane's message of version 4, and its `forge_token_given` and `forge_token_refused` as a runner's
- **THEN** each is refused

### Requirement: A message holds only what its version names

Every object a version names SHALL take only the fields it names, at every depth, but a sluice row's `flaky` and a thing a stage's end made, each carried as its tool printed it. Every field the interface's types always write SHALL be required. A time of the runner's or the control plane's SHALL be RFC 3339 in UTC to the second. An integer SHALL be within 2^53 − 1 of zero, and a lease's number SHALL be 1 or more (core.42).

#### Scenario: A field the version does not name

- **WHEN** examples of version 3 with a field added at the envelope, in a launch, in a lease, in a place's status copy, in an end's delivery, in a confirm, in a hold, in a preview, in what the runner answered of a preview, in a preview's copy, in a release's copy, in a key share, in an observing hello and in an observed report's launch are checked
- **THEN** each is refused

#### Scenario: A time not in UTC to the second

- **WHEN** a lease expiring at `2026-11-01T12:00:00+02:00`, and one at `2026-11-01T10:00:00.5Z`, are checked against version 3
- **THEN** each is refused

#### Scenario: An integer past what JavaScript reads exactly

- **WHEN** a lease numbered 2^53, and one numbered 0, are checked against version 3
- **THEN** each is refused

### Requirement: The messages carry the interface's shapes

Version 3 SHALL carry the interface's `Id`, `Lease`, `Want`, `ToolVersion`, `Answer`, `Acted`, `Commanded` and `InboxCarried` as `boomdrive-control`'s serde types write them, every want and act sort among them, and its `Hello` and `Report` but for their copies. Version 2 SHALL carry the same but a hold; version 1, but a preview, a land, a confirm, a hold and `previews`. An outcome SHALL be `done` or `refused` (boomdrive-cloud record 0056; record 0052).

#### Scenario: The interface's values pass

- **WHEN** one value of each want, act and command sort version 3 names, a preview not done and one done, a land, a confirm and a hold among them, a lease with an expiry and one without, a launch with every field and one with none, each tool version, an acted, a commanded and an inbox carried are written by `boomdrive-control`'s types and checked against their definitions in version 3
- **THEN** each passes

#### Scenario: Every sort the interface names

- **WHEN** a value of every sort of `Want`, `ActSort` and `CommandSort`, each named by a match with no wildcard, is written by `boomdrive-control`'s types and checked against version 3's `want` and against version 2's
- **THEN** the sorts named are every sort the interface has, a hold among them; each value passes version 3's; and each but the hold's passes version 2's, which refuses the hold

#### Scenario: An act's outcome

- **WHEN** an acted, and a commanded, whose outcome is `launched` are checked against version 3
- **THEN** each is refused

### Requirement: A place's status is carried as its copy

A place's status SHALL be carried as the copy of `flume status --json`'s place, holding only the fields design Decision 7 names and record 0037 adds, each as Flume printed it. It SHALL hold no agent's last words, screen, printed output, notes, settings or mode; no task's text; no readiness; no group's title; no teardown's words; no end's report; no stall's move, answerer, stop-short or hold; no verdict's words; and no path but the place's own (core.27; record 0037).

#### Scenario: The copy's fields are the list

- **WHEN** the field paths of version 3's `place-status.schema.json` are read
- **THEN** they equal the list design Decision 7 gives with the paths record 0037 adds, which the test holds

#### Scenario: Text the cloud does not keep

- **WHEN** a status copy holding an agent's `last_words`, `screen` or `printed`, a task's `text`, `readiness`, a group's `title`, a teardown's `words`, an end's `report`, or a stall's `waiting` or `stop_short` is checked against version 3
- **THEN** each is refused

#### Scenario: Flume's answers as copies

- **WHEN** the `report` examples of version 3 are checked: one waiting on an answer, one on hold, a role's run delivered, a submission under test, a landing kept by a failed teardown, a tidied place, a live agent with its pane's address, a dropped place held for the runner, a done place held for the runner, an end handed over at a session's start, an end owed, expired and one confirmed, and an end by its delivery alone
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

No name of any version SHALL hold the word `grant`, `role`, `permission`, `token`, `credential`, `secret` or `password`, but version 4's forge token: its kinds `forge_token`, `forge_token_given` and `forge_token_refused`, each answer's `forge_token`, and `forge_token_given`'s `token`. A connection's account, and what it may do, SHALL come from its enrollment, never from a message (core.4; record 0053; boomdrive-cloud records 0047 and 0065).

#### Scenario: Every name read

- **WHEN** every property name, `properties` key, `$defs` key and `required` field of every file of each version is read
- **THEN** none holds one of those words but version 4's forge token's names, and each of those is found in version 4 alone

#### Scenario: A word caught in any name

- **WHEN** the names `roles`, `grant`, `Secret` and `credential_file` are read
- **THEN** each is caught as holding one of those words

### Requirement: The runner's types write and read only what the schema takes

`boomdrive_hosted::protocol::v4` SHALL hold a type for every message of version 4, using the interface's own types wherever version 4 carries a shape as the interface writes it, and the runner SHALL speak no other version. Every example of version 4 SHALL read into its direction's type and write back to an equal value that passes the schema, and every kind SHALL have an example (research row F50; records 0037, 0052 and 0053).

#### Scenario: Every example read and written back

- **WHEN** each example under version 4's `examples/from-runner/` and `examples/to-runner/` is read into its direction's `Message`, written back, and the written value checked
- **THEN** the written value equals the example's as JSON and passes its direction's schema

#### Scenario: Every kind has an example

- **WHEN** the kinds of version 4's examples are read
- **THEN** every kind of each direction is among them, `forge_token`, `forge_token_given` and `forge_token_refused` with them

#### Scenario: A token's Debug names its end alone

- **WHEN** a `forge_token_given` read from version 4's example is written with `{:?}`
- **THEN** the text names its repository and its end, and holds no character of its token

### Requirement: An enrollment is one enroll, answered by enrolled or enroll_refused

An `enroll` SHALL hold `key`, 32 bytes, `signature`, 64 bytes, each in base64url, and `machine`, the host's name. An `enrolled` SHALL hold `host`, the control plane's id for the host. An `enroll_refused` SHALL hold `enroll`, the `enroll` message's id, and `words`. None SHALL hold a token: the token is the request's bearer (core.4, core.31; record 0018).

#### Scenario: Enrollment's examples

- **WHEN** the examples of `enroll`, `enrolled` and `enroll_refused` are checked
- **THEN** each passes, and reads into its direction's type and writes back equal

#### Scenario: An enroll unsigned, or holding a token

- **WHEN** an `enroll` with no `signature`, one whose `key` is not 32 bytes, and one holding a `token` are checked
- **THEN** each is refused

### Requirement: The README names the paths and the signatures' contexts

The protocol's README SHALL describe version 4: `runner/enroll`, `runner/connect` and `runner/forge-token`, the kinds each takes, `boomdrive.v4`, the `BoomdriveHost` header, a `pulled`, a `report`, a forge token's ask and answers, how versions 3, 2 and 1 differ, and the bytes each signature covers, each context starting `boomdrive.v4`, with a copy of the worked examples the tests read (record 0049). The hosted implementation SHALL sign exactly those bytes (records 0018, 0052 and 0053).

#### Scenario: The README's worked contexts

- **WHEN** the hosted implementation makes the enrollment's, the connection's and the ask's contexts from the fixed key, token, origin, time, nonce and repository of the worked examples, `crates/boomdrive-hosted/tests/worked-bytes.json`
- **THEN** each equals the worked examples' bytes, and each starts with the bytes of `boomdrive.v4`

#### Scenario: The README of version 2

- **WHEN** the protocol's README is read
- **THEN** a section `Version 2` names `boomdrive.v2` and the releases that spoke it, and says it is version 3 but for its subprotocol, its contexts, its files' `$id`s and the hold, which it does not have

#### Scenario: The README of version 3

- **WHEN** the protocol's README is read
- **THEN** a section `Version 3` names `boomdrive.v3` and the releases that spoke it, and says it is version 4 but for its subprotocol, its contexts, its files' `$id`s and a forge token's ask and answers, which it does not have

#### Scenario: The README of version 4

- **WHEN** the protocol's README is read
- **THEN** its envelope names `protocol_version` 4; its paths name `runner/forge-token` and the subprotocol `boomdrive.v4`; its files name `forge-token.schema.json`; its messages name `forge_token`, `forge_token_given` and `forge_token_refused`, and say none goes on the socket; its signatures name the context `forge token` and its fields; and sections `Version 2` and `Version 1` stay

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

### Requirement: Version 3 names every end Flume 0.21.0 gives a place

Version 3's copy of a place's status SHALL name four forms of an end: a stage's, a submission's, a stall's, and a drop's or a done's, `{"of":"over",…}`. Each SHALL require `key`, `read_at`, `deliveries`, `pending_for`, `owed`, each reader with whether it expired and is known, and `confirmed`, each reader with when, and take `delivery` and `expires_at`. A hand-over SHALL be `by` `notice`, `wait`, `inbox` or `session_start` (core.27, core.42; records 0037 and 0052; weir record 0006).

#### Scenario: A drop's and a done's end

- **WHEN** a place's status copy holding an end `{"of":"over","over":"dropped",…}` with its key, delivery and readers, and one holding `"over":"done"`, are checked against version 3
- **THEN** each passes

#### Scenario: A hand-over at a session's start

- **WHEN** a status copy whose end was handed over `by` `session_start` is checked against version 3
- **THEN** it passes

#### Scenario: An end with no key

- **WHEN** a status copy whose stage's end has no `key`, and one whose end has no `owed`, are checked against version 3
- **THEN** each is refused

### Requirement: An end of a form version 3 does not name goes up as its delivery alone

Version 3 SHALL name an end's delivery alone, `end_delivery`, as a form of an end: its `of`, any string as Flume printed it, `key`, `read_at`, `owed`, `confirmed` and `pending_for`, each required, and `delivery` and `expires_at`, and nothing else. No end SHALL pass both `end_delivery` and a named form (core.27, core.42; records 0037 and 0052).

#### Scenario: An end of a form unnamed

- **WHEN** a status copy whose end is `{"of":"moved"}` with its key, delivery and readers alone is checked against version 3
- **THEN** it passes

#### Scenario: Content beside a delivery alone

- **WHEN** a status copy whose end is `{"of":"moved"}` with its key, delivery and readers, and its `deliveries`, or a `stage`, or `words`, is checked against version 3
- **THEN** it is refused

### Requirement: Version 3 carries a control plane's confirm

Version 3 SHALL name a confirm among the acts a `pulled` holds, as version 2 does: `{"confirm":{"recipient":…,"deliveries":[…]}}`, as `boomdrive-control`'s `ActSort::Confirm` writes it, a string recipient and an array of string keys, bounding neither, so that a confirm the runner refuses reaches the runner as an act (core.42, core.56; records 0037 and 0052).

#### Scenario: A confirm is an act of version 2

- **WHEN** a want whose act is a confirm of two keys, and one of no key, as `boomdrive-control`'s types write them, are checked against version 2's `want`, and a `pulled` holding the first against version 2's `to-runner.schema.json`
- **THEN** each passes

#### Scenario: A confirm is an act of version 3

- **WHEN** a want whose act is a confirm of two keys, and one of no key, as `boomdrive-control`'s types write them, are checked against version 3's `want`, and a `pulled` holding the first against version 3's `to-runner.schema.json`
- **THEN** each passes

### Requirement: Version 1 stays as a release spoke it

Every file under `protocol/v1/`, its schemas and its examples, SHALL stay as release `v0.7.3` holds it, and no file SHALL be added there. Every example of version 1 SHALL still pass its direction's envelope of version 1. `boomdrive_hosted::protocol::v1` SHALL hold version 1's files and no type (core.42; record 0037).

#### Scenario: Version 1 as released

- **WHEN** each file under `protocol/v1/` is read and its SHA-256 taken
- **THEN** the files are exactly those the test lists, each with the digest the test holds for it

#### Scenario: Version 1's examples still pass

- **WHEN** each example under version 1's `examples/` is checked against its direction's envelope of version 1
- **THEN** each passes

### Requirement: Version 3 carries a land

Version 3 SHALL name a land among the acts a `pulled` holds, as version 2 does, written as the word `land`, as `boomdrive-control`'s `ActSort::Land` writes it, holding nothing but its act's launch and lease. The retry naming the submission SHALL stay among version 3's acts (core.42, core.53; record 0037 as amended on 2026-11-09; record 0052).

#### Scenario: A land is an act of version 2

- **WHEN** a want whose act is a land, as `boomdrive-control`'s types write it, is checked against version 2's `want`, and a `pulled` holding it against version 2's `to-runner.schema.json`
- **THEN** each passes, the act's `what` being `"land"`

#### Scenario: A land is an act of version 3

- **WHEN** a want whose act is a land, as `boomdrive-control`'s types write it, is checked against version 3's `want`, and a `pulled` holding it against version 3's `to-runner.schema.json`
- **THEN** each passes, the act's `what` being `"land"`

### Requirement: Version 3 carries a preview and its done

Version 3 SHALL name a preview among the wants a `pulled` holds, as version 2 does: `{"preview":{"id","lease","repository","branch","revision","done"}}`, as `boomdrive-control`'s `Want::Preview` writes it, each field required. Its branch and its revision SHALL be any string, so that a preview sluice refuses reaches the runner as a preview, and not the whole `pulled` as a message (core.42, core.55; record 0037 as amended on 2026-11-09; record 0052).

#### Scenario: A preview is a want of version 2

- **WHEN** a preview not done, one done, and one whose revision is `R1`, as `boomdrive-control`'s types write them, are checked against version 2's `want`, and a `pulled` holding a preview and its done against version 2's `to-runner.schema.json`
- **THEN** each passes

#### Scenario: A preview is a want of version 3

- **WHEN** a preview not done, one done, and one whose revision is `R1`, as `boomdrive-control`'s types write them, are checked against version 3's `want`, and a `pulled` holding a preview and its done against version 3's `to-runner.schema.json`
- **THEN** each passes

### Requirement: Version 3's report holds what the runner answered of each preview

Every report of version 3 SHALL hold `previews`, by id, each preview the runner answered with its `lease`, `repository`, `revision`, `done`, `outcome` (`held`, `done` or `refused`), `copy`, `words`, `status` and `at`, each required: `copy` the copy of sluice's answer, a preview's or a release's, or `null`; `status` the copy of sluice's listing with its time and whether it was listed, or `null` (core.27, core.42, core.55; records 0037 and 0052).

#### Scenario: sluice's answers of a preview as copies

- **WHEN** the `report` examples of version 3 are checked: a preview brought up, a preview held, a preview put away, and a preview sluice refused
- **THEN** each passes

#### Scenario: A report with no previews

- **WHEN** a `report` of version 3 with no `previews` is checked
- **THEN** it is refused

#### Scenario: A preview's outcome

- **WHEN** what the runner answered of a preview, its outcome `launched`, is checked against version 3
- **THEN** it is refused

### Requirement: A preview's copy holds no output of its command

Version 3's `preview.schema.json` SHALL hold the copy of a preview, its `name`, required, `branch`, `revision`, `state`, `made` and `addresses`, a map of strings, and the copy of a release, its `preview`, required, and `released`, each as sluice printed it. Neither SHALL hold a preview's `checkout`, `names`, `ports`, `exit`, `output` or `release_failed` (core.11, core.27, core.55; record 0037 as amended on 2026-11-09).

#### Scenario: The copies' fields are the lists

- **WHEN** the field paths of `preview.schema.json`'s `preview` and `release` are read
- **THEN** the first's are `name`, `branch`, `revision`, `state`, `made` and `addresses`, and the second's `preview` and `released`, which the tests hold

#### Scenario: A preview's output

- **WHEN** a report whose preview's copy, or whose preview's status copy, holds `output`, `checkout`, `names`, `ports`, `exit` or `release_failed` is checked against version 3
- **THEN** each is refused

### Requirement: Version 2 stays as a release spoke it

Every file under `protocol/v2/`, its schemas and its examples, SHALL stay as release `v0.9.0` holds it, and no file SHALL be added there. Every release from `v0.8.0`, the first to speak version 2, to the last before version 3 holds the same files. Every example of version 2 SHALL still pass its direction's envelope of version 2. `boomdrive_hosted::protocol::v2` SHALL hold version 2's files and no type (core.42; records 0037 and 0052).

#### Scenario: Version 2 as released

- **WHEN** each file under `protocol/v2/` is read and its SHA-256 taken
- **THEN** the files are exactly those the test lists, each with the digest the test holds for it, as `git show v0.9.0:<path>` gives it

#### Scenario: A file of version 2 changed or added

- **WHEN** a file under `protocol/v2/` differs from `v0.9.0`'s, or a file is added there
- **THEN** `boomdrive-hosted`'s test `version_2_is_as_released` fails, naming the file

#### Scenario: Version 2's examples still pass

- **WHEN** each example under version 2's `examples/` is checked against its direction's envelope of version 2
- **THEN** each passes

### Requirement: Version 1 and version 2 carry no hold

Version 1 and version 2 name no hold, and a release has spoken each, so neither SHALL change to carry one. A want whose act is a hold SHALL fail each one's check, and a hold SHALL reach the runner from a hosted control plane only in version 3 or a later version (core.42, core.59; record 0052).

#### Scenario: A hold is no act of version 1 or 2

- **WHEN** a want whose act is a hold, as `boomdrive-control`'s types write it, is checked against version 1's `want` and against version 2's
- **THEN** each refuses it

### Requirement: Version 3 is version 2 whole, but for a hold and an observing connection

Version 3's files SHALL be version 2's but for these alone: each `$id` under `boomdrivedev/boomdrive`'s `…/protocol/v3/`, `protocol_version` 3, a hold among `act_sort`'s branches, the observing hello and the observed report beside the hello and the report, and their descriptions. Its examples SHALL be version 2's, each naming `protocol_version` 3 and `enroll.json` signed over version 3's context, a `pulled` holding a hold, and three of the observing connection (core.42; records 0048 and 0052).

#### Scenario: Version 3 is version 2 but for its hold

- **WHEN** each schema file of version 3 is read with every `description` set aside, its `$id` read under version 2's address, its `protocol_version` read as 2, the hold's branch taken out of `act_sort`, `observing_hello`, `observed_report` and `observed_launch` taken out of its `$defs`, and the `hello` and `report` kinds each read as the reference to its one working form
- **THEN** it equals version 2's file of the same name with every `description` set aside; the hold's branch is `ActSort::Hold` as the interface writes it; `observing_hello` is `hello` with `observing` in place of `capacity` and `leases`; and `observed_launch` is `launched` without `lease`

#### Scenario: Version 3's examples are version 2's

- **WHEN** version 3's examples are read
- **THEN** their names are version 2's, `to-runner/pulled-with-a-hold.json`, `from-runner/hello-observing.json`, `from-runner/report-observed.json` and `to-runner/pulled-answering-an-observing-hello.json`, and each of version 2's names, read with `protocol_version` as 2 and `enroll.json`'s signature set aside, equals version 2's

### Requirement: Version 3 carries a control plane's hold

Version 3 SHALL name a hold among the acts a `pulled` holds, `{"hold":{"words":…,"bound_interval":…}}`, as `boomdrive-control`'s `ActSort::Hold` writes it: its words any string, and its bound an integer from 0 that JavaScript reads exactly, bounded no further, so that a hold the runner refuses reaches the runner as an act (core.42, core.59; record 0052).

#### Scenario: A hold is an act of version 3

- **WHEN** a want whose act is a hold of words and a bound of 28800, one whose words are empty, and one whose bound is 0, as `boomdrive-control`'s types write them, are checked against version 3's `want`, and version 3's `pulled-with-a-hold.json` against its `to-runner.schema.json`
- **THEN** each passes

#### Scenario: A hold's bound past what JavaScript reads exactly

- **WHEN** a want whose act is a hold of bound 2^53 is checked against version 3's `want`
- **THEN** it is refused

### Requirement: Version 4 is version 3 whole, but for a forge token's ask and answers

Version 4's files SHALL be version 3's but for these alone: each `$id` under `…/protocol/v4/`, `protocol_version` 4, a twelfth file, `forge-token.schema.json`, the three kinds it holds among `from-runner.schema.json`'s and `to-runner.schema.json`'s bodies, and their descriptions. Its examples SHALL be version 3's, each naming `protocol_version` 4 and `enroll.json` signed over version 4's context, and four of a forge token (core.42; record 0053).

#### Scenario: Version 4 is version 3 but for a forge token

- **WHEN** each schema file of version 4 but `forge-token.schema.json` is read with every `description` set aside, its `$id` read under version 3's address, its `protocol_version` read as 3, and the forge token's kinds taken out of the bodies
- **THEN** it equals version 3's file of the same name with every `description` set aside, and the kinds taken out are exactly `forge_token` from the runner's and `forge_token_given` and `forge_token_refused` from the control plane's

#### Scenario: Version 4's examples are version 3's

- **WHEN** version 4's examples are read
- **THEN** their names are version 3's, `from-runner/forge_token.json`, `to-runner/forge_token_given.json`, `to-runner/forge_token_refused-not-installed.json` and `to-runner/forge_token_refused-a-repository-no-drive-serves.json`, and each of version 3's names, read with `protocol_version` as 3 and `enroll.json`'s signature set aside, equals version 3's

### Requirement: Version 4 carries a forge token's ask and its answers

Version 4 SHALL name the runner's ask, `forge_token`, holding `repository`, an address; the answer `forge_token_given`, holding `forge_token`, the ask's message id, `repository`, `token`, a string of one character or more, and `expires_at`, a time; and `forge_token_refused`, holding `forge_token`, `repository`, `not_installed`, a boolean, and `words`. Each SHALL be closed, every field required (core.33, core.42; record 0053).

#### Scenario: A forge token's examples

- **WHEN** version 4's `forge_token.json`, `forge_token_given.json` and both `forge_token_refused` examples are checked against their direction's envelope of version 4
- **THEN** each passes

#### Scenario: A forge token's ask or answer that is not whole

- **WHEN** version 4's ask with a `path` added, one whose `repository` is `flume`, its `forge_token_given` with no `expires_at`, one whose `expires_at` is `2026-11-18T12:00:00+02:00`, one whose `token` is empty, and its `forge_token_refused` with no `not_installed` are checked against their direction's envelope
- **THEN** each is refused

### Requirement: Version 3 stays as a release spoke it

Every file under `protocol/v3/`, its schemas and its examples, SHALL stay as release `v0.18.0` holds it, and no file SHALL be added there. Every release from `v0.18.0`, the first to speak version 3, to the last before version 4 holds the same files. Every example of version 3 SHALL still pass its direction's envelope of version 3. `boomdrive_hosted::protocol::v3` SHALL hold version 3's files and no type (core.42; records 0052 and 0053).

#### Scenario: Version 3 as released

- **WHEN** each file under `protocol/v3/` is read and its SHA-256 taken
- **THEN** the files are exactly those the test lists, each with the digest the test holds for it, as `git show v0.18.0:<path>` gives it

#### Scenario: A file of version 3 changed or added

- **WHEN** a file under `protocol/v3/` differs from `v0.18.0`'s, or a file is added there
- **THEN** `boomdrive-hosted`'s test `version_3_is_as_released` fails, naming the file

#### Scenario: Version 3's examples still pass

- **WHEN** each example under version 3's `examples/` is checked against its direction's envelope of version 3
- **THEN** each passes

### Requirement: Versions 1 to 3 carry no forge token

Versions 1, 2 and 3 name no forge token, and a release has spoken each, so none SHALL change to carry one. A forge token's ask or answer SHALL fail each one's check, and a cloud host's token SHALL reach the runner only in version 4 or a later version (core.33, core.42; record 0053).

#### Scenario: A forge token is no message of version 1, 2 or 3

- **WHEN** version 4's `forge_token.json`, `forge_token_given.json` and `forge_token_refused-not-installed.json`, each naming `protocol_version` 3, are checked against version 3's envelope of their direction, each naming 2 against version 2's, and each naming 1 against version 1's
- **THEN** each is refused

### Requirement: Version 5 is version 4 whole, but for the observing forms and a report's unleased places

Version 5's files SHALL be version 4's but for these alone: each `$id` under `…/protocol/v5/`, `protocol_version` 5, no observing hello or observed report, each kind taking its one form, and the report's `unleased`. Its examples SHALL be version 4's but the observing connection's three, each report naming `unleased`, and `report-a-place-started-elsewhere.json` (core.42; record 0054).

#### Scenario: Version 5 is version 4 but for its unleased places

- **WHEN** each schema file of version 5 is read with every `description` set aside, its `$id` read under version 4's address, its `protocol_version` read as 4, `unleased` and `unleased_place` taken out of its report, and version 4's observing forms taken out of version 4's files
- **THEN** it equals version 4's file of the same name, so read, with every `description` set aside

#### Scenario: Version 5's examples are version 4's

- **WHEN** version 5's examples are read
- **THEN** their names are version 4's but the observing connection's three, and `from-runner/report-a-place-started-elsewhere.json`

### Requirement: Version 5's report names each place the partition did not lease, with no lease

Version 5's report SHALL require `unleased`, a map from a place's path, as Flume printed it, to `{repository, status}`: `repository` an address, and `status` a carried status as a launch's is, or null. It SHALL name no lease, answer, act or entry. No observing hello or observed report SHALL pass version 5 (core.10, core.42; record 0054).

#### Scenario: A place started elsewhere

- **WHEN** version 5's `report-a-place-started-elsewhere.json` is checked against `from-runner.schema.json` of version 5
- **THEN** it passes

#### Scenario: An unleased place that names a lease

- **WHEN** that example with a `lease` added to its unleased place, and one with no `unleased`, are checked against version 5
- **THEN** each is refused

#### Scenario: No observing connection in version 5

- **WHEN** version 4's `hello-observing.json` and `report-observed.json`, each naming `protocol_version` 5, are checked against version 5's envelope
- **THEN** each is refused

### Requirement: Version 4 stays as a release spoke it

Every file under `protocol/v4/`, its schemas and its examples, SHALL stay as release `v0.21.0` holds it, and no file SHALL be added there. Every example of version 4 SHALL still pass its direction's envelope of version 4. `boomdrive_hosted::protocol::v4` SHALL hold version 4's files and no type (core.42; records 0053 and 0054).

#### Scenario: Version 4 as released

- **WHEN** each file under `protocol/v4/` is read and its SHA-256 taken
- **THEN** the files are exactly those the test lists, each with the digest the test holds for it, as `git show v0.21.0:<path>` gives it

#### Scenario: Version 4's examples still pass

- **WHEN** each example under version 4's `examples/` is checked against its direction's envelope of version 4
- **THEN** each passes

### Requirement: The runner names the versions it speaks in one file of data

`crates/boomdrive-hosted/protocol/spoken.json` SHALL name each version directory under `protocol/` once, lowest first, as `{"version": N, "spoken": bool}` under `versions`, and nothing else. It SHALL mark spoken exactly one version, which the runner's code reads from the file, and checks, sends and offers. The crate's test SHALL hold the file to the directories and to the code (core.42, principle 16).

#### Scenario: The file names each version directory

- **WHEN** `spoken.json` is read and `protocol/` listed
- **THEN** it holds `versions` alone, naming versions 1 to 7 once each, lowest first, and versions 1 to 6 are not spoken

#### Scenario: The runner speaks what the file marks spoken

- **WHEN** the crate's test reads `spoken.json`
- **THEN** it marks one version spoken, `protocol::spoken()`, which is version 5, 6 or 7, and the upgrade offers its subprotocol

### Requirement: The gate refuses a runner that offers a version the cloud's main does not speak

The gate's check `boomdrive-hosted:cloud-protocol` SHALL read `workers/control-plane/runner-protocol/pin.json` from boomdrive-cloud's `main` as the forge holds it, never from a cache, and SHALL fail where `spoken.json` marks spoken none, or a version the pin does not mark spoken, naming both lists. Where it cannot read the pin, it SHALL fail naming what it could not read (Boom: `the-cloud-leads-the-runners-protocol`; core.42).

#### Scenario: A runner whose versions the cloud speaks passes

- **WHEN** the check runs against a cloud whose `main` pins versions 4 and 5 spoken, the runner marking 5 alone, or 4 and 5, spoken
- **THEN** it passes, saying the cloud speaks each version the runner offers

#### Scenario: A runner that offers a version the cloud does not speak is refused

- **WHEN** the runner marks version 5 spoken and the cloud's `main` pins 3 and 4 spoken, whether or not the runner also marks 4, or the runner marks 4 alone and the cloud's `main` pins 5 alone, or the runner marks none
- **THEN** the check fails, naming the versions each side speaks and each version the runner offers that the cloud does not speak

#### Scenario: A pin that cannot be read is refused

- **WHEN** the cloud's repository cannot be fetched, its `main` holds no pin, or its pin is not JSON or does not list versions
- **THEN** the check fails, naming `workers/control-plane/runner-protocol/pin.json` and what it could not read

### Requirement: Version 6 is version 5 whole, but for the newest release

Version 6's files SHALL be version 5's but for three things. Each `$id` is under `…/protocol/v6/`. Each message names `protocol_version` 6. `to-runner.schema.json` holds `newest_release`, a branch of the body and its definition. Its examples SHALL be version 5's, each naming `protocol_version` 6, and `to-runner/newest_release.json` (core.42, Boom: `the-cloud-leads-the-runners-protocol`).

#### Scenario: Version 6 is version 5 but for the newest release

- **WHEN** each schema file of version 6 is read with every `description` set aside and its `$id` under version 5's address
- **AND** its `protocol_version` is read as 5, and `newest_release` is taken out of `to-runner.schema.json`
- **THEN** it equals version 5's file of the same name, with every `description` set aside

#### Scenario: Version 6's examples are version 5's

- **WHEN** version 6's examples are read
- **THEN** their names are version 5's and `to-runner/newest_release.json`, and each of version 5's, its `protocol_version` read as 5, equals version 5's

### Requirement: Version 6 carries the newest release

Version 6's control plane SHALL send `newest_release`, `{"version": …}`, closed. Its `version` is three numbers with no leading zero, as a release's tag `v<version>` names it. The control plane SHALL send it on the socket after its `pulled` answers the hello, where the hello's `runner_version` is older. It MAY send it again when a newer release is made. No other version SHALL take it (core.42, Boom: `the-cloud-leads-the-runners-protocol`).

#### Scenario: Version 6's examples read

- **WHEN** each example of version 6 is checked against its direction's envelope of version 6 and read into the runner's types
- **THEN** each passes and writes back equal, and `newest_release.json` reads as a newest release of `0.32.0`

#### Scenario: A newest release that is no release's

- **WHEN** a `newest_release` naming a field it does not name, no version, `v0.32.0`, `0.32`, `0.32.0-rc.1` or `00.32.0` is checked against version 6, and `newest_release.json` naming `protocol_version` 5 against version 5
- **THEN** each is refused

### Requirement: The runner speaks the one version spoken.json marks spoken

`protocol::spoken()` SHALL be the one version `spoken.json` marks spoken. Every message the runner sends or reads, on a socket or by a request, SHALL be of that version. Each upgrade SHALL ask for its subprotocol alone. The crate's tests SHALL pass with either version marked spoken, so a change to the mark alone moves the runner (core.42, Boom: `the-cloud-leads-the-runners-protocol`).

#### Scenario: The upgrade's subprotocol

- **WHEN** a partition's connection is dialed
- **THEN** its upgrade asks for the subprotocol of the version spoken alone, `boomdrive.v7` while version 7 is marked spoken

### Requirement: Version 7 is version 6 whole, but for the repositories' statuses

Version 7's files SHALL be version 6's but for three things. Each `$id` is under `…/protocol/v7/`. Each message names `protocol_version` 7. The hello requires `repository_statuses`, with its definitions. Its examples SHALL be version 6's, each naming `protocol_version` 7 and each hello naming its statuses, and one more hello (core.42).

#### Scenario: Version 7 is version 6 but for the repositories' statuses

- **WHEN** each schema file of version 7 is read with every `description` set aside and its `$id` under version 6's address
- **AND** its `protocol_version` is read as 6, and `repository_statuses` and its definitions `repository_status`, `repository_status_copy`, `binding_held`, `binding_read`, `merge_queue_bound` and `kind_lands` are taken out of `hello.schema.json`
- **THEN** it equals version 6's file of the same name, with every `description` set aside

#### Scenario: Version 7's examples are version 6's

- **WHEN** version 7's examples are read
- **THEN** their names are version 6's and `from-runner/hello-with-no-merge-queue-ready.json`, and each of version 6's, its `protocol_version` read as 6 and its hello's `repository_statuses` taken out, equals version 6's

### Requirement: Version 7's hello names each repository's status

Version 7's hello SHALL hold `repository_statuses`, a map that names each address of `repositories` once. Each value SHALL be `{"printed": …}`, the copy of Flume's item of `status.repositories` without its `repository`, or `{"missing": "<words>"}`. The copy SHALL be closed: `binding`, `merge_queue` and `kinds`, each as Flume's report names it. No version before 7 SHALL take the field (core.27, core.42).

#### Scenario: Version 7's examples read

- **WHEN** each example of version 7 is checked against its direction's envelope of version 7 and read into the runner's types
- **THEN** each passes and writes back equal, and each hello names the status of each of its repositories once

#### Scenario: A status of another form

- **WHEN** a hello of version 7 names no `repository_statuses`, or a status that is neither `printed` nor `missing`
- **AND** a copy names the clone's folder, or a field that Flume's report does not name
- **AND** a `from` is not `origin` or `local`, a `ready` is no boolean, or a key is no address
- **AND** version 7's `hello.json` is checked against version 6
- **THEN** each is refused

### Requirement: The runner's code reads and writes versions 5, 6 and 7

`boomdrive_hosted::protocol::v7` SHALL hold the runner's types, which read and write each message of versions 5, 6 and 7. `protocol::V5`, `protocol::V6` and `protocol::V7` SHALL each hold its version's number, subprotocol, address and files (core.42).

#### Scenario: Each version the code speaks is its own files

- **WHEN** `protocol::V5`, `protocol::V6` and `protocol::V7` are read
- **THEN** each holds its module's number, subprotocol, address and files
