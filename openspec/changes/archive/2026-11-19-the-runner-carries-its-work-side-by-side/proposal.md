# Proposal

## Why

A part's control pass carries its wants one after another, each blocking on its tool's process. `flume launch` answers only once its place is readied and its first stage started, from seconds to minutes, so one slow launch holds every other launch, land, answer and hold behind it. On the evening of 2026-11-18 about ten launches (offers 2160 to 2175) took hours, and the user's lands and the signer fixes waited behind them (signals `2026-11-19-boomdrive-ops-7502663a/01-the-runner-carries-work-concurrently` and `2026-11-19-boomdrive-ops-4865984e/01-the-runner-launches-one-place-at-a-time`). The turns of 2026-11-14, a pass past due making one launch and one act and holding the rest, were only a workaround for that wait. Record 0002 as amended on 2026-11-19, with core.50 and core.55, has each want carried on a thread of its own and each answer come back over a channel.

## What Changes

- **Each want on its own thread.** A launch, an act on a place, a preview and a release each run their tool's command on a thread of their own as soon as the pull gives them; the pass waits on none.
- **Room in the control plane's order.** A pull's launches take the stream's room in order, and every launch taken runs at once; a launch in flight holds its room until Flume lists its place.
- **Order only where it changes the outcome.** An act on a launch waits until the launch is made; acts on one launch run one at a time in order; acts on different launches side by side. A preview starts once every launch of its pull is under way.
- **Each answer goes up once made.** Each answer comes back over the part's channel and wakes the part, which takes it, starts what waited on it and sends, between its passes.
- **Once.** A want in flight is not taken again; an answer for a lease let go or refused while its command ran is not reported; a command in flight at the stop is left to finish and its answer sent.
- **The turn-taking comes out.** No launch is held `held` for a turn, no act or preview is left to the next pass because the next is due, and no pass sends before each command. `Carrier`'s `due` and `TURN` go.
- `run --once`'s one pass waits for every command it started before it carries the statuses and sends.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `work-into-flume`: each want carried on a thread of its own; a launch in flight holds its room; an act waits only until its launch is made; each answer goes up once made; `run --once` waits for its commands; a want in flight is not taken again; an answer for a lease let go meanwhile is not reported; a command in flight at the stop finishes. The turn-taking and past-due sending requirements are removed.
- `previews`: a preview's command starts once every launch is under way and waits on none; none is left to the next pass.
- `running`: the scenario of acts left for the next pass goes.

## Impact

- `crates/boomdrive/src/hands.rs` (new): the jobs, the scoped threads, the channel and what a part has under way.
- `crates/boomdrive/src/carry.rs`: the pass starts commands and takes answers; `carry_pass`, `collect` and `finish` beside `pass`.
- `crates/boomdrive/src/run.rs`: a part's thread keeps its commands under way across its passes, and its bell rings when an answer comes back.
- `crates/boomdrive/src/flume.rs`, `sluice.rs`: `Flume` and `Sluice` are `Sync`, since their commands run on several threads.
- Tests: `tests/turns.rs` goes; `tests/sends.rs` holds the side-by-side cases; the fakes' Flume is shared across threads and can hold a launch or an act; the stubs claim their call numbers.
- Record 0002 as amended on 2026-11-19, core.50 and core.55 already say this, so no rule or record changes here. Neither the protocol nor crew's implementation changes.

## Touches

```
crates/boomdrive/src/hands.rs
crates/boomdrive/src/carry.rs
crates/boomdrive/src/run.rs
crates/boomdrive/src/lib.rs
crates/boomdrive/src/flume.rs
crates/boomdrive/src/sluice.rs
crates/boomdrive/tests/
```
