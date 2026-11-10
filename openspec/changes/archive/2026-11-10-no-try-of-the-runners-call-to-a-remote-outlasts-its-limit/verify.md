# Verify: no-try-of-the-runners-call-to-a-remote-outlasts-its-limit

The work on `change/no-try-of-the-runners-call-to-a-remote-outlasts-its-limit` was read at f6343cc. The gate passes, the work meets every requirement and scenario of its change, and every task is ticked, so the change stays archived with this report.

- The gate ran once through sluice against `main` at b574550. Every `check-fast` task passed, with 877 tests passed and none failed.
- The change's deltas add 4 requirements and 9 scenarios. A test that passed in the gate asserts the outcome of each scenario.
- All 11 tasks are ticked. The code stage archived the change at f6343cc, as task 5.1 says to. This report sits beside the archived change.
- `main` and `bolt/no-try-of-the-runners-call-to-a-remote-outlasts-its-limit` are both at b574550, the branch's base, so the branch merges into each with no conflict.

## Checks

- **The gate.** `sluice exec main` at f6343cc ran the `check-fast` tasks that f6343cc affects against `main` at b574550. It exited 0, and sluice recorded 37 results under their keys for f6343cc. moon completed 39 tasks.
  - Passed, every project's tasks:
    - `boomdrive-bounded:format`, `lint`, `typecheck` and `test`, the new project's;
    - `boomdrive`, `boomdrive-control`, `boomdrive-crew` and `boomdrive-hosted`: `format`, `lint`, `typecheck` and `test` each;
    - `nix:format`, `lint`, `typecheck` and `test`;
    - `root:format`, `toolchain`, `adrs`, `specs` (22 of 22), `changes`, `commits`, `releases` and `workflow`;
    - `site:install`, `check`, `prose`, `vocab`, `build`, `test` and `workflow`.
  - The 877 tests include every test the tasks name.
  - moon warned that it could not write its shared cache or its output archives outside the sandbox. It also had no internet connection for a dependency install. None of these touched a task's result.
- The gate was run once, as this stage's brief says, and nothing was fixed here.

## Tasks

All 11 tasks of `tasks.md`, groups 1 to 5, are ticked `[x]`, and none is open. What they build is in the tree, as the design names it:

- **The crate that holds the limit** (1.1 to 1.4):
  - `crates/boomdrive-bounded` is a library crate with `publish = false` and `doctest = false`. It depends on the workspace's `libc` alone, and is a workspace member, with `Cargo.toml`'s comment naming it and `Cargo.lock` moved with it.
  - `TRY_FOR` is 30 s and `GRACE` 2 s, each documented as agentplot/sluice record 0109's. The crate's documentation cites record 0109 and sluice's `bounded.rs` at `e6809f9`.
  - `output` spawns with `process_group(0)` and reads each pipe on a thread of its own. At the limit it sends `SIGTERM` to the group by `killpg`, waits `GRACE`, then sends `SIGKILL`, and then waits no more than 200 ms for the pipes. `Ended` holds `status`, `stopped`, `stdout` and `stderr`.
  - `stopped` and `span` give `30s`, or `300ms` under a second.
  - The six unit tests the tasks name are present and passed.
  - `moon.yml` declares `rust` and `library`, with four `check-fast` tasks, each one check of `-p boomdrive-bounded`, run from the workspace root and taking the inputs `boomdrive-control`'s take. `.moon/workspace.yml` lists the project, and its header comment says what it holds.
- **A clone's fetch** (2.1 to 2.3):
  - `boomdrive` depends on `boomdrive-bounded`, its `Cargo.toml` comment citing record 0042. Each of its `moon.yml` tasks takes the crate's sources.
  - `Git` carries a private `try_for`, set to `TRY_FOR` by `new` and `default`. `trying_for` gives a git with another limit. `call_remote` runs through `boomdrive_bounded::output`.
  - A stopped call fails with git's last line, then `; `, then the stopped words, and its standard output is not read. A call that ends by itself is read as `run`'s is. `git.rs`'s module documentation cites record 0042.
  - `clone::fetch` runs `fetch --prune --quiet origin` by `call_remote(At::GitDir(bare), "origin", …)`. `not_by_ssh`'s `ls-remote --get-url` stays a `run`.
  - `Scratch::silent` and `Scratch::pass_trying_for` are as design Decision 6 has them. The silent remote is a loopback `TcpListener`, and the address is rewritten over `http` by `insteadOf`, so no test reaches the network.
