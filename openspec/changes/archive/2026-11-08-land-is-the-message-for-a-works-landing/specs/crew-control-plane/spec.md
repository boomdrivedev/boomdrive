## RENAMED Requirements

- FROM: `### Requirement: A retry, a drop and a priority are acts on the launch they name`
- TO: `### Requirement: A retry, a land, a drop and a priority are acts on the launch they name`

## MODIFIED Requirements

### Requirement: A retry, a land, a drop and a priority are acts on the launch they name

A `retry` SHALL be given as an act on the launch `<label>/<Of>` with its `Stage` and `Words`, asking for the submission where it holds a `Submission` field. A `land` SHALL be given as an act landing `<label>/<Of>`, and crew's implementation SHALL read no other field of it. A `drop` SHALL be an act dropping `<label>/<Of>`. A `priority` SHALL be an act setting `<label>/<Of>`'s priority to `Priority` (core.53; crew record 0056).

#### Scenario: A retry of the submission

- **WHEN** request 14 retries `Of: 12` with `Submission: yes`
- **THEN** the pull gives an act `home/14` on `home/12` asking for the submission

#### Scenario: A land

- **WHEN** request 17 says `Act: land` and `Of: 12`, naming this host
- **THEN** the pull gives an act `home/17` landing `home/12` under lease 1

### Requirement: An act naming no host is given to the runner that holds its launch

A retry, land, drop or priority naming no `Host` SHALL be for this host only where the runner's report, or its own branch, answers the launch its `Of` names. Otherwise crew's implementation SHALL give nothing for it and write nothing of it.

#### Scenario: An act on another host's launch

- **WHEN** request 22 retries `Of: 20` naming no `Host`, and `home/runner/host-b` answers 20 while `host-a`'s report does not
- **THEN** `host-a`'s pull gives nothing for 22

#### Scenario: A land naming no host, of a launch this runner holds

- **WHEN** `host-a` claims launch 21, which names no `Host`, and request 24 says `Act: land` and `Of: 21`, naming no `Host`, while `home/runner/host-b` answers launch 20 and request 25 lands `Of: 20`, naming no `Host`
- **THEN** `host-a`'s pull gives `home/24`, landing `home/21`, and nothing for 25
