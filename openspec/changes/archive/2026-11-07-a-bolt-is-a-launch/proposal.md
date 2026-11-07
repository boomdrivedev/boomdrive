# Proposal

## Why

A bolt's branch had no place. The runner made it with `sluice branch create`, as a command under a lease of its own (record 0002's settled point 3, the branch-commands spec), and crew's plugin made it the same way. The units' places on it were Flume's and landed through Flume's settles, while the bolt was submitted, waited on and removed by hand. One depth was Flume's and the other crew's and the runner's. The user ruled that out on 2026-11-06 ("The code shouldn't know about bolts and units. [...] I want it to be a consistent and obvious design in terms of where the merge queue runs"; on the outer branch's place, "No worktree: a branch and a record"). They accepted Flume's record 0051 and its follow-ups that day.

Flume's statement now has such a branch as a base place's: a place whose kind lands and has no stages, its branch made and closed by the merge queue at Flume's asking, taking no room, and submitted only on its caller's retry naming the submission (Flume's 202 to 206). Its caller is the runner: "each launch its control plane asks, passed as given, a bolt's base place and its units' places among them; a retry of a base place's submission its control plane asks (203)". Flume's design names this repository's follow-up B1: "the runner keeps the host's watch, and a bolt is a launch". This change is the second half of B1. Its sibling `each-pass-ends-with-a-settle` keeps the watch.

The brief: "A control plane's bolt reaches the runner as a Flume launch of its caller's kind with no stages, and its send as a retry of that place's submission, so sluice alone makes and moves a bolt's branch and the runner keeps no branch command."

## What Changes

- **A bolt is a launch.** The runner already passes any launch as given. A launch of a kind with no stages is judged by its stream's room and launched as any other, and its units by `--base <its branch>` as now. The runner reads no kind, and gains tests that show it.
- **A bolt's send is an act.** It is a retry naming the submission, carried out as `flume retry <place> --submission`, its answer Flume's. This is already built, and gains tests that show it for a bolt.
- **The runner keeps no branch command.** A branch command is refused, naming sluice, whatever its repository, and runs nothing. `carry.rs`'s branch, sluice's `branch` call and its `Printed`, `clone::origin_commit`, and sluice 0.61.1's branch fixtures go.
- **The shapes stay.** The interface's `CommandSort::Branch` and `Commanded::origin_commit` stay, because version 1 of the hosted protocol, which a release has spoken, carries them (core.42), and `report.json` keeps the origin commit for the release before it (core.36). Their documentation says no command this release answers fills them.
- **A command runs no tool.** So a pass whose wants ran only commands reads Flume's listing once (record 0027).
- **Rule core.52** says the runner asks no tool for a branch but through Flume's commands on a place. **Record 0002** gains an amendment, proposed, that retires settled point 3. Both are written with this change.

## Capabilities

### New Capabilities

- `commands`: how the runner answers the commands its control plane sends. Every command is refused, naming its tool, a branch command among them, and is answered once.

### Modified Capabilities

- `work-into-flume`: a bolt is a launch, and its send a retry of its submission; a command whose lease lapsed waits, then is refused; a command runs nothing that makes the statuses' listing be read again.
- `branch-commands`: retired. Every requirement is removed, and what holds of commands moves to `commands`.

## Impact

- `docs/adr/0002-…`: the amendment of 2026-11-07, *a bolt is a launch*, proposed. `docs/architecture/core.md`: core.52 and its details. Both are written with this change.
- `crates/boomdrive`:
  - `src/carry.rs`: a branch command refused, naming sluice, and its making gone;
  - `src/sluice.rs`: `Sluice::branch`, `Cli::branch` and `Printed` gone;
  - `src/clone.rs`: `origin_commit` gone;
  - `tests/branches.rs` becomes `tests/commands.rs`, for the commands spec;
  - `tests/bolts.rs`, new, for the work-into-flume spec's bolt;
  - `tests/leases.rs`, `tests/statuses.rs`, `tests/sluice.rs`, `tests/rules.rs` and `tests/readme.rs`;
  - `tests/fixtures/sluice-0.61.1/`: `branch-made.json`, `branch-there.json` and `branch-ungated.json` go, with the README's section on them.
- `crates/boomdrive-control/src/want.rs` and `src/report.rs`: documentation only.
- `README.md`: how a control pass treats a bolt and a command.
- Nothing else changes: no crate, no tool, no tracked file type, no protocol version or message, and neither control plane's implementation.
- Outside this repository: boomdrive-cloud's records 0017 and 0051 make a bolt by a branch Command, so the cloud follows by launching a bolt and sending it by a retry. crew's follow-up C1 makes its bolts base places. Both are named in design.md and told to the orchestrator.

## Touches

```
docs/adr/0002-the-runner-takes-leased-work-while-its-flume-stream-has-room-and-carries-each-tools-record-back-as-given.md
docs/architecture/core.md
crates/boomdrive/src/carry.rs
crates/boomdrive/src/sluice.rs
crates/boomdrive/src/clone.rs
crates/boomdrive/tests/branches.rs
crates/boomdrive/tests/commands.rs
crates/boomdrive/tests/bolts.rs
crates/boomdrive/tests/leases.rs
crates/boomdrive/tests/statuses.rs
crates/boomdrive/tests/sluice.rs
crates/boomdrive/tests/rules.rs
crates/boomdrive/tests/readme.rs
crates/boomdrive/tests/fakes/mod.rs
crates/boomdrive/tests/fixtures/sluice-0.61.1/README.md
crates/boomdrive/tests/fixtures/sluice-0.61.1/branch-made.json
crates/boomdrive/tests/fixtures/sluice-0.61.1/branch-there.json
crates/boomdrive/tests/fixtures/sluice-0.61.1/branch-ungated.json
crates/boomdrive-control/src/want.rs
crates/boomdrive-control/src/report.rs
README.md
```
