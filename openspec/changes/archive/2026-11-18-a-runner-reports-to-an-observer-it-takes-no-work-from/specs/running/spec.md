## ADDED Requirements

### Requirement: Each part's observer is handed its control passes, and no pass waits on it

Where the configuration names an observer, `boomdrive run` SHALL open it when it starts and give each part the observer of its label. Each control pass of a part that read its pull SHALL hand that observer its hello and the part's share once it has sent its report, whatever its control plane answered, waiting on nothing. An observer of no part SHALL be said once at start. `run --once` SHALL hand no observer anything (core.19, core.49; record 0048).

#### Scenario: A part observed

- **WHEN** the runner carries `home` and `work`, and an observer is opened for `home` alone
- **THEN** after each of `home`'s control passes, its observer holds that pass's hello and `home`'s share as reported, and no observer is handed `work`'s

#### Scenario: An observer unreachable

- **WHEN** the configuration's observer of `home` names a loopback port with nothing on it, and `home` offers a launch whose stream has room
- **THEN** `home`'s launch is made and reported within seconds, as with no observer

#### Scenario: An observer of no part

- **WHEN** `[observer]` names the partition `garden`, and the control plane serves `home` alone
- **THEN** one line at start says `boomdrive: observer garden: no part of the control plane is labelled garden, so it observes nothing`, and `home` is carried as before

#### Scenario: One pass

- **WHEN** `boomdrive run --once` runs with an observer of `home` named
- **THEN** no connection is made to the observer
