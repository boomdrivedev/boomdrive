# Design

## Context

See proposal.md for why. This change writes the runner's half of a contract both repositories test against. It builds nothing that speaks it.

What stands:

- **This repository**, at `8ef1992`, has the interface `the-runner-carries-crews-work-into-flume` built (#2, archived in `openspec/changes/archive/2026-11-01-the-runner-carries-crews-work-into-flume`).
  - `crates/flywheel-control` gives `ControlPlane`, with `pull(hello, report)` and `report(report)`, and the types that pass over it: `Hello`, `ToolVersion`, `Capacity`, `HostPart`, `Pulled`, `Reported`, `Want`, `Offer`, `Lease`, `Launch`, `Act`, `ActSort`, `Command`, `CommandSort`, `Inbox`, `Terminal` with its `Link`, `Report`, `Launched`, `Answer`, `Outcome`, `Carried`, `Acted`, `Commanded`, `InboxCarried`, `InboxFile`, `Id` and `At`.
  - Every value but a terminal's link round-trips as JSON through serde (`tests/json.rs`). Enums are externally tagged in snake case, `Id` is a bare string, `At` is RFC 3339 in UTC to the second, and an absent option is written `null`.
  - `Carried::text` holds a tool's answer as a text. `HostPart`'s four fields each hold the text of a Flume field as Flume printed it.
  - `crates/flywheel-crew` is the one implementation. `crates/flywheel/src/planes.rs` lists it, and refuses `type = "hosted"` as a word the binary does not carry (that change's Decision 20).
  - Rules core.1 to core.41 bind. Record 0004 takes the herdr terminal relay out until Flume relays one, and `_open.md` names the code that still stands against core.29 for it.
- **agentplot/flywheel-cloud**, at `6b63a54`, read 2026-11-02.
  - **Records.**
    - 0016: a launch's terminal is relayed through the Host object, encrypted end to end, and its key exchange is in the protocol from its first version.
    - 0022: every runner message names its version, is checked before any method sees it, and is refused whole if it fails, and every offer, lease, command and event carries an id.
    - 0047: a host is enrolled to its Account alone.
    - 0049: of a place's status the cloud keeps only ids, states, times, counts, what it made, its pane's address and the words a person reads to follow or answer it, and the protocol's schema has no field for the rest.
    - 0056, superseding 0046: the schema lives in this repository, one set of files per version, beside the interface; the cloud pins it by revision.
    - 0061: the page connects to the Flywheel object by the Agents SDK's `useAgent`, and draws a terminal with xterm.js.
    - 0065: grants are WorkOS FGA's, and runners are not its subjects.
  - **Rules.** core.13, the Host object; core.18, a terminal relayed and never stored; core.23 and core.43, a member's certified key and the log of keys; core.36 to core.39, the runner contract; core.106, the schema's home; core.108, what of a place's status reaches it.
  - **Its roadmap**, `design/roadmap.md`: item 1.16 is this change, under ruling D5. Item 1.6 pins the schema and builds the Worker's check, and item 1.15, in this repository, builds the hosted implementation. Both wait on this change.
  - **Its research**, `design/research/awesome-lists-disposition.md`: rows S6 (the terminal encrypted end to end), F54 (frames addressed by byte offset), F50 (schema tools), and F8 (the Worker's validator and a compatibility check), which is 1.6's.
- **The tools' answers** the copies are cut from:
  - Flume 0.4.1's `status --json`, as `crates/flywheel/tests/fixtures/flume-0.4.1/` holds it. Flume's place-status spec makes its JSON a form that only gains fields.
  - sluice 0.61.1's `status --json`, as `crates/flywheel/tests/fixtures/sluice-0.61.1/` holds it, and the fields of it the runner already carries (`crates/flywheel/src/sluice.rs`, that change's Decision 13).
  - Where a fixture shows a field only empty, its shape is read from Flume's own types at the tag it was read from (agentplot/flume `v0.4.1`, `crates/flume-seams/src/held.rs` and `merge_queue.rs`).

There are no area pages, so `docs/architecture/core.md` binds every path this change touches.

## What wave 0 settles for this change

The cloud roadmap's wave 0 was made so that wave 1 builds from it. Each item bears on this change as follows.

- **0.5, `the-runner-carries-crews-work-into-flume`**, landed here as #2.
  - Its `ControlPlane` is the source of the messages' shapes. Version 1 carries its `Hello`, its `Want`s and its `Report` as its serde types write them, field for field.
  - The wire departs from them in three places only, each forced by a rule (Decision 6):
    - a place's status and sluice's entry go up as their copies, not as texts;
    - the host's capacity goes up as values, its streams without their queues;
    - a terminal's link is a set of sealed frames.
  - Its Decision 20 named the hosted implementation's crate `flywheel-hosted`, and this change makes that crate to hold the protocol (Decision 1).
- **The first run** (flywheel-cloud `design/the-first-run-from-an-empty-account.md` and `.html`).
  - **`--keep`.** The install command carries the repositories to keep, so the person who runs it writes the runner's configuration on the host, and the cloud sends nothing to the machine. Version 1 has no message that writes the configuration or names a repository for the host to keep. That agrees with core.16, and leaves `setup`'s managed level (core.17, record 0009) to the change that builds it (Decision 11).
  - **FR3, a new host asks to be admitted.** The host's first hello carries its tools, the repositories it keeps and its streams' room (Decisions 5 and 8). The token it used and who took it are the cloud's own, from enrolment. The host takes no lease before it is admitted, and the protocol needs nothing for that: the control plane offers it nothing. The machine FR3 names is not in the interface's hello. It comes with enrolment, which 1.15 builds and adds to version 1 (Decision 11).
  - **FR7, the states that reach the protocol.**
    - *A tool missing on the host*: the hello's `tool_versions` names it as `missing`, with the words, and `capacity` is `missing` where Flume's status cannot be read (Decision 5).
    - *The host gone quiet*: no message says it. It is the socket's silence under the WebSocket protocol's own pings, which core.21 keeps out of the application's messages. The Host object tells the Flywheel (cloud core.13).
    - *The token lapsed*: that is enrolment's answer, and 1.15's to add (Decision 11).
- **Records 0047 and 0065, and `design/research/authorization.md`.** A runner's host token is its Account's, and runners are not FGA subjects. So no message of version 1 names an account, a member's grant, a role or a permission. A connection is its enrolment's, and what it may do comes from the cloud's definition, never from a message. The one identity a message carries is a signature's: the browser's key share is signed by a key certified to a member (Decision 10). The cloud checks the member's permission to open a terminal (its core.115, record 0055), and sends no grant down. A test holds version 1 to naming none (Decision 14).
- **Record 0061.** The page's socket is the Agents SDK's `useAgent` to the Flywheel object, apart from the runner's protocol. So S6's key exchange and F54's offsets are the runner's frames, and nothing of the page's socket is in this schema. The frames pass the cloud sealed. A frame's plaintext is the terminal's bytes, which the page's xterm.js writes as they come (Decision 10).

Where a wave 0 record changes what the roadmap's entry for 1.16 says, the record wins:

- **The copy of a place's status is in version 1.** The entry names the messages and the terminal's frames. Record 0049, of wave 0's item 0.4, gives the schema no field for the text the cloud does not keep. Record 0014 had left the copy's fields to the change that builds the hosted implementation. Since record 0056 the schema is written here, so this change writes the copy, field by field, as core.108 keeps it (Decision 7). 1.15 cuts each status to it.
- **Scrollback comes from the agent's pane through Flume, not from herdr's.** The entry, F54, has scrollback come from the herdr pane and never the cloud. Record 0061 makes the frames the runner's. This repository's core.29 and record 0004 have the runner reach a terminal only through Flume's command line, naming no multiplexer. So a reconnecting browser's redraw comes from the agent's pane through Flume's relay, never the cloud's and never kept by the runner (Decision 10).
- **No message carries a grant.** The entry says nothing of grants. Records 0047 and 0065 settle that none goes over the runner's socket.

## Goals / Non-Goals

**Goals:**

- Version 1 of every message between a runner and a hosted control plane, written as JSON Schema 2020-12 in this repository, which the cloud can pin by revision (cloud record 0056, core.106).
- Every message names its version and carries an id of its own (cloud core.36, core.38; core.21).
- Every message is the interface's shape, but where a rule says the wire must differ (core.27, core.26, core.30), and holds nothing its version does not name (core.42).
- A place's status, sluice's entry and the host's capacity go up as copies that hold only what the cloud keeps (core.27; cloud core.108).
- A terminal's key exchange and its frames are in version 1: shares signed, frames sealed, each with its stream's offset (S6, F54; core.30; cloud record 0016).
- The runner's own types for every message, tested against the schema (F50).

**Non-Goals:**

- **The hosted implementation** (1.15): the socket, enrolment, the key, the certificates, the cutting of a status to its copy, the runtime check of every message, and the opener in `planes.rs`.
- **The cloud's check** (1.6): its pin, its validators compiled at build time, and jsoncompat holding the version before (F8).
- **The browser's half of a terminal** (5.5), and the suite both ends use (Decision 10).
- **Messages no change has designed** (Decision 11): health, forge tokens, enrolment, the runner's own commands and a stray place's drop.
- **A change to the interface.** `flywheel-control` and crew's implementation stay as they are.

## What this change rests on

1. **`the-runner-carries-crews-work-into-flume`, landed.** It gives `flywheel-control`'s types and their JSON (Context), and the fixtures of Flume 0.4.1 and sluice 0.61.1. Where a type changes before this change is built, the coder carries the change into the schema and names it.
2. **The cloud's records and rules above**, accepted by the user. This change cites them by id and copies no text of them (core.41).
3. **Flume's JSON form**, which only gains fields within a version (Flume's place-status spec). The copies name fields Flume 0.4.1 prints, so a later Flume's added fields never reach them (Decision 7).

## Decisions

### 1. Version 1 lives in the hosted implementation's crate

The protocol is `crates/flywheel-hosted/protocol/`, one directory per version, version 1 at `v1/`. `crates/flywheel-hosted` is the hosted implementation's crate, which the archived change's Decision 20 named. It is made now to hold the protocol:

- its schema files;
- the protocol's README;
- the runner's types for each message (Decision 13);
- the tests that hold the types to the schema (Decision 14).

It implements no `ControlPlane` yet. `planes.rs` does not list it, and `type = "hosted"` stays refused until 1.15 builds the implementation in this crate.

core.2 keeps the hosted protocol out of every crate but an implementation's. Record 0056 puts the schema beside the interface, and this reads "beside" as a sibling crate of the same workspace, which depends on the interface whose types the messages carry. The cloud pins `crates/flywheel-hosted/protocol/v1/` by revision.

Alternatives:

- **In `crates/flywheel-control`.** Not taken: the interface's crate would then name the hosted protocol (core.2), and crew's implementation would build it.
- **A crate of the protocol alone.** Not taken: it would be code outside an implementation naming the hosted protocol (core.2).
- **A `protocol/` directory at the tree's root.** Not taken: no crate's checks would own the files (core.7), and the types that are tested against them would sit in another tree.

### 2. A set of files, draft 2020-12, with nothing fetched

Version 1 is these files, each a JSON Schema of draft 2020-12, named for what it holds (core.12):

| file | what it holds |
|---|---|
| `from-runner.schema.json` | the envelope of every message a runner sends |
| `to-runner.schema.json` | the envelope of every message a control plane sends |
| `shared.schema.json` | `$defs` both directions use: an id, a message's id, a time, an integer signed and unsigned, a lease, a lease's number, a repository's address, a commit, a digest, bytes in base64url, and `message_refused` |
| `hello.schema.json` | the hello, a tool's version, and the copy of Flume's host part (Decision 8) |
| `wants.schema.json` | a want: an offer with its launch, an act, a command, an inbox; and `pulled` |
| `report.schema.json` | the runner's report as version 1 carries it, and `reported` |
| `place-status.schema.json` | the copy of a Flume place's status (Decision 7) |
| `submission-entry.schema.json` | the copy of a sluice entry (Decision 9) |
| `terminal.schema.json` | a key share and a terminal's frames (Decision 10) |

- **Each file's `$id`** is its address among the repository's raw files at `main`, under the version's directory: `https://raw.githubusercontent.com/agentplot/flywheel-runner/main/crates/flywheel-hosted/protocol/v1/<file>`.
- **Every `$ref` is relative**, to a file of the same version and a `$defs` entry in it. No check fetches a file: each validator, the runner's tests and the Worker alike, is handed the version's files and nothing else. A reference outside them fails to resolve.
- **A pattern** uses only what ECMA-262 and Rust's `regex` read alike: anchors, classes, counts and alternation. The Worker's validator and the runner's tests then refuse the same strings.
- **`examples/from-runner/` and `examples/to-runner/`** hold one example message or more of each kind, each a whole message, with neutral names (core.41). They are the protocol's examples for a reader, and the tests' cases (Decision 14).
- **`README.md`**, in `protocol/`, says what a reader of the contract needs:
  - the versions and when one is fixed (Decision 12);
  - the messages and their order (Decision 5);
  - the copies (Decisions 7 to 9);
  - the terminal's frames and the context its signatures are made over (Decision 10);
  - the WebSocket subprotocol name, `flywheel-runner.v1`, which a connection asks for at its upgrade.

Alternatives:

- **One file.** Not taken: the copies alone run to many definitions, and a reviewer reads a file per subject more easily.
- **AsyncAPI or OpenAPI.** Not taken: the Worker's validator (F8) reads JSON Schema, and neither adds a check the schema lacks.
- **Protocol Buffers with buf** (F50). Not taken: record 0056 fixes JSON Schema, and the messages are JSON.

### 3. The envelope: a version, an id of its own, one body, two directions

Every message is a JSON object with three fields:

```json
{"protocol_version": 1, "id": "<the message's id>", "body": {"<kind>": { … }}}
```

- **`protocol_version`** is `1` in every message of version 1 (cloud core.36). It is the protocol's number, apart from the hello's `interface_version`, which is the interface's. A connection asks for the subprotocol `flywheel-runner.v1` at its upgrade, so a control plane that does not accept the version refuses the connection before any message (1.6, 1.15).
- **`id`** is the message's own, given by its sender and unique among what that sender sends on its enrolment, across reconnects (cloud core.38). A resend keeps it (core.21), so a second delivery changes nothing. It is a string of 1 to 128 characters of letters, digits and `._:/-`, and the runner never reads into another's. It is not a want's id: an offer's, an act's or a command's `id` is the interface's `Id`, inside the body.
- **`body`** holds exactly one kind, by its name, which holds that kind's fields. A body is externally tagged, as the interface's own enums are.
- **Two directions.** `from-runner.schema.json` takes only the kinds a runner sends, and `to-runner.schema.json` only those a control plane sends. The Worker checks what it is sent against the first, and the runner checks what it is sent against the second (core.42). A kind of the other direction is refused.

Alternatives:

- **The kind as a `type` field beside the body's fields.** Not taken: a body's own `id`, such as an offer's, would clash with the message's. serde can read it only by flattening, which cannot also refuse unknown fields.
- **One envelope for both directions.** Not taken: the Worker would take a control plane's message from a runner.

### 4. Closed objects, and the forms of values

- **Closed.** Every object the schema names lists its fields and takes no other: `additionalProperties: false`, at every depth (core.42). The schema extends no object through `allOf`, so the closure needs no `unevaluatedProperties`.
  - A **map**, keyed by ids, paths or tools' names, constrains its keys with `propertyNames` and its values with a closed definition. Such maps are a hello's `tool_versions` and `leases`, a report's parts by id, an inbox's files by path, and a reported's `refused`.
  - The **values carried whole**, of any form, are a sluice row's `flaky` (Decision 9) and a thing an end made (Decision 7), each as its tool printed it.
- **Required.** Every field the interface's serde types always write is required, `null` where the interface writes none. The schema refuses a missing field that serde would read as none, so the two ends agree on what a message holds.
- **A time** of the runner's or a control plane's is RFC 3339 in UTC to the second, `YYYY-MM-DDTHH:MM:SSZ`, by pattern. `format: date-time` is not used, since validators treat it as an annotation by default and would disagree. A time inside a copy is the tool's, as it printed it: Flume's are Unix seconds, and sluice's are its own strings.
- **An integer** is between 0 and 2^53 − 1, or between −(2^53 − 1) and 2^53 − 1 where it may be negative, such as a launch's priority. The Worker's JavaScript then reads it exactly. A lease's number is 1 or more.
- **Bytes** are base64url without padding, by pattern: a key, a signature, a certificate and a sealed frame.
- **A repository's address** is `owner/name`. **A commit** is 40 or 64 lowercase hex digits. **A digest** is `sha256:` and 64 hex digits, as an inbox's brief is carried.

Alternatives:

- **Open objects**, so that a newer runner's added field passes an older control plane. Not taken: a field the cloud must not keep (core.27, cloud core.108) would then pass its check too.
- **`format: date-time`.** Not taken, as above.

### 5. The messages of version 1

| kind | sent by | holds | the interface's |
|---|---|---|---|
| `hello` | the runner | the hello (Decision 8 for its capacity) | `Hello`, handed to every pull |
| `report` | the runner | the runner's report, with each status and entry as its copy | `Report`, sent by `report` |
| `pulled` | the control plane | `wants`, in its order, and `report`, the runner's report as it holds it, or `null` | `Pulled`, but its terminals |
| `reported` | the control plane | `report`, the id of the `report` message it answers; `let_go`; `refused` | `Reported` |
| `message_refused` | either | `message`, the id of a message that failed its check; `words` | none: the transport's own |
| `terminal_open` | the control plane | `terminal`, `launch`, `lease`, and `share`, the browser's | `Terminal` |
| `terminal_opened` | the runner | `terminal`, and `share`, the runner's | none |
| `terminal_output` | the runner | `terminal`, `offset`, `redraw`, `sealed` | `Link::screen` |
| `terminal_input` | the control plane | `terminal`, `offset`, `sealed` | `Link::input`'s `Text` and `Keys` |
| `terminal_resume` | the control plane | `terminal`, `seen`, the offset its browser has seen up to | none |
| `terminal_close` | the control plane | `terminal` | `Link::input`'s `Close` |
| `terminal_closed` | the runner | `terminal`, `words` | `Link::close` |

Their order, which the README states and 1.15 and 1.6 build:

1. **At every connection**, the runner's first message is its whole `hello` (core.21). The control plane answers it with `pulled`, its `report` the runner's report as the control plane holds it. That is how a runner whose data directory was lost takes its report back (the archived change's Decision 11).
2. **Whenever its wants change**, the control plane sends `pulled` again, its `report` `null`. The hosted implementation answers each pull from the last `pulled`, and calls the `Wake` its opener was handed when one arrives.
3. **The runner sends `hello` again when it changes.** At each report the runner makes, it sends `report`, which renews every lease it names (cloud record 0009).
4. **The control plane answers each `report` with `reported`**, naming it by id. After a reconnect the runner resends, under its own id, the last `report` no `reported` has answered (core.21), and no earlier one, since a report is whole.
5. **A message that fails its check** is answered with `message_refused`, naming it, and nothing else is done with it (core.42; cloud core.36). The sender does not send it again.
6. **The terminals** (Decision 10).

A `pulled` is the whole of what the control plane wants, and a `report` the whole report, as the interface has them. A second delivery of either changes nothing, and the receiver needs no queue.

Alternatives:

- **A message for each want, and each part of the report**, each acknowledged by id. Not taken: the interface gives and takes them whole, and a part sent alone would need a renewal message for the parts that did not change. A report's size grows with its inboxes' files (*Risks*).
- **An application heartbeat.** Ruled out by core.21.

### 6. The interface's shapes, and the three places the wire differs

Version 1 carries these exactly as the interface's serde types write them:

- `Id`; `Lease`, with `number` and `expires`; `Answer` and `Outcome`;
- `Want` with `Offer`, `Launch`, `Hold`, `AgentSetting`, `Act`, `ActSort`, `Command`, `CommandSort` and `Inbox`;
- `ToolVersion`; `Acted`; `Commanded`; `InboxCarried` with `InboxFile`;
- `Hello`'s and `Report`'s other fields, and `Reported`'s `let_go` and `refused`.

It names where an interface type allows more than the protocol does: an act's or a command's outcome is `done` or `refused` (the interface's own doc comment says so), and an id holds the characters of Decision 3. `CommandSort::Other` is carried as the interface has it, so a control plane may ask for a tool's command this binary does not carry, which the runner refuses naming its tool (core.16).

It departs from them in three places, each because a rule makes the interface's form wrong for a hosted control plane:

1. **A place's status and sluice's entry go up as copies.** The interface's `Carried::text` holds a tool's answer as a text. Version 1's carried status is `{"copy": {…}, "at": …, "listed": …}`, its `copy` an object holding only the fields Decision 7 names, each as Flume printed it. A text would hide its fields from the Worker's check, and a validator reads no schema into a string, so the cloud could not refuse a field core.108 keeps from it. Its `at` and `listed` are the interface's. sluice's entry goes the same way (Decision 9).
2. **The host's capacity goes up as values**, its streams without their queues (Decision 8). The interface's `HostPart` holds each Flume field as a text, `streams` among them, whose queues name places of every partition on the host (core.26).
3. **A terminal's link is frames.** The interface's `Terminal` holds a `Link`, a channel. On the wire it is the terminal messages of Decision 10, sealed end to end (core.30).

The runner's own report file, `report.json`, is unchanged. It is written by the interface's types and read by the release before (core.36), and crew takes Flume's status whole (core.27 "Not ruled out").

### 7. The copy of a place's status

The copy is `flume status --json <place>`'s one place, `status.places[0]`, cut to these fields. Every object is closed (Decision 4), and every value is as Flume printed it. It is the allowed list of record 0014: "the list is that copy's schema". Its fields follow cloud core.108: a place's ids, states, times and counts; what it made; its pane's address; and the words a person reads to follow or answer it, an end's, a question's and a hold's.

| field | what the copy keeps of it |
|---|---|
| `place` | the place's own path (core.27 keeps it) |
| `kind`, `version`, `declared`, `name`, `mark` | whole |
| `branch`, `base`, `head` | whole |
| `lands`, `delivered`, `stalled`, `priority`, `current` | whole |
| `recipients` | whole: addresses |
| `stream` | `name`, `from` |
| `intent` | `name`, `from`, `base`, `digest`; never its `repository`, a path |
| `phase` | its variant and what says where the place stands: `"live"`; `making` with `by`; `over` with `over` and `kept`; `gone` with `over` and `at`; `tidied` with `over`, `at` and `pending_for` |
| an over (`phase.*.over`) | `"dropped"`, `"done"`, or `landed` with `commit` |
| a kept (`phase.over.kept`) | its variant alone, but `teardown`'s `step` and `agent_live`'s `identity`; never a teardown's `words` or an unreadable's `reason` |
| `stages[]` | `name`, `state`, `starts`, `end` (as an end), and `group`'s `place`, `of` and `number`; never `group.title` (a task group's title), `refused_part`, `refused_setting` or `conversation` |
| `ends[]` | `of`, `stage`, `start`, `submit`, `state`, `read_at`, `pending_for`, `deliveries` (each `recipient`, `at`, `by`), `end` (as an end), and `verdict` (as a verdict) |
| an end (`stages[].end`, `ends[].end`) | `head`, `words` (an end's words), `needs`, `commits`, `made` (each thing as the run wrote it, a line of text or an object naming the thing, carried whole); never `last_words`, `screen` or `report` (a path other than the place's own) |
| a verdict (`ends[].verdict`, `submission.verdict`) | its variant: `landed` with `commit`, `place_removed` and `branch_removed`; `failed`, `conflict` or `refused`, each with no field; `"withdrawn"`; never a verdict's `words`, `reason` or `files` |
| `holds[]` | `on`, `stage`, `since`, `words` (a hold's words) |
| `hold_words` | whole: each stage's hold words |
| `waiting_on` | `act`, `stage`, `why`, `since`, `words` (a question's or a hold's) |
| `submission` | `held`, `submits`, `send_backs`, `waiting`, `request`, `verdict` (as a verdict), and `read`: `"none"`, `found` with its submission's `id`, `branch`, `target`, `request` and `state`, or `could_not_read` with no field |
| a submission's state (`submission.read.found.submission.state`) | its variant: `"waiting"`, `"under_test"` or `"withdrawn"`; `landed` with `commit`; `failed` or `conflict`, each with no field; never a failure's `words` or a conflict's `files` or `base` |
| `agent` | `identity`, `stage`, `program`, `account` (`pool`, `name`, `from`), `state`, `started`, `asking`, `own_work`, `past_limit`; never `screen`, `printed`, `last_words`, `notes`, `settings` or `mode` |
| `clues` | `shown_at`'s `agent`, the pane's address (cloud core.108; core.29 "Not ruled out"); never `title`, `display_agent`, `stage`, `state` or `shown_at`'s `place` and `base` |
| `tasks[]` | `ticked` and `group` alone, never `text` (cloud core.108: a task's text never) |

Left out whole:

- `repository`, a path other than the place's own (core.27);
- `readiness`, whose steps' words are tools' output;
- `cannot_read`, `failures` and `refused_acts`, Flume's words;
- `moves`, whose accounts are Flume's bookkeeping, `unsettled`, and `role`;
- a stage's `quiet_from` and a stall's `since` and `waiting`, of `ends[]`;
- anything Flume adds after 0.4.1.

A field above is optional where Flume prints it only for some places, such as `stages[].group`, `ends[].submit` and `ends[].verdict`. It is `null` where Flume prints `null`.

1.15 cuts each status to this list. Whether it walks the schema itself or a list of its own is 1.15's choice. This change holds the list to the schema in a test (Decision 14).

Alternatives:

- **The whole status, with the cloud dropping what it does not keep.** Ruled out by core.27 and cloud record 0049.
- **Only the top-level fields named, each carried whole.** Not taken: `agent`, `stages[].end` and `phase.over.kept` hold text the cloud may not keep, beside fields it does.
- **A made thing closed to its `name`.** Not taken: Flume carries each thing's fields as the run wrote them, so a closed thing would refuse a report holding a thing with a field of its own. What a place made is the cloud's to keep (cloud core.108).
- **Leaving the copy to 1.15**, as record 0014 had it. Not taken: record 0049 and record 0056 make it version 1's (*What wave 0 settles*).

### 8. The hello, and the copy of Flume's host part

The hello is the interface's `Hello`, field for field:

- `interface_version`, an integer of 1 or more;
- `runner_version`;
- `tool_versions`, each tool's `printed` or `missing`;
- `repositories`, the addresses kept, which 1.15 gives as the partition's own (core.26; record 0006);
- `capacity`;
- `leases`, each id with its number.

Its `capacity` is `{"printed": {…}}` or `{"missing": "<words>"}`, as the interface's. The printed part is the copy of Flume's `status.host`:

| field | what the copy keeps |
|---|---|
| `running`, `bound`, `room` | whole, as Flume printed each: a number, or `null` for no bound |
| `streams[]` | `name`, `priority`, `running`, `bound`, `room`; never its `queue` |

A stream's `queue` names places of every partition the host serves, which core.26 keeps off another partition's connection. A stream's `room` says what the control plane needs to offer work (core.9). Flume's `pools`, `addresses`, `bases`, `queue` and `watch` are left out, as the archived change's Decision 6 left them out of `HostPart`: they are Flume's bookkeeping, and account names are among them.

### 9. The copy of sluice's entry

The copy of a submission's entry is the object the runner already builds from `sluice status <n> --json` (the archived change's Decision 13), carried as an object (Decision 6):

| field | what the copy keeps |
|---|---|
| `id`, `branch`, `head`, `state`, `landed_as` | whole, where sluice printed each |
| `failed[]` | `task`, `hash`, `flaky`, `system`; never `log` |

A row's `flaky` holds the task's outcomes and times, and the runner carries it whole, as sluice printed it. Its form is sluice's, and Decision 4 names it as the one value of any form.

### 10. A terminal: signed shares, sealed frames, and offsets

Cloud record 0016 puts the key exchange in the protocol's first version, and core.30 keeps every byte of a pane from leaving the host in the clear. Record 0061 makes the frames the runner's, which reach the page's xterm.js.

- **The shares.** `terminal_open` carries the browser's share, and `terminal_opened` the runner's, each a `share`:
  - `suite`, the scheme both ends use;
  - `key`, the end's public key for this terminal alone;
  - `signature`, by the end's signing key;
  - `certificate`, in base64url, which the control plane issued on that signing key: a member's short-lived key for the browser (cloud core.23), the host's key for the runner (core.31).

  The runner takes the browser's share only where its signature checks against a key certified to a member. It never opens a terminal on a share that fails, and never falls back to the clear (core.30). The signature is over the UTF-8 bytes of `flywheel-runner.v1 terminal share`, then, each after a NUL byte, the end (`browser` or `runner`), the terminal's id, the launch's id, the suite, and the key's bytes. A share therefore binds its terminal, its launch and its end, so a share is never taken for another terminal or reflected back.
- **The suite is named, not fixed.** `suite` is a string the two ends agree. The cloud relays it and cannot read what it names. The change that builds the runner's half with the browser's (1.15 and 5.5) chooses the scheme (cloud record 0016: HPKE, or ECDH with HKDF and AES-GCM), and names it in the README, without a new version.
- **Sealed frames.** `terminal_output` and `terminal_input` carry `sealed`, the suite's ciphertext of the frame's bytes, in base64url. A frame has no other field holding its bytes, so a frame in the clear fails the check.
- **The plaintext** of `terminal_output` is the pane's bytes as Flume's relay gives them, which the browser hands to xterm.js as they come. The plaintext of `terminal_input` is the bytes xterm.js gives for the person's keys, which the runner passes to Flume as a person's keys (record 0004).
- **Offsets (F54).** Each frame carries `offset`, the place of its first byte in its direction's stream, counted from 0 at the terminal's opening. A browser whose socket dropped, its keys still held, sends `terminal_resume` with `seen`, the offset it has every byte up to.
  - The runner keeps none of what it sent (core.11), and the cloud keeps nothing (cloud core.18). So where `seen` is behind what the runner has sent, the runner answers with a frame whose `redraw` is `true`: the pane's present screen, read from the pane through Flume's relay, at the runner's next offset. The browser clears its terminal and draws it.
  - Until Flume relays a stream of bytes, each screen the runner reads is a whole screen, so every frame it sends is a redraw.
  - A browser that lost its keys opens a new terminal.
- **Closing.** `terminal_close` closes a terminal from the browser's end. `terminal_closed` says the runner closed one, with its words, such as that no agent runs in its place. A terminal opens only while its host is connected (cloud record 0016), and a terminal is closed with its socket.

Alternatives:

- **Binary WebSocket frames for the bytes.** Not taken: the schema could not describe them, and the Worker could not check them (cloud core.36). base64url costs a third more bytes.
- **A buffer of sent bytes on the runner, replayed from `seen`.** Not taken: it keeps a terminal's bytes (core.11).
- **A fixed suite.** Not taken: the scheme is the building change's (cloud record 0016), and that change comes after a release speaks version 1.

### 11. What version 1 leaves out

The rules name more messages than any change has designed yet. Version 1 as this change writes it leaves out:

| what | its rule or record | the change that adds it |
|---|---|---|
| enrolment: the one-time token exchanged, the host's key registered, its certificate issued, a lapsed token's refusal, and the machine's name FR3 shows | core.31, core.32; record 0006; FR3, FR7 | 1.15 |
| the proof at each connection that the host's key is held | core.31 | 1.15 |
| a forge token sent down the runner's connection | core.33; record 0007 | the change that builds `flywheel forge-token` |
| health: pass durations, failures by kind, reconnects | core.28; record 0013 | the change that builds it |
| the runner's own commands, drain, resume, upgrade and setup, and a stray place's drop | core.16; records 0008, 0009, 0010 | each command's change |

Each such change adds its messages to version 1 while no release of the runner speaks it, or else makes version 2 (Decision 12). The first is likely for 1.15's, which comes before any release dials a control plane.

### 12. When a version stops changing

A version may change until a release of the runner speaks it to a control plane. From the first such release it never changes, and its directory is never removed (core.42). The cloud's pin can then hold the current version and the one before (cloud core.36, record 0056). Every change after that is a new version, in a directory of its own beside it, made here first. The cloud takes it by moving its pin.

The gate cannot know what a release spoke until a release exists (record 0012), so this is a rule a reviewer holds. The cloud's jsoncompat check (F8, 1.6) holds the version before against the current one on its side. The change that cuts the first release that speaks version 1 adds this repository's check, a digest of each of its files.

### 13. The runner's types, tested against the schema (F50)

The runner's types for every message are serde types in `flywheel_hosted::protocol::v1`:

- **`Message<Body>`**: `protocol_version`, `id` (a `MessageId`) and `body`.
- **`FromRunner` and `ToRunner`**, the bodies, one variant for each kind of Decision 5.
- **The interface's own types**, wherever version 1 carries a shape as the interface writes it (Decision 6). These are `flywheel_control`'s `Want`, `Lease`, `Id`, `At`, `Answer`, `ToolVersion`, `Acted`, `Commanded` and `InboxCarried`. They are used, never redefined.
- **Version 1's form of a type** where the wire differs, named as the interface names the thing it carries, so that the module's path says which form a reader holds. These are `v1::Hello`, `v1::Capacity`, `v1::HostPart`, `v1::Report`, `v1::Launched`, `v1::Carried`, `v1::Pulled` and `v1::Reported`. A copy, and each of `HostPart`'s fields, is a `Box<serde_json::value::RawValue>`, so a value goes up as the tool printed it, never re-serialised (core.27).
- **The terminal's types**: `Share`, and one struct for each terminal message.
- **`VERSION`**, `1`; **`ADDRESS`**, the address each file's `$id` is under; and **`FILES`**, each schema file's name and text by `include_str!`. The tests build their validators from `FILES`, and 1.15's runtime check (core.42) reads the same files.

Nothing here converts the interface's values into version 1's or back. That cutting is 1.15's, with the socket.

The tests check what the types write, and what they read, against the schema with the `jsonschema` crate, a dev-dependency with no default features, so that it fetches nothing (Decision 14). The runner's runtime check of what it is sent is 1.15's, and may use the same crate.

Alternatives (F50):

- **typify, generating the types from the schema.** Not taken: it would make a second model of the interface's shapes under the schema's names (core.12), and reuse none of the interface's types.
- **schemars, generating the schema from the types.** Not taken: the schema is the public contract, fixed once spoken (Decision 12), and every edit of a type would edit it unreviewed.

### 14. The tests

`flywheel-hosted:test` holds the schema and the types to each other. It reaches no network.

- **The files.**
  - Every file under `protocol/v1/` that ends `.schema.json` is in `FILES`, and every file in `FILES` is there.
  - Each passes the 2020-12 metaschema, and its `$id` is its own address (Decision 2).
  - Validators built from `FILES` alone resolve every `$ref`.
- **The examples.**
  - Each example under `examples/from-runner/` passes `from-runner.schema.json`, reads into `Message<FromRunner>`, and writes back to an equal JSON value. The same holds for `examples/to-runner/` and `Message<ToRunner>`.
  - Every kind of Decision 5 has an example.
  - The place-status examples are cut, by Decision 7's list, from the shapes of Flume 0.4.1's fixtures: a place waiting on an answer, one on hold, a role's run delivered, a submission under test, a landing kept by a failed teardown, a tidied place, and a live agent with its pane. Their names are neutral (core.41).
- **The interface's values.** One value of each want, act and command sort, a lease with and without an expiry, a launch with every field and one with none, each tool version, an acted, a commanded and an inbox carried, written by `flywheel_control`'s types, each pass their definitions.
- **The refusals.** Each case below is a valid example altered, and each is refused:
  - a message with no id, with an empty id, or with an id holding a space;
  - `protocol_version` 2;
  - no body, two kinds in one body, a body of an unknown kind, and a body of the other direction;
  - a field the version does not name: at the envelope, in a launch, in a lease, in a status copy, in a share;
  - a time with an offset other than `Z`, or a fraction of a second;
  - an integer past 2^53 − 1, and a lease numbered 0;
  - an acted's or a commanded's outcome `launched`;
  - a status copy holding an agent's `last_words`, `screen` or `printed`, a task's `text`, `readiness`, a group's `title`, a teardown's `words` or an end's `report`;
  - a capacity stream holding its `queue`, and a sluice row holding its `log`;
  - a terminal frame with no `offset`, one carrying its bytes under any field but `sealed`, and one whose `sealed` is not base64url;
  - a share with no `signature` or no `certificate`.
- **The copies' lists.** The field paths of the place-status copy, of the host part's copy and of sluice's entry, read from the schema, each equal the list the test holds, which is Decisions 7 to 9's tables. A field added to a copy or taken from it fails the test, naming it, as Flume's suite holds its own field list.
- **No grant, no credential.** No property anywhere in version 1 is named `grant`, `role`, `permission`, `token`, `credential`, `secret` or `password` (records 0047 and 0065; core.4).

`flywheel:test`'s rules test holds the crate to core.2 and core.7 (the control-plane-interface spec).

### 15. JSON in the tree

No tracked JSON is formatted today: the tools' fixtures are excluded, and nothing else is JSON. The schema files and the examples are this repository's own, so `treefmt.nix` claims `*.json` with `programs.jsonfmt`, from the nixpkgs `devenv.lock` pins (core.7, core.8). The fixtures stay excluded by name. A test compares JSON as values, never as bytes, so the formatter's layout changes nothing a test holds.

Alternative: excluding the protocol's files from formatting. Not taken: they are written by hand and read by the public, and core.7 wants every file owned.

### Rules relied on

- **core.2**: Decisions 1 and 13, and the control-plane-interface spec's scenario.
- **core.3**: nothing here opens a socket. A connection is the runner's, outgoing (Decision 3).
- **core.4**: Decision 14's names. No credential is a field, and none is in an example.
- **core.5** and **core.16**: no message names a repository for the host to keep (*What wave 0 settles*).
- **core.7**: Decisions 1 and 15.
- **core.8**: jsonfmt from the pinned nixpkgs, and `jsonschema` pinned by `Cargo.lock`.
- **core.11**: Decisions 7 to 10. Nothing of a terminal is kept.
- **core.12**: every file, field and type named for what it holds (Decisions 2, 5 and 13).
- **core.21**: Decisions 3 and 5. Ids kept on a resend, the whole hello first, and no heartbeat.
- **core.26**: Decision 8. No stream's queue, and the partition's own repositories.
- **core.27**: Decisions 6 to 9.
- **core.29**: Decision 10. The pane is reached through Flume, and its address is carried as Flume printed it.
- **core.30**: Decision 10.
- **core.36**: `report.json` unchanged (Decision 6).
- **core.41**: the examples' neutral names. No text of the cloud's is copied: its records and rules are cited by id.
- **core.42**, added in this change's commit: every decision.

### Rules departed from

None.

## What this change asks of others

- **flywheel-cloud, 1.6.**
  - Pin `crates/flywheel-hosted/protocol/v1/` by revision.
  - Check every message a runner sends against `from-runner.schema.json`, with validators built from the version's files alone and nothing fetched.
  - Answer a failing message with `message_refused`.
  - Refuse an upgrade that asks for a subprotocol it does not accept.
- **flywheel-cloud, 1.11.** Build the launch's copy and history from Decision 7's fields, and ask here first for any field it lacks.
- **flywheel-cloud, 5.5.** The browser's half of a terminal:
  - its share, signed over Decision 10's context;
  - `terminal_resume` on a reconnect, and a redraw drawn after clearing;
  - the suite, chosen with the runner's half.
- **flywheel-cloud, the issuer of certificates** (its 1.7 and 2.2). Give the runner, at enrolment, what it checks a member's certificate against, so it can check the browser's share.
- **This repository's 1.15.**
  - Cut each status to Decision 7's copy and the host part to Decision 8's.
  - Check every message it is sent against `to-runner.schema.json`.
  - Give message ids that survive a reconnect.
  - Add enrolment and the key's proof to version 1 before any release speaks it (Decision 11).
- **Flume**, as record 0004 already asks: a command that relays a place's terminal, whose bytes the runner seals.

## Risks / Trade-offs

- **Any added field is a new version** once a release speaks version 1 (Decision 4). The cloud accepts two versions at once (cloud core.36), so a field costs a version and a pin's move. Version 1 stays open to change until 1.15's first release, so the fields wave 1 finds missing are added before then.
- **A report is sent whole.** Its size grows with each inbox's files and each place's copy. The copies are small, and an inbox's briefs go by digest. A host with a large inbox makes large reports, and splitting the report is a new version.
- **The copy follows Flume 0.4.1's form.** Flume only gains fields within its version, so a later Flume's fields are left out of the copy and never refused. A Flume that changes a kept field's form would fail the check. It would break its own contract first, and the runner's checked versions (core.38) name it.
- **A redraw is the only resume.** A browser that misses frames redraws the whole screen, and anything that scrolled off the pane's screen in the gap is lost to it. That is the price of keeping nothing (core.11, cloud core.18).
- **The suite is unfixed.** Two ends that name different suites fail to open a terminal, and never fall back to the clear.
- **`$id`s name a URL that serves nothing until the repository is public** (record 0014). No validator fetches it. They name the public repository `agentplot/flywheel-runner`. Should it open under another name, the `$id`s move with it before a release speaks version 1.
- **The gate's sandbox reaches only localhost.** The `jsonschema` crate and its dependencies must be in cargo's cache before the gate runs, as the weir revision is.

## Migration Plan

Nothing on a host changes. The runner carries no new behaviour, its configuration and report file are unchanged, and `type = "hosted"` is still refused. The cloud's 1.6 takes the schema by pinning this repository's revision once this change lands.

## After deploy

Nothing. No step of this change is outside the gate.

## Task notes

**1.1** `crates/flywheel-hosted/Cargo.toml`:

- `publish = false`, with the workspace's version and edition, and `[lib] doctest = false`;
- dependencies: `flywheel-control` by path, `serde`, and `serde_json` with `raw_value`, from `[workspace.dependencies]`;
- dev-dependency: `jsonschema`, which the root `Cargo.toml`'s `[workspace.dependencies]` gains with `default-features = false`.

Add the crate to `members`, with a line in the root `Cargo.toml`'s comment.

`crates/flywheel-hosted/moon.yml` copies `crates/flywheel-crew/moon.yml`'s four tasks with `-p flywheel-hosted`, `language: 'rust'` and `layer: 'library'`. Their inputs are:

- `/crates/flywheel-hosted/**/*` and `/crates/flywheel-control/**/*`;
- `/Cargo.toml`, `/Cargo.lock`, `/devenv.nix`, `/devenv.lock`, `/.prototools` and `/rust-toolchain.toml`.

Add `flywheel-hosted: 'crates/flywheel-hosted'` to `.moon/workspace.yml`, with its line in the comment there and in the root `moon.yml`'s. `src/lib.rs` says what the crate is: the hosted implementation, which holds the protocol now and implements `ControlPlane` with 1.15. README's and CLAUDE.md's lists of crates and projects name it.

**1.2** In `crates/flywheel/tests/rules.rs`:

- `the_crates_depend_as_decision_1_has_them` gains `("flywheel-hosted", {"flywheel-control"})`, and its doc comment names it;
- a new test, for the control-plane-interface spec's scenario "Only the hosted implementation's crate holds the protocol", reads every file under `crates/` outside `crates/flywheel-hosted`, `planes.rs` and the test's own file aside, and `crates/flywheel/moon.yml` with its input `/crates/flywheel-hosted/**/*` taken out. Those two hold the crate to the rule, so they name it. It checks that none names `flywheel_hosted` or `flywheel-hosted`, that no file name ends `.schema.json`, and that no crate depends on `flywheel-hosted`. The names are written whole in the test, not built from parts: built names would hide the test's own mention of the crate instead of excepting it by name.

`crates/flywheel/moon.yml`'s `test` task takes `/crates/flywheel-hosted/**/*` among its inputs, since the rules tests read it.

**1.3** `treefmt.nix`: `programs.jsonfmt.enable = true`. The fixtures' exclude stands, and a comment says jsonfmt owns the tree's own JSON. Run `treefmt` once on the new files.

**2.1** `protocol/v1/shared.schema.json` holds the `$defs` of Decision 2's table, with the forms of Decision 4. `from-runner.schema.json` and `to-runner.schema.json` hold the envelope of Decision 3:

- `protocol_version` `const` 1;
- `id` the message id;
- `body` a `oneOf` of one-property objects, one for each kind of Decision 5 in that direction, each `$ref`ing its definition.

`src/protocol/v1.rs`, or a module tree under it, holds `VERSION` and `FILES`. `tests/files.rs` holds Decision 14's tests of the files. Build the validators with `jsonschema::options()`, each file added as a resource under its `$id`, and with no retriever, so that a reference outside `FILES` fails. Check each file with `jsonschema::meta`.

**2.2** `hello.schema.json`, `wants.schema.json` and `report.schema.json`, as Decisions 5, 6 and 8 have them. Each object is closed and each field the interface's serde writes is required. `tests/interface.rs` builds the interface's values as `flywheel-control`'s `tests/json.rs` builds them, with digests and commits of full length. It checks each against its definition by a validator of `{"$ref": "<file's $id>#/$defs/<name>"}`. Add one example of `hello`, `report`, `pulled`, `reported` and `message_refused` to each direction that sends it.

**2.3** `place-status.schema.json` and `submission-entry.schema.json`, and the host part's copy in `hello.schema.json`, as Decisions 7 to 9's tables have them.

- Read each kept field's form from Flume 0.4.1's fixtures, and, where a fixture shows it only empty or `null`, from agentplot/flume's `crates/flume-seams/src/held.rs` and `merge_queue.rs` at `v0.4.1`.
- Write the place-status examples of Decision 14 as `report` examples, each copy cut by hand from a fixture's place by the table, its paths and names made neutral, such as `/home/runner/places/fix--tabs` and `example/app`. Copy no path or name of a fixture's (core.41).
- `tests/copies.rs` walks each copy's schema for its field paths and compares them with the tables' lists, written in the test.

**2.4** `terminal.schema.json`, as Decision 10 has it, and an example of each terminal kind.

**2.5** `crates/flywheel-hosted/protocol/README.md` says what Decision 2 lists for it: the versions and when one is fixed, the messages and their order, the copies, the terminal's frames with their signatures' context, and the subprotocol. It says that no validator fetches a `$id`, and that the repository's own records and rules are its source, by id. It is prose, which treefmt excludes as `*.md`.

**3.1** The types of Decision 13 in `src/protocol/v1.rs`. Each derives `Serialize` and `Deserialize`. Each struct of version 1's own takes `#[serde(deny_unknown_fields)]`, so that reading a message with a field the version does not name fails, as the schema does. `tests/examples.rs` reads every example into its direction's `Message`, writes it back, compares the two as `serde_json::Value`s, and checks the written one against the schema. A list in the test of every kind of Decision 5 is met by at least one example of each.

**3.2** `tests/refused.rs`: each case of Decision 14's refusals, built from an example by `serde_json::Value` edits, checked against its direction's schema, refused, and named in the assertion's message.

**3.3** `tests/names.rs` walks every file in `FILES` for its property names, and its `properties` and `$defs` keys, and finds none of Decision 14's names.
