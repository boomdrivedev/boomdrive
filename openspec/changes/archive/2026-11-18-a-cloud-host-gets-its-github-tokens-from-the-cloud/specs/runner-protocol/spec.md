## RENAMED Requirements

- FROM: `### Requirement: The README names the two paths and the signatures' contexts`
- TO: `### Requirement: The README names the paths and the signatures' contexts`

## MODIFIED Requirements

### Requirement: Each version of the protocol is JSON Schema in the hosted implementation's crate

The runner's protocol with a hosted control plane SHALL be JSON Schema of draft 2020-12, one directory per version under `crates/boomdrive-hosted/protocol/`: `v1/`, `v2/`, `v3/` and `v4/`. Each schema file SHALL name its own address as its `$id`. Every `$ref` SHALL resolve among its version's files, and no check SHALL fetch one. Each version's module SHALL hold in `FILES` every schema file of its version, and nothing else (core.42; records 0037, 0052 and 0053; boomdrive-cloud record 0056).

#### Scenario: Each file is a schema of its draft

- **WHEN** each file of each version is checked against the draft 2020-12 metaschema
- **THEN** each passes, and its `$id` is its own address under its version's directory

#### Scenario: Nothing outside the version is referenced

- **WHEN** validators are built from one version's `FILES` alone, with nothing fetched, for each version
- **THEN** every `$ref` of every file of that version resolves

#### Scenario: Every file is listed

- **WHEN** the files under each version's directory ending `.schema.json` are compared with that version's `FILES`
- **THEN** each is in `FILES`, and each in `FILES` is among them

### Requirement: Each direction sends only its own kinds

A runner SHALL send only `enroll`, `hello`, `report`, `terminal_opened`, `terminal_output`, `terminal_closed` and `message_refused`, and in version 4 `forge_token`. A control plane SHALL send only `enrolled`, `enroll_refused`, `pulled`, `reported`, `terminal_open`, `terminal_resume`, `terminal_input`, `terminal_close` and `message_refused`, and in version 4 `forge_token_given` and `forge_token_refused`. Each direction's envelope SHALL take its own kinds alone (core.42; record 0053).

#### Scenario: A body of the other direction

- **WHEN** a `pulled` is checked as a runner's message, and a `hello` as a control plane's
- **THEN** each is refused

#### Scenario: Enrollment's kinds in the other direction

- **WHEN** an `enroll` is checked as a control plane's message, and an `enrolled` and an `enroll_refused` as a runner's
- **THEN** each is refused

#### Scenario: A forge token's kinds in the other direction

- **WHEN** version 4's `forge_token` is checked as a control plane's message of version 4, and its `forge_token_given` and `forge_token_refused` as a runner's
- **THEN** each is refused

### Requirement: No version names a grant or a credential

No name of any version SHALL hold the word `grant`, `role`, `permission`, `token`, `credential`, `secret` or `password`, but version 4's forge token: its kinds `forge_token`, `forge_token_given` and `forge_token_refused`, each answer's `forge_token`, and `forge_token_given`'s `token`. A connection's account, and what it may do, SHALL come from its enrollment, never from a message (core.4; record 0053; boomdrive-cloud records 0047 and 0065).

#### Scenario: Every name read

- **WHEN** every property name, `properties` key, `$defs` key and `required` field of every file of each version is read
- **THEN** none holds one of those words but version 4's forge token's names, and each of those is found in version 4 alone

#### Scenario: A word caught in any name

- **WHEN** the names `roles`, `grant`, `Secret` and `credential_file` are read
- **THEN** each is caught as holding one of those words

### Requirement: The runner's types write and read only what the schema takes

`boomdrive_hosted::protocol::v4` SHALL hold a type for every message of version 4, using the interface's own types wherever version 4 carries a shape as the interface writes it, and the runner SHALL speak no other version. Every example of version 4 SHALL read into its direction's type and write back to an equal value that passes the schema, and every kind SHALL have an example (research row F50; records 0037, 0052 and 0053).

#### Scenario: Every example read and written back

- **WHEN** each example under version 4's `examples/from-runner/` and `examples/to-runner/` is read into its direction's `Message`, written back, and the written value checked
- **THEN** the written value equals the example's as JSON and passes its direction's schema

#### Scenario: Every kind has an example

- **WHEN** the kinds of version 4's examples are read
- **THEN** every kind of each direction is among them, `forge_token`, `forge_token_given` and `forge_token_refused` with them

#### Scenario: A token's Debug names its end alone

- **WHEN** a `forge_token_given` read from version 4's example is written with `{:?}`
- **THEN** the text names its repository and its end, and holds no character of its token

### Requirement: The README names the paths and the signatures' contexts

The protocol's README SHALL describe version 4: `runner/enroll`, `runner/connect` and `runner/forge-token`, the kinds each takes, `boomdrive.v4`, the `BoomdriveHost` header, a `pulled`, a `report`, a forge token's ask and answers, how versions 3, 2 and 1 differ, and the bytes each signature covers, each context starting `boomdrive.v4`, with a copy of the worked examples the tests read (record 0049). The hosted implementation SHALL sign exactly those bytes (records 0018, 0052 and 0053).

#### Scenario: The README's worked contexts

- **WHEN** the hosted implementation makes the enrollment's, the connection's and the ask's contexts from the fixed key, token, origin, time, nonce and repository of the worked examples, `crates/boomdrive-hosted/tests/worked-bytes.json`
- **THEN** each equals the worked examples' bytes, and each starts with the bytes of `boomdrive.v4`

