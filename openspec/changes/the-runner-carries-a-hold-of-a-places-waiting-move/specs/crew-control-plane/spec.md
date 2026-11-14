## MODIFIED Requirements

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

## ADDED Requirements

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
