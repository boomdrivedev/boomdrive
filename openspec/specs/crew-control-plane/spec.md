# crew-control-plane Specification

## Purpose
crew's implementation of the control-plane interface, the one crate that names crew. It reads crew's records at one tip of each partition's drive branch, `<label>/main`: the requests crew writes, its roles that run as Flume kinds, its asks and its plan. It gives them to the runner as offers and acts under the lease number 1 with no expiry, since crew never moves work once a host's runner holds it. A request naming a host is that host's runner's. A launch naming none, a kit's work, is taken by a runner whose configuration keeps its repository, which claims it on its own branch and gives it only once no other runner's branch answers it. It writes the runner's answers, and Flume's status of each place, on the runner's own branch, `<label>/runner/<host>`, in crew's formats field for field, and pushes no other branch. It names no kinds file: a role's kind is given as its identity, which the host's Flume resolves (core.2, core.5, core.10; crew record 0052; crew's `teams-cross-to-flume-as-their-bolts-land` Decision 4 and `roles-run-as-flume-kinds` R1 to R4, as built at crew `9913f249`; crew's `the-team-machinery-leaves` Decision 4).

## Requirements

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

### Requirement: crew's records are read at one tip of each partition's drive branch

At each pull, for each partition, crew's implementation SHALL read `requests.rec`, `roles.rec`, `asks.rec` and `plan.rec` at one tip of `<label>/main`, through a bare cache of that partition's own, `<data>/control/crew/<owner>/<name>/<label>.git`. A file missing at the tip SHALL hold nothing. It SHALL read every `<label>/runner/*` branch the state repository lists, taking its own host's answers and places from the report the runner hands the pull (record 0026).

#### Scenario: A file missing at the tip

- **WHEN** `home/main`'s tip holds `requests.rec` and no `roles.rec`
- **THEN** the pull gives the requests for this host, and offers no role's run

#### Scenario: Two partitions of one state repository

- **WHEN** crew's table names the partitions `home` and `work`, both at `example/state`
- **THEN** `home`'s part reads through `<data>/control/crew/example/state/home.git`, `work`'s through `<data>/control/crew/example/state/work.git`, and neither writes in the other's

### Requirement: A partition that cannot be read hides only its own work

Each partition SHALL be a part of its own. A part whose `<label>/main` cannot be read SHALL give `Unread`, with its words, while every other part pulls on its own. A part's report SHALL write its partition's runner branch, with its let-go, and SHALL give `Unsent`, naming the branch, where it could not write it.

#### Scenario: One partition missing

- **WHEN** partition `work`'s state repository has no `work/main` and `home`'s holds requests for this host
- **THEN** `home`'s part gives `home`'s requests, and `work`'s part is unread, naming `work/main`

#### Scenario: No partition read

- **WHEN** no partition's state repository can be read
- **THEN** every part is unread, each naming its own

#### Scenario: One partition missing at a report

- **WHEN** this host is `host-a`, partition `work`'s state repository has no `work/main`, and `home`'s unit `u` is tidied and gone from its plan
- **THEN** `home`'s part writes `home/runner/host-a`, its let-go names `u`'s launch, and `work`'s part's report is unsent, naming `work/runner/host-a`

### Requirement: Each request for this host is a want under the lease number 1

crew's implementation SHALL give each request for this host, in `Request` order, as a want with the id `<label>/<Request>` and the lease number 1, with no expiry. A request naming a `Host` SHALL be for the host it names, and for no other. A launch naming no `Host`, and an act on one, SHALL be for this host as the requirements below give them. A request whose work has left crew's plan, and a launch a drop names, SHALL be given only as the requirements below give them.

#### Scenario: Only this host's requests

- **WHEN** `requests.rec` holds launches naming `Host: host-a` and launches naming `Host: host-b`, and the host is `host-a`
- **THEN** the pull gives `host-a`'s alone, in `Request` order, each under lease 1

### Requirement: A launch naming no host is given only where this runner keeps its repository

A launch naming no `Host` that no runner answers SHALL be for this host only where its `Repo` is a repository the runner keeps: an address among the hello's repositories, or a name alone that exactly one of them has as its name. Where the runner keeps no such repository, crew's implementation SHALL give nothing for the launch and write nothing of it, so that another host's runner may take it. Where two addresses the hello names share the name, it SHALL give nothing and name both in its log.

#### Scenario: A kit named by its name alone

- **WHEN** a launch naming no `Host` says `Repo: crew`, and the hello names `example/crew` and `agentplot/flume`
- **THEN** the launch is for this host, its repository `example/crew`

#### Scenario: A repository not kept

- **WHEN** a launch naming no `Host` says `Repo: agentplot/sluice`, and the hello does not name it
- **THEN** the pull gives nothing for it, and the runner's branch holds no answer to it

