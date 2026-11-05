# running Specification

## Purpose
How `boomdrive` runs on a host. It covers the version it prints, and `boomdrive run`, the long-running process the host starts and keeps alive, one per data directory. It also covers the passes that process makes and when it stops, what it says, and the rule that no connection comes in to it (crew record 0052 part 2; core.3).

## Requirements

### Requirement: boomdrive prints its version

`boomdrive --version` and `boomdrive -V` SHALL print `boomdrive <version>` on standard output, the crate's version, and exit 0. `boomdrive` with no command, or with one it does not know, SHALL print its usage on standard error and exit 2.

#### Scenario: The version

- **WHEN** `boomdrive --version` runs
- **THEN** it prints `boomdrive <the crate's version>` and exits 0

#### Scenario: No command

- **WHEN** `boomdrive` runs with no arguments
- **THEN** it prints its usage on standard error and exits 2

### Requirement: boomdrive run keeps the repositories until it is stopped

`boomdrive run` SHALL make a pass over the configuration at once and then one pass every `fetch_interval`, timed from the start of the pass before. Between passes it SHALL act on each landing it is told of. It SHALL run until it is sent SIGTERM or SIGINT. It SHALL then stop before the next repository it would keep and exit 0. A git call that fails SHALL be named and SHALL never stop it.

#### Scenario: Stopped while waiting

- **WHEN** `boomdrive run` has made its first pass and is sent SIGTERM
- **THEN** it exits 0 within a second

#### Scenario: Passes on the timer

- **WHEN** `fetch_interval` is two minutes and a pass began at 10:00:00
- **THEN** the next pass begins at 10:02:00, or at once when the pass before ends later than that

#### Scenario: A failure does not stop it

- **WHEN** a pass of `boomdrive run` finds `agentplot/flume`'s fetch failing
- **THEN** the runner names it and goes on, keeping every other repository, and tries `agentplot/flume` again at the next pass

### Requirement: boomdrive run --once makes one pass

`boomdrive run --once` SHALL make one keeping pass, just as a pass of `boomdrive run`, then, where the configuration names a control plane, one control pass, and exit. It SHALL exit 0 when every git call of the keeping pass succeeded and the control pass both pulled and reported, and 1 otherwise. Something left and named as the specs say SHALL not be a failure: an offer left untaken for want of room, and a tool the control pass could not run or read, among it.

#### Scenario: Every repository kept

- **WHEN** `boomdrive run --once` clones and fetches every configured repository and makes their views
- **THEN** it exits 0

#### Scenario: A clone failed

- **WHEN** `boomdrive run --once` cannot clone one configured repository
- **THEN** it keeps the others and exits 1

#### Scenario: Left for the user

- **WHEN** `boomdrive run --once` leaves a dropped repository held by a branch the forge lacks, and every git call succeeds
- **THEN** it exits 0

#### Scenario: One control pass

- **WHEN** `boomdrive run --once` runs with crew's control plane over a scratch state repository holding a launch for this host, and a stub `flume` that launches it
- **THEN** it keeps the repositories, launches once, writes the runner's branch, and exits 0

#### Scenario: Flume's status refused

- **WHEN** `boomdrive run --once` runs with crew's control plane over a scratch state repository holding a launch for this host, and a stub `flume` that refuses its status
- **THEN** it names Flume's status as unread, launches nothing, writes the runner's branch, and exits 0

#### Scenario: A control plane that cannot be read

- **WHEN** `boomdrive run --once` runs with crew's control plane and no partition's state repository can be read
- **THEN** it keeps the repositories, names the control plane as unread, runs no `flume` command but the hello's `flume --json status` and `flume --version`, and exits 1

### Requirement: One runner per data directory

`boomdrive run`, with or without `--once`, SHALL hold an exclusive lock on its data directory for as long as it runs. Whichever starts while another holds the lock SHALL exit 2, naming the data directory as held, and touch nothing.

#### Scenario: A second runner

- **WHEN** `boomdrive run` is running and `boomdrive run --once` starts on the same data directory
- **THEN** the second exits 2 naming the data directory as held by a running runner, and makes, fetches, moves and removes nothing

### Requirement: boomdrive run refuses to start rather than run blind

