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

`boomdrive enroll --control-plane <url> --drive <drive> --repository <owner/name>…` SHALL enroll the partition `<drive>` at `<url>`. It SHALL make an Ed25519 key in memory and send one `POST runner/enroll`, with the token as its bearer and an `enroll` of version 6 as its body. It SHALL keep the key only once `enrolled` answers, and print `enrolled` or the refusal's words. It SHALL write nothing in the data directory (core.31, core.39, records 0006 and 0053).

#### Scenario: A partition enrolled

- **WHEN** enroll runs with `--drive home --repository example/app` and a token, where the configuration names the partition `home` at the fake's url. The fake control plane answers `enrolled` with host `h-1`.
- **THEN** the request bore the token, and its `enroll` passed version 6. Its signature checked over version 6's context. The key is in the store under `home`, and the enrollment names host `h-1`.

#### Scenario: A lapsed token

- **WHEN** the fake control plane answers `enroll_refused` with the words `the token has lapsed`
- **THEN** enroll refuses with `the token has lapsed`, as the control plane's refusal and not the host's to fix. The store and the runner's file are as they were.

#### Scenario: The answer as JSON

- **WHEN** `boomdrive enroll --json` runs with a token, and `--control-plane` names a loopback port with nothing on it
- **THEN** standard output is one object, `{"refused":{"refusal":{"reason":…}}}`. Its reason names the control plane as unreachable, and it exits 1.

#### Scenario: The token never shown

- **WHEN** `boomdrive enroll` gets the token `secret-enrollment-token`, and `--control-plane` names a loopback port with nothing on it
- **THEN** neither standard output nor standard error holds `secret-enrollment-token`

### Requirement: The host's key is kept only in its credential store

Each partition's key SHALL be kept only in the macOS keychain, or as a user-scoped credential encrypted by `systemd-creds` 256 or later in the user's credential store, read at each connection. It SHALL never be in a file in the clear, an argument, a log or the data directory, and nothing SHALL fall back to a file. A host with neither store SHALL be refused at `enroll`, naming what it lacks (core.32).

#### Scenario: Kept by systemd-creds

- **WHEN** partition `home` is enrolled into the `systemd-creds` store, its program a stub reporting version 256
- **THEN** the stub is run as `encrypt --user --name=boomdrive.home.<the url's host>`, the key's bytes reach it only on standard input, and no argument holds them

#### Scenario: An old systemd

- **WHEN** the `systemd-creds` store is asked for, its program a stub whose `--version` reports 255
- **THEN** it is refused naming systemd 256 as the least it takes, as the host's to fix, and an enrollment over it makes no request

### Requirement: Every connection proves the host's key

Each upgrade of `runner/connect` SHALL ask for `boomdrive.v6` alone. It SHALL carry `Authorization: BoomdriveHost` with the key, the time, a nonce and a signature over version 6's context, new for each attempt. The runner SHALL keep nothing of enrollment but the key. It SHALL say a refused key or subprotocol once and try again at the backoff's cap (core.22, core.23, core.31, record 0053).

#### Scenario: A connection proved

- **WHEN** partition `home` dials the fake control plane twice
- **THEN** each upgrade asks for `boomdrive.v6` alone, its signature checks over its context of version 6 with `home`'s key, and the two nonces differ

#### Scenario: A key refused

