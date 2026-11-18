## RENAMED Requirements

- FROM: `### Requirement: What goes up is version 2's copy, checked before it goes`
- TO: `### Requirement: What goes up is version 3's copy, checked before it goes`

## MODIFIED Requirements

### Requirement: A partition is enrolled once by boomdrive enroll

`boomdrive enroll <label>` SHALL read the token from standard input, make an Ed25519 key in memory, and send one `POST runner/enroll` under `url`, the token its bearer and its body an `enroll` of version 3. It SHALL keep the key only once `enrolled` answers, print `enrolled` or the refusal's words as given, and write no configuration and nothing in the data directory (core.31, core.39; records 0006, 0037 and 0052).

#### Scenario: A partition enrolled

- **WHEN** partition `home` is enrolled with a token, and the fake control plane takes the `enroll` and answers `enrolled` with host `h-1`
- **THEN** the request bore the token, its `enroll` passed version 3 and its signature checked over version 3's context, the key is in the store under `home`, and the enrollment names host `h-1`

#### Scenario: A lapsed token

- **WHEN** the fake control plane answers `enroll_refused` with the words `the token has lapsed`
- **THEN** the enrollment is refused with `the token has lapsed`, as the control plane's refusal and not the host's to fix, and the store is as it was

#### Scenario: The answer as JSON

- **WHEN** `boomdrive enroll --json home` is given a token and `url` names a loopback port with nothing on it
- **THEN** standard output is one object, `{"refused":{"refusal":{"reason":…}}}`, its reason naming the control plane as unreachable, and it exits 1

#### Scenario: The token never shown

- **WHEN** `boomdrive enroll home` is given the token `secret-enrollment-token`, and `url` names a loopback port with nothing on it
- **THEN** neither standard output nor standard error holds `secret-enrollment-token`

### Requirement: Every connection proves the host's key

Each upgrade of `runner/connect` SHALL ask for `boomdrive.v3` alone and carry `Authorization: BoomdriveHost` with the key, the time, a nonce and a signature over version 3's context, made afresh for each attempt. The runner SHALL keep nothing of enrollment but the key, which it proves under version 3 whichever version enrolled it. A key or a subprotocol refused SHALL be said once and tried again at the backoff's cap (core.22, core.23, core.31, core.35; records 0037 and 0052).

#### Scenario: A connection proved

- **WHEN** partition `home` dials the fake control plane twice
- **THEN** each upgrade asks for `boomdrive.v3` alone, its signature checks over its context of version 3 with `home`'s key, and the two nonces differ

#### Scenario: A key refused

- **WHEN** the fake control plane refuses `home`'s key three times
- **THEN** one line names `home`, the fake's words and `boomdrive enroll home`

#### Scenario: A control plane that speaks version 1 alone

- **WHEN** the fake control plane answers `home`'s upgrades without the subprotocol, three times
- **THEN** one line names `home` and `boomdrive.v3`, each later attempt waits for the backoff's cap, and no message is sent

### Requirement: A partition's connection carries only its own repositories and work

Each partition's hello SHALL name only the repositories its table names, of those kept with their clones made, and only its own leases. No hello SHALL name a repository the runner drains, and no report the repository a launch was answered in, since version 3 carries neither. Each report SHALL carry only the entries under its prefix. A want naming a repository the partition does not name SHALL not be given to the runner, and one line SHALL name it (core.26; ledger 3.b).

#### Scenario: Each hello its own

- **WHEN** the runner keeps `example/app` and `example/service`, `home` names `example/app`, and the report holds `home/l-1` and `work/l-2`
- **THEN** `home`'s hello names `example/app` and lease `l-1` alone

#### Scenario: Another partition's repository

- **WHEN** `home`'s control plane offers a launch on `example/service`, which `home` does not name
- **THEN** the pull does not give it, and one line names the offer, `home` and `example/service`

#### Scenario: A repository drained goes up in no hello

- **WHEN** the runner keeps `example/service` and drains `example/app`, which `home` names, and its report's launch `home/l-1` names `example/app` as the repository it was answered in
- **THEN** `home`'s hello names no repository, and its report's `l-1` holds no repository

### Requirement: What goes up is version 3's copy, checked before it goes

Each place's status, sluice entry, preview's answer and listing, and host part SHALL go up cut to version 3's copy by walking its schema, and checked against it. A copy that fails outside a place's ends SHALL go up `null`, or the host part `missing`, with one line naming the launch or the preview and the schema's path, never a value. Every message SHALL be checked against `from-runner.schema.json` before it is sent (core.11, core.27, core.42; record 0037 as amended on 2026-11-09; record 0052).

#### Scenario: Flume's places cut

