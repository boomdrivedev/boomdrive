# Design

## Context

See proposal.md for why. This change builds the host's half of boomdrive-cloud's walking skeleton, its roadmap's items 1.4 to 1.15: a host enrolls, dials its control plane, takes launches under their leases, and carries each place's status up as version 1's copy.

What stands:

- **This repository**, at `508e41d`.
  - `crates/boomdrive-control` gives `ControlPlane`, with `pull(hello, report)` and `report(report)`, the `Wake` an opener is handed, and the shared cases every implementation passes (`the-runner-carries-crews-work-into-flume`, archived).
  - `crates/boomdrive-crew` is the one implementation. `crates/boomdrive/src/planes.rs` lists it as a word against its opener, and refuses `hosted` as a word this binary does not carry.
  - `crates/boomdrive-hosted` holds version 1 of the protocol, `protocol/v1/`, and the runner's types for its messages, `boomdrive_hosted::protocol::v1`. It implements no `ControlPlane` (`the-runner-keeps-its-protocol-as-one-json-schema-per-version`, archived).
  - The control pass, `carry.rs`, hastens its next pass by the midpoint to the earliest lease expiry, a lapsed one too, so passes run back to back once a lease has lapsed. It says the control plane could not be read at every pass that cannot read it. `_open.md` names both, against core.22 and core.23.
  - The pass launches an offer and carries out an act or a command whatever its lease's expiry. crew's leases have none, so core.24 has not yet bitten.
  - Rules core.1 to core.42 bind. Records 0001 to 0015 stand, 0006 to 0010 and 0012 to 0015 proposed.
- **agentplot/boomdrive-cloud**, read 2026-11-03.
  - Its roadmap, `design/roadmap.md`: item 1.15 is this change, needing 0.5 and 1.16, under ruling D5, with research rows F10, F49 and F51.
  - Its records 0005, 0009, 0016, 0022 and 0043 to 0049, 0056 and 0059, and its rules core.13, core.18, core.21, core.24, core.36 to core.39, core.50, core.52, core.106, core.108, core.118 and core.121. All are cited by id, and none is quoted (core.41).
  - Its first run, `design/the-first-run-from-an-empty-account.md`: FR3 and FR7, and the install command's `--keep`.
  - Its items 1.4 to 1.7 are designed beside or after this one, so this change sets the runner's half of each path and says what 1.7 builds to meet it.
- **This repository's constitution interview**, agentplot/boomdrive-archive's `design/research/the-constitution-interview.md`, *The roadmap's changes*: it lists this change as its seventh.

There are no area pages, so `docs/architecture/core.md` binds every path this change touches.

## What the cloud's earlier items settle for this change

The cloud roadmap's earlier items bear on this change as follows.