- **WHEN** the fake control plane refuses `home`'s key three times
- **THEN** one line names `home` and the fake's words. The line names the command `boomdrive enroll --control-plane <the fake's url> --drive home --repository example/app`.

#### Scenario: A control plane that speaks version 1 alone

- **WHEN** the fake control plane answers `home`'s upgrades without the subprotocol, three times
- **THEN** one line names `home` and `boomdrive.v6`, each later attempt waits for the backoff's cap, and no message is sent

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

Each partition's hello SHALL name only the repositories its table names, of those kept with their clones made, and only its own leases. No hello SHALL name a repository the runner drains, and no report the repository a launch was answered in, since version 6 carries neither. Each report SHALL carry only the entries under its prefix. A want naming a repository the partition does not name SHALL not be given to the runner, and one line SHALL name it (core.26; ledger 3.b; record 0053).

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

### Requirement: What goes up is version 6's copy, checked before it goes

Each place's status, sluice entry, preview's answer and listing, and host part SHALL go up cut to version 6's copy by walking its schema, and checked against it. A copy that fails outside a place's ends SHALL go up `null`, or the host part `missing`, with one line naming the launch or the preview and the schema's path, never a value. Every message SHALL be checked against `from-runner.schema.json` before it is sent (core.11, core.27, core.42; record 0037 as amended on 2026-11-09; record 0052).

#### Scenario: Flume's places cut

- **WHEN** each place of Flume 0.21.0's and Flume 0.4.1's fixtures is carried in a report
- **THEN** its copy passes `place-status.schema.json` of version 6 and holds no `last_words`, `screen`, `printed`, task `text`, `readiness` or `repository`; each end of Flume 0.21.0's goes up in a form version 6 names, with its `key` and `delivery`; and each end of Flume 0.4.1's, which Flume printed with no `key`, is left out

#### Scenario: A value Flume adds

- **WHEN** a place's status names a stage's state Flume 0.21.0 does not have, in its `stages`
- **THEN** that launch's status goes up `null`, the report passes and is sent, and one line names the launch and the path, and not the value

#### Scenario: sluice's previews cut

- **WHEN** sluice 0.74.3's answer of a preview brought up and each row of its listing are cut as a preview's copy, and its answers of a release as a release's copy
- **THEN** each copy passes its definition of version 6, keeps the `state` or `released` sluice printed, and no preview's copy holds `checkout`, `names`, `ports`, `exit` or `output`

#### Scenario: A preview's copy that fails

- **WHEN** a preview's status, as the runner carries it, holds a `made` that is a number
- **THEN** that preview's `status` goes up `null` and its `copy` as before, the report passes and is sent, and one line names the preview and the path, and not the value, once while it fails

### Requirement: Every message the runner is sent is checked before it is read

Every message a connection receives SHALL be checked against `to-runner.schema.json` of version 6 before it is read. One that fails SHALL be answered with `message_refused` naming it, with words naming the schema's path, and nothing else done with it, as SHALL an `enrolled` or `enroll_refused`, which only `runner/enroll` takes. One whose id cannot be read SHALL close the connection, which is tried again on the backoff (core.42; records 0052 and 0053).

#### Scenario: A pulled with a field unnamed

- **WHEN** the fake control plane sends a `pulled` whose launch holds a field version 6 does not name
- **THEN** the runner answers `message_refused` naming that message, and the pull gives none of its wants

#### Scenario: A message of version 1

- **WHEN** the fake control plane sends a `pulled` naming `protocol_version` 1
- **THEN** the runner answers `message_refused` naming that message, its words saying it fails version 6's check, and the pull gives none of its wants

#### Scenario: A message of version 2

- **WHEN** the fake control plane sends a `pulled` naming `protocol_version` 2
- **THEN** the runner answers `message_refused` naming that message, its words saying it fails version 6's check, and the pull gives none of its wants

#### Scenario: A message of version 3

- **WHEN** the fake control plane sends a `pulled` naming `protocol_version` 3
- **THEN** the runner answers `message_refused` naming that message, its words saying it fails version 6's check, and the pull gives none of its wants

#### Scenario: An enrollment's answer on the socket

- **WHEN** the fake control plane sends `enrolled` on `home`'s connection
- **THEN** the runner answers `message_refused` naming it, its words naming `runner/enroll`

### Requirement: A terminal the control plane opens is closed at once

Every `terminal_open` SHALL be answered with `terminal_closed`, its words saying the runner has no end-to-end exchange for a terminal yet and relays none in the clear, and no key made. Messages naming a terminal never opened SHALL be dropped, and a pull SHALL give no terminal (core.30; record 0004 as amended on 2026-11-15).

#### Scenario: A terminal opened

- **WHEN** the fake control plane sends `terminal_open` for a launch the runner holds
- **THEN** the runner answers `terminal_closed` for it, with words naming its end-to-end exchange and not naming Flume, and the pull gives no terminal

### Requirement: A report's answer names taken each status that went up as its copy

The answer to a part's report SHALL name taken each launch whose status went up whole as its copy in the report it answers, every end of it in a form version 6 names. It SHALL name none whose status went up `null`, and none whose status went up with an end carried by its delivery alone or left out. An answer to an earlier report SHALL name none (core.11, core.27, core.42, core.54; records 0030 as amended on 2026-11-09, 0037, 0052 and 0053).

#### Scenario: A status that goes up as its copy

- **WHEN** a place's status passes version 6
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

Each end of a place's status SHALL be cut by the form version 6 names for its `of`, and checked; where there is none, or the check fails, cut to its delivery alone and checked; and where that fails too, left out. The rest of the status SHALL go up as its copy whatever its ends. A status with an end carried by its delivery alone or left out SHALL be said in one line while it lasts, naming the launch and each end's path, never a value (core.11, core.27, core.42; records 0037 and 0052).

#### Scenario: An end Flume adds

- **WHEN** a place's status holds an end whose `of` is `moved`, a stage's end whose `state` version 6 does not have, an end with no `key`, and a submission's end that passes
- **THEN** the first two go up as their delivery alone, the third is left out, the fourth goes up whole, the rest of the status goes up as its copy, the report passes and is sent, and one line names the launch and each of the three ends' paths, and no value

### Requirement: A control plane's confirm is given to the runner as any act

A `pulled` holding an act whose `what` is a confirm SHALL pass version 6's check and be given to the runner as an act, its id and its launch under the partition's prefix, its recipient and its delivery keys as the control plane gave them, in its order. The hosted implementation SHALL refuse nothing of a confirm and read nothing of it, the runner carrying it out as it carries crew's (core.2, core.26, core.56; records 0037, 0052 and 0053).

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

A `pulled` holding an act whose `what` is `land` SHALL pass version 6's check and be given to the runner as an act, its id and its launch under the partition's prefix. The hosted implementation SHALL read nothing of a land, the runner carrying it out as it carries crew's (core.2, core.26, core.53; record 0037 as amended on 2026-11-09; records 0052 and 0053).

#### Scenario: A land carried

- **WHEN** `home`'s control plane offers launch `l-1` and asks act `land-1` on it, a land
- **THEN** `home`'s part's pull gives act `home/land-1` on `home/l-1`, a land, and the runner sent no `message_refused`

### Requirement: A control plane's preview is given to the runner under the partition's prefix

A `pulled` holding a preview SHALL pass version 6's check and be given to the runner, its id under the partition's prefix and every other field as given, in the control plane's order. A preview of a repository the partition is not served SHALL be withheld, one line naming it while it is offered. The hosted implementation SHALL read nothing else of a preview (core.2, core.26, core.55; record 0037 as amended on 2026-11-09; records 0052 and 0053).

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

A partition's hello SHALL name no drive and no clone's folder, since version 6 carries neither and a path goes up only where the protocol's copy names it (core.27, core.28, core.42; record 0053).

#### Scenario: No drive or folder goes up

- **WHEN** the runner's hello names the drive `family` with `example/app` at `/data/repositories/example/app`, and `home` names `example/app`
- **THEN** `home`'s hello names `example/app`, and neither `family` nor `/data/repositories/example/app` appears in it

### Requirement: A control plane's hold is given to the runner as any act

A `pulled` holding an act whose `what` is a hold SHALL pass version 6's check and be given to the runner as an act, its id and its launch under the partition's prefix, its words and its bound as the control plane gave them. The hosted implementation SHALL refuse nothing of a hold and read nothing of it, the runner carrying it out as it carries crew's (core.2, core.26, core.59; records 0052 and 0053).

#### Scenario: A hold carried

- **WHEN** `home`'s control plane offers launch `l-1` and asks act `hold-1` on it, a hold under the words `the user sends it on the landings card, row 3` for 28800 seconds
- **THEN** `home`'s part's pull gives act `home/hold-1` on `home/l-1`, a hold under those words for 28800 seconds, and the runner sent no `message_refused`

#### Scenario: A hold the runner refuses

- **WHEN** `home`'s control plane asks act `hold-2` on `l-1`, a hold whose words are empty and whose bound is 0
- **THEN** `home`'s part's pull gives act `home/hold-2` with no words and a bound of 0, the runner sent no `message_refused`, and it is the runner's to refuse

### Requirement: A served repository's token is asked by one request of version 6

The hosted implementation SHALL ask for a repository's token by one `POST runner/forge-token` under `url`, its body a `forge_token` of version 6 naming the repository, its `Authorization` a `BoomdriveHost` statement signed over version 6's `forge token` context, made afresh for each ask with a partition's key read from the host's store. It SHALL open no socket and no part, keep nothing of an ask, and stop its asks at twenty seconds (core.3, core.31, core.33, core.35; record 0053).

#### Scenario: A token given

- **WHEN** `home`, served `example/app`, asks for `example/app`'s token, and the fake control plane answers `forge_token_given` with a token and `expires_at` `2026-11-18T13:00:00Z`
- **THEN** the ask's body passed version 6 and named `example/app`, its signature checks over the `forge token` context with `home`'s key and `example/app`, and the answer is that token, ending at that time

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

### Requirement: An ask's answer is checked against version 6 before it is read

An answer to an ask SHALL be read only once it passes `to-runner.schema.json` of version 6, whatever its status, and only where it is a `forge_token_given` or `forge_token_refused` naming the ask and its repository. An answer that is no such message, or that never comes, SHALL refuse that partition's ask, naming `runner/forge-token` and the status, and no value of the answer (core.11, core.42; record 0053).

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

The host can have no credential store, or no partition served the repository can have its key in the store. There, the runner SHALL refuse the ask as the host's to fix, with no request made. The refusal SHALL name the command that enrolls each such partition. The command is `boomdrive enroll --control-plane <url> --drive <label>`, with a `--repository` for each of its repositories (core.32 and core.39, with record 0053).

#### Scenario: No key of the partition

- **WHEN** `home` is served `example/app`, and the store holds no key of `home`
- **THEN** the runner refuses the ask as the host's to fix, naming `boomdrive enroll --control-plane https://control.example --drive home --repository example/app`. The fake control plane took no request.

