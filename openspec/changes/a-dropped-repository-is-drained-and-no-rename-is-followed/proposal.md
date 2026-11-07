# Proposal

## Why

Record 0021, accepted by the user on 2026-11-07 as revised on their card, rules out handling a repository renamed in flight: "We are not going to build special functionality to handle repositories being renamed in flight. Big picture with boomdrive, you add repositories and, when you remove repositories, I don't think there's a rename. I think you would drain the work from it, remove it, do the rename separately, and then add it back."

The fix `the-runner-follows-each-place-it-launched-to-its-end` built the option the record rejected. The pass asks Flume of a place by its location alone once the configuration drops its repository, and crew's implementation gives a launch the runner answered whatever the hello names, its `Repo` as written. Neither keeps the dropped repository's clone, so the keeper removes it as soon as Flume's place leaves it, and a place tidied with an end pending is followed through no clone at all.

The record's consequences take that fix back: the runner asks Flume of a place only through a kept clone, crew's implementation gives a launch only where the runner keeps its repository, draining ones among them, and the work-into-flume and crew-control-plane specs say so. core.5's "Not ruled out" already names a dropped repository's clone and view kept, with nothing launched in it, while a place the runner launched there is live, then removed.

## What Changes

- **A dropped repository is drained.** Where the configuration drops a repository while the runner's report holds a launch at a place there that has not ended, the keeper keeps its clone and view, neither fetched nor moved, and names them held by that place. A place has ended once the status carried of it reads it tidied or gone, or stands not listed. Then the keeper removes them as it removes any dropped repository's.
- **The report records each launch's repository.** The interface's `Launched` gains the repository the offer named, set when the runner first answers it and never changed, so the keeper can tell which places hold which clone.
- **The hello names the repositories the runner drains**, apart from those it keeps. The interface's `Hello` gains them. The hosted implementation's version 1 names none.
- **Nothing more is launched in a drained repository.** An offer there is refused, as one in a repository not kept, but where Flume's listing holds its place under its mark. An offer for a repository the runner neither keeps nor drains is refused before any mark is read. A branch command or an inbox in a drained repository is answered as one in a repository not kept.
- **A place is followed only through a clone that stands.** The pass carries Flume's status of a place, and sluice's entry of its submission, through the repository's clone, named or drained. Where no clone stands, nothing is carried. A landing in a drained repository is told to no keeper.
- **crew's implementation gives a launch only for a repository the runner keeps or drains.** One no runner answered is given only where its `Repo` names a repository the hello keeps. One the runner answered is given where its `Repo` names one the hello keeps, else one it drains. Where neither does, it is given nothing, and no old name is matched to a new one.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `clones`: a dropped repository is drained while a place the runner launched there has not ended, then removed.
- `work-into-flume`: the hello names the repositories drained; an offer in a drained repository is refused but where its mark is found, and one in a repository neither kept nor drained is refused before its mark is read; a place's status is carried only through its repository's clone; the report keeps each launch's repository; the requirement that followed a place by its location alone is removed.
- `crew-control-plane`: an unanswered launch is given only for a repository the hello keeps; an answered one for a repository it keeps or drains, and nothing where neither; the requirement that gave an answered launch whatever the hello names is removed.
- `tool-records`: a submission's entry is read in the clone of the launch's repository, named or drained.
- `control-plane-interface`: the report names each launch's repository.
- `hosted-control-plane`: no hello names a drained repository.

## Impact

- `crates/boomdrive-control`: `Hello` gains `draining`, and `Launched` gains `repository`, each read as empty where a file or a message lacks it; every construction of them in every crate names them.
- `crates/boomdrive`: `report.rs` gains the places that hold a dropped clone; `facts.rs` reads a place's phase tidied or gone; `pass.rs` keeps a drained clone and names it held; `carry.rs` gains `Keeping::Dropped`, refuses work in a drained repository but a launch found by its mark, carries statuses and entries through any clone that stands, and records each launch's repository; `hello.rs` names the drained repositories.
- `crates/boomdrive-crew`: `pull_partition` resolves a launch's `Repo` against the hello's kept repositories, and an answered launch's, where none has it, against its drained ones.
- `crates/boomdrive-hosted`: no code but the new fields' constructions; a test pins that a drained repository goes up in no hello.
- `README.md`: a dropped repository is drained.
- No rule or record changes: record 0021 and core.5 already say this. No crate, tool, tracked file type or message of the protocol is added.
- Outside this repository: under the hosted implementation, whether the cloud keeps giving a launch whose repository the runner drains is the cloud's. Version 1's hello cannot say a repository is drained, and a version that can is a change of its own, in boomdrive-cloud and here.

## Touches

```
crates/boomdrive-control/src/lib.rs
crates/boomdrive-control/src/report.rs
crates/boomdrive-control/src/shared.rs
crates/boomdrive-control/tests/fake.rs
crates/boomdrive-control/tests/json.rs
crates/boomdrive/src/carry.rs
crates/boomdrive/src/facts.rs
crates/boomdrive/src/hello.rs
crates/boomdrive/src/pass.rs
crates/boomdrive/src/report.rs
crates/boomdrive/tests/fakes/mod.rs
crates/boomdrive/tests/dropped.rs
crates/boomdrive/tests/entries.rs
crates/boomdrive/tests/offers.rs
crates/boomdrive/tests/statuses.rs
crates/boomdrive-crew/src/lib.rs
crates/boomdrive-crew/src/read.rs
crates/boomdrive-crew/tests/claims.rs
crates/boomdrive-crew/tests/common/mod.rs
crates/boomdrive-crew/tests/letgo.rs
crates/boomdrive-crew/tests/read.rs
crates/boomdrive-crew/tests/roles.rs
crates/boomdrive-crew/tests/write.rs
crates/boomdrive-hosted/src/plane.rs
crates/boomdrive-hosted/tests/connection.rs
crates/boomdrive-hosted/tests/plane.rs
README.md
```
