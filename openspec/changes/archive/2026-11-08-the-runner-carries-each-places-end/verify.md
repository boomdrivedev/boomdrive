# Verify: the-runner-carries-each-places-end

The work on `change/the-runner-carries-each-places-end` was read at 02bc77c. The gate passes, the work meets every requirement and scenario of its change, and every task is ticked, so the change stays archived with this report.

- The gate ran once through sluice against `main` at bbf088a. All 13 of the `check-fast` tasks it ran passed, with 498 tests passed and none failed or ignored.
- The change has 10 requirements and 24 scenarios: 14 are new, and 10 are named by the change but not new to it. A test that passed in the gate asserts the outcome of each scenario.
- All 23 tasks are ticked. The code stage archived the change at 02bc77c, as task 8.1 says to. This report sits beside the archived change.
- The branch holds `main` at bbf088a, merged at f2cef6c. It also holds `bolt/the-runner-carries-each-places-end` at c6eb79b, which is an ancestor of bbf088a. So the branch merges into the bolt and into `main` with no conflict.

## Checks

- **The gate.** `sluice exec main` at 02bc77c ran the 13 `check-fast` tasks that 02bc77c affects against `main` at bbf088a. It exited 0, and sluice recorded the 13 results under their keys for 02bc77c.
  - Passed:
    - `boomdrive:format`, `lint`, `typecheck` and `test`;
    - `nix:format`, `lint`, `typecheck` and `test`;
    - `root:format`, `adrs`, `specs` (17 of 17), `changes` and `releases`.
  - The 498 tests include every test the tasks name, and `family.rs`'s tests against the set's own Flume `v0.20.0`.
  - sluice did not run the tasks it judged 02bc77c leaves unaffected against `main`:
    - `boomdrive-control`'s, `boomdrive-crew`'s and `boomdrive-hosted`'s;
    - `root:toolchain`, `root:commits` and `root:workflow`.

    The branch changes none of those crates' or tasks' inputs against `main`. `root:commits` declares no inputs. The 20 commits the branch adds to `main` are all Conventional Commits.
  - moon warned that it could not write its shared cache or its output archives outside the sandbox. It also had no internet connection for a dependency install, and the OTLP exporter failed to shut down. None of these touched a task's result.
- The gate was run once, as this stage's brief says, and nothing was fixed here.

## Tasks

All 23 tasks of `tasks.md`, groups 1 to 8, are ticked `[x]`, and none is open. What they build is in the tree, as the design names it:

- **The set's Flume** (1.1, 1.2):
  - Record 0030 (`proposed`), core.54, and the amended details of core.1, core.13 and core.15 are in the tree.
  - `flake.nix` and `flake.lock` name Flume `v0.20.0` (`9ab0e2c`).
  - `nix/packages/flume-git-crates.nix` holds Flume's git crates at that tag.
  - `flume::LEAST` is `(0, 20, 0)`.
  - The rule is core.54, since `main`'s land holds core.53. Every citation of this change's rule names core.54: the record, the change's files, the specs, `flume.rs`, `carry.rs` and the tests. The core.53 citations left in the tree are the land's.
- **Flume's command line** (2.1 to 2.4):
  - `flume::RECIPIENT` is `"boomdrive"`. `flume::recipients` names it after a launch's own recipients, never twice, and names none where the launch names none.
  - `Flume::collect` runs `--json inbox <record> --recipient boomdrive`, naming no repository.
  - `flume::runs_with` reads the first line of `--version`, ignores a suffix after the patch, and gives design Decision 5's words.
- **Launches** (3.1, 3.2): every launch argv asserted whole holds `--recipient boomdrive` after the launch's own recipients. A bolt and its unit name no recipient, and none of the runner's.
- **Collection** (4.1 to 4.8):
  - `facts::pending_for` is true only for a `tidied` phase whose `pending_for` names the recipient.
  - `Carrier::pass` carries statuses, entries and previews. Then it runs `Work::collect` only where `work.save` holds, and before the report is sent.
  - `Work::collect` goes through the pull's offers in order. It collects only for:
    - a launch the report holds, answered `launched` at a place;
    - in a clone that stands, named or dropped;
    - whose carried status is listed and pending for the runner.

    A refused collection, or one Flume could not run, is said in one line and made again at the next pass.
  - Task 4.8: `tests/statuses.rs` compares with `std::slice::from_ref` of the record or the refusal, and `boomdrive:lint` passes.