### Requirement: A forge token's answer on a partition's socket is refused

A `forge_token_given` or `forge_token_refused` a partition's connection receives SHALL be answered with `message_refused` naming it, with words naming `runner/forge-token`, which alone takes a forge token's ask and answers. Nothing else SHALL be done with it: no token of it is kept, said or given to the runner (core.4, core.42; record 0053).

#### Scenario: A forge token's answer on the socket

- **WHEN** the fake control plane sends `forge_token_given` on `home`'s connection
- **THEN** the runner answers `message_refused` naming it, its words naming `runner/forge-token`, and nothing of it is kept, said or given to the runner

### Requirement: Each partition is told of the unleased places in its repositories alone

Each partition's report SHALL hold `unleased`: each unleased place of its share whose repository its table names, each status cut to version 6's copy as a launch's is, and no other. A place in a repository its table does not name SHALL not go up on its connection (record 0054; core.26, core.27).

#### Scenario: A place in the partition's repositories

- **WHEN** the partition `home`, whose table names `agentplot/flume`, is handed a share whose `unleased` holds a place in `agentplot/flume` and one in `agentplot/sluice`
- **THEN** its report's `unleased` names the place in `agentplot/flume` alone, its status a copy of version 6, with no `lease`

#### Scenario: No unleased place

