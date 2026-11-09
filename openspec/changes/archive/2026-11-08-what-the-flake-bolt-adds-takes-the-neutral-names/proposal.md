# Proposal

## Why

`main` holds what core.41 and core.6 allow: the change `boomdrives-tree-holds-nothing-private` brought it to one vocabulary of neutral names, and the change `what-main-adds-takes-the-neutral-names` held what `main` added after it to the same names. The bolt `the-boomdrive-flake-configures-the-whole-host` was written before that scrub reached it. Its changes `a-hosts-configuration-names-its-accounts-and-their-drives`, `the-flake-configures-each-tool-by-its-own-module`, `crews-partitions-give-the-runner-its-crew-table`, `each-repository-is-named-once-by-its-owner` and `herdr-and-ghostty-come-with-the-flake-and-an-app-opens-the-agents` name a personal account's repository in the runner's tests and in four requirements' scenarios, and name the operator's host, and in one quote of the user's the operator's employer, in a module's comment, a test and their archived changes. The bolt takes `main` in before it lands, so what it adds takes the same scrub, and nothing private lands with it.

## What Changes

- **The tests** the bolt adds, the unit tests of `crates/boomdrive/src/config.rs` and `crates/boomdrive/tests/planes.rs`, name `example/crew` and the host `host-a`, each test proving the scenario it proved.
- **The module's comment** in `nix/modules/options.nix` says where its defaults come from, record 0028, and names no host.
- **The archived changes** the bolt adds are scrubbed in place to the same names; where one names the operator's host's facts, it says a host's; the user's words on a drive of another machine keep what they decide, and name no host or employer.
- **The specs** take the neutral names through this change's deltas: each requirement the bolt added whose scenarios name a private value is modified with the neutral one, and nothing else of it changes. No requirement is added.
- Nothing is added to the gate: no check, test or task. core.41 stays the rule a reviewer holds every diff to.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `configuration`: the scenarios of *The local layer's repositories add to the managed layer's, and its partitions replace them*, *The managed layer may pin a key the local layer could set* and *The configuration names its accounts, each account's drives and each drive's repositories* name the repository `example/crew`.
- `views`: the scenario of *Each drive has a VS Code workspace at the repository root where the configuration asks* names the repository `example/crew`.

## Impact

- **Tests**: `crates/boomdrive`, each test proving the scenario it proved.
- **Nix**: a comment of `nix/modules/options.nix`; no option or value changes.
- **Docs**: `openspec/changes/archive/`. No record or rule changes.
- **Specs**: the two capabilities above, through this change's deltas.
- No crate, tool, tracked file type, task or rule is added, and no behaviour of the runner or the modules changes.
