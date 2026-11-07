# Proposal

## Why

A control pass takes a pull's launches ahead of its acts on places (record 0002's first amendment of 2026-11-07), but a launch the next pull offers still waits for every act of the pass under way, and every act on a place costs Flume's settle of every place on the host, where no caller keeps watch (Flume's clause 119), even one Flume refuses because its place no longer stands.

On deneb on 2026-11-07 one pass carried out some 180 of one partition's acts on places, 162 of them refused, at about 30 seconds each: `flume --json priority <place> 12` took 27 to 33 seconds to refuse a place that no longer stood, where `flume --json status <place>`, which settles nothing, refused it in about ten milliseconds in the same words. Another partition's launches 63 and 64, asked at 13:48:54Z, waited through that pass and most of the next, with that partition's stream running none of its four agents.

The orchestrator's brief, 2026-11-07: "Boomdrive's runner answers a pull's acts in seconds: no partition's acts hold another partition's launches, within a partition launches go ahead of other acts, and an act Flume refuses because its place no longer stands costs no more than reading the refusal, as records 0002 and 0010 say."

## What Changes

- **An act on a place the pass's listing does not hold reads Flume's status of it first.** Where Flume refuses it, that refusal, as Flume gave it, is the act's answer, and the act's command is not run. Where Flume gives it, the command runs as before.
- **Once the next control pass is due**, by the control timer or by the control plane's wake, a pass runs no further act's command but its first, and leaves the rest to that pass, whose launches go first. `boomdrive run --once` makes no next pass, so its pass carries out every act.
- **Rule core.50**, this bolt's rule core.49 renumbered beside main's core.49 (record 0026), says it, and **rule core.51** is added; core.1's details allow both. Records 0002 and 0010 carry the decisions.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `work-into-flume`: a pull's launches go ahead of its acts on places, and of the acts left by the pass before; an act on a place Flume does not stand costs the reading of Flume's refusal.

## Impact

- `docs/architecture/core.md`: core.49 becomes core.50 and is amended, core.51 is added, and core.1's details allow them. Record 0002 gains a second amendment of 2026-11-07, and record 0010's amendment of that day carries the new decision.
- `crates/boomdrive`:
  - `src/carry.rs`: the status read before an act's command on a place the listing does not hold, and the pass's `due`;
  - `src/run.rs`: the runner's one set of notices, whose wake the pass under way reads, and the `due` the loop hands each pass;
  - `tests/acts.rs`, `tests/fakes/mod.rs` and `tests/pass.rs`: the spec's scenarios, and the bench's `due`.
- `README.md`: when a control pass carries out an act on a place.
- Nothing else changes: no crate, no tool, no tracked file type, no message of the protocol, no control plane's implementation.
- Outside this repository: Flume settles every place before it finds that a place does not stand. That is Flume's, and is told to the orchestrator.

## Touches

```
docs/architecture/core.md
docs/adr/0002-the-runner-takes-leased-work-while-its-flume-stream-has-room-and-carries-each-tools-record-back-as-given.md
docs/adr/0010-a-lost-leases-place-is-left-until-a-command-drops-it-and-the-runner-never-retries-back-to-back.md
crates/boomdrive/src/carry.rs
crates/boomdrive/src/run.rs
crates/boomdrive/tests/acts.rs
crates/boomdrive/tests/fakes/mod.rs
crates/boomdrive/tests/pass.rs
README.md
```
