# Verify: the-runner-confirms-an-end-once-its-control-plane-has-it

I read the work on `change/the-runner-confirms-an-end-once-its-control-plane-has-it` at 7259895, and it holds:

- **The gate passes.** `sluice exec main` ran all 25 `check-fast` tasks and exited 0. 814 tests passed, none failed and none were ignored.
- **Every scenario is asserted.** The change has 11 requirements, with 29 scenarios, and removes 2 requirements. For each scenario, a test that passed in this gate asserts its outcome.
- **Every task is ticked.** All 26 are ticked.
- **The specs reconcile.** The code stage archived the change at 7259895, as task 9.1 says to. Its archive reconciles the bolt's specs with its own, and `root:specs` passes 17 of 17.
- **It lands cleanly.** The branch holds its bolt, `bolt/the-runner-confirms-the-ends-it-carries`, at the bolt's tip, 5fafc2e. It also merges with `main` at 88dc07f with no conflict.

This report sits beside the archived change, and the change stays archived.

## Checks

- **The gate.** `sluice exec main` at 7259895 ran the `check-fast` tasks that 7259895 affects against `main` at 88dc07f. It exited 0, and sluice recorded the 25 results under their keys for 7259895:
  - `boomdrive:format`, `lint`, `typecheck` and `test`;
  - `boomdrive-control:format`, `lint`, `typecheck` and `test`;
  - `boomdrive-crew:format`, `lint`, `typecheck` and `test`;
  - `boomdrive-hosted:format`, `lint`, `typecheck` and `test`;
  - `nix:format`, `lint`, `typecheck` and `test`;
  - `root:format`, `adrs`, `specs` (17 of 17), `changes` (no open change) and `releases`.
- **Warnings.** moon warned that it could not write the shared local cache or package task outputs, and that it skipped installing dependencies offline. None of those warnings failed a task.
- **Tasks.** All 26 tasks in `tasks.md` are ticked.
- **Its bolt.**
  - `bolt/the-runner-confirms-the-ends-it-carries` stands at 5fafc2e on origin. 5fafc2e is an ancestor of 7259895, taken by the merge 3edf19e.
  - `flume.rs` holds one form of `flume confirm`: `confirm_args`, `confirm --recipient=<name> -- <key>…`.
    - `Cli::confirm` runs it, and so does the bolt's `Act::Confirm` (design Decision 5; core.56).
    - These tests assert that argv: `tests/flume.rs`'s `a_confirmation_names_its_keys_and_its_recipient` and `a_confirms_argv`, and `tests/family.rs`'s confirm.
  - `facts.rs` holds `owed` beside the bolt's `deliveries`. The fakes hold `pending_of` beside `ends`.
  - The bolt's `tests/confirms.rs` passes, 7 of 7, on `FakeFlume`'s places holding this change's ends.
- **Against `main`.**
  - `main` has moved since the bolt was cut. It is at 88dc07f, which adds record 0002's amendment of 2026-11-09.
  - `git merge-tree --write-tree HEAD 88dc07f` merges clean.
- **Paths.** Against the bolt at 5fafc2e, the branch changes only these paths:
  - the files that the proposal's Touches lists;
  - the change's own files;
  - the six specs that its archive reconciled.
- **Nothing added.** No crate, tool, tracked file type, configuration key, report field or protocol message is added.
- **Record and rules.**
  - Record 0030's amendment of 2026-11-09 is in the tree, and so are core.54's rule and details and core.1's and core.13's details.
  - core.1's details and the record name the runner's confirm in `confirm_args`'s form.
  - Record 0030 stays `proposed`.
- **Nothing collected.** The runner runs no `flume inbox`. `Flume::collect`, `Cli::collect` and the fakes' collection are gone.

## Requirements and scenarios

