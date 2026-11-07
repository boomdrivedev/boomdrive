# Control Plane Interface

## MODIFIED Requirements

### Requirement: The runner's report is reported whole

A report SHALL take the runner's whole report: each launch with its lease number, the repository it was answered in where the runner recorded one, its answer and carried statuses, and each act, command and inbox, by id. A report file or a message that names no repository for a launch SHALL read as recording none. A second report of the same report SHALL change nothing the control plane holds.

#### Scenario: A report sent twice

- **WHEN** the runner reports the same report twice to the fake control plane
- **THEN** the fake control plane's last report is the same after the second report as after the first

#### Scenario: A report from before

- **WHEN** a report's JSON names no repository for launch `a`, and a hello's JSON names no drained repository
- **THEN** each reads, `a` recording no repository and the hello naming none drained, and each round-trips through JSON unchanged
