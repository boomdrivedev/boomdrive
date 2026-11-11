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

`boomdrive run` SHALL make a pass over the configuration at once and then one every `fetch_interval`, timed from the start of the pass before. Between passes, and between two repositories of a pass, it SHALL act on each landing it is told of. It SHALL run until sent SIGTERM or SIGINT, then stop before the next repository it would keep, begin no part's pass, let each part end the pass in hand, and exit 0. A git call that fails SHALL be named and SHALL never stop it.

#### Scenario: Stopped while waiting

- **WHEN** `boomdrive run` has made its first pass and is sent SIGTERM
- **THEN** it exits 0 within a second

#### Scenario: Stopped with a part's pass under way

- **WHEN** the runner is stopped while `home`'s control pass waits on its pull, and `work` waits for its next pass
- **THEN** `work` begins no pass, `home`'s pass ends with its report saved, the runner says it stopped, and no pass begins after that line

#### Scenario: Passes on the timer

- **WHEN** `fetch_interval` is two minutes and a pass began at 10:00:00
- **THEN** the next pass begins at 10:02:00, or at once when the pass before ends later than that

#### Scenario: A failure does not stop it

- **WHEN** a pass of `boomdrive run` finds `agentplot/flume`'s fetch failing
- **THEN** the runner names it and goes on, keeping every other repository, and tries `agentplot/flume` again at the next pass

### Requirement: boomdrive run --once makes one pass

`boomdrive run --once` SHALL make one keeping pass, as `boomdrive run` does, then, where the configuration names a control plane, one control pass of each part, side by side, then the settle that ends them, and exit. It SHALL exit 0 when every git call of the keeping pass succeeded and some part both pulled and reported, and 1 otherwise. What the specs leave and name SHALL be no failure: an offer left for want of room, a tool not run or read, a part unread while another reported.

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

- **WHEN** `boomdrive run --once` runs with crew's control plane, `fetch_interval` two minutes and `poll_interval` one minute, over a scratch state repository holding a launch for this host, and a stub `flume` that launches it
- **THEN** it keeps the repositories, launches once, writes the runner's branch, runs `flume --json settle --every 60s` after every other `flume` command, and exits 0

#### Scenario: Flume's status refused

- **WHEN** `boomdrive run --once` runs with crew's control plane over a scratch state repository holding a launch for this host, and a stub `flume` that refuses its status
- **THEN** it names Flume's status as unread, launches nothing, writes the runner's branch, and exits 0

#### Scenario: A control plane that cannot be read

- **WHEN** `boomdrive run --once` runs with crew's control plane and no partition's state repository can be read
- **THEN** it keeps the repositories, names each part as unread, runs no `flume` command but each hello's `flume --json status` and `flume --version` and the settle that ends the pass, and exits 1

#### Scenario: One part unread

- **WHEN** `boomdrive run --once` runs with crew's control plane serving the partitions `home` and `work`, the state repository holds no `home/main`, and `work/main` holds a launch for this host
- **THEN** it names `home` as unread, launches `work`'s launch once, writes `work`'s runner branch, and exits 0

#### Scenario: A settle Flume refuses

- **WHEN** `boomdrive run --once` keeps its repositories with no control plane, and a stub `flume` refuses its settle with the words `no store`
- **THEN** it says `boomdrive: Flume refused the settle of every place: no store`, and exits 0

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

Where the configuration names a control plane, `boomdrive run` SHALL make each part's first control pass after its first keeping pass, and its next `poll_interval` after the start of the part's pass before. After a part's pass that reported, it SHALL make that part's next sooner, by the midpoint between that pass's end and the earliest expiry of a lease the part holds not yet lapsed. A lapsed lease, or a pass that did not report, SHALL hasten none (core.22).

#### Scenario: On the control timer

- **WHEN** `poll_interval` is one minute, `fetch_interval` two minutes, and a control pass of `home` began at 10:00:00
- **THEN** `home`'s next control pass begins at 10:01:00, and the keeping passes keep their own two minutes

#### Scenario: A lapsed lease

- **WHEN** `poll_interval` is one minute, the runner holds a lease that lapsed before a control pass that began at 10:00:00, and the control plane cannot be reached
- **THEN** the next control pass begins at 10:01:00, and none runs between

#### Scenario: A pass that did not report

- **WHEN** `poll_interval` is one minute, the runner holds a lease expiring 40 seconds after a control pass that began at 10:00:00, and that pass's report was not sent
- **THEN** the next control pass begins at 10:01:00

#### Scenario: Each part on its own timer

