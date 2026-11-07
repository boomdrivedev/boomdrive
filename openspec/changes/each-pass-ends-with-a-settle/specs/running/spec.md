# running

## ADDED Requirements

### Requirement: Each turn of passes ends with a settle of every place

`boomdrive run` SHALL end each turn of its loop, after the passes due and before it waits, with one `flume --json settle --every <bound>`, run once and awaited. A keeping pass that a control pass follows at once SHALL share its settle. A turn whose configuration did not read SHALL settle all the same. A runner told to stop SHALL begin no settle (core.13; record 0029; Flume's 208).

#### Scenario: Passes back to back share one settle

- **WHEN** `fetch_interval` and `poll_interval` are an hour, and the runner's first keeping pass and first control pass run back to back with a stub `flume`
- **THEN** one `flume --json settle --every 3600s` runs, after every other `flume` command of the control pass, and no other settle runs before the next turn

#### Scenario: Each control pass ends with a settle

- **WHEN** `fetch_interval` is an hour and `poll_interval` a second, and the runner makes three control passes with a stub `flume`
- **THEN** each control pass's `flume` commands end with one `flume --json settle --every 1s`, before the next control pass reads Flume's status

#### Scenario: Each keeping pass ends with a settle

- **WHEN** the configuration names no control plane, `fetch_interval` is one second, and the runner makes three keeping passes with a stub `flume`
- **THEN** each keeping pass is followed by one `flume --json settle --every 1s`, and the stub is asked nothing else

#### Scenario: A pass on a broken configuration settles

- **WHEN** the configuration of a running runner that keeps a repository each second is made invalid, with a stub `flume`
- **THEN** each pass names the file and keeps nothing, and is followed by one `flume --json settle --every 1s` all the same

### Requirement: The settle's bound is the runner's interval

The bound of the settle SHALL be written `<n>s`, in whole seconds: `fetch_interval`, or the shorter of `fetch_interval` and `poll_interval` while the runner makes control passes, `fetch_interval` being the one the latest reading of the configuration gave (record 0029).

#### Scenario: The shorter interval

- **WHEN** `boomdrive run --once` runs with crew's control plane, `fetch_interval` two minutes and `poll_interval` one minute
- **THEN** its settle is `flume --json settle --every 60s`

#### Scenario: A control plane that could not be opened

- **WHEN** the configuration says `[control_plane]` with `type = "elsewhere"` and names no `fetch_interval`
- **THEN** `boomdrive run --once` runs no `flume` command but `flume --json settle --every 120s`

### Requirement: The settle is the runner's own and said only when it changes

The settle SHALL be no part of a control pass: no want's answer SHALL turn on it, and its answer SHALL be carried to no control plane and kept nowhere. Where a settle is refused, or gives no answer, the runner SHALL say so in one line only where the settle before was answered or it is the first, and SHALL say `boomdrive: Flume settles every place again` at the first settle answered after one that was not. No settle SHALL stop the runner (core.1, core.11, core.23).

#### Scenario: A refusal said once until Flume settles again

- **WHEN** Flume refuses three settles in a row with the words `no store`, then answers the fourth
- **THEN** the first says `boomdrive: Flume refused the settle of every place: no store`, the fourth says `boomdrive: Flume settles every place again`, and the second and third say nothing

#### Scenario: A settle with no answer

- **WHEN** `flume` cannot be run for two settles in a row
- **THEN** the first says `boomdrive: Flume gave no answer to the settle of every place: ` and why, and the second says nothing

## MODIFIED Requirements

### Requirement: boomdrive run --once makes one pass

`boomdrive run --once` SHALL make one keeping pass, just as a pass of `boomdrive run`, then, where the configuration names a control plane, one control pass, then the settle that ends them, and exit. It SHALL exit 0 when every git call of the keeping pass succeeded and the control pass both pulled and reported, and 1 otherwise. Something left and named as the specs say SHALL not be a failure: an offer left untaken for want of room, a tool the control pass could not run or read, and a settle Flume refused or gave no answer to, among it.

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
- **THEN** it keeps the repositories, names the control plane as unread, runs no `flume` command but the hello's `flume --json status` and `flume --version` and the settle that ends the pass, and exits 1

#### Scenario: A settle Flume refuses

- **WHEN** `boomdrive run --once` keeps its repositories with no control plane, and a stub `flume` refuses its settle with the words `no store`
- **THEN** it says `boomdrive: Flume refused the settle of every place: no store`, and exits 0
