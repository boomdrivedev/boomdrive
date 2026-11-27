## ADDED Requirements

### Requirement: A hello names each kept repository's status, and reads with none where it names none

A hello SHALL carry each kept repository's status by its address, printed as Flume printed it or missing with words. It SHALL round-trip through JSON unchanged. A hello's JSON that names no statuses SHALL read as naming none.

#### Scenario: A hello naming a repository's status

- **WHEN** a hello names `example/crew`'s status as Flume printed it
- **THEN** it round-trips through JSON unchanged, as a missing status does
