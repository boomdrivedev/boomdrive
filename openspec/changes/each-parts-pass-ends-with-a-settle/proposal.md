# Proposal

## Why

Two changes landed side by side on 2026-11-07. `each-pass-ends-with-a-settle`, on `main`, has each turn of the runner's one loop end with Flume's settle of every place, after the keeping pass and the control pass due, so the runner keeps Flume's watch over its host (record 0029). `each-partition-the-runner-serves-runs-on-its-own`, on the bolt `the-runner-serves-each-partition-apart`, carries each part of the control plane on a thread of its own from the end of the first keeping pass, so no part's pull, act, command or report waits on another part's or on a keeping pass (record 0026's amendment of 2026-11-07; core.49).

Taken together, the loop's turn holds the keeping pass alone: a part's control passes run on the part's own thread, on its own timer. The running spec's "Each turn of passes ends with a settle of every place" no longer says when a control pass settles, and a settle on the loop could not follow a part's pass without the part waiting on the loop. The merge of `main` into the bolt runs the settle at the end of each pass on that pass's own thread; this change says so in the spec, the record and the rules.

## What Changes

- **Each pass ends with a settle, on its own thread.** A keeping pass ends with one `flume --json settle --every <bound>` on the loop, and each part's control pass ends with one on the part's thread, before that thread waits. The first keeping pass, which each part's first control pass follows at once, shares their settles, so no launch waits behind a settle. A keeping pass whose configuration did not read settles all the same, and a runner told to stop begins no settle.
- **No two settles at once.** The loop and every part's thread share one watch. A pass that ends while another pass's settle is under way shares that settle and runs none, waiting for nothing.
- **What the runner says of its settles is the host's**, said once when a settle is refused or gives no answer, and once when Flume settles again, whichever thread's settle it was.
- **The bound stays the runner's interval**: the latest `fetch_interval`, or the shorter of it and `poll_interval` while the runner makes control passes.
- **Record 0029 is amended**, and core.13's rule and details, core.1's details and core.49's details read with it.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `running`: each pass, keeping or a part's control pass, ends with a settle of every place on its own thread; the first keeping pass shares the parts' first settles; a pass ending while another's settle is under way shares it.

## Impact

- `docs/adr/0029-…`: an amendment of 2026-11-08. `docs/architecture/core.md`: core.13's rule and details, core.1's and core.49's details.
- `crates/boomdrive/src/run.rs`: `Watch`, the settle shared by the loop and every part's thread, which the merge brought; its test of a settle under way.
- `README.md`: "Running on a host" and the control pass's section say each pass ends with a settle on its own thread.
- No crate, no tool, no tracked file type, no configuration key and no message of the protocol is added.

## Touches

```
docs/adr/0029-each-pass-of-the-runner-ends-with-a-settle-of-every-place-which-keeps-flumes-watch-over-the-host.md
docs/architecture/core.md
crates/boomdrive/src/run.rs
README.md
```