- **WHEN** the partition `home` is handed a share holding no unleased place
- **THEN** its report names `unleased` as `{}`

### Requirement: boomdrive enroll refuses before the token what it cannot set up

`boomdrive enroll` SHALL exit 2 before it reads the token and before any request in each of these cases:

- A flag is missing or out of form.
- The runner's file neither names the enrollment nor is one that enroll writes.
- The file that enroll would write fails the runner's reading or the hosted implementation's.
- The host has no credential store.

An empty token SHALL also exit 2, with no request. A control plane that enroll cannot reach SHALL exit 1, and enroll SHALL write no key and no file.

#### Scenario: A flag missing

- **WHEN** `boomdrive enroll --control-plane https://control.example --repository example/app` runs
- **THEN** it exits 2, naming `--drive`, and reads no token and makes no request

#### Scenario: A drive name out of form

- **WHEN** `boomdrive enroll --control-plane https://control.example --drive Home --repository example/app` runs
- **THEN** it exits 2, naming `Home` and the form, and reads no token

#### Scenario: A drive another account holds

- **WHEN** enroll runs with `--drive home`. The runner's file holds the drive `home` under the account `own`, which crew serves.
- **THEN** it exits 2, naming the drive `home` and the account `own`. It reads no token and makes no request.

#### Scenario: Nothing listening

