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

`boomdrive enroll <label>` SHALL read the token from standard input, make an Ed25519 key in memory, and send one `POST runner/enroll` under `url`, the token its bearer and its body an `enroll` of version 4. It SHALL keep the key only once `enrolled` answers, print `enrolled` or the refusal's words as given, and write no configuration and nothing in the data directory (core.31, core.39; records 0006, 0037, 0052 and 0053).

#### Scenario: A partition enrolled

- **WHEN** partition `home` is enrolled with a token, and the fake control plane takes the `enroll` and answers `enrolled` with host `h-1`
- **THEN** the request bore the token, its `enroll` passed version 4 and its signature checked over version 4's context, the key is in the store under `home`, and the enrollment names host `h-1`

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

Each upgrade of `runner/connect` SHALL ask for `boomdrive.v4` alone and carry `Authorization: BoomdriveHost` with the key, the time, a nonce and a signature over version 4's context, made afresh for each attempt. The runner SHALL keep nothing of enrollment but the key, which it proves under version 4 whichever version enrolled it. A key or a subprotocol refused SHALL be said once and tried again at the backoff's cap (core.22, core.23, core.31, core.35; records 0037, 0052 and 0053).

#### Scenario: A connection proved

- **WHEN** partition `home` dials the fake control plane twice
- **THEN** each upgrade asks for `boomdrive.v4` alone, its signature checks over its context of version 4 with `home`'s key, and the two nonces differ

#### Scenario: A key refused

- **WHEN** the fake control plane refuses `home`'s key three times
- **THEN** one line names `home`, the fake's words and `boomdrive enroll home`

#### Scenario: A control plane that speaks version 1 alone

- **WHEN** the fake control plane answers `home`'s upgrades without the subprotocol, three times
- **THEN** one line names `home` and `boomdrive.v4`, each later attempt waits for the backoff's cap, and no message is sent

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

Each partition's hello SHALL name only the repositories its table names, of those kept with their clones made, and only its own leases. No hello SHALL name a repository the runner drains, and no report the repository a launch was answered in, since version 4 carries neither. Each report SHALL carry only the entries under its prefix. A want naming a repository the partition does not name SHALL not be given to the runner, and one line SHALL name it (core.26; ledger 3.b; record 0053).

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

### Requirement: What goes up is version 4's copy, checked before it goes

Each place's status, sluice entry, preview's answer and listing, and host part SHALL go up cut to version 4's copy by walking its schema, and checked against it. A copy that fails outside a place's ends SHALL go up `null`, or the host part `missing`, with one line naming the launch or the preview and the schema's path, never a value. Every message SHALL be checked against `from-runner.schema.json` before it is sent (core.11, core.27, core.42; record 0037 as amended on 2026-11-09; record 0052).

#### Scenario: Flume's places cut

- **WHEN** each place of Flume 0.21.0's and Flume 0.4.1's fixtures is carried in a report
- **THEN** its copy passes `place-status.schema.json` of version 4 and holds no `last_words`, `screen`, `printed`, task `text`, `readiness` or `repository`; each end of Flume 0.21.0's goes up in a form version 4 names, with its `key` and `delivery`; and each end of Flume 0.4.1's, which Flume printed with no `key`, is left out

#### Scenario: A value Flume adds

- **WHEN** a place's status names a stage's state Flume 0.21.0 does not have, in its `stages`
- **THEN** that launch's status goes up `null`, the report passes and is sent, and one line names the launch and the path, and not the value

#### Scenario: sluice's previews cut

- **WHEN** sluice 0.74.3's answer of a preview brought up and each row of its listing are cut as a preview's copy, and its answers of a release as a release's copy
- **THEN** each copy passes its definition of version 4, keeps the `state` or `released` sluice printed, and no preview's copy holds `checkout`, `names`, `ports`, `exit` or `output`

#### Scenario: A preview's copy that fails

- **WHEN** a preview's status, as the runner carries it, holds a `made` that is a number
- **THEN** that preview's `status` goes up `null` and its `copy` as before, the report passes and is sent, and one line names the preview and the path, and not the value, once while it fails

### Requirement: Every message the runner is sent is checked before it is read

Every message a connection receives SHALL be checked against `to-runner.schema.json` of version 4 before it is read. One that fails SHALL be answered with `message_refused` naming it, with words naming the schema's path, and nothing else done with it, as SHALL an `enrolled` or `enroll_refused`, which only `runner/enroll` takes. One whose id cannot be read SHALL close the connection, which is tried again on the backoff (core.42; records 0052 and 0053).

