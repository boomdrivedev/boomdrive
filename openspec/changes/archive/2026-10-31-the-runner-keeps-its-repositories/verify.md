# Verify: the-runner-keeps-its-repositories

The work on `change/the-runner-keeps-its-repositories`, read at 4a6ea6c, holds. Every task is ticked. Each of the 31 requirements is met in the code, and each of the 71 scenarios is proved by a test that asserts its outcome. The checks task 7.1 names pass, and so do the gate's other two root checks. The change is archived on this branch with this report.

## Tasks

All 23 tasks of `tasks.md`, groups 1 to 7, are ticked `[x]`, and none is open. Each group's closing task names the checks that ran it. Those checks pass at 4a6ea6c (below).

## Checks

Run through sluice, so the sandbox and its `network = "localhost"` applied:

- `sluice exec main -- moon run flywheel:format flywheel:lint flywheel:typecheck flywheel:test root:format root:toolchain root:changes root:specs root:adrs` passed: 9 tasks. moon replayed flywheel's four tasks from its cache. `root:toolchain` reported rustc, cargo, clippy and rustfmt at 1.98.1, git at 2.55.0 and openspec at 1.14.3. `root:changes` validated this change under `--strict`.
- `sluice exec main -- moon run --force flywheel:format flywheel:lint flywheel:typecheck flywheel:test` ran flywheel's four tasks again without the cache, and all passed. There were 126 tests: 57 unit tests in the library and 1 in the binary; in `tests/`, 13 in `clones`, 14 in `dropped`, 5 in `loop`, 2 in `no_listener`, 1 in `readme`, 13 in `running`, 4 in `scratch` and 16 in `views`.

A trial archive on a copy of `openspec/` created the four specs with their Purpose sections. `openspec validate --specs --strict` passed all four, so `root:specs` holds once they move to `openspec/specs/`.

## Scenarios

Each scenario's test was read against its THEN clause. A test named `x::y` is the function `y` in `crates/flywheel/tests/x.rs`. A test named `src/m.rs` is a unit test of that module.

### configuration (6 requirements, 16 scenarios)

| Scenario | Proved by |
|---|---|
| The default file | `running::the_default_file` |
| The file named | `running::the_file_named` |
| No file | `running::no_file` |
| An address with no owner | `src/config.rs` `an_address_with_no_owner` |
| One repository twice | `src/config.rs` `one_repository_twice` |
| No list | `src/config.rs` `no_list` |
| Keeping none | `src/config.rs` `keeping_none` |
| No views setting | `src/config.rs` `no_views_setting` |
| A home-relative root | `src/config.rs` `a_home_relative_root` |
| A relative root | `src/config.rs` `a_relative_root` |
| No period | `src/config.rs` `no_period` |
| A period of nothing | `src/config.rs` `a_period_of_nothing` |
| A misspelt key | `src/config.rs` `a_misspelt_key` |
| A token | `src/config.rs` `a_token` |
| A repository added while running | `loop::a_repository_added_while_running` |
| A file broken while running | `loop::a_file_broken_while_running` |

### clones (11 requirements, 26 scenarios)

| Scenario | Proved by |
|---|---|
| A new repository is cloned | `clones::a_new_repository_is_cloned` |
| The clone is not under the views root | `views::the_clone_is_not_under_the_views_root` |
| Origin cannot be reached | `clones::origin_cannot_be_reached` |
| A folder already there | `clones::a_folder_already_there` |
| A stranger in the data directory | `dropped::a_stranger_in_the_data_directory` |
| Git asks no one | `src/git.rs` `git_asks_no_one` |
| The host rewrites https to ssh | `clones::the_host_rewrites_https_to_ssh` |
| Origin gains a commit | `clones::origin_gains_a_commit` |
| Origin deletes a branch | `clones::origin_deletes_a_branch` |
| One fetch fails | `clones::one_fetch_fails` |
| Told between passes | `loop::told_between_passes` |
| Told of a repository it does not keep | `loop::told_of_a_repository_it_does_not_keep` |
| Origin's default is not main | `views::origins_default_is_not_main` |
| An empty origin | `views::an_empty_origin` |
| Behind origin | `clones::behind_origin` |
| Held by a worktree | `clones::held_by_a_worktree` |
| Ahead of origin | `clones::ahead_of_origin` |
| Dropped while holding work | `dropped::dropped_while_holding_work` |
| Dropped with nothing of its own | `dropped::dropped_with_nothing_of_its_own` |
| Emptied later | `dropped::emptied_later` |
| The view's folder is not the view any more | `dropped::the_views_folder_is_not_the_view_any_more` |
| A branch the forge lacks | `dropped::a_branch_the_forge_lacks` |
| A change in the view | `dropped::a_change_in_the_view` |
| A commit in the view | `dropped::a_commit_in_the_view` |
| A place in the clone | `dropped::a_place_in_the_clone` |
| Only ignored files | `dropped::only_ignored_files` |

### running (7 requirements, 14 scenarios)