### Requirement: A launch naming no host is claimed before it is given

A launch naming no `Host` that no runner's branch answers and no drop names SHALL be claimed: crew's implementation SHALL write its branch with the launch's `Answer` `held`, its words saying this host's runner claims it, one write for every claim of the pull. It SHALL then read every other `<label>/runner/*` branch again, give each claimed launch no other branch answers as an offer, and withdraw each another branch answers, writing its branch without that claim and giving nothing for it.

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

A claim SHALL stand on the runner's branch until the runner's report answers its launch, or until a drop names its launch, when the claim SHALL leave at the branch's next write. At every later pull, a launch the runner's report answers SHALL be given, save a dropped launch the runner has not launched; one only its own claim answers SHALL be given while no other branch answers it and no drop names it; and one its own claim and another branch both answer SHALL be withdrawn.

#### Scenario: A claim waiting for its stream's room

- **WHEN** `host-a` holds the claim of launch 20 and the runner leaves the offer unanswered for want of room in its stream
- **THEN** the report's write keeps request 20's claim on `home/runner/host-a`, and the next pull gives the offer again

#### Scenario: A claim found again after a stop

- **WHEN** `host-a`'s runner stopped between its claim of launch 20 and its check, and starts again with `home/runner/host-b` answering 20 as well
- **THEN** `host-a`'s first pull withdraws its claim of 20 and gives nothing for it

#### Scenario: A claimed launch dropped

- **WHEN** `home/runner/host-a` holds `host-a`'s claim of launch 20, the runner left the offer unanswered for want of room, and request 21, naming no `Host`, drops 20
- **THEN** `host-a`'s pull gives nothing for 20 or 21, and its next report's write leaves `home/runner/host-a` holding no answer to 20

### Requirement: A withdrawn launch is claimed again in the name order of its claimants

A launch crew's implementation withdrew SHALL be claimed again only while no other branch answers it, and no sooner than as many pulls later as its host's place, counting from 1, in the name order of the hosts whose claims it saw.

#### Scenario: Claimed again in name order

- **WHEN** `host-a` and `host-b` each withdrew launch 20, each having seen the other's claim
- **THEN** `host-a` claims 20 again at its next pull, and `host-b` not before the pull after its next

### Requirement: An act naming no host is given to the runner that holds its launch

A retry, land, confirm, hold, drop or priority naming no `Host` SHALL be for this host only where the runner's report, or its own branch, answers the launch its `Of` names. Otherwise crew's implementation SHALL give nothing for it and write nothing of it.

#### Scenario: An act on another host's launch

- **WHEN** request 22 retries `Of: 20` naming no `Host`, and `home/runner/host-b` answers 20 while `host-a`'s report does not
- **THEN** `host-a`'s pull gives nothing for 22

#### Scenario: A land naming no host, of a launch this runner holds

- **WHEN** `host-a` claims launch 21, which names no `Host`, and request 24 says `Act: land` and `Of: 21`, naming no `Host`, while `home/runner/host-b` answers launch 20 and request 25 lands `Of: 20`, naming no `Host`
- **THEN** `host-a`'s pull gives `home/24`, landing `home/21`, and nothing for 25

#### Scenario: A confirm naming no host, of a launch this runner holds

- **WHEN** `host-a` claims launch 21, which names no `Host`, and request 26 confirms ends of `Of: 21` as `scratch/planner`, naming no `Host`, while `home/runner/host-b` answers launch 20 and request 27 confirms ends of `Of: 20`, naming no `Host`
- **THEN** `host-a`'s pull gives `home/26`, confirming ends of `home/21`, and nothing for 27

#### Scenario: A hold naming no host, of a launch this runner holds

- **WHEN** `host-a` claims launch 21, which names no `Host`, and request 26 holds `Of: 21` under `Words: the user sends it` with `Bound: 28800`, naming no `Host`, while `home/runner/host-b` answers launch 20 and request 27 holds `Of: 20` the same way, naming no `Host`
- **THEN** `host-a`'s pull gives `home/26`, holding `home/21` under `the user sends it` for 28800 seconds, and nothing for 27

### Requirement: A launch request is an offer of what it names

A `launch` SHALL be given as an offer whose launch holds its repository, its `Kind`, `Name`, `Base`, `Stream` where it names one, `Priority`, `Intent`, each `Recipient` and `Brief`, each `After` as the launch `<label>/<n>`, and `Hold: submission` as a held submission. Its kind SHALL be `Kind` as crew wrote it.

#### Scenario: A launch's every field

- **WHEN** request 12 is a launch for this host naming every field, with a `Brief` over three `+` lines and `After` 10 and 11
- **THEN** the pull gives an offer `home/12`, its brief the three lines joined by newlines, coming after `home/10` and `home/11`

#### Scenario: A kit's launch naming no stream