- **WHEN** each place of Flume 0.21.0's and Flume 0.4.1's fixtures is carried in a report
- **THEN** its copy passes `place-status.schema.json` of version 3 and holds no `last_words`, `screen`, `printed`, task `text`, `readiness` or `repository`; each end of Flume 0.21.0's goes up in a form version 3 names, with its `key` and `delivery`; and each end of Flume 0.4.1's, which Flume printed with no `key`, is left out

#### Scenario: A value Flume adds

- **WHEN** a place's status names a stage's state Flume 0.21.0 does not have, in its `stages`
- **THEN** that launch's status goes up `null`, the report passes and is sent, and one line names the launch and the path, and not the value

#### Scenario: sluice's previews cut

- **WHEN** sluice 0.74.3's answer of a preview brought up and each row of its listing are cut as a preview's copy, and its answers of a release as a release's copy
- **THEN** each copy passes its definition of version 3, keeps the `state` or `released` sluice printed, and no preview's copy holds `checkout`, `names`, `ports`, `exit` or `output`

#### Scenario: A preview's copy that fails

- **WHEN** a preview's status, as the runner carries it, holds a `made` that is a number
- **THEN** that preview's `status` goes up `null` and its `copy` as before, the report passes and is sent, and one line names the preview and the path, and not the value, once while it fails

### Requirement: Every message the runner is sent is checked before it is read

Every message a connection receives SHALL be checked against `to-runner.schema.json` of version 3 before it is read. One that fails SHALL be answered with `message_refused` naming it, with words naming the schema's path, and nothing else done with it, as SHALL an `enrolled` or `enroll_refused`, which only `runner/enroll` takes. One whose id cannot be read SHALL close the connection, which is tried again on the backoff (core.42; record 0052).

#### Scenario: A pulled with a field unnamed

- **WHEN** the fake control plane sends a `pulled` whose launch holds a field version 3 does not name
- **THEN** the runner answers `message_refused` naming that message, and the pull gives none of its wants

#### Scenario: A message of version 1

- **WHEN** the fake control plane sends a `pulled` naming `protocol_version` 1
- **THEN** the runner answers `message_refused` naming that message, its words saying it fails version 3's check, and the pull gives none of its wants

#### Scenario: A message of version 2

- **WHEN** the fake control plane sends a `pulled` naming `protocol_version` 2
- **THEN** the runner answers `message_refused` naming that message, its words saying it fails version 3's check, and the pull gives none of its wants

#### Scenario: An enrollment's answer on the socket

- **WHEN** the fake control plane sends `enrolled` on `home`'s connection
- **THEN** the runner answers `message_refused` naming it, its words naming `runner/enroll`

### Requirement: A report's answer names taken each status that went up as its copy

The answer to a part's report SHALL name taken each launch whose status went up whole as its copy in the report it answers, every end of it in a form version 3 names. It SHALL name none whose status went up `null`, and none whose status went up with an end carried by its delivery alone or left out. An answer to an earlier report SHALL name none (core.11, core.27, core.42, core.54; records 0030 as amended on 2026-11-09, 0037 and 0052).

#### Scenario: A status that goes up as its copy

- **WHEN** a place's status passes version 3
- **THEN** its copy goes up with the time it was read and no `repository`, and the report's answer names that launch's status taken

#### Scenario: A status that goes up null

- **WHEN** a place's status names a stage's state Flume 0.21.0 does not have, in its `stages`
- **THEN** that launch's status goes up `null`, and the report's answer names no status of that launch taken

#### Scenario: A status with an end by its delivery alone

- **WHEN** a place's status holds an end whose `of` is `moved` beside one that passes
- **THEN** the status goes up with the first end as its delivery alone, and the report's answer names no status of that launch taken

#### Scenario: A drop held for the runner

- **WHEN** a place's status is Flume 0.21.0's fixture of a tidied place dropped, launched naming `scratch/planner` and `boomdrive`, its drop's end pending for `boomdrive`
- **THEN** its copy goes up with its drop's end, `over` `dropped`, and the delivery's key Flume gave it, and the report's answer names that launch's status taken

### Requirement: Each end of a place's status is copied on its own

Each end of a place's status SHALL be cut by the form version 3 names for its `of`, and checked; where there is none, or the check fails, cut to its delivery alone and checked; and where that fails too, left out. The rest of the status SHALL go up as its copy whatever its ends. A status with an end carried by its delivery alone or left out SHALL be said in one line while it lasts, naming the launch and each end's path, never a value (core.11, core.27, core.42; records 0037 and 0052).

#### Scenario: An end Flume adds

