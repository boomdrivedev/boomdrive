# Verify: the-runner-keeps-its-clones-packed

The work holds, and the change is archived. The gate passes on `8fa904a`. All eleven tasks are ticked, and every scenario of the delta spec is proven by a test that passed in the gate. The test of "An agent's commit starts no collection" now fails where git's collection is on: a probe that left both keys unset in a new clone made it fail.

This report is on `change/the-runner-keeps-its-clones-packed` at `8fa904a`, cut from `bolt/the-runners-copies-stay-small-and-fast` at `3f7d69b`, which is `main`. git is 2.54.0.

## The whole checks

`devenv shell -- sluice exec main` ran once on `8fa904a`. It ran the checks that commit affects against `main` (`3f7d69b`) and exited 0. All 9 passed, and sluice recorded their results under their keys for `8fa904a`:

- `boomdrive`: `format`, `lint`, `typecheck` and `test`;
- `nix`: `typecheck` and `test`, each saying "every case holds";
- `root`: `format` ("formatted 411 files (0 changed)"), `adrs` ("48 records, each with a number of its own") and `changes` ("1 open, each valid under openspec validate --strict and archiving to specs that are").

`boomdrive:test` ran 45 test binaries and 721 tests, each `ok`, none failed or ignored. Among them are `clone::tests::an_occasion_comes_past_either_bound` and the 14 tests of `tests/packs.rs`. Those are every test the tasks name.

moon gave two kinds of warning, and no check failed because of either. It could not write its output archives under `.moon/cache/outputs`, nine times. It also skipped the Rust dependency install for want of a network. Both come from the sandbox.

`root:changes` archived the open change into a scratch copy of the specs and held them to `openspec validate --strict` (core.61). That is task 6.1's check.

## Tasks

All eleven tasks are ticked, 1.1 to 7.1, and none is open:

- Tasks 1.1 to 4.1 and 7.1 each name `cargo test` runs of tests that passed in `boomdrive:test` above.
- Task 5.1 names two greps of `README.md`, and each prints a line.
- Task 6.1's check is `root:changes`, above.

The diff from `3f7d69b` stays inside the files proposal.md's Impact lists and the change's own directory: `clone.rs`, `pass.rs`, the new `tests/packs.rs`, `README.md`, record 0051 and `core.md`. It adds no crate, tool, tracked file type or key of the configuration. Record 0051 and core.14 as amended, with core.13's details, are in the change's first commits, `7fa1bbd` and `343e739`. The code and tests cite them and change neither.

## The delta, scenario by scenario

### Git's own collection is off in every clone

| Scenario | Proven by |
|---|---|
| A new clone's collection is off | `a_new_clones_collection_is_off`: the bare repository's own config gives `gc.auto` as `0` and `maintenance.auto` as `false`. `make_in` sets both with the clone's other keys, before its first fetch |
| A clone made before is turned off at its next pass | `a_clone_made_before_is_turned_off_at_its_next_pass`: from `gc.auto` at `6700` and no `maintenance.auto`, the next pass leaves `0` and `false` and says no line |
| An agent's commit starts no collection | `an_agents_commit_starts_no_collection`: in a clone holding 10,000 loose objects, a commit in a worktree runs any collection it starts in the foreground, by `git -c gc.autoDetach=false -c maintenance.autoDetach=false commit`. Afterwards every loose object is still loose, the commit's among them, and the packs are as before |
| A clone whose config cannot be written | `a_clone_whose_config_cannot_be_written`: with a `config.lock` made, the pass fails, a line reads `agentplot/flume: git's own collection not turned off: <git's words>`, and `refs/remotes/origin/main` is origin's new commit |

The test of the agent's commit was probed. `make_in` was made to set neither key, and the test was run alone with `devenv shell -- cargo test -p boomdrive --test packs -- an_agents_commit_starts_no_collection`. The commit then collected, and the test failed at `loose_after.is_superset(&loose_before)`. The probe was then undone, and the tree is as committed.

The requirement's "writing neither where the config already says so" has no scenario. The code holds it: `clone::turn_collection_off` reads each key by its type and skips the write where the read gives the value that turns collection off.

### A keeping pass packs a clone once what arrived passes git's bound

| Scenario | Proven by |
|---|---|
| Nothing is packed at the bound | `nothing_is_packed_at_the_bound`: with 50 packs, the loose objects and packs are the same after the pass, and no line is said |
| Loose objects past the bound | `loose_objects_past_the_bound`: with 7,003 loose objects and 2 packs, the one line is `packed <n> loose objects and <m> packs`. After the pass the clone holds no loose object, at most two packs and `objects/info/commit-graph`, and every object it held before |
| Packs past the bound | `packs_past_the_bound`: with 51 packs, the one line names 51 packs, and after the pass no loose object and at most two packs remain |
| A dropped repository is not packed | `a_dropped_repository_is_not_packed`: `wip` holds a commit origin lacks, so the clone is left, held. It is not said packed, and its loose objects and packs are as before |

The bounds are proven at and past each one by the unit test `an_occasion_comes_past_either_bound`. The packing round, `pack_kept`, runs after the repositories are kept and let go of, the workspaces kept and the partial clones cleared, as the requirement has it.

### A pack drops only what nothing of the clone reaches, past two weeks

| Scenario | Proven by |
|---|---|
| An old object nothing reaches is dropped | `an_old_object_nothing_reaches_is_dropped`: the blob set back 21 days is gone, and the one written that day is kept |
| What a place and sluice hold is kept | `what_a_place_and_sluice_hold_is_kept`: five objects are each set back 21 days, and every one is still in the clone. They are a place's commit, its tree, its committed blob, a blob in its index alone, and `refs/sluice/heads/x`'s commit |

### A pack touches no ref, log or worktree

| Scenario | Proven by |
|---|---|
| Refs, logs and worktrees are left as they were | `refs_logs_and_worktrees_are_left_as_they_were`: after the pack, `refs/sluice/heads/x` is still a file of its own, its log of one year-old entry is byte for byte the same, there is no `packed-refs`, and `git worktree list` still names the worktree whose folder was removed |

### Packing reads the runner's stop and lets a landing in

| Scenario | Proven by |
|---|---|
| A landing told while the pass packs | `a_landing_told_while_the_pass_packs`: `agentplot/timber`'s line "view moved to origin/main at <landing>" comes after `agentplot/flume: packed` and before `agentplot/sluice: packed` |
| A stop while the pass packs | `a_stop_while_the_pass_packs`: `agentplot/flume` is said packed and `agentplot/sluice` is not, and its loose objects and packs are as before |

A pack in flight runs to its end. `clone::pack` runs its three commands by `Git::run`, which has no limit, as a git call that reaches no remote does (core.58).

### A pack that fails is named, and the pass goes on

| Scenario | Proven by |
|---|---|
| One pack fails | `one_pack_fails`: with `agentplot/flume`'s `objects/pack` at mode 555, the pass fails, a line reads `agentplot/flume: not packed: <git's words>`, and `agentplot/sluice` is said packed. Once the mode is 755 again, the next pass packs `agentplot/flume` |

## Outcome

The checks pass and the work meets every requirement and scenario of the delta. The change is archived on `change/the-runner-keeps-its-clones-packed` with this report, by `openspec archive the-runner-keeps-its-clones-packed --yes`.
