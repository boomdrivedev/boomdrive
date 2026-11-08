## ADDED Requirements

### Requirement: Version 1 carries no land

Version 1 names no land, and a release has spoken it, so it SHALL NOT change to carry one. A want whose act is a land SHALL fail version 1's check, and a land SHALL reach the runner from a hosted control plane only in a later version that names it (core.42, core.53).

#### Scenario: A land is no act of version 1

- **WHEN** a want whose act is a land, as `boomdrive-control`'s types write it, is checked against version 1's `want`
- **THEN** it is refused

## MODIFIED Requirements

### Requirement: The messages carry the interface's shapes

Version 1 SHALL carry the interface's `Id`, `Lease`, `Want`, `ToolVersion`, `Answer`, `Acted`, `Commanded` and `InboxCarried` as `boomdrive-control`'s serde types write them, a want whose act is a land apart, and its `Hello` and `Report` but for their copies. An acted's and a commanded's outcome SHALL be `done` or `refused` (boomdrive-cloud record 0056).

#### Scenario: The interface's values pass

- **WHEN** one value of each want, act and command sort version 1 names, every act sort but a land, a lease with an expiry and one without, a launch with every field and one with none, each tool version, an acted, a commanded and an inbox carried are written by `boomdrive-control`'s types and checked against their definitions in version 1
- **THEN** each passes

#### Scenario: An act's outcome

- **WHEN** an acted, and a commanded, whose outcome is `launched` are checked
- **THEN** each is refused
