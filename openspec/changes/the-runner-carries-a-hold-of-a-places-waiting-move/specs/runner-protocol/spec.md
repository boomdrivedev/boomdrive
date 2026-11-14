## MODIFIED Requirements

### Requirement: The messages carry the interface's shapes

Version 2 SHALL carry the interface's `Id`, `Lease`, `Want`, `ToolVersion`, `Answer`, `Acted`, `Commanded` and `InboxCarried` as `boomdrive-control`'s serde types write them, every want and act sort the interface names among them but a hold, and its `Hello` and `Report` but for their copies. Version 1 SHALL carry the same but a preview, a land, a confirm and `previews`. An acted's and a commanded's outcome SHALL be `done` or `refused` (boomdrive-cloud record 0056; record 0037 as amended on 2026-11-09; core.59).

#### Scenario: The interface's values pass

- **WHEN** one value of each want, act and command sort version 2 names, a preview not done and one done, a land and a confirm among them, a lease with an expiry and one without, a launch with every field and one with none, each tool version, an acted, a commanded and an inbox carried are written by `boomdrive-control`'s types and checked against their definitions in version 2
- **THEN** each passes

#### Scenario: Every sort the interface names

- **WHEN** a value of every sort of `Want`, `ActSort` and `CommandSort`, each named by a match with no wildcard, is written by `boomdrive-control`'s types and checked against version 2's `want`
- **THEN** the sorts named are every sort the interface has, a hold among them, and each value passes but the hold's, which is refused

#### Scenario: An act's outcome

- **WHEN** an acted, and a commanded, whose outcome is `launched` are checked against version 2
- **THEN** each is refused

## ADDED Requirements

### Requirement: No version carries a hold

Version 1 and version 2 name no hold, and a release has spoken each, so neither SHALL change to carry one. A want whose act is a hold SHALL fail each version's check, and a hold SHALL reach the runner from a hosted control plane only in a later version that names it (core.42, core.59).

#### Scenario: A hold is no act of version 1 or 2

- **WHEN** a want whose act is a hold, as `boomdrive-control`'s types write it, is checked against version 1's `want` and against version 2's
- **THEN** each refuses it
