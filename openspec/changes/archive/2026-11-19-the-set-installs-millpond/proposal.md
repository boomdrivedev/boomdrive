# Proposal

## Why

The runner's pass runs millpond's tick of each inbox the configuration's `inboxes` names (record 0033; the change the-runners-pass-runs-millponds-tick). A host on the set needs millpond installed, millpond's own host configuration naming its inboxes, and the runner's `inboxes` naming the same, each inbox named once.

## What Changes

- **millpond in the set.** `flake.nix` names millpond at its release tag `v0.1.0`; the set's `millpond` is millpond's own flake's package, its git crates fetched by the hashes `nix/packages/millpond-git-crates.nix` holds, since millpond's flake fetches them at evaluation, which the gate's evaluation against no store cannot.
- **Installed with the runner.** `services.boomdrive.millpond.package`, the set's by default, goes in the system profile; a host's own replaces it, and `null` installs none.
- **millpond's host configuration from one place.** `services.boomdrive.millpond.settings`, millpond's keys (`host`, `secrets`, `declarations`, `inbox`), is rendered at evaluation and linked at `~/.config/millpond/config.toml` at each activation (`activate.sh millpond`); the runner's `inboxes` are its inboxes' names. With no inbox, nothing of millpond's is written.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `family-packages`: the set gives `millpond`.
- `service-modules`: the modules install millpond with the runner and write its host configuration and the runner's `inboxes`.

## Impact

- `flake.nix`, `flake.lock`: the input `millpond`, its `nixpkgs` and `rust-overlay` following the flake's.
- `nix/packages/millpond.nix`, `nix/packages/millpond-git-crates.nix` (new).
- `nix/modules/millpond.nix` (new), imported by `darwin.nix` and `nixos.nix`, which link its file at each activation; `activate.sh` gains `millpond`; `toml.nix` takes its header.
- `nix/tests/millpond.nix` (new); `nix/tests/typecheck.nix` lists `millpond`.
- The family's cache builds and pushes millpond with every package of the set (core.47).

## Touches

```
flake.nix
flake.lock
nix/
README.md
```
