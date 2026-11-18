## RENAMED Requirements

- FROM: `### Requirement: Version 3 is version 2 whole, but for a hold`
- TO: `### Requirement: Version 3 is version 2 whole, but for a hold and an observing connection`

## MODIFIED Requirements

### Requirement: Version 3 is version 2 whole, but for a hold and an observing connection

Version 3's files SHALL be version 2's but for these alone: each `$id` under `boomdrivedev/boomdrive`'s `…/protocol/v3/`, `protocol_version` 3, a hold among `act_sort`'s branches, the observing hello and the observed report beside the hello and the report, and their descriptions. Its examples SHALL be version 2's, each naming `protocol_version` 3 and `enroll.json` signed over version 3's context, a `pulled` holding a hold, and three of the observing connection (core.42; records 0048 and 0052).

#### Scenario: Version 3 is version 2 but for its hold

- **WHEN** each schema file of version 3 is read with every `description` set aside, its `$id` read under version 2's address, its `protocol_version` read as 2, the hold's branch taken out of `act_sort`, `observing_hello`, `observed_report` and `observed_launch` taken out of its `$defs`, and the `hello` and `report` kinds each read as the reference to its one working form
- **THEN** it equals version 2's file of the same name with every `description` set aside; the hold's branch is `ActSort::Hold` as the interface writes it; `observing_hello` is `hello` with `observing` in place of `capacity` and `leases`; and `observed_launch` is `launched` without `lease`

#### Scenario: Version 3's examples are version 2's

- **WHEN** version 3's examples are read
- **THEN** their names are version 2's, `to-runner/pulled-with-a-hold.json`, `from-runner/hello-observing.json`, `from-runner/report-observed.json` and `to-runner/pulled-answering-an-observing-hello.json`, and each of version 2's names, read with `protocol_version` as 2 and `enroll.json`'s signature set aside, equals version 2's

### Requirement: A message holds only what its version names

Every object a version names SHALL take only the fields it names, at every depth, but a sluice row's `flaky` and a thing a stage's end made, each carried as its tool printed it. Every field the interface's types always write SHALL be required. A time of the runner's or the control plane's SHALL be RFC 3339 in UTC to the second. An integer SHALL be within 2^53 − 1 of zero, and a lease's number SHALL be 1 or more (core.42).

#### Scenario: A field the version does not name

- **WHEN** examples of version 3 with a field added at the envelope, in a launch, in a lease, in a place's status copy, in an end's delivery, in a confirm, in a hold, in a preview, in what the runner answered of a preview, in a preview's copy, in a release's copy, in a key share, in an observing hello and in an observed report's launch are checked
- **THEN** each is refused

#### Scenario: A time not in UTC to the second

- **WHEN** a lease expiring at `2026-11-01T12:00:00+02:00`, and one at `2026-11-01T10:00:00.5Z`, are checked against version 3
- **THEN** each is refused

#### Scenario: An integer past what JavaScript reads exactly

- **WHEN** a lease numbered 2^53, and one numbered 0, are checked against version 3
- **THEN** each is refused

## ADDED Requirements

### Requirement: Version 3 carries an observing hello and an observed report

Version 3's `hello` kind SHALL take the hello or an observing hello, `observing` true with the hello's versions, tool versions and repositories and no capacity or leases. Its `report` kind SHALL take the report or an observed report, `observing` true with `launches`, each a launch's answer, status and entry with no lease, and nothing else. Each form SHALL be closed, so none passes as another (core.10, core.42; records 0048 and 0052 as amended on 2026-11-18).

#### Scenario: The observing connection's examples

- **WHEN** version 3's `hello-observing.json`, `report-observed.json` and `pulled-answering-an-observing-hello.json` are checked against their direction's envelope of version 3
- **THEN** each passes

#### Scenario: An observing hello that offers room or holds a lease

- **WHEN** version 3's observing hello with `capacity` added, one with `leases` added, and its hello with `observing` added are each checked against version 3's `from-runner.schema.json`
- **THEN** each is refused

#### Scenario: An observed report that names a lease or an act

- **WHEN** version 3's observed report with a `lease` added to its launch, one with `acts` added, and its report with `observing` added are each checked against version 3's `from-runner.schema.json`
- **THEN** each is refused

#### Scenario: No observing connection in version 1 or 2

- **WHEN** version 3's `hello-observing.json` and `report-observed.json`, each naming `protocol_version` 2, are checked against version 2's envelope, and each naming 1 against version 1's
- **THEN** each is refused
