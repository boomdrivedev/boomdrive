## ADDED Requirements

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

## MODIFIED Requirements

### Requirement: A preview request is a preview of what it names

A `preview` request naming this host SHALL be given as a preview with the id `<label>/<Request>` and the lease number 1, with no expiry: its repository its `Repo`, resolved as a launch naming this host resolves it, its `Branch` and its `Revision`. It SHALL be done where its `Done` reads `yes`, `true` or `1`, and not done where it reads `no`, `false` or `0`, or where it has none (core.55; crew record 0056).

#### Scenario: A preview

- **WHEN** request 30 says `Act: preview`, naming this host, `Repo: crew`, `Branch: main` and `Revision: R1`, and the hello names `example/crew`
- **THEN** the pull gives a preview `home/30` of `main` at `R1` in `example/crew`, not done, under lease 1

#### Scenario: A preview marked done

- **WHEN** request 31 says `Act: preview`, naming this host, with the same `Repo`, `Branch` and `Revision` as request 30, and `Done: yes`, and the runner's report holds request 30 answered `done`
- **THEN** the pull gives a preview `home/31` of `main` at `R1` in `example/crew`, done

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

### Requirement: A request whose work has left crew's plan is given only while the runner holds it

A request whose `Work` has left `plan.rec` at `<label>/main`'s tip SHALL be given only where the runner's report or this host's branch answers it, a claim apart, or as the drop's requirement below, or a done's, gives it. `unit/<u>` and `chore/<u>` work SHALL have left the plan once it holds no `Unit` record of that name, and `bolt/<bolt>` and `fix/<bolt>/<name>` once it holds no `Bolt` record of `<bolt>`. `fix/main/…`, `role/…` and any other work SHALL never have left it.

#### Scenario: Work gone that the runner does not hold

- **WHEN** request 12 launches `unit/u` naming this host, no runner answers it, and `plan.rec` holds no `Unit: u`
- **THEN** the pull gives nothing for 12

#### Scenario: A preview of a bolt gone that the runner does not hold

- **WHEN** request 30 asks for a preview of `bolt/y`, naming this host, no runner answers it, and `plan.rec` holds no `Bolt: y`
- **THEN** the pull gives nothing for 30

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
