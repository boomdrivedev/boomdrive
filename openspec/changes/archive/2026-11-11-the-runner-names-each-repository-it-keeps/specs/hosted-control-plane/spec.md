## ADDED Requirements

### Requirement: No hello names a drive or a clone's folder

A partition's hello SHALL name no drive and no clone's folder, since version 2 carries neither and a path goes up only where the protocol's copy names it (core.27, core.28, core.42).

#### Scenario: No drive or folder goes up

- **WHEN** the runner's hello names the drive `family` with `example/app` at `/data/repositories/example/app`, and `home` names `example/app`
- **THEN** `home`'s hello names `example/app`, and neither `family` nor `/data/repositories/example/app` appears in it
