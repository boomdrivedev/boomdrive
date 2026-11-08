# Design

## Context

See proposal.md for why. This change brings what a-dropped-launch-is-never-offered wrote to core.41 before the tree goes public. It changes no behaviour of the runner.

What stands, read on 2026-11-07:

- **Where the change's names are.** `crates/boomdrive-crew/tests/drops.rs`, which it added; one launch it added to `read.rs`'s `a_retry_of_the_submission`; the scenarios of five requirements of `crew-control-plane`, four it added and one, "A claim stands until the runner answers its launch", it added a scenario to; and its archived design, tasks and delta. Its README paragraph, its sources and the interface's shared case name nothing private.
- **Where the tests' ids come from.** crew's tests build every id, branch and state repository from the label `tests/common`'s `State::crew` serves. `drops.rs` cannot name `home/12` while `tests/common` serves another label.
- **The scrub.** boomdrives-tree-holds-nothing-private has landed on its own bolt, not on this branch's. Its commit `ed66819` gives `tests/common` and crew's other tests the neutral names; its delta modifies, among others, "A claim stands until the runner answers its launch", in its words before a-dropped-launch-is-never-offered reworded it.

There are no area pages, so `docs/architecture/core.md` binds every path this change touches. It relies on core.41, which says what the tree may hold, and on no other rule.

## Goals / Non-Goals

**Goals:**

- No host, partition or account of anyone's in what a-dropped-launch-is-never-offered wrote: its tests, its scenarios and its archived change.
- Every scenario still proven by the test that proved it, its names the test's.
- Where this branch and boomdrives-tree-holds-nothing-private's meet, the same lines, so the two merge with the least to reconcile.

**Non-Goals:**

- **What the scrub scrubs that a-dropped-launch-is-never-offered did not write.** The unit tests in crew's sources, the interface's tests, the runner's tests, the README's examples, the records and `design/` are boomdrives-tree-holds-nothing-private's, which has landed them on its bolt.
- **A check that the tree holds nothing private**, which would have to hold the names it looks for (boomdrives-tree-holds-nothing-private's Non-Goals).

## Decisions

### 1. crew's tests take the scrub's names as its commit gives them

`tests/common` and crew's other tests take boomdrives-tree-holds-nothing-private's commit `ed66819` hunk for hunk, its one conflict resolved with the launch a-dropped-launch-is-never-offered added to `read.rs`, and `drops.rs` takes the same vocabulary: partition `home`, hosts `host-a` and `host-b`, repository `example/crew`, a host's binding `host_a` or `host_b` as `ed66819`'s tests name theirs.

Where the two branches merge, each file of `ed66819` then holds the same lines on both sides.

Alternatives:

- **Scrub `drops.rs` alone, over a state of its own serving `home`.** Not taken: `drops.rs` would hold a second `State` beside `tests/common`'s, and once the scrub merges, two ways of naming one state repository.
- **Scrub the hosts alone and leave the partition to the merge.** Not taken: the partition's name is in every id the tests read, and the brief asks for the same scrub, whole.
- **Wait for the scrub to reach this bolt and scrub at the merge.** Not taken: whoever merges would scrub tests they did not write, and the brief asks for it here.

### 2. A requirement the delta restates takes the scrub's words for the scenarios the scrub wrote

The delta modifies the five requirements whose scenarios a-dropped-launch-is-never-offered wrote. "A claim stands until the runner answers its launch" holds two scenarios older than that change; the delta restates them in boomdrives-tree-holds-nothing-private's words, word for word, so the merge of the two deltas' results differs only in the requirement's own text, which a-dropped-launch-is-never-offered changed.

The requirements a-dropped-launch-is-never-offered reworded and added no scenario to are left to the scrub's delta.

### 3. The archived change is scrubbed in place

a-dropped-launch-is-never-offered's archived design, tasks and delta take the same names, as boomdrives-tree-holds-nothing-private scrubbed the archived changes before it ("The archived changes are scrubbed in place to the same names and citations, as the history they are"). Its proposal and verify name nothing private and are unchanged.

## Risks / Trade-offs

- **The two branches still meet in `crew-control-plane`'s spec.** a-dropped-launch-is-never-offered reworded four requirements the scrub's delta also restates. Whoever merges the two keeps a-dropped-launch-is-never-offered's requirement text and the scrub's scenarios; Decision 2 leaves nothing else to reconcile in the requirements this change restates.
