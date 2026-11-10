# Verify: the-runner-keeps-no-name-from-before-the-rename

The work on `change/the-runner-keeps-no-name-from-before-the-rename` was read at 89ee021. The gate passes, the work meets every requirement and scenario of its change, and every task is ticked, so the change stays archived with this report.

- The gate ran once through sluice against `main` at cee700c. All 13 of the `check-fast` tasks it ran passed, with 753 tests passed and none failed.
- The change's deltas hold 4 requirements and 7 scenarios that it adds or modifies, and 3 requirements that it removes. A test that passed in the gate asserts the outcome of each scenario.
- All 6 tasks are ticked. The code stage archived the change at 89ee021, as task 3.1 says to. This report sits beside the archived change.
- `main` and `bolt/the-runner-keeps-no-name-from-before-the-rename` are both at cee700c, the branch's base, so the branch merges into each with no conflict.

## Checks

- **The gate.** `sluice exec main` at 89ee021 ran the 13 `check-fast` tasks that 89ee021 affects against `main` at cee700c. It exited 0, and sluice recorded the 13 results under their keys for 89ee021.
  - Passed:
    - `boomdrive:format`, `lint`, `typecheck` and `test`;
    - `boomdrive-crew:lint`, `typecheck` and `test`;
    - `boomdrive-hosted:test`;
    - `nix:typecheck` and `test`;
    - `root:format`, `specs` and `changes`.
  - The 753 tests include every test the tasks name.
  - sluice did not run the tasks it judged 89ee021 leaves unaffected against `main`:
    - `boomdrive-control`'s four tasks;
    - `boomdrive-crew:format`, `boomdrive-hosted:format`, `lint` and `typecheck`;
    - `nix:format` and `lint`;
    - `root:toolchain`, `adrs`, `commits`, `releases` and `workflow`.

    The branch changes none of those crates' or the flake's files. It adds no record and changes no workflow or toolchain pin. All 7 commits the branch adds to `main` are Conventional Commits.
  - moon warned that it could not write its shared cache or its output archives outside the sandbox. It also had no internet connection for a dependency install, and the OTLP exporter failed to shut down. None of these touched a task's result.
- The gate was run once, as this stage's brief says, and nothing was fixed here.

## Tasks

All 6 tasks of `tasks.md`, groups 1 to 3, are ticked `[x]`, and none is open. What they build is in the tree, as the design names it:

- **The data directory** (1.1):
  - `config::data_dir` is `xdg(..).join("boomdrive")` and reads nothing on disk. Its doc cites core.35, core.36 and record 0035.
  - `the_data_directory_is_the_runners_own` replaces the unit test of the old folder.
- **A clone's records** (1.2):
  - `clone::carry_over`, its call in `find` and `SECTION` are gone. `find` reads `recorded` alone, and its doc names a folder recording its address under another name as not the runner's.
  - `tests/views.rs`'s `a_clone_made_before_the_rename` is gone.
  - `a_new_root_while_sluice_keeps_the_default_branch` keeps a clone of the runner's own, with every assertion it made.
  - `tests/clones.rs` gains `a_clone_recording_its_address_under_another_name`.
- **The fixtures** (1.3):
  - Each of the 19 JSON fixtures of `flume-0.4.1/` is byte for byte its earlier self with each `"flywheel:scratch/` turned into `"boomdrive:scratch/`, and nothing else.
  - The fixtures' README gives `--mark boomdrive:scratch/<n>` and names the mark as the one word record 0035 rewrote.
  - `acts.rs`, `facts.rs`, `once.rs` and `statuses.rs` read the runner's marks.
- **A launch's mark** (1.4):
  - `carry::marks` and `BEFORE_RENAME` are gone. The offer path asks `facts::marked(listing, &mark(id))` alone.
  - `tests/once.rs` gains `a_mark_the_runner_does_not_give`.
- **The release's notes** (1.1, 1.2, 1.4): 6d78c1f, f507c99 and 1f8cc15 each drop one reading. Each is committed on its own, with the subject its task names, marked breaking. Each body is the chore that tasks.md gives as record 0035's. So git-cliff's notes carry each one under "Breaking changes" (core.36, core.48).
- **README** (2.1): the section "A host from before the rename" is gone.
- **The specs** (3.1): the deltas are archived into `openspec/specs/clones` and `work-into-flume`, and `root:specs` passes 17 of 17.

Outside `docs/adr/`, `docs/architecture/` and `openspec/changes/archive/`, no tracked file says the old name (`git grep -i flywheel`).

Against `main` at cee700c, the branch changes only these paths:

- the files the proposal's Impact lists;
- the change's own files;
- the two specs its archive reconciled.

No crate, tool, tracked file type, task, configuration key, report field or protocol message is added. No record or rule is written.

## Requirements and scenarios

| spec | requirement | scenario | asserted by |
|---|---|---|---|
| clones | A folder the runner did not make is never touched (modified) | A folder already there | `clones.rs` `a_folder_already_there` |
| | | A stranger in the data directory | not new to this change; `dropped.rs` `a_stranger_in_the_data_directory` passed |
| | | A clone recording its address under another name | `clones.rs` `a_clone_recording_its_address_under_another_name` |
| | The runner moves no branch of the clone (modified) | Behind origin; Sluice keeps the branch; Ahead of origin | not new to this change; `clones.rs` `behind_origin`, `sluice_keeps_the_default_branch` and `ahead_of_origin` passed |
| | | A new repository root while sluice keeps the branch | `views.rs` `a_new_root_while_sluice_keeps_the_default_branch` |
| | The data directory is Boomdrive's alone (added) | The data directory is the runner's own | `config::tests` `the_data_directory_is_the_runners_own` |
| | A data directory from before the rename is kept (removed) | — | `data_dir` reads nothing on disk |
| | A clone's records from before the rename are carried over (removed) | — | `carry_over` is gone |
| work-into-flume | The runner acts only on its own places (modified) | The user's own place | not new to this change; `statuses.rs` `the_users_own_place` passed |
| | | A mark the runner does not give | `once.rs` `a_mark_the_runner_does_not_give` |
| | A launch marked before the rename is found by its old mark (removed) | — | `carry::marks` is gone |

`a_launch_found_by_its_mark` and `a_launch_whose_answer_was_lost` (`once.rs`) still prove work-into-flume's "A launch found by its mark", the first over Flume's own listing. Both passed in the gate.

## Outside this repository

Each host moves its own data by hand at its cutover, with its runner stopped and no place live, before it runs the release (record 0035). That chore is ops', and it is not part of this change. The gate's moon cache path on this host still names the old folder, which is that same chore.