- **`the-runner-carries-crews-work-into-flume`** (the cloud's item 0.5), landed here as #2.
  - Its `ControlPlane` is what this change implements. It is unchanged, but for the opener in `planes.rs` taking `poll_interval` (Decision 1).
  - Its Decision 20 sketched the hosted implementation. Three of its points are overtaken by later records (below), and the rest stand: one WebSocket, outgoing; the hello at every reconnect; a pull answered from the messages received; the `Wake` called when one arrives; a refused lease filling `Reported::refused`; one more word in `planes.rs`; the shared cases passed.
  - Its report file, its marks and its control pass are what the hosted implementation feeds.
- **`the-runner-keeps-its-protocol-as-one-json-schema-per-version`** (the cloud's item 1.16), landed as #4 at `508e41d`.
  - Version 1 is what goes over the socket: the envelope with its id, the messages and their order (its Decision 5), the copies (its Decisions 7 to 9) and the terminal's frames (its Decision 10).
  - Its Decision 11 left enrollment and the proof at each connection to this change, to add to version 1 before a release speaks it. This change adds them (Decision 12).
  - It asked of this change: cut each status and the host part to their copies (Decision 9); check every message the runner is sent (Decision 10); give message ids that survive a reconnect (Decision 7); add enrollment and the key's proof (Decisions 4, 5 and 12).
- **Record 0056** puts the schema in this repository, which the cloud pins by revision. So the enrollment messages are written here, in version 1, and the cloud's 1.6 moves its pin to this change's landing.
- **Record 0049** keeps of a place's status only what a person reads to follow or answer it, and gives the schema no field for the rest. 1.16 wrote that copy, and this change cuts each status to it. Because every object of version 1 is closed, the check before sending is what holds the line, not the cut alone (Decision 9).
- **Record 0047** enrolls a host to its Account alone, never to a member. So `enroll` names no member and no account. A partition is one enrollment, to one Account, whose token the Account issued (Decision 4).
- **Record 0059**, proposed, takes effect with the stage that brings host pools back, and changes nothing now. It agrees with this design where it reaches it:
  - The hosted table is the machine's, written by hand, by nix or by the install. The cloud never sends it (its core.118; core.17).
  - Enrollment is the cloud's on every host (its core.121). `boomdrive enroll` reads the token on standard input, so a manager may place it, and the key is still made on the host.
  - Work policy and live controls are not touched.
- **Rulings 0043 to 0049.**
  - 0043, a member's launch, reaches the runner as any offer does, and needs nothing of its own here.
  - 0044, 0045 and 0048 have no bearing on the host.
  - 0046 is superseded by 0056.
  - 0047 and 0049 are above.
- **The first run (0.3).**
  - FR3. `enroll` carries the machine's name. The repositories it keeps and its streams' room are its first hello's. The token it used and who took it are the cloud's own. A host not admitted is offered nothing, and the runner needs nothing for that.
  - FR7. The token lapsed is `enroll_refused`, with the control plane's words, which `boomdrive enroll` prints as given. A tool missing is the hello's. The host gone quiet is the socket's silence.
  - `--keep`. The install command writes the runner's configuration on the host, the repositories to keep among it, and the cloud sends nothing to the machine. That install is this repository's `the-runner-installs-as-one-verified-release` (record 0012), not this change (*What this change leaves out*).
- **Items 1.1, 1.2, 1.3 and 1.14** are the cloud's own. A host dials what 1.14 deploys, and its address goes in a host's `url`, and never in this repository (core.41).

Where a record of the cloud's or of this repository's changes what the cloud's item 1.15 or the archived Decision 20 says, the record wins:

- **The host's credential is a key, not a kept token.** Decision 20 read a token from a `token_file`. Record 0006, with core.31 and core.32, makes the credential a key pair made on the host and kept only in its credential store, proved at every connection. So the token is exchanged once, to register the key. Nothing in the table names a token or a file, and no token is kept (Decisions 3 to 5).
- **A report is whole.** Decision 20 sent each piece the server had not acknowledged under its own id. 1.16's Decision 5 sends the report whole and resends only the last one no `reported` answered, under its own id (Decision 7).
- **No terminal opens yet.** Decision 20 made a terminal's link a stream of the socket's messages. Record 0004 closes every terminal at once until Flume relays one, and 1.16's frames are sealed end to end. So this change answers every `terminal_open` with `terminal_closed` (Decision 11).
- **The install is a change of its own.** Item 1.15 and its F51 ask for the installers and the service units. Record 0012 puts the one verified install in its own change. That change waits on the public repository (record 0014), since a release a stranger's `curl` fetches must be public. This change builds what that install runs, `boomdrive enroll`, and until it lands a host is installed by hand (*What this change leaves out*).

## Goals / Non-Goals

**Goals:**

- `type = "hosted"` carried, through the one interface, by the hosted implementation's crate (core.2).
- Each partition enrolled once by `boomdrive enroll`, its key made on the host and kept only in its credential store (core.31, core.32; record 0006).
- One outgoing WebSocket per partition: proved at its upgrade, kept by pings, retried on a bounded backoff, its hello first, its unanswered report resent (core.3, core.21, core.22).
- Each partition's own repositories, leases and work, and nothing of another's (core.26).
- Of each tool's answer, only version 1's copy goes up, checked before it goes. Every message the runner is sent is checked before it is read (core.27, core.42).
- The control pass holding to core.22, core.23 and core.24 under leases that expire.

**Non-Goals:**

- **The install and upgrade**: dist's release, provenance, the service units and the nix flake (`the-runner-installs-as-one-verified-release`; record 0012; core.37).
- **A terminal**: the runner's half of the key exchange, its suite and its certificate (`a-terminal-encrypted-end-to-end`, with the cloud's 5.5).
- **Health and the OTLP export** (record 0013).
- **Forge tokens and `boomdrive forge-token`** (record 0007).
- **The runner's own commands and a stray place's drop**: drain, resume, upgrade and setup (records 0008 to 0010).
- **The managed level** (record 0009).
- **`status`, `check`, and times on every line** (core.39, core.40).
- **A change to `boomdrive-control`'s types**, or to crew's implementation.

## What this change rests on

1. **`the-runner-carries-crews-work-into-flume` and `the-runner-keeps-its-protocol-as-one-json-schema-per-version`, landed.** Where a type or a schema file changes before this change is built, the coder takes the change and names it.
2. **The cloud's records and rules above.** This change cites them by id and copies no text of them (core.41).
3. **Version 1 still open.** No release of the runner has spoken version 1, so this change adds to it (core.42 "Not ruled out").
4. **The cloud's 1.5 and 1.7 building the other ends of the two paths** as Decisions 4 and 5 and the README describe them. Until they do, the hosted implementation is proved against its fake (Decision 14).

## Decisions

### 1. The hosted implementation, listed in `planes.rs` with its enrollment

`crates/boomdrive-hosted` implements `ControlPlane`. It is the one crate that names the hosted protocol (core.2, core.42). Its modules:

| module | what it holds |
|---|---|
| `table` | the hosted table, read and refused (Decision 2) |
| `key` | the host's key, its stores and the signatures it makes (Decision 3) |
| `enrollment` | the enrollment request and its answer (Decision 4) |
| `connection` | one partition's socket: its proof, hello, pings, backoff and reach (Decisions 5 and 6) |
| `plane` | `Hosted`, the `ControlPlane` over every partition's connection (Decisions 7, 8 and 11) |
| `copy` | the cut of a tool's answer to its copy, walked from the schema (Decision 9) |
| `check` | the validators of version 1, built once from `FILES` (Decisions 9 and 10) |
| `protocol` | version 1, as it stands, with enrollment added (Decision 12) |

`planes.rs` stays the one place outside an implementation that names one. Its list becomes a list of entries, each a word, its opener and its enrollment where it has one:

- `crew`: crew's opener, and no enrollment;
- `hosted`: `boomdrive_hosted::open`, and `boomdrive_hosted::enroll`.

An opener is handed `poll_interval` beside what it is handed now, since the hosted connection's backoff stops there (core.22). crew's ignores it.

`boomdrive enroll` (Decision 4) reaches an implementation's enrollment through `planes::enroll`, by the configuration's `type`. A control plane whose implementation enrolls nothing, crew's, is refused, naming it. So the binary's command names no implementation, and the rules test that holds `planes.rs` to be the one place still holds.

`crates/boomdrive` depends on `boomdrive-hosted` as it depends on `boomdrive-crew`. The control-plane-interface spec's "no crate depends on it", written while the crate implemented nothing, becomes: only the binary depends on it, and only `planes.rs` names it among the binary's sources.

The implementation runs its connections on a tokio runtime of its own, on threads it owns. The runner's loop stays one thread (the archived Decision 5), and reaches the connections only through `pull` and `report`, which hand them what they need and wait at most as Decision 7 says. Dropping the implementation closes each socket with a normal close and stops its runtime.

Alternatives:

- **The enrollment in the binary, calling the hosted crate directly.** Not taken: a second file of the binary's would name the hosted implementation (core.2).
- **An `enroll` call on the `ControlPlane` trait.** Not taken: enrollment runs before any control plane can be opened, with no data directory and no lock. It is a step of one implementation's, not of the interface's.
- **The socket on the runner's own loop thread, polled between passes.** Not taken: pings and a `pulled` would wait on a git fetch. A runtime of its own lets the socket answer at once and wake the loop (`Wake`).

### 2. The hosted table

```toml
[control_plane]
type = "hosted"
poll_interval = "1m"

[control_plane.hosted]
url = "https://control.example"

[[control_plane.hosted.partitions]]
label = "home"
repositories = ["example/app"]

[[control_plane.hosted.partitions]]
label = "work"
repositories = ["example/service", "example/app"]
```

- **`url`** is the control plane's address. It is required. It is `https://`, or `http://` only to a loopback address, `127.0.0.1`, `[::1]` or `localhost`, which is how the tests reach their fake. It holds no user, query or fragment, and may hold a path, under which the protocol's two paths go (Decision 12).
- **`partitions`** are each one enrollment (record 0006), at least one:
  - `label` is the host's own name for the partition. It names the partition's key in the store, prefixes its ids (Decision 8), and starts its lines. It is 1 to 32 lowercase letters, digits and `-`, starting with a letter, given once.
  - `repositories` are the addresses, `owner/name`, that this partition is served: the ones its hello names and whose work it is given (Decision 8). It is required, and may be empty. Two partitions may name one repository, which the host keeps in its one clone (core.14).
- **Any other key is refused**, naming it, as crew's table refuses one. A `token`, `token_file` or `key` is refused as a key the hosted implementation does not read, with words saying the host's key is in its credential store and made by `boomdrive enroll`. The value appears in no output (core.4).

What the host keeps is still the configuration's top-level `repositories` (core.5). A partition's `repositories` says which of them the partition is served, and a repository it names that the host does not keep is never in its hello. An offer for such a repository reaches the runner, which refuses it as one this host does not keep (the archived Decision 7), as it would from crew.

The table is read at start, as every control-plane table is (the configuration spec). A partition added while the runner runs is served from its next start.

Alternatives:

- **Every kept repository to every partition.** Ruled out by core.26: one hello listing every repository sent to every partition.
- **Each partition's `repositories` as the only list, with no top-level one under a hosted control plane.** Not taken: the keeper reads the top-level list for every control plane alike (core.5), and a repository two partitions name is kept once.
- **One `url` for each partition.** Not taken: a runner serves one control plane (core.19), and every partition is that control plane's.

### 3. The host's key, made on the host and kept in its credential store

Each partition's key is an Ed25519 key pair, made on the host by `boomdrive enroll` from the operating system's randomness. The private key never leaves the host (core.31). It signs three things: an enrollment (Decision 4), a connection (Decision 5) and, once a terminal is built, a terminal's share (1.16's Decision 10).

It is kept only in the host's own credential store (core.32; F10):

