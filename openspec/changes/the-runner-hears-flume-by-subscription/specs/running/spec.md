## ADDED Requirements

### Requirement: The runner hears Flume's status from flume host

While `boomdrive run` carries a control plane's parts, it SHALL hold one subscription to flume host, `{"ask":"subscribe"}`, keep the latest `document` as flume host sent it, and read each pass's status from it, running no `flume --json status`. A document that differs from the one held SHALL wake every part. `boomdrive run --once` SHALL ask `{"ask":"status"}` for each read. A runner that carries no part SHALL hold none (core.13).

#### Scenario: Passes read the subscription

- **WHEN** the runner carries `home`, `poll_interval` is a second, and a stub flume host answers on the socket
- **THEN** three control passes run, the stub flume host is asked one subscription, and the stub `flume` is asked no `status`

#### Scenario: A change heard wakes the parts

- **WHEN** the runner carries `home`, `poll_interval` is an hour, its first control pass has ended, and the stub flume host sends a document that differs from the one held
- **THEN** `home` makes a control pass within seconds

#### Scenario: A runner that carries no part

- **WHEN** the configuration names no control plane
- **THEN** the runner makes no connection to flume host's socket

### Requirement: The socket is flume host's own

The runner SHALL find flume host's socket as flume host makes it: `host-<key>.sock` in `$FLUME_USER_SCRATCH` where it is set, else in `flume-<uid>` under `/tmp` resolved, `<key>` the first 10 hex digits of the SHA-1 of flume's state folder, `$XDG_STATE_HOME/flume` or `~/.local/state/flume`, resolved. It SHALL take flume host's answer to its hello only where it names `protocol` 1, within ten seconds.

#### Scenario: The socket of a scratch home

- **WHEN** `XDG_STATE_HOME` names a folder holding `flume`, and `FLUME_USER_SCRATCH` names another folder
- **THEN** the socket is `host-<the first 10 hex digits of the SHA-1 of the resolved flume folder>.sock` in the second folder

#### Scenario: Another protocol

- **WHEN** flume host answers the hello with `protocol` 2
- **THEN** the runner asks nothing on the connection, and Flume's status is unread, with words naming protocol 2

### Requirement: No flume host, no status

Where no flume host answers, or the subscription ends, Flume's status SHALL be unread, with words naming the socket and why, and the pass SHALL act as it acts on any unread status. The runner SHALL try the subscription again after a second, doubling up to a minute, with jitter, and SHALL say `boomdrive: flume host does not answer: <why>` once when it stops answering, and `boomdrive: flume host answers again` once it does (core.22, core.23).

#### Scenario: flume host stops and comes back

- **WHEN** the stub flume host stops while the runner carries `home`, and starts again
- **THEN** one line says `boomdrive: flume host does not answer: ` and why, `home`'s passes meanwhile say its status could not be read and launch nothing, and one line says `boomdrive: flume host answers again`

#### Scenario: A refusal in the document

- **WHEN** flume host's document is `{"refused":{"refusal":{"reason":"no store"}}}`
- **THEN** the pass reads Flume's status as refused with `no store`, as it read the command's refusal

## MODIFIED Requirements

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
- **THEN** it keeps the repositories, names each part as unread, asks a stub flume host for each hello's status, runs no `flume` command but each hello's `flume --version`, and exits 1

#### Scenario: One part unread

- **WHEN** `boomdrive run --once` runs with crew's control plane serving the partitions `home` and `work`, the state repository holds no `home/main`, and `work/main` holds a launch for this host
- **THEN** it names `home` as unread, launches `work`'s launch once, writes `work`'s runner branch, and exits 0
