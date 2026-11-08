# Proposal

## Why

`boomdrive run` makes every pass on one loop, one at a time. A control pass pulls every partition its control plane serves, answers every want, carries every place's status and reports to every partition, and a keeping pass runs on the same loop. So every drive's work waits in one line, behind every other drive's and behind the keeping pass.

On deneb on 2026-11-07, one drive's backlog of acts on places held another drive's launches 63 and 64 for a quarter of an hour, while that drive's stream had room for four agents and ran none. The user, that day: "We might want to improve Boomdrive's design so that each account partition operates independently unless there's some sort of host-level cap and synchronization that needs to be designed." Record 0026, accepted, decides it, and core.49 holds it. The chore `a-launch-waits-on-no-other-act` took the first step within one line (core.50). This change takes the rest.

The brief: "No partition the runner serves has its launches, acts or reports delayed by another partition's backlog, slow answer, refusal or failure; what partitions share on the host is bounded only by its owner's cap (Flume's agents and streams, sluice's runs) and a repository's one clone is fetched under a synchronization of its own." Record 0028 reads "drive" for the partition that is a body of work, and "account" for the one a runner enrolls.

## What Changes

- **The interface serves parts.** An implementation opens its control plane as parts, each a label and a `ControlPlane` of its own, and says by `owns` which ids are its. crew's implementation gives a part for each partition, a drive. The hosted implementation gives a part for each partition, an account's enrollment and its one connection. Each part has its own say and its own wake.
- **Each part runs on a thread of its own.** From the end of the first keeping pass, each part's control passes pull, answer, carry and report for that part alone. Each has its own timer, its own wake, its own `due` for core.50 and its own reach, said once each way and naming its label. No part waits on another part's pull, act, command or report, and no control pass waits on a keeping pass.
- **The loop keeps the host's own work.** It runs the keeping passes and the landings, and keeps a landing between two repositories of a keeping pass, not after the whole pass.
- **A clone's lock.** The keeper holds a repository's lock while it keeps or lets go of it, and a part holds it for a launch into the clone, which finds its clone again under the lock. Nothing else takes it.
- **One report file, each part's share.** `report.json` keeps its form and gains `parts`, the labels whose share it holds. Each part saves its share under the file's own lock, held for that write alone, keeping every other part's entries. A part the file does not hold takes its share from its pull's report.
- **Room counted per part.** Each part counts its own launches against the room its own listing gives. Where two parts take one room at once, Flume's bound holds.
- **crew's caches per drive.** Each partition reads its state through a bare cache of its own, so two drives of one state repository share no git directory and no weir lock. The shared cache of before is removed at open.
- **`run --once`** makes every part's control pass side by side, and succeeds where some part pulled and reported, as it does today.
- **Record 0026** gains an amendment holding these choices, and the details of core.49, core.9 and core.23 name them, in the commit that holds this change.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `running`: each part is carried on its own, on its own timer and wake, with its lines and its reach naming it. Stopping lets each part end its pass, and `run --once` carries every part side by side.
- `clones`: a clone's keep and a launch into it take turns on the clone's lock, and a landing is kept between two repositories of a keeping pass.
- `work-into-flume`: each part saves its share of the one report file, a part the file does not hold takes its share from its pull, and each part counts its own room.
- `control-plane-interface`: an implementation serves its control plane as parts, each saying which ids are its own, and is handed a say and a wake for each.
- `crew-control-plane`: each partition is a part with a cache of its own, and the shared cache of before is removed.
- `hosted-control-plane`: each partition is a part, its pull and report waiting on its own connection alone, and woken alone.

## Impact

- `docs/adr/0026-…`: an amendment of 2026-11-07. `docs/architecture/core.md`: the details of core.49, core.9 and core.23.
- `crates/boomdrive-control`: `src/lib.rs` (`Part`, `ControlPlane::owns`), `src/fake.rs` (`Fake::owns`, `Fake::owning`), `src/shared.rs` (the case `each_id_given_is_owned`), `tests/fake.rs`.
- `crates/boomdrive-crew`: `src/lib.rs` (`parts`, `Crew::owns`, each partition's cache, the shared cache removed), `tests/read.rs`, `tests/shared.rs`, `tests/common/mod.rs`.
- `crates/boomdrive-hosted`: `src/plane.rs` (`parts`, `Hosted::owns`, one connection per part on a shared runtime), `src/lib.rs`, `tests/plane.rs`, `tests/shared.rs`.
- `crates/boomdrive`:
  - `src/planes.rs`: the opener gives parts, handed a say and a wake for each;
  - `src/run.rs`: the loop, each part's thread, its timer and wake, the stop, `once`;
  - `src/carry.rs`: a pass over one part, its share, its lines, the launch under the clone's lock;
  - `src/report.rs`: each part's share of the file;
  - `src/locks.rs`, new: each clone's lock;
  - `src/pass.rs`: the keeper under each clone's lock, and the hook between repositories;
  - `src/lib.rs`: the module;
  - tests: `tests/control.rs`, `tests/loop.rs`, `tests/once.rs`, `tests/room.rs`, `tests/acts.rs`, `tests/offers.rs`, `tests/running.rs`, `tests/fakes/mod.rs`, `tests/pass.rs`, `tests/after.rs`, `tests/planes.rs`.
- `README.md`: each part runs on its own, and what its lines say.
- No crate, no tool, no tracked file type, no configuration key and no message of the protocol is added (core.7, core.8, core.42).
- Outside this repository: under the hosted control plane, the drives of one account share that account's connection, and protocol version 1 names no drive on a want, so the runner cannot run them apart (design Decision 8). Telling them apart is a new protocol version naming a want's drive, the hosted control plane's to design with boomdrive-cloud. Told to the orchestrator.

## Touches

```
docs/adr/0026-each-partition-the-runner-serves-runs-on-its-own-and-what-partitions-share-on-a-host-is-a-designed-cap.md
docs/architecture/core.md
crates/boomdrive-control/src/lib.rs
crates/boomdrive-control/src/fake.rs
crates/boomdrive-control/src/shared.rs
crates/boomdrive-control/tests/fake.rs
crates/boomdrive-crew/src/lib.rs
crates/boomdrive-crew/tests/read.rs
crates/boomdrive-crew/tests/shared.rs
crates/boomdrive-crew/tests/common/mod.rs
crates/boomdrive-hosted/src/lib.rs
crates/boomdrive-hosted/src/plane.rs
crates/boomdrive-hosted/tests/plane.rs
crates/boomdrive-hosted/tests/shared.rs
crates/boomdrive/src/lib.rs
crates/boomdrive/src/planes.rs
crates/boomdrive/src/run.rs
crates/boomdrive/src/carry.rs
crates/boomdrive/src/report.rs
crates/boomdrive/src/locks.rs
crates/boomdrive/src/pass.rs
crates/boomdrive/tests/control.rs
crates/boomdrive/tests/loop.rs
crates/boomdrive/tests/once.rs
crates/boomdrive/tests/room.rs
crates/boomdrive/tests/acts.rs
crates/boomdrive/tests/offers.rs
crates/boomdrive/tests/running.rs
crates/boomdrive/tests/fakes/mod.rs
crates/boomdrive/tests/pass.rs
crates/boomdrive/tests/after.rs
crates/boomdrive/tests/planes.rs
README.md
```
