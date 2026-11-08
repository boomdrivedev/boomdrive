## ADDED Requirements

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

## MODIFIED Requirements

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

`boomdrive run --once` SHALL make one keeping pass, just as a pass of `boomdrive run`, then, where the configuration names a control plane, one control pass of each part, side by side, and exit. It SHALL exit 0 when every git call of the keeping pass succeeded and some part both pulled and reported, and 1 otherwise. Something left and named as the specs say SHALL not be a failure: an offer left for want of room, a tool not run or read, and a part unread while another reported.

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
- **THEN** it keeps the repositories, names each part as unread, runs no `flume` command but each hello's `flume --json status` and `flume --version`, and exits 1

#### Scenario: One part unread

- **WHEN** `boomdrive run --once` runs with crew's control plane serving the partitions `home` and `work`, the state repository holds no `home/main`, and `work/main` holds a launch for this host
- **THEN** it names `home` as unread, launches `work`'s launch once, writes `work`'s runner branch, and exits 0

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

### Requirement: A control plane that cannot be read is said once until it is read again

A part's control pass whose pull cannot be read SHALL say `boomdrive: <label>: the control plane could not be read`, with the pull's words, only where that part's pass before read it, or where it is the part's first. A part's pass that reads it after one that could not SHALL say `boomdrive: <label>: the control plane is read again`. That part's passes between SHALL say nothing of it, and no part's reach SHALL be said for another (core.23).

#### Scenario: Three passes unread, then read

- **WHEN** the fake control plane's part `home` refuses three pulls in a row with the words `no network`, then gives the fourth
- **THEN** one line names `home` as unread with `no network`, and one line, at the fourth pass, says `home` is read again

#### Scenario: Each part's reach its own

- **WHEN** the runner carries `home` and `work`, `home` refuses every pull, and `work` gives every pull
- **THEN** one line names `home` as unread, and no line names `work`'s reach
