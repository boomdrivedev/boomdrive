## MODIFIED Requirements

### Requirement: A message holds only what its version names

Every object a version names SHALL take only the fields it names, at every depth, but a sluice row's `flaky` and a thing a stage's end made, each carried as its tool printed it. Every field the interface's types always write SHALL be required. A time of the runner's or the control plane's SHALL be RFC 3339 in UTC to the second. An integer SHALL be within 2^53 − 1 of zero, and a lease's number SHALL be 1 or more (core.42).

#### Scenario: A field the version does not name

- **WHEN** examples of version 2 with a field added at the envelope, in a launch, in a lease, in a place's status copy, in an end's delivery, in a confirm, in a preview, in what the runner answered of a preview, in a preview's copy, in a release's copy and in a key share are checked
- **THEN** each is refused

#### Scenario: A time not in UTC to the second

- **WHEN** a lease expiring at `2026-11-01T12:00:00+02:00`, and one at `2026-11-01T10:00:00.5Z`, are checked against version 2
- **THEN** each is refused

#### Scenario: An integer past what JavaScript reads exactly

- **WHEN** a lease numbered 2^53, and one numbered 0, are checked against version 2
- **THEN** each is refused

### Requirement: The messages carry the interface's shapes

Version 2 SHALL carry the interface's `Id`, `Lease`, `Want`, `ToolVersion`, `Answer`, `Acted`, `Commanded` and `InboxCarried` as `boomdrive-control`'s serde types write them, every want sort and every act sort the interface names among them, a preview, a land and a confirm, and its `Hello` and `Report` but for their copies. Version 1 SHALL carry the same, but for a preview, a land and a confirm, and its `Report` with no `previews`. An acted's and a commanded's outcome SHALL be `done` or `refused` (boomdrive-cloud record 0056; record 0037 as amended on 2026-11-09).

#### Scenario: The interface's values pass

- **WHEN** one value of each want, act and command sort version 2 names, a preview not done and one done, a land and a confirm among them, a lease with an expiry and one without, a launch with every field and one with none, each tool version, an acted, a commanded and an inbox carried are written by `boomdrive-control`'s types and checked against their definitions in version 2
- **THEN** each passes

#### Scenario: Every sort the interface names

- **WHEN** a value of every sort of `Want`, `ActSort` and `CommandSort`, each named by a match with no wildcard, is written by `boomdrive-control`'s types and checked against version 2's `want`
- **THEN** the sorts named are every sort the interface has, and each value passes

#### Scenario: An act's outcome

- **WHEN** an acted, and a commanded, whose outcome is `launched` are checked against version 2
- **THEN** each is refused

### Requirement: The README names the two paths and the signatures' contexts

The protocol's README SHALL describe version 2: `runner/enroll` and `runner/connect` under the control plane's address, the kinds each takes, the subprotocol `boomdrive.v2`, the `BoomdriveHost` header, and the bytes each signature of enrollment and of a connection is made over, each context starting `boomdrive.v2`, with a worked example of each. It SHALL name what a `pulled` and a `report` hold, and SHALL say how version 1 differs. The hosted implementation SHALL make each signature over exactly those bytes (records 0018 and 0037).

#### Scenario: The README's worked contexts

- **WHEN** the hosted implementation makes the enrollment's and the connection's contexts from the README's fixed key, token, origin, time and nonce
- **THEN** each equals the README's worked bytes, and each starts with the bytes of `boomdrive.v2`

#### Scenario: The README of version 2

- **WHEN** the protocol's README is read
- **THEN** its envelope names `protocol_version` 2; its paths name the subprotocol `boomdrive.v2`; its files name `preview.schema.json`; its copies name an end's `delivery`, an end's delivery alone and a preview's copy; its messages name a confirm and a land among a `pulled`'s acts, a preview among its wants, and `previews` in a `report`; and a section `Version 1` names `boomdrive.v1` and says it has no land and no preview

