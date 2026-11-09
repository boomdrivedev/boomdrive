## ADDED Requirements

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

## MODIFIED Requirements

### Requirement: An act naming no host is given to the runner that holds its launch

A retry, land, confirm, drop or priority naming no `Host` SHALL be for this host only where the runner's report, or its own branch, answers the launch its `Of` names. Otherwise crew's implementation SHALL give nothing for it and write nothing of it.

#### Scenario: An act on another host's launch

- **WHEN** request 22 retries `Of: 20` naming no `Host`, and `home/runner/host-b` answers 20 while `host-a`'s report does not
- **THEN** `host-a`'s pull gives nothing for 22

#### Scenario: A land naming no host, of a launch this runner holds

- **WHEN** `host-a` claims launch 21, which names no `Host`, and request 24 says `Act: land` and `Of: 21`, naming no `Host`, while `home/runner/host-b` answers launch 20 and request 25 lands `Of: 20`, naming no `Host`
- **THEN** `host-a`'s pull gives `home/24`, landing `home/21`, and nothing for 25

#### Scenario: A confirm naming no host, of a launch this runner holds

- **WHEN** `host-a` claims launch 21, which names no `Host`, and request 26 confirms ends of `Of: 21` as `scratch/planner`, naming no `Host`, while `home/runner/host-b` answers launch 20 and request 27 confirms ends of `Of: 20`, naming no `Host`
- **THEN** `host-a`'s pull gives `home/26`, confirming ends of `home/21`, and nothing for 27
