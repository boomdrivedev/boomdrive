# crew's control plane

## Purpose

crew's implementation of the control-plane interface, the one crate that names crew. It reads crew's records at one tip of each partition's flywheel branch, `<label>/main`: the requests crew writes, its roles that run as Flume kinds, its asks and its plan. It gives them to the runner as offers and acts, under the lease number 1 with no expiry, since crew addresses each request to one host and never moves it. It writes the runner's answers, and Flume's status of each place, on the runner's own branch, `<label>/runner/<host>`, in crew's formats field for field, and pushes no other branch (core.2, core.10; crew record 0052; crew's `teams-cross-to-flume-as-their-bolts-land` Decision 4 and `roles-run-as-flume-kinds` R1 to R4, as built at crew `9913f249`).

## ADDED Requirements

### Requirement: crew's table names the host, the partitions and the kinds files

crew's implementation SHALL open from `[control_plane.crew]`:

- `host`, crew's name for this host, required, with no default;
- `partitions`, at least one, each a `label` given once and a `state` repository as `owner/name`;
- `kinds`, each kinds file a role record may name, by that name, as a path.

A missing or malformed key SHALL be refused, naming the key and the file.

#### Scenario: The table read

- **WHEN** the table says `host = "deneb"`, one partition `madswan` with `state = "afterthought/madswan-state"`, and `kinds = { crew = "/k/kinds.toml" }`
- **THEN** crew's implementation opens for host `deneb`, serving `madswan` from `afterthought/madswan-state`, with crew's kinds file at `/k/kinds.toml`

#### Scenario: No host

- **WHEN** the table names no `host`
- **THEN** it is refused, naming `host` and the configuration file

#### Scenario: A label twice

- **WHEN** two partitions say `label = "madswan"`
- **THEN** it is refused, naming `madswan`

### Requirement: A state repository is reached through the host's own credential helper

A state repository SHALL be reached at `https://github.com/<owner>/<name>.git` by the runner's git calls, through the host's git credential helper. crew's table SHALL hold no credential, and a key it does not name SHALL be refused (core.4).

#### Scenario: A token in crew's table

- **WHEN** `[control_plane.crew]` says `token = "..."`
- **THEN** it is refused, naming `token` as a key crew's implementation does not read, and the value appears in no output

### Requirement: crew's records are read at one tip of each partition's flywheel branch

At each pull, for each partition, crew's implementation SHALL read `requests.rec`, `roles.rec`, `asks.rec` and `plan.rec` at one tip of `<label>/main`, through a bare cache at `<data>/control/crew/<owner>/<name>.git`. A file missing at the tip SHALL hold nothing. It SHALL read every `<label>/runner/*` branch the state repository lists, taking its own host's places from the record the runner hands the pull.

#### Scenario: A file missing at the tip

- **WHEN** `madswan/main`'s tip holds `requests.rec` and no `roles.rec`
- **THEN** the pull gives the requests for this host, and offers no role's run

### Requirement: A partition that cannot be read hides only its own work

A partition whose `<label>/main` cannot be read SHALL give nothing that pass, with its words, while the other partitions give their own. A pull SHALL be unread only when no partition could be read.

#### Scenario: One partition missing

- **WHEN** partition `swancloud`'s state repository has no `swancloud/main` and `madswan`'s holds requests for this host
- **THEN** the pull gives `madswan`'s requests and names `swancloud/main` as unread

#### Scenario: No partition read

- **WHEN** no partition's state repository can be read
- **THEN** the pull is unread, naming each

### Requirement: Each request for this host is a want under the lease number 1

crew's implementation SHALL give each `Request` whose `Host` is this host, in `Request` order, as a want with the id `<label>/<Request>` and the lease number 1, with no expiry. It SHALL give none for another host.

#### Scenario: Only this host's requests

- **WHEN** `requests.rec` holds launches for `deneb` and for `vega`, and the host is `deneb`
- **THEN** the pull gives `deneb`'s alone, in `Request` order, each under lease 1

### Requirement: A launch request is an offer of what it names

