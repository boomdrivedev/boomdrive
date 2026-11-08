# Proposal

## Why

The change a-dropped-launch-is-never-offered wrote its tests and scenarios with the operator's own partition and hosts, and a personal account as its repository's owner, and called them neutral. core.41 keeps them out of the tree. Batch 6's review of that change found it: the change boomdrives-tree-holds-nothing-private, of the same batch, replaces exactly such names, so these tests need the same scrub before the tree goes public (records 0012 and 0014).

## What Changes

- **The drops' tests take the neutral names.** `crates/boomdrive-crew/tests/drops.rs`, and the launch the change added to `read.rs`'s retry of the submission, name the partition `home`, the hosts `host-a` and `host-b` and the repository `example/crew`: the vocabulary of boomdrives-tree-holds-nothing-private's Decision 1.
- **crew's other tests take them with them.** Their ids are built from the label `tests/common` serves, so `tests/common` and crew's other tests take the same names, as boomdrives-tree-holds-nothing-private's commit `ed66819` gives them (design Decision 1).
- **The scenarios the change wrote** take the same names through this change's delta: each requirement whose scenarios it added is modified with the neutral names, and nothing else of it changes. No requirement is added.
- **The archived change** a-dropped-launch-is-never-offered is scrubbed in place to the same names, as boomdrives-tree-holds-nothing-private scrubbed the archived changes before it.
- Nothing is added to the gate: no check, test or task. core.41 stays the rule a reviewer holds every diff to.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `crew-control-plane`: the scenarios of five requirements name the partition `home`, the hosts `host-a` and `host-b` and the repository `example/crew`.

## Impact

- `crates/boomdrive-crew/tests`: `drops.rs`, `read.rs`, `common/mod.rs`, `claims.rs`, `letgo.rs`, `roles.rs`, `shared.rs` and `write.rs`. No test changes what it proves.
- `openspec/changes/archive/2026-11-07-a-dropped-launch-is-never-offered/`: its design, tasks and delta.
- `openspec/specs/crew-control-plane/spec.md`, through this change's delta.
- No crate, tool, tracked file type, task or rule is added, and no behaviour of the runner changes.
- Out of scope: what boomdrives-tree-holds-nothing-private scrubs that a-dropped-launch-is-never-offered did not write, among it the unit tests in crew's sources and the scenarios of the requirements that change reworded but added none to.

## Touches

```
crates/boomdrive-crew/tests/claims.rs
crates/boomdrive-crew/tests/common/mod.rs
crates/boomdrive-crew/tests/drops.rs
crates/boomdrive-crew/tests/letgo.rs
crates/boomdrive-crew/tests/read.rs
crates/boomdrive-crew/tests/roles.rs
crates/boomdrive-crew/tests/shared.rs
crates/boomdrive-crew/tests/write.rs
openspec/changes/archive/2026-11-07-a-dropped-launch-is-never-offered/design.md
openspec/changes/archive/2026-11-07-a-dropped-launch-is-never-offered/tasks.md
openspec/changes/archive/2026-11-07-a-dropped-launch-is-never-offered/specs/crew-control-plane/spec.md
openspec/specs/crew-control-plane/spec.md
```
