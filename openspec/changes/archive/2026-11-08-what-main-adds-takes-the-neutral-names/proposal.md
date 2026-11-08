# Proposal

## Why

The bolt that opens the repository (records 0012, 0014, 0025) takes `main` in before it lands, and whatever `main` newly adds takes the same scrub as the bolt's tree, so nothing private lands with it. The change `boomdrives-tree-holds-nothing-private` brought the tree to core.41: every host, partition, state repository and account in its code, tests, specs and docs takes one vocabulary of neutral names (its design Decision 1). Since then `main` has landed the changes `a-bolt-is-a-launch`, `each-pass-ends-with-a-settle` and `a-dropped-repository-is-drained-and-no-rename-is-followed`, whose new scenarios, tests, archived changes and record name the operator's partition and host and a personal account's repository, and cite a planner's signal by its id in a state repository.

The merge takes `main`'s tests and archived changes and record 0029 with the neutral names in their own commits. The specs change only through an OpenSpec change, so their new scenarios take the neutral names through this change's deltas.

## What Changes

- **The specs** take the neutral names through this change's deltas: each requirement `main` added whose scenarios name a private value is modified with the neutral one, and nothing else of it changes. No requirement is added.
- **The tests** `main` added, `crates/boomdrive/tests/bolts.rs` and `commands.rs` and the unit tests of `crates/boomdrive/src/carry.rs`, and those in the files the merge resolved, name `example/crew`, ids and streams under `home`, and the hosts `host-a` and `host-b`.
- **The archived changes** `main` added are scrubbed in place to the same names, and crew's follow-up names crew's own repository, agentplot/crew.
- **Record 0029** cites the planner's signal by its date, with no state repository or signal id.
- Nothing is added to the gate: no check, test or task. core.41 stays the rule a reviewer holds every diff to.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `work-into-flume`: the scenarios of *A bolt is a launch* and *A bolt's send is a retry of its submission* name ids and the stream under `home` and the repository `example/crew`.
- `crew-control-plane`: the scenarios of *A launch the runner answered is given while the runner keeps or drains its repository* and *A launch naming this host takes its repository from those kept, then those drained* name the partition `home` and the host `host-a`.
- `commands`: the branch command's scenario names `example/crew`.

## Impact

- **Specs**: the three capabilities above, through this change's deltas.
- **Tests**: `crates/boomdrive`, `crates/boomdrive-crew` and `crates/boomdrive-control`, each test proving the scenario it proved.
- **Docs**: record 0029 and `openspec/changes/archive/`. No rule changes.
- No crate, tool, tracked file type, task or rule is added, and no behaviour of the runner changes.