- **The drain** (5.1, 5.2): `report::ended` reads a tidied or gone place as ended only where its record is not pending for `flume::RECIPIENT`.
- **The least Flume** (6.1 to 6.3):
  - Each part keeps its own `Least` beside its `Reach`: on its thread in `Host::carry_on`, and in `run --once`'s scoped threads. `Bench` and `tests/pass.rs` keep one too.
  - `Least::read` says `boomdrive: <label>: …` only where the words differ from those last said, and clears them once Flume runs with the least. A `Missing` version changes nothing.
  - `Work::offer` returns, below the least, after the hold and room checks and before any launch, lock or room taken. So a new offer stays unanswered and a held one stays held.
- **README** (7.1) says all of this in the control pass's section, beside `main`'s previews.
- **The specs** (8.1): the deltas are archived into `openspec/specs/work-into-flume`, `clones` and `family-packages`, and `root:specs` passes 17 of 17.

Against `main` at bbf088a, the branch changes only these paths:

- the files the proposal's Touches lists;
- the change's own files;
- the three specs its archive reconciled.

No crate, tool, tracked file type, configuration key, report field or protocol message is added. `crates/boomdrive-control`, `-crew` and `-hosted` are unchanged.

## Requirements and scenarios

| spec | requirement | scenario | asserted by |
|---|---|---|---|
| work-into-flume | The runner is one more recipient of a launch that names recipients, and no brief is left | A launch's environment and brief | `offers.rs` `a_launchs_environment_and_brief`; `flume.rs` `flume_runs_with_no_recipient_of_the_runners_own` |
| | | The runner is one more recipient | `offers.rs` and `flume.rs` `the_runner_is_one_more_recipient`; `flume::tests` |
| | A launch is one `flume launch` naming what the launch gives | A unit's launch | `offers.rs` `a_units_launch`; `flume.rs` `a_launch_with_every_flag` |
| | | A launch naming no stream, priority or intent | `offers.rs` and `flume.rs` `a_launch_naming_no_stream_priority_or_intent` |
| | | the bolt's scenarios (task 3.2) | `bolts.rs` `a_bolt_and_its_unit`, `a_bolt_counts_against_its_streams_room_within_its_pass` |
| | A place's end is held for the runner until its report holds it | A landing held for the runner | `statuses.rs` `a_landing_held_for_the_runner`; `once.rs` `a_held_record_is_collected_once_its_report_holds_it` |
| | | A drop held for the runner | `statuses.rs` `a_drop_held_for_the_runner` |
| | Only a record pending for the runner is collected | A record pending for a recipient too | `statuses.rs` `a_record_pending_for_a_recipient_too` |
| | | A record pending for no end of the runner's | `statuses.rs` `a_record_pending_for_no_end_of_the_runners` |
| | | A lease refused before its end was collected | `leases.rs` `a_lease_refused_before_its_end_was_collected` |
| | A collection Flume refuses or cannot run is made again | A collection Flume could not run | `statuses.rs` `a_collection_flume_could_not_run` |
| | | A collection Flume refuses | `statuses.rs` `a_collection_flume_refuses` |
| | What waits on a place reads its landing from the record Flume holds | A dependent of a landing held for the runner | `after.rs` `a_dependent_of_a_landing_held_for_the_runner` |
| | No launch is made with a Flume older than the least | A Flume older than the least | `offers.rs` `a_flume_older_than_the_least`; `flume::tests` |
| | | A Flume at the least | `offers.rs` `a_flume_at_the_least`; `flume::tests` |
| | A Flume older than the least is said once | Said once while it lasts | `offers.rs` `said_once_while_it_lasts`, its line naming the part |
| clones | A place has ended once the status carried of it says Flume holds it no more | The place tidied | `dropped.rs` `the_place_tidied` |
| | | The place no longer listed | `dropped.rs` `the_place_no_longer_listed` |
| | | The place's end held for the runner | `dropped.rs` `the_places_end_held_for_the_runner`; `report::tests` |
| family-packages | The set is the one the runner's tests pass with | The set's Flume takes the runner as a recipient | `family.rs` `the_sets_flume_takes_the_runner_as_a_recipient` |
| | | Each tool's version; Flume's status of a scratch host; The set's Flume keeps the runner's watch; A submission sluice has no record of; Not the host's tools | not new to this change; `family.rs`'s existing tests passed against the set at `v0.20.0` |

Task 4.4 also names `a_place_tidied` and `a_place_tidied_while_the_pass_answers_its_wants` (statuses). Task 4.6 names `a_lease_refused`, and task 4.7 names `a_dependent_held_then_launched`. Those tests come from earlier changes, and each passed in the gate.

Two parts of the requirements have no scenario of their own, and the code meets each:

- **No record is collected in a pass whose report could not be saved** (core.25). `collect` runs only inside `if work.save(..)`.
- **Below the least, a held offer stays held.** `Work::offer` returns before any answer changes.

## Outside this repository

Record 0030 is `proposed` until the user has seen it as a record. crew's unit `crew-reads-each-pieces-end-from-the-runners-report` follows this change.
