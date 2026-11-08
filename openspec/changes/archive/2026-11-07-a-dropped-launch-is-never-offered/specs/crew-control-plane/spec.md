# crew's Control Plane

## MODIFIED Requirements

### Requirement: Each request for this host is a want under the lease number 1

crew's implementation SHALL give each request for this host, in `Request` order, as a want with the id `<label>/<Request>` and the lease number 1, with no expiry. A request naming a `Host` SHALL be for the host it names, and for no other. A launch naming no `Host`, and an act on one, SHALL be for this host as the requirements below give them. A request whose work has left crew's plan, and a launch a drop names, SHALL be given only as the requirements below give them.

#### Scenario: Only this host's requests

- **WHEN** `requests.rec` holds launches naming `Host: host-a` and launches naming `Host: host-b`, and the host is `host-a`
- **THEN** the pull gives `host-a`'s alone, in `Request` order, each under lease 1

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

### Requirement: A launch the runner answered is given whatever the hello names

A launch naming no `Host` that the runner's report or this host's branch answers, a claim apart, SHALL be given at every pull whatever the hello names, so that its place is followed to its end, save a dropped launch the runner has not launched. Its repository SHALL be the address its `Repo` names among those the hello names where there is one, else its `Repo` as written. It SHALL be neither claimed nor named in the log.

#### Scenario: A launch answered before its repository left the hello

- **WHEN** `home/runner/host-a` answers launch 20 `launched`, launch 20 names no `Host` and says `Repo: old-kit`, and the hello names `example/new-kit` alone
- **THEN** the pull gives the offer `home/20`, its repository `old-kit`, writes nothing, and says nothing

#### Scenario: An answered launch whose name two kept addresses share

- **WHEN** the runner's report answers launch 20 `launched`, launch 20 names no `Host` and says `Repo: crew`, and the hello names `example/crew` and `agentplot/crew`
- **THEN** the pull gives the offer `home/20`, its repository `crew`

## ADDED Requirements

### Requirement: A request whose work has left crew's plan is given only while the runner holds it

A request whose `Work` has left `plan.rec` at `<label>/main`'s tip SHALL be given only where the runner's report or this host's branch answers it, a claim apart, or as the drop's requirement below gives it. `unit/<u>` and `chore/<u>` work SHALL have left the plan once it holds no `Unit` record of that name, and `fix/<bolt>/<name>` once it holds no `Bolt` record of `<bolt>`. `fix/main/…`, `role/…` and any other work SHALL never have left it.

#### Scenario: Work gone that the runner does not hold

- **WHEN** request 12 launches `unit/u` naming this host, no runner answers it, and `plan.rec` holds no `Unit: u`
- **THEN** the pull gives nothing for 12

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
