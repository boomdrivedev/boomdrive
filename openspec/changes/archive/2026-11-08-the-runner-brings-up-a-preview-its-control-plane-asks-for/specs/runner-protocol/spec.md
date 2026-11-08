## ADDED Requirements

### Requirement: Version 1 carries no preview

Version 1 names no preview, and a release has spoken it, so it SHALL NOT change to carry one. A want that is a preview SHALL fail version 1's check, and a preview SHALL reach the runner from a hosted control plane only in a later version that names it. The runner's report SHALL go up in version 1 with no `previews` (core.42, core.55).

#### Scenario: A preview is no want of version 1

- **WHEN** a want that is a preview, as `boomdrive-control`'s types write it, is checked against version 1's `want`
- **THEN** it is refused

## MODIFIED Requirements

### Requirement: The messages carry the interface's shapes

Version 1 SHALL carry the interface's `Id`, `Lease`, `Want`, `ToolVersion`, `Answer`, `Acted`, `Commanded` and `InboxCarried` as `boomdrive-control`'s serde types write them, a want that is a preview apart, and its `Hello` and `Report` but for their copies, the report's previews apart. An acted's and a commanded's outcome SHALL be `done` or `refused` (boomdrive-cloud record 0056).

#### Scenario: The interface's values pass

- **WHEN** one value of each want sort but a preview, and of each act and command sort, a lease with an expiry and one without, a launch with every field and one with none, each tool version, an acted, a commanded and an inbox carried are written by `boomdrive-control`'s types and checked against their definitions in version 1
- **THEN** each passes

#### Scenario: An act's outcome

- **WHEN** an acted, and a commanded, whose outcome is `launched` are checked
- **THEN** each is refused
