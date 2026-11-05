# Proposal

## Why

The runner reaches its control plane through one interface, `ControlPlane`, and carries one implementation of it, crew's. The hosted control plane's is named and unbuilt: `type = "hosted"` is refused as a word this binary does not carry. `the-runner-keeps-its-protocol-as-one-json-schema-per-version` (landed as #4) wrote version 1 of the messages the two would speak, and the crate that holds them, `crates/boomdrive-hosted`, which implements nothing yet.

boomdrive-cloud's wave 1 is a walking skeleton: a member enrolls their own machine, asks a drive to launch a kind on one of its repositories, and watches the launch run on that host and come back live. The cloud's half is its changes 1.4 to 1.13. The host's half is this change, the cloud roadmap's item 1.15, 8 points, under ruling D5, with research notes F10, F49 and F51. Until it lands, no host can dial the cloud: no enrollment, no socket, no copy of a place's status goes up.

What it must hold to is already written:

- **The host's identity** is a key pair made on the host, kept only in its credential store, and proved at every connection (core.31, core.32; record 0006).
- **One enrollment per partition**, each with its own key, connection and Host object, and nothing of one partition's on another's connection (core.26; record 0006).
- **The socket** is outgoing, kept by the WebSocket protocol's own pings, tried again on a bounded backoff, and after every reconnect sends its whole hello and resends what was not acknowledged (core.3, core.21, core.22; record 0008).
- **What goes up** of a place's status is only the copy version 1 names, and every message the runner is sent is checked against its version first (core.27, core.42; boomdrive-cloud records 0049 and 0056).

Leases from a hosted control plane expire, where crew's never do. That makes three rules live that crew's leases let slide: a lapsed lease must not set control passes running back to back (core.22), an unreachable control plane is said once and not at every pass (core.23), and work under a lapsed lease waits for a live one (core.24). This change brings the control pass to all three.

## What Changes

- **`[control_plane]` `type = "hosted"` is carried.** `planes.rs` lists the hosted implementation beside crew's. Its table, `[control_plane.hosted]`, names the control plane's `url` and its `partitions`, each a `label` and the `repositories` that partition is served. It holds no token and no key (core.4).
- **`boomdrive enroll <label>`**, a new command (record 0006).
  - It reads the partition's one-time enrollment token from standard input.
  - It makes the partition's key pair, an Ed25519 key, and registers the public key with the control plane by one request.
  - It keeps the key in the host's credential store once the control plane has taken it: the macOS keychain, or a user-scoped encrypted systemd credential on Linux. It writes no file of its own and no configuration.
  - It answers in words or `--json` and exits 0, 1 or 2 (core.39).
- **One outgoing WebSocket per partition** (core.3, core.21, core.22, core.23).
  - It opens on version 1's subprotocol, with a statement of the connection's time and nonce, signed by the partition's key, in the upgrade's header (core.31).
  - Its first message is the whole hello.
  - It is kept by protocol pings, and tried again on a backoff with jitter from one second up to `poll_interval`.
  - It says one line when its control plane becomes unreachable, and one when it returns.
- **The interface over the socket.**
  - A pull gives each connected partition's wants from its last `pulled`, under ids prefixed with the partition's label.
  - A report goes to each partition as its own part of the runner's report, cut to version 1's copies, and is answered by `reported`. The last one no `reported` answered is resent under its own id after a reconnect.
  - A `pulled` that arrives wakes the runner for a control pass at once.
- **Each partition's own** (core.26). A partition's hello names only its own repositories and leases. A want naming another partition's repository is not given to the runner.
- **What goes up is cut and checked** (core.27, core.42; boomdrive-cloud core.108).
  - Each place's status, sluice's entry and the host part are cut to version 1's copies by walking the version's own schema.
  - Each copy is checked against the schema before it goes. A copy that fails goes up as `null`, or as `missing` for the host part, with one line naming the launch and the schema's path, never a value.
  - Every message the runner sends is checked against version 1, and every message it is sent is checked before it is read. A failing message is answered with `message_refused`.
- **Every `terminal_open` is closed at once**, with words saying the host's Flume relays no terminal (record 0004; core.30). No key is made for a terminal.
- **Version 1 gains enrollment**, while no release of the runner speaks it (core.42).
  - A new file, `enrollment.schema.json`, holds `enroll` from the runner, and `enrolled` and `enroll_refused` to it.
  - The protocol's README names the two paths under the control plane's `url`, `runner/enroll` and `runner/connect`, and the connection's signed statement.
- **The control pass**:
  - a lapsed lease, or a pass that could not report, sets no pass sooner than `poll_interval` (core.22);
  - the unread control plane is said once until it is read again (core.23);
  - an offer, an act or a command whose lease has lapsed by the runner's clock is not carried out (core.24).
- **Record 0017 and the details of core.26, core.31 and core.32**, in this change's commit beside its design: how enrollment and each connection prove the key, where the key is kept, and that each partition names its repositories.

## Capabilities

### New Capabilities

- `hosted-control-plane`: the hosted control plane's implementation of the interface. It covers:
  - its table;
  - enrollment and `boomdrive enroll`;
  - the host's key and its stores;
  - the connection, its proof, its pings and backoff, and the reach said;
  - the pull and the report over each partition's connection, with its own ids and repositories;
  - the copies cut and checked, and every message checked both ways;
  - the terminals closed;
  - the wake.

### Modified Capabilities

- `control-plane-interface`: `hosted` is carried and passes the shared cases; the binary depends on the hosted implementation's crate, named in `planes.rs` alone, as crew's is.
- `runner-protocol`: version 1 gains `enroll`, `enrolled` and `enroll_refused`.
- `running`: the control timer is hastened only by a live lease after a pass that reported, and an unread control plane is said once until it is read again.
- `work-into-flume`: work under a lapsed lease waits for a live one.

## Impact

- `docs/adr/` gains record 0017. `docs/architecture/core.md` amends the details of core.26, core.31 and core.32. Both are in this change's commit, beside its design.
- `crates/boomdrive-hosted` implements `ControlPlane`, and holds the table, the key's stores, enrollment, the connection, the cut and the checks.
  - Its dependencies gain tokio, tokio-tungstenite with rustls, ed25519-dalek, ureq with rustls, keyring-core and keyring v4's macOS store, url, uuid, base64 and gethostname.
  - `jsonschema` becomes a dependency of its own, still with no default features.
  - None of them serves HTTP or WebSockets (core.3).
- `crates/boomdrive-hosted/protocol/v1/` gains `enrollment.schema.json` and its examples; `from-runner.schema.json`, `to-runner.schema.json` and the README change.
- `crates/boomdrive`:
  - `planes.rs` lists `hosted` with its opener and its enrollment, and the opener is handed `poll_interval`;
  - `main.rs` and a new `enroll.rs` give `boomdrive enroll`, its token read without echo by rpassword where standard input is a terminal;
  - `carry.rs` and `run.rs` gain the timing, the reach line and the lapsed-lease checks;
  - `Cargo.toml` depends on `boomdrive-hosted`;
  - the rules tests and `moon.yml` follow.
- crew's implementation is unchanged: `planes.rs`'s opener for it ignores `poll_interval`.
- README and CLAUDE.md name the hosted implementation as built, and `boomdrive enroll`.
- Nothing changes for a host under crew. Its configuration, its report file and crew's records are as they were.
- boomdrive-cloud's 1.6 moves its pin of `crates/boomdrive-hosted/protocol/v1/` to this change's landing, and its 1.5 and 1.7 build the two paths' other ends.

## Touches

```
docs/adr/0017-the-hosted-implementation-enrolls-each-partition-once-proves-the-hosts-key-at-every-connection-and-keeps-nothing-but-the-key.md
docs/architecture/core.md
Cargo.toml
Cargo.lock
crates/boomdrive-hosted/**
crates/boomdrive/Cargo.toml
crates/boomdrive/moon.yml
crates/boomdrive/src/main.rs
crates/boomdrive/src/planes.rs
crates/boomdrive/src/run.rs
crates/boomdrive/src/carry.rs
crates/boomdrive/src/enroll.rs
crates/boomdrive/src/lib.rs
crates/boomdrive/tests/**
README.md
CLAUDE.md
```