- **A state repository's listing** (3.1, 3.2):
  - `boomdrive-crew` depends on `boomdrive-bounded`, its `Cargo.toml` comment citing record 0042. Each of its `moon.yml` tasks takes the crate's sources.
  - `Store` gains `pub try_for` and `Store::new`, which `Crew::store` now uses.
  - `Store::hosts` runs its `ls-remote` through `boomdrive_bounded::output`. A stopped listing fails with `stopped("git ls-remote", &self.url, try_for)` after git's last line where git wrote one. A listing that ends by itself is read as before.
  - `the_crates_depend_as_decision_1_has_them` has `boomdrive` depending on `boomdrive-bounded` beside the interface and the implementations, `boomdrive-crew` on `boomdrive-control` and `boomdrive-bounded`, and `boomdrive-bounded` on none of the tree's crates.
- **What the runner says of it** (4.1): README's pass says a fetch, a new clone's among them, is stopped at thirty seconds, citing record 0109. Its crew section says the same of the listing of runner branches. README's and CLAUDE.md's lists of projects name `boomdrive-bounded`.
- **The specs** (5.1): the deltas are archived into `openspec/specs/clones` and `crew-control-plane`, and `root:specs` passes 22 of 22.

Record 0042 and core.58, with its details, are in the tree as the plan wrote them. Outside test code, the only git calls that reach a remote are `clone::fetch`'s fetch and `Store::hosts`'s listing, and both now run through `boomdrive-bounded`. Every other call reaches no remote and still runs to its end (design Decision 4).

Against `main` at b574550, the branch changes only these paths:

- the files the proposal's Touches lists;
- the change's own files;
- the two specs its archive reconciled.

The one crate added is `boomdrive-bounded`. No tool, tracked file type, configuration key, `report.json` field, crew file or protocol message is added.

## Requirements and scenarios

| spec | requirement | scenario | asserted by |
|---|---|---|---|
| clones | No try of a fetch outlasts its limit | What a call started ends with it | `boomdrive-bounded` `a_process_that_never_ends_is_ended_with_its_group` |
| | | A call asked to end is given its grace | `boomdrive-bounded` `a_process_asked_to_end_says_so_in_its_grace` |
| | | A call that will not end is killed | `boomdrive-bounded` `a_process_that_will_not_end_is_killed` |
| | | A call that ends within its limit | `boomdrive-bounded` `a_process_that_ends_gives_its_exit_and_all_it_printed`; `git::tests` `a_call_to_a_remote_stopped_at_its_limit`, its last two cases |
| | | The limit is thirty seconds | `git::tests` `a_git_tries_for_the_familys_limit` and `boomdrive-bounded` `the_limit_and_grace_are_sluices` |
| | A fetch stopped at its limit is named, and the pass goes on | A fetch whose origin never answers | `clones.rs` `a_fetch_whose_origin_never_answers` |
| | | A clone whose origin never answers | `clones.rs` `a_clone_whose_origin_never_answers` |
| crew-control-plane | No listing of the runner branches outlasts its limit | The listing's limit is thirty seconds | `read.rs` `a_stores_listing_tries_for_thirty_seconds` |
| | A listing stopped at its limit leaves its partition unread | A state repository that never answers the listing | `read.rs` `a_state_repository_that_never_answers_the_listing` |

The words of a stopped call, with and without a line of git's before them, are asserted by `git::tests` `a_call_to_a_remote_stopped_at_its_limit` and `boomdrive-bounded` `a_stopped_calls_words_name_its_call_remote_and_limit`.

`Crew::others` passes a failed listing up, so its part is unread with the listing's words, as for any listing that fails. `read.rs`'s `a_state_repository_that_cannot_be_reached_is_named_on_one_line` and `a_partition_read_again_once_it_can_be` still prove this, and both passed in the gate.

The clones spec's other scenarios still pass in `clones.rs`, `one_fetch_fails` and `origin_cannot_be_reached` among them.

## Outside this repository

Design Risk 1 and record 0042 name weir-branch's fetch and push of crew's state repository, and Flume's fetch, as calls to a remote that still have no limit. They belong to their own tools, and this change leaves them as they are. Record 0042 tells the orchestrator about them, beside record 0109's own note of Flume's fetch.