- **WHEN** request 20 is a launch naming no `Host` and no `Stream`, claimed by this host
- **THEN** the offer `home/20` names no stream, so the launch runs in the stream the host's Flume configuration maps its repository to

### Requirement: A retry, a land, a drop and a priority are acts on the launch they name

A `retry` SHALL be given as an act on the launch `<label>/<Of>` with its `Stage` and `Words`, asking for the submission where it holds a `Submission` field. A `land` SHALL be given as an act landing `<label>/<Of>`, and crew's implementation SHALL read no other field of it. A `drop` SHALL be an act dropping `<label>/<Of>`. A `priority` SHALL be an act setting `<label>/<Of>`'s priority to `Priority` (core.53; crew record 0056).

#### Scenario: A retry of the submission

- **WHEN** request 14 retries `Of: 12` with `Submission: yes`
- **THEN** the pull gives an act `home/14` on `home/12` asking for the submission

#### Scenario: A land

- **WHEN** request 17 says `Act: land` and `Of: 12`, naming this host
- **THEN** the pull gives an act `home/17` landing `home/12` under lease 1

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

A launch SHALL leave the runner's files, and be named in the report's let-go, once Flume no longer lists its place and its work has left `plan.rec` at `<label>/main`'s tip. `unit/<u>` and `chore/<c>` SHALL leave with the plan's `Unit` record of that name, and `bolt/<bolt>` and `fix/<bolt>/<name>` with its `Bolt` record. `fix/main/…` and `role/…` work SHALL keep its last record. A role's earlier run SHALL go once a later run is recorded and the earlier place is over.

#### Scenario: A unit landed and gone from the plan

- **WHEN** unit `u`'s place is tidied, and `plan.rec` no longer holds `Unit: u`
- **THEN** its `Place` and `Answer` leave the runner's branch, and the report's let-go names its launch

#### Scenario: A unit still in the plan

- **WHEN** unit `u`'s place is tidied, and `plan.rec` still holds `Unit: u`
- **THEN** its `Place` stays, with its last status

#### Scenario: A bolt landed and gone from the plan

- **WHEN** bolt `b`'s base place, a launch of `bolt/b`, is tidied, and `plan.rec` no longer holds `Bolt: b`
- **THEN** its `Place` and `Answer` leave the runner's branch, and the report's let-go names its launch and its act

#### Scenario: A fix of main

- **WHEN** a `fix/main/crew/x` launch's place is tidied
- **THEN** its `Place` stays

### Requirement: A repository draining gives no launch no runner answered

A launch naming no `Host` that no runner answers SHALL not be for this host where its `Repo` names a repository the hello names only as drained, and crew's implementation SHALL give nothing for it and write nothing of it.

#### Scenario: A repository draining

- **WHEN** a launch naming no `Host` says `Repo: example/kit`, no runner answers it, and the hello names `example/kit` only as drained
- **THEN** the pull gives nothing for it, and the runner's branch holds no answer to it

### Requirement: A launch the runner answered is given while the runner keeps or drains its repository

A launch naming no `Host` that the runner's report or this host's branch answers, a claim apart, save a dropped launch the runner has not launched, SHALL be given at every pull while its `Repo` names a repository the hello names: one among its repositories, else one among those it drains, each read as an unanswered launch's `Repo` is read. Its repository SHALL be that address, and it SHALL be neither claimed nor named in the log.

#### Scenario: A launch answered in a repository draining

- **WHEN** `home/runner/host-a` answers launch 20 `launched`, launch 20 names no `Host` and says `Repo: kit`, and the hello names `example/other` among its repositories and `example/kit` as drained
- **THEN** the pull gives the offer `home/20`, its repository `example/kit`, writes nothing, and says nothing

### Requirement: No old name of a repository is matched to a new one

Where neither the hello's repositories nor those it drains name the `Repo` of a launch the runner answered, crew's implementation SHALL give nothing for the launch, write nothing of it and say nothing. It SHALL match no name a `Repo` once had to another.

#### Scenario: A launch answered before its repository was renamed

- **WHEN** the runner's report answers launch 20 `launched`, launch 20 names no `Host` and says `Repo: example/old-kit`, and the hello names `example/new-kit` alone
- **THEN** the pull gives nothing for launch 20, writes nothing, and says nothing

### Requirement: A launch naming this host takes its repository from those kept, then those drained

A launch naming this host SHALL take as its repository the address its `Repo` names among the hello's repositories, else among those it drains, else its `Repo` as written.

#### Scenario: A launch naming this host in a repository draining

- **WHEN** request 12 is a launch naming `Host: host-a` and saying `Repo: kit`, and the hello names `example/kit` as drained
- **THEN** the pull gives the offer `home/12`, its repository `example/kit`

### Requirement: A request whose work has left crew's plan is given only while the runner holds it

