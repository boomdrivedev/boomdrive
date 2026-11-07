## ADDED Requirements

### Requirement: A bolt is a launch

The runner SHALL take a launch of a kind that lands with no stages, as a bolt's base place is, as it takes any launch: under its lease, for a repository it keeps, while its stream has room as Flume's status gives it, counted against that room within the pass, and as one `flume launch` naming what the launch gives. It SHALL read no kind to tell such a launch from another, and SHALL ask no tool to make, move or remove a branch (core.9, core.15, core.52; Flume's 202).

#### Scenario: A bolt and its unit

- **WHEN** a pull offers `madswan/bolt/3` of kind `bolt` named `x` in `afterthought/crew` on `main`, then `madswan/14` of kind `change` named `a` on `bolt/x`, both with stream `madswan`, whose `room` is 2, and the clone is made
- **THEN** the stub `flume` is run as `flume --json launch --repository <the container> --kind bolt --name x --base main --stream madswan --mark boomdrive:madswan/bolt/3`, then as `flume --json launch --repository <the container> --kind change --name a --base bolt/x --stream madswan --mark boomdrive:madswan/14`; both are answered `launched`, and no `sluice` command is run

#### Scenario: A bolt counts against its stream's room within its pass

- **WHEN** a pull offers the bolt `madswan/bolt/3`, then its unit `madswan/14`, both with stream `madswan`, whose `room` is 1
- **THEN** the bolt is launched and the unit is left unanswered; and at the next pass, whose listing gives `madswan` a `room` of 1 again, the unit is launched

### Requirement: A bolt's send is a retry of its submission

The runner SHALL carry out the send of a place whose kind lands with no stages, a retry naming the submission, as `flume retry <place> --submission`, as it carries out any retry. Flume's answer SHALL be the act's, a refusal while a place on its branch is live among them (core.1, core.11, core.52; Flume's 203).

#### Scenario: A bolt's send

- **WHEN** an act retries the submission of launch `madswan/bolt/3`, which was launched at place `P`
- **THEN** the stub `flume` is run as `flume --json retry P --submission`, and the act is answered `done`

#### Scenario: A send Flume refuses

- **WHEN** an act retries the submission of launch `madswan/bolt/3` at place `P`, and the stub `flume` exits 1 refusing it with the reason `the places on bolt/x are live: Q`
- **THEN** the act is answered `refused` with the words `the places on bolt/x are live: Q`, and no `sluice` command is run

## MODIFIED Requirements

### Requirement: Work under a lapsed lease waits for a live one

By the runner's clock at the start of a control pass, an offer whose lease has expired SHALL be neither launched, held nor refused, an act on a launch whose lease in the report has expired SHALL not be carried out, and a command whose lease has expired SHALL not be answered. Each SHALL wait until a pull gives it under a live lease. An answer already given SHALL stand (core.10, core.24).

#### Scenario: An offer whose lease lapsed

- **WHEN** the fake control plane offers launch `a` under a lease that expired a minute before the pass
- **THEN** no `flume launch` is run, `a` has no answer in the report, and once a pull offers `a` under a live lease it is launched

#### Scenario: An act on a lapsed launch

- **WHEN** the runner holds launch `a` at place `P` under a lease that has expired, and the fake control plane asks a retry of `a`
- **THEN** no `flume` command names `P`, and the retry has no answer in the report

#### Scenario: A command whose lease lapsed

- **WHEN** the fake control plane gives a branch command under a lease that has expired
- **THEN** the command has no answer in the report and the pass says nothing; and once a pull gives it under a live lease, it is answered `refused`, naming sluice, and no `sluice` command is run

### Requirement: The statuses are carried against Flume's listing as it stands once the wants are answered

The statuses' listing SHALL be Flume's listing as it stands once the pull's wants are answered. Where answering them ran a `flume launch` or an act of Flume's, it SHALL be a `flume --json status` read then. Where it ran neither, it SHALL be the pass's listing (record 0027). A command runs no tool, so it SHALL NOT make the listing be read again.

#### Scenario: A place tidied while the pass answers its wants

- **WHEN** a carried place `P` stands live in the pass's listing, as carried, and while the pass launches another offer Flume tidies `P` landed, keeping its record `P@<at>`
- **THEN** Flume's listing is read again after the launch, and that pass carries Flume's answer for `P` as its status, naming `P@<at>`, tidied and landed

#### Scenario: A quiet pass reads the listing once

- **WHEN** a pass's wants run no `flume` command
- **THEN** it runs `flume --json status` once, before its pull

#### Scenario: A pass that answers a command reads the listing once

- **WHEN** the runner holds launch `a` at a place Flume stands, and a pull gives the offer `a` and a branch command, and no other want
- **THEN** the command is answered `refused`, naming sluice, and the pass runs `flume --json status` once, before its pull