- **WHEN** enroll runs with a token on a host with no runner file, and `--control-plane` names a loopback port with nothing on it
- **THEN** it refuses, naming the control plane as unreachable, and exits 1. It writes no key and no runner file.

### Requirement: The token comes on standard input alone

`boomdrive enroll` SHALL take the token from standard input alone, never from a flag, an argument or the environment (core.31). The control plane's page SHALL so give the whole command as `echo <token> | boomdrive enroll …`. Where standard input is a terminal, enroll SHALL ask for the token on standard error and read it with no echo.

#### Scenario: A token flag

- **WHEN** enroll runs with `--token secret-enrollment-token` beside its other flags
- **THEN** it exits 2 as a usage error, naming `--token`, and its output holds no `secret-enrollment-token`

### Requirement: The drive names the account, the drive and the partition

enroll SHALL name the account, the drive and the hosted partition after `--drive`, so no person chooses a partition's label. The account SHALL say `control_plane = "hosted"`. The drive and the partition SHALL each name the `--repository` flags, in their order (core.5 and core.19, with record 0028).

#### Scenario: The names

- **WHEN** enroll writes the runner's file for `--control-plane https://control.example --drive atlas --repository madswan/atlas`
- **THEN** the file holds the account `atlas` with `control_plane = "hosted"`, and its drive `atlas` keeps `madswan/atlas`. `[control_plane.hosted]` names the `url` and the partition `atlas` with `madswan/atlas`.

### Requirement: A file that names the enrollment is left as it is

Both layers, as `run` reads them, can name the enrollment. They then name the hosted `url` given, a partition of the drive's name, and that drive under a hosted account. The partition and the drive each name every repository given. There, enroll SHALL write no file, touch no agent, and enroll the partition.

#### Scenario: A host Nix set up

- **WHEN** enroll runs with a token that the fake control plane enrolls. The runner's file is a link that names the account `atlas`, its drive and the partition `atlas` at the fake's url, each with `madswan/atlas`.
- **THEN** the key is in the store under `atlas`, and the link is as it was. enroll writes no agent, and `config`'s outcome is `unchanged`.

### Requirement: enroll writes the runner's own file on macOS

On macOS, the runner's managed file can be missing, or its first line can be setup's. There, enroll SHALL write the file once the control plane enrolls the partition, and never before. It SHALL set the account, the drive and the partition. It SHALL keep every other account, drive and table as it stands, by setup's own writer and with setup's first line (record 0054 and core.18). Where the file's hosted table names another `url`, enroll SHALL exit 2 before it reads the token, naming both.

