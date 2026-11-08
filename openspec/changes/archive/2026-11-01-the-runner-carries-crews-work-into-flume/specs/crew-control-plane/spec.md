# crew's control plane

## Purpose

crew's implementation of the control-plane interface, the one crate that names crew. It reads crew's records at one tip of each partition's flywheel branch, `<label>/main`: the requests crew writes, its roles that run as Flume kinds, its asks and its plan. It gives them to the runner as offers and acts under the lease number 1 with no expiry, since crew never moves work once a host's runner holds it. A request naming a host is that host's runner's. A launch naming none, a kit's work, is taken by a runner whose configuration keeps its repository, which claims it on its own branch and gives it only once no other runner's branch answers it. It writes the runner's answers, and Flume's status of each place, on the runner's own branch, `<label>/runner/<host>`, in crew's formats field for field, and pushes no other branch. It names no kinds file: a role's kind is given as its identity, which the host's Flume resolves (core.2, core.5, core.10; crew record 0052; crew's `teams-cross-to-flume-as-their-bolts-land` Decision 4 and `roles-run-as-flume-kinds` R1 to R4, as built at crew `9913f249`; crew's `the-team-machinery-leaves` Decision 4).

## ADDED Requirements

### Requirement: crew's table names the host and the partitions

crew's implementation SHALL open from `[control_plane.crew]`:

- `host`, crew's name for this host, required, with no default;
- `partitions`, at least one, each a `label` given once and a `state_repository` as `owner/name`.

A missing or malformed key SHALL be refused, naming the key and the file. Any other key SHALL be refused as one crew's implementation does not read; a refusal of `kinds` SHALL say kinds are Flume's host configuration (core.5).

#### Scenario: The table read

- **WHEN** the table says `host = "host-a"` and one partition `home` with `state_repository = "example/home-state"`
- **THEN** crew's implementation opens for host `host-a`, serving `home` from `example/home-state`

#### Scenario: No host

- **WHEN** the table names no `host`
- **THEN** it is refused, naming `host` and the configuration file

#### Scenario: A label twice

- **WHEN** two partitions say `label = "home"`
- **THEN** it is refused, naming `home`

#### Scenario: A kinds file named

- **WHEN** the table holds `kinds = { crew = "/k/kinds.toml" }`
- **THEN** it is refused, naming `kinds` as a key crew's implementation does not read and saying kinds are Flume's host configuration

### Requirement: A state repository is reached through the host's own credential helper

A state repository SHALL be reached at `https://github.com/<owner>/<name>.git` by the runner's git calls, through the host's git credential helper. crew's table SHALL hold no credential (core.4).

#### Scenario: A token in crew's table

- **WHEN** `[control_plane.crew]` says `token = "..."`
- **THEN** it is refused, naming `token` as a key crew's implementation does not read, and the value appears in no output

### Requirement: crew's records are read at one tip of each partition's flywheel branch

At each pull, for each partition, crew's implementation SHALL read `requests.rec`, `roles.rec`, `asks.rec` and `plan.rec` at one tip of `<label>/main`, through a bare cache at `<data>/control/crew/<owner>/<name>.git`. A file missing at the tip SHALL hold nothing. It SHALL read every `<label>/runner/*` branch the state repository lists, taking its own host's answers and places from the report the runner hands the pull.

#### Scenario: A file missing at the tip

- **WHEN** `home/main`'s tip holds `requests.rec` and no `roles.rec`
- **THEN** the pull gives the requests for this host, and offers no role's run

### Requirement: A partition that cannot be read hides only its own work

A partition whose `<label>/main` cannot be read SHALL give nothing that pass, with its words, while the other partitions give their own. A pull SHALL be unread only when no partition could be read. A report SHALL write each partition's runner branch it can, with its let-go, name each it could not, and be unsent only when it wrote none.

#### Scenario: One partition missing

- **WHEN** partition `work`'s state repository has no `work/main` and `home`'s holds requests for this host
- **THEN** the pull gives `home`'s requests and names `work/main` as unread

#### Scenario: No partition read

- **WHEN** no partition's state repository can be read
- **THEN** the pull is unread, naming each

#### Scenario: One partition missing at a report

- **WHEN** partition `work`'s state repository has no `work/main`, and `home`'s unit `u` is tidied and gone from its plan
- **THEN** the report writes `home/runner/host-a`, its let-go names `u`'s launch, and it names `work/runner/host-a` as not written

### Requirement: Each request for this host is a want under the lease number 1

crew's implementation SHALL give each request for this host, in `Request` order, as a want with the id `<label>/<Request>` and the lease number 1, with no expiry. A request naming a `Host` SHALL be for the host it names, and for no other. A launch naming no `Host`, and an act on one, SHALL be for this host as the requirements below give them.

#### Scenario: Only this host's requests

- **WHEN** `requests.rec` holds launches naming `Host: host-a` and launches naming `Host: host-b`, and the host is `host-a`
- **THEN** the pull gives `host-a`'s alone, in `Request` order, each under lease 1

### Requirement: A launch naming no host is given only where this runner keeps its repository