A request whose `Work` has left `plan.rec` at `<label>/main`'s tip SHALL be given only where the runner's report or this host's branch answers it, a claim apart, or as the drop's requirement below, or a done's, gives it. `unit/<u>` and `chore/<u>` work SHALL have left the plan once it holds no `Unit` record of that name, and `bolt/<bolt>` and `fix/<bolt>/<name>` once it holds no `Bolt` record of `<bolt>`. `fix/main/…`, `role/…` and any other work SHALL never have left it.

#### Scenario: Work gone that the runner does not hold

- **WHEN** request 12 launches `unit/u` naming this host, no runner answers it, and `plan.rec` holds no `Unit: u`
- **THEN** the pull gives nothing for 12

#### Scenario: A preview of a bolt gone that the runner does not hold

- **WHEN** request 30 asks for a preview of `bolt/y`, naming this host, no runner answers it, and `plan.rec` holds no `Bolt: y`
- **THEN** the pull gives nothing for 30

### Requirement: A drop is given to the runner that holds its launch whatever the plan holds

A drop naming this host or none SHALL be given, whatever `plan.rec` holds, where the runner's report or this host's branch answers the launch its `Of` names, a claim apart. It SHALL then be given by the rules of a request naming this host, or of an act naming no `Host`.

#### Scenario: A drop written as its work leaves the plan

- **WHEN** the runner's report answers launch 12 of `unit/u` `launched` at place `P`, and request 14, naming this host, drops 12 at a tip whose `plan.rec` holds no `Unit: u`
- **THEN** the pull gives the offer `home/12`, then the act `home/14` dropping it

#### Scenario: A kit's drop goes to the runner holding its launch

- **WHEN** launch 20 of `unit/u` names no `Host`, `home/runner/host-a` answers it `launched`, and request 21, naming no `Host`, drops 20 at a tip whose `plan.rec` holds no `Unit: u`
- **THEN** `host-a`'s pull gives the offer `home/20` and the act `home/21`, and `host-b`'s pull gives nothing for 20 or 21

### Requirement: A launch is dropped as crew reads it

A launch SHALL be dropped where a request of `requests.rec` at `<label>/main`'s tip says `Act: drop` and names the launch's number as its `Of`, whatever that request's `Host`, its `Work` or its place in the file, as crew's own `standing` reads it.

#### Scenario: A drop naming another host

- **WHEN** launch 12 names this host, the runner's report answers it `held` on its After, and request 14, naming `Host: host-b`, drops 12
- **THEN** the pull gives no offer of `home/12`, and nothing for 14

### Requirement: A launch a drop names is offered only where the runner launched it

crew's implementation SHALL give a dropped launch as an offer only where the runner's report answers it `launched`, or, where the report holds no answer to it, this host's branch does. Otherwise it SHALL give no offer for it and claim it not, whatever `Host` it names and whatever `plan.rec` holds. An act naming no `Host` on a dropped launch that only this host's claim answers SHALL be given nothing.

#### Scenario: A held launch dropped

- **WHEN** launch 12 names this host, the runner's report answers it `held` on its After, and request 14, naming this host, drops 12
- **THEN** the pull gives the act `home/14` and no offer of `home/12`

#### Scenario: A kit's launch dropped before it is claimed

- **WHEN** launch 20 names no `Host` and says `Repo: example/crew`, which the hello names, request 21, naming no `Host`, drops 20, no runner's branch answers 20, and the host has room
- **THEN** the pull claims nothing, writes no branch, and gives nothing for 20 or 21

#### Scenario: A dropped launch the runner launched

- **WHEN** the runner's report answers launch 12 `launched` at place `P`, its place listed, and request 14, naming this host, drops 12 at a tip whose `plan.rec` still holds the launch's unit
- **THEN** the pull gives the offer `home/12` and the act `home/14`

### Requirement: A dropped launch the runner holds unlaunched is let go

At each report, a launch the runner's report holds that a drop names, answered `held`, `refused` or `done`, SHALL leave the runner's files and be named in the report's let-go, whatever `plan.rec` holds. A dropped launch answered `launched` SHALL leave only once Flume no longer lists its place and its work has left the plan, as any launch does. An act on a dropped launch SHALL leave once its launch has left and its own work has left the plan.

#### Scenario: A held launch dropped is let go

- **WHEN** the runner's report answers launch 12 of `unit/u` `held`, request 14 drops 12, and `plan.rec` still holds `Unit: u`
- **THEN** the report's let-go names `home/12`, and `home/runner/host-a` holds no answer to 12

#### Scenario: A drop answered from the report, its work gone

- **WHEN** the runner's report answers launch 12 of `unit/u` `done` with words naming its drop, and act 14, which drops 12, `done`, and `plan.rec` holds no `Unit: u`
- **THEN** the report's let-go names `home/12` and `home/14`

