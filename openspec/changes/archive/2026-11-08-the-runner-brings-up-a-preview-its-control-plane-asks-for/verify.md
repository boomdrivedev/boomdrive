# Verify: the-runner-brings-up-a-preview-its-control-plane-asks-for

The work on `change/the-runner-brings-up-a-preview-its-control-plane-asks-for`, read at 054f9c1, meets its change. The change stays archived, with this report beside it.

- Every task is ticked.
- The gate ran through sluice and passed all 22 `check-fast` tasks it ran for 054f9c1. 726 tests passed, none failed and none was ignored.
- The change has 29 requirements and 40 scenarios across four specs. For each scenario, a test that passed in the gate asserts its outcome.
- The code stage archived the change in 43d2bc4, as task 7.3 says to. `root:specs` and `root:changes` passed on the archived tree.
- The branch was cut from `bolt/the-runner-brings-up-previews`, which stands at 14a9139. `git merge-tree` merges the bolt and the branch with no conflict.

## Tasks

All 24 tasks of `tasks.md`, in groups 1 to 7, are ticked `[x]`, and none is open. What they build is in the tree, as the design names it:

- **The interface** (1.1, 1.2).
  - `Want::Preview(Preview)` is written as `{"preview": {…}}` with exactly `branch`, `done`, `id`, `lease`, `repository` and `revision`.
  - `Report::previews` is a `BTreeMap<Id, Previewed>`, written only where it holds something.
  - `Report::leases` names each preview's lease.
  - The fake control plane, `boomdrive-control`'s `fake.rs`, lets go and drops a preview as it does any other answer.
- **Version 1 carries no preview** (2.1).
  - `plane.rs` withholds a preview naming a repository its partition is not served, and gives its id the partition's prefix.
  - `taken_back` gives no previews.
  - `upward` writes version 1's report from its four parts alone, so a report's previews never go up.
- **The control pass** (3.1, 5.1 to 5.6). `carry.rs` handles previews as Decisions 2, 3 and 5 to 8 have them:
  - a preview is answered from the report;
  - a preview under a lapsed lease is left untaken;
  - a preview in a repository the runner does not keep is refused, and one whose clone is not yet made is left unanswered;
  - the second round asks sluice for a preview after every launch;
  - a held preview is asked again at each pass, and the line saying it waits is said once;
  - sluice's listing is read once a pass for each repository;
  - a done releases only what the report holds that the runner asked sluice for;
  - once the next pass is due, the previews' count is kept apart from the acts';
  - a preview whose lease is refused is left as sluice holds it.

  Every pass reads the report from its file (`carry.rs`, `report::read`), so the bench's second pass in `a_preview_given_twice` is a restart.
- **sluice's word of a preview** (4.1 to 4.3).
  - `tests/fixtures/sluice-0.74.3/` holds the five fixtures and a README saying how each was read on a scratch host. They name no real host, account or person's path (core.41).
  - sluice 0.74.3's source at its tag agrees with the fixtures:
    - `sluice previews --json` prints its listing with `to_string_pretty`, as `previews.json` holds it;
    - an ask stopped while it waits says `stopped: <name> waits; ask for it again to start it` and exits `STOPPED`, 130, with no document, as the stub in `tests/fakes` does.
  - The `Sluice` trait gains `preview`, `previews` and `release`, each run in the clone's container. `PREVIEW_PATIENCE` is 20 seconds, and the patience ends with SIGINT.
  - `libc` moved from the crate's dev-dependencies to its dependencies. `Cargo.lock`, which already listed `libc` among the crate's dependencies, is unchanged.
- **crew's implementation** (6.1 to 6.7).
  - It reads `Act: preview`. A preview naming no `Host` is given nothing and named in one line.
  - A preview the runner answered is given while its work has left the plan.
  - `previews.rec` is written under `write::PREVIEWS` only where a preview is or was, and passes `recfix --check`.
  - The report is read back from `previews.rec`.
  - A preview is let go once its work has left the plan and it holds no preview up.
