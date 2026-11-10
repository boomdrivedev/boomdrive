# hosted-control-plane Specification

## Purpose
The hosted control plane's implementation of the control-plane interface, in `crates/boomdrive-hosted`, the one crate that names the runner's protocol with it. It enrolls each partition once, with a key made on the host and kept only in its credential store. It keeps one outgoing WebSocket for each partition, proved at its upgrade, kept by pings and tried again on a bounded backoff. It carries each partition's own wants down and its own part of the report up, of each tool's answer only version 1's copy, and checks every message both ways (core.2, core.3, core.21 to core.23, core.26, core.27, core.31, core.32, core.42; records 0004, 0006, 0008 and 0018; boomdrive-cloud records 0047, 0049 and 0056).

## Requirements

### Requirement: The hosted table names the control plane's address and its partitions

The hosted implementation SHALL open from `[control_plane.hosted]`: `url`, required, `https://` or `http://` to a loopback address alone, with no user, query or fragment; and `partitions`, at least one, each a `label` of lowercase letters, digits and `-` starting with a letter, given once, and `repositories`, each `owner/name`. Any other key SHALL be refused, naming it and the file, its value in no output (core.4, core.26).

#### Scenario: The table read

- **WHEN** the table says `url = "https://control.example"` and partitions `home`, with `example/app`, and `work`, with `example/service` and `example/app`
- **THEN** the hosted implementation opens serving `home` and `work`, each with its repositories

#### Scenario: A control plane in the clear

- **WHEN** the table says `url = "http://control.example"`
- **THEN** it is refused, naming `url` and saying it is `https://`, or `http://` to a loopback address

#### Scenario: A token in the table

- **WHEN** the table says `token = "..."`, or `token_file = "/h/token"`
- **THEN** it is refused, naming the key as one the hosted implementation does not read and saying the host's key is made by `boomdrive enroll`, and the value appears in no output

#### Scenario: A label twice

- **WHEN** two partitions say `label = "home"`
- **THEN** it is refused, naming `home`

### Requirement: A partition is enrolled once by boomdrive enroll

`boomdrive enroll <label>` SHALL read the token from standard input, make an Ed25519 key in memory, and send one `POST runner/enroll` under `url`, the token its bearer and its body an `enroll` of version 2. It SHALL keep the key only once `enrolled` answers, print `enrolled` or the refusal's words as given, and write no configuration and nothing in the data directory (core.31, core.39; records 0006 and 0037).

#### Scenario: A partition enrolled

- **WHEN** partition `home` is enrolled with a token, and the fake control plane takes the `enroll` and answers `enrolled` with host `h-1`
- **THEN** the request bore the token, its `enroll` passed version 2 and its signature checked over version 2's context, the key is in the store under `home`, and the enrollment names host `h-1`

#### Scenario: A lapsed token

- **WHEN** the fake control plane answers `enroll_refused` with the words `the token has lapsed`
- **THEN** the enrollment is refused with `the token has lapsed`, as the control plane's refusal and not the host's to fix, and the store is as it was

#### Scenario: The answer as JSON

- **WHEN** `boomdrive enroll --json home` is given a token and `url` names a loopback port with nothing on it
- **THEN** standard output is one object, `{"refused":{"refusal":{"reason":…}}}`, its reason naming the control plane as unreachable, and it exits 1

#### Scenario: The token never shown

- **WHEN** `boomdrive enroll home` is given the token `secret-enrollment-token`, and `url` names a loopback port with nothing on it
- **THEN** neither standard output nor standard error holds `secret-enrollment-token`

### Requirement: boomdrive enroll refuses what it cannot enroll

`boomdrive enroll` SHALL be refused, exiting 2 before any request, where the configuration names no control plane whose implementation enrolls, where its table has no partition of the label, where the token is empty, and where the host has no credential store. A control plane that cannot be reached SHALL be refused with exit 1, and no key written.

#### Scenario: crew's control plane

- **WHEN** `boomdrive enroll home` runs with `type = "crew"`
- **THEN** it is refused naming crew's control plane as one that enrolls nothing, and exits 2

#### Scenario: An unknown label