| spec | requirement | scenario | asserted by (passed in the gate) |
|---|---|---|---|
| work-into-flume | An end owed to the runner is confirmed by its key once its control plane has taken it | A landing held for the runner | `statuses.rs` `a_landing_held_for_the_runner` |
| | | A drop held for the runner | `statuses.rs` `a_drop_held_for_the_runner` |
| | | A live place's end | `statuses.rs` `a_live_places_end` |
| | No launch is made with a Flume older than the least | A Flume older than the least | `offers.rs` `a_flume_older_than_the_least`; `flume::tests` |
| | | A Flume at the least | `offers.rs` `a_flume_at_the_least`; `flume::tests` |
| | A Flume older than the least is said once | Said once while it lasts | `offers.rs` `said_once_while_it_lasts` |
| | A place's ends are owed to the runner until its control plane has taken them | A report not taken | `statuses.rs` `a_report_not_taken` |
| | | A status not taken | `statuses.rs` `a_status_not_taken` |
| | | The report file lost | `statuses.rs` `the_report_file_lost` |
| | Only an end owed to the runner, in a status its control plane took, is confirmed | A record pending for a recipient too | `statuses.rs` `a_record_pending_for_a_recipient_too` |
| | | A record pending for no end of the runner's | `statuses.rs` `a_record_pending_for_no_end_of_the_runners` |
| | | A lease refused before its end was confirmed | `leases.rs` `a_lease_refused_before_its_end_was_confirmed` |
| | | Nothing confirmed below the least | `statuses.rs` `nothing_confirmed_below_the_least` |
| | A confirmation Flume refuses or cannot run is made again | A confirmation Flume could not run | `statuses.rs` `a_confirmation_flume_could_not_run` |
| | | A confirmation Flume refuses | `statuses.rs` `a_confirmation_flume_refuses` |
| clones | A place has ended once the status carried of it says Flume holds it no more | The place tidied | `dropped.rs` `the_place_tidied` |
| | | The place no longer listed | `dropped.rs` `the_place_no_longer_listed` |
| | | The place's end held for the runner | `dropped.rs` `the_places_end_held_for_the_runner` |
| control-plane-interface | A report's answer names which statuses were taken | A status taken and one left | `boomdrive-control` `fake.rs` `a_status_taken_and_one_left` |
| | | An answer from before | `json.rs` `a_reports_answer`, `a_reports_answer_from_before_reads` |
| crew-control-plane | A report's answer names taken each status places.rec holds | A status places.rec holds is taken | `write.rs` `a_places_status_as_flume_gave_it`, `the_first_report` |
| hosted-control-plane | A report's answer names taken each status that went up as its copy | A status that goes up as its copy | `plane.rs` `a_status_that_passes_goes_up_as_its_copy` |
| | | A status that goes up null | `plane.rs` `a_value_flume_adds` |
| family-packages | The set is the one the runner's tests pass with | The set's Flume takes the runner as a recipient | `family.rs` `the_sets_flume_takes_the_runner_as_a_recipient` |
| | | Each tool's version | `family.rs` `each_tools_version_is_its_tags`, at `v0.21.0` |
| | | Flume's status of a scratch host; The set's Flume keeps the runner's watch; A submission sluice has no record of; Not the host's tools | not new to this change; `family.rs`'s existing tests passed against the set at `v0.21.0` |

The tasks also name these tests, and each passed in the gate:

- `a_lease_refused` (5.6);
- `a_dependent_of_a_landing_held_for_the_runner` and `a_dependent_held_then_launched` (5.7);
- `a_place_tidied` and `a_place_tidied_while_the_pass_answers_its_wants` (5.1);
- `tests/confirms.rs` (4.5).

The two removed requirements are gone from `openspec/specs/work-into-flume/spec.md`. They are "Only a record pending for the runner is collected" and "A collection Flume refuses or cannot run is made again". Each one's migration names a requirement above.

Two parts of the requirements have no scenario of their own, and the code meets each:

- **No end is confirmed in a pass whose share could not be saved after the answer** (core.25). `Carrier::pass` runs `Work::confirm` only on `Some((answer, true))`, and only when Flume is at or above the least.
- **No end is confirmed for a launch let go.** `Work::confirm` reads only launches that the report still holds after the answer, whose status the answer names taken.

## Outside this repository

- Record 0030's amendment stays `proposed` until the user has seen it as a record.
- blueprints' branch `design/the-runner-confirms-an-end-once-its-control-plane-has-it`, which amends Flume's statement's runner row and its S121, is the orchestrator's to push.
