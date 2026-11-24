# Proposal

## Why

`boomdrive run` ends each pass with `flume --json settle --every <bound>`. Each settle is a new flume process that reads the merge queue, and so starts sluice processes. The accepted invariant `one-resident-per-tool` names this as a break: a tool must do its work in its resident, and not in processes that clients start. flume v0.35.0 has its resident, `flume host`. It settles the places at each change that it hears from sluice host, and one minute after its last settle. Its own module runs it as a service of the host. So the runner does not need to keep the host's watch.

## What Changes

- **The runner starts no settle.** No pass of `boomdrive run` or `boomdrive run --once` runs `flume settle`. The settle's bound, its lines and its lock go.
- **Each pass ends with its ticks.** millpond's tick of each inbox stays at the end of each pass, on the same shared lock. A pass that ends while another pass's ticks are under way shares them.
- **The set takes Flume v0.35.0.** Its module runs `flume host` as the launchd agent `flume-host` on nix-darwin and the user unit `flume-host` on NixOS, wherever it installs Flume. The lock moves the `flume` node alone.
- **core.13 says so.** The runner's only housekeeping is millpond's tick at the end of each pass. Flume's watch over the host is `flume host`'s.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `running`: no settle; each pass ends with its ticks.
- `configuration`: the inboxes' and the control plane's scenarios name no settle.
- `family-packages`: the set's tests ask the set's Flume for no settle.
- `service-modules`: with the runner on, Flume's own module runs `flume host`.

## Impact

- `crates/boomdrive/src/settle.rs` goes; `run.rs`, `flume.rs`, `millpond.rs`, `config.rs` and `lib.rs` lose the settle.
- The tests of the crate that count settles count passes by another sign, or by the ticks.
- `flake.nix`, `flake.lock`: Flume at v0.35.0. `nix/tests`: a case for `flume-host`.
- `README.md`: the pass's end. `docs/architecture/core.md`: core.13 and the rules that name the settle.
- Boom: the decision `the-runner-leaves-flumes-watch-to-flume-host`, proposed for the user to accept.

## Touches

```
crates/boomdrive/src/settle.rs
crates/boomdrive/src/run.rs
crates/boomdrive/src/flume.rs
crates/boomdrive/src/millpond.rs
crates/boomdrive/src/config.rs
crates/boomdrive/src/lib.rs
crates/boomdrive/tests/
flake.nix
flake.lock
nix/tests/
README.md
docs/architecture/core.md
```
