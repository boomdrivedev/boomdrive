## MODIFIED Requirements

### Requirement: The runner names the versions it speaks in one file of data

`crates/boomdrive-hosted/protocol/spoken.json` SHALL name each version directory under `protocol/` once, lowest first, as `{"version": N, "spoken": bool}` under `versions`, and nothing else. It SHALL mark spoken exactly one version, which the runner's code reads from the file, and checks, sends and offers. The crate's test SHALL hold the file to the directories and to the code (core.42, principle 16).

#### Scenario: The file names each version directory

- **WHEN** `spoken.json` is read and `protocol/` listed
- **THEN** it holds `versions` alone, naming versions 1 to 7 once each, lowest first, and versions 1 to 6 are not spoken

#### Scenario: The runner speaks what the file marks spoken

- **WHEN** the crate's test reads `spoken.json`
- **THEN** it marks one version spoken, `protocol::spoken()`, which is version 5, 6 or 7, and the upgrade offers its subprotocol

### Requirement: The runner speaks the one version spoken.json marks spoken

`protocol::spoken()` SHALL be the one version `spoken.json` marks spoken. Every message the runner sends or reads, on a socket or by a request, SHALL be of that version. Each upgrade SHALL ask for its subprotocol alone. The crate's tests SHALL pass with either version marked spoken, so a change to the mark alone moves the runner (core.42, Boom: `the-cloud-leads-the-runners-protocol`).

#### Scenario: The upgrade's subprotocol

- **WHEN** a partition's connection is dialed
- **THEN** its upgrade asks for the subprotocol of the version spoken alone, `boomdrive.v7` while version 7 is marked spoken
