## ADDED Requirements

### Requirement: Version 5 is version 4 whole, but for the observing forms and a report's unleased places

Version 5's files SHALL be version 4's but for these alone: each `$id` under `…/protocol/v5/`, `protocol_version` 5, no observing hello or observed report, each kind taking its one form, and the report's `unleased`. Its examples SHALL be version 4's but the observing connection's three, each report naming `unleased`, and `report-a-place-started-elsewhere.json` (core.42; record 0054).

#### Scenario: Version 5 is version 4 but for its unleased places

- **WHEN** each schema file of version 5 is read with every `description` set aside, its `$id` read under version 4's address, its `protocol_version` read as 4, `unleased` and `unleased_place` taken out of its report, and version 4's observing forms taken out of version 4's files
- **THEN** it equals version 4's file of the same name, so read, with every `description` set aside

#### Scenario: Version 5's examples are version 4's

- **WHEN** version 5's examples are read
- **THEN** their names are version 4's but the observing connection's three, and `from-runner/report-a-place-started-elsewhere.json`

### Requirement: Version 5's report names each place the partition did not lease, with no lease

Version 5's report SHALL require `unleased`, a map from a place's path, as Flume printed it, to `{repository, status}`: `repository` an address, and `status` a carried status as a launch's is, or null. It SHALL name no lease, answer, act or entry. No observing hello or observed report SHALL pass version 5 (core.10, core.42; record 0054).

#### Scenario: A place started elsewhere

- **WHEN** version 5's `report-a-place-started-elsewhere.json` is checked against `from-runner.schema.json` of version 5
- **THEN** it passes

#### Scenario: An unleased place that names a lease

- **WHEN** that example with a `lease` added to its unleased place, and one with no `unleased`, are checked against version 5
- **THEN** each is refused

#### Scenario: No observing connection in version 5

- **WHEN** version 4's `hello-observing.json` and `report-observed.json`, each naming `protocol_version` 5, are checked against version 5's envelope
- **THEN** each is refused

### Requirement: The runner speaks version 5 alone

`boomdrive_hosted::protocol::v5` SHALL hold version 5's files and the runner's types for its messages, and every message the runner sends or reads, on a socket or by a request, SHALL be of version 5. Each upgrade SHALL ask for `boomdrive.v5` alone (core.42; record 0054).

#### Scenario: The upgrade's subprotocol

- **WHEN** a partition's connection is dialed
- **THEN** its upgrade asks for `boomdrive.v5` alone

### Requirement: Version 4 stays as a release spoke it

Every file under `protocol/v4/`, its schemas and its examples, SHALL stay as release `v0.21.0` holds it, and no file SHALL be added there. Every example of version 4 SHALL still pass its direction's envelope of version 4. `boomdrive_hosted::protocol::v4` SHALL hold version 4's files and no type (core.42; records 0053 and 0054).

#### Scenario: Version 4 as released

- **WHEN** each file under `protocol/v4/` is read and its SHA-256 taken
- **THEN** the files are exactly those the test lists, each with the digest the test holds for it, as `git show v0.21.0:<path>` gives it

#### Scenario: Version 4's examples still pass

- **WHEN** each example under version 4's `examples/` is checked against its direction's envelope of version 4
- **THEN** each passes

## REMOVED Requirements

### Requirement: Version 3 carries an observing hello and an observed report

**Reason**: Record 0048 is superseded by record 0054, and the runner speaks no version that carries an observing connection. Version 3's files stay as released, held by "Version 3 stays as a release spoke it".
**Migration**: None.
