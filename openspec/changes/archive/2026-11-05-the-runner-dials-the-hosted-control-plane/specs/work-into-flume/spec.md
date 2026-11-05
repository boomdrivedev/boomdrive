# Work into Flume

## ADDED Requirements

### Requirement: Work under a lapsed lease waits for a live one

By the runner's clock at the start of a control pass, an offer whose lease has expired SHALL be neither launched, held nor refused, an act on a launch whose lease in the report has expired SHALL not be carried out, and a command whose lease has expired SHALL not be taken. Each SHALL wait until a pull gives it under a live lease. An answer already given SHALL stand (core.10, core.24).

#### Scenario: An offer whose lease lapsed

- **WHEN** the fake control plane offers launch `a` under a lease that expired a minute before the pass
- **THEN** no `flume launch` is run, `a` has no answer in the report, and once a pull offers `a` under a live lease it is launched

#### Scenario: An act on a lapsed launch

- **WHEN** the runner holds launch `a` at place `P` under a lease that has expired, and the fake control plane asks a retry of `a`
- **THEN** no `flume` command names `P`, and the retry has no answer in the report

#### Scenario: A command whose lease lapsed

- **WHEN** the fake control plane gives a branch command under a lease that has expired
- **THEN** no `sluice` command is run, and the command has no answer in the report