#### Scenario: A pulled with a field unnamed

- **WHEN** the fake control plane sends a `pulled` whose launch holds a field version 4 does not name
- **THEN** the runner answers `message_refused` naming that message, and the pull gives none of its wants

#### Scenario: A message of version 1

- **WHEN** the fake control plane sends a `pulled` naming `protocol_version` 1
- **THEN** the runner answers `message_refused` naming that message, its words saying it fails version 4's check, and the pull gives none of its wants

#### Scenario: A message of version 2

- **WHEN** the fake control plane sends a `pulled` naming `protocol_version` 2
- **THEN** the runner answers `message_refused` naming that message, its words saying it fails version 4's check, and the pull gives none of its wants

#### Scenario: A message of version 3

- **WHEN** the fake control plane sends a `pulled` naming `protocol_version` 3
- **THEN** the runner answers `message_refused` naming that message, its words saying it fails version 4's check, and the pull gives none of its wants

#### Scenario: An enrollment's answer on the socket

- **WHEN** the fake control plane sends `enrolled` on `home`'s connection
- **THEN** the runner answers `message_refused` naming it, its words naming `runner/enroll`

### Requirement: A terminal the control plane opens is closed at once

Every `terminal_open` SHALL be answered with `terminal_closed`, its words saying the runner has no end-to-end exchange for a terminal yet and relays none in the clear, and no key made. Messages naming a terminal never opened SHALL be dropped, and a pull SHALL give no terminal (core.30; record 0004 as amended on 2026-11-15).

#### Scenario: A terminal opened

- **WHEN** the fake control plane sends `terminal_open` for a launch the runner holds
- **THEN** the runner answers `terminal_closed` for it, with words naming its end-to-end exchange and not naming Flume, and the pull gives no terminal

### Requirement: A report's answer names taken each status that went up as its copy

The answer to a part's report SHALL name taken each launch whose status went up whole as its copy in the report it answers, every end of it in a form version 4 names. It SHALL name none whose status went up `null`, and none whose status went up with an end carried by its delivery alone or left out. An answer to an earlier report SHALL name none (core.11, core.27, core.42, core.54; records 0030 as amended on 2026-11-09, 0037, 0052 and 0053).

#### Scenario: A status that goes up as its copy

- **WHEN** a place's status passes version 4
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

Each end of a place's status SHALL be cut by the form version 4 names for its `of`, and checked; where there is none, or the check fails, cut to its delivery alone and checked; and where that fails too, left out. The rest of the status SHALL go up as its copy whatever its ends. A status with an end carried by its delivery alone or left out SHALL be said in one line while it lasts, naming the launch and each end's path, never a value (core.11, core.27, core.42; records 0037 and 0052).

#### Scenario: An end Flume adds

- **WHEN** a place's status holds an end whose `of` is `moved`, a stage's end whose `state` version 4 does not have, an end with no `key`, and a submission's end that passes
- **THEN** the first two go up as their delivery alone, the third is left out, the fourth goes up whole, the rest of the status goes up as its copy, the report passes and is sent, and one line names the launch and each of the three ends' paths, and no value

### Requirement: A control plane's confirm is given to the runner as any act

A `pulled` holding an act whose `what` is a confirm SHALL pass version 4's check and be given to the runner as an act, its id and its launch under the partition's prefix, its recipient and its delivery keys as the control plane gave them, in its order. The hosted implementation SHALL refuse nothing of a confirm and read nothing of it, the runner carrying it out as it carries crew's (core.2, core.26, core.56; records 0037, 0052 and 0053).

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

A `pulled` holding an act whose `what` is `land` SHALL pass version 4's check and be given to the runner as an act, its id and its launch under the partition's prefix. The hosted implementation SHALL read nothing of a land, the runner carrying it out as it carries crew's (core.2, core.26, core.53; record 0037 as amended on 2026-11-09; records 0052 and 0053).

#### Scenario: A land carried

- **WHEN** `home`'s control plane offers launch `l-1` and asks act `land-1` on it, a land
- **THEN** `home`'s part's pull gives act `home/land-1` on `home/l-1`, a land, and the runner sent no `message_refused`

### Requirement: A control plane's preview is given to the runner under the partition's prefix

