## MODIFIED Requirements

### Requirement: The settle's bound is the runner's interval

The bound of the settle SHALL be written `<n>s`, in whole seconds: `fetch_interval`, or the shorter of `fetch_interval` and `poll_interval` while the runner makes control passes, `fetch_interval` being the one the latest reading of the configuration gave (record 0029).

#### Scenario: The shorter interval

- **WHEN** `boomdrive run --once` runs with crew's control plane, `fetch_interval` two minutes and `poll_interval` one minute
- **THEN** its settle is `flume --json settle --every 60s`

#### Scenario: A control plane that could not be opened

- **WHEN** the account `own` names the hosted control plane, whose table names no `url`, and the configuration names no `fetch_interval`
- **THEN** `boomdrive run --once` runs no `flume` command but `flume --json settle --every 120s`

## ADDED Requirements

### Requirement: The parts of every plane the accounts name are carried side by side

`boomdrive run` SHALL open, when it starts, each implementation an account names, crew's then the hosted one's, and carry every part of each on a thread of its own, as it carries one implementation's parts. No part of one plane SHALL wait on a part of another. Two parts of one label SHALL be refused at the start, naming the label and both implementations (record 0054; core.19, core.49).

#### Scenario: crew and a cloud side by side

- **WHEN** the account `own` names crew, whose partition `home` offers a launch in `agentplot/flume`, and the account `cloud` names the hosted control plane, whose partition `cloud` offers a launch in `example/app`, each stream with room
- **THEN** both launches are made, `home`'s reported to crew and `cloud`'s to the hosted control plane

#### Scenario: One label in two planes

- **WHEN** crew's table and the hosted table each name the partition `home`
- **THEN** `boomdrive run` refuses to start, exiting 2, naming `home`, `crew` and `hosted`

## REMOVED Requirements

### Requirement: Each part's observer is handed its control passes, and no pass waits on it

**Reason**: Record 0048 is superseded by record 0054.
**Migration**: None; no runner reports to an observer.
