## MODIFIED Requirements

### Requirement: The runner speaks the one version spoken.json marks spoken

`protocol::spoken()` SHALL be the one version `spoken.json` marks spoken. Every message the runner sends or reads, on a socket or by a request, SHALL be of that version. Each upgrade SHALL ask for its subprotocol alone. The crate's tests SHALL pass with either version marked spoken, so a change to the mark alone moves the runner (core.42, Boom: `the-cloud-leads-the-runners-protocol`).

#### Scenario: The upgrade's subprotocol

- **WHEN** a partition's connection is dialed
- **THEN** its upgrade asks for the subprotocol of the version spoken alone, `boomdrive.v6` while version 6 is marked spoken
