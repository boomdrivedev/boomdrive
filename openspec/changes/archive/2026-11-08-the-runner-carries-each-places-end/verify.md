# Verify: the-runner-carries-each-places-end

The work on `change/the-runner-carries-each-places-end` was read at 952c45d. It meets every requirement and scenario of its change, and every task is ticked. The gate fails, though: `boomdrive:lint` refuses five lines of `crates/boomdrive/tests/statuses.rs`. So the change goes back to its code stage.

- The gate ran through sluice against `main` at bbf088a. 12 of its 13 `check-fast` tasks passed. `boomdrive:lint` failed. 441 tests passed, with none failed or ignored.
- The change has 10 requirements and 24 scenarios, 14 new and 10 named by the change but not new to it. A test that passed in the gate asserts the outcome of each scenario.
- All 17 tasks are ticked. The code stage archived the change at 952c45d, as task 8.1 says to. This report is beside it, and this stage archives nothing more.
- The branch was cut from `bolt/the-runner-carries-each-places-end`, which stands at c6eb79b. `git merge-tree` merges the bolt and the branch with no conflict. Against `main` at bbf088a it conflicts (see Landing).

## What fails

`sluice exec main` at 952c45d: `boomdrive:lint` (clippy, `-D warnings`) fails on clippy's `cloned_ref_to_slice_refs` lint, five times, all in tests task 4.2 and 4.5 added:

```
error: unnecessary use of `clone` to create a slice from a reference
   --> crates/boomdrive/tests/statuses.rs:383:5
383 |     assert_eq!(collected(&bench), [record.clone()]);
   --> crates/boomdrive/tests/statuses.rs:438:5
438 |     assert_eq!(collected(&bench), [record.clone()]);
   --> crates/boomdrive/tests/statuses.rs:442:5
442 |     assert_eq!(collected(&bench), [record.clone()]);
   --> crates/boomdrive/tests/statuses.rs:468:5
468 |     assert_eq!(lines, [refused.clone()]);
   --> crates/boomdrive/tests/statuses.rs:469:5
469 |     assert_eq!(collected(&bench), [record.clone()]);
    = note: `-D clippy::cloned-ref-to-slice-refs` implied by `-D warnings`
error: could not compile `boomdrive` (test "statuses") due to 5 previous errors
```

These are in `a_record_pending_for_a_recipient_too` (383), `a_collection_flume_could_not_run` (438, 442) and `a_collection_flume_refuses` (468, 469). The tests themselves pass. Each task's own check is a `cargo test`, so none of them caught the lint. The gate is what holds the branch to it.

## Tasks

All 17 tasks of `tasks.md`, groups 1 to 8, are ticked `[x]`, and none is open. What they build is in the tree, as the design names it:

- **The set's Flume** (1.1, 1.2):
  - Record 0030 (`proposed`), core.53, and the amended details of core.1, core.13 and core.15 are in 803890f.
  - `flake.nix` and `flake.lock` name Flume `v0.20.0` (`9ab0e2c`).
  - `nix/packages/flume-git-crates.nix` holds `weir-capture` and `weir-kinds` at `daacda39`.
  - README's override example names `v0.21.0`, one release past the set, as it named `v0.17.2` past `v0.17.1`.
  - `flume::LEAST` is `(0, 20, 0)`.
- **Flume's command line** (2.1 to 2.4):
  - `flume::RECIPIENT` is `"boomdrive"`. `flume::recipients` names it after a launch's recipients, never twice, and names none where the launch names none. `launch_args` passes them.
  - `Flume::collect` runs `--json inbox <record> --recipient boomdrive` through `Cli::run`, naming no repository, with `FLUME_RECIPIENT` unset.
  - `flume::version` and `runs_with` read the first line of `--version`, ignore a suffix after the patch, and give design Decision 5's words.
