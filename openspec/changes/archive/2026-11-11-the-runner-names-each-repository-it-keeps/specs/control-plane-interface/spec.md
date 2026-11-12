## ADDED Requirements

### Requirement: A hello names each drive's repositories, and reads with none where it names none

A hello SHALL carry each drive the runner names, with each repository's address and its clone's container, and SHALL round-trip through JSON unchanged. A hello's JSON that names no drives SHALL read as naming no drive.

#### Scenario: A hello naming its drives

- **WHEN** a hello names the drive `home` with `example/crew` at `/data/repositories/example/crew`, and the drive `work` with none
- **THEN** it round-trips through JSON unchanged

#### Scenario: A hello from before its drives

- **WHEN** a hello's JSON names no drained repository and no drives
- **THEN** it reads naming none drained and no drive