- **WHEN** `poll_interval` is one minute, the runner carries `home` and `work`, and `work` holds a lease expiring 40 seconds after a pass of `work` that reported
- **THEN** `work`'s next control pass begins 20 seconds after that pass ended, and `home`'s keeps its own minute

### Requirement: A control pass runs at once when its control plane wakes the runner

`boomdrive run` SHALL make a part's control pass at once when its implementation wakes that part, through the wake its opener was handed for it, and SHALL time that part's next one from that pass's start. No other part's timer SHALL move.

#### Scenario: Woken by the control plane

- **WHEN** the implementation calls the wake its opener was handed for `home`, between two control passes
- **THEN** a control pass of `home` runs at once, and its timer's next pass is `poll_interval` after its start

#### Scenario: One part woken

- **WHEN** the runner carries `home` and `work`, and the implementation calls the wake handed for `home` between two passes of each
- **THEN** `home`'s control pass runs at once, and `work`'s next pass begins when its own timer says

### Requirement: A control pass after a broken configuration acts on nothing

A control pass that follows a reading of the configuration that failed SHALL name the file and act on nothing, until the file reads again.

#### Scenario: The configuration broken while running

- **WHEN** the configuration of a running runner carrying crew's work is made invalid
- **THEN** the next control pass names the file and acts on nothing, and once the file is valid again, the pass after acts as before

### Requirement: A control plane that cannot be read is said once until it is read again

A part's control pass whose pull cannot be read SHALL say `boomdrive: <label>: the control plane could not be read`, with the pull's words, only where that part's pass before read it, or where it is the part's first. A part's pass that reads it after one that could not SHALL say `boomdrive: <label>: the control plane is read again`. That part's passes between SHALL say nothing of it, and no part's reach SHALL be said for another (core.23).

#### Scenario: Three passes unread, then read

- **WHEN** the fake control plane's part `home` refuses three pulls in a row with the words `no network`, then gives the fourth
- **THEN** one line names `home` as unread with `no network`, and one line, at the fourth pass, says `home` is read again

#### Scenario: Each part's reach its own

- **WHEN** the runner carries `home` and `work`, `home` refuses every pull, and `work` gives every pull
- **THEN** one line names `home` as unread, and no line names `work`'s reach

### Requirement: Each part of the control plane is carried on its own

Where the configuration names a control plane and it opens, `boomdrive run` SHALL carry each part its implementation serves on a thread of its own, from the end of its first keeping pass until it stops. Each part's control passes SHALL pull, answer its wants, carry its statuses and report for that part alone. No part's pull, act, command or report SHALL wait on another part's, and no control pass SHALL wait on a keeping pass (record 0026; core.49).

#### Scenario: A slow part

- **WHEN** the runner carries two parts, `home` and `work`, `home`'s pull answers only after two minutes, and `work` offers a launch whose stream has room
- **THEN** `work`'s launch is made and reported within seconds, and `work`'s next control pass begins `poll_interval` after the start of its last

#### Scenario: A failing part

- **WHEN** the runner carries `home` and `work`, `home` refuses every pull and report with the words `no network`, and `work` offers a launch whose stream has room
- **THEN** `work`'s launch is made and reported, and one line says `boomdrive: home: the control plane could not be read: no network`

#### Scenario: A keeping pass under way

- **WHEN** a keeping pass is under way, its fetch of one repository held for a minute, and `work`'s control pass falls due
- **THEN** `work`'s control pass pulls, answers and reports while the fetch is held

### Requirement: The runner's lines of a part's pass name the part

Each line the runner says of a part's control pass SHALL name the part's label after `boomdrive:`, as `boomdrive: <label>: <words>`. A line about one want SHALL begin with the want's id, as it does, the id being the part's own.

#### Scenario: Acts left for the next pass

- **WHEN** `home`'s next control pass falls due while its pass has two acts on places still to carry out
- **THEN** one line says `boomdrive: home: the next control pass is due, so 1 act on a place waits for it`

### Requirement: Each pass ends with a settle of every place

`boomdrive run` SHALL end each keeping pass, on its loop, and each part's control pass, on the part's thread, with one `flume --json settle --every <bound>`, awaited before that thread waits. The first keeping pass SHALL share the settles of the parts' first control passes. A pass that ends while another's settle is under way SHALL share it and run none. A keeping pass whose configuration did not read SHALL settle all the same, and a runner told to stop SHALL begin none (core.13; record 0029).

#### Scenario: Passes back to back share one settle

- **WHEN** `fetch_interval` and `poll_interval` are an hour, and the runner's first keeping pass and its part's first control pass run back to back with a stub `flume`
- **THEN** one `flume --json settle --every 3600s` runs, after every other `flume` command of the control pass, and no other settle runs before the next pass

