# Verify: the-hosted-protocol-carries-land-and-preview

I read the work on `change/the-hosted-protocol-carries-land-and-preview` at d568fda. The gate passes and the work meets its specs, so the change stands archived:

- **The gate passes.** `sluice exec main` ran the 25 `check-fast` tasks d568fda affects and exited 0. All 25 passed.
- **Every test passes.** 851 tests passed in the gate's test tasks, none failed and none were ignored: 513 of `boomdrive`, 171 of `boomdrive-hosted`, 123 of `boomdrive-crew` and 44 of `boomdrive-control`.
- **Every scenario is asserted.** The change modifies 3 requirements of `runner-protocol`, adds 4 and removes one, with 15 scenarios. It modifies one requirement of `hosted-control-plane` and adds 4, with 10 scenarios. For each scenario, a test that passed in this gate asserts its outcome, as the tables below give.
- **Every task is ticked.** All 18 are ticked, and each one's check passed in this gate.
- **The specs reconcile.** The code stage archived the change at d568fda, as task 6.1 says to. `root:specs` passed in the gate, and `openspec validate --specs --strict` passes all 17 specs.
- **Version 1 is as released.** `git diff v0.7.3 HEAD -- crates/boomdrive-hosted/protocol/v1` is empty, and `tests/files.rs`'s `version_1_is_as_released` passed in the gate.
- **The runner's own carrying is unchanged.** No file of `crates/boomdrive/src/` changed since the bolt at e348c83 (core.53, core.55).

This report sits in the change's archive at d568fda.

## Checks

- **The gate.** `sluice exec main` at d568fda ran the `check-fast` tasks d568fda affects against `main` at 95ef0e1 and exited 0. sluice recorded the 25 results under their keys for d568fda:
  - `boomdrive:format`, `lint`, `typecheck` and `test` passed;
  - `boomdrive-control:format`, `lint`, `typecheck` and `test` passed;
  - `boomdrive-crew:format`, `lint`, `typecheck` and `test` passed;
  - `boomdrive-hosted:format`, `lint`, `typecheck` and `test` passed;
  - `nix:format`, `lint`, `typecheck` and `test` passed;
  - `root:format`, `adrs`, `specs`, `changes` and `releases` passed.
- **Warnings.** moon warned that it could not package task outputs into its cache or store blobs in the shared cache, that it had no internet connection for the Rust toolchain's install, and that the OTLP exporter could not shut down. None failed a task.
- **Its bolt.** The branch was cut from `bolt/the-hosted-protocol-carries-every-end`, which stands at e348c83 on origin, an ancestor of d568fda. The change lands on it as a fast-forward.
- **Against `main`.** sluice said `docs/architecture/core.md` would conflict with `main`. The bolt at e348c83 conflicts with `main` in the same file on its own, since `main` accepted records 0002, 0029 and 0031 in core.md after the bolt was cut (7b878da, 886f1b5). That conflict is the bolt's landing's, not this change's work.
- **The record and the rules.** Record 0037 gains its amendment of 2026-11-09, proposed until the user sees it. It names the options it did not take and the details of core.27, core.42, core.53 and core.55 it amends. core.md amends those four details in the same commit, 8587d64.

## runner-protocol