- **On macOS**, in the user's login keychain, through keyring v4's macOS store, as a generic password whose service is `boomdrive` and whose account is `<label>@<the url's host>`. The runner's launchd agent runs in the user's session, so it reads the keychain.
- **On Linux**, as a user-scoped encrypted systemd credential, which needs systemd 256 or later (record 0006).
  - `boomdrive enroll` encrypts the key with `systemd-creds encrypt --user --name=<name>`, reading it on standard input, and `systemd-creds` writes the credential to `$XDG_CONFIG_HOME/credstore.encrypted/<name>` (`$HOME/.config` where `XDG_CONFIG_HOME` is unset), the user's encrypted credential store, which the user's service manager searches. The runner makes that directory, the user's alone, where it is missing.
  - The runner decrypts it with `systemd-creds decrypt --user --name=<name>` at each connection, reading the key from standard output into memory.
  - `<name>` is `boomdrive.<label>.<the url's host>`.
  - The credential is bound to the host's own key, and to its TPM where it has one, and only this user on this host can decrypt it.
- **Elsewhere, or with neither store**, `boomdrive enroll` is refused before any request, naming what the host lacks, and exits 2. On Linux that is `systemd-creds` missing, or older than 256 by its own `--version`. Nothing falls back to a file (core.32).

The store is read again at every connection attempt, never cached across one. A partition enrolled, or enrolled again, while the runner runs is dialed with its new key at its next attempt, with no restart. A key that cannot be read is said once, naming the partition and the store's words, and is tried again at the backoff's cap (Decision 6). A host with neither store opens the hosted implementation all the same: each partition's key is then unread, with words naming what the host lacks, so each partition says so once and the runner keeps its repositories.

The key is named by the `url`'s host as well as the label, so a host moved from one control plane to another, a staging one and then production, keeps a key for each and never presents one to the other.

In tests, the store is a trait with an in-memory store. `systemd-creds` is a stub program the test writes, as Flume's and sluice's are. The macOS keychain is never touched by a test (Decision 14).

Alternatives:

- **`LoadCredentialEncrypted=` in the service unit**, which F10 names. Not taken: the unit is the install's (record 0012), and would have to name each partition's credential. A partition enrolled later would also wait for a restart. Decrypting through `systemd-creds` uses the same store and the same binding, by the tool's own command line.
- **Secret Service through keyring on Linux.** Not taken: a headless host has none (F10).
- **A P-256 key.** Not taken: Ed25519 has one encoding, no curve parameters to get wrong, and is verified by WebCrypto in the cloud and in the browser that will check a terminal's share.
- **A file readable only by the user.** Ruled out by core.32.

### 4. Enrollment: `boomdrive enroll`, one request, the token read once

```
boomdrive enroll [--config <file>] [--json] <label>
```

1. **The configuration** is read as `run` reads it. It must name a control plane whose implementation enrolls, and a partition `<label>` in its table. Otherwise it is refused, naming what is missing, and exits 2.
2. **The store** is found (Decision 3), or the command is refused, exit 2. Nothing has been sent.
3. **The token** is one line read from standard input, never an argument or the environment (core.31). Where standard input is a terminal, it asks on standard error and reads without echo. An empty token is refused, exit 2.
4. **The key** is made in memory.
5. **The request.** It is one HTTPS `POST` to `<url>/runner/enroll`:
   - `Authorization: Bearer <token>`;
   - `Content-Type: application/json`;
   - a body that is a version-1 message, `{"protocol_version":1,"id":…,"body":{"enroll":{…}}}`.

   `enroll` holds:
   - `key`, the public key, 32 bytes in base64url;
   - `signature`, by that key;
   - `machine`, the host's name as the operating system gives it, which FR3 shows.

   The signature is over the UTF-8 bytes of `boomdrive.v1 enroll`, then, each after a NUL byte: the `url`'s origin, lowercased; `sha256:` and the hex digest of the token's bytes; and the key's bytes. It binds the key to this control plane and this token, so a key is never registered by someone who does not hold it, and a signature is never replayed with another token.
6. **The answer** is a version-1 message, checked against `to-runner.schema.json` before it is read (Decision 10):
   - `200` with `enrolled`, holding `host`, the control plane's id for the host;
   - `401` or `403` with `enroll_refused`, holding `enroll`, the request's message id, and `words`, such as that the token has lapsed or was spent;
   - `400` with `message_refused`, the request having failed the control plane's check.

   Any other status, an answer that fails its check, and a request that could not be made are refused with words naming what came back. None of them shows the token.
7. **The key is kept** in the store only once `enrolled` has come, replacing the partition's key if one was there. A refused or failed enrollment leaves the store as it was. A write that fails after `enrolled` is refused, exit 2, saying the control plane holds a key this host does not, and that a new token enrolls it again.
8. **The answer it gives**, by Flume's output contract (core.39):
   - in words, `<label>: enrolled as host <host>`;
   - with `--json`, `{"enrolled":{"partition":"<label>","host":"<host>"}}`;
   - a refusal, `{"refused":{"refusal":{"reason":"<words>"}}}` on standard output with `--json`, and the words on standard error without it;
   - exit 0 enrolled; 1 refused by the control plane or unreachable; 2 for a usage error or a host that must be fixed, the configuration, the store or the token.

A request sent again with the same message id and the same key, after an answer was lost, is answered `enrolled` again by the control plane rather than refused as a spent token (cloud core.38). `boomdrive enroll` itself makes the request once. A person who lost the answer runs it again with the same token, which makes a new key and a new id, and is refused as spent. The cloud's 1.5 chooses whether a spent token's refusal names that.

`boomdrive enroll` writes no configuration (record 0006) and nothing under the data directory (core.35). It takes no lock, so it may run beside a running runner, which dials the partition at its next attempt (Decision 3).

Alternatives:

- **Enrollment over the WebSocket**, as the first message of a socket opened with the token. Not taken: it opens a socket for one exchange, and the cloud would accept it before it holds the host. A request answered once is the exchange the cloud's 1.7 names.
- **The token placed by the installer and exchanged at the first connection.** Not taken in record 0006, which chose `boomdrive enroll`.
- **The token in the body.** Not taken: version 1 names no credential (1.16's Decision 14), and a header is where a bearer goes.

### 5. Every connection proves the key is held

Each partition's socket is a WebSocket upgrade of `GET <url>/runner/connect`, over TLS (`wss`), with:

- `Sec-WebSocket-Protocol: boomdrive.v1`;
- `Authorization: BoomdriveHost <key>.<time>.<nonce>.<signature>`, where:
  - `<key>` is the partition's public key in base64url;
  - `<time>` is the runner's clock in Unix seconds;
  - `<nonce>` is 16 random bytes in base64url;
  - `<signature>` is in base64url, over the UTF-8 bytes of `boomdrive.v1 connect`, then, each after a NUL byte: the `url`'s origin, lowercased; the key's bytes; `<time>` as decimal digits; and the nonce's bytes.

A new statement is signed for every attempt, and none is kept.

The control plane takes the upgrade only where:

- the key is one it enrolled and has not revoked;
- the signature checks;
- `<time>` is within 300 seconds of its own clock;
- it has not seen `<nonce>` with that key within that window.

It finds the enrollment, the Host object, by the key itself. So the runner keeps nothing of enrollment but the key: no host id, no certificate, no file (core.35). Refusals of the upgrade:

- **`401` or `403`**: the control plane does not take this host's key. The runner says one line, naming the partition and the control plane's words, and that `boomdrive enroll <label>` enrolls it again. It tries again at the backoff's cap (Decision 6), reading the store afresh each time.
- **A subprotocol it does not accept**: the version is not one it speaks. The runner says one line naming `boomdrive.v1`, and tries again at the cap.

That statement is the host's token as the cloud's core.21 has it, proved before the socket reaches the host's Host object. It is no bearer kept as the host's identity (core.31): it is good for five minutes, once.

Alternatives:

- **A challenge answered on the socket.** Not taken: the Host object would hold a socket not yet proved (cloud core.21).
- **A JWT signed with EdDSA.** Not taken: one signature form, a context then NUL-separated fields, serves the enrollment, the connection and the terminal's share alike, and there is no header naming an algorithm to get wrong.
- **TLS client certificates.** Not taken: the cloud would need a client CA of its own.
- **A certificate issued at enrollment and shown at each connection.** Not needed to prove the key, which the cloud holds. The certificate a terminal's share carries is the terminal change's to fetch (*What this change leaves out*).

### 6. One connection per partition, kept by pings and tried again on a backoff

Each partition has one socket, opened by tokio-tungstenite over rustls, which verifies the control plane's certificate against the host's own roots, as `rustls-native-certs` loads them (record 0008; F49):

- **Opened only with a hello.** The first message at every connection is the partition's whole hello (core.21), so a connection is first attempted once the first pull has handed one (Decision 7). A hello handed that differs from the one sent goes at once. One handed while another waits to go replaces it, since a hello is whole.
- **Pings.** The runner sends a WebSocket ping every 30 seconds. A connection whose pong has not come by the next ping is counted lost. No application message is a heartbeat (core.21). The cloud's Host object answers pings without waking (its core.50, F12).
- **A backoff.** A connection lost or refused is tried again after a wait drawn at random, in whole milliseconds, from above zero up to a cap. The cap starts at one second, doubles at each failed attempt, and stops at `poll_interval` (core.22; full jitter). It goes back to one second once a connection has taken its first `pulled`. A refusal waits the cap's stop, `poll_interval`: a partition with no key, or whose key could not be read or was refused, or whose control plane does not speak the subprotocol.
- **The reach.** A partition says one line when its connection is lost or cannot be made, `<label>: the control plane is unreachable: <why>`, and one when a connection takes its first `pulled` again, `<label>: the control plane is reached`. Nothing is said for each failed attempt in between (core.23). The first connection after start says it is reached.
- **A refusal said once.** A refusal is the host's to fix, so the line is said again, once, when a refusal first comes in an outage already said: a key revoked while the socket was up is said, and not again while it lasts (Decisions 3 and 5).
- **Closed** with a normal close when the implementation is dropped, at the runner's stop and at the end of `run --once`.

Each line goes through the `say` the opener was handed, which the runner writes on standard error.

Alternatives:

- **A fixed retry at `poll_interval`.** Not taken: a laptop that wakes would wait up to a minute to dial, and core.22 asks for a backoff with jitter.
- **backon's exponential backoff with its jitter.** Not taken: its jitter adds up to the whole delay on top of it, so a wait at the cap could reach twice `poll_interval` (core.22). The doubling cap and its draw are a few lines of the connection's own.

### 7. The interface over the socket

The interface's two calls map onto version 1's messages, in 1.16's order (its Decision 5):

- **`pull(hello, report)`**:
  1. For each partition, it hands the connection the partition's hello (Decision 8). Where it differs from the hello last sent on a live connection, the connection sends it as `hello`.
  2. For each partition whose connection has taken a `pulled` since its hello, it takes that partition's wants from the last `pulled`.
  3. For a partition whose connection is being attempted, it waits up to 10 seconds for the attempt's first `pulled`. A partition in its backoff's wait is not waited for.
  4. It gives the wants of every partition read, in the table's order of partitions and each in its control plane's order, under the partition's ids (Decision 8).
  5. It is `Unread` only when no partition was read, naming each with why.
  6. Its `report` is each read partition's report as its control plane holds it: the partition's part of the last report the control plane answered with `reported`, as the runner handed it, but what it let go or refused; or, before any, the report the control plane gave at the connection's first `pulled`, where it gave one (*A report taken back*).
  7. Its `terminals` are none (Decision 11).
- **`report(report)`**:
  1. For each partition with a live connection, it sends that partition's part of the report, cut to version 1 (Decision 9), as `report` under a new message id.
  2. It waits up to 10 seconds for each `reported` naming its message.
  3. It gives `Reported` with every answered partition's `let_go` and `refused`, under the partition's ids, and those of any `reported` that came late since the last call.
  4. It is `Unsent` only when no partition answered. A partition that did not answer is named in a line.
  5. Each partition's part that `reported` answered is what its control plane now holds, which the next pull gives (the pull's step 6).
- **A report not answered** is held by its connection, with its message id. After a reconnect, the connection sends the hello, then that report again under the same id (core.21). It is dropped once a newer report is sent, since a report is whole. A `report` the control plane refused with `message_refused` is not sent again (1.16's Decision 5). The pass's next report goes.
- **The wake.** A connection that takes a `pulled` calls the `Wake` its opener was handed, so the runner makes a control pass at once (the running spec). The first `pulled` of each connection wakes it too.
- **Message ids** are UUIDs of version 7, time-ordered and random, so no id is given twice on an enrollment, across reconnects and restarts (cloud core.38). An id is kept only on a resend.

Every report renews the leases it names (cloud record 0009), so the pass reports at every pass, changed or not, as it does for crew.

**A report taken back.** The control plane holds each status as its copy, not as Flume's answer. So a report taken back from a `pulled`, used only where the runner's report file is missing (the archived Decision 11), gives each launch's lease and answer, each act, command and inbox, and no launch's status or entry. The pass then reads each place from Flume again, as it does for a status it has not carried. The runner's report file never holds a copy in place of Flume's answer.

Alternatives:

- **`report` returning as soon as it has sent**, with `reported` read at the next call. Not taken: a refused lease would then be acted on for one pass more (core.10).
- **A pull that waits for every partition.** Not taken: one unreachable partition would hold every other's work (core.26's partitions are apart), as crew's implementation lets one partition's state repository fail alone.
- **A pull's report only as the control plane gave it at the connection's first `pulled`.** Not taken: a live connection takes no `pulled` with a report again, so a report answered since would not come back at the next pull, as the shared case "a report comes back" asks of every implementation.

### 8. Each partition's own ids, repositories and leases

A partition's connection carries only that partition's own (core.26; record 0006):

- **Ids.** Every id a partition's control plane gives is handed to the runner as `<label>/<id>`. Each id is prefixed alike, and every id going up has its prefix taken off. Prefixed ids are:
  - an offer's, an act's, a command's and an inbox's `id`;
  - an act's `launch`;
  - a launch's `after`;
  - the keys of a report taken back, and an acted's `launch`;
  - `Reported`'s `let_go` and `refused`.

  So two control planes' ids never meet in the runner's report, and a launch's mark is `boomdrive:<label>/<id>`. The mark names the launch, not its lease: a launch offered again under a new number finds its place by the mark (the archived Decision 11), and the lease number goes up with every report (cloud record 0009).
- **The hello.** Each partition's hello is the runner's hello with:
  - `repositories` cut to those the partition's table names, in the hello's order (ledger 3.b: the repositories kept, which register nothing);
  - `leases` cut to the partition's own ids, their prefixes taken off.

  Its tool versions and its capacity are the host's, as Flume gives them, for every partition (core.26 "Not ruled out").
- **The report.** Each partition is sent only the entries whose ids carry its prefix. An entry with no partition's prefix goes to none.
- **A want for another's repository.** An offer, a `branch` command or an inbox naming a repository the partition's table does not name is not given to the runner. One line names the want's id, the partition and the repository, once for as long as it is offered. The control plane may give it to a host that keeps it for that partition.

Alternative: ids passed as the control plane gives them. Not taken: two Accounts' control planes on one host may each give the id `l-1`. crew's implementation prefixes its partitions' ids the same way.