- **WHEN** `boomdrive enroll garden` runs with a table naming `home` and `work`
- **THEN** it is refused naming `garden`, and exits 2

#### Scenario: Nothing listening

- **WHEN** `boomdrive enroll home` is given a token and `url` names a loopback port with nothing on it
- **THEN** it is refused naming the control plane as unreachable, exits 1, and writes no key

### Requirement: The host's key is kept only in its credential store

Each partition's key SHALL be kept only in the macOS keychain, or as a user-scoped credential encrypted by `systemd-creds` 256 or later in the user's credential store, read at each connection. It SHALL never be in a file in the clear, an argument, a log or the data directory, and nothing SHALL fall back to a file. A host with neither store SHALL be refused at `enroll`, naming what it lacks (core.32).

#### Scenario: Kept by systemd-creds

- **WHEN** partition `home` is enrolled into the `systemd-creds` store, its program a stub reporting version 256
- **THEN** the stub is run as `encrypt --user --name=boomdrive.home.<the url's host>`, the key's bytes reach it only on standard input, and no argument holds them

#### Scenario: An old systemd

- **WHEN** the `systemd-creds` store is asked for, its program a stub whose `--version` reports 255
- **THEN** it is refused naming systemd 256 as the least it takes, as the host's to fix, and an enrollment over it makes no request

### Requirement: Every connection proves the host's key

Each upgrade of `runner/connect` SHALL ask for `boomdrive.v2` alone and carry `Authorization: BoomdriveHost` with the key, the time, a nonce and a signature over version 2's context, made afresh for each attempt. The runner SHALL keep nothing of enrollment but the key, which it proves under version 2 whichever version enrolled it. A key or a subprotocol refused SHALL be said once and tried again at the backoff's cap (core.22, core.23, core.31, core.35; record 0037).

#### Scenario: A connection proved

- **WHEN** partition `home` dials the fake control plane twice
- **THEN** each upgrade asks for `boomdrive.v2` alone, its signature checks over its context of version 2 with `home`'s key, and the two nonces differ

#### Scenario: A key refused

- **WHEN** the fake control plane refuses `home`'s key three times
- **THEN** one line names `home`, the fake's words and `boomdrive enroll home`

#### Scenario: A control plane that speaks version 1 alone

- **WHEN** the fake control plane answers `home`'s upgrades without the subprotocol, three times
- **THEN** one line names `home` and `boomdrive.v2`, each later attempt waits for the backoff's cap, and no message is sent

### Requirement: Each partition keeps one outgoing connection, its whole hello first

Each partition SHALL keep one WebSocket it opens, sending its whole hello first at every connection, and its hello again whenever it changes. It SHALL keep the socket alive with WebSocket pings, never a message of the protocol's, and count it lost when a pong has not come by the next ping (core.3, core.21; boomdrive-cloud core.50).

#### Scenario: The hello first

- **WHEN** partition `home` connects, is dropped by the fake control plane, and connects again
- **THEN** the first message of each connection is `home`'s whole `hello`

#### Scenario: No heartbeat message

- **WHEN** a connection stands idle for two minutes of the test's clock
- **THEN** the fake control plane takes pings and no message

### Requirement: A connection lost is tried again on a bounded backoff with jitter

A connection lost, refused or not made SHALL be tried again after a wait drawn at random up to a cap that starts at one second, doubles with each failed attempt, and stops at `poll_interval`. The cap SHALL go back to one second once a connection takes its first `pulled` (core.22).

#### Scenario: Never back to back, never past the poll interval

- **WHEN** the fake control plane refuses ten attempts in a row, with `poll_interval` one minute on the test's clock
- **THEN** no two attempts are at the same instant, and no wait is longer than a minute

### Requirement: A partition's reach is said once each way

A partition SHALL say one line when its connection is lost or cannot be made, `<label>: the control plane is unreachable: <why>`, and one when a connection takes its first `pulled`, `<label>: the control plane is reached`, and nothing for the attempts between but a refusal of its key or subprotocol, said once (core.23).

#### Scenario: An outage

- **WHEN** partition `home` is reached, then the fake control plane stops answering for five attempts, then answers
- **THEN** `home` says it is reached, once that it is unreachable, and once that it is reached again