| Scenario | Asserted by |
|---|---|
| A field the version does not name | `tests/refused.rs` `a_field_the_version_does_not_name`, which adds a field to a preview, a `previewed`, a preview's copy and a release's copy beside the others |
| A time not in UTC to the second | `tests/refused.rs` `a_time_not_in_utc_to_the_second` |
| An integer past what JavaScript reads exactly | `tests/refused.rs` `an_integer_past_what_javascript_reads_exactly` |
| The interface's values pass | `tests/interface.rs` `the_interfaces_values_pass`, with a preview not done and one done, and a land |
| Every sort the interface names | `tests/interface.rs` `every_sort_the_interface_names`, by `want_sort`, `act_sort` and `command_sort`, each a match with no `_` arm |
| An act's outcome | `tests/refused.rs` `an_acts_outcome` |
| The README's worked contexts | `tests/signatures.rs` `the_enrollments_worked_bytes` and `the_connections_worked_bytes` |
| The README of version 2 | `tests/signatures.rs` `the_readme_describes_version_2` |
| A land is an act of version 2 | `tests/interface.rs` `a_land_is_an_act_of_version_2`, with `pulled-with-a-land.json` |
| A preview is a want of version 2 | `tests/interface.rs` `a_preview_is_a_want_of_version_2`, with `pulled-with-a-preview-and-its-done.json` and a preview at `R1` |
| sluice's answers of a preview as copies | `tests/copies.rs` `sluices_previews_as_copies`, over the four new `report-a-preview-*.json` examples |
| A report with no previews | `tests/refused.rs` `a_report_with_no_previews` |
| A preview's outcome | `tests/refused.rs` `a_previews_outcome` |
| The copies' fields are the lists | `tests/copies.rs` `the_previews_fields_are_the_list` and `the_releases_fields_are_the_list` |
| A preview's output | `tests/refused.rs` `a_previews_output` |

"Version 2 carries no land and no preview" is removed, with its reason and migration. `a_land_is_no_act_of_version_2` and `a_preview_is_no_want_of_version_2` are gone, and `a_land_is_no_act_of_version_1` and `a_preview_is_no_want_of_version_1` still pass, as "Version 1 SHALL carry the same but a preview, a land, a confirm and `previews`" asks.

## hosted-control-plane

| Scenario | Asserted by |
|---|---|
| Flume's places cut | `tests/cut.rs` `flumes_places_cut` |
| A value Flume adds | `tests/plane.rs` `a_value_flume_adds` |
| sluice's previews cut | `tests/cut.rs` `sluices_previews_cut`, over sluice 0.74.3's fixtures |
| A preview's copy that fails | `tests/plane.rs` `a_previews_copy_that_fails`: `status` `null`, `copy` kept, two reports sent, one line naming `home/preview-1` and `/made`, and not the value |
| Previews go up as their copies | `tests/plane.rs` `previews_go_up_as_their_copies` |
| A land carried | `tests/plane.rs` `a_land_carried` |
| A preview carried | `tests/plane.rs` `a_preview_carried` |
| A preview of a revision that is not a commit | `tests/plane.rs` `a_preview_of_a_revision_that_is_not_a_commit` |
| A preview of a repository not served | `tests/plane.rs` `a_preview_of_a_repository_not_served` |
| A report taken back holds its previews | `tests/plane.rs` `a_report_taken_back_holds_its_previews` |

## The code against its requirements

- **A land and a preview are given as any want.** `Partition::own_wants` withholds a preview whose repository the partition is not served, naming it once while it is offered, and prefixes its id. It reads nothing else of a land or a preview (core.2, core.26).
- **Each preview goes up as its copies.** `Partition::upward` cuts a done preview's text as a release's copy and any other's as a preview's, and its status as `{copy, at, listed}`. A copy that fails goes up `null`, and `failing` says it once, naming the preview and the schema's path and never a value (core.11, core.27). No preview changes which statuses the report's answer names taken.
- **`copy` is never ambiguous.** `previewed.copy` is `oneOf` a preview's copy, a release's copy or `null`. A preview's copy requires `name` and a release's requires `preview`, each closed, so no value passes both.
- **A report taken back gives each preview its copy's JSON as its text, and no status**, under the partition's prefix (core.10, core.55).
- **The documentation follows the code.** `boomdrive-control`'s `Want::Preview`, `ActSort::Land`, `Preview` and `Previewed`, `v2.rs`'s and `copy.rs`'s module documentation, the protocol's README and the root README say version 2 carries a land and a preview, and version 1 none. No word left says version 2 names no land or no preview.

## Findings

None. Nothing goes back to the code stage.
