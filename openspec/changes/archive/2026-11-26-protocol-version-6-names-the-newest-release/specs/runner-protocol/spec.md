## ADDED Requirements

### Requirement: Version 6 is version 5 whole, but for the newest release

Version 6's files SHALL be version 5's but for three things. Each `$id` is under `…/protocol/v6/`. Each message names `protocol_version` 6. `to-runner.schema.json` holds `newest_release`, a branch of the body and its definition. Its examples SHALL be version 5's, each naming `protocol_version` 6, and `to-runner/newest_release.json` (core.42, Boom: `the-cloud-leads-the-runners-protocol`).

#### Scenario: Version 6 is version 5 but for the newest release

- **WHEN** each schema file of version 6 is read with every `description` set aside and its `$id` under version 5's address
- **AND** its `protocol_version` is read as 5, and `newest_release` is taken out of `to-runner.schema.json`
- **THEN** it equals version 5's file of the same name, with every `description` set aside

#### Scenario: Version 6's examples are version 5's

- **WHEN** version 6's examples are read
- **THEN** their names are version 5's and `to-runner/newest_release.json`, and each of version 5's, its `protocol_version` read as 5, equals version 5's

### Requirement: Version 6 carries the newest release

Version 6's control plane SHALL send `newest_release`, `{"version": …}`, closed. Its `version` is three numbers with no leading zero, as a release's tag `v<version>` names it. The control plane SHALL send it on the socket after its `pulled` answers the hello, where the hello's `runner_version` is older. It MAY send it again when a newer release is made. No other version SHALL take it (core.42, Boom: `the-cloud-leads-the-runners-protocol`).

#### Scenario: Version 6's examples read

- **WHEN** each example of version 6 is checked against its direction's envelope of version 6 and read into the runner's types
- **THEN** each passes and writes back equal, and `newest_release.json` reads as a newest release of `0.32.0`

#### Scenario: A newest release that is no release's

- **WHEN** a `newest_release` naming a field it does not name, no version, `v0.32.0`, `0.32`, `0.32.0-rc.1` or `00.32.0` is checked against version 6, and `newest_release.json` naming `protocol_version` 5 against version 5
- **THEN** each is refused

### Requirement: The runner speaks the one version spoken.json marks spoken

`protocol::spoken()` SHALL be the one version `spoken.json` marks spoken. Every message the runner sends or reads, on a socket or by a request, SHALL be of that version. Each upgrade SHALL ask for its subprotocol alone. The crate's tests SHALL pass with either version marked spoken, so a change to the mark alone moves the runner (core.42, Boom: `the-cloud-leads-the-runners-protocol`).

#### Scenario: The upgrade's subprotocol

- **WHEN** a partition's connection is dialed
- **THEN** its upgrade asks for the subprotocol of the version spoken alone, `boomdrive.v5` while version 5 is marked spoken

### Requirement: The runner's code reads and writes versions 5 and 6

`boomdrive_hosted::protocol::v6` SHALL hold the runner's types, which read and write each message of version 5 and of version 6. `protocol::V5` and `protocol::V6` SHALL each hold its version's number, subprotocol, address and files (core.42).

#### Scenario: Each version the code speaks is its own files

- **WHEN** `protocol::V5` and `protocol::V6` are read
- **THEN** each holds its module's number, subprotocol, address and files

## MODIFIED Requirements

### Requirement: Each direction sends only its own kinds

A runner SHALL send only `enroll`, `hello`, `report`, `terminal_opened`, `terminal_output`, `terminal_closed`, `message_refused` and, from version 4, `forge_token`. A control plane SHALL send only `enrolled`, `enroll_refused`, `pulled`, `reported`, the four `terminal_` kinds and `message_refused`, from version 4 `forge_token_given` and `forge_token_refused`, and from version 6 `newest_release`. Each envelope SHALL take its own kinds alone (core.42).

#### Scenario: A body of the other direction

- **WHEN** a `pulled` is checked as a runner's message, and a `hello` as a control plane's
- **THEN** each is refused

#### Scenario: Enrollment's kinds in the other direction

- **WHEN** an `enroll` is checked as a control plane's message, and an `enrolled` and an `enroll_refused` as a runner's
- **THEN** each is refused

#### Scenario: A forge token's kinds in the other direction

- **WHEN** version 4's `forge_token` is checked as a control plane's message of version 4, and its `forge_token_given` and `forge_token_refused` as a runner's
- **THEN** each is refused

### Requirement: The runner names the versions it speaks in one file of data

`crates/boomdrive-hosted/protocol/spoken.json` SHALL name each version directory under `protocol/` once, lowest first, as `{"version": N, "spoken": bool}` under `versions`, and nothing else. It SHALL mark spoken exactly one version, which the runner's code reads from the file, and checks, sends and offers. The crate's test SHALL hold the file to the directories and to the code (core.42, principle 16).

#### Scenario: The file names each version directory

- **WHEN** `spoken.json` is read and `protocol/` listed
- **THEN** it holds `versions` alone, naming versions 1 to 6 once each, lowest first, and versions 1 to 4 and 6 are not spoken

#### Scenario: The runner speaks what the file marks spoken

- **WHEN** the crate's test reads `spoken.json`
- **THEN** it marks one version spoken, `protocol::spoken()`, which is version 5 or version 6, and the upgrade offers its subprotocol

## REMOVED Requirements

### Requirement: The runner speaks version 5 alone

**Reason**: The runner speaks the one version `spoken.json` marks spoken, and its code reads and writes version 5 and version 6.

**Migration**: "The runner speaks the one version spoken.json marks spoken" holds the upgrade's subprotocol and the version of every message.