### Requirement: A pull gives each partition's wants under its own ids

Each partition SHALL be a part of its own. A part's pull SHALL give the wants of its connection once it has taken a `pulled`, waiting up to 10 seconds for that connection alone, each want's id as `<label>/<id>`. A part not read SHALL be unread, its words not repeating its label, while every other part pulls on its own. A `pulled` that comes SHALL wake the part whose connection took it, and no other (record 0026; core.26).

#### Scenario: Ids under their partition

- **WHEN** `home`'s control plane offers launch `l-1` after `l-0`, and `work`'s offers launch `l-1`
- **THEN** `home`'s part gives `home/l-1`, after `home/l-0`, and `work`'s part gives `work/l-1`

#### Scenario: One partition unreachable

- **WHEN** `work`'s control plane cannot be reached and `home`'s offers a launch
- **THEN** `home`'s part gives its offer without waiting on `work`'s connection, and `work`'s part is unread

#### Scenario: Woken

- **WHEN** `home`'s control plane sends a `pulled` between passes
- **THEN** the wake the opener was handed for `home` is called, and the one handed for `work` is not

### Requirement: A partition's connection carries only its own repositories and work

Each partition's hello SHALL name only the repositories its table names, of those kept with their clones made, and only its own leases. No hello SHALL name a repository the runner drains, and no report the repository a launch was answered in, since version 2 carries neither. Each report SHALL carry only the entries under its prefix. A want naming a repository the partition does not name SHALL not be given to the runner, and one line SHALL name it (core.26; ledger 3.b).

#### Scenario: Each hello its own

- **WHEN** the runner keeps `example/app` and `example/service`, `home` names `example/app`, and the report holds `home/l-1` and `work/l-2`
- **THEN** `home`'s hello names `example/app` and lease `l-1` alone

#### Scenario: Another partition's repository

- **WHEN** `home`'s control plane offers a launch on `example/service`, which `home` does not name
- **THEN** the pull does not give it, and one line names the offer, `home` and `example/service`

#### Scenario: A repository drained goes up in no hello

- **WHEN** the runner keeps `example/service` and drains `example/app`, which `home` names, and its report's launch `home/l-1` names `example/app` as the repository it was answered in
- **THEN** `home`'s hello names no repository, and its report's `l-1` holds no repository

### Requirement: A report goes to each partition, answered by its id

A part's report SHALL send its connection the part's share as `report` under a new message id, and wait up to 10 seconds for its `reported`, waiting on no other part's connection. It SHALL be unsent where its connection is not live or does not answer. After a reconnect, a part SHALL send its last `report` no `reported` answered again under the same id, after its hello, unless a newer one was sent (core.21; boomdrive-cloud core.38).

#### Scenario: A lease refused

- **WHEN** `home`'s control plane answers a report refusing `l-1`
- **THEN** `home`'s part's report answer refuses `home/l-1` with the fake's words

#### Scenario: A report resent

- **WHEN** `home`'s connection drops after a `report` and before its `reported`, then connects again
- **THEN** after the `hello`, the same `report` is sent again under the same message id

#### Scenario: A report unanswered by another partition

- **WHEN** `work`'s control plane takes a `report` and never answers it, and `home`'s answers at once
- **THEN** `home`'s part's report is answered at once, and `work`'s part's report is unsent after 10 seconds

### Requirement: What goes up is version 2's copy, checked before it goes

Each place's status, sluice entry, preview's answer and listing, and host part SHALL go up cut to version 2's copy by walking its schema, and checked against it. A copy that fails outside a place's ends SHALL go up `null`, or the host part `missing`, with one line naming the launch or the preview and the schema's path, never a value. Every message SHALL be checked against `from-runner.schema.json` before it is sent (core.11, core.27, core.42; record 0037 as amended on 2026-11-09).

#### Scenario: Flume's places cut

- **WHEN** each place of Flume 0.21.0's and Flume 0.4.1's fixtures is carried in a report
- **THEN** its copy passes `place-status.schema.json` of version 2 and holds no `last_words`, `screen`, `printed`, task `text`, `readiness` or `repository`; each end of Flume 0.21.0's goes up in a form version 2 names, with its `key` and `delivery`; and each end of Flume 0.4.1's, which Flume printed with no `key`, is left out

