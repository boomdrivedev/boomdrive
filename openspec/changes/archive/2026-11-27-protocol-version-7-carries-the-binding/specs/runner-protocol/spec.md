## ADDED Requirements

### Requirement: Version 7 is version 6 whole, but for the repositories' statuses

Version 7's files SHALL be version 6's but for three things. Each `$id` is under `…/protocol/v7/`. Each message names `protocol_version` 7. The hello requires `repository_statuses`, with its definitions. Its examples SHALL be version 6's, each naming `protocol_version` 7 and each hello naming its statuses, and one more hello (core.42).

#### Scenario: Version 7 is version 6 but for the repositories' statuses

- **WHEN** each schema file of version 7 is read with every `description` set aside and its `$id` under version 6's address
- **AND** its `protocol_version` is read as 6, and `repository_statuses` and its definitions `repository_status`, `repository_status_copy`, `binding_held`, `binding_read`, `merge_queue_bound` and `kind_lands` are taken out of `hello.schema.json`
- **THEN** it equals version 6's file of the same name, with every `description` set aside

#### Scenario: Version 7's examples are version 6's

- **WHEN** version 7's examples are read
- **THEN** their names are version 6's and `from-runner/hello-with-no-merge-queue-ready.json`, and each of version 6's, its `protocol_version` read as 6 and its hello's `repository_statuses` taken out, equals version 6's

### Requirement: Version 7's hello names each repository's status

Version 7's hello SHALL hold `repository_statuses`, a map that names each address of `repositories` once. Each value SHALL be `{"printed": …}`, the copy of Flume's item of `status.repositories` without its `repository`, or `{"missing": "<words>"}`. The copy SHALL be closed: `binding`, `merge_queue` and `kinds`, each as Flume's report names it. No version before 7 SHALL take the field (core.27, core.42).

#### Scenario: Version 7's examples read

- **WHEN** each example of version 7 is checked against its direction's envelope of version 7 and read into the runner's types
- **THEN** each passes and writes back equal, and each hello names the status of each of its repositories once

#### Scenario: A status of another form

- **WHEN** a hello of version 7 names no `repository_statuses`, or a status that is neither `printed` nor `missing`
- **AND** a copy names the clone's folder, or a field that Flume's report does not name
- **AND** a `from` is not `origin` or `local`, a `ready` is no boolean, or a key is no address
- **AND** version 7's `hello.json` is checked against version 6
- **THEN** each is refused

### Requirement: The runner's code reads and writes versions 5, 6 and 7

`boomdrive_hosted::protocol::v7` SHALL hold the runner's types, which read and write each message of versions 5, 6 and 7. `protocol::V5`, `protocol::V6` and `protocol::V7` SHALL each hold its version's number, subprotocol, address and files (core.42).

#### Scenario: Each version the code speaks is its own files

- **WHEN** `protocol::V5`, `protocol::V6` and `protocol::V7` are read
- **THEN** each holds its module's number, subprotocol, address and files

## MODIFIED Requirements

### Requirement: The runner names the versions it speaks in one file of data

`crates/boomdrive-hosted/protocol/spoken.json` SHALL name each version directory under `protocol/` once, lowest first, as `{"version": N, "spoken": bool}` under `versions`, and nothing else. It SHALL mark spoken exactly one version, which the runner's code reads from the file, and checks, sends and offers. The crate's test SHALL hold the file to the directories and to the code (core.42, principle 16).

#### Scenario: The file names each version directory

- **WHEN** `spoken.json` is read and `protocol/` listed
- **THEN** it holds `versions` alone, naming versions 1 to 7 once each, lowest first, and versions 1 to 5 and 7 are not spoken

#### Scenario: The runner speaks what the file marks spoken

- **WHEN** the crate's test reads `spoken.json`
- **THEN** it marks one version spoken, `protocol::spoken()`, which is version 5, 6 or 7, and the upgrade offers its subprotocol

## REMOVED Requirements

### Requirement: The runner's code reads and writes versions 5 and 6

**Reason**: The runner's code reads and writes versions 5, 6 and 7.

**Migration**: "The runner's code reads and writes versions 5, 6 and 7" holds the types and the versions.