- **Launches** (3.1, 3.2): every launch argv asserted whole holds `--recipient boomdrive` after the launch's own. `bolts.rs` is unchanged: its `a_bolt_and_its_unit` already asserts both argvs whole, with no `--recipient`.
- **Collection** (4.1 to 4.7):
  - `facts::pending_for` is true only for a lone `tidied` phase whose `pending_for` array names the recipient.
  - `Carrier::pass` runs `Work::collect` only where `work.save` holds, after `carry_entries` and before `send`.
  - `Work::collect` goes through the pull's offers in order. It skips:
    - a launch the report no longer holds;
    - a launch not answered `launched` at a place;
    - a clone that is `NotYet` or `NotNamed`;
    - a carried status that is not listed;
    - an entry not pending for the runner.
  - It reads the record's id from the entry's own `place` through `carried_entry` (Task notes 4.2). A refused or unrun collection is said with design Decision 3's words.
- **The drain** (5.1, 5.2): `report::ended` reads a tidied or gone place as ended only where its record is not pending for `flume::RECIPIENT`.
- **The least Flume** (6.1, 6.2):
  - `Least` is kept beside `Reach` by `run.rs`'s `Control`, by `Bench` and by `tests/pass.rs`.
  - `Least::read` says the line only where it differs from the last said, and clears it once Flume runs with the least.
  - A `Missing` version changes nothing.
  - `Work::offer` returns before `launch` while `below_least` holds.
- **README** (7.1) says all of this in the control pass's section. **The specs** (8.1): the deltas are archived into `openspec/specs/work-into-flume`, `clones` and `family-packages`, and `root:specs` passed (16 of 16).

The paths changed since c6eb79b are those the proposal's Touches lists, the change's own files and the three specs its archive reconciled. Nothing else changed. No crate, tool, tracked file type, configuration key, report field or protocol message is added. `crates/boomdrive-control`, `-crew` and `-hosted` are unchanged.

## Requirements and scenarios