A launch naming no `Host` SHALL be for this host only where its `Repo` is a repository the runner keeps: an address the hello names, or a name alone that exactly one address the hello names has as its name. Where the runner keeps no such repository, crew's implementation SHALL give nothing for the launch and write nothing of it, so that another host's runner may take it. Where two addresses the hello names share the name, it SHALL give nothing and name both in its log.

#### Scenario: A kit named by its name alone

- **WHEN** a launch naming no `Host` says `Repo: crew`, and the hello names `example/crew` and `agentplot/flume`
- **THEN** the launch is for this host, its repository `example/crew`

#### Scenario: A repository not kept

- **WHEN** a launch naming no `Host` says `Repo: agentplot/sluice`, and the hello does not name it
- **THEN** the pull gives nothing for it, and the runner's branch holds no answer to it

### Requirement: A launch naming no host is claimed before it is given

A launch naming no `Host` that no runner's branch answers SHALL be claimed: crew's implementation SHALL write its branch with the launch's `Answer` `held`, its words saying this host's runner claims it, one write for every claim of the pull. It SHALL then read every other `<label>/runner/*` branch again, give each claimed launch no other branch answers as an offer, and withdraw each another branch answers, writing its branch without that claim and giving nothing for it.

#### Scenario: Claimed alone

- **WHEN** `host-a` and `host-b` both keep `example/crew`, launch 20 names no `Host` and says `Repo: example/crew`, no runner's branch answers it, and `host-a`'s host has room
- **THEN** `host-a`'s pull writes `home/runner/host-a` with request 20's `Answer` `held`, reads `home/runner/host-b` holding no answer to 20, and gives an offer `home/20`

#### Scenario: Another's claim seen

- **WHEN** after `host-a`'s claim of launch 20, `home/runner/host-b` holds an answer to 20
- **THEN** `host-a`'s pull writes its branch without the answer to 20 and gives nothing for it

#### Scenario: Taken elsewhere

- **WHEN** `home/runner/host-b` answers launch 20 before `host-a`'s pull
- **THEN** `host-a`'s pull writes nothing of 20 and gives nothing for it

### Requirement: A launch naming no host is claimed only while the host has room

crew's implementation SHALL claim a launch only while the hello's capacity gives the host room, its `room` `null` or above zero, so that a host with none leaves the launch for a host that has.

#### Scenario: No room on the host

- **WHEN** no runner's branch answers launch 20, and the hello's capacity gives `room` 0
- **THEN** the pull claims nothing and gives nothing for 20

### Requirement: A claim stands until the runner answers its launch

A claim SHALL stand on the runner's branch until the runner's report answers its launch. At every later pull, a launch the runner's report answers SHALL be given; one only its own claim answers SHALL be given while no other branch answers it; and one its own claim and another branch both answer SHALL be withdrawn.

#### Scenario: A claim waiting for its stream's room

- **WHEN** `host-a` holds the claim of launch 20 and the runner leaves the offer unanswered for want of room in its stream
- **THEN** the report's write keeps request 20's claim on `home/runner/host-a`, and the next pull gives the offer again

#### Scenario: A claim found again after a stop

- **WHEN** `host-a`'s runner stopped between its claim of launch 20 and its check, and starts again with `home/runner/host-b` answering 20 as well
- **THEN** `host-a`'s first pull withdraws its claim of 20 and gives nothing for it

### Requirement: A withdrawn launch is claimed again in the name order of its claimants

A launch crew's implementation withdrew SHALL be claimed again only while no other branch answers it, and no sooner than as many pulls later as its host's place, counting from 1, in the name order of the hosts whose claims it saw.

#### Scenario: Claimed again in name order

- **WHEN** `host-a` and `host-b` each withdrew launch 20, each having seen the other's claim
- **THEN** `host-a` claims 20 again at its next pull, and `host-b` not before the pull after its next

### Requirement: An act naming no host is given to the runner that holds its launch

A retry, drop or priority naming no `Host` SHALL be for this host only where the runner's report, or its own branch, answers the launch its `Of` names. Otherwise crew's implementation SHALL give nothing for it and write nothing of it.

#### Scenario: An act on another host's launch

- **WHEN** request 22 retries `Of: 20` naming no `Host`, and `home/runner/host-b` answers 20 while `host-a`'s report does not
- **THEN** `host-a`'s pull gives nothing for 22

### Requirement: A launch request is an offer of what it names

A `launch` SHALL be given as an offer whose launch holds its repository, its `Kind`, `Name`, `Base`, `Stream` where it names one, `Priority`, `Intent`, each `Recipient` and `Brief`, each `After` as the launch `<label>/<n>`, and `Hold: submission` as a held submission. Its kind SHALL be `Kind` as crew wrote it.

#### Scenario: A launch's every field

- **WHEN** request 12 is a launch for this host naming every field, with a `Brief` over three `+` lines and `After` 10 and 11
- **THEN** the pull gives an offer `home/12`, its brief the three lines joined by newlines, coming after `home/10` and `home/11`

#### Scenario: A kit's launch naming no stream

