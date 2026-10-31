# crew's control plane

## Purpose

crew's implementation of the control-plane interface, the one part of
the runner that names crew (core.2). For each partition its
configuration names, it reads what crew wants of the host from crew's
records at `<label>/main` of the partition's state repository. It
writes the runner's answers, and Flume's status of each place, on the
runner's own branch `<label>/runner/<host>`, in crew's formats
(crew's `teams-cross-to-flume-as-their-bolts-land` Decision 4, and
`roles-run-as-flume-kinds` R1 to R4). It pushes no other branch.

## ADDED Requirements

### Requirement: crew's implementation is configured per host and partition

crew's table SHALL name:

- `host`, this host as crew's hosts file names it, with no default;
- at least one partition, each a `label` once and a `state` repository
  as `owner/name`;
- `kinds`, each kinds file a role record may name, by that name.

A missing or malformed key SHALL be refused at start, naming the key.
A state repository SHALL be reached over https through the host's git
credential helper, and the table SHALL name no credential.

#### Scenario: A partition with no state repository

- **WHEN** crew's table names a partition `madswan` with no `state`
- **THEN** crew's implementation refuses to open, naming `state` of partition `madswan`

#### Scenario: No host

- **WHEN** crew's table names no `host`
- **THEN** crew's implementation refuses to open, naming `host`

### Requirement: The requests for this host are read from `<label>/main`'s tip

Each pull SHALL read `requests.rec` at one tip of `<label>/main` for
each partition. It SHALL give each request whose `Host` is this host,
in `Request` order, keyed by its partition's label and its number,
with each field crew's request names:

- a launch's `After` as the keys of the launches it names;
- its `Brief` with every line;
- `Hold: submission` as a held submission;
- a retry's `Submission` field as a retry of the submission.

#### Scenario: Two hosts' requests

- **WHEN** `madswan/main`'s `requests.rec` holds launch 1 for `deneb` and launch 2 for `alpha`, and this host is `deneb`
- **THEN** the pull gives launch 1 alone

#### Scenario: A launch's every field

- **WHEN** launch 3 for this host names `Repo`, `Kind`, `Name`, `Base`, `Stream`, `Priority`, `Intent`, `Recipient`, a `Brief` over three lines, and `After` 1 and `After` 2
- **THEN** the pull gives a launch with each of those values, the brief's three lines, and the keys of launches 1 and 2

### Requirement: A request or a partition that cannot be read stops nothing else

A request it cannot read SHALL be given so the runner refuses it,
naming the field. A partition whose branch cannot be read SHALL give
nothing that pass, and every other partition SHALL be read as it
stands.

#### Scenario: An act crew does not write

- **WHEN** request 5 for this host names `Act: rebase`
- **THEN** the pull gives request 5 so that the runner answers it `refused`, naming `Act`

#### Scenario: One partition unreadable

- **WHEN** the configuration names `madswan` and `wldn`, and `wldn/main` does not exist
- **THEN** the pull gives `madswan`'s requests, and `wldn`'s absence is logged

### Requirement: The runner's branch is written by weir-branch's write

Each report SHALL write `runner.rec`, `answers.rec` and `places.rec` on
`<label>/runner/<host>` for each partition:

- formed against the branch's tip;
- each file checked by `recfix --check`;
- one commit;
- the branch moved only from that tip, never with force.

Nothing SHALL be written when the files' text is unchanged. The
branch's first write SHALL hold `runner.rec`, naming the host, the
runner's version and when it began serving the partition.

#### Scenario: The first pass makes the branch

- **WHEN** the runner reports its first record for `madswan`, which holds no answers
- **THEN** `madswan/runner/deneb` exists on the state repository with a `runner.rec` naming `deneb`, the runner's version and a `Since`

#### Scenario: Nothing changed

- **WHEN** the runner reports the same record twice
- **THEN** the second report makes no commit on `madswan/runner/deneb`

### Requirement: No branch but the runner's own is pushed

No ref but `<label>/runner/<host>` SHALL be pushed to a state
repository, and `<label>/main` SHALL never be written.

#### Scenario: The flywheel's branch untouched

- **WHEN** a report writes answers and places for `madswan`
- **THEN** `madswan/main` stands at the commit it stood at before, and the state repository has no new ref but `madswan/runner/deneb`

### Requirement: An answer is one record per request read

`answers.rec` SHALL hold one `Answer` record per request the runner
has read, in `Request` order:

- `Request`;
- `Outcome`, one of `held`, `launched`, `done` and `refused`;
- `Place`, once launched;
- `Words`, as given;
- `At`.

A held launch, once launched, SHALL be the same record with its new
outcome.

#### Scenario: Held, then launched

- **WHEN** launch 4 is reported `held`, and at a later pass `launched` at a place
- **THEN** `answers.rec` holds one record of `Request` 4, its `Outcome` `launched` and its `Place` that place

### Requirement: A place is carried with Flume's status as one line