#### Scenario: A dropped launch the runner launched stays until its place is over

- **WHEN** the runner's report answers launch 12 of `unit/u` `launched` at place `P`, and act 14, which drops 12, `done`, and `plan.rec` holds no `Unit: u`
- **THEN** while Flume lists `P`, the report's let-go names neither `home/12` nor `home/14`; and once Flume no longer lists `P`, it names both

### Requirement: The shared cache of a release before is removed

At open, crew's implementation SHALL remove the shared cache a release before kept for a state repository, `<data>/control/crew/<owner>/<name>.git`, where it is there, saying one line naming it. A cache is made again from the control plane (core.35).

#### Scenario: The shared cache of a release before

- **WHEN** crew's implementation opens with `<data>/control/crew/example/state.git` there
- **THEN** that cache is removed, one line names it, and each partition's pull fetches into its own cache

### Requirement: A preview request is a preview of what it names

A `preview` request naming this host SHALL be given as a preview with the id `<label>/<Request>` and the lease number 1, with no expiry: its repository its `Repo`, resolved as a launch naming this host resolves it, its `Branch` and its `Revision`. It SHALL be done where its `Done` reads `yes`, `true` or `1`, and not done where it reads `no`, `false` or `0`, or where it has none (core.55; crew record 0056).

#### Scenario: A preview

- **WHEN** request 30 says `Act: preview`, naming this host, `Repo: crew`, `Branch: main` and `Revision: R1`, and the hello names `example/crew`
- **THEN** the pull gives a preview `home/30` of `main` at `R1` in `example/crew`, not done, under lease 1

#### Scenario: A preview marked done

- **WHEN** request 31 says `Act: preview`, naming this host, with the same `Repo`, `Branch` and `Revision` as request 30, and `Done: yes`, and the runner's report holds request 30 answered `done`
- **THEN** the pull gives a preview `home/31` of `main` at `R1` in `example/crew`, done

### Requirement: A preview request crew's implementation cannot read is refused by its host's runner

A `preview` request naming this host that lacks its `Branch` or its `Revision`, or whose `Done` reads anything but `yes`, `true`, `1`, `no`, `false` or `0`, SHALL be given as one the runner refuses, with words naming the field.

#### Scenario: A preview with no revision

- **WHEN** request 32 says `Act: preview`, naming this host, with no `Revision`
- **THEN** the pull gives it as one the runner answers `refused`, with words naming `Revision`

### Requirement: A preview naming no host is given nothing

A `preview` request naming no `Host` SHALL be given nothing, written nothing of, and named in the log, since a preview is brought up only on the host its control plane names and two runners keeping its repository would each bring it up.

#### Scenario: A preview naming no host

- **WHEN** request 33 says `Act: preview`, naming no `Host`, of a repository the hello names
- **THEN** the pull gives nothing for it, the runner's branch holds no answer to it, and a line names request 33 as a preview that names no Host

### Requirement: A preview the runner answered is the runner's

A preview the runner's report holds, or this host's `previews.rec` answers, SHALL be the runner's, and SHALL be given while its work has left crew's plan, as any request the runner answered is.

#### Scenario: A preview whose work has left the plan

- **WHEN** the runner's report holds preview `home/30` of `unit/u`, answered `done`, and `plan.rec` holds no `Unit: u`
- **THEN** the pull gives the preview `home/30`

### Requirement: A preview's answer is written in previews.rec

crew's implementation SHALL write `previews.rec` on the runner's branch, under crew's descriptor of a `Preview`, one `Preview` per preview request the report answers. Each SHALL hold its `Request` and `Outcome`; sluice's answer as carried as its `Text`; its `Words`; sluice's listing of it as carried as its `Status`, with `Listed` `yes` or `no`; and its `At`, the status's time where a status is carried, else the answer's. The file SHALL pass `recfix --check`.

#### Scenario: A preview brought up

- **WHEN** the runner reports preview `home/30` answered `done`, its text and its status carried listed
- **THEN** `home/runner/host-a`'s `previews.rec` opens with crew's descriptor of a `Preview`, byte for byte, and holds one `Preview` with `Request: 30`, `Outcome: done`, `Text` and `Status` as carried, byte for byte, `Listed: yes` and the status's time as its `At`, and the file passes `recfix --check`

### Requirement: previews.rec is written only where a preview is or was

crew's implementation SHALL write `previews.rec` where the partition's report holds a preview, and as the descriptor alone where the tip holds one and the report none. Otherwise the runner's branch SHALL hold no `previews.rec`.

#### Scenario: A branch with no preview

- **WHEN** the runner first reports for `home`, its report holding a launch and no preview
- **THEN** `home/runner/host-a` holds no `previews.rec`

### Requirement: A preview is read back from previews.rec

