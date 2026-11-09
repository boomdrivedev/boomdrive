# Proposal

## Why

The brief: "A reader that talks only to its control plane, crew's role runs among them, has each end it acted on confirmed in Flume as that reader on whichever host holds the place, at its control plane's ask to the runner that holds the launch, and an end its control plane never named stays owed."

Flume now keeps each end of a place owed to each of its recipients until that recipient confirms it by its delivery's key. A hand-over, by a notice, a wait or the inbox, confirms nothing. A tidied place's record stays until every end of it is confirmed, and an end nobody confirms expires and shows in status with its recipient (weir record 0006, accepted by the user on 2026-11-08; Flume record 0061, accepted on 2026-11-09, Flume `v0.21.0`). Flume takes a recipient's confirmation from the caller that names it, and never confirms for one itself.

crew names a role's address as the recipient of every place it launches, and crew's role runs act on those places' ends. Since crew's change `crews-role-runs-confirm-what-they-collect` (crew's main at 271e2c4), a role's run that ends a turn posts one `confirm` request per launch whose ends the turn took. It names the launch, the role's address and each end's delivery key. crew never calls Flume, on the user's words in crew's record 0056, point 1: "crew doesn't know about Flume directly except for whatever the control plane contract allows". The place may be on another host, which only the runner that holds the launch reaches. Today the runner reads no confirm. It gives a confirm naming no host nothing, and names it in its log. So every end of every place crew launched stays owed to a role's address, and Flume never lets go of a tidied place's record.

## What Changes

- **A confirm is an act.** The interface gains `ActSort::Confirm { recipient, deliveries }`, an act on the launch it names, under that launch's lease. It holds the one recipient it confirms as and each delivery's key, as Flume gave it, in the control plane's order. Its serde form is `{"confirm": {"recipient": …, "deliveries": […]}}`.
- **The runner carries out a confirm as Flume's confirm.** It runs `flume --json confirm --recipient=<recipient> -- <key>…` on the host that holds the launch, naming exactly the keys the act names, as the recipient it names. Flume's answer is the act's. It is carried out once, in the pass's second round, and never under a lapsed lease, as every act is. Its line names it `confirm as <recipient>`.
- **Each key is held to the launch's place.** Flume keeps a host's deliveries in one document and confirms any key on the host. So the runner first reads its launch's place as Flume gives it, in the pass's listing or by `flume --json status <place>`. A key that is the `delivery` of no end of that place is refused, naming the key, and nothing runs. A confirm under one launch's lease confirms no other launch's end, no other drive's and no other account's.
- **A confirm as the runner, or of no key, is refused**, running nothing. The runner's own ends, as `boomdrive`, are the runner's to carry and collect.
- **A confirm of a launch not yet launched is refused**, naming where the launch stands, as a retry, a land or a done is.
- **The runner confirms nothing of its own accord for another reader.** An end its control plane never named in a confirm stays owed in Flume, until a confirm names it or it expires and Flume shows it.
- **crew's implementation reads `Act: confirm`**: its `Of`, its one `Recipient` and each `Delivery`, in the order written. One naming no `Host` goes to the runner that holds its launch. One with no `Recipient`, two, or no `Delivery` cannot be read.
- **Version 1 of the hosted protocol carries no confirm.** It may not change (core.42), and its copy of a place's status names no end's delivery key.
- **Rule core.56** says what a confirm is and how the runner carries it out. **Record 0002** gains an amendment of 2026-11-09, proposed. Both are written with this change.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `control-plane-interface`: an act is a retry, a land, a confirm, a drop, a priority or a done, and a confirm holds its recipient and its delivery keys.
- `work-into-flume`: a confirm is Flume's confirm of exactly the keys it names, as the recipient it names; each key is held to the launch's place; a confirm as the runner, of no key, or of a launch not launched is refused.
- `crew-control-plane`: a `confirm` request is an act confirming the deliveries it names as the recipient it names, and one naming no host goes to the runner holding its launch.
- `runner-protocol`: version 1 carries the interface's shapes but a land and a confirm, which its check refuses.

## Impact

- `docs/adr/0002-…`: the amendment of 2026-11-09, *a control plane's confirm of a reader's ends*, proposed. `docs/architecture/core.md`: core.56 and its details. Both are written with this change.
- `crates/boomdrive-control`:
  - `src/want.rs`: `ActSort::Confirm`, documented;
  - `tests/json.rs`.
- `crates/boomdrive`:
  - `src/flume.rs`: `Act::Confirm` and its arguments;
  - `src/carry.rs`: a confirm's Flume command, its word in the lines, its refusals, and each key held to the launch's place;
  - `src/facts.rs`: the delivery keys of a place's ends;
  - `tests/fakes/mod.rs`, a new `tests/confirms.rs`, and `tests/after.rs`.
- `crates/boomdrive-crew`:
  - `src/read.rs`: a `confirm` request read;
  - `src/lib.rs`: a `confirm` naming no host goes to the runner holding its launch;
  - `tests/read.rs`, `tests/claims.rs` and `tests/common/mod.rs`.
- `crates/boomdrive-hosted/tests/interface.rs`: a confirm is refused by version 1's check.
- `README.md`: the control pass carries out a confirm.
- Nothing else changes. No crate, tool or tracked file type is added. No protocol version or file changes, and no field of `report.json`. The least Flume and the set's Flume stay where they are: this bolt's other unit, `the-runner-confirms-an-end-once-its-control-plane-has-it`, needs the same Flume release for the runner's own confirmation and moves both, once, for the bolt (design Decision 8).
- Outside this repository:
  - crew posts `confirm` already, and needs nothing more;
  - boomdrive-cloud and this repository carry a confirm, and an end's delivery key, in a later protocol version;
  - Flume's statement names the runner's confirm in its row of callers.

  Each is named in design.md and told to the orchestrator.

## Touches

```
docs/adr/0002-the-runner-takes-leased-work-while-its-flume-stream-has-room-and-carries-each-tools-record-back-as-given.md
docs/architecture/core.md
crates/boomdrive-control/src/want.rs
crates/boomdrive-control/tests/json.rs
crates/boomdrive/src/flume.rs
crates/boomdrive/src/carry.rs
crates/boomdrive/src/facts.rs
crates/boomdrive/tests/fakes/mod.rs
crates/boomdrive/tests/confirms.rs
crates/boomdrive/tests/after.rs
crates/boomdrive-crew/src/read.rs
crates/boomdrive-crew/src/lib.rs
crates/boomdrive-crew/tests/read.rs
crates/boomdrive-crew/tests/claims.rs
crates/boomdrive-crew/tests/common/mod.rs
crates/boomdrive-hosted/tests/interface.rs
README.md
openspec/specs/control-plane-interface/spec.md
openspec/specs/work-into-flume/spec.md
openspec/specs/crew-control-plane/spec.md
openspec/specs/runner-protocol/spec.md
```
