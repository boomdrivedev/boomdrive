# Running

## MODIFIED Requirements

### Requirement: flywheel run keeps the repositories until it is stopped

`flywheel run` SHALL make a pass over the configuration at once and then one pass every `fetch_interval`, timed from the start of the pass before. Between passes it SHALL act on each landing it is told of. It SHALL run until it is sent SIGTERM or SIGINT. It SHALL then stop before the next repository it would keep and exit 0. A git call that fails SHALL be named and SHALL never stop it.

#### Scenario: Stopped while waiting

- **WHEN** `flywheel run` has made its first pass and is sent SIGTERM
- **THEN** it exits 0 within a second

#### Scenario: Passes on the timer

- **WHEN** `fetch_interval` is two minutes and a pass began at 10:00:00
- **THEN** the next pass begins at 10:02:00, or at once when the pass before ends later than that

#### Scenario: A failure does not stop it

- **WHEN** a pass of `flywheel run` finds `agentplot/flume`'s fetch failing
- **THEN** the runner names it and goes on, keeping every other repository, and tries `agentplot/flume` again at the next pass

### Requirement: flywheel run --once makes one pass

`flywheel run --once` SHALL make one keeping pass, just as a pass of `flywheel run`, then, where the configuration names a control plane, one control pass, and exit. It SHALL exit 0 when every git call of the keeping pass succeeded and the control pass both pulled and reported, and 1 otherwise. Something left and named as the specs say, and not because a git call failed, SHALL not be a failure: an offer left untaken for want of room among it.

#### Scenario: Every repository kept

- **WHEN** `flywheel run --once` clones and fetches every configured repository and makes their views
- **THEN** it exits 0

#### Scenario: A clone failed

- **WHEN** `flywheel run --once` cannot clone one configured repository
- **THEN** it keeps the others and exits 1

#### Scenario: Left for the user

- **WHEN** `flywheel run --once` leaves a dropped repository held by a branch the forge lacks, and every git call succeeds
- **THEN** it exits 0

#### Scenario: One control pass

- **WHEN** `flywheel run --once` runs with crew's control plane over a scratch state repository holding a launch for this host, and a stub `flume` that launches it
- **THEN** it keeps the repositories, launches once, writes the runner's branch, and exits 0

#### Scenario: A control plane that cannot be read

- **WHEN** `flywheel run --once` runs with crew's control plane and no partition's state repository can be read
- **THEN** it keeps the repositories, names the control plane as unread, runs no `flume` command, and exits 1

### Requirement: The runner says what it does, one line each

The runner SHALL write one line on standard error for each thing it does to a clone or a view: a clone made, a branch moved, a view made, moved or removed, a dropped repository removed. It SHALL also write one line for each thing it names, prefixed with the repository's address. It SHALL say when it starts, with its version, configuration file, data directory and repository root, and when it stops. A pass in which nothing changed and nothing is named SHALL write nothing.

#### Scenario: A quiet pass

- **WHEN** a pass finds origin unchanged for every kept repository and every view where the runner last put it
- **THEN** the pass writes nothing

#### Scenario: A view moved

- **WHEN** a pass moves the view of `agentplot/flume` to a new `origin/main`
- **THEN** it writes one line beginning `agentplot/flume:` naming the view moved and the commit

## ADDED Requirements

### Requirement: flywheel run makes control passes on a timer of their own

Where the configuration names a control plane, `flywheel run` SHALL make a control pass after its first keeping pass. It SHALL make the next one `poll_interval` after the start of the one before, or sooner, by the midpoint between the end of the pass before and the earliest expiry of a lease it holds. Control passes and keeping passes SHALL run on the runner's one loop, one at a time.

#### Scenario: On the control timer

- **WHEN** `poll_interval` is one minute, `fetch_interval` two minutes, and a control pass began at 10:00:00
- **THEN** the next control pass begins at 10:01:00, and the keeping passes keep their own two minutes

### Requirement: A control pass runs at once when its control plane wakes the runner

`flywheel run` SHALL make a control pass at once when its control plane's implementation wakes it, and SHALL time the next one from that pass's start.

#### Scenario: Woken by the control plane

- **WHEN** the implementation wakes the runner between two control passes
- **THEN** a control pass runs at once, and the timer's next pass is `poll_interval` after its start

### Requirement: A control pass after a broken configuration acts on nothing

A control pass that follows a reading of the configuration that failed SHALL name the file and act on nothing, until the file reads again.

#### Scenario: The configuration broken while running

- **WHEN** the configuration of a running runner carrying crew's work is made invalid
- **THEN** the next control pass names the file and acts on nothing, and once the file is valid again, the pass after acts as before
