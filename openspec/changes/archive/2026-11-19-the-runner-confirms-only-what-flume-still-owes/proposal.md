# Proposal

## Why

The runner confirms to Flume each end of a place owed to it once its control plane has taken the status that gives it (record 0030 as amended on 2026-11-09; core.54). It reads which keys are owed from the status it carried, and Flume's listing that marked that status listed was read before the pass's report was sent. A send to crew's state repository is a commit and a push, and can take a while. On deneb on 2026-11-19, Flume forgot a tidied place's record meanwhile, the place gone from `flume list --all` and its key from the inbox, and the runner confirmed from the status carried before: Flume could only refuse it ("no delivery is recorded under" the key), 38 times in that day's log.

## What Changes

- **The runner confirms only what Flume still lists owed.** Once its report is taken, and only where some end is to be confirmed, the runner reads Flume's listing once more, and confirms of each place only the keys that both the status its control plane took and that listing give pending for `boomdrive`. A key Flume no longer lists owed, or a place it no longer lists, is not confirmed, and nothing is said of it.
- **No listing, no confirm.** Where Flume's listing cannot be read then, its words are said in the pass's line for Flume's status, and nothing is confirmed that pass; a later pass confirms what Flume still owes.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `work-into-flume`: an end is confirmed only while Flume's listing, read after the report is taken, still gives it owed to the runner.

## Impact

- `crates/boomdrive/src/carry.rs`: `Work::confirm` reads Flume's listing after the send and keeps only the keys it gives owed.
- `crates/boomdrive/tests/statuses.rs`: two cases; `crates/boomdrive/tests/fakes/mod.rs`: a scripted listing taken back.
- No rule, record, protocol or configuration changes: core.54 already confirms only what Flume owes.

## Touches

```
crates/boomdrive/src/carry.rs
crates/boomdrive/tests/statuses.rs
crates/boomdrive/tests/fakes/mod.rs
```