- **WHEN** request 20 is a launch naming no `Host` and no `Stream`, claimed by this host
- **THEN** the offer `home/20` names no stream, so the launch runs in the stream the host's Flume configuration maps its repository to

### Requirement: A retry, a drop and a priority are acts on the launch they name

A `retry` SHALL be given as an act on the launch `<label>/<Of>` with its `Stage` and `Words`, asking for the submission where it holds a `Submission` field. A `drop` SHALL be an act dropping `<label>/<Of>`. A `priority` SHALL be an act setting `<label>/<Of>`'s priority to `Priority`.

#### Scenario: A retry of the submission

- **WHEN** request 14 retries `Of: 12` with `Submission: yes`
- **THEN** the pull gives an act `home/14` on `home/12` asking for the submission

### Requirement: A request crew's implementation cannot read is refused by its host's runner

A request naming this host with a number that is not whole, an `Act` crew's implementation does not know, or a `Hold` other than `submission` SHALL be given as one the runner refuses, with words naming the field. Such a request naming no `Host` SHALL be given nothing and named in the log, since no one runner's answer to it is the one. A record with no `Request` number SHALL be left out and named, since it can be given no id.

#### Scenario: An act crew does not write

- **WHEN** request 15 names this host and says `Act: pause`
- **THEN** the pull gives it as one the runner answers `refused`, with words naming `Act` and `pause`

### Requirement: The runner's branch is written by weir's write, and only it is pushed

At each report, and at a pull that claims or withdraws, for each partition, crew's implementation SHALL write `<label>/runner/<host>` by weir-branch's write: formed against the branch's tip, each `.rec` file checked by `recfix --check`, one commit, and the branch moved only from that tip by a leased push, never forced. It SHALL write nothing when the files' text equals the tip's. It SHALL push no ref but `<label>/runner/<host>`.

#### Scenario: Nothing changed

- **WHEN** the runner reports the same report twice
- **THEN** the second report makes no commit

#### Scenario: Only the runner's branch

- **WHEN** a report has been written
- **THEN** the state repository's refs are those it held before, with `home/runner/host-a` beside them, and every file written passes `recfix --check`

### Requirement: The runner's branch holds crew's three files in crew's formats

crew's implementation SHALL write `runner.rec`, one `Runner` with `Host`, `Version` and `Since`, the first write's time, kept after. It SHALL write `answers.rec`, one `Answer` per request the report answers or a claim stands for. It SHALL write `places.rec`, one `Place` per launched request, then one `Run` per role, by `Address`. Each file SHALL open with crew's own descriptor, byte for byte as crew's `runner.py` holds it.

#### Scenario: The first report

- **WHEN** the runner first reports for `home` on host `host-a`
- **THEN** the state repository gains `home/runner/host-a`, holding `runner.rec` with `Host: host-a`, the runner's version and the time, and `home/main`'s tip is unchanged

#### Scenario: A place's status as Flume gave it

- **WHEN** the report carries a place's status as a fixture's text
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

- **WHEN** `home/runner/host-b`'s `places.rec` holds a place whose launch names `home/planner` as `Recipient` and whose status waits with `why` `answer`
- **THEN** the pull gives an offer of `planner`'s run

#### Scenario: A role's work landed

- **WHEN** a `role/planner/x` launch's place reads landed, and no ended turn took it
- **THEN** the pull gives an offer of `planner`'s run

### Requirement: A role's run is offered while it is wanted and none stands

crew's implementation SHALL offer a run of a wanted role whose latest run has no place reading `live` or `making`, as the launch `<label>/<role>/<k+1>`, where `k` is the highest the report holds of the role. Its launch SHALL be the role's `Repository`, its kind the identity `<Kinds>/<Kind>`, its `Name`, `Stream`, `Priority` and `Intent`, and the agent setting `model=<Model>`, with no base, brief or recipient, and no kinds file.

#### Scenario: A burst of asks

- **WHEN** `asks.rec` holds three asks of `planner` that no ended turn took, and no run of `planner` stands
- **THEN** the pull gives one offer, `home/planner/1`, of kind `crew/crew-planner` named `home`, its stream the role's, priority 0, intent `home-planner` and the agent setting `model=<the role's Model>`

#### Scenario: A run standing

- **WHEN** asks of `planner` are pending and its latest run, `home/planner/1`, reads `live` in its carried status
- **THEN** the pull gives `home/planner/1` again, the run the report holds, and offers no new run of `planner`

#### Scenario: A run waiting for room keeps its id

- **WHEN** an offer of `home/planner/3` is left untaken for want of room
- **THEN** the next pull offers `home/planner/3` again; and once the runner records it refused, the pull after offers `home/planner/4`

### Requirement: A role's run is declared done once its turn has delivered

Where a role's latest run's carried status waits on its caller with `why` `delivered`, crew's implementation SHALL give one act, `<label>/<role>/<k>/done`, declaring that launch done. A refusal of it SHALL be written as the run's `Words`.

#### Scenario: A turn delivered

- **WHEN** `home/planner/2`'s carried status waits on its caller with `why` `delivered`
- **THEN** the pull gives a done of `home/planner/2`, with the id `home/planner/2/done`

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