| spec | requirement | scenario | asserted by |
|---|---|---|---|
| work-into-flume | The runner is one more recipient of a launch that names recipients, and no brief is left | A launch's environment and brief | `offers.rs` `a_launchs_environment_and_brief`; `flume.rs` `flume_runs_with_no_recipient_of_the_runners_own` |
| | | The runner is one more recipient | `offers.rs` and `flume.rs` `the_runner_is_one_more_recipient`; `flume::tests::the_runner_is_named_after_a_launchs_recipients` |
| | A launch is one `flume launch` naming what the launch gives | A unit's launch | `offers.rs` `a_units_launch` (argv whole, `--recipient home/planner --recipient boomdrive`); `flume.rs` `a_launch_with_every_flag` |
| | | A launch naming no stream, priority or intent | `offers.rs` and `flume.rs` `a_launch_naming_no_stream_priority_or_intent` |
| | | the bolt's scenarios (task 3.2) | `bolts.rs` `a_bolt_and_its_unit`, `a_bolt_counts_against_its_streams_room_within_its_pass` (no `--recipient`) |
| | A place's end is held for the runner until its report holds it | A landing held for the runner | `statuses.rs` `a_landing_held_for_the_runner` (record carried, one inbox after the status read, the pass after not listed and no inbox); `once.rs` `a_held_record_is_collected_once_its_report_holds_it` (the file held the record, listed, when the inbox ran) |
| | | A drop held for the runner | `statuses.rs` `a_drop_held_for_the_runner` |
| | Only a record pending for the runner is collected | A record pending for a recipient too | `statuses.rs` `a_record_pending_for_a_recipient_too` |
| | | A record pending for no end of the runner's | `statuses.rs` `a_record_pending_for_no_end_of_the_runners` |
| | | A lease refused before its end was collected | `leases.rs` `a_lease_refused_before_its_end_was_collected` (no call names the place, the record left on the host) |
| | A collection Flume refuses or cannot run is made again | A collection Flume could not run | `statuses.rs` `a_collection_flume_could_not_run` |
| | | A collection Flume refuses | `statuses.rs` `a_collection_flume_refuses` |
| | What waits on a place reads its landing from the record Flume holds | A dependent of a landing held for the runner | `after.rs` `a_dependent_of_a_landing_held_for_the_runner` |
| | No launch is made with a Flume older than the least | A Flume older than the least | `offers.rs` `a_flume_older_than_the_least`; `flume::tests::a_flume_older_than_the_least_is_refused` |
| | | A Flume at the least | `offers.rs` `a_flume_at_the_least`; `flume::tests::versions_taken` |
| | A Flume older than the least is said once | Said once while it lasts | `offers.rs` `said_once_while_it_lasts` |
| clones | A place has ended once the status carried of it says Flume holds it no more | The place tidied | `dropped.rs` `the_place_tidied` |
| | | The place no longer listed | `dropped.rs` `the_place_no_longer_listed` |
| | | The place's end held for the runner | `dropped.rs` `the_places_end_held_for_the_runner`; `report::tests::a_place_pending_for_the_runner_has_not_ended` |
| family-packages | The set is the one the runner's tests pass with | The set's Flume takes the runner as a recipient | `family.rs` `the_sets_flume_takes_the_runner_as_a_recipient` (exit 1 with a JSON reason for each, read as `Reply::Refused`; `runs_with` takes the set's `--version`) |
| | | Each tool's version, Flume's status of a scratch host, The set's Flume keeps the runner's watch, A submission sluice has no record of, Not the host's tools | not new to this change; `family.rs`'s existing tests passed against the set at `v0.20.0` |

Task 4.4 also names `a_place_tidied` and `a_place_tidied_while_the_pass_answers_its_wants` (statuses), and task 4.6 names `a_lease_refused`. Task 4.7 names `a_dependent_held_then_launched`. Those tests come from earlier changes, and they passed unchanged.

Two parts of the requirements have no scenario of their own, and the code meets each:

- No record is collected in a pass whose report could not be saved (core.25). `collect` runs only inside `if work.save(say)`.
- Where Flume is below the least, a held offer stays held. `Work::offer`'s held branch returns before the least is read.

`tests/pass.rs` tries a missing Flume's launches as before (6.2).

## Checks

- **The gate.** `sluice exec main` at 952c45d ran the 13 `check-fast` tasks that 952c45d affects against `main` at bbf088a. It exited 1, and sluice recorded the 13 results under their keys for 952c45d.
  - Passed: `boomdrive:format`, `typecheck` and `test`; `nix:format`, `lint`, `typecheck` and `test`; `root:format`, `adrs`, `specs` (16 of 16), `changes` and `releases`.
  - Failed: `boomdrive:lint`, as above.
  - 441 tests passed, with none failed or ignored. Among them are every test the tasks name, and `family.rs`'s tests against the set's own Flume `v0.20.0`.
  - sluice did not run the tasks it judged 952c45d leaves unaffected: `boomdrive-control`'s, `boomdrive-crew`'s, `boomdrive-hosted`'s and `root:toolchain`. This change touches none of their inputs.
  - moon warned that it could not write its shared cache or its output archives outside the sandbox, and the OTLP exporter failed to shut down. Neither touched a task's result.
- The gate was run once, as this stage's brief says, and nothing was fixed here.

## Landing

sluice said, before running the checks: `would conflict: README.md, crates/boomdrive/src/carry.rs, crates/boomdrive/src/run.rs, crates/boomdrive/tests/fakes/mod.rs, crates/boomdrive/tests/pass.rs, docs/architecture/core.md with main`. `main` has moved from c6eb79b to bbf088a, bringing #52 `bolt/the-runner-brings-up-previews` and records 0031 to 0032. `git merge-tree bbf088a HEAD` confirms the conflicts in those six files. The bolt still stands at c6eb79b, so the branch merges into it cleanly. The conflict is the bolt's to take up when it lands on `main`.

## Outside this repository

Record 0030 is `proposed` until the user has seen it as a record. crew's unit `crew-reads-each-pieces-end-from-the-runners-report` follows this change.