### 9. What goes up is cut to version 1's copies and checked before it goes

The interface's report holds each status and entry as the tool printed it, and its hello holds Flume's host part as texts. crew takes both whole, and the runner's report file keeps them so (core.27 "Not ruled out"). The hosted implementation cuts each to its copy as it builds the message:

- **A place's status**: `status.places[0]` of the carried answer, cut by `place-status.schema.json#/$defs/place_status`.
- **sluice's entry**: the carried object, cut by `submission-entry.schema.json`'s definition.
- **The host part**: `running`, `bound`, `room` and `streams` read from their texts, cut by the host part's definition in `hello.schema.json`.

**The cut walks the version's own schema**, so the schema is the one list of what goes up (core.42), and the tests' lists check it (1.16's Decision 14):

- an object keeps only the properties its definition names, each cut by its own definition;
- an array cuts each item;
- a `oneOf` cuts by the one branch whose form the value takes: a constant by its value, a one-property object by its key, `null` by its type;
- a `$ref` is followed within the version's files;
- a value carried whole, such as a made thing or a sluice row's `flaky`, and every other value, is kept as the tool printed it, never written again.

A value that fits no branch is not cut, and fails the check below.

**Each copy is checked** against its definition before it goes:

- A copy that passes is carried, with the interface's `at` and `listed`.
- A launch's status or entry that fails goes up as `null`. A host part that fails goes up as `capacity` `missing`, with words naming the path.
- The runner says one line naming the launch, or the host part, and the schema's path that failed. It says no value, since a value may be text the cloud must not hold (core.11). The line comes once for as long as that copy fails.

So a Flume that adds a value to one of its sets fails one launch's status, not the report that renews every lease.

**Every message the runner sends** is checked whole against `from-runner.schema.json` before it is sent. One that fails is a fault of the runner's: it is not sent, a line names its kind and the path, and the call gives `Unsent`, or `Unread` for a hello. A hello or a report is checked first against its body's own definition, so the line names the path deep in it, such as `/body/report/launches/l-1/lease/number`, where the envelope's `oneOf` names `/body` alone.

The lines of a launch's copy start with the launch's id, `<label>/<id>`, and the host part's with `hosted:`, being no partition's.

Because every object of version 1 is closed, it is the check that holds core.27 and the cloud's core.108, at both ends. The cut only makes the copy that passes.

Alternatives:

- **A list of kept paths in the code.** Not taken: it would be a third list beside the schema and the tests', free to drift from them.
- **The last copy that passed, in place of one that fails.** Not taken: the page would show an old state as the place's present one. `null` says the copy could not be made.
- **The report refused whole when one copy fails.** Not taken: every lease it renews would lapse.

### 10. What the runner is sent is checked before it is read

The implementation builds validators once, from version 1's `FILES` alone, each under its `$id`, with nothing fetched (core.42). `jsonschema` becomes a dependency of the crate's own, still with no default features.

- **Each message** a connection receives is checked against `to-runner.schema.json` before it is read into a type.
  - One that fails is answered with `message_refused`, naming its id, with words naming the schema's path that failed and never a value, and nothing else is done with it. One line names its id and the paths.
  - A message whose id cannot be read, being no JSON object, a binary frame or holding no valid `id`, cannot be named. The connection closes with WebSocket code 1007 and is tried again on the backoff.
  - An `enrolled` or `enroll_refused` on the socket passes the schema, which checks a message's form and not its path. The runner refuses it with `message_refused`, its words saying an enrollment's answer is taken on `runner/enroll` alone, as a control plane refuses an `enroll` on the socket.
- **A `message_refused` from the control plane** names one of the runner's messages. The runner says one line naming the message's kind and the control plane's words. A refused `report` counts as unanswered, and is not sent again.
- **`enrolled`, `enroll_refused` and `message_refused`** answering an enrollment are checked against the same schema (Decision 4).

### 11. A terminal is closed at once

Record 0004 takes the relay out until Flume relays a terminal, and has the runner close one at once, with words saying the host's Flume relays none. core.30 lets a terminal leave the host only sealed, by a suite the two ends agree, and no suite is chosen yet (1.16's Decision 10).

So the hosted implementation answers every `terminal_open` with `terminal_closed`, its words saying this host's Flume relays no terminal, and makes no key. A `terminal_input`, `terminal_resume` or `terminal_close` naming a terminal the runner never opened is dropped. `Pulled::terminals` is always empty from this implementation, so the runner's herdr relay, which core.29 and `_open.md` name, is never reached through a hosted control plane.

Record 0004's consequences have the runner's half of the exchange built with the hosted implementation. That half needs Flume's relay command and the browser's half, the cloud's 5.5, and wave 5 brings both. So it is the change `a-terminal-encrypted-end-to-end` (the constitution interview's eighth). Until then, closing every terminal is record 0004's own decision.

### 12. Version 1 gains enrollment

No release of the runner speaks version 1, so it changes in place (core.42; 1.16's Decision 12):

| kind | sent by | holds |
|---|---|---|
| `enroll` | the runner, as the body of `POST runner/enroll` | `key`, `signature`, `machine` |
| `enrolled` | the control plane, answering `enroll` | `host`, its id for the host |
| `enroll_refused` | the control plane, answering `enroll` | `enroll`, the `enroll` message's id; `words` |

- **A new file, `enrollment.schema.json`**, holds the three definitions. `key` and `signature` are bytes in base64url. `key` is 43 characters, 32 bytes, and `signature` 86, 64 bytes. `machine` is 1 to 255 characters, `host` an id, and `words` text. Every object is closed.
- **`from-runner.schema.json`** takes `enroll`, and **`to-runner.schema.json`** takes `enrolled` and `enroll_refused`. The cloud refuses `enroll` on the socket and the hello on the enrollment path, by path, as the README says. The schema checks a message's form, not its transport.
- **`FILES`** gains the file. `FromRunner` gains `Enroll`, and `ToRunner` gains `Enrolled` and `EnrollRefused`. Each kind gets examples.
- **The README** gains:
  - *The paths*: `runner/enroll` and `runner/connect` under the control plane's `url`, the subprotocol, and which kinds go on each.
  - *Enrollment*: Decision 4's request, its signature's context and its answers.
  - *The connection's proof*: Decision 5's header, its signature's context, and the control plane's window and nonce.
  - Its sentence that version 1 has no message for enrollment or the proof becomes one naming what it still lacks: a forge token, health, the runner's own commands, and a terminal's certificate.
- **No name of a grant or a credential.** `enroll` names no token: the token is the request's header. 1.16's test of names holds.

The cloud's 1.6 pins this change's landing, so its check takes the new kinds.

### 13. The control pass under leases that lapse

A hosted lease expires (cloud core.39). Three rules crew's leases never tested now bite, and the runner's own code is brought to them. Each is the runner's, for every implementation.

- **core.22, the timer.**
  - `next_due` hastens the next pass by the midpoint to the earliest expiry only for a lease not yet lapsed, and only after a pass that reported. While the control plane cannot be reached, a pass sooner renews nothing.
  - A lapsed lease, or a pass that did not report, leaves the next pass at `poll_interval` after the one before began, or sooner when the control plane wakes the runner. A connection that returns wakes it (Decision 7), so a returning host reports at once.
- **core.23, the line.** The pass says `boomdrive: the control plane could not be read: <words>` once when a pull first fails, and `boomdrive: the control plane is read again` once when one next succeeds, and nothing at the passes between. The hosted implementation's own lines are per partition (Decision 6).
- **core.24, work under a lapsed lease.** By the runner's clock at the pass:
  - an offer whose lease has expired is not launched, held or refused. It is left unanswered, as one with no room is;
  - an act whose launch's lease in the report has expired is not carried out;
  - a command whose lease has expired is not taken.

  Each waits until a pull gives it again under a live lease. A launch already answered keeps its answer, and its place runs on in Flume (core.10).

The interview's `the-runner-keeps-time-and-backs-off` keeps core.35's briefs and core.40's times.

### 14. The tests

Every test stays inside the gate. It reaches no network beyond the loopback address, touches no keychain, credential or configuration of the user's, and runs no real Flume, sluice or `systemd-creds`.

- **A fake control plane**, in `crates/boomdrive-hosted/tests/common/`, on the loopback address:
  - it accepts the upgrade on `runner/connect` with tokio-tungstenite's handshake, and answers `runner/enroll` with HTTP written by hand;
  - it checks every message the runner sends against `from-runner.schema.json`, and verifies the enrollment's and the connection's signatures over their contexts;
  - it holds wants a test sets, answers `hello` with `pulled` and each `report` with `reported`, and records what it took;
  - it holds each partition's last report as a control plane does, but what it lets go or refuses, and gives it in each `pulled` answering a hello;
  - a test can make it refuse a key, a subprotocol or a token, drop a connection, withhold a `reported`, send a message that fails its check, and open a terminal.

  It is test code. No crate's `src` names a listener, and the lock names no HTTP or WebSocket server crate (the running spec's "No listener in the code").