| Scenario | Proved by |
|---|---|
| The version | `running::the_version` |
| No command | `running::no_command` |
| Stopped while waiting | `running::stopped_while_waiting` |
| Passes on the timer | `src/run.rs` `passes_on_the_timer` |
| A failure does not stop it | `loop::a_failure_does_not_stop_it` |
| Every repository kept | `running::every_repository_kept` |
| A clone failed | `running::a_clone_failed` |
| Left for the user | `dropped::left_for_the_user` |
| A second runner | `running::a_second_runner` |
| An old git | `src/git.rs` `an_old_git` |
| A bad configuration at start | `running::a_bad_configuration_at_start` |
| A quiet pass | `running::a_quiet_pass` |
| A view moved | `running::a_view_moved` |
| No listener in the code | `no_listener::no_listener_in_the_code` |

### views (7 requirements, 15 scenarios)

| Scenario | Proved by |
|---|---|
| The first view | `views::the_first_view` |
| A view the user deleted | `views::a_view_the_user_deleted` |
| A folder of the user's | `views::a_folder_of_the_users` |
| Made and moved detached | `views::made_and_moved_detached` |
| Origin moves on | `views::origin_moves_on` |
| The user moved it | `views::the_user_moved_it` |
| Back where the runner put it | `views::back_where_the_runner_put_it` |
| An uncommitted change | `views::an_uncommitted_change` |
| On a branch | `views::on_a_branch` |
| An untracked file | `views::an_untracked_file` |
| A new root, the old view clean | `views::a_new_root_the_old_view_clean` |
| A new root, the old view holding work | `views::a_new_root_the_old_view_holding_work` |
| A new root, the old folder replaced | `views::a_new_root_the_old_folder_replaced` |
| The owner's last view | `dropped::the_owners_last_view` |
| The owner's folder holds more | `dropped::the_owners_folder_holds_more` |

Every test runs under the scratch host of `tests/common`. That host sets `GIT_CONFIG_GLOBAL`, `GIT_CONFIG_NOSYSTEM`, `GIT_CEILING_DIRECTORIES` and a home of its own. It sends `https://github.com/` to a forge on disk and allows no transport but `file`. `scratch::the_scratch_host_reaches_no_network` proves that no test reaches the network.

## Requirements read in the code

The code meets the requirements that no scenario states outright:

- A clone is made in `.<name>.partial` and renamed into place only after its first fetch. A failed clone removes the partial folder, and the owner's folder where that leaves it empty (`clone::make`).
- The runner knows its clones by `flywheel.address`. The address must name the container's own owner and name, compared without case (`clone::recorded`).
- The view is made only by `worktree add --detach` and moved only by `switch --detach`. It is never put on a branch (`view::make`, `view::move_to`).
- The runner makes no checkout of a clone other than its view. A clone has no anchor worktree.
- The start's checks run in design 10's order, and each exits 2 before anything is touched: `HOME`, the configuration, git's version, the data directory, then the lock (`run::Runner::start`). `running::no_home` proves the first.
- The runner says one line when it starts, and `flywheel: stopped` when it stops (`run::Runner::started`, `run::Runner::keep`).

## Rules

- **core.1.** The runner starts no work. A Flume place in a dropped clone holds the clone and is never removed (`dropped::a_place_in_the_clone`).
- **core.2.** Nothing in `crates/flywheel` names crew, its records or a hosted control plane. `example/crew` appears only as an example address in a configuration test. A landing reaches the loop as `Event::Landed`.
- **core.3.** The code has no listener and `Cargo.lock` has no server crate (`no_listener`). The only connections are git's fetches.
- **core.4.** The configuration refuses unknown keys, and a refusal never quotes the file's text (`a_token`, `a_broken_file_is_named_by_its_line_and_never_quoted`). git runs with `GIT_TERMINAL_PROMPT=0`, without either askpass variable, and with no `credential` argument. The string `ghp_n0t4r34lt0k3n` in `src/config.rs`'s tests is a stand-in that proves a value is never echoed. It is not a credential, and it is too short to match GitHub's token format.
- **core.5.** Its detail was amended in 229a870, the same commit as the design that departs from it, as `CLAUDE.md` requires. A dropped clone is never fetched (`dropped::dropped_while_holding_work`). It is removed once nothing is held, and left and named while anything is.
- **core.6.** Nothing commercial appears in the diff.
- **core.7.** No crate and no file type was added. `root:format` passed over every tracked file. `flywheel:test` names `/README.md` among its inputs.
- **core.8.** git is pinned at 2.55.0 in `devenv.nix`, asserted against `pkgs.git.version` and checked by `check-toolchain.sh`. openspec ran at the pinned 1.14.3, not the host's 1.15.0.

## Observations

None of these holds the change back.

- `clone::is_partial` and the pass's last step remove any `.<name>.partial` folder under `repositories/<owner>/` that is not a clone. The spec says a folder the runner did not make is never removed. The code reads that name as the runner's own, as design 4 and 9 do, and `clones::a_clone_left_unfinished_is_made_again` proves the behaviour.
- `openspec archive` warns, without blocking, that a change with more than 10 deltas could be split. This one has 31 requirements across 4 new capabilities.
- moon warned that the sandbox could not write its output archives or shared cache. The tasks' results were unaffected.
