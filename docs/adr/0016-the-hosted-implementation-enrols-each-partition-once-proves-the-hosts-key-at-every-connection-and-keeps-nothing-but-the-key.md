---
number: 16
title: The hosted implementation enrols each partition once, proves the host's key at every connection, and keeps nothing but the key
status: proposed
date: 2026-11-03
decision-makers:
- Chuck Swanberg
---

# The hosted implementation enrols each partition once, proves the host's key at every connection, and keeps nothing but the key

Status: proposed on 2026-11-03 by the construct stage of the change `the-runner-dials-the-hosted-control-plane`, as the choices that change makes on rules already written; accepted when the user approves the change in its review.

Deciders: the construct stage, on the written rules; the user, at the change's review.
Sources: records 0004, 0006, 0008, 0010, 0012 and 0015; rules core.2, core.4, core.5, core.19, core.21, core.22, core.26, core.31, core.32, core.35 and core.42; agentplot/flywheel-cloud's records 0047, 0049, 0056 and 0059, its rules core.21, core.38 and core.50, its roadmap's item 1.15 and ruling D5, and its research rows F10, F49 and F51 (`design/research/awesome-lists-disposition.md`), cited by id; systemd's `systemd-creds(1)` and `systemd.exec(5)`, on user-scoped credentials and the user's credential store; the change's design, Decisions 1 to 13.

## Context and problem statement

Record 0006 makes a host's identity a key pair made on the host, kept only in its credential store, with one enrolment per partition. `flywheel enroll` reads the one-time token on standard input and registers the public key over the runner's own outgoing connection. core.31 has every connection prove the key is held. core.32 keeps the key in the macOS keychain or an encrypted systemd credential, never in a file in the clear. core.26 keeps each partition's repositories, leases and work to its own connection. core.35 keeps the data directory to what can be made again.

None of them says how enrolment and each connection prove the key, or which signature the key makes. Nor does any say how the runner reaches the key on Linux, what of enrolment the host keeps, or how the runner knows which repositories are a partition's.

## Considered options

For enrolment:

1. One HTTPS request to the control plane. The token is its bearer, and its body is a version-1 message, `enrol`, holding the public key, a signature by it, and the machine's name.
2. The first message of a WebSocket opened with the token.
3. The token placed by the installer, and exchanged at the first connection.

For the proof at each connection:

1. A statement of the connection's time and a nonce, signed by the key, in the upgrade's `Authorization` header. The control plane checks it within a window, and finds the enrolment by the key itself.
2. A challenge sent on the socket and answered by a signature.
3. A JWT signed with EdDSA.
4. TLS client certificates.

For the key: Ed25519; or P-256.

For the key on Linux:

1. Encrypted by `systemd-creds encrypt --user` into the user's credential store, and decrypted by the runner through `systemd-creds decrypt --user` at each connection.
2. Loaded by the service unit's `LoadCredentialEncrypted=`.
3. Secret Service through keyring.

For a partition's repositories:

1. Named in each partition's own table, from those the configuration keeps.
2. Every kept repository to every partition.

## Decision outcome

Option 1 each time: one request, a signed statement at the upgrade, Ed25519, `systemd-creds` read by the runner, and each partition naming its repositories. core.26, core.31 and core.32 hold it, their details amended in this record's commit.

- **One request enrols a partition.** `flywheel enroll <label>` makes the key in memory, and sends `POST runner/enrol` under the control plane's `url`, the token as its bearer. The `enrol` message's signature is over a fixed context, the control plane's origin, the token's digest and the key. The key is kept only once the control plane answers `enrolled`. Option 2 opens a socket for one exchange, before any Host object exists. Option 3 is not what record 0006 chose.
- **Every connection is a signed statement.** The upgrade of `runner/connect` carries the key, the runner's time, a nonce and a signature over a fixed context. The control plane takes it within five minutes of its own clock, and only once for each nonce. It finds the enrolment by the key, so the host keeps no host id, certificate or file. The statement is good once, for minutes, and is no kept bearer (core.31). Option 2 lets the Host object hold a socket not yet proved, where the cloud's one seam proves a credential first (cloud core.21). Option 3 adds a header naming an algorithm, where one signature form, a context then NUL-separated fields, serves enrolment, the connection and a terminal's share. Option 4 needs a client certificate authority at the cloud's edge.
- **Ed25519.** It has one encoding and no curve parameters, and WebCrypto verifies it in the Worker and in the browser.
- **`systemd-creds`, read by the runner.** The credential is bound to the host and the user, and to the host's TPM where it has one. Decrypting it through the tool's own command line at each connection lets a partition enrolled while the runner runs be dialed with no restart. It also needs no service unit to name each partition's credential. Option 2 is F10's, and would tie every enrolment to the unit the install writes. Option 3 has no store on a headless host.
- **Each partition names its repositories.** `[[control_plane.hosted.partitions]]` holds a `label` and `repositories`. The hello of each partition names only its own, and a want for another partition's repository is not given to the runner. What the host keeps is still the configuration's top-level `repositories` (core.5). Option 2 is ruled out by core.26.
- **Each partition's ids are its own.** Every id a partition's control plane gives reaches the runner as `<label>/<id>`, so two control planes' ids never meet in the runner's report.

### Consequences

- Version 1 gains `enrol`, `enrolled` and `enrol_refused`, and its README names the two paths and both signature contexts, before a release speaks it (core.42).
- The cloud's change 1.7, with its 1.4's seam, verifies the statement and the enrolment's signature as the README states them. Its 1.5 issues the token the request bears.
- A key moved, a home directory copied, or a binary that the keychain does not know, gives the control plane no usable key. The partition is enrolled again with a new token.
- A Linux host needs systemd 256 or later (record 0006).
- A partition's label is part of every id and mark of its work, so renaming it leaves that work behind.