- **The shared cases**, `boomdrive-control`'s, run over the hosted implementation against the fake, as they run over the fake control plane and crew's.
- **Each requirement of the hosted-control-plane spec** has its test over the fake: the table's refusals; enrollment and its refusals; the key's stores; the proof; pings and the backoff; the reach; partitions and their ids; the cut and the check; the checks both ways; terminals closed; the wake.
  - The backoff is tested with tokio's paused clock: no two attempts back to back, none waiting past `poll_interval`.
  - A test on the paused clock holds it still while a connection is live, by a blocking task, which stops tokio moving the clock on as the runtime idles, and moves it by `advance`. Otherwise the clock runs through the pings while a message crosses the loopback in the host's own time. The fake's `until` waits without blocking the test's runtime.
  - The cut is tested over Flume 0.4.1's fixtures in `crates/boomdrive/tests/fixtures/`: each fixture's place cut passes the schema and holds no field 1.16's refusals name.
  - The Linux store is tested with a stub `systemd-creds`. The macOS store is tested with keyring's own mock store.
- **The binary's own**, in `crates/boomdrive/tests/`, each with no server listening:
  - `type = "hosted"` opens;
  - `boomdrive enroll` is refused for crew's control plane, for a label the table lacks, and for an empty token;
  - an enrollment whose control plane is not listening is refused with exit 1, writing no key;
  - `run --once` with a hosted control plane that cannot be reached keeps the repositories, names the control plane unread and exits 1;
  - the timer and the line of Decision 13 over the fake control plane of `boomdrive-control`;
  - a lapsed lease's offer, act and command, over the same fake.

  The binary's tests use a loopback port with nothing on it, an empty data directory and `HOME` of the test's own, and a stub `systemd-creds` first on `PATH` that keeps no credential, so no keychain or credential is reached: on macOS a `HOME` of the test's own holds no default keychain, so a key read there fails before any dial, and an enrollment that is refused writes no key. The in-process tests of `planes.rs` open the hosted implementation only with a table it refuses, before it looks for the host's store.
- **The rules tests** hold the binary to depending on `boomdrive-hosted` and naming it only in `planes.rs` among its sources. No file outside the crate is a JSON Schema, and none of `boomdrive-control` or `boomdrive-crew` names it (control-plane-interface spec).

### Rules relied on

- **core.1**: the pass decides nothing new. A want for another partition's repository is withheld by the implementation, never refused in the runner's name (Decision 8).
- **core.2**: Decision 1. Only `planes.rs` names the hosted implementation, its enrollment among it.
- **core.3**: Decisions 4 to 6. Every connection is outgoing, and no crate's sources hold a listener.
- **core.4**: Decisions 2 to 5. No token in the table, the token read from standard input, never shown, and no key in a file.
- **core.5**: Decision 2. The top-level `repositories` is what is kept.
- **core.9** and **core.10**: Decision 7's renewal at every pass, and Decision 13's lapsed leases.
- **core.11** and **core.27**: Decision 9. Only the copies go up, and a line names a path, never a value.
- **core.12**: `url`, `label`, `repositories`, `enroll`, `enrolled`, `enroll_refused`, `machine` and `host` each name what they hold.
- **core.17**: the table is written by hand, and the cloud sends none of it.
- **core.19**: one `url` for every partition.
- **core.21**: Decisions 6 and 7. Pings, the whole hello first, the unanswered report resent under its id.
- **core.22**, **core.23** and **core.24**: Decisions 6 and 13.
- **core.26**: Decision 8.
- **core.28**: what leaves is the protocol's two paths. `machine` is a name, and nothing else of the host goes.
- **core.29** and **core.30**: Decision 11.
- **core.31** and **core.32**: Decisions 3 to 5, their details amended in this change's commit (record 0018).
- **core.35**: nothing of the hosted implementation's is in the data directory.
- **core.36**: the report file is unchanged.
- **core.39**: `boomdrive enroll`'s `--json`, refusal and exits.
- **core.41**: neutral names in tests and examples. No control plane's real address is in the tree.
- **core.42**: Decisions 9, 10 and 12.

### Rules departed from

None. Two rules stay unmet by the code, each owed by a change of its own, and this change adds to neither:

- **core.29**: the herdr relay stands, reached by no hosted terminal (Decision 11).
- **core.40**: the hosted implementation's lines, like every line the running runner writes, carry no time yet. They go through the runner's one `say`, which the change that brings core.40 times.

## What this change leaves out

The constitution interview's list for this change was wider than the cloud's item 1.15. These are left to changes of their own, each adding its messages to version 1 while no release speaks it, or else making version 2 (core.42):

| what | its record | the change |
|---|---|---|
| the one `curl` install, the service units, `upgrade` and the nix flake (F51) | 0012 | `the-runner-installs-as-one-verified-release`, after `the-runner-opens-as-a-public-repository` |
| a terminal's key exchange, its suite and the certificate on the host's key its share carries | 0004 | `a-terminal-encrypted-end-to-end`, with the cloud's 5.5 |
| health in the protocol, and the opt-in OTLP export | 0013 | the change that builds health |
| forge tokens down the connection, and `boomdrive forge-token` | 0007 | the change that builds `forge-token` |
| drain, resume, upgrade and setup as commands, and a stray place's drop | 0008, 0009, 0010 | each command's change |

Until the install lands, a host is installed by hand: the runner built or fetched by its person, its configuration written by them, and `boomdrive enroll <label>` run with the token piped in. That is the walk's step 3 on the operator's own machine.

## What this change asks of others

- **boomdrive-cloud, 1.6.**
  - Move the pin of `crates/boomdrive-hosted/protocol/v1/` to this change's landing.
  - Check `enroll` at `runner/enroll`, and every message on `runner/connect`, against version 1.
