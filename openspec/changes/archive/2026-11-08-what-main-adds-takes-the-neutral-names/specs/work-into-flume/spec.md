## MODIFIED Requirements

### Requirement: A bolt is a launch

The runner SHALL take a launch of a kind that lands with no stages, as a bolt's base place is, as it takes any launch: under its lease, for a repository it keeps, while its stream has room as Flume's status gives it, counted against that room within the pass, and as one `flume launch` naming what the launch gives. It SHALL read no kind to tell such a launch from another, and SHALL ask no tool to make, move or remove a branch (core.9, core.15, core.52; Flume's 202).

#### Scenario: A bolt and its unit

- **WHEN** a pull offers `home/bolt/3` of kind `bolt` named `x` in `example/crew` on `main`, then `home/14` of kind `change` named `a` on `bolt/x`, both with stream `home`, whose `room` is 2, and the clone is made
- **THEN** the stub `flume` is run as `flume --json launch --repository <the container> --kind bolt --name x --base main --stream home --mark boomdrive:home/bolt/3`, then as `flume --json launch --repository <the container> --kind change --name a --base bolt/x --stream home --mark boomdrive:home/14`; both are answered `launched`, and no `sluice` command is run

#### Scenario: A bolt counts against its stream's room within its pass

- **WHEN** a pull offers the bolt `home/bolt/3`, then its unit `home/14`, both with stream `home`, whose `room` is 1
- **THEN** the bolt is launched and the unit is left unanswered; and at the next pass, whose listing gives `home` a `room` of 1 again, the unit is launched

### Requirement: A bolt's send is a retry of its submission

The runner SHALL carry out the send of a place whose kind lands with no stages, a retry naming the submission, as `flume retry <place> --submission`, as it carries out any retry. Flume's answer SHALL be the act's, a refusal while a place on its branch is live among them (core.1, core.11, core.52; Flume's 203).

#### Scenario: A bolt's send

- **WHEN** an act retries the submission of launch `home/bolt/3`, which was launched at place `P`
- **THEN** the stub `flume` is run as `flume --json retry P --submission`, and the act is answered `done`

#### Scenario: A send Flume refuses

- **WHEN** an act retries the submission of launch `home/bolt/3` at place `P`, and the stub `flume` exits 1 refusing it with the reason `the places on bolt/x are live: Q`
- **THEN** the act is answered `refused` with the words `the places on bolt/x are live: Q`, and no `sluice` command is run
