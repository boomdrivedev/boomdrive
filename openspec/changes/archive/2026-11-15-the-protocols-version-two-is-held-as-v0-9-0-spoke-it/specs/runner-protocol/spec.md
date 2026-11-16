## ADDED Requirements

### Requirement: Version 2 stays as a release spoke it

Every file under `protocol/v2/`, its schemas and its examples, SHALL stay as release `v0.9.0` holds it, and no file SHALL be added there. Every release from `v0.8.0`, the first to speak version 2, holds the same files. A change to version 2 SHALL be a new version beside it (core.42; record 0037).

#### Scenario: Version 2 as released

- **WHEN** each file under `protocol/v2/` is read and its SHA-256 taken
- **THEN** the files are exactly those the test lists, each with the digest the test holds for it, as `git show v0.9.0:<path>` gives it

#### Scenario: A file of version 2 changed or added

- **WHEN** a file under `protocol/v2/` differs from `v0.9.0`'s, or a file is added there
- **THEN** `boomdrive-hosted`'s test `version_2_is_as_released` fails, naming the file
