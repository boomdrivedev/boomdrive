# Runner protocol

## MODIFIED Requirements

### Requirement: Each direction sends only its own kinds

A runner SHALL send only `enrol`, `hello`, `report`, `terminal_opened`, `terminal_output`, `terminal_closed` and `message_refused`. A control plane SHALL send only `enrolled`, `enrol_refused`, `pulled`, `reported`, `terminal_open`, `terminal_resume`, `terminal_input`, `terminal_close` and `message_refused`. Each direction's envelope SHALL take its own kinds alone (core.42).

#### Scenario: A body of the other direction

- **WHEN** a `pulled` is checked as a runner's message, and a `hello` as a control plane's
- **THEN** each is refused

#### Scenario: Enrolment's kinds in the other direction

- **WHEN** an `enrol` is checked as a control plane's message, and an `enrolled` and an `enrol_refused` as a runner's
- **THEN** each is refused

## ADDED Requirements

### Requirement: An enrolment is one enrol, answered by enrolled or enrol_refused

An `enrol` SHALL hold `key`, 32 bytes, `signature`, 64 bytes, each in base64url, and `machine`, the host's name. An `enrolled` SHALL hold `host`, the control plane's id for the host. An `enrol_refused` SHALL hold `enrol`, the `enrol` message's id, and `words`. None SHALL hold a token: the token is the request's bearer (core.4, core.31; record 0016).

#### Scenario: Enrolment's examples

- **WHEN** the examples of `enrol`, `enrolled` and `enrol_refused` are checked
- **THEN** each passes, and reads into its direction's type and writes back equal

#### Scenario: An enrol unsigned, or holding a token

- **WHEN** an `enrol` with no `signature`, one whose `key` is not 32 bytes, and one holding a `token` are checked
- **THEN** each is refused

### Requirement: The README names the two paths and the signatures' contexts

The protocol's README SHALL name `runner/enrol` and `runner/connect` under the control plane's address, the kinds each takes, the `FlywheelHost` header, and the bytes each signature of enrolment and of a connection is made over, with a worked example of each. The hosted implementation SHALL make each signature over exactly those bytes (record 0016).

#### Scenario: The README's worked contexts

- **WHEN** the hosted implementation makes the enrolment's and the connection's contexts from the README's fixed key, token, origin, time and nonce
- **THEN** each equals the README's worked bytes
