# Design

## Context

See proposal.md for why. The tree holds what core.41 and core.6 allow once the change `boomdrives-tree-holds-nothing-private` landed on the bolt. `main` has since added three changes written before that scrub reached it. Taking `main` in, read on 2026-11-08 at `e82cd25`:

- **Where the private names are.** In what `main` added: the runner's tests `bolts.rs` and `commands.rs`, a unit test of `carry.rs`, the tests the merge resolved (the interface's `json.rs`, crew's `claims.rs`, the runner's `control.rs`), the archived changes `a-bolt-is-a-launch` and `a-dropped-repository-is-drained-and-no-rename-is-followed`, record 0029's sources, and the specs `work-into-flume`, `crew-control-plane` and `commands`.
- **What holds nothing private.** Flume 0.17.1's fixture, read on a scratch host under `env -i`; the archived change `each-pass-ends-with-a-settle`; records 0002's and 0003's amendments, which cite the cloud's records by id and say what they decide in this repository's own words.

There are no area pages, so `docs/architecture/core.md` binds every path this change touches. It relies on core.41 and core.6, which say what the tree may hold.

## Goals / Non-Goals

**Goals:**

- Nothing `main` adds lands with a host, partition, state repository, person's path or account of anyone's.
- The specs reconciled through this change's deltas, and every scenario still proven by the test that proved it.

**Non-Goals:**

- **A check that the tree holds nothing private**, for the reason the scrub's design gives: it would hold the names it looks for.
- **The specs' purposes.** They cite afterthought/swancloud's rules by name and id, which core.41 allows, and stay as the scrub left them.

## Decisions

### 1. The scrub's vocabulary, unchanged

What `main` adds takes the names of the scrub's design Decision 1: the partition `home`, and every id, branch, stream and mark built from its label (`home/14`, `home/bolt/3`, `home/runner/host-a`, the stream `home`, `boomdrive:home/14`); the host `host-a`; crew's old address `example/crew`; a state repository `example/crew-state`. A test that names both a stream and a pool keeps them apart, as the scrub's `pool-a` does; none of `main`'s does.

Alternatives:

- **Names fresh to these tests.** Not taken: two vocabularies for one thing.

### 2. crew's follow-up names crew's repository

The archived design of `a-bolt-is-a-launch` names crew's follow-up C1 under crew's repository. It is crew the tool's, so it names agentplot/crew, crew's public repository, as record 0023 names crew's flake.

Alternatives:

- **`example/crew`.** Not taken: that name stands for a repository a test keeps, and the follow-up is crew's own.

### 3. A planner's signal is cited by its date

Record 0029's sources named a planner's signal by its id in a state repository. As the scrub's design Decision 4 has it, a state repository is no citation: the source becomes "a planner's signal of 2026-11-06", as records 0026 and 0027 cite theirs.

### 4. The specs take the names through this change's deltas

Each requirement `main` added whose scenarios name a private value is modified with Decision 1's names, and nothing else of it changes: two of `work-into-flume`'s, two of `crew-control-plane`'s and one of `commands`'. Each scenario is proven by the test that proved it, which takes the same names.

Alternatives:

- **The spec files edited in the merge.** Not taken: a spec changes only through an OpenSpec change (`openspec/config.yaml`; CLAUDE.md), and the merge keeps `main`'s spec text as `main` has it.

## Risks / Trade-offs

- **A missed name.** Nothing in the gate finds one. The verify stage reads the tree.
