# Proposal

## Why

A control plane lands work today by asking a retry naming the submission, which the runner carries out as `flume retry <place> --submission` (the work-into-flume spec's "A bolt's send is a retry of its submission"). Retry and submission are Flume's words, so a control plane that lands a bolt names Flume's command for it.

On 2026-11-08 the user rewrote crew's record 0056 and accepted it on their card. In their words: "It's not crew's job to land anymore. That's the whole point of the runner in Flume and sluice. [...] It is going to be a decision in the user's rail to land, so that's a message for a Flume place, theoretically." And: "There can be messages back and forth with the control plane but crew doesn't know about Flume directly except for whatever the control plane contract allows." crew's record names the message `land`, for a bolt and a fix alike, and leaves the runner's half to a Boomdrive unit: the runner takes `land` as the message for a work's landing, and carries it out as Flume's retry of the work's submission.

The brief: "The runner takes land, naming a launch, as the control plane's message to land that work, and carries it out as Flume's retry of the work's submission, so a control plane names no Flume word for a landing."

## What Changes

- **A land is an act.** The interface gains `ActSort::Land`, an act on the launch it names, under that launch's lease, holding nothing else. Its serde form is the word `land`.
- **The runner carries out a land as Flume's retry of the submission.** It runs `flume --json retry <place> --submission` on the launch's place, with no stage and no words. Every launch is treated alike: the runner reads no kind, so a bolt, a fix and a unit land the same way. Flume's answer is the act's. A land is carried out once, in the pass's second round, after Flume's status of a place the listing does not hold, and never under a lapsed lease, as every act is.
- **A land of a launch not yet launched is refused**, naming where the launch stands, as a retry or a done is.
- **A bolt is landed on a land.** The work-into-flume spec's bolt requirement takes `land` in place of a retry naming the submission.
- **crew's implementation reads `Act: land`.** A `land` request is given as a land of the launch its `Of` names. One that names no `Host` goes to the runner that holds its launch.
- **The retry naming the submission stays**, in the interface and in crew's implementation. crew still asks it, with words, to answer a place stopped short in its submission. Version 1 of the hosted protocol names it, and a release has spoken that version (core.42).
- **Version 1 of the hosted protocol carries no land.** It may not change (core.42). So a land is no act of version 1, and is refused by its check. A hosted control plane lands work by version 1's retry naming the submission until a later version carries `land`.
- **Rule core.53** says what a land is and how the runner carries it out. **core.52** names it as how a branch other places land on is landed. **Record 0002** gains an amendment of 2026-11-08, proposed. All three are written with this change.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `control-plane-interface`: an act is a retry, a land, a drop, a priority or a done, and a land holds nothing but its launch.
- `work-into-flume`: a land is Flume's retry of its launch's submission; a land of a launch not launched is refused; a bolt is landed on a land.
- `crew-control-plane`: a `land` request is an act landing the launch it names, and one naming no host goes to the runner holding its launch.
- `runner-protocol`: version 1 carries the interface's shapes but a land, which its check refuses.

## Impact

- `docs/adr/0002-…`: the amendment of 2026-11-08, *land is the message for a work's landing*, proposed. `docs/architecture/core.md`: core.53 and its details, and core.52's text and details. These are written with this change.
- `crates/boomdrive-control`:
  - `src/want.rs`: `ActSort::Land`, documented;
  - `tests/json.rs`.
- `crates/boomdrive`:
  - `src/carry.rs`: a land's Flume command, its word in the lines, and its refusal before its launch is made;
  - `tests/acts.rs`, `tests/after.rs` and `tests/bolts.rs`.
- `crates/boomdrive-crew`:
  - `src/read.rs`: a `land` request read;
  - `src/lib.rs`: a `land` naming no host goes to the runner holding its launch;
  - `tests/read.rs`, `tests/claims.rs` and `tests/common/mod.rs`.
- `crates/boomdrive-hosted/tests/interface.rs`: a land is refused by version 1's check.
- `README.md`: the control pass carries out a land, and a bolt is landed on one.
- Nothing else changes: no crate, no tool, no tracked file type, no protocol version or file, no field of `report.json`, and no Flume command the runner did not run before.
- Outside this repository:
  - crew's unit that lands each bolt through Flume's base place posts `land`;
  - boomdrive-cloud lands by version 1's retry until a version carries `land`;
  - Flume's statement names crew's retry and the runner's in its section 9.

  Each is named in design.md and told to the orchestrator.

## Touches

```
docs/adr/0002-the-runner-takes-leased-work-while-its-flume-stream-has-room-and-carries-each-tools-record-back-as-given.md
docs/architecture/core.md
crates/boomdrive-control/src/want.rs
crates/boomdrive-control/tests/json.rs
crates/boomdrive/src/carry.rs
crates/boomdrive/tests/acts.rs
crates/boomdrive/tests/after.rs
crates/boomdrive/tests/bolts.rs
crates/boomdrive-crew/src/read.rs
crates/boomdrive-crew/src/lib.rs
crates/boomdrive-crew/tests/read.rs
crates/boomdrive-crew/tests/claims.rs
crates/boomdrive-crew/tests/common/mod.rs
crates/boomdrive-hosted/tests/interface.rs
crates/boomdrive-hosted/tests/connection.rs
README.md
openspec/specs/control-plane-interface/spec.md
openspec/specs/work-into-flume/spec.md
openspec/specs/crew-control-plane/spec.md
openspec/specs/runner-protocol/spec.md
```