#### Scenario: Each control pass ends with a settle

- **WHEN** `fetch_interval` is an hour and `poll_interval` a second, and the runner makes three control passes of its part with a stub `flume`
- **THEN** each control pass's `flume` commands end with one `flume --json settle --every 1s`, before the next control pass reads Flume's status

#### Scenario: Each keeping pass ends with a settle

- **WHEN** the configuration names no control plane, `fetch_interval` is one second, and the runner makes three keeping passes with a stub `flume`
- **THEN** each keeping pass is followed by one `flume --json settle --every 1s`, and the stub is asked nothing else

#### Scenario: A pass on a broken configuration settles

- **WHEN** the configuration of a running runner that keeps a repository each second is made invalid, with a stub `flume`
- **THEN** each pass names the file and keeps nothing, and is followed by one `flume --json settle --every 1s` all the same

#### Scenario: A pass ends while a settle is under way

- **WHEN** one pass's settle is under way, and another pass ends
- **THEN** the other pass runs no settle, and returns without waiting for the one under way to end

### Requirement: The settle's bound is the runner's interval

The bound of the settle SHALL be written `<n>s`, in whole seconds: `fetch_interval`, or the shorter of `fetch_interval` and `poll_interval` while the runner makes control passes, `fetch_interval` being the one the latest reading of the configuration gave (record 0029).

#### Scenario: The shorter interval

- **WHEN** `boomdrive run --once` runs with crew's control plane, `fetch_interval` two minutes and `poll_interval` one minute
- **THEN** its settle is `flume --json settle --every 60s`

#### Scenario: A control plane that could not be opened

- **WHEN** the configuration says `[control_plane]` with `type = "elsewhere"` and names no `fetch_interval`
- **THEN** `boomdrive run --once` runs no `flume` command but `flume --json settle --every 120s`

### Requirement: The settle is the runner's own and said only when it changes

The settle SHALL be no part of a control pass: no want's answer SHALL turn on it, and its answer SHALL be carried to no control plane and kept nowhere. Where a settle is refused, or gives no answer, the runner SHALL say so in one line only where the settle before was answered or it is the first, and SHALL say `boomdrive: Flume settles every place again` at the first settle answered after one that was not. No settle SHALL stop the runner or fail a pass (core.1, core.11, core.23).

#### Scenario: A refusal said once until Flume settles again

- **WHEN** Flume refuses three settles in a row with the words `no store`, then answers the fourth
- **THEN** the first says `boomdrive: Flume refused the settle of every place: no store`, the fourth says `boomdrive: Flume settles every place again`, and the second and third say nothing

#### Scenario: A settle with no answer

- **WHEN** `flume` cannot be run for two settles in a row
- **THEN** the first says `boomdrive: Flume gave no answer to the settle of every place: ` and why, and the second says nothing

### Requirement: A stop of boomdrive run ends its try in flight

Sent SIGTERM or SIGINT, `boomdrive run` SHALL end each try of a call to a remote it has in flight as that try's limit would, and SHALL begin no such try, before it stops as it otherwise does: before the next repository it would keep, beginning no part's pass, letting each part end the pass in hand, and exiting 0 (record 0042 as amended on 2026-11-11; core.58).

#### Scenario: Stopped with a fetch in flight

- **WHEN** `boomdrive run` keeps `agentplot/flume`, with no clone of it, whose origin takes the connection and never answers, and is sent SIGTERM once the origin has taken the connection
- **THEN** it exits 0 within five seconds, well before the fetch's limit; its last two lines read `agentplot/flume: not cloned: git fetch with origin was stopped as the runner stopped` and `boomdrive: stopped`; and the origin's connection is closed within a second of its exit

### Requirement: boomdrive run --once stops as boomdrive run does

Sent SIGTERM or SIGINT, `boomdrive run --once` SHALL stop as `boomdrive run` does: end each try of a call to a remote it has in flight as that try's limit would, begin no such try, stop before the next repository it would keep, begin no part's pass and no settle, let each part end the pass in hand, say `boomdrive: stopped`, and exit 1 (record 0042 as amended on 2026-11-11; core.39).

#### Scenario: Stopped with a fetch in flight

- **WHEN** `boomdrive run --once` keeps `agentplot/flume`, with no clone of it, whose origin takes the connection and never answers, with a stub `flume` that notes each call it is given, and is sent SIGINT once the origin has taken the connection
- **THEN** it exits 1 within five seconds, well before the fetch's limit; its last two lines read `agentplot/flume: not cloned: git fetch with origin was stopped as the runner stopped` and `boomdrive: stopped`; it runs no `flume` command; and the origin's connection is closed within a second of its exit