`previews.rec` SHALL be read with the runner's other files at each pull, and checked as they are: no record before its `%rec`, and no request twice. Where the runner hands the pull no report of its own, the report the pull gives back SHALL hold each `Preview` of this host's branch as the preview its request names, under lease 1. Its repository SHALL be resolved as at a pull, its outcome, text and words as written, and its status the `Status`, at its `At`, listed as `Listed` says.

#### Scenario: A preview read back

- **WHEN** preview `home/30` has been reported `done`, its status listed, and a pull is handed no report
- **THEN** the pull's report holds `home/30` answered `done`, its text, its status listed and its repository as reported

### Requirement: A preview is let go once its work has left crew's plan and it holds no preview up

At each report, a bring-up the report holds SHALL leave the runner's files, and be named in the report's let-go, once its work has left `plan.rec` at `<label>/main`'s tip and it holds no preview up: a bring-up refused, or one brought up or held whose status sluice's listing no longer holds. A bring-up whose status sluice still lists SHALL stay, whatever the plan holds, until a done puts it away. A done SHALL go as the requirement of a done answered gives it.

#### Scenario: A preview put away, its work gone

- **WHEN** the runner's report holds preview `home/30` of `unit/u` answered `done`, its status not listed, and its done `home/31` answered `done`, and `plan.rec` holds no `Unit: u`
- **THEN** the report's let-go names `home/30` and `home/31`, and `home/runner/host-a`'s `previews.rec` holds neither

#### Scenario: A preview sluice no longer lists, its work gone

- **WHEN** the runner's report holds preview `home/30` of `unit/u` answered `done`, its status not listed, and `plan.rec` holds no `Unit: u`
- **THEN** the report's let-go names `home/30`, and `home/runner/host-a`'s `previews.rec` does not hold it

#### Scenario: A preview still up, its work gone

- **WHEN** the runner's report holds preview `home/30` of `unit/u` answered `done`, its status listed, and `plan.rec` holds no `Unit: u`
- **THEN** the report's let-go does not name `home/30`, and `previews.rec` still holds it

#### Scenario: A refused preview, its bolt landed

- **WHEN** the runner's report holds preview `home/30` of `bolt/y` answered `refused`, and `plan.rec` holds no `Bolt: y`
- **THEN** the report's let-go names `home/30`

#### Scenario: A refused preview, its bolt still in the plan

- **WHEN** the runner's report holds preview `home/30` of `bolt/y` answered `refused`, and `plan.rec` holds `Bolt: y`
- **THEN** the report's let-go does not name `home/30`

### Requirement: A preview request the runner does not hold is given only while it is crew's newest of its host, repository and revision

A `preview` request naming this host that crew's implementation reads, and that neither the runner's report nor this host's `previews.rec` answers, SHALL be given only while it is the newest such request naming this host, its `Repo` and its `Revision`, as crew keys its asks. One not marked done, a bring-up, SHALL then be given only while its work is in crew's plan (core.55; crew record 0056; record 0031 as amended on 2026-11-09).

#### Scenario: A preview and its done the runner does not hold

- **WHEN** request 30 asks for a preview of `main` at `R1` of `unit/u`, naming this host, request 31 is its done, `plan.rec` holds `Unit: u`, and neither the runner's report nor this host's branch answers either
- **THEN** the pull gives nothing for 30 or 31

#### Scenario: A done written before a later bring-up

- **WHEN** the runner's report holds request 30, a preview of `main` at `R1` of `unit/u`, answered `done`, request 31 is its done, which no runner answers, request 32 asks for the preview of `main` at `R1` again, and `plan.rec` holds `Unit: u`
- **THEN** the pull gives `home/30` and `home/32`, and nothing for 31

### Requirement: A done is given while the runner holds a bring-up of it, whatever the plan holds

A done that the runner does not hold, and that is crew's newest of its host, repository and revision, SHALL be given whatever `plan.rec` holds, but only while the runner's report or this host's `previews.rec` answers a bring-up naming this host, its `Repo` and its `Revision` (record 0031 as amended on 2026-11-09).

#### Scenario: A done of a landed bolt's preview

- **WHEN** the runner's report holds request 30, a preview of `main` at `R1` of `bolt/y`, answered `done` with its status listed, request 31 is its done, and `plan.rec` holds no `Bolt: y`
- **THEN** the pull gives `home/30`, and `home/31` done

### Requirement: A done answered is let go with the previews it put away

At each report, a done the runner's report holds SHALL leave the runner's files, and be named in the report's let-go, whatever `plan.rec` holds. With it SHALL go each bring-up the report holds whose request names this host and the done's `Repo` and `Revision`, and was written before the done (record 0031 as amended on 2026-11-09).

#### Scenario: A preview put away, its work still in the plan