A `pulled` holding a preview SHALL pass version 4's check and be given to the runner, its id under the partition's prefix and every other field as given, in the control plane's order. A preview of a repository the partition is not served SHALL be withheld, one line naming it while it is offered. The hosted implementation SHALL read nothing else of a preview (core.2, core.26, core.55; record 0037 as amended on 2026-11-09; records 0052 and 0053).

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

### Requirement: No hello names a drive or a clone's folder

A partition's hello SHALL name no drive and no clone's folder, since version 4 carries neither and a path goes up only where the protocol's copy names it (core.27, core.28, core.42; record 0053).

#### Scenario: No drive or folder goes up

- **WHEN** the runner's hello names the drive `family` with `example/app` at `/data/repositories/example/app`, and `home` names `example/app`
- **THEN** `home`'s hello names `example/app`, and neither `family` nor `/data/repositories/example/app` appears in it

### Requirement: A control plane's hold is given to the runner as any act

A `pulled` holding an act whose `what` is a hold SHALL pass version 4's check and be given to the runner as an act, its id and its launch under the partition's prefix, its words and its bound as the control plane gave them. The hosted implementation SHALL refuse nothing of a hold and read nothing of it, the runner carrying it out as it carries crew's (core.2, core.26, core.59; records 0052 and 0053).

#### Scenario: A hold carried

- **WHEN** `home`'s control plane offers launch `l-1` and asks act `hold-1` on it, a hold under the words `the user sends it on the landings card, row 3` for 28800 seconds
- **THEN** `home`'s part's pull gives act `home/hold-1` on `home/l-1`, a hold under those words for 28800 seconds, and the runner sent no `message_refused`

#### Scenario: A hold the runner refuses

- **WHEN** `home`'s control plane asks act `hold-2` on `l-1`, a hold whose words are empty and whose bound is 0
- **THEN** `home`'s part's pull gives act `home/hold-2` with no words and a bound of 0, the runner sent no `message_refused`, and it is the runner's to refuse

### Requirement: The observer table names a loopback origin and the partitions it observes

The hosted implementation SHALL read `[observer]` as it reads the hosted table, but that its `url` SHALL be `http://` or `https://` to a loopback address alone, and that each partition's `label` names the part whose work it observes. A refusal SHALL name `[observer]`, the file and the key, its value in no output (core.19, core.26; record 0048).

#### Scenario: The observer read

- **WHEN** `[observer]` says `url = "http://localhost:8787"` and the partition `home`, with `example/app`
- **THEN** it is read, observing `home` with `example/app`

#### Scenario: An observer off the host

- **WHEN** `[observer]` says `url = "https://control.example"`
- **THEN** it is refused, naming `[observer]` and `url` and saying an observer is reached at a loopback address alone

#### Scenario: A token in the observer table

- **WHEN** `[observer]` says `token = "..."`
- **THEN** it is refused, naming `token` and saying the host's key is made by `boomdrive enroll --observer`, and the value appears in no output

### Requirement: An observer's partition is enrolled once by boomdrive enroll --observer

`boomdrive enroll --observer <label>` SHALL enroll the partition `<label>` of `[observer]` as `boomdrive enroll` enrolls a hosted partition, at the observer's `runner/enroll`, whatever `[control_plane]`'s `type`. It SHALL keep the key under `observer.<label>`, which no hosted partition's label can be, and refuse with exit 2 where the file names no `[observer]` or no partition `<label>` in it (core.31, core.32; records 0006, 0018 and 0048; record 0053).

#### Scenario: An observer enrolled beside crew

- **WHEN** `boomdrive enroll --observer home` runs with `type = "crew"`, and the fake observer takes the `enroll` and answers `enrolled` with host `h-1`
- **THEN** the request went to the observer's `runner/enroll`, its `enroll` passed version 4, the key is in the store under `observer.home`, and the enrollment names host `h-1`

#### Scenario: Kept apart from a partition of the same label

- **WHEN** an observer's partition `home` is enrolled into the `systemd-creds` store, its program a stub reporting version 256
- **THEN** the stub is run as `encrypt --user --name=boomdrive.observer.home.<the url's host>`

#### Scenario: No observer named

- **WHEN** `boomdrive enroll --observer home` runs with no `[observer]`
- **THEN** it is refused naming `[observer]`, and exits 2

### Requirement: An observer is told the observing hello and the places the part follows, with no lease

