# Design

## Context

See proposal.md for why. What stands, once `main` is taken into the bolt:

- `Runner::keep` (`crates/boomdrive/src/run.rs`) runs the keeping passes and the landings on its loop, and from the end of the first keeping pass carries each part on a thread of its own through `Host::carry_on`, each with its own timer, wake, reach and terminals (record 0026).
- `Runner::once` runs one keeping pass, then one control pass of each part side by side, and waits for them all.
- `settle::bound`, `settle::every` and `Settles::heard` (`crates/boomdrive/src/settle.rs`) give the bound, its argument and what the runner says of its settles; `flume::Cli::settle_every` runs the settle (record 0029).
- The runner's `Programs` are the runner's, set by `Runner::tools` or `carry_for`.

## Decisions

### 1. Each pass ends with a settle, on its own thread

A keeping pass ends with the settle on the loop, after the pass and before the loop waits. A part's control pass ends with the settle on the part's thread, after the pass and before the part waits: the part's next pass is timed from its pass's start, as before, so a settle longer than the interval leaves the next pass due at once. The settle is no part of the control pass: no want's answer turns on it, and its answer goes to no control plane.

The first keeping pass, which each part's first control pass follows at once, runs no settle of its own: the parts' settles end it, so no launch waits behind a settle of every place (record 0029's reason for sharing). A keeping pass after it settles on its own, since with `fetch_interval` shorter than `poll_interval` the parts' settles would come too seldom for the bound.

### 2. No two settles at once

The loop and every part's thread share one `Watch`, which holds whether the last settle was answered behind a lock, and the latest `fetch_interval`. A pass takes the lock without waiting. Where it is held, another pass's settle is under way: the watch's freshness is when its newest settle began, which was within the bound, so the pass shares that settle and runs none. It waits for nothing, so no part waits on another part's settle or on the loop's (core.49).

A place a pass moved after the shared settle read it moves at the next settle, within the bound, as any place does under a watch (Flume's 208).

### 3. What is said is the host's

`Settles` lives in the shared `Watch`, so a refusal is said once, and Flume settling again once, whichever thread's settle it was (core.23).

### 4. The bound

The loop takes the `fetch_interval` of each reading of the configuration into the watch. Every settle's bound is that, or the shorter of it and `poll_interval` while the runner makes control passes: while it carries one part or more.

## Alternatives

- **The settle on the loop alone, on the shorter interval.** The settle on a timer of its own, between passes, which record 0029 and core.13 rule out.
- **Each part's settle waiting on the one under way.** Every part would wait on a settle another began, so a slow settle would delay every part's next pass: the shared line core.49 rules out.
- **Settles run at once.** With many parts, as many settles of every place at once, each the cost of a host's full status read, for no watch the first does not keep.

## Risks

- A settle shared across threads began before the sharing pass's moves; those move at the next settle, within the bound.
