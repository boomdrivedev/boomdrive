# Proposal

## Why

The brief: "A runner reports a partition's work to an observer its configuration names at a loopback origin, enrolled with the host's key, sending its hello, which says it observes, and its reports of the places it follows with no lease, under a protocol version that carries them; it offers the observer no room and refuses and logs any lease, launch, act, command, preview or terminal from it, and an observer's reach holds up nothing of the control plane it takes work from."

Record 0048, accepted by the user on 2026-11-16, lets a runner report to an observer on its own host, which it takes no work from, and core.19 was amended with it. boomdrive-cloud's record 0069 builds the cloud's local preview on it: a local stage shows the host's real runner read-only, and its pin of the runner's protocol waits on the version that carries an observing hello. No version carries one, and the runner opens no connection but its control plane's.

## What Changes

- **Version 3 carries an observing connection.** Record 0052 is amended for it, and core.42's and core.10's details read with it. All three are written with this change. No release has spoken version 3, so it may still change (core.42).
  - The `hello` kind takes an observing hello: `observing`, `true`, with the hello's versions, tool versions and repositories, and no capacity or lease.
  - The `report` kind takes an observed report: `observing`, `true`, with each launch's answer, its status and its entry, and no lease.
  - Each form is closed, so no message passes as two of them.
  - Three examples are added: an observing hello, an observed report, and a `pulled` answering an observing hello.
- **The managed layer may name `[observer]`**: a `url` at a loopback origin, and `partitions`, each the `label` of a part of the runner's control plane whose work it observes, with the `repositories` it names. The hosted implementation reads it. It is refused where no `[control_plane]` is named, and a local layer that sets it is not taken.
- **`boomdrive enroll --observer <label>`** enrolls an observer's partition as a hosted partition is enrolled, whatever `[control_plane]`'s `type`. Its key is kept in the host's credential store under `observer.<label>`.
- **The interface names an `Observer`.** It takes a hello and a part's share of the report and returns nothing, so nothing it holds reaches the runner's work.
- **The hosted implementation gives an observer for each partition of `[observer]`.**
  - Each keeps one outgoing connection, proved as a partition's is.
  - It sends the observing hello, naming only its partition's repositories.
  - After each pass it sends an observed report of the part's launches, cut to version 3's copies.
  - It refuses every want, handed-back report and terminal message with `message_refused`, and says each in one line.
  - It wakes nothing, and it waits on nothing.
- **The runner opens its observers when it starts.** It gives each part the observer of its label, and hands it the pass's hello and share once the pass has sent its report. It says once an observer that observes no part. `run --once` hands none anything.
- **What the runner says names the observer**: README's hosted section, with an example of `[observer]`, and the protocol's README.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `runner-protocol`: version 3 carries an observing hello and an observed report, and is version 2 whole but for a hold and them.
- `hosted-control-plane`: the observer table, an observer's enrollment, what it is sent, what it refuses, its reach, and that nothing waits on it.
- `configuration`: the managed layer may name `[observer]`.
- `control-plane-interface`: an `Observer` takes a part's passes and gives nothing back.
- `running`: each part's observer is handed its passes, and no pass waits on it.

## Impact

- `docs/adr/0052-…md` gains its amendment of 2026-11-18. `docs/architecture/core.md` amends core.42's and core.10's details. Each is written with this change.
- `crates/boomdrive-hosted`:
  - `protocol/v3/`: `hello.schema.json`, `report.schema.json`, `from-runner.schema.json` and `shared.schema.json`'s `message_refused` description, and three examples;
  - `protocol/README.md`;
  - `src/protocol/v3.rs`, `src/check.rs`, `src/table.rs`, `src/key.rs`, `src/enrollment.rs`, `src/connection.rs`, `src/plane.rs` and `src/lib.rs`, and a new `src/observer.rs`;
  - `tests/`: `files.rs`, `interface.rs`, `refused.rs`, `examples.rs`, `enrollment.rs` and `key.rs`, a new `observer.rs`, and `common/`.
- `crates/boomdrive-control/src/lib.rs` and `src/fake.rs`: `Observer`, `Observed`, and a fake observer.
- `crates/boomdrive`:
  - `src/config.rs`, `src/planes.rs`, `src/run.rs`, `src/carry.rs`, `src/enroll.rs` and `src/main.rs`;
  - `tests/enroll.rs`, `tests/control.rs`, `tests/once.rs`, `tests/local_layer.rs`, `tests/examples.rs` and a new `tests/examples/observer.toml`.
- `README.md`: the hosted section.
- No crate, tool or tracked file type is added. No field of the report file changes, and no file under `protocol/v1/` or `protocol/v2/` changes. The flake's modules write `[observer]` through `services.boomdrive.settings`, whose freeform type takes it as the file names it (core.46).
- Outside this repository:
  - boomdrive-cloud takes version 3, the observing connection with the hold, by a change of its own (its record 0069);
  - a host that wants its runner observed names `[observer]` and enrolls it, which is the host's own.

  Each is named in design.md and told to the orchestrator.

## Touches

```
docs/adr/0052-the-runner-speaks-version-3-of-its-protocol-which-is-version-2-but-for-a-control-planes-hold.md
docs/architecture/core.md
README.md
crates/boomdrive-control/src/lib.rs
crates/boomdrive-control/src/fake.rs
crates/boomdrive-hosted/protocol/README.md
crates/boomdrive-hosted/protocol/v3/
crates/boomdrive-hosted/src/
crates/boomdrive-hosted/tests/
crates/boomdrive/src/config.rs
crates/boomdrive/src/planes.rs
crates/boomdrive/src/run.rs
crates/boomdrive/src/carry.rs
crates/boomdrive/src/enroll.rs
crates/boomdrive/src/main.rs
crates/boomdrive/tests/enroll.rs
crates/boomdrive/tests/control.rs
crates/boomdrive/tests/once.rs
crates/boomdrive/tests/local_layer.rs
crates/boomdrive/tests/examples.rs
crates/boomdrive/tests/examples/observer.toml
openspec/changes/a-runner-reports-to-an-observer-it-takes-no-work-from/
```
