## ADDED Requirements

### Requirement: Each repository's status goes up in its own partition's hello

From version 7, each partition's hello SHALL name the status of each of its own repositories by its address, and no other status (core.26). A printed status SHALL go up as its copy, cut by version 7's `repository_status_copy` and checked. The copy SHALL hold no folder of the clone and no field that the copy does not name (core.27). A status the runner gives as missing SHALL go up missing, with its words. Before version 7, a hello SHALL name no `repository_statuses` (core.42).

#### Scenario: Each repository's status goes up in its own partition's hello

- **WHEN** the runner's hello names `example/app`, whose status Flume printed with its folder and a field Flume added
- **AND** it names `example/service`, whose status is missing, and `home` serves `example/app` while `work` serves both
- **THEN** under version 7, `home`'s hello names `example/app`'s copy alone, without the folder or the added field
- **AND** `work`'s hello names that copy, and `example/service` missing with the runner's words
- **AND** under version 6, neither hello names `repository_statuses`

### Requirement: A repository's status whose copy fails goes up missing

From version 7, a printed status whose copy fails version 7's check SHALL go up missing, with words that name the path and no value. The plane SHALL say one line while it fails. A repository of the hello whose status the runner gives none of SHALL go up missing, with words that say so (core.11, core.23).

#### Scenario: A repository's status whose copy fails goes up missing

- **WHEN** under version 7 the runner's hello gives `example/app` a status whose `kinds` is no list, and names no status of `example/service`, at two pulls
- **THEN** `example/app` goes up missing, its words naming `/kinds` and no value, `example/service` goes up missing with `the runner read no status of the repository from Flume`, and one line names the failing copy

### Requirement: A hello of version 7 names its repositories' statuses

A connection of version 7 SHALL ask for `boomdrive.v7` at its upgrade and send its hello with each repository's status as it was handed. A connection of an earlier version SHALL send no hello that names a status, and SHALL say why it is not sent (core.42).

#### Scenario: A hello of version 7 names its repositories' statuses

- **WHEN** `home` speaks version 7 and is handed a hello naming its repositories' statuses
- **AND** a connection of version 6 is handed the same hello
- **THEN** the first upgrade asks for `boomdrive.v7` and its hello names the statuses as handed, and the second refuses the hello with `the hello fails version 6's check, so it is not sent`
