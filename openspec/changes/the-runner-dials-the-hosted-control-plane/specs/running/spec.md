# Running

## MODIFIED Requirements

### Requirement: flywheel run makes control passes on a timer of their own

Where the configuration names a control plane, `flywheel run` SHALL make a control pass after its first keeping pass, and the next `poll_interval` after the start of the one before. After a pass that reported, it SHALL make it sooner, by the midpoint between that pass's end and the earliest expiry of a lease it holds not yet lapsed. A lapsed lease, or a pass that did not report, SHALL hasten none. Control and keeping passes SHALL run on the one loop, one at a time (core.22).

#### Scenario: On the control timer

- **WHEN** `poll_interval` is one minute, `fetch_interval` two minutes, and a control pass began at 10:00:00
- **THEN** the next control pass begins at 10:01:00, and the keeping passes keep their own two minutes

#### Scenario: A lapsed lease

- **WHEN** `poll_interval` is one minute, the runner holds a lease that lapsed before a control pass that began at 10:00:00, and the control plane cannot be reached
- **THEN** the next control pass begins at 10:01:00, and none runs between

#### Scenario: A pass that did not report

- **WHEN** `poll_interval` is one minute, the runner holds a lease expiring 40 seconds after a control pass that began at 10:00:00, and that pass's report was not sent
- **THEN** the next control pass begins at 10:01:00

## ADDED Requirements

### Requirement: A control plane that cannot be read is said once until it is read again

A control pass whose pull cannot be read SHALL say `the control plane could not be read`, with the pull's words, only where the pass before read it, or where it is the first. A pass that reads it after one that could not SHALL say `the control plane is read again`. Passes between SHALL say nothing of it (core.23).

#### Scenario: Three passes unread, then read

- **WHEN** the fake control plane refuses three pulls in a row with the words `no network`, then gives the fourth
- **THEN** one line names the control plane as unread with `no network`, and one line, at the fourth pass, says it is read again