An observer's partition SHALL keep one outgoing connection, proved as a hosted partition's is, whose first message is its whole observing hello, naming only the repositories its partition names of those kept. After each control pass of the part it observes, it SHALL be sent an observed report of that part's launches, each id as the runner holds it, its answer, and its status and entry as version 3's copies, and no lease, act, command, inbox or preview (core.26, core.27; record 0048).

#### Scenario: The observing hello

- **WHEN** an observer of `home`, which names `example/app`, is handed a hello keeping `example/app` and `example/service`, with a capacity and a lease
- **THEN** its first message is a `hello` with `observing` true, naming `example/app` alone, and no capacity and no lease

#### Scenario: The places observed

- **WHEN** an observer of `home` is handed a share holding the launch `home/1` with its place's status, and an act `home/2`
- **THEN** it sends a `report` with `observing` true, holding `home/1` with its answer and its status's copy and no lease, and no act

### Requirement: What an observer sends of work is refused and said

The runner SHALL take no work from an observer. A `pulled` holding no want and no report SHALL be its answer to the hello. A `pulled` holding a want or a report, and every `terminal_open`, `terminal_input`, `terminal_resume` and `terminal_close`, SHALL be answered with `message_refused`, its words saying the connection observes, and said in one line naming its kind and id, and nothing else SHALL be done with it (core.1, core.19; record 0048).

#### Scenario: An offer from an observer

- **WHEN** the fake observer answers `home`'s observing hello with a `pulled` holding an offer
- **THEN** the runner answers `message_refused` naming it, its words saying the connection observes, and one line names the `pulled` and its id

#### Scenario: A terminal from an observer

- **WHEN** the fake observer sends `terminal_open` on `home`'s connection
- **THEN** the runner answers `message_refused` naming it, sends no `terminal_closed`, and makes no key

### Requirement: An observer's answers change nothing and wake nothing

A `reported` from an observer SHALL answer its observed report, so it is not sent again, and its let-go and refused ids SHALL change nothing. No message from an observer SHALL wake the runner, and none SHALL name a status taken, so no end is confirmed on an observer's answer (core.10, core.54; record 0048).

#### Scenario: A report's answer that lets go

- **WHEN** the fake observer answers an observed report with a `reported` letting go `home/1`
- **THEN** the observed report is not sent again, and the next observed report still holds `home/1`

#### Scenario: A pulled wakes nothing

- **WHEN** the fake observer sends `home`'s observer three `pulled`, each holding no want
- **THEN** the wake the observer was opened beside is never called

### Requirement: Nothing waits on an observer

Handing an observer its hello and report SHALL return at once, whether its connection is live, being attempted, or waiting on its backoff, and SHALL wait for no answer. Its newest report SHALL be sent at once on a live connection, and after the hello of its next one, replacing any it was not answered for. An observer's connection lost SHALL be tried again on the backoff a hosted partition's is (core.21, core.22; record 0048).

#### Scenario: An observer down

- **WHEN** the fake observer is down, and an observer of `home` is handed a hello and a report three times
- **THEN** each handing returns within 100 milliseconds

#### Scenario: Back again

- **WHEN** the fake observer is down while an observer of `home` is handed two reports, and then comes back
- **THEN** after the observing hello, the second report alone is sent

### Requirement: An observer's reach is said once each way

An observer's partition SHALL say one line when its connection is lost or cannot be made, `<label>: the observer is unreachable: <why>`, and one when it takes its first `pulled`, `<label>: the observer is reached`, and a refusal of its key in one line naming `boomdrive enroll --observer <label>` (core.23; record 0048).

#### Scenario: An observer's outage

- **WHEN** an observer of `home` is reached, then the fake observer stops answering for five attempts, then answers
- **THEN** `home` says the observer is reached, once that it is unreachable, and once that it is reached again

### Requirement: A served repository's token is asked by one request of version 4

The hosted implementation SHALL ask for a repository's token by one `POST runner/forge-token` under `url`, its body a `forge_token` of version 4 naming the repository, its `Authorization` a `BoomdriveHost` statement signed over version 4's `forge token` context, made afresh for each ask with a partition's key read from the host's store. It SHALL open no socket and no part, keep nothing of an ask, and stop its asks at twenty seconds (core.3, core.31, core.33, core.35; record 0053).

#### Scenario: A token given

- **WHEN** `home`, served `example/app`, asks for `example/app`'s token, and the fake control plane answers `forge_token_given` with a token and `expires_at` `2026-11-18T13:00:00Z`
- **THEN** the ask's body passed version 4 and named `example/app`, its signature checks over the `forge token` context with `home`'s key and `example/app`, and the answer is that token, ending at that time

