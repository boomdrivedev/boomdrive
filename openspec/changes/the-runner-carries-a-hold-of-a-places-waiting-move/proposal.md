# Proposal

## Why

The brief: "The runner takes hold from its control plane, naming a launch, words and a bound, and carries it as Flume's hold of that place's waiting move."

Flume reads a place as stalled once its next move has been its caller's for five minutes under no hold's words. It then delivers the stall once to each recipient of the place and to the answerer behind its stream. A caller may hold the move the place waits on it for, with words saying who makes it and how, and a bound after which the hold lapses. While the hold stands the place is not stalled, and a retry naming the stage or the submission ends the hold as the move made (Flume's clause 130, and Flume's rule core.24). Flume gives this as `flume hold <place> --waiting [--for <time>] <words>`, from its release `v0.17.0`.

crew talks only to the runner, by messages named for what they do (crew's record 0056, point 1). A proven bolt that waits on the user's send, on a batch, or on a hold the user put on the drive or the kit, now waits on its caller with no words. After five minutes its base place reads as stalled, and its stall reaches ops, the operator and the user as though something had failed. That is signal `2026-11-14-boomdrive-ops-f85c2fe2/01`, on afterthought/crew-state's `boomdrive/main`. crew's record 0067, as amended on 2026-11-14 by boomdrive-design, has crew post `hold`, naming the bolt's launch and the words. The runner carries it as Flume's hold of the base place's waiting move, with the words and bound given. It names that as a Boomdrive unit, built first. crew's unit `a-bolt-waiting-on-the-user-waits-under-words` builds on it.

Today the interface's `ActSort` holds a retry, a land, a confirm, a drop, a priority and a done, and no hold (`crates/boomdrive-control/src/want.rs`). crew's implementation refuses any other `Act`, naming it, for a request naming its host, and gives one naming no host nothing.

## What Changes

- **A hold is an act.** The interface gains `ActSort::Hold { words, bound_interval }`, an act on the launch it names, under that launch's lease. It asks that the move the launch's place waits on its caller for be held under `words`, for `bound_interval` whole seconds, after which Flume lets the hold lapse. Its serde form is `{"hold": {"words": …, "bound_interval": …}}`.
- **The runner carries out a hold as Flume's hold of the waiting move.** It runs `flume --json hold <place> --waiting --for=<n>s -- <words>` on the launch's place, so the words are never read as a flag. Flume's answer is the act's. It is carried out once, in the pass's second round, and never under a lapsed lease, as every act is. Its line names it `hold for <n>s`, and leaves the words out, as a retry's line does.
- **The runner judges nothing of the wait.** Whether the place waits on its caller, and on what, is Flume's to answer. Flume refuses a hold of a place that waits on no move of its caller's, and that refusal is the act's answer. A later hold replaces the earlier one in Flume, and a land ends it as the move made. The runner watches no event, and renews or ends no hold of its own accord.
- **A hold with no words is refused**, running nothing. It would hide the place's stall with nothing said of who makes the move.
- **A hold of a launch not yet launched is refused**, naming where the launch stands, as a retry, a land, a confirm or a done is.
- **crew's implementation reads `Act: hold`**: its `Of`, its `Words` as a retry's are read, and its `Bound`, in whole seconds. One with no `Words`, no `Bound`, or a `Bound` that is no whole number of seconds cannot be read. One naming no `Host` goes to the runner that holds its launch.
- **No version of the hosted protocol carries a hold.** Version 1 was spoken by `v0.1.0` to `v0.7.3`, and version 2 by `v0.9.0`, so neither may change (core.42). A hosted control plane holds nothing through the runner until a later version names a hold.
- **Rule core.59** says what a hold is and how the runner carries it out. **Record 0002** gains an amendment of 2026-11-14. Both are written with this change.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `control-plane-interface`: an act is a retry, a land, a confirm, a hold, a drop, a priority or a done, and a hold holds its words and its bound.
- `work-into-flume`: a hold is Flume's hold of the move its launch's place waits on, with the act's words and bound. A hold with no words, or of a launch not launched, is refused.
- `crew-control-plane`: a `hold` request is an act holding the move its launch's place waits on, under its `Words`, for its `Bound`. One naming no host goes to the runner holding its launch.
- `runner-protocol`: version 2 carries every sort the interface names but a hold, and neither version carries a hold.

## Impact

- `docs/adr/0002-…`: the amendment of 2026-11-14, *a control plane's hold of a place's waiting move*. `docs/architecture/core.md`: core.59 and its details. Both are written with this change.
- `crates/boomdrive-control`:
  - `src/want.rs`: `ActSort::Hold`, documented;
  - `tests/json.rs`.
- `crates/boomdrive`:
  - `src/flume.rs`: `Act::Hold` and its arguments;
  - `src/carry.rs`: a hold's Flume command, its word in the lines, and its refusals;
  - `tests/flume.rs`, `tests/fakes/mod.rs`, a new `tests/holds.rs`, and `tests/after.rs`.
- `crates/boomdrive-crew`:
  - `src/read.rs`: a `hold` request read;
  - `src/lib.rs`: a `hold` naming no host goes to the runner holding its launch;
  - `tests/read.rs`, `tests/claims.rs` and `tests/common/mod.rs`.
- `crates/boomdrive-hosted`:
  - `tests/interface.rs`: a hold is refused by version 1's and version 2's checks, and every other sort still passes version 2's;
  - `protocol/README.md`: version 2 has no message for a hold. No file under `protocol/v1/` or `protocol/v2/` changes.
- `README.md`: the control pass carries out a hold.
- Nothing else changes. No crate, tool or tracked file type is added. No protocol version changes, and no field of `report.json`. The least Flume, `0.21.0`, and the set's Flume, `v0.21.2`, both have `flume hold --waiting --for`, so neither moves.
- Outside this repository:
  - crew's unit `a-bolt-waiting-on-the-user-waits-under-words` posts `hold` with `Of`, `Words` and `Bound`, as design Decision 6 names them;
  - boomdrive-cloud and this repository carry a hold in a later protocol version;
  - Flume's statement names the runner's hold in its row of callers;
  - version 2's files are held as `v0.9.0` spoke them, as version 1's are as `v0.7.3` spoke them.

  Each is named in design.md and told to the orchestrator.

## Touches

```
docs/adr/0002-the-runner-takes-leased-work-while-its-flume-stream-has-room-and-carries-each-tools-record-back-as-given.md
docs/architecture/core.md
crates/boomdrive-control/src/want.rs
crates/boomdrive-control/tests/json.rs
crates/boomdrive/src/flume.rs
crates/boomdrive/src/carry.rs
crates/boomdrive/tests/flume.rs
crates/boomdrive/tests/fakes/mod.rs
crates/boomdrive/tests/holds.rs
crates/boomdrive/tests/after.rs
crates/boomdrive-crew/src/read.rs
crates/boomdrive-crew/src/lib.rs
crates/boomdrive-crew/tests/read.rs
crates/boomdrive-crew/tests/claims.rs
crates/boomdrive-crew/tests/common/mod.rs
crates/boomdrive-hosted/tests/interface.rs
crates/boomdrive-hosted/protocol/README.md
README.md
openspec/specs/control-plane-interface/spec.md
openspec/specs/work-into-flume/spec.md
openspec/specs/crew-control-plane/spec.md
openspec/specs/runner-protocol/spec.md
```