- **boomdrive-cloud, 1.4 and 1.7.**
  - Take `BoomdriveHost` at the upgrade of `runner/connect`, by Decision 5's form and window. Find the enrollment by its key, and refuse an unknown or revoked key with `401` and words.
  - Refuse a subprotocol it does not accept at the upgrade.
  - Answer each `hello` with `pulled`, its `report` as held. Send `pulled` again when the wants change, and `reported` for each `report`, naming it.
  - Answer pings without waking the Host object (F12).
  - Take `runner/enroll`: the bearer token, the `enroll` signature over Decision 4's context, and the answer of `enrolled`, `enroll_refused` or `message_refused`. Answer an `enroll` sent again with the same id and key as it answered the first.
- **boomdrive-cloud, 1.5.**
  - The enrollment token: one machine, once, naming its Account (record 0047), with `enroll_refused` words for a lapsed or a spent one (FR7).
  - Until this repository's install lands, the page's install command is the runner's configuration and `boomdrive enroll <label>` with the token piped to it.
- **boomdrive-cloud, 1.9 and 1.10.** A lease's expiry well past `poll_interval` and the runner's waits: minutes, not seconds. A runner renews at every pass, and sooner by the midpoint to an expiry while it is reached (Decision 13).
- **boomdrive-cloud, 1.11.** A launch's status or entry may come up `null` where its copy failed the runner's check (Decision 9).
- **This repository's later changes**: those of *What this change leaves out*.

## Risks / Trade-offs

- **The first release fixes version 1.** Enrollment and the connection are in it. Health, forge tokens, the runner's commands and a terminal's certificate are not, and each becomes version 2 unless it lands before a release speaks version 1.
- **A macOS keychain item belongs to the program that made it.** A runner upgraded to a binary signed differently may be asked by the keychain whether it may read the key. The launchd agent cannot answer, and its partition says the key could not be read. The install change signs its releases. Until then, `boomdrive enroll` run again makes a new key the new binary owns.
- **Linux needs systemd 256 or later**, for user-scoped credentials (record 0006). An older host is refused at `enroll`.
- **The loop waits on the socket.** A pull may wait 10 seconds for a connecting partition, and a report 10 seconds for its `reported`. A keeping pass or a landing waits behind them, and a stop at that moment is as late.
- **A copy that fails goes up `null`.** The page loses that launch's status until a runner whose version 1, or version 2, names the new value.
- **A partition's label is in every id and mark.** Renaming a label orphans its work: its entries in the report go to no control plane, and its places stay as Flume holds them (core.10).
- **A report file lost while a partition is unreachable.** The pass that rebuilds the report from the pull takes back only the reachable partitions' parts. Launches are found again by their marks, but an act or a command of the unreachable partition, carried out since its last report, may be carried out again.
- **Clock skew past five minutes** refuses every connection. The line carries the control plane's words, which say so.
- **No end-to-end run of the binary over a live socket.** The binary's wiring is tested without a server, and the hosted implementation against its fake. They meet through the interface, which crew's implementation and the fake already hold to (core.2).
- **New dependencies**: tokio, tokio-tungstenite with rustls-native-certs, ed25519-dalek with getrandom and zeroize, ureq with rustls-platform-verifier, keyring-core and keyring v4's macOS store, url, uuid, base64, gethostname and rpassword, and `jsonschema` at run time. Each is pinned by `Cargo.lock` (core.8). The gate's sandbox reaches only the loopback address, so they must be in cargo's cache before it runs.

## Migration Plan

- **A host under crew**: nothing changes.
- **A host dialing the cloud**:
  1. Write `[control_plane]` with `type = "hosted"`, `[control_plane.hosted]` with its `url`, and a partition for each enrollment.
  2. Run `boomdrive enroll <label>` with each partition's token.
  3. Restart the runner's service.

  Removing `[control_plane]` stops the dialing, and the runner keeps its repositories as before.

## After deploy

Nothing. No step of this change is outside the gate. The cloud's ends of the two paths are its 1.5, 1.6 and 1.7.

## Task notes

**1.1** `next_due(began, ended, poll_interval, left, reported)`: `left` is the time to the earliest expiry of a lease not yet lapsed, and is taken only where `reported`. `Passed` already says whether the pass reported. `run.rs` passes it. Tests in `carry.rs`'s own module and `tests/leases.rs`: a lapsed lease, and an unreported pass, each leave the next pass at `poll_interval`. The work-into-flume scenario "Renewed before it lapses" still holds. Remove `_open.md`'s core.22 line.

**1.2** The control loop keeps whether the last pull was read. The pass says the unread line only on the change from read to unread, with the pull's words, and the read-again line on the change back. Tests in `tests/control.rs` over `boomdrive-control`'s fake: three unread passes say one line, and a read pass then says one. Remove `_open.md`'s core.23 line.

**1.3** In `carry.rs`:

- `offer`: an offer whose `lease.expires` is before the pass's `At::now()` is left before any other step, the "Settled" step aside, so an offer already answered keeps its answer.
- `act`: an act whose launch's lease in the report has an expiry before now is left untaken.
- `command`: a command whose lease has an expiry before now is left untaken.

The pass reads the time once, at its start. Tests in `tests/leases.rs` with leases expiring before and after the pass's start.

**2.1** `protocol/v1/enrollment.schema.json` with `$defs` `enroll`, `enrolled` and `enroll_refused`, as Decision 12 has them, each closed. The base64url pattern is `shared.schema.json`'s, with lengths. `from-runner.schema.json`'s body `oneOf` gains `enroll`, and `to-runner.schema.json`'s gains `enrolled` and `enroll_refused`. `FILES` gains the file.

`v1.rs` gains `Enroll { key, signature, machine }`, `Enrolled { host }` and `EnrollRefused { enroll, words }`, each `deny_unknown_fields`, and the three variants.

Examples: `from-runner/enroll.json`, `to-runner/enrolled.json`, `to-runner/enroll_refused-a-lapsed-token.json`, with neutral names such as `machine: "host-a"`.

`tests/refused.rs` gains:

- an `enroll` with no `signature`;
- an `enroll` whose `key` is not 32 bytes;
- an `enroll` holding a `token`;
- `enrolled` as a runner's message;
- `enroll` as a control plane's.

**2.2** The README's sections of Decision 12, in the voice of its others. Its paths are relative to the control plane's `url`. The signature contexts are written exactly as Decisions 4 and 5 give them, with one worked example of each context's bytes, as hex, made from fixed keys. Those bytes are a test's too (3.3), so the README and the code cannot differ.

**3.1** `table.rs`: `Table::read(table: &toml::Table, file: &Path) -> Result<Table, String>`, refusing as crew's `config.rs` refuses, naming the key and the file. The `url` is parsed with the `url` crate. Its origin, lowercased, is what the signatures name. `runner/enroll` and `runner/connect` are joined under its path, which gains a trailing `/` where it has none.

**3.2** `key.rs`:

- `trait Store { fn read(&self, name: &KeyName) -> Result<Option<SigningKey>, String>; fn write(&self, name: &KeyName, &SigningKey) -> Result<(), String>; }`, where `KeyName` is a label and the `url`'s host, which `Table::key_name` gives, and an `Arc` of a store is a store;
- `Keychain`, over a keyring-core store: `Keychain::login()`, keyring v4's macOS store, under `cfg(target_os = "macos")`, and `Keychain::over(store)` on every platform;
- `SystemdCreds::open(program, credstore_dir)`, refused below systemd 256 by the program's own `--version`, and `SystemdCreds::for_user()`, `systemd-creds` from `PATH` and the user's credential store;
- `Memory`, for tests;
- `store_for_host()`, giving the host's store or why there is none.