A `launch` SHALL be given as an offer whose asks are its `Repo`, `Kind`, `Name`, `Base`, `Stream`, `Priority`, `Intent`, each `Recipient` and `Brief`, each `After` as the launch `<label>/<n>`, and `Hold: submission` as a held submission.

#### Scenario: A launch's every field

- **WHEN** request 12 is a launch for this host naming every field, with a `Brief` over three `+` lines and `After` 10 and 11
- **THEN** the pull gives an offer `madswan/12`, its brief the three lines joined by newlines, coming after `madswan/10` and `madswan/11`

### Requirement: A retry, a drop and a priority are acts on the launch they name

A `retry` SHALL be given as an act on the launch `<label>/<Of>` with its `Stage` and `Words`, asking for the submission where it holds a `Submission` field. A `drop` SHALL be an act dropping `<label>/<Of>`. A `priority` SHALL be an act setting `<label>/<Of>`'s priority to `Priority`.

#### Scenario: A retry of the submission

- **WHEN** request 14 retries `Of: 12` with `Submission: yes`
- **THEN** the pull gives an act `madswan/14` on `madswan/12` asking for the submission

### Requirement: A request crew's implementation cannot read is refused

A request with a number that is not whole, an `Act` crew's implementation does not know, or a `Hold` other than `submission` SHALL be given as one the runner refuses, with words naming the field. A record with no `Request` number SHALL be left out and named, since it can be given no id.

#### Scenario: An act crew does not write

- **WHEN** request 15 says `Act: pause`
- **THEN** the pull gives it as one the runner answers `refused`, with words naming `Act` and `pause`

### Requirement: The runner's branch is written by weir's write, and only it is pushed

At each report, for each partition, crew's implementation SHALL write `<label>/runner/<host>` by weir-branch's write: formed against the branch's tip, each `.rec` file checked by `recfix --check`, one commit, and the branch moved only from that tip by a leased push, never forced. It SHALL write nothing when the files' text equals the tip's. It SHALL push no ref but `<label>/runner/<host>`.

#### Scenario: Nothing changed

- **WHEN** the runner reports the same record twice
- **THEN** the second report makes no commit

#### Scenario: Only the runner's branch

- **WHEN** a report has been written
- **THEN** the state repository's refs are those it held before, with `madswan/runner/deneb` beside them, and every file written passes `recfix --check`

### Requirement: The runner's branch holds crew's three files in crew's formats

crew's implementation SHALL write `runner.rec`, one `Runner` with `Host`, `Version` and `Since`, the first write's time, kept after. It SHALL write `answers.rec`, one `Answer` per request the record holds. It SHALL write `places.rec`, one `Place` per launched request, then one `Run` per role, by `Address`. Each file SHALL open with crew's own descriptor, byte for byte as crew's `runner.py` holds it.

#### Scenario: The first report

- **WHEN** the runner first reports for `madswan` on host `deneb`
- **THEN** the state repository gains `madswan/runner/deneb`, holding `runner.rec` with `Host: deneb`, the runner's version and the time, and `madswan/main`'s tip is unchanged

#### Scenario: A place's status as Flume gave it

- **WHEN** the record carries a place's status as a fixture's text
- **THEN** `places.rec`'s `Status` for its request is that text, byte for byte, and crew's descriptors open each file byte for byte

### Requirement: A role is wanted while anything is pending for it

For each `Role` of `roles.rec` whose `Host` is this host, crew's implementation SHALL read what is pending for it as crew's `asks.py` does: each `Ask` of the role, and each place that waits on the role's `Address`, that no `Turn` of the role with `Ended` took. A superseded or open turn SHALL answer nothing.

#### Scenario: Nothing pending

- **WHEN** every ask of `planner` was taken by an ended turn, and no place waits on it
- **THEN** the pull gives no offer for `planner`

#### Scenario: A superseded turn answers nothing

- **WHEN** the only turn that took an ask of `planner` is superseded
- **THEN** the ask is pending, and the pull gives an offer of `planner`'s run

### Requirement: A place waits on a role as crew reads it

