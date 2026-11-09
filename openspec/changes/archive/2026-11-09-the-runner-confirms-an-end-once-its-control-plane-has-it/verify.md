# Verify: the-runner-confirms-an-end-once-its-control-plane-has-it

The work on `change/the-runner-confirms-an-end-once-its-control-plane-has-it` was read at 6ee1153. A test that passed in the gate asserts every scenario of the change, and every task is ticked. Two things fail, though, so the change goes back to its code stage:

- **The gate fails.** `boomdrive-crew:test` fails on `the_first_report` in `crates/boomdrive-crew/tests/write.rs`. That test was not moved with task 3.1.
- **The branch does not merge into its bolt.** `bolt/the-runner-confirms-the-ends-it-carries` now holds this bolt's other unit, `the-runner-carries-a-control-planes-confirm`. The branch conflicts with it in six files. It also runs `flume confirm` with a second argument form, where both units' designs say the one that lands second builds its call from the first's.

Summary:

- The gate ran once through sluice against `main` at 0859e14. 24 of its 25 `check-fast` tasks passed. 800 tests passed, 1 failed and none were ignored.
- The change has 11 requirements, with 29 scenarios, and removes 2 requirements. A test that passed in the gate asserts the outcome of each scenario.
- All 23 tasks are ticked. The code stage archived the change at 6ee1153, as task 9.1 says to. This report sits beside it, and this stage archives nothing more.

## What fails

### The gate: `the_first_report`

`sluice exec main` at 6ee1153 exited 1. Its one failed task is `boomdrive-crew:test`:

```
---- the_first_report stdout ----
thread 'the_first_report' panicked at crates/boomdrive-crew/tests/write.rs:181:5:
assertion `left == right` failed
  left: Reported { let_go: [], refused: {}, statuses_taken: {Id("home/12")} }
 right: Reported { let_go: [], refused: {}, statuses_taken: {} }
test result: FAILED. 12 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
```

- `the_first_report` reports `report_launched(r#"{"status":{}}"#)`. That report holds launch `home/12` at a place, with a status. The test then asserts that the answer is `Reported::default()`.
- Under task 3.1, crew's answer now names taken each status that `places.rec` holds. So it names `home/12`, as the crew-control-plane spec's "A report's answer names taken each status places.rec holds" asks.
- The code is right and the test's expected answer is stale. Task 3.1's own check runs only `a_places_status_as_flume_gave_it`, so it did not catch this. The gate is what holds the crate to it.

### The landing: the bolt's other unit came first

- The branch was cut from `bolt/the-runner-confirms-the-ends-it-carries` when the bolt stood at `main`'s 0859e14. The bolt now stands at 5fafc2e. That commit holds `the-runner-carries-a-control-planes-confirm`, landed as #67.
- `git merge-tree --write-tree HEAD origin/bolt/the-runner-confirms-the-ends-it-carries` conflicts in six files:
  - `crates/boomdrive/src/facts.rs`;
  - `crates/boomdrive/tests/fakes/mod.rs`;
  - `crates/boomdrive/tests/flume.rs`;
  - `openspec/specs/control-plane-interface/spec.md`;
  - `openspec/specs/crew-control-plane/spec.md`;
  - `openspec/specs/work-into-flume/spec.md`.
- Both units' designs say who reconciles `flume confirm`:
  - This change's design, under Risks: "Whichever lands on the bolt second takes the first's `confirm`, `LEAST` and set as they stand."
  - The other unit's design: "Both run `flume confirm`, so both name one form of its arguments, `confirm --recipient=<name> -- <key>…`. Whichever of the two the bolt takes second builds its call from the other's, so `flume.rs` holds that form once."
- The bolt's `flume.rs` builds that form in `confirm_args`, for `Act::Confirm`. This branch's `Cli::confirm` builds its own: `confirm <key>… --recipient <name>`. `tests/flume.rs`'s `a_confirmation_names_its_keys_and_its_recipient` and `tests/family.rs`'s confirm argv assert that form.
- The bolt also adds `facts::deliveries(entry)` beside this branch's `facts::owed(entry, name)`, at the same place in `facts.rs`.
- The other unit moves neither `flume::LEAST` nor the set's Flume, leaving both to this change. So this branch's moves to `0.21.0` and `v0.21.0` stand as they are.

So this change lands second, and it must take the bolt at 5fafc2e before it can land:

- resolve the six conflicts;
- build `Cli::confirm` from `confirm_args`, so `flume.rs` holds one form of the command, and move the argv the tests assert to it;
- archive the specs again over the bolt's.

## Checks

- **The gate.** `sluice exec main` at 6ee1153 ran the 25 `check-fast` tasks that 6ee1153 affects against `main` at 0859e14. It exited 1, and sluice recorded the 25 results under their keys for 6ee1153.
  - Passed:
    - `boomdrive:format`, `lint`, `typecheck` and `test`;
    - `boomdrive-control:format`, `lint`, `typecheck` and `test`;
    - `boomdrive-crew:format`, `lint` and `typecheck`;
    - `boomdrive-hosted:format`, `lint`, `typecheck` and `test`;
    - `nix:format`, `lint`, `typecheck` and `test`;
    - `root:format`, `adrs`, `specs` (17 of 17), `changes` and `releases`.
  - Failed: `boomdrive-crew:test`, on `the_first_report` alone. The crate's other test binaries passed, and so did the other 12 tests of `tests/write.rs`.
  - sluice did not run `root:toolchain`, `root:commits` or `root:workflow`, which it judged 6ee1153 leaves unaffected. The 24 commits the branch adds to `main` are all Conventional Commits.
  - moon warned that it could not write its shared cache or its output archives outside the sandbox. It also had no internet connection for a dependency install, and the OTLP exporter failed to shut down. None of these touched a task's result.
