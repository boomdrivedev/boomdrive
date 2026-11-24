## ADDED Requirements

### Requirement: The runner runs no settle of Flume's

`boomdrive run` and `boomdrive run --once` SHALL run no `flume settle`, in no pass, keeping or control. Flume's watch over the host SHALL be `flume host`'s, which the host's service manager runs (core.13; `one-resident-per-tool`).

#### Scenario: No pass runs a settle

- **WHEN** the configuration names no control plane, `fetch_interval` is one second, and the runner makes three keeping passes with a stub `flume`
- **THEN** the stub `flume` is asked nothing

#### Scenario: A control pass runs no settle

- **WHEN** `fetch_interval` is an hour and `poll_interval` a second, and the runner makes three control passes of its part with a stub `flume`
- **THEN** the stub `flume` is asked for no settle, and each control pass reads Flume's status

### Requirement: boomdrive run --once makes one pass and exits

`boomdrive run --once` SHALL make one keeping pass, as `boomdrive run` does, then, where the configuration names a control plane, one control pass of each part, side by side, then the ticks that end them, and exit. It SHALL exit 0 when every git call of the keeping pass succeeded and some part both pulled and reported, and 1 otherwise. What the specs leave and name SHALL be no failure: an offer left for want of room, a tool not run or read, a part unread while another reported.

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
- **THEN** it keeps the repositories, launches once, writes the runner's branch, asks the stub `flume` for no settle, and exits 0

#### Scenario: Flume's status refused

- **WHEN** `boomdrive run --once` runs with crew's control plane over a scratch state repository holding a launch for this host, and a stub `flume` that refuses its status
- **THEN** it names Flume's status as unread, launches nothing, writes the runner's branch, and exits 0

#### Scenario: A control plane that cannot be read

- **WHEN** `boomdrive run --once` runs with crew's control plane and no partition's state repository can be read
- **THEN** it keeps the repositories, names each part as unread, runs no `flume` command but each hello's `flume --json status` and `flume --version`, and exits 1

#### Scenario: One part unread

- **WHEN** `boomdrive run --once` runs with crew's control plane serving the partitions `home` and `work`, the state repository holds no `home/main`, and `work/main` holds a launch for this host
- **THEN** it names `home` as unread, launches `work`'s launch once, writes `work`'s runner branch, and exits 0

### Requirement: Passes share their ticks

The first keeping pass SHALL share the ticks of the parts' first control passes. A pass that ends while another's ticks are under way SHALL share them, run none and wait for none. A keeping pass whose configuration did not read SHALL tick all the same, the inboxes of the last reading, and a runner told to stop SHALL begin none (record 0033; core.13, core.49).

#### Scenario: Passes back to back share their ticks

- **WHEN** the configuration names the inbox `home`, `fetch_interval` and `poll_interval` are an hour, and the runner's first keeping pass and its part's first control pass run back to back with a stub `millpond`
- **THEN** one `millpond tick --inbox home --json` runs, and no other tick runs before the next pass

#### Scenario: A pass ends while a tick is under way

- **WHEN** one pass's ticks are under way, and another pass ends
- **THEN** the other pass runs no tick, and returns without waiting for the ones under way to end

#### Scenario: A pass on a broken configuration ticks

- **WHEN** the configuration of a running runner that names the inbox `home` and keeps a repository each second is made invalid
- **THEN** each pass names the file and keeps nothing, and is followed by `millpond tick --inbox home --json` all the same

## MODIFIED Requirements

### Requirement: A stop of the runner ends its calls in flight

Sent SIGTERM or SIGINT, `boomdrive run` and `boomdrive run --once` SHALL end each try of a call to a remote in flight as its limit would, through the runner's halt, `weir-bounded`'s, begin no such try, keep no further repository, begin no part's pass and no tick, and let each part end the pass in hand. `boomdrive run` SHALL then exit 0, and `boomdrive run --once` SHALL say `boomdrive: stopped` and exit 1 (record 0042 as amended on 2026-11-12; core.39, core.58).

#### Scenario: Stopped with a fetch in flight

- **WHEN** `boomdrive run` keeps `agentplot/flume`, with no clone of it, from an origin that takes the connection and never answers, and is sent SIGTERM once the origin has taken it
- **THEN** it exits 0 within five seconds; the lines after its start are `agentplot/flume: not cloned: git fetch with origin was stopped as its caller asked` and `boomdrive: stopped`; no folder of `agentplot/flume`'s is left in the data directory; and the origin's connection is closed within a second of its exit

#### Scenario: Stopped with a part's read in flight

- **WHEN** `boomdrive run` carries crew's partition `home`, whose state repository takes the connection and never answers, and is sent SIGTERM once `home`'s read of `home/main` has taken it
- **THEN** it exits 0 within five seconds; one line reads `boomdrive: home: the control plane could not be read: ` followed by words ending `git fetch with <url> was stopped as its caller asked`, naming the state repository's URL; its last line is `boomdrive: stopped`; and the state repository's connection is closed within a second of its exit

#### Scenario: run --once stopped with a fetch in flight

- **WHEN** `boomdrive run --once` keeps `agentplot/flume`, with no clone of it, from an origin that takes the connection and never answers, and is sent SIGINT once the origin has taken it
- **THEN** it exits 1 within five seconds; the lines after its start are `agentplot/flume: not cloned: git fetch with origin was stopped as its caller asked` and `boomdrive: stopped`; it runs no `flume`; and the origin's connection is closed within a second of its exit

### Requirement: Each pass ends with a tick of each inbox

`boomdrive run` SHALL end each keeping pass and each part's control pass with `millpond tick --inbox <name> --json`, once for each inbox the configuration's `inboxes` names, in its order, awaited on that pass's thread, and none where it names none; `boomdrive run --once` SHALL end its passes so. millpond SHALL decide what a tick runs, and the runner SHALL read nothing of its answer but whether millpond gave one (record 0033; core.1, core.13).

#### Scenario: Each pass ticks each inbox

- **WHEN** the configuration names the inboxes `home` and `work`, and the runner makes three keeping passes with a stub `millpond`
- **THEN** each pass ends with `millpond tick --inbox home --json`, then `millpond tick --inbox work --json`, and an answered tick says nothing

#### Scenario: No inbox, no tick

- **WHEN** the configuration names no `inboxes`
- **THEN** the runner runs no `millpond` command

## REMOVED Requirements

### Requirement: boomdrive run --once makes one pass

**Reason**: Its passes end with a settle, which the runner no longer runs. The requirement `boomdrive run --once makes one pass and exits` says the same with no settle.

**Migration**: None.

### Requirement: Each pass ends with a settle of every place

**Reason**: `flume host` keeps the host's watch, and a settle of the runner's is a second flume process that `one-resident-per-tool` forbids.

**Migration**: None on a host: the set at Flume v0.35.0 runs `flume host` by Flume's own module.

### Requirement: The settle's bound is the runner's interval

**Reason**: The runner runs no settle.

**Migration**: None.

### Requirement: The settle is the runner's own and said only when it changes

**Reason**: The runner runs no settle, so it says nothing of one.

**Migration**: None.

## RENAMED Requirements

- FROM: `### Requirement: Each pass ticks each inbox after its settle`
- TO: `### Requirement: Each pass ends with a tick of each inbox`