`boomdrive run` SHALL exit 2 before it touches anything, naming the reason, when: its configuration cannot be read; `HOME` is unset; git cannot be run; or git is older than 2.48, the first release whose fetch keeps origin's HEAD current.

#### Scenario: An old git

- **WHEN** `git --version` on the host says `git version 2.47.1`
- **THEN** `boomdrive run` exits 2 naming the version found and 2.48 as the least it runs with

#### Scenario: A bad configuration at start

- **WHEN** `boomdrive run` starts with a configuration that names `flume` as a repository
- **THEN** it exits 2 naming the file and the entry, and clones nothing

### Requirement: The runner says what it does, one line each

The runner SHALL write one line on standard error for each thing it does to a clone or a view: a clone made, a branch moved, a view made, moved or removed, a dropped repository removed. It SHALL also write one line for each thing it names, prefixed with the repository's address. It SHALL say when it starts, with its version, configuration file, data directory and repository root, and when it stops. A pass in which nothing changed and nothing is named SHALL write nothing.

#### Scenario: A quiet pass

- **WHEN** a pass finds origin unchanged for every kept repository and every view where the runner last put it
- **THEN** the pass writes nothing

#### Scenario: A view moved

- **WHEN** a pass moves the view of `agentplot/flume` to a new `origin/main`
- **THEN** it writes one line beginning `agentplot/flume:` naming the view moved and the commit

### Requirement: No connection comes in to the runner

The runner SHALL open no listening socket of any kind. Every connection it makes SHALL be outgoing, git's fetch over https among them (core.3).

#### Scenario: No listener in the code

- **WHEN** the crate's sources are read
- **THEN** none names `TcpListener`, `UnixListener`, `UdpSocket` or a `bind` of a socket, and the crate depends on no HTTP or WebSocket server

### Requirement: boomdrive run makes control passes on a timer of their own

Where the configuration names a control plane, `boomdrive run` SHALL make a control pass after its first keeping pass, and the next `poll_interval` after the start of the one before. After a pass that reported, it SHALL make it sooner, by the midpoint between that pass's end and the earliest expiry of a lease it holds not yet lapsed. A lapsed lease, or a pass that did not report, SHALL hasten none. Control and keeping passes SHALL run on the one loop, one at a time (core.22).

#### Scenario: On the control timer

- **WHEN** `poll_interval` is one minute, `fetch_interval` two minutes, and a control pass began at 10:00:00
- **THEN** the next control pass begins at 10:01:00, and the keeping passes keep their own two minutes

#### Scenario: A lapsed lease

- **WHEN** `poll_interval` is one minute, the runner holds a lease that lapsed before a control pass that began at 10:00:00, and the control plane cannot be reached
- **THEN** the next control pass begins at 10:01:00, and none runs between

#### Scenario: A pass that did not report

- **WHEN** `poll_interval` is one minute, the runner holds a lease expiring 40 seconds after a control pass that began at 10:00:00, and that pass's report was not sent
- **THEN** the next control pass begins at 10:01:00

### Requirement: A control pass runs at once when its control plane wakes the runner

`boomdrive run` SHALL make a control pass at once when its control plane's implementation wakes it, through the wake its opener was handed, and SHALL time the next one from that pass's start.

#### Scenario: Woken by the control plane

- **WHEN** the implementation calls the wake its opener was handed, between two control passes
- **THEN** a control pass runs at once, and the timer's next pass is `poll_interval` after its start

### Requirement: A control pass after a broken configuration acts on nothing

A control pass that follows a reading of the configuration that failed SHALL name the file and act on nothing, until the file reads again.

#### Scenario: The configuration broken while running

- **WHEN** the configuration of a running runner carrying crew's work is made invalid
- **THEN** the next control pass names the file and acts on nothing, and once the file is valid again, the pass after acts as before

### Requirement: A control plane that cannot be read is said once until it is read again

A control pass whose pull cannot be read SHALL say `the control plane could not be read`, with the pull's words, only where the pass before read it, or where it is the first. A pass that reads it after one that could not SHALL say `the control plane is read again`. Passes between SHALL say nothing of it (core.23).

#### Scenario: Three passes unread, then read

- **WHEN** the fake control plane refuses three pulls in a row with the words `no network`, then gives the fourth
- **THEN** one line names the control plane as unread with `no network`, and one line, at the fourth pass, says it is read again