- The gate was run once, as this stage's brief says, and nothing was fixed here.
- The bolt merge was tried with `git merge-tree`, which writes nothing. The branch merges into `main` at 0859e14, its own base, with no conflict.

## Tasks

All 23 tasks of `tasks.md`, groups 1 to 9, are ticked `[x]`, and none is open. What they build is in the tree, as the design names it:

- **The set's Flume** (1.1):
  - `flake.nix` and `flake.lock` name Flume `v0.21.0` (`77d5b93`).
  - `nix/packages/flume-git-crates.nix` holds `weir-capture`, `weir-delivery`, `weir-kinds` and `weir-pack` at `52b9552`, each with Flume's own hash.
  - README's override example names `v0.22.0`.
- **The interface** (2.1, 2.2):
  - `Reported::statuses_taken` is a `BTreeSet<Id>`, serde `default` and skipped where empty.
  - `Fake` names taken each launch of a report that holds a status, except those let go, refused, or left by `leave_status` until `take_status`.
- **The implementations** (3.1, 3.2):
  - crew's `report` extends `statuses_taken` by `write::statuses_held(label, &kept)` once the branch is written. `texts` reads the same rule. For each role it reads only the latest run, as before.
  - The hosted `Partition` keeps `Sent { id, part, statuses }`. `statuses_up` leaves out each id `upward` put in `failing` for its status.
  - `answered` names taken only for the last report sent, less what that answer let go or refused. An answer read from `answers()` names none.
- **Flume's command line** (4.1 to 4.3):
  - `Flume::confirm(recipient, keys)` runs `--json confirm <key>… --recipient <recipient>`, naming no repository.
  - `facts::owed` gives each end's `delivery` pending for the name, in the order of the ends, and leaves out an end with no `delivery` string.
  - The set's Flume refuses a confirm of a key no end is under on exit 1. `flume::Cli` reads that as `Reply::Refused`.
- **The pass** (5.1 to 5.9):
  - `send` gives `Option<(Reported, bool)>`, the bool being the save after the answer.
  - `Carrier::pass` runs `Work::confirm` only on `Some((answer, true))` with `below_least` none.
  - `Work::confirm` walks the pull's offers in order. It confirms for a launch only where all of these hold:
    - the answer names it in `statuses_taken`;
    - the report still holds it, `launched` at a place;
    - its clone stands;
    - its carried status is listed;
    - `facts::owed` gives a key.
  - It runs one `flume confirm` for each such launch. It says a refusal or an unrun call in one line, prefixed with the id, and keeps nothing of the answer.
  - `Flume::collect`, `Cli::collect` and the fakes' collection are gone. The tree runs no `flume inbox`.
- **The drain** (6.1): `report::ended`'s documentation says confirmed. `the_places_end_held_for_the_runner` reads the record pending for `boomdrive` and `scratch/planner`, then for `scratch/planner` alone.
- **The least Flume** (7.1 to 7.3): `flume::LEAST` is `(0, 21, 0)`. `runs_with`'s words name "the first whose ends a recipient confirms by key".
- **README** (8.1): the control pass's section says each of these.
- **The specs** (9.1): the deltas are archived into the six specs, and `root:specs` passes 17 of 17.
- **Record and rules:** record 0030's amendment of 2026-11-09 and core.54's rule and details are in the tree, and so are the details of core.1 and core.13. The record stays `proposed`.

Against `main` at 0859e14, the branch changes only these paths:

- the files the proposal's Touches lists;
- the change's own files;
- the six specs its archive reconciled.

No crate, tool, tracked file type, configuration key, report field or protocol message is added.

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
| crew-control-plane | A report's answer names taken each status places.rec holds | A status places.rec holds is taken | `write.rs` `a_places_status_as_flume_gave_it` |
| hosted-control-plane | A report's answer names taken each status that went up as its copy | A status that goes up as its copy | `plane.rs` `a_status_that_passes_goes_up_as_its_copy` |
| | | A status that goes up null | `plane.rs` `a_value_flume_adds` |
| family-packages | The set is the one the runner's tests pass with | The set's Flume takes the runner as a recipient | `family.rs` `the_sets_flume_takes_the_runner_as_a_recipient` |
| | | Each tool's version | `family.rs` `each_tools_version_is_its_tags`, at `v0.21.0` |
| | | Flume's status of a scratch host; The set's Flume keeps the runner's watch; A submission sluice has no record of; Not the host's tools | not new to this change; `family.rs`'s existing tests passed against the set at `v0.21.0` |

The tasks also name `a_lease_refused` (5.6), `a_dependent_of_a_landing_held_for_the_runner` and `a_dependent_held_then_launched` (5.7). They also name `a_place_tidied` and `a_place_tidied_while_the_pass_answers_its_wants` (5.1). Each passed in the gate.

The two removed requirements, "Only a record pending for the runner is collected" and "A collection Flume refuses or cannot run is made again", are gone from `openspec/specs/work-into-flume/spec.md`. Each one's migration names a requirement above.

Two parts of the requirements have no scenario of their own, and the code meets each:

- **No end is confirmed in a pass whose share could not be saved after the answer** (core.25). `confirm` runs only on `Some((answer, true))`.
- **No end is confirmed for a launch let go.** `reported` forgets it before `confirm` reads the report, and the fake does not name it taken.

## Outside this repository

- Record 0030's amendment stays `proposed` until the user has seen it as a record.
- blueprints' branch `design/the-runner-confirms-an-end-once-its-control-plane-has-it`, which amends Flume's statement's runner row and its S121, is the orchestrator's to push.
