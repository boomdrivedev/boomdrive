## ADDED Requirements

### Requirement: A newest release is said once on each connection

On a connection of version 6, the runner SHALL say each `newest_release` in one line, `<label>: this runner is <runner_version>, and the newest release is <version>`, its `runner_version` the hello's that the connection sent. It SHALL say each release one time on each connection, and again on a new connection. It SHALL compare no version, act on none, and raise no alarm (core.1, core.23, Boom: `the-cloud-leads-the-runners-protocol`).

#### Scenario: A newest release is said once on each connection

- **WHEN** partition `home` speaks version 6 with a hello that names `boomdrive 0.1.0`, and the fake control plane sends `newest_release` of `0.32.0` two times, then of `0.33.0`
- **AND** the fake drops the connection and sends `0.33.0` on the next
- **THEN** the upgrade asks for `boomdrive.v6`, `home` says `home: this runner is boomdrive 0.1.0, and the newest release is 0.32.0` one time, the line for `0.33.0` one time on each connection, and refuses nothing

### Requirement: Version 5 carries no newest release

On a connection of version 5, a `newest_release` fails the check and SHALL be answered with `message_refused`, and no line SHALL name a release (core.42).

#### Scenario: Version 5 carries no newest release

- **WHEN** partition `home` speaks version 5 and the fake control plane sends `newest_release`
- **THEN** the runner answers `message_refused` naming that message, its words saying it fails version 5's check, and no line names a release