#### Scenario: A host with no file

- **WHEN** enroll runs on macOS with a token that the fake control plane enrolls, and the home has no runner file
- **THEN** `~/.config/boomdrive/config.toml` starts with setup's first line and holds the account, the drive and the partition. It reads as the runner's configuration, and `config`'s outcome is `written`.

#### Scenario: Beside crew's drives

- **WHEN** enroll runs with `--drive atlas` on macOS. Setup's file holds the account `own`, which crew serves, with its drive `home` and `[control_plane.crew]`.
- **THEN** the file holds the account `own`, its drive `home` and `[control_plane.crew]` as they stood. It also holds the account `atlas`, its drive and `[control_plane.hosted]`.

#### Scenario: Another hosted control plane

- **WHEN** enroll runs with `--control-plane https://control.example`, and setup's file holds `[control_plane.hosted]` with `url = "https://other.example"`
- **THEN** it exits 2, naming both addresses. It reads no token and writes nothing.

#### Scenario: The token refused

- **WHEN** enroll runs on a host with no file, and the fake control plane answers `enroll_refused`
- **THEN** enroll writes no runner file and no agent

### Requirement: A file Nix or a person wrote is not changed

The runner's managed file can be a link, or a file whose first line is not setup's, that does not name the enrollment. There, enroll SHALL change nothing and read no token. It SHALL exit 2, naming the file and the keys to add. Its words SHALL say that the same command enrolls the host after the change. They SHALL say that the token is not spent (core.17 and core.18).

#### Scenario: A person's own file

- **WHEN** enroll runs, and `~/.config/boomdrive/config.toml` is a file whose first line is not setup's and that names no hosted control plane
- **THEN** it exits 2, naming the file and the TOML keys to add. The file is as it was, and enroll makes no request.

### Requirement: For a file Nix wrote, enroll names the module's options

Where the runner's managed file is a link and does not name the enrollment, enroll SHALL name the keys to add as the options `services.boomdrive.accounts.<drive>` and `services.boomdrive.settings.control_plane.hosted`.

#### Scenario: The file is Nix's

- **WHEN** enroll runs with `--drive atlas --repository madswan/atlas`, and `~/.config/boomdrive/config.toml` is a link that names no hosted control plane
- **THEN** it exits 2, naming the file, `services.boomdrive.accounts.atlas` and `services.boomdrive.settings.control_plane.hosted`. The link and `~/Library/LaunchAgents` are as they were, and enroll makes no request.

### Requirement: Off macOS, enroll writes no file

On a system other than macOS, where the file does not name the enrollment, enroll SHALL change nothing, read no token and exit 2. It SHALL name `services.boomdrive` as where a host of that system sets up its runner.

#### Scenario: On Linux with no file

- **WHEN** enroll's host is Linux and the home has no runner file
- **THEN** it exits 2, naming `linux` and `services.boomdrive`. It writes nothing and makes no request.

### Requirement: The runner's agent runs the file enroll wrote

Where enroll wrote the runner's file on macOS, it SHALL write and load setup's agent as setup does. The runner reads `[control_plane]` when it starts. So where the agent is loaded with its plist unchanged, enroll SHALL run `launchctl kickstart -k gui/<uid>/io.github.agentplot.boomdrive`. Where enroll wrote no file, it SHALL run no `launchctl` (core.13).

#### Scenario: No agent loaded

- **WHEN** enroll writes the runner's file, and `launchctl print` finds no agent
- **THEN** it writes the plist and runs `launchctl bootstrap gui/<uid>` with it, and `service`'s outcome is `loaded`

#### Scenario: The agent running

- **WHEN** enroll changes the runner's file, and `launchctl print` finds the agent with its plist unchanged
- **THEN** it runs `launchctl kickstart -k gui/<uid>/io.github.agentplot.boomdrive` and no `bootstrap`. `service`'s outcome is `restarted`.