#### Scenario: The README of version 2

- **WHEN** the protocol's README is read
- **THEN** a section `Version 2` names `boomdrive.v2` and the releases that spoke it, and says it is version 3 but for its subprotocol, its contexts, its files' `$id`s and the hold, which it does not have

#### Scenario: The README of version 3

- **WHEN** the protocol's README is read
- **THEN** a section `Version 3` names `boomdrive.v3` and the releases that spoke it, and says it is version 4 but for its subprotocol, its contexts, its files' `$id`s and a forge token's ask and answers, which it does not have

#### Scenario: The README of version 4

- **WHEN** the protocol's README is read
- **THEN** its envelope names `protocol_version` 4; its paths name `runner/forge-token` and the subprotocol `boomdrive.v4`; its files name `forge-token.schema.json`; its messages name `forge_token`, `forge_token_given` and `forge_token_refused`, and say none goes on the socket; its signatures name the context `forge token` and its fields; and sections `Version 2` and `Version 1` stay

## ADDED Requirements

### Requirement: Version 4 is version 3 whole, but for a forge token's ask and answers

Version 4's files SHALL be version 3's but for these alone: each `$id` under `…/protocol/v4/`, `protocol_version` 4, a twelfth file, `forge-token.schema.json`, the three kinds it holds among `from-runner.schema.json`'s and `to-runner.schema.json`'s bodies, and their descriptions. Its examples SHALL be version 3's, each naming `protocol_version` 4 and `enroll.json` signed over version 4's context, and four of a forge token (core.42; record 0053).

#### Scenario: Version 4 is version 3 but for a forge token

- **WHEN** each schema file of version 4 but `forge-token.schema.json` is read with every `description` set aside, its `$id` read under version 3's address, its `protocol_version` read as 3, and the forge token's kinds taken out of the bodies
- **THEN** it equals version 3's file of the same name with every `description` set aside, and the kinds taken out are exactly `forge_token` from the runner's and `forge_token_given` and `forge_token_refused` from the control plane's

#### Scenario: Version 4's examples are version 3's

- **WHEN** version 4's examples are read
- **THEN** their names are version 3's, `from-runner/forge_token.json`, `to-runner/forge_token_given.json`, `to-runner/forge_token_refused-not-installed.json` and `to-runner/forge_token_refused-a-repository-no-drive-serves.json`, and each of version 3's names, read with `protocol_version` as 3 and `enroll.json`'s signature set aside, equals version 3's

### Requirement: Version 4 carries a forge token's ask and its answers

Version 4 SHALL name the runner's ask, `forge_token`, holding `repository`, an address; the answer `forge_token_given`, holding `forge_token`, the ask's message id, `repository`, `token`, a string of one character or more, and `expires_at`, a time; and `forge_token_refused`, holding `forge_token`, `repository`, `not_installed`, a boolean, and `words`. Each SHALL be closed, every field required (core.33, core.42; record 0053).

#### Scenario: A forge token's examples

- **WHEN** version 4's `forge_token.json`, `forge_token_given.json` and both `forge_token_refused` examples are checked against their direction's envelope of version 4
- **THEN** each passes

#### Scenario: A forge token's ask or answer that is not whole

- **WHEN** version 4's ask with a `path` added, one whose `repository` is `flume`, its `forge_token_given` with no `expires_at`, one whose `expires_at` is `2026-11-18T12:00:00+02:00`, one whose `token` is empty, and its `forge_token_refused` with no `not_installed` are checked against their direction's envelope
- **THEN** each is refused

### Requirement: Version 3 stays as a release spoke it

Every file under `protocol/v3/`, its schemas and its examples, SHALL stay as release `v0.18.0` holds it, and no file SHALL be added there. Every release from `v0.18.0`, the first to speak version 3, to the last before version 4 holds the same files. Every example of version 3 SHALL still pass its direction's envelope of version 3. `boomdrive_hosted::protocol::v3` SHALL hold version 3's files and no type (core.42; records 0052 and 0053).

#### Scenario: Version 3 as released

- **WHEN** each file under `protocol/v3/` is read and its SHA-256 taken
- **THEN** the files are exactly those the test lists, each with the digest the test holds for it, as `git show v0.18.0:<path>` gives it

#### Scenario: A file of version 3 changed or added

- **WHEN** a file under `protocol/v3/` differs from `v0.18.0`'s, or a file is added there
- **THEN** `boomdrive-hosted`'s test `version_3_is_as_released` fails, naming the file

#### Scenario: Version 3's examples still pass

- **WHEN** each example under version 3's `examples/` is checked against its direction's envelope of version 3
- **THEN** each passes

### Requirement: Versions 1 to 3 carry no forge token

Versions 1, 2 and 3 name no forge token, and a release has spoken each, so none SHALL change to carry one. A forge token's ask or answer SHALL fail each one's check, and a cloud host's token SHALL reach the runner only in version 4 or a later version (core.33, core.42; record 0053).

#### Scenario: A forge token is no message of version 1, 2 or 3

- **WHEN** version 4's `forge_token.json`, `forge_token_given.json` and `forge_token_refused-not-installed.json`, each naming `protocol_version` 3, are checked against version 3's envelope of their direction, each naming 2 against version 2's, and each naming 1 against version 1's
- **THEN** each is refused
