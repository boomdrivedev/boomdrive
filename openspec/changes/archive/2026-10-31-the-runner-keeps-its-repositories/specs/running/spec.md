# running

## Purpose

How `flywheel` runs on a host. It covers the version it prints, and `flywheel run`, the long-running process the host starts and keeps alive, one per data directory. It also covers the passes that process makes and when it stops, what it says, and the rule that no connection comes in to it (crew record 0052 part 2; core.3).

## ADDED Requirements

### Requirement: flywheel prints its version

`flywheel --version` and `flywheel -V` SHALL print `flywheel <version>` on standard output, the crate's version, and exit 0. `flywheel` with no command, or with one it does not know, SHALL print its usage on standard error and exit 2.

#### Scenario: The version

- **WHEN** `flywheel --version` runs
- **THEN** it prints `flywheel <the crate's version>` and exits 0

#### Scenario: No command

- **WHEN** `flywheel` runs with no arguments
- **THEN** it prints its usage on standard error and exits 2

### Requirement: flywheel run keeps the repositories until it is stopped

`flywheel run` SHALL make a pass over the configuration at once and then one pass every `fetch_every`, timed from the start of the pass before. Between passes it SHALL act on each landing it is told of. It SHALL run until it is sent SIGTERM or SIGINT. It SHALL then stop before the next repository it would keep and exit 0. A git call that fails SHALL be named and SHALL never stop it.

#### Scenario: Stopped while waiting

- **WHEN** `flywheel run` has made its first pass and is sent SIGTERM
- **THEN** it exits 0 within a second

#### Scenario: Passes on the timer

- **WHEN** `fetch_every` is two minutes and a pass began at 10:00:00
- **THEN** the next pass begins at 10:02:00, or at once when the pass before ends later than that

#### Scenario: A failure does not stop it

- **WHEN** a pass of `flywheel run` finds `agentplot/flume`'s fetch failing
- **THEN** the runner names it and goes on, keeping every other repository, and tries `agentplot/flume` again at the next pass

### Requirement: flywheel run --once makes one pass

`flywheel run --once` SHALL make one pass, just as a pass of `flywheel run`, and exit. It SHALL exit 0 when every git call of the pass succeeded, and 1 when any failed. Something left and named as the specs say, and not because a git call failed, SHALL not be a failure.

#### Scenario: Every repository kept

- **WHEN** `flywheel run --once` clones and fetches every configured repository and makes their views
- **THEN** it exits 0

#### Scenario: A clone failed

- **WHEN** `flywheel run --once` cannot clone one configured repository
- **THEN** it keeps the others and exits 1

#### Scenario: Left for the user

- **WHEN** `flywheel run --once` leaves a dropped repository held by a branch the forge lacks, and every git call succeeds
- **THEN** it exits 0

### Requirement: One runner per data directory

`flywheel run`, with or without `--once`, SHALL hold an exclusive lock on its data directory for as long as it runs. Whichever starts while another holds the lock SHALL exit 2, naming the data directory as held, and touch nothing.

#### Scenario: A second runner

- **WHEN** `flywheel run` is running and `flywheel run --once` starts on the same data directory
- **THEN** the second exits 2 naming the data directory as held by a running runner, and makes, fetches, moves and removes nothing

### Requirement: flywheel run refuses to start rather than run blind

`flywheel run` SHALL exit 2 before it touches anything, naming the reason, when: its configuration cannot be read; `HOME` is unset; git cannot be run; or git is older than 2.48, the first release whose fetch keeps origin's HEAD current.

#### Scenario: An old git

- **WHEN** `git --version` on the host says `git version 2.47.1`
- **THEN** `flywheel run` exits 2 naming the version found and 2.48 as the least it runs with

#### Scenario: A bad configuration at start

- **WHEN** `flywheel run` starts with a configuration that names `flume` as a repository
- **THEN** it exits 2 naming the file and the entry, and clones nothing

### Requirement: The runner says what it does, one line each

The runner SHALL write one line on standard error for each thing it does to a clone or a view: a clone made, a branch moved, a view made, moved or removed, a dropped repository removed. It SHALL also write one line for each thing it names, prefixed with the repository's address. It SHALL say when it starts, with its version, configuration file, data directory and views root, and when it stops. A pass in which nothing changed and nothing is named SHALL write nothing.

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
