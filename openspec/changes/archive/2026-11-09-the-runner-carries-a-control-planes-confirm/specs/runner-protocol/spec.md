## ADDED Requirements

### Requirement: Version 1 carries no confirm

Version 1 names no confirm, and its copy of a place's status names no end's delivery key, and a release has spoken it, so it SHALL NOT change to carry either. A want whose act is a confirm SHALL fail version 1's check, and a confirm SHALL reach the runner from a hosted control plane only in a later version that names it (core.42, core.56).

#### Scenario: A confirm is no act of version 1

- **WHEN** a want whose act is a confirm, as `boomdrive-control`'s types write it, is checked against version 1's `want`
- **THEN** it is refused

## MODIFIED Requirements

### Requirement: The messages carry the interface's shapes

Version 1 SHALL carry the interface's `Id`, `Lease`, `Want`, `ToolVersion`, `Answer`, `Acted`, `Commanded` and `InboxCarried` as `boomdrive-control`'s serde types write them, a want that is a preview and one whose act is a land or a confirm apart, and its `Hello` and `Report` but for their copies, the report's previews apart. An acted's and a commanded's outcome SHALL be `done` or `refused` (boomdrive-cloud record 0056).

#### Scenario: The interface's values pass

- **WHEN** one value of each want, act and command sort version 1 names, every want sort but a preview and every act sort but a land and a confirm, a lease with an expiry and one without, a launch with every field and one with none, each tool version, an acted, a commanded and an inbox carried are written by `boomdrive-control`'s types and checked against their definitions in version 1
- **THEN** each passes

#### Scenario: An act's outcome

- **WHEN** an acted, and a commanded, whose outcome is `launched` are checked
- **THEN** each is refused
