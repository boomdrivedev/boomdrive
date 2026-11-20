# Proposal

## Why

Record 0033, accepted by the user on 2026-11-11, has the runner's pass run millpond's tick as it runs Flume's settle, so a host keeps one clock for Flume's watch and millpond's schedules (the superfans' four-hourly passes and the tools' watches). millpond v0.1.0 gives `millpond tick --inbox <name> --json`. The record's last consequence asks core.13 to name the tick, and the running and configuration specs and the code to follow once the tick exists.

## What Changes

- **The tick, after each settle.** Each pass, keeping or a part's control pass, and `run --once`, ends with the settle, then one `millpond tick --inbox <name> --json` of each inbox the configuration's `inboxes` names, in order, awaited on the settle's thread. A pass that shares a settle under way shares its ticks.
- **Said only when it changes.** A refused tick, or one with no answer, is said once for its inbox until millpond ticks it again; no tick stops a pass, and none is carried or kept.
- **`inboxes` in the configuration.** A list of millpond's names of the inboxes this host keeps; none where the file names none.
- **core.13 is amended** to name the tick beside the settle, as the second and last housekeeping the runner runs of its own accord.

Not in this change: the runner's watch adapter (record 0033's Boomdrive watch), and the set's installing millpond and writing its host configuration, which follow in their own changes.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `running`: each pass ticks each inbox after its settle; a tick is said only when it changes.
- `configuration`: the configuration names the inboxes the host keeps.

## Impact

- `crates/boomdrive/src/millpond.rs` (new): millpond's command line and what the runner says of its ticks.
- `crates/boomdrive/src/run.rs`: the watch ticks each inbox after its settle; `Programs` names millpond.
- `crates/boomdrive/src/config.rs`: `inboxes`.
- `docs/architecture/core.md`: core.13 and its details.
- Tests: `tests/loop.rs` with a stub `millpond`; the config's and millpond's unit tests.

## Touches

```
crates/boomdrive/src/millpond.rs
crates/boomdrive/src/run.rs
crates/boomdrive/src/config.rs
crates/boomdrive/src/lib.rs
crates/boomdrive/tests/
docs/architecture/core.md
README.md
```