- **WHEN** the runner's report holds preview `home/30` of `unit/u` answered `done`, and its done `home/31` answered `done`, and `plan.rec` holds `Unit: u`
- **THEN** the report's let-go names `home/30` and `home/31`, and `home/runner/host-a`'s `previews.rec` holds neither

#### Scenario: A bring-up written after the done stays

- **WHEN** the runner's report holds preview `home/30` of `unit/u` answered `done`, its done `home/31` answered `done`, and `home/32`, a later bring-up of the same `Repo` and `Revision`, answered `done`, and `plan.rec` holds `Unit: u`
- **THEN** the report's let-go names `home/30` and `home/31` and not `home/32`, and `previews.rec` still holds request 32

### Requirement: A confirm request is an act confirming the deliveries it names, as the recipient it names

A `confirm` SHALL be given as an act on the launch `<label>/<Of>` confirming each `Delivery` it holds, in the order written, as its one `Recipient`, each as crew wrote it, and crew's implementation SHALL read no other field of it (core.56; crew's change `crews-role-runs-confirm-what-they-collect`).

#### Scenario: A confirm

- **WHEN** request 17 says `Act: confirm`, `Of: 12`, `Recipient: scratch/planner` and two `Delivery` lines, naming this host
- **THEN** the pull gives an act `home/17` on `home/12` under lease 1, confirming the two keys, in the order written, as `scratch/planner`

### Requirement: A confirm naming no recipient, two, or no delivery cannot be read

A confirm that names no `Recipient`, names two, or names no `Delivery` SHALL be one crew's implementation cannot read. Where it names this host, it SHALL be given as one the runner refuses, with words naming the field. Where it names no `Host`, it SHALL be given nothing and named in the log.

#### Scenario: A confirm naming no recipient, two, or no delivery

- **WHEN** request 18 confirms `Of: 12` with a `Delivery` and no `Recipient`, request 19 confirms it as `scratch/planner` with no `Delivery`, and request 20 names two `Recipient` lines, each naming this host
- **THEN** the pull gives each as one the runner refuses, 18's and 20's words naming `Recipient` and 19's naming `Delivery`

### Requirement: A report's answer names taken each status places.rec holds

Once the runner's branch is written, or holds crew's files already, the answer to the report SHALL name taken each launch whose status `places.rec` holds, and no other (record 0030 as amended on 2026-11-09).

#### Scenario: A status places.rec holds is taken

- **WHEN** the report carries a place's status as a fixture's text, beside a request answered `held` with no status
- **THEN** `places.rec`'s `Status` for its request is that text, and the report's answer names that launch's status taken and not the held request's

### Requirement: No listing of the runner branches outlasts its limit

crew's implementation SHALL list `<label>/runner/*` by a `git ls-remote` run as every git call of the runner's to a remote is: in a process group of its own, stopped at thirty seconds with everything it started (record 0042; core.58).

#### Scenario: The listing's limit is thirty seconds

- **WHEN** crew's implementation makes the store of a partition
- **THEN** its listing's limit is thirty seconds

### Requirement: A listing stopped at its limit leaves its partition unread

A listing stopped at its limit SHALL fail with the words `git ls-remote with <url> was still going after <limit>, the most a try may take, and was stopped`, naming the state repository's URL, after git's last line on standard error where git wrote one. Its partition SHALL be unread with those words, and pulled again as any part that could not be read (record 0042).

#### Scenario: A state repository that never answers the listing

- **WHEN** crew's implementation lists `home/runner/*` of a state repository that takes the connection and never answers
- **THEN** the listing ends within its limit and grace and a margin, and fails with `git ls-remote with <url> was still going after <limit>, the most a try may take, and was stopped`, naming the state repository's URL

### Requirement: repositories.rec names exactly the repositories the partition's drive keeps

`repositories.rec` SHALL open with crew's descriptor of a `Repository`, byte for byte as crew's `runner.py` holds it. It SHALL hold one `Repository` per repository the hello names under the drive whose name is the partition's label, in the hello's order: its address as `Repository`, and the clone's container named with it as `Folder`. It SHALL name no repository of another drive, and none the runner drains (record 0028; record 0032; core.5, core.26).

#### Scenario: A drive's repositories and their folders

- **WHEN** the hello names the drive `home` with `example/crew` at `/data/repositories/example/crew` and `example/kit` at `/data/repositories/example/kit`, and the drive `work` with `example/app` at `/data/repositories/example/app`, and the runner reports for `home` on host `host-a`
- **THEN** `home/runner/host-a`'s `repositories.rec` opens with crew's descriptor of a `Repository`, byte for byte, holds `example/crew` then `example/kit`, each with its folder as its `Folder`, names no `example/app`, and passes `recfix --check`

#### Scenario: A launch in a repository another drive keeps

