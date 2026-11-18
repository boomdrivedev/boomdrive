## ADDED Requirements

### Requirement: The observer table names a loopback origin and the partitions it observes

The hosted implementation SHALL read `[observer]` as it reads the hosted table, but that its `url` SHALL be `http://` or `https://` to a loopback address alone, and that each partition's `label` names the part whose work it observes. A refusal SHALL name `[observer]`, the file and the key, its value in no output (core.19, core.26; record 0048).

#### Scenario: The observer read

- **WHEN** `[observer]` says `url = "http://localhost:8787"` and the partition `home`, with `example/app`
- **THEN** it is read, observing `home` with `example/app`

#### Scenario: An observer off the host

- **WHEN** `[observer]` says `url = "https://control.example"`
- **THEN** it is refused, naming `[observer]` and `url` and saying an observer is reached at a loopback address alone

#### Scenario: A token in the observer table

- **WHEN** `[observer]` says `token = "..."`
- **THEN** it is refused, naming `token` and saying the host's key is made by `boomdrive enroll --observer`, and the value appears in no output

### Requirement: An observer's partition is enrolled once by boomdrive enroll --observer

`boomdrive enroll --observer <label>` SHALL enroll the partition `<label>` of `[observer]` as `boomdrive enroll` enrolls a hosted partition, at the observer's `runner/enroll`, whatever `[control_plane]`'s `type`. It SHALL keep the key under `observer.<label>`, which no hosted partition's label can be, and refuse with exit 2 where the file names no `[observer]` or no partition `<label>` in it (core.31, core.32; records 0006, 0018 and 0048).

#### Scenario: An observer enrolled beside crew

- **WHEN** `boomdrive enroll --observer home` runs with `type = "crew"`, and the fake observer takes the `enroll` and answers `enrolled` with host `h-1`
- **THEN** the request went to the observer's `runner/enroll`, its `enroll` passed version 3, the key is in the store under `observer.home`, and the enrollment names host `h-1`

#### Scenario: Kept apart from a partition of the same label

- **WHEN** an observer's partition `home` is enrolled into the `systemd-creds` store, its program a stub reporting version 256
- **THEN** the stub is run as `encrypt --user --name=boomdrive.observer.home.<the url's host>`

#### Scenario: No observer named

- **WHEN** `boomdrive enroll --observer home` runs with no `[observer]`
- **THEN** it is refused naming `[observer]`, and exits 2

### Requirement: An observer is told the observing hello and the places the part follows, with no lease

An observer's partition SHALL keep one outgoing connection, proved as a hosted partition's is, whose first message is its whole observing hello, naming only the repositories its partition names of those kept. After each control pass of the part it observes, it SHALL be sent an observed report of that part's launches, each id as the runner holds it, its answer, and its status and entry as version 3's copies, and no lease, act, command, inbox or preview (core.26, core.27; record 0048).

#### Scenario: The observing hello

- **WHEN** an observer of `home`, which names `example/app`, is handed a hello keeping `example/app` and `example/service`, with a capacity and a lease
- **THEN** its first message is a `hello` with `observing` true, naming `example/app` alone, and no capacity and no lease

#### Scenario: The places observed

- **WHEN** an observer of `home` is handed a share holding the launch `home/1` with its place's status, and an act `home/2`
- **THEN** it sends a `report` with `observing` true, holding `home/1` with its answer and its status's copy and no lease, and no act

### Requirement: What an observer sends of work is refused and said

The runner SHALL take no work from an observer. A `pulled` holding no want and no report SHALL be its answer to the hello. A `pulled` holding a want or a report, and every `terminal_open`, `terminal_input`, `terminal_resume` and `terminal_close`, SHALL be answered with `message_refused`, its words saying the connection observes, and said in one line naming its kind and id, and nothing else SHALL be done with it (core.1, core.19; record 0048).

#### Scenario: An offer from an observer

- **WHEN** the fake observer answers `home`'s observing hello with a `pulled` holding an offer
- **THEN** the runner answers `message_refused` naming it, its words saying the connection observes, and one line names the `pulled` and its id

#### Scenario: A terminal from an observer

- **WHEN** the fake observer sends `terminal_open` on `home`'s connection
- **THEN** the runner answers `message_refused` naming it, sends no `terminal_closed`, and makes no key

### Requirement: An observer's answers change nothing and wake nothing

A `reported` from an observer SHALL answer its observed report, so it is not sent again, and its let-go and refused ids SHALL change nothing. No message from an observer SHALL wake the runner, and none SHALL name a status taken, so no end is confirmed on an observer's answer (core.10, core.54; record 0048).

#### Scenario: A report's answer that lets go

- **WHEN** the fake observer answers an observed report with a `reported` letting go `home/1`
- **THEN** the observed report is not sent again, and the next observed report still holds `home/1`

#### Scenario: A pulled wakes nothing

- **WHEN** the fake observer sends `home`'s observer three `pulled`, each holding no want
- **THEN** the wake the observer was opened beside is never called

### Requirement: Nothing waits on an observer

Handing an observer its hello and report SHALL return at once, whether its connection is live, being attempted, or waiting on its backoff, and SHALL wait for no answer. Its newest report SHALL be sent at once on a live connection, and after the hello of its next one, replacing any it was not answered for. An observer's connection lost SHALL be tried again on the backoff a hosted partition's is (core.21, core.22; record 0048).

#### Scenario: An observer down

- **WHEN** the fake observer is down, and an observer of `home` is handed a hello and a report three times
- **THEN** each handing returns within 100 milliseconds

#### Scenario: Back again

- **WHEN** the fake observer is down while an observer of `home` is handed two reports, and then comes back
- **THEN** after the observing hello, the second report alone is sent

### Requirement: An observer's reach is said once each way

An observer's partition SHALL say one line when its connection is lost or cannot be made, `<label>: the observer is unreachable: <why>`, and one when it takes its first `pulled`, `<label>: the observer is reached`, and a refusal of its key in one line naming `boomdrive enroll --observer <label>` (core.23; record 0048).

#### Scenario: An observer's outage

- **WHEN** an observer of `home` is reached, then the fake observer stops answering for five attempts, then answers
- **THEN** `home` says the observer is reached, once that it is unreachable, and once that it is reached again
