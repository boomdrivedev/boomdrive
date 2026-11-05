# Runner protocol

## MODIFIED Requirements

### Requirement: Each direction sends only its own kinds

A runner SHALL send only `enroll`, `hello`, `report`, `terminal_opened`, `terminal_output`, `terminal_closed` and `message_refused`. A control plane SHALL send only `enrolled`, `enroll_refused`, `pulled`, `reported`, `terminal_open`, `terminal_resume`, `terminal_input`, `terminal_close` and `message_refused`. Each direction's envelope SHALL take its own kinds alone (core.42).

#### Scenario: A body of the other direction

- **WHEN** a `pulled` is checked as a runner's message, and a `hello` as a control plane's
- **THEN** each is refused

#### Scenario: Enrollment's kinds in the other direction

- **WHEN** an `enroll` is checked as a control plane's message, and an `enrolled` and an `enroll_refused` as a runner's
- **THEN** each is refused

## ADDED Requirements

### Requirement: An enrollment is one enroll, answered by enrolled or enroll_refused

An `enroll` SHALL hold `key`, 32 bytes, `signature`, 64 bytes, each in base64url, and `machine`, the host's name. An `enrolled` SHALL hold `host`, the control plane's id for the host. An `enroll_refused` SHALL hold `enroll`, the `enroll` message's id, and `words`. None SHALL hold a token: the token is the request's bearer (core.4, core.31; record 0018).

#### Scenario: Enrollment's examples

- **WHEN** the examples of `enroll`, `enrolled` and `enroll_refused` are checked
- **THEN** each passes, and reads into its direction's type and writes back equal

#### Scenario: An enroll unsigned, or holding a token

- **WHEN** an `enroll` with no `signature`, one whose `key` is not 32 bytes, and one holding a `token` are checked
- **THEN** each is refused

### Requirement: The README names the two paths and the signatures' contexts

The protocol's README SHALL name `runner/enroll` and `runner/connect` under the control plane's address, the kinds each takes, the `BoomdriveHost` header, and the bytes each signature of enrollment and of a connection is made over, with a worked example of each. The hosted implementation SHALL make each signature over exactly those bytes (record 0018).

#### Scenario: The README's worked contexts

- **WHEN** the hosted implementation makes the enrollment's and the connection's contexts from the README's fixed key, token, origin, time and nonce
- **THEN** each equals the README's worked bytes
