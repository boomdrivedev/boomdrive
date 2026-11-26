# Design

## Context

The runner carries sluice's entry of each launch's submission (the tool-records spec). It reads the number at `submission.read.found.submission.id` of Flume's status of the place. It asks `sluice status <id> --json` in the clone's container, at each pass, until the carried entry's state is final.

sluice numbers each branch's queue on its own. Since 0.110, `sluice status <id>` refuses a number that two branches hold, and asks for `--branch`. The runner never gave it. A refusal kept nothing, so the runner asked again at every pass, with no end.

This design relies on core.1, core.4, core.11, core.36 and core.45.

## Decisions

### Decision 1: The branch is Flume's `target`

Flume's status names the submission whole: `id`, `branch`, `target` and `state`. `branch` is the place's own branch. `target` is the branch whose queue holds the submission. sluice's `--branch` names that queue, and sluice's own entry calls it `branch`. So the runner gives Flume's `target` as `--branch`. The runner chooses no branch of its own (core.1). A status that names no `target` names no entry the runner can ask for, so it runs no sluice.

`--branch` is given at every ask, also where one branch holds the number. sluice answers the same there, so there is one form of the ask.

### Decision 2: An entry is its number on its branch

A final entry stops the asks only where its `id` and its `branch`, as sluice printed them, are the number and the branch that Flume names. A unit's submission 18 to a bolt's branch can be final while the bolt's submission 18 to `main` is not.

### Decision 3: A refusal is kept, and the runner asks no more

sluice refuses an ask only when it cannot answer it: no such number on that branch, or an ask it cannot read. The same ask gets the same refusal. So where sluice refuses, the runner keeps the refusal on the launch, `entry_refused`: the number, the branch, sluice's words and when. It says the refusal once. A later pass that finds the same number and branch asks nothing. Where Flume's status names another number or branch, the runner asks for it, and an answer clears the refusal.

Where sluice could not be run at all, or a signal ended it, the ask had no answer. The runner keeps nothing and asks at the next pass, as before. That is a fault of the host, which its line names at each pass.

The field is new, read as none where a report lacks it, and written only where it holds one. So a report of before reads, and a release before ignores it (core.36). The refusal goes to no control plane: the hosted copy and crew's records take the status and the entry alone (core.11).

### Decision 4: The gate runs each of the runner's asks against the set's sluice

The runner's tests of its sluice asks answer through a stub that takes any arguments. The set's sluice was asked only for a number that no branch holds. So the gate passed with sluice 0.92.0 and later, which refuse a number on two branches.

The family test `the_sets_sluice_takes_each_of_the_runners_asks` runs the set's sluice, the one devenv pins, in the runner's clone of a scratch repository. `main` keeps its records on its remote, since the policy names a signer, and `next` keeps its records on the host. So the first submission to each is 1, from two counts, as on the hosts. The test makes the signing key at run time (core.4), and runs `sluice host` as its child, so no process outlives it.

Through `sluice::Cli`, the runner's own reader, the test asks for the entry of 1 on `main` and on `next`, a preview, the previews and a release. A wrapper notes each exit. The test fails where an exit is 64, a command line sluice cannot read, or where sluice refuses an entry. The arguments are the runner's own, not a copy, so a change to them is tested here.

With the ask of before, `status 1 --json`, sluice refuses: "submission 1 is on more than one branch: main, next". So the test fails. A move of sluice's pin in `flake.lock` runs this test in the gate (core.45). So the gate would have refused the move to 0.92.0 until the runner named the branch.

## Risks

- A refusal that a later sluice could answer stays kept. The launch's next submission has a new number, so the runner asks again for it. A host can remove the field from the report to ask once more.