- **The words** (7.1 to 7.3).
  - The README's control pass names `sluice preview <branch> <revision>` and `sluice release <name>`, and its section on crew's implementation names `previews.rec`.
  - The module documentation of `sluice.rs` and `carry.rs` cites core.55. The documentation of `Preview` and `Previewed` says version 1 names neither (core.42).
  - The deltas are archived into `openspec/specs/previews`, `control-plane-interface`, `crew-control-plane` and `runner-protocol`.

Rule core.55 and its details, core.13's details on the interrupted ask, and record 0031 (proposed) are in de8977b, beside the design, and read as the code does. 054f9c1 adds the code stage's three choices of its own to record 0031. Each is cheap to change, and each names the path not taken.

**Paths changed against Touches.** `git diff --name-only 14a9139 HEAD` changes three paths that the proposal's Touches does not list:

- `crates/boomdrive-control/src/fake.rs` lets go and drops a preview. Without it, the scenarios that read the fake's held report, and "A preview's lease refused", cannot hold.
- `crates/boomdrive-hosted/tests/shared.rs` takes a preview's prefix off. Task 1.2 names it.
- `crates/boomdrive/tests/readme.rs` gains the test that task 7.1 names.

Each is test support, or a fake the spec's scenarios need, and two are named by the tasks themselves. The Touches list falls short of the work, but the work stays within the change: no crate, tool, tracked file type, protocol version or protocol file is added. This verify judges that it does not hold the change back.

## Requirements and scenarios