## ADDED Requirements

### Requirement: Version 2 carries a land

Version 2 SHALL name a land among the acts a `pulled` holds, written as the word `land`, as `boomdrive-control`'s `ActSort::Land` writes it, holding nothing but its act's launch and lease. The retry naming the submission SHALL stay among version 2's acts (core.42, core.53; record 0037 as amended on 2026-11-09).

#### Scenario: A land is an act of version 2

- **WHEN** a want whose act is a land, as `boomdrive-control`'s types write it, is checked against version 2's `want`, and a `pulled` holding it against version 2's `to-runner.schema.json`
- **THEN** each passes, the act's `what` being `"land"`

### Requirement: Version 2 carries a preview and its done

Version 2 SHALL name a preview among the wants a `pulled` holds, `{"preview":{"id","lease","repository","branch","revision","done"}}`, as `boomdrive-control`'s `Want::Preview` writes it, each field required. Its branch and its revision SHALL be any string, so that a preview sluice refuses reaches the runner as a preview, and not the whole `pulled` as a message (core.42, core.55; record 0037 as amended on 2026-11-09).

#### Scenario: A preview is a want of version 2

- **WHEN** a preview not done, one done, and one whose revision is `R1`, as `boomdrive-control`'s types write them, are checked against version 2's `want`, and a `pulled` holding a preview and its done against version 2's `to-runner.schema.json`
- **THEN** each passes

### Requirement: Version 2's report holds what the runner answered of each preview

Every report of version 2 SHALL hold `previews`, by id, each preview the runner answered with its `lease`, `repository`, `revision`, `done`, `outcome` (`held`, `done` or `refused`), `copy`, `words`, `status` and `at`, each required: `copy` the copy of sluice's answer, a preview's or a release's, or `null`; `status` the copy of sluice's listing with its time and whether it was listed, or `null` (core.27, core.42, core.55; record 0037 as amended on 2026-11-09).

#### Scenario: sluice's answers of a preview as copies

- **WHEN** the `report` examples of version 2 are checked: a preview brought up, a preview held, a preview put away, and a preview sluice refused
- **THEN** each passes

#### Scenario: A report with no previews

- **WHEN** a `report` of version 2 with no `previews` is checked
- **THEN** it is refused

#### Scenario: A preview's outcome

- **WHEN** what the runner answered of a preview, its outcome `launched`, is checked against version 2
- **THEN** it is refused

### Requirement: A preview's copy holds no output of its command

Version 2's `preview.schema.json` SHALL hold the copy of a preview, its `name`, required, `branch`, `revision`, `state`, `made` and `addresses`, a map of strings, and the copy of a release, its `preview`, required, and `released`, each as sluice printed it. Neither SHALL hold a preview's `checkout`, `names`, `ports`, `exit`, `output` or `release_failed` (core.11, core.27, core.55; record 0037 as amended on 2026-11-09).

#### Scenario: The copies' fields are the lists

- **WHEN** the field paths of `preview.schema.json`'s `preview` and `release` are read
- **THEN** the first's are `name`, `branch`, `revision`, `state`, `made` and `addresses`, and the second's `preview` and `released`, which the tests hold

#### Scenario: A preview's output

- **WHEN** a report whose preview's copy, or whose preview's status copy, holds `output`, `checkout`, `names`, `ports`, `exit` or `release_failed` is checked against version 2
- **THEN** each is refused

## REMOVED Requirements

### Requirement: Version 2 carries no land and no preview

**Reason**: Version 2 carries a land and a preview with its done before its first release, so a hosted control plane asks what crew's can and no later version is needed for them (record 0037 as amended on 2026-11-09).

**Migration**: A control plane asks a land as version 2's act `"land"`, and a preview as its want `preview`, and reads what the runner answered of each preview from the report's `previews` ("Version 2 carries a land", "Version 2 carries a preview and its done", "Version 2's report holds what the runner answered of each preview"). Version 1 still carries neither.
