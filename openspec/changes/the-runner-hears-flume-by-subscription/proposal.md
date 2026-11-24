# Proposal

## Why

Each control pass of `boomdrive run` reads Flume's status by `flume --json status`, once before its pull and again once its wants are answered. Each read is a new flume process, run on the part's timer, so a host with a runner sees flume and sluice processes come and go every few seconds. The accepted invariants that each tool has one resident process, and that a resident asks another tool only through the socket of that tool's resident, name this as a break. flume host serves Flume's status on its socket, and a subscription that sends a new document each time the status changes (flume's change `flume-status-reads-flume-host`).

## What Changes

- **One subscription to flume host.** While the runner carries a control plane's parts, it holds one connection to flume host's socket, asks `{"ask":"subscribe"}`, and keeps the latest document. Each pass reads Flume's status from that document, and runs no `flume --json status`.
- **A change heard wakes the parts.** Each document that differs from the one held wakes every part, so its control plane hears a change of Flume's places at once, and not at the part's next timer.
- **`--once` asks once.** `boomdrive run --once` asks flume host `{"ask":"status"}` on its socket for each read, and holds no subscription.
- **No flume host, no status.** Where no flume host answers, Flume's status cannot be read: the pass says so, launches nothing and acts as it does today when Flume's status is unread. The subscription is tried again on a bounded backoff, and the runner says one line when flume host stops answering and one when it answers again.
- A place's own status, Flume's launch, its acts and its confirm stay Flume's commands, which flume host does not serve on its socket.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `running`: the runner's one subscription to flume host, and what it says of its reach.
- `work-into-flume`: the pass's listing is Flume's status as flume host gives it.
- `control-plane-interface`: an unread pull runs no `flume --json status`.

## Impact

- `crates/boomdrive/src/flume_host.rs`: the socket, the asks and the subscription. `run.rs`: the subscription's thread and the parts' wake. `Cargo.toml`: `sha1`, for the socket's key as flume makes it.
- The crate's tests: a stub flume host on a scratch socket, which answers from the stub `flume`'s status answer.
- `README.md`, `docs/architecture/core.md` (core.13's details).
- Boom: the decision `the-runner-hears-flume-host-by-one-subscription`, proposed for the user to accept.

## Touches

```
Cargo.toml
Cargo.lock
crates/boomdrive/Cargo.toml
crates/boomdrive/src/flume_host.rs
crates/boomdrive/src/lib.rs
crates/boomdrive/src/run.rs
crates/boomdrive/tests/
README.md
docs/architecture/core.md
```