| spec | scenario | asserted by |
|---|---|---|
| previews | A preview brought up | `previews.rs` `a_preview_brought_up` |
| | A preview sluice refuses | `previews.rs` `a_preview_sluice_refuses` |
| | A preview given twice | `previews.rs` `a_preview_given_twice` (one `sluice preview` across the restart; text and time as they stood) |
| | A preview waiting at sluice's bound | `previews.rs` `a_preview_waiting_at_sluices_bound` (patience 1s, the pass under 5s, held, said once, done at the next pass); `sluice.rs` `an_ask_past_the_patience_is_interrupted_and_waits` |
| | A preview in a repository not kept | `previews.rs` `a_preview_in_a_repository_not_kept` |
| | A preview in a repository whose clone is not yet made | `previews.rs` `a_preview_in_a_repository_whose_clone_is_not_yet_made` |
| | A failed preview's output left out | `sluice.rs` `a_failed_previews_output_is_left_out`; `a_preview_carried` and `a_release_carried` hold the fields and their order |
| | One listing for two previews of one repository | `previews.rs` `one_listing_for_two_previews_of_one_repository` |
| | A preview comes up | `previews.rs` `a_preview_comes_up` (Flume's listing read once in the pass) |
| | A preview released by a person | `previews.rs` `a_preview_released_by_a_person` |
| | A listing that cannot be read | `previews.rs` `a_listing_that_cannot_be_read` |
| | A preview put away | `previews.rs` `a_preview_put_away` |
| | A bring-up and its done in one pull | `previews.rs` `a_bring_up_and_its_done_in_one_pull` |
| | A held preview put away | `previews.rs` `a_held_preview_put_away` |
| | A done of a preview never asked for | `previews.rs` `a_done_of_a_preview_never_asked_for` |
| | A launch ahead of a preview | `previews.rs` `a_launch_ahead_of_a_preview` |
| | Previews wait for the next pass once it is due | `previews.rs` `previews_wait_for_the_next_pass_once_it_is_due`; `acts.rs` `acts_on_places_wait_for_the_next_pass_once_it_is_due` still passes |
| | A preview's lease refused | `previews.rs` `a_previews_lease_refused` |
| | A preview whose lease lapsed | `previews.rs` `a_preview_whose_lease_lapsed` |
| control-plane-interface | A preview of each kind | `json.rs` `a_preview_of_each_kind` |
| | A report holding a preview | `json.rs` `a_report_holding_a_preview` |
| | A preview's lease among the report's leases | `json.rs` `a_previews_lease_among_the_reports_leases` |
| | A report from before | `json.rs` `a_report_from_before_reads` (no `previews` read as none, written back with none) |
| | Wants in order, A pull that cannot be read, An offer's lease kept with its answer, A report sent twice | unchanged by this change; `fake.rs`'s `wants_in_order`, `a_pull_that_cannot_be_read` and `a_report_sent_twice`, and `offers.rs`'s `a_pull_that_cannot_be_read` and `an_offers_lease_kept_with_its_answer`, passed in the gate |
| crew-control-plane | A preview | `read.rs` `a_preview` |
| | A preview marked done | `read.rs` `a_preview_marked_done` |
| | A preview with no revision | `read.rs` `a_preview_with_no_revision` (a `Done` that is no yes or no too) |
| | A preview naming no host | `read.rs` `a_preview_naming_no_host` |
| | A preview whose work has left the plan | `letgo.rs` `a_preview_whose_work_has_left_the_plan` |
| | A preview brought up | `write.rs` `a_preview_brought_up_is_written_in_previews_rec`; `nothing_changed` and `only_the_runners_branch` with a preview |
| | A branch with no preview | `write.rs` `a_branch_with_no_preview` |
| | A preview read back | `write.rs` `a_preview_read_back` |
| | A preview put away, its work gone | `letgo.rs` `a_preview_put_away_its_work_gone` |
| | A preview still up, its work gone | `letgo.rs` `a_preview_still_up_its_work_gone` |
| runner-protocol | A preview is no want of version 1 | `interface.rs` `a_preview_is_no_want_of_version_1` (done and not done) |
| | The interface's values pass | `interface.rs` `the_interfaces_values_pass` |
| | An act's outcome | unchanged; `refused.rs` `an_acts_outcome` passed |

`a_preview_naming_no_host` proves that the runner's branch holds no answer to request 33 by reading `answers.rec`, not `previews.rec`. The scenario still holds: the pull gives nothing for the request, and the report the test writes holds no preview, so no `previews.rec` is written. A tighter test would read `previews.rec` too.

## Checks

- **The gate.** `sluice exec main` at 054f9c1 ran the 22 `check-fast` tasks that 054f9c1 affects against `main` at 904c8fa, and all passed:
  - `boomdrive:format`, `lint`, `typecheck` and `test`;
  - `boomdrive-control:format`, `lint`, `typecheck` and `test`;
  - `boomdrive-crew:format`, `lint`, `typecheck` and `test`;
  - `boomdrive-hosted:format`, `lint`, `typecheck` and `test`;
  - `nix:typecheck` and `test`;
  - `root:format`, `adrs`, `specs` and `changes`.

  sluice recorded the 22 results under their keys for 054f9c1.
  - 726 tests passed, none failed and none was ignored. Every test the tasks name is among them.
  - sluice did not run `nix:format`, `nix:lint`, `root:toolchain` or `root:releases`, which it judged 054f9c1 leaves unaffected.
  - moon warned that it could not write its shared cache or its output archives outside the sandbox, that it had no internet connection for dependency installs, and that the OTLP exporter failed to shut down. None of these touched a task's result.
- **Against `main`.** `origin/main` has moved on to 904c8fa since the bolt was cut, with `the-runner-serves-each-partition-apart` among what landed. Merging the branch with 904c8fa conflicts in six files:
  - `crates/boomdrive-crew/tests/read.rs`;
  - `crates/boomdrive/src/carry.rs`;
  - `crates/boomdrive/tests/fakes/mod.rs`;
  - `docs/architecture/core.md`;
  - `openspec/specs/control-plane-interface/spec.md`;
  - `openspec/specs/crew-control-plane/spec.md`.

  The branch lands on its bolt, which it merges into cleanly. The bolt takes `main` in before it lands, and that merge is where these conflicts get resolved.
- **Outside this repository.** As the proposal and design say, four things are for the orchestrator to carry:
  - crew's unit that posts `Act: preview` and reads `previews.rec` under its descriptor byte for byte;
  - the hosted protocol's later version that names `preview`, made with boomdrive-cloud;
  - sluice's statement naming Boomdrive as a caller of its previews;
  - sluice's possible ask that never waits at the bound.

  Record 0031 stays `proposed` until the user accepts it.