- **WHEN** the hello names the drive `home` with `example/crew` and the drive `work` with `example/app`, launch 20 of `home` names no `Host` and says `Repo: example/app`, no runner's branch answers it, and the host has room
- **THEN** `home`'s pull claims 20 and gives the offer `home/20`, its repository `example/app`, and `home/runner/host-a`'s `repositories.rec` names `example/crew` alone

### Requirement: repositories.rec is written at every write of the runner's branch

crew's implementation SHALL write `repositories.rec` at every write of `<label>/runner/<host>`: at a report, from the drives of the partition's latest pull, and at a pull that claims or withdraws, from the drives of the hello it was handed. A write's commit SHALL name each address whose record it adds, changes or removes.

#### Scenario: A repository the drive comes to keep

- **WHEN** `home/runner/host-a`'s `repositories.rec` names `example/crew` alone, and the next pull's hello names the drive `home` with `example/crew` and `example/kit`
- **THEN** the next report's commit names `repositories example/kit`, and the file holds both

#### Scenario: A repository the drive no longer names

- **WHEN** `home/runner/host-a`'s `repositories.rec` names `example/crew` and `example/kit`, and the next pull's hello names the drive `home` with `example/crew` alone and `example/kit` as drained
- **THEN** the next report's commit names `repositories example/kit`, and the file holds `example/crew` alone

#### Scenario: A claim names the drive's repositories

- **WHEN** `host-a` has no branch of `home`, and its pull claims launch 20, the hello naming the drive `home` with `example/crew`
- **THEN** the claim's write makes `home/runner/host-a` holding request 20's claim and a `repositories.rec` naming `example/crew` with its folder

### Requirement: A partition whose label names no drive names no repository

Where the hello names no drive by the partition's label, crew's implementation SHALL write `repositories.rec` as crew's descriptor alone, and SHALL say one line naming the label and each drive the hello names, once while it stands. It SHALL give and claim launches as it does for any partition.

#### Scenario: A label no drive has

- **WHEN** crew's table serves the partition `home` on host `host-a`, and the hello names the drives `family` and `work` alone
- **THEN** `home/runner/host-a`'s `repositories.rec` is crew's descriptor alone, and one line says this host's configuration names no drive `home`, naming `family` and `work`

### Requirement: No try of weir-branch's call to a state repository outlasts its limit

crew's implementation SHALL read `<label>/main` and each runner's branch, and write its own, by a weir-branch that runs each try of its fetch, listing and push to `weir-bounded`'s limit, thirty seconds, stopped with everything it started. A stopped read SHALL leave its partition unread, and a stopped write its report unsent, each with weir-branch's words (weir record 0008; record 0042 as amended on 2026-11-11; core.58).

#### Scenario: A state repository that never answers the read

- **WHEN** crew's implementation reads `home/main` of a state repository that takes the connection and never answers
- **THEN** the read ends within thirty seconds, its grace and a margin, and fails with words ending `git fetch with <url> was still going after 30s, the most a try may take, and was stopped`, naming the state repository's URL

#### Scenario: A state repository that never answers a write

- **WHEN** crew's implementation writes `home/runner/host-a` to a state repository that takes the connection and never answers
- **THEN** the write ends within thirty seconds, its grace and a margin, having formed nothing, and fails with words ending `git fetch with <url> was still going after 30s, the most a try may take, and was stopped`, naming the state repository's URL

### Requirement: A hold request is an act holding the move its launch's place waits on

A `hold` SHALL be given as an act on the launch `<label>/<Of>` holding the move its place waits on under its `Words`, as crew wrote them, for its `Bound` in whole seconds, and crew's implementation SHALL read no other field of it (core.59; crew's record 0067 as amended on 2026-11-14).

#### Scenario: A hold

- **WHEN** request 17 says `Act: hold`, `Of: 12`, `Words: the user sends it on the landings card, row 3` and `Bound: 28800`, naming this host
- **THEN** the pull gives an act `home/17` on `home/12` under lease 1, holding the move under those words for 28800 seconds

### Requirement: A hold naming no words, no bound, or a bound of no whole seconds cannot be read

A hold that names no `Words`, names no `Bound`, or names a `Bound` that is no whole number of seconds SHALL be one crew's implementation cannot read. Where it names this host, it SHALL be given as one the runner refuses, with words naming the field. Where it names no `Host`, it SHALL be given nothing and named in the log.

#### Scenario: A hold naming no words, no bound, or a bound of no whole seconds

- **WHEN** request 18 holds `Of: 12` with `Bound: 60` and no `Words`, request 19 holds it with `Words` and no `Bound`, and request 20 holds it with `Words` and `Bound: 8h`, each naming this host
- **THEN** the pull gives each as one the runner refuses, 18's words naming `Words`, 19's naming `Bound`, and 20's naming `Bound` and `8h`