#### Scenario: Each ask signed afresh

- **WHEN** `home` asks for `example/app`'s token twice
- **THEN** the two statements' nonces differ, and the fake control plane took no upgrade of `runner/connect`

#### Scenario: A control plane that never answers

- **WHEN** the fake control plane takes the ask and never answers, and the asks stop at 500 ms
- **THEN** the ask is refused within 5 seconds, its words naming `runner/forge-token`

### Requirement: Each partition served the repository is asked in the table's order

The hosted implementation SHALL ask under each partition whose `repositories` hold the repository, in the table's order, until one gives a token. Where none does, the answer SHALL be the App's absence where each said its installation does not cover the repository, and otherwise a refusal naming each partition with its words as given. A repository no partition is served SHALL be refused, naming it, with no request made (core.26, core.33; record 0053).

#### Scenario: The second partition gives it

- **WHEN** `home` and `work` are each served `example/app`, and the fake control plane answers `home`'s ask `forge_token_refused` with `not_installed` true and `work`'s `forge_token_given`
- **THEN** `home` is asked first and `work` second, each under its own key, and the answer is `work`'s token

#### Scenario: The App installed under no partition

- **WHEN** `home` and `work` are each served `example/app`, and the fake control plane answers each `forge_token_refused` with `not_installed` true
- **THEN** the answer is the App's absence, its words naming `home` and `work` with the fake's words for each

#### Scenario: A refusal carried as given

- **WHEN** the fake control plane answers `home`'s ask `forge_token_refused` with `not_installed` false and the words `example/app is registered to no drive this host serves`
- **THEN** the ask is refused, its words naming `home` and holding the fake's words as given

#### Scenario: A repository no partition is served

- **WHEN** a token is asked for `example/service`, which no partition names
- **THEN** it is refused naming `example/service`, and the fake control plane took no request

### Requirement: An ask's answer is checked against version 4 before it is read

An answer to an ask SHALL be read only once it passes `to-runner.schema.json` of version 4, whatever its status, and only where it is a `forge_token_given` or `forge_token_refused` naming the ask and its repository. An answer that is no such message, or that never comes, SHALL refuse that partition's ask, naming `runner/forge-token` and the status, and no value of the answer (core.11, core.42; record 0053).

#### Scenario: A control plane that speaks version 3

- **WHEN** the fake control plane answers `home`'s ask with 404 and no body
- **THEN** the ask is refused, its words naming `runner/forge-token` and 404

#### Scenario: A token given with a field unnamed

- **WHEN** the fake control plane answers `forge_token_given` with a field `scope` added
- **THEN** the ask is refused, its words naming `runner/forge-token` and the schema's path, and the token is in no words

#### Scenario: A token for another repository

- **WHEN** the fake control plane answers the ask for `example/app` with a `forge_token_given` naming `example/other`
- **THEN** the ask is refused, its words naming `runner/forge-token` and `example/other`, and the token is in no words

#### Scenario: The ask refused as a message

- **WHEN** the fake control plane answers 400 with `message_refused` naming the ask, with words
- **THEN** the ask is refused, its words naming `runner/forge-token` and holding the fake's words as given

### Requirement: A host with no partition's key must be fixed

Where the host has no credential store, or no partition served the repository has its key in the store, the ask SHALL be refused as the host's to fix, naming `boomdrive enroll <label>` for each such partition, with no request made (core.32, core.39; record 0053).

#### Scenario: No key of the partition

- **WHEN** `home` is served `example/app`, and the store holds no key of `home`
- **THEN** the ask is refused as the host's to fix, naming `boomdrive enroll home`, and the fake control plane took no request

### Requirement: A forge token's answer on a partition's socket is refused

A `forge_token_given` or `forge_token_refused` a partition's connection receives SHALL be answered with `message_refused` naming it, with words naming `runner/forge-token`, which alone takes a forge token's ask and answers. Nothing else SHALL be done with it: no token of it is kept, said or given to the runner (core.4, core.42; record 0053).

#### Scenario: A forge token's answer on the socket

- **WHEN** the fake control plane sends `forge_token_given` on `home`'s connection
- **THEN** the runner answers `message_refused` naming it, its words naming `runner/forge-token`, and nothing of it is kept, said or given to the runner
