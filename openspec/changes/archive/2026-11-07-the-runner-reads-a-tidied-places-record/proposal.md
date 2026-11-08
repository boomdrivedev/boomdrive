# Proposal

## Why

The brief, 2026-11-07: "Boomdrive's runner reads a place Flume has tidied from Flume's record of it at `<location>@<at>`, as work-into-flume's spec says, so a unit, chore or fix that landed and was then tidied is carried as landed, never at the stage it last showed."

The runner already takes a tidied place's record as the place's entry. What it compares against is stale. A control pass reads Flume's listing once, before its pull, answers the pull's wants, and only then carries each place's status, asking Flume again only for a place whose entry in that listing differs from the one carried. On 2026-11-07 one pass ran its launches and acts for about 40 minutes. Two chores Flume tidied, landed, while it ran were compared against the listing from its start, where both stood live as carried, and stayed live. Flume keeps such a record only while an end of it is pending for a recipient (Flume's 43, 62), so a landing not read before the recipient takes its ends is lost to the runner for good.

## What Changes

- **Record 0027**, in this change's commit: the runner carries each place's status against Flume's listing as it stands once the pass's commands have run.
- **The listing is read again before the statuses are carried**, where answering the pull's wants ran a tool's command: a launch, an act of Flume's or a branch of sluice's. A pass that ran none carries them against its own listing, read seconds before.
- **The hello's capacity and the room stay the pass's own listing's**, read before the pull.
- **A launch made in a pass is carried listed in that pass**, since the listing read after it holds its place.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `work-into-flume`: the host part and the room are read from the listing a pass reads before its pull; each place's status is carried against Flume's listing read again once the wants have run a tool's command, a place tidied meanwhile carried from its record, and a quiet pass reads the listing once.

## Impact

- `crates/boomdrive/src/carry.rs`: the pass reads Flume's listing again before `carry_statuses` where a want ran a launch, an act or a branch.
- Tests: `crates/boomdrive/tests/statuses.rs`, with a fake Flume that moves a place on while a command runs, in `crates/boomdrive/tests/fakes/mod.rs`.
- `docs/adr/0027-…`, the record.
- `README.md`: the listing read again.
- Nothing else changes: no crate, no tool, no tracked file type, no message of the protocol, no record crew's implementation writes.
- Outside this repository: a landing whose record Flume removes before the runner reads it stays lost. That is Flume's to keep for the place's launcher, or the control plane's to read from sluice's entry, and is told to the orchestrator.

## Touches

```
crates/boomdrive/src/carry.rs
crates/boomdrive/tests/statuses.rs
crates/boomdrive/tests/fakes/mod.rs
docs/adr/0027-the-runner-carries-each-places-status-against-flumes-listing-as-it-stands-once-the-passs-commands-have-run.md
README.md
```
