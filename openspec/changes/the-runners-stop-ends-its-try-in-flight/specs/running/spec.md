## MODIFIED Requirements

### Requirement: boomdrive run keeps the repositories until it is stopped

`boomdrive run` SHALL make a pass over the configuration at once and then one every `fetch_interval`, timed from the start of the pass before. Between passes, and between two repositories of a pass, it SHALL act on each landing it is told of. It SHALL run until sent SIGTERM or SIGINT, then end each try of a call to a remote it has in flight as that try's limit would, begin no such try, stop before the next repository it would keep, begin no part's pass, let each part end the pass in hand, and exit 0. A git call that fails SHALL be named and SHALL never stop it.

#### Scenario: Stopped while waiting

- **WHEN** `boomdrive run` has made its first pass and is sent SIGTERM
- **THEN** it exits 0 within a second

#### Scenario: Stopped with a fetch in flight

- **WHEN** `boomdrive run` keeps `agentplot/flume`, with no clone of it, whose origin takes the connection and never answers, and is sent SIGTERM once the origin has taken the connection
- **THEN** it exits 0 within five seconds, well before the fetch's limit; its last two lines read `agentplot/flume: not cloned: git fetch with origin was stopped as the runner stopped` and `boomdrive: stopped`; and the origin's connection is closed within a second of its exit

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

`boomdrive run --once` SHALL make one keeping pass, as `boomdrive run` does, then, where the configuration names a control plane, one control pass of each part, side by side, then the settle that ends them, and exit. It SHALL exit 0 when every git call of the keeping pass succeeded and some part both pulled and reported, and 1 otherwise. What the specs leave and name SHALL be no failure: an offer left for want of room, a tool not run or read, a part unread while another reported. Sent SIGTERM or SIGINT, it SHALL stop as `boomdrive run` does: end each try of a call to a remote it has in flight as that try's limit would, begin no such try, stop before the next repository it would keep, begin no part's pass and no settle, let each part end the pass in hand, say `boomdrive: stopped`, and exit 1.

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

#### Scenario: Stopped with a fetch in flight

- **WHEN** `boomdrive run --once` keeps `agentplot/flume`, with no clone of it, whose origin takes the connection and never answers, with a stub `flume` that notes each call it is given, and is sent SIGINT once the origin has taken the connection
- **THEN** it exits 1 within five seconds, well before the fetch's limit; its last two lines read `agentplot/flume: not cloned: git fetch with origin was stopped as the runner stopped` and `boomdrive: stopped`; it runs no `flume` command; and the origin's connection is closed within a second of its exit