`places.rec` SHALL hold one `Place` record per place launched for a
request, with `Request`, `Place`, `At` and `Status`, the place's
object in Flume's status, one line, byte for byte. It SHALL hold one
`Run` record per role whose run the runner keeps, by `Address`, with
its `Place`, `At` and `Status`, or `Words` where no run could be
launched.

#### Scenario: A place's status as Flume printed it

- **WHEN** the runner reports launch 3's place with a status fixture from flume 0.3.6
- **THEN** `places.rec` on the branch holds a `Place` record of `Request` 3 whose `Status` is that fixture's place object, byte for byte

#### Scenario: A run that could not be launched

- **WHEN** the planner's run of madswan is recorded with Flume's refusal and no place
- **THEN** `places.rec` holds a `Run` record of `Address` `madswan/planner` with those `Words` and no `Place`

### Requirement: A tidied place's record is kept until its work has left crew's plan

A place Flume no longer lists SHALL keep its last `Place` record until
the work its request served has left crew's plan at `<label>/main`'s
tip:

- `unit/<u>` and `chore/<c>` with the plan's `Unit` record of that
  name;
- `fix/<bolt>/<name>` with the plan's `Bolt` record of `<bolt>`.

Then it SHALL be let go. Work the plan never holds SHALL keep its last
record.

#### Scenario: A unit still in the plan

- **WHEN** unit `a`'s place is no longer listed by Flume and the plan still holds `Unit` `a`
- **THEN** `places.rec` keeps `a`'s record

#### Scenario: The bolt landed

- **WHEN** unit `a`'s place is no longer listed and the plan holds neither `Unit` `a` nor its bolt
- **THEN** `places.rec` holds no record of `a`'s request, and the report lets that place go

### Requirement: Each role record of this host is a run

For each `Role` record whose `Host` is this host, the pull SHALL give
a run keyed by its `Address`:

- launched from its `Repository`, `Kind`, `Name`, `Stream`, `Priority`
  and `Intent`;
- the kinds file the configuration names for its `Kinds`, or
  unrunnable where none is named;
- the agent setting `model=<Model>`.

#### Scenario: The planner's role record

- **WHEN** `madswan/main`'s `roles.rec` holds the planner's record for `deneb`, `Kinds` `crew`, and the configuration names a kinds file for `crew`
- **THEN** the pull gives a run keyed `madswan/planner`, its launch naming that kinds file, `crew-planner`, `madswan`, its stream, priority 0, intent `madswan-planner` and `model=` its model

#### Scenario: A role record on another host

- **WHEN** `roles.rec` holds the planner's record for `alpha`, and this host is `deneb`
- **THEN** the pull gives no run of `madswan/planner`

#### Scenario: A role that stands

- **WHEN** `roles.rec` holds no record of madswan's design agent
- **THEN** the pull gives no run of `madswan/design`

### Requirement: A role's run is wanted while anything is pending for it, read as crew reads it

A role's run SHALL be wanted while anything is pending for the role:

- an `Ask` of the role whose `ask/<Id>` no ended `Turn` of the role
  took;
- a place of any host's `places.rec` whose launch names the role's
  `Address` as `Recipient`, waiting on its caller as crew reads a
  wait, whose key no ended turn took;
- a place of a `role/…` launch, landed, whose key no ended turn took.

A host's runner branch that cannot be read SHALL add no waits, and
every other source SHALL still count.

#### Scenario: A burst

- **WHEN** three asks of madswan's planner are written and no turn has taken them
- **THEN** the pull gives the planner's run wanted

#### Scenario: An ask during a run

- **WHEN** a turn of the planner that took asks 1 and 2 has ended, and ask 3 was written after it began
- **THEN** the pull gives the planner's run wanted

#### Scenario: Nothing pending

- **WHEN** every ask of the planner was taken by a turn that ended, and no place waits on `madswan/planner`
- **THEN** the pull gives the planner's run not wanted

#### Scenario: A stage on another host asks the planner

- **WHEN** `madswan/runner/alpha`'s `places.rec` holds request 9's place waiting on its caller with `why` `answer`, and request 9's launch names `madswan/planner` as its `Recipient`
- **THEN** the pull on deneb, the planner's host, gives the planner's run wanted

#### Scenario: A superseded turn answers nothing

- **WHEN** a turn of the planner took ask 5 and was superseded, and no ended turn took ask 5
- **THEN** the pull gives the planner's run wanted

#### Scenario: Another host's branch unreadable

- **WHEN** `madswan/runner/alpha`'s `places.rec` is refused by `recfix`, and one ask of the planner is pending
- **THEN** the pull gives the planner's run wanted, and alpha's branch is logged by name

### Requirement: The record is read back from the runner's own branch

Where the runner holds no record of its own, a pull SHALL give the
record that `answers.rec` and `places.rec` on `<label>/runner/<host>`
hold, each request answered there keyed as the pull keys its request.

#### Scenario: A host whose data directory was lost

- **WHEN** `madswan/runner/deneb` answers request 3 `done`, and the runner starts with no record of its own
- **THEN** the pull's record answers request 3 `done`, under the key the pull gives request 3