- **WHEN** a place's status holds an end whose `of` is `moved`, a stage's end whose `state` version 3 does not have, an end with no `key`, and a submission's end that passes
- **THEN** the first two go up as their delivery alone, the third is left out, the fourth goes up whole, the rest of the status goes up as its copy, the report passes and is sent, and one line names the launch and each of the three ends' paths, and no value

### Requirement: A control plane's confirm is given to the runner as any act

A `pulled` holding an act whose `what` is a confirm SHALL pass version 3's check and be given to the runner as an act, its id and its launch under the partition's prefix, its recipient and its delivery keys as the control plane gave them, in its order. The hosted implementation SHALL refuse nothing of a confirm and read nothing of it, the runner carrying it out as it carries crew's (core.2, core.26, core.56; records 0037 and 0052).

#### Scenario: A confirm carried

- **WHEN** `home`'s control plane offers launch `l-1` and asks act `c-1` on it, a confirm as `scratch/planner` of one delivery key
- **THEN** `home`'s part's pull gives act `home/c-1` on `home/l-1`, a confirm as `scratch/planner` of that key, and the runner sent no `message_refused`

#### Scenario: A confirm the runner refuses

- **WHEN** `home`'s control plane asks act `c-2`, a confirm of no key
- **THEN** `home`'s part's pull gives act `home/c-2` with no key, the runner sent no `message_refused`, and it is the runner's to refuse

### Requirement: A control plane's land is given to the runner as any act

A `pulled` holding an act whose `what` is `land` SHALL pass version 3's check and be given to the runner as an act, its id and its launch under the partition's prefix. The hosted implementation SHALL read nothing of a land, the runner carrying it out as it carries crew's (core.2, core.26, core.53; record 0037 as amended on 2026-11-09; record 0052).

#### Scenario: A land carried

- **WHEN** `home`'s control plane offers launch `l-1` and asks act `land-1` on it, a land
- **THEN** `home`'s part's pull gives act `home/land-1` on `home/l-1`, a land, and the runner sent no `message_refused`

### Requirement: A control plane's preview is given to the runner under the partition's prefix

A `pulled` holding a preview SHALL pass version 3's check and be given to the runner, its id under the partition's prefix and every other field as given, in the control plane's order. A preview of a repository the partition is not served SHALL be withheld, one line naming it while it is offered. The hosted implementation SHALL read nothing else of a preview (core.2, core.26, core.55; record 0037 as amended on 2026-11-09; record 0052).

#### Scenario: A preview carried

- **WHEN** `home`'s control plane asks a preview `preview-1` of `main` in `example/app` at a revision, then `preview-2`, its done
- **THEN** `home`'s part's pull gives the previews `home/preview-1` and `home/preview-2`, in that order, each field as given, and the runner sent no `message_refused`

#### Scenario: A preview of a revision that is not a commit

- **WHEN** `home`'s control plane asks a preview of `example/app` at the revision `R1`
- **THEN** `home`'s part's pull gives it as given, the runner sent no `message_refused`, and it is sluice's to answer

#### Scenario: A preview of a repository not served

- **WHEN** `home`'s control plane asks, at two pulls, a preview of `example/service`, which `home` is not served
- **THEN** neither pull gives it, and one line names the preview, `home` and `example/service`

### Requirement: No hello names a drive or a clone's folder

A partition's hello SHALL name no drive and no clone's folder, since version 3 carries neither and a path goes up only where the protocol's copy names it (core.27, core.28, core.42).

#### Scenario: No drive or folder goes up

- **WHEN** the runner's hello names the drive `family` with `example/app` at `/data/repositories/example/app`, and `home` names `example/app`
- **THEN** `home`'s hello names `example/app`, and neither `family` nor `/data/repositories/example/app` appears in it

## ADDED Requirements

### Requirement: A control plane's hold is given to the runner as any act

A `pulled` holding an act whose `what` is a hold SHALL pass version 3's check and be given to the runner as an act, its id and its launch under the partition's prefix, its words and its bound as the control plane gave them. The hosted implementation SHALL refuse nothing of a hold and read nothing of it, the runner carrying it out as it carries crew's (core.2, core.26, core.59; record 0052).

#### Scenario: A hold carried

- **WHEN** `home`'s control plane offers launch `l-1` and asks act `hold-1` on it, a hold under the words `the user sends it on the landings card, row 3` for 28800 seconds
- **THEN** `home`'s part's pull gives act `home/hold-1` on `home/l-1`, a hold under those words for 28800 seconds, and the runner sent no `message_refused`

#### Scenario: A hold the runner refuses

- **WHEN** `home`'s control plane asks act `hold-2` on `l-1`, a hold whose words are empty and whose bound is 0
- **THEN** `home`'s part's pull gives act `home/hold-2` with no words and a bound of 0, the runner sent no `message_refused`, and it is the runner's to refuse