#### Scenario: A value Flume adds

- **WHEN** a place's status names a stage's state Flume 0.21.0 does not have, in its `stages`
- **THEN** that launch's status goes up `null`, the report passes and is sent, and one line names the launch and the path, and not the value

#### Scenario: sluice's previews cut

- **WHEN** sluice 0.74.3's answer of a preview brought up and each row of its listing are cut as a preview's copy, and its answers of a release as a release's copy
- **THEN** each copy passes its definition of version 2, keeps the `state` or `released` sluice printed, and no preview's copy holds `checkout`, `names`, `ports`, `exit` or `output`

#### Scenario: A preview's copy that fails

- **WHEN** a preview's status, as the runner carries it, holds a `made` that is a number
- **THEN** that preview's `status` goes up `null` and its `copy` as before, the report passes and is sent, and one line names the preview and the path, and not the value, once while it fails

### Requirement: Every message the runner is sent is checked before it is read

Every message a connection receives SHALL be checked against `to-runner.schema.json` of version 2 before it is read. One that fails SHALL be answered with `message_refused` naming it, with words naming the schema's path, and nothing else done with it, as SHALL an `enrolled` or `enroll_refused`, which only `runner/enroll` takes. One whose id cannot be read SHALL close the connection, which is tried again on the backoff (core.42).

#### Scenario: A pulled with a field unnamed

- **WHEN** the fake control plane sends a `pulled` whose launch holds a field version 2 does not name
- **THEN** the runner answers `message_refused` naming that message, and the pull gives none of its wants

#### Scenario: A message of version 1

- **WHEN** the fake control plane sends a `pulled` naming `protocol_version` 1
- **THEN** the runner answers `message_refused` naming that message, and the pull gives none of its wants

#### Scenario: An enrollment's answer on the socket

- **WHEN** the fake control plane sends `enrolled` on `home`'s connection
- **THEN** the runner answers `message_refused` naming it, its words naming `runner/enroll`

### Requirement: A terminal the control plane opens is closed at once

Every `terminal_open` SHALL be answered with `terminal_closed`, its words saying this host's Flume relays no terminal, and no key made. Messages naming a terminal never opened SHALL be dropped, and a pull SHALL give no terminal (core.30; record 0004).

#### Scenario: A terminal opened

- **WHEN** the fake control plane sends `terminal_open` for a launch the runner holds
- **THEN** the runner answers `terminal_closed` for it, and the pull gives no terminal

### Requirement: A report's answer names taken each status that went up as its copy

The answer to a part's report SHALL name taken each launch whose status went up whole as its copy in the report it answers, every end of it in a form version 2 names. It SHALL name none whose status went up `null`, and none whose status went up with an end carried by its delivery alone or left out. An answer to an earlier report SHALL name none (core.11, core.27, core.42, core.54; records 0030 as amended on 2026-11-09 and 0037).

#### Scenario: A status that goes up as its copy

- **WHEN** a place's status passes version 2
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

Each end of a place's status SHALL be cut by the form version 2 names for its `of`, and checked; where there is none, or the check fails, cut to its delivery alone and checked; and where that fails too, left out. The rest of the status SHALL go up as its copy whatever its ends. A status with an end carried by its delivery alone or left out SHALL be said in one line while it lasts, naming the launch and each end's path, never a value (core.11, core.27, core.42; record 0037).

#### Scenario: An end Flume adds

- **WHEN** a place's status holds an end whose `of` is `moved`, a stage's end whose `state` version 2 does not have, an end with no `key`, and a submission's end that passes
- **THEN** the first two go up as their delivery alone, the third is left out, the fourth goes up whole, the rest of the status goes up as its copy, the report passes and is sent, and one line names the launch and each of the three ends' paths, and no value

### Requirement: A control plane's confirm is given to the runner as any act

A `pulled` holding an act whose `what` is a confirm SHALL pass version 2's check and be given to the runner as an act, its id and its launch under the partition's prefix, its recipient and its delivery keys as the control plane gave them, in its order. The hosted implementation SHALL refuse nothing of a confirm and read nothing of it, the runner carrying it out as it carries crew's (core.2, core.26, core.56; record 0037).

