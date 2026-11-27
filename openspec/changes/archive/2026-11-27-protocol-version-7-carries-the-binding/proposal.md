# Proposal

## Why

The first-run walk of 2026-11-26 offered `flume/fix@1` on a repository that had no merge queue. Flume refused the launch two times: first because the repository had no `.config/sluice.toml`, then because its `.config/flume.toml` named no `merge_queue`. A control plane cannot see this before it offers a launch, because the hello of version 6 names each repository by its address alone.

Flume now reports each repository's binding without a launch (Flume v0.37.0, its change `flume-reports-a-repositorys-binding`). Flume answers a status ask that names repositories by their folders with `status.repositories`. For each repository, it gives the binding at the default branch, and whether the merge queue of the binding lands that branch. It also gives each kind that Flume can run there, whether the kind lands, and why the binding refuses it. The runner reads Flume only through flume host's one subscription (Boom: `the-runner-hears-flume-host-by-one-subscription`). So the runner asks that subscription for the repositories it keeps, and carries the report to its control plane.

A version that a release spoke never changes (core.42), so version 7 carries the report. The cloud leads the protocol (Boom: `the-cloud-leads-the-runners-protocol`), so this change writes version 7 and marks it not spoken. The runner offers version 7 only after a later change marks it spoken, when the cloud's `main` speaks it.

## What Changes

- **Version 7 names each repository's status.** `protocol/v7/` is version 6, but for one required field of the hello: `repository_statuses`. It names each repository of `repositories` once, by its address. Each value is `{"printed": …}`, the copy of Flume's item without its folder, or `{"missing": "<words>"}`.
- **The hello carries the report.** The runner sends the report in the hello, and in no message of its own. The hello already carries the host's facts about the repositories it keeps, and the runner sends it again whole when it changes (core.21). So a control plane receives a new report at each connection and at each change of a binding, with no new kind.
- **The subscription names the kept clones.** The subscription to flume host names the folder of each clone that a pass reads. A read that names other folders makes the subscription ask again. The first document of an ask wakes no part, because the read that made the ask waits for it. `boomdrive run --once` names the same folders in each status ask.
- **The interface's hello holds each repository's status.** `Hello.repository_statuses` holds Flume's item for each kept repository, as Flume printed it, or the words that say why there is none. The hosted implementation cuts it to version 7's copy, and only when version 7 is spoken. crew's implementation reads nothing of it.
- **The set takes Flume v0.37.0**, whose flume host gives the report. The lock moves the flume node alone (core.45).
- **The mark in `spoken.json` alone chooses the version.** `spoken.json` names version 7, not spoken. The types move to `protocol::v7` and read and write versions 5, 6 and 7. The crate's tests pass with version 6 or version 7 marked spoken.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `runner-protocol`: version 7, its `repository_statuses`, and the files and types of version 7.
- `hosted-control-plane`: the hello of version 7, each partition's share of the statuses, and a status whose copy fails.
- `running`: the subscription and the status ask name the folders of the kept clones.
- `work-into-flume`: the hello holds each kept repository's status, as Flume gave it.
- `control-plane-interface`: the hello's `repository_statuses`.

## Impact

- `crates/boomdrive-hosted/protocol/v7/`, `spoken.json` and the README.
- `crates/boomdrive-hosted/src/`: `protocol::V7`, `protocol::v7` with the types, `v6.rs` with its files alone, `copy::repository_status`, `check::repository_status`, and the plane's hello.
- `crates/boomdrive-control/src/lib.rs`: `Hello.repository_statuses` and `RepositoryStatus`.
- `crates/boomdrive/src/`: `Flume::status` names the folders it reads, the subscription asks again for other folders, and `hello::repository_statuses`.
- The tests of the three crates, and crew's tests' hello.
- `flake.nix` and `flake.lock`: Flume v0.37.0.
- `docs/architecture/core.md`, core.42's details: version 7.
- No record: the decision is Boom's.

## Touches

```
docs/architecture/core.md
flake.nix
flake.lock
crates/boomdrive-control/
crates/boomdrive-crew/tests/common/mod.rs
crates/boomdrive-hosted/protocol/spoken.json
crates/boomdrive-hosted/protocol/README.md
crates/boomdrive-hosted/protocol/v7/
crates/boomdrive-hosted/src/
crates/boomdrive-hosted/tests/
crates/boomdrive/src/
crates/boomdrive/tests/
```