`SystemdCreds` and `Keychain::over` are built on every platform, so their tests run on any host, over a stub and over keyring's mock. `store_for_host()` alone chooses by the platform, and `Keychain::login()` is built only on macOS.

The key's bytes pass to `systemd-creds` on standard input and come back on standard output. They never go in an argument, and are never written to a file of the runner's. `SigningKey` is zeroized on drop (ed25519-dalek's `zeroize` feature). The name forms are Decision 3's.

**3.3** `enrollment.rs`: `enroll(table, label, token, store, machine) -> Result<Enrolled, Refusal>`, where `store` finds the host's store, and is called once the partition is found, so a store refused makes no request; `token` gives the token, or words saying why it could not be read, and is called once the store is found, so a terminal is asked for the token only after the configuration and the store (Decision 4's order); and `Refusal` holds the words and `host_to_fix`, whether the host must be fixed, which gives the exit. It says nothing beyond its answer, which the binary prints.

- The `POST` is made by ureq with rustls.
- The answer is checked by the `check` module before it is read.
- The signature context is a function shared with Decision 5's, each built from its fields by one `context(label: &str, fields: &[&[u8]])`.

`boomdrive_hosted::enroll(table: &toml::Table, file: &Path, label: &str, token: impl FnOnce() -> Result<String, String>)` is what `planes.rs` lists. It finds the store by `store_for_host()`, and the machine's name by gethostname.

A test builds the enrollment's and the connection's contexts from the README's fixed key, token, origin, time and nonce, and compares them with the README's worked bytes, read from the README itself.

**3.4** `tests/common/fake.rs`: the fake hosted control plane.

- It is a `tokio::net::TcpListener` on `127.0.0.1:0`, in test code alone.
- A request for `runner/connect` is upgraded by `tokio_tungstenite::accept_hdr_async`, whose callback checks the subprotocol and the `BoomdriveHost` statement.
- A request for `runner/enroll` is read and answered as HTTP/1.1 written by hand: its request line, its headers to the blank line, and a body of `Content-Length` bytes.
- Its validators are built from `FILES`, as the implementation's are.
- Its handle lets a test:
  - set each partition's wants and the report it holds;
  - refuse a key, a subprotocol or a token;
  - drop a connection;
  - withhold or delay a `reported`;
  - send any message;
  - read what it took, in order.

`boomdrive:test`'s no-listener test reads every crate's `src` and the lock, so it stands; the fake's crates are tokio and tokio-tungstenite, neither a server crate of its list.

**4.1** `connection.rs`: `Connection::open(runtime, Dial)` spawns one task per partition on the runtime it is handed, `Dial::new(table, label, store, poll_interval, say, wake)` naming what it dials.

- Its state is the last hello handed, its phase (unhanded, attempting, live or waiting), the last `pulled` and the report held at the first, why it is unreached and whether it is reached. 5.2 adds the report held unanswered with its id, and the late `reported`s.
- `hand(hello)` refuses a hello that fails version 1, and `read(patience)` gives the wants of the last `pulled`, waiting only while an attempt is made. `close()` sends a normal close and gives the task to wait on.
- The upgrade is built as an `http::Request` with the two headers.
- Pings are `tokio::time::interval_at(now + 30s, 30s)`, the lost connection being one whose `Pong` has not come by the next tick.
- The backoff is the connection's own: the cap doubling from one second to `poll_interval`, and the wait drawn up to it by getrandom.
- The runtime is a multi-thread runtime of two workers, made by `open`. `Drop` sends each socket a close, waits at most a second, and shuts the runtime down.

**4.2** `check.rs`: one `Validator` for each direction's envelope, and one for each copy's definition, built once with `jsonschema::options()`, each file a resource under its `$id`, with no retriever. Errors give their `instance_path` and `schema_path`, never the instance.

**5.1** `copy.rs`: `cut(reference: &str, value: &Value) -> Value`, where `reference` is `<file>#<pointer>` among `FILES`, walking as Decision 9 has it. A `oneOf` branch is chosen by:

- `const` or `enum` against the value;
- against an object, every key the branch requires present and every `const` its properties name held;
- `type`, `null` among it, against the value.

A `$ref` is resolved among `FILES` by its file and its JSON pointer. A closed object keeps only what it names, and an open one, or a definition with no `properties`, keeps the rest whole. A value that fits no branch is kept whole, so the check names its path. `place_status(text)`, `submission_entry(text)` and `host_part(&HostPart)` give each copy cut and checked against its definition, or words naming the path. `check.rs` holds the three copies' validators, and the hello's and the report's bodies', beside the envelopes'.

**5.2** `plane.rs`: `Hosted` and `open`, as Decision 1 gives the opener's arguments with `poll_interval`. `open(table, file, poll_interval, say, wake)` finds the host's store, and `Hosted::new(table, store, poll_interval, say, wake)` takes one, as the tests do with a store in memory. The opener's `say` is one `Mutex` shared by every partition's connection.

- Pull and report as Decision 7.
- Ids as Decision 8, by `<label>/` and the first `/`, since a label holds none.
- Copies as Decision 9.
- Terminals as Decision 11.

**5.3** `tests/shared.rs` runs `boomdrive_control::shared`'s cases over `Hosted` against the fake, its harness making a fake control plane holding the given wants for one partition, and seeing the report the fake holds.

**6.1** `planes.rs`:

```rust
pub struct Entry { pub word: &'static str, pub open: Opener, pub enroll: Option<Enroller> }
pub const PLANES: [Entry; 2] = [ /* crew, hosted */ ];
pub fn enroll(word, table, file, label, token: impl FnOnce() -> Result<String, String>) -> Result<Enrolled, Refusal>;
```

`Opener` gains `poll_interval: Duration`. `run.rs`'s `Control::open` passes `table.poll_interval`. `Enroller` takes the token as a boxed `FnOnce`, which `planes::enroll` hands on uncalled: a word not carried, or one whose implementation enrolls nothing, is refused before the token is asked for, and the implementation asks for it once its partition and store are found. `Enrolled` and `Refusal` are the binary's own plain types, a host id and words with whether the host must be fixed, which the hosted crate's are mapped into in `planes.rs`.

**6.2** `src/enroll.rs` and `Command::Enroll { config, json, label }` in `main.rs`.

- It reads the configuration by `config::read`.
- It reads the token as Decision 4 has it, with rpassword where standard input is a terminal, and the first line of standard input otherwise.
- It calls `planes::enroll` and prints by core.39.

The binary's tests are in `tests/enroll.rs` (Decision 14).

**6.3** `crates/boomdrive/Cargo.toml` depends on `boomdrive-hosted`. In `tests/rules.rs`:

- `the_crates_depend_as_decision_1_has_them` gains `boomdrive`'s dependency on it;
- `only_the_hosted_implementations_crate_holds_the_protocol` reads `crates/boomdrive/src` but `planes.rs`, `crates/boomdrive-control` and `crates/boomdrive-crew` for the names, and every file under `crates/` outside the hosted crate for `.schema.json`. `boomdrive`'s `Cargo.toml` is the one manifest that names it.

Every task of `crates/boomdrive/moon.yml` that builds the binary, its format, lint, typecheck and test, takes `/crates/boomdrive-hosted/**/*` among its inputs, as the rules test `each_task_takes_the_sources_of_every_crate_it_builds` asks. `boomdrive-hosted/moon.yml`'s test task takes `/crates/boomdrive/tests/fixtures/**/*`, which its cut's tests read.

**6.4** README and CLAUDE.md: the hosted implementation built, its table, `boomdrive enroll`, and the control plane reached through an outgoing WebSocket now, no longer later.