#### Scenario: A confirm carried

- **WHEN** `home`'s control plane offers launch `l-1` and asks act `c-1` on it, a confirm as `scratch/planner` of one delivery key
- **THEN** `home`'s part's pull gives act `home/c-1` on `home/l-1`, a confirm as `scratch/planner` of that key, and the runner sent no `message_refused`

#### Scenario: A confirm the runner refuses

- **WHEN** `home`'s control plane asks act `c-2`, a confirm of no key
- **THEN** `home`'s part's pull gives act `home/c-2` with no key, the runner sent no `message_refused`, and it is the runner's to refuse

### Requirement: What the runner answered of each preview goes up

A part's report SHALL carry each of its previews under its id without the partition's prefix, as the runner holds it, but its text, which goes up as its `copy`, cut as a release's copy where the preview is done and as a preview's where it is not, and its status, which goes up as `{copy, at, listed}`. No preview SHALL change which statuses the report's answer names taken (core.27, core.54, core.55; record 0037 as amended on 2026-11-09).

#### Scenario: Previews go up as their copies

- **WHEN** `home`'s part's report holds a preview brought up, with sluice's answer and its listing's row as the runner carries them, its done, answered with sluice's release, and a preview refused with sluice's words
- **THEN** the report passes and is sent; each preview goes up under its id without `home/`; the first's `copy` and `status.copy` hold sluice's `name`, `branch`, `revision`, `state`, `made` and `addresses`; the done's `copy` holds `preview` and `released`; and the refused one's `copy` and `status` are `null`

### Requirement: A control plane's land is given to the runner as any act

A `pulled` holding an act whose `what` is `land` SHALL pass version 2's check and be given to the runner as an act, its id and its launch under the partition's prefix. The hosted implementation SHALL read nothing of a land, the runner carrying it out as it carries crew's (core.2, core.26, core.53; record 0037 as amended on 2026-11-09).

#### Scenario: A land carried

- **WHEN** `home`'s control plane offers launch `l-1` and asks act `land-1` on it, a land
- **THEN** `home`'s part's pull gives act `home/land-1` on `home/l-1`, a land, and the runner sent no `message_refused`

### Requirement: A control plane's preview is given to the runner under the partition's prefix

A `pulled` holding a preview SHALL pass version 2's check and be given to the runner, its id under the partition's prefix and every other field as given, in the control plane's order. A preview of a repository the partition is not served SHALL be withheld, one line naming it while it is offered. The hosted implementation SHALL read nothing else of a preview (core.2, core.26, core.55; record 0037 as amended on 2026-11-09).

#### Scenario: A preview carried

- **WHEN** `home`'s control plane asks a preview `preview-1` of `main` in `example/app` at a revision, then `preview-2`, its done
- **THEN** `home`'s part's pull gives the previews `home/preview-1` and `home/preview-2`, in that order, each field as given, and the runner sent no `message_refused`

#### Scenario: A preview of a revision that is not a commit

- **WHEN** `home`'s control plane asks a preview of `example/app` at the revision `R1`
- **THEN** `home`'s part's pull gives it as given, the runner sent no `message_refused`, and it is sluice's to answer

#### Scenario: A preview of a repository not served

- **WHEN** `home`'s control plane asks, at two pulls, a preview of `example/service`, which `home` is not served
- **THEN** neither pull gives it, and one line names the preview, `home` and `example/service`

### Requirement: A report handed back holds its previews

A report the control plane hands back in a `pulled` SHALL give the runner each preview it holds, under the partition's prefix, with its lease, repository, revision, done, outcome, words and time, its copy's JSON as its text where it has one, and no status, so a done after the runner lost its report file puts away the preview by the name sluice gave it (core.10, core.55; record 0037 as amended on 2026-11-09).

#### Scenario: A report taken back holds its previews

- **WHEN** `home`'s control plane holds a report whose preview was brought up, and hands it back in its `pulled`
- **THEN** the part's pull gives a report holding that preview under `home/`, with its lease, repository, revision, done, outcome, words and time as held, its text a JSON object whose `name` is its copy's, and no status