A place of any host's `places.rec` SHALL wait on a role where its launch request names the role's `Address` as `Recipient`, and its status waits on its caller with `why` `answer`, `lost` or `past_bound`, with `on_hold` naming no stage, or `stalled`. A `role/…` launch's place that has landed SHALL wait on the role too.

#### Scenario: Another host's place asks the planner

- **WHEN** `madswan/runner/vega`'s `places.rec` holds a place whose launch names `madswan/planner` as `Recipient` and whose status waits with `why` `answer`
- **THEN** the pull gives an offer of `planner`'s run

#### Scenario: A role's work landed

- **WHEN** a `role/planner/x` launch's place reads landed, and no ended turn took it
- **THEN** the pull gives an offer of `planner`'s run

### Requirement: A role's run is offered while it is wanted and none stands

crew's implementation SHALL offer a run of a wanted role whose latest run has no place reading `live` or `making`, as the launch `<label>/<role>/<k+1>`, where `k` is the highest the record holds of the role. Its asks SHALL be the role's `Repository`, the configured path for its `Kinds`, its `Kind`, `Name`, `Stream`, `Priority` and `Intent`, and the agent setting `model=<Model>`, with no base, brief or recipient.

#### Scenario: A burst of asks

- **WHEN** `asks.rec` holds three asks of `planner` that no ended turn took, and no run of `planner` stands
- **THEN** the pull gives one offer, `madswan/planner/1`, of kind `crew-planner` named `madswan`, its kinds crew's configured file, its stream the role's, priority 0, intent `madswan-planner` and the agent setting `model=<the role's Model>`

#### Scenario: A run standing

- **WHEN** asks of `planner` are pending and its latest run's carried status reads `live`
- **THEN** the pull gives no offer for `planner`

#### Scenario: A run waiting for room keeps its id

- **WHEN** an offer of `madswan/planner/3` is left untaken for want of room
- **THEN** the next pull offers `madswan/planner/3` again; and once the runner records it refused, the pull after offers `madswan/planner/4`

### Requirement: A role whose kinds file is not configured is offered no run

Where the configuration names no kinds file for a role's `Kinds`, crew's implementation SHALL offer no run of the role, and SHALL write the role's `Run` record with words saying so.

#### Scenario: No kinds file

- **WHEN** a role's `Kinds` is `crew` and the configuration's `kinds` names none
- **THEN** no offer is given for it, and its `Run` record's `Words` say the configuration names no kinds file `crew`

### Requirement: A role's run is declared done once its turn has delivered

Where a role's latest run's carried status waits on its caller with `why` `delivered`, crew's implementation SHALL give one act, `<label>/<role>/<k>/done`, declaring that launch done. A refusal of it SHALL be written as the run's `Words`.

#### Scenario: A turn delivered

- **WHEN** `madswan/planner/2`'s carried status waits on its caller with `why` `delivered`
- **THEN** the pull gives a done of `madswan/planner/2`, with the id `madswan/planner/2/done`

### Requirement: A launch is let go once its work has left crew's plan

A launch SHALL leave the runner's files, and be named in the report's let-go, once Flume no longer lists its place and its work has left `plan.rec` at `<label>/main`'s tip. `unit/<u>` and `chore/<c>` SHALL leave with the plan's `Unit` record of that name, and `fix/<bolt>/<name>` with its `Bolt` record. `fix/main/…` and `role/…` work SHALL keep its last record. A role's earlier run SHALL go once a later run is recorded and the earlier place is over.

#### Scenario: A unit landed and gone from the plan

- **WHEN** unit `u`'s place is tidied, and `plan.rec` no longer holds `Unit: u`
- **THEN** its `Place` and `Answer` leave the runner's branch, and the report's let-go names its launch

#### Scenario: A unit still in the plan

- **WHEN** unit `u`'s place is tidied, and `plan.rec` still holds `Unit: u`
- **THEN** its `Place` stays, with its last status

#### Scenario: A fix of main

- **WHEN** a `fix/main/crew/x` launch's place is tidied
- **THEN** its `Place` stays
