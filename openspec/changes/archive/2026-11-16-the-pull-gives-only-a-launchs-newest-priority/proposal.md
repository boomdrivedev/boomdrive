# Proposal

## Why

The brief, 2026-11-16: crew's pull, in `boomdrive-crew`, hands deneb's runner every priority request written for a launch. A later priority for the same launch makes each earlier one the runner has not answered moot. crew's bolt-teams has "A start or a reorder appends each priority it changed", and that night `boomdrive/72` alone had priorities 479, 506, 538 and on to 1102. A pass past due runs one act on a place (core.50), so the moot priorities buried the user's lands. deneb's log said "the next control pass is due, so 354 acts on places wait for it", and the lands asked at 23:54Z (632 and 633) and 1032 to 1035 waited over five hours.

boomdrive-design ruled that the pull gives a priority only while it is its launch's newest priority, or while the runner already holds it, as `Pulling::preview_current` already does for previews by `Main::newest_preview` (record 0031 as amended on 2026-11-09). That clears the backlog at the next pull. core.50, the runner's carrying, Flume and crew's numbering stay as they are.

## What Changes

- **A priority the runner does not hold is given only while it is its launch's newest.** A request that reads as a priority has `Act: priority`, a whole `Of` and a whole `Priority`. crew's implementation gives it only where no request of a higher number that reads as a priority names the same `Of`, or where the runner's report or this host's branch answers it. This holds whatever `Host` it names. Every other rule that gives a priority stays as it is: its work in the plan or held, and, naming no host, the runner holding its launch.
- **A priority that cannot be read hides no earlier one.** A later request whose `Of` or `Priority` is no whole number does not make an earlier priority moot. It is given or left out as before: refused by its host's runner where it names one, and named in the log where it names none.
- Nothing changes in the runner, its order of wants, Flume, crew's records or the interface.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `crew-control-plane`: a priority the runner does not hold is given only while it is its launch's newest, and a priority that cannot be read makes no earlier one moot.

## Impact

- `crates/boomdrive-crew/src/read.rs`: `Request::priority_of` and `Main::newest_priority`.
- `crates/boomdrive-crew/src/lib.rs`: `Pulling::current` gives a priority only while it is its launch's newest or the runner holds it.
- `crates/boomdrive-crew/tests/priorities.rs`: the spec's scenarios.
- `docs/adr/0002-*.md`: the amendment of 2026-11-16 that records design's ruling.
- On deneb, the first pull after the upgrade gives `boomdrive/72` its newest priority alone, and the acts waiting behind its moot ones go. A moot priority the runner already answered is still given, and answered from its report, until crew lets it go as before.
- Nothing else changes: no rule, no crate, no tool, no tracked file type, no message of the protocol, and no other control plane's implementation.

## Touches

```
crates/boomdrive-crew/src/read.rs
crates/boomdrive-crew/src/lib.rs
crates/boomdrive-crew/tests/priorities.rs
docs/adr/0002-the-runner-takes-leased-work-while-its-flume-stream-has-room-and-carries-each-tools-record-back-as-given.md
```