### Requirement: The answer names the file and the agent

The answer SHALL keep `partition` and `host`, and add `config`, with `file` and `outcome`, `written` or `unchanged` (core.39). Where enroll wrote the file, the answer SHALL add `service`, with `file` and `outcome`, `loaded`, `reloaded`, `restarted` or `running`. Without `--json`, enroll SHALL say one line for each step.

#### Scenario: The answer

- **WHEN** enroll runs with `--json` on macOS with no runner file, and the fake control plane enrolls host `h-1`
- **THEN** it prints `{"enrolled":{"partition":"atlas","host":"h-1","config":{…,"outcome":"written"},"service":{…,"outcome":"loaded"}}}` and exits 0

### Requirement: A launch Flume refuses goes up refused

Where Flume refuses a launch that a hosted control plane offers, the part's next report SHALL hold that launch under its own id. Its answer SHALL have the outcome `refused`, Flume's words as Flume gave them, and no place (core.1, core.11, core.27).

#### Scenario: A launch Flume refuses goes up refused

- **WHEN** `home`'s control plane offers the launch `l-1`, and the stub `flume` refuses it with its words
- **THEN** the report that the pass sends holds `l-1` with the outcome `refused`, Flume's words and no place, and the report passes version 6

### Requirement: A newest release is said once on each connection

On a connection of version 6, the runner SHALL say each `newest_release` in one line, `<label>: this runner is <runner_version>, and the newest release is <version>`, its `runner_version` the hello's that the connection sent. It SHALL say each release one time on each connection, and again on a new connection. It SHALL compare no version, act on none, and raise no alarm (core.1, core.23, Boom: `the-cloud-leads-the-runners-protocol`).

#### Scenario: A newest release is said once on each connection

- **WHEN** partition `home` speaks version 6 with a hello that names `boomdrive 0.1.0`, and the fake control plane sends `newest_release` of `0.32.0` two times, then of `0.33.0`
- **AND** the fake drops the connection and sends `0.33.0` on the next
- **THEN** the upgrade asks for `boomdrive.v6`, `home` says `home: this runner is boomdrive 0.1.0, and the newest release is 0.32.0` one time, the line for `0.33.0` one time on each connection, and refuses nothing

### Requirement: Version 5 carries no newest release

On a connection of version 5, a `newest_release` fails the check and SHALL be answered with `message_refused`, and no line SHALL name a release (core.42).

#### Scenario: Version 5 carries no newest release

- **WHEN** partition `home` speaks version 5 and the fake control plane sends `newest_release`
- **THEN** the runner answers `message_refused` naming that message, its words saying it fails version 5's check, and no line names a release

### Requirement: On macOS the key is kept through macOS's own security

On macOS, the runner SHALL read and write a partition's key only through macOS's own `security`, by its path `/usr/bin/security`. The key SHALL reach `security` only on standard input, and SHALL come back only on standard output. So each build of the runner reads the item with no ask (core.32, record 0057).

#### Scenario: Kept in the keychain through security

- **WHEN** the keychain store, its program a stub `security`, reads the key of `home` at `control.example`, keeps it, then reads it again
- **THEN** each read runs `find-generic-password -s boomdrive -a home@control.example -w`, and the first read gives no key
- **AND** the write runs `-i`, with the line `add-generic-password -U -s boomdrive -a 'home@control.example' -X <the key in hex>` on standard input, and no argument holds the key
- **AND** the second read gives the key that was kept

#### Scenario: A key of printable bytes

- **WHEN** the stub `security` prints an item of 32 printable bytes, as `security` prints an item whose bytes are all printable
- **THEN** the key is those 32 bytes, also where they are hex digits

#### Scenario: The keychain refuses

- **WHEN** the stub `security` refuses a read with exit 51, or refuses a write with `add-generic-password: returned 25`
- **THEN** the read or the write fails with `security`'s words, and names the partition
