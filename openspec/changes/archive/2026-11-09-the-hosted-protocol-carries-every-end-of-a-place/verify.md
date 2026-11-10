# Verify: the-hosted-protocol-carries-every-end-of-a-place

I read the work on `change/the-hosted-protocol-carries-every-end-of-a-place` at fa430b1. The gate passes and the work meets its specs, so the change stands archived:

- **The gate passes.** `sluice exec main` ran the 25 `check-fast` tasks fa430b1 affects and exited 0. All 25 passed, `boomdrive-hosted:lint` among them, which runs `cargo clippy -p boomdrive-hosted --all-targets --all-features -- -D warnings` over every test target that takes `mod common`.
- **Every test passes.** 836 tests passed in the gate's test tasks, none failed and none were ignored: 513 of `boomdrive`, 156 of `boomdrive-hosted`, 123 of `boomdrive-crew` and 44 of `boomdrive-control`.
- **Every scenario is asserted.** The change modifies 8 requirements of `runner-protocol` and adds 5, with 29 scenarios, and renames one. It modifies 6 requirements of `hosted-control-plane`, adds 2 and renames one, with 22 scenarios. For each scenario, a test that passed in this gate asserts its outcome, as the tables below give.
- **Every task is ticked.** All 23 are ticked, and each one's check passed in this gate.
- **The specs reconcile.** The code stage archived the change at fa430b1, as task 8.1 says to. `root:specs` passed in the gate, and `openspec validate --specs --strict` passes all 17 specs.
- **Version 1 is as released.** `git diff v0.7.3 HEAD -- crates/boomdrive-hosted/protocol/v1` is empty, and `tests/files.rs`'s `version_1_is_as_released` passed in the gate.

This report sits in the change's archive at fa430b1.

## Checks

- **The gate.** `sluice exec main` at fa430b1 ran the `check-fast` tasks fa430b1 affects against `main` at 2ce0b3f and exited 0. sluice recorded the 25 results under their keys for fa430b1:
  - `boomdrive:format`, `lint`, `typecheck` and `test` passed;
  - `boomdrive-control:format`, `lint`, `typecheck` and `test` passed;
  - `boomdrive-crew:format`, `lint`, `typecheck` and `test` passed;
  - `boomdrive-hosted:format`, `lint`, `typecheck` and `test` passed;
  - `nix:format`, `lint`, `typecheck` and `test` passed;
  - `root:format`, `adrs`, `specs`, `changes` and `releases` passed.
- **Warnings.** moon warned that it could not package task outputs into its cache or store blobs in the shared cache, that it had no internet connection for the Rust toolchain's install, and the OTLP exporter could not shut down. None failed a task.
- **Task 2.3.** `tests/common/mod.rs` names a version's validators `messages_from_runner` and `messages_to_runner`, taking `&self`. `from_runner` and `to_runner` remain free functions of the current version, taking no `self`. `examples.rs` and `refused.rs` call version 1's validators by the new names, and no call of `.from_runner()` or `.to_runner()` on a version remains. clippy's `wrong_self_convention` refuses neither under `-D warnings`.
- **Since 11526af.** The code stage's fix changed `tests/common/mod.rs`, `tests/examples.rs` and `tests/refused.rs` and ticked task 2.3. The archive at fa430b1 then reconciled the same two specs, `hosted-control-plane` and `runner-protocol`, as the archive at f37d58a: `git diff f37d58a HEAD -- openspec/specs` is empty.
- **Its bolts.**
  - The branch was cut from `bolt/the-hosted-protocol-carries-every-end`, which stands at ecbe5fa on origin, an ancestor of fa430b1. The change lands on it as a fast-forward.
  - Task 1.1 merged `origin/main` at cee700c (866c32f) and `origin/bolt/the-runner-confirms-the-ends-it-carries` at f3f693b (c7c533c). The confirm bolt's `tests/confirms.rs` passed in this gate.
- **Since the merges.** Both have moved since task 1.1 took them. Neither move is of this change's work, and both are for the landing, not the code stage:
  - `origin/bolt/the-runner-confirms-the-ends-it-carries` stands at 4ff4702, which accepts record 0002's amendments of 2026-11-07 and 2026-11-08. `git merge-tree --write-tree HEAD 4ff4702` merges clean.
  - `origin/main` stands at 2ce0b3f, which accepts records 0002, 0029 and 0031 and amends records 0025 and 0031. `git merge-tree --write-tree HEAD 2ce0b3f` conflicts in `docs/architecture/core.md`, and sluice said so at the gate's start. That conflict is the confirm bolt's: `git merge-tree` of 4ff4702 and 2ce0b3f conflicts on the same lines, core.53's and core.55's status words beside core.56. It falls to whichever landing first brings the confirm bolt onto `main`.
- **Paths.** Since the confirm bolt's merge, c7c533c, the branch changes only these paths:
  - the paths the proposal's Touches lists;
  - `crates/boomdrive/tests/readme.rs`, which task 7.2 names;
  - the change's own files and the two specs its archive reconciled.
  Record 0037 and core.42's and core.27's details were written with the proposal, f24979b, and stand in the tree as it wrote them.
- **Nothing added.** No crate, tool, tracked file type, configuration key or report field is added. Version 2 is a new directory of the protocol, which core.42 provides for.
- **The fixtures.** `crates/boomdrive/tests/fixtures/flume-0.21.0/` holds `status-dropped-held.json` and `status-done-held.json`, and a README saying how each was read on a scratch host. treefmt's `crates/*/tests/fixtures/**` exclude covers them. Neither holds a person's path. The place's paths are written as `/private/tmp/scratch/host/` (core.41).
- **The copy.** `copy::place_status` cuts each end by its named form, else by `end_delivery`, else leaves it out, and checks each against `end_record`. `end_record` is a `oneOf`, so no end passes both a named form and its delivery alone. `PlaceCopy::whole` is false while any end went up by its delivery alone or was left out. `Partition::upward` keeps those launches in `partial`, and `statuses_up` names none of them taken.
- **Record and rules.** Record 0037 is in the tree, `proposed`, and core.42's and core.27's details cite it.

## Requirements and scenarios

### runner-protocol

| requirement | scenario | asserted by (passed in the gate) |
|---|---|---|
| Each version of the protocol is JSON Schema in the hosted implementation's crate | Each file is a schema of its draft | `files.rs` `each_file_is_a_schema_of_its_draft` |
| | Nothing outside the version is referenced | `files.rs` `nothing_outside_the_version_is_referenced` |
| | Every file is listed | `files.rs` `every_file_is_listed` |
| Every message names its version, carries an id of its own and holds one body | A message with no id | `refused.rs` `a_message_with_no_id` |
| | A message of another version | `refused.rs` `a_message_of_another_version` |
| | A body of two kinds, or of none | `refused.rs` `a_body_of_two_kinds_or_of_none` |
| A message holds only what its version names | A field the version does not name | `refused.rs` `a_field_the_version_does_not_name` |
| | A time not in UTC to the second | `forms.rs` `a_time_not_in_utc_to_the_second` |
| | An integer past what JavaScript reads exactly | `forms.rs` `an_integer_past_what_javascript_reads_exactly` |
| The messages carry the interface's shapes | The interface's values pass | `interface.rs` `the_interfaces_values_pass`, a confirm among its acts |
| | An act's outcome | `refused.rs` `an_acts_outcome` |
| A place's status is carried as its copy | The copy's fields are the list | `copies.rs` `the_copys_fields_are_the_list` |
| | Text the cloud does not keep | `refused.rs` `text_the_cloud_does_not_keep` |
| | Flume's answers as copies | `copies.rs` `flumes_answers_as_copies` |
| No version names a grant or a credential | Every name read | `names.rs` `every_name_read` |
| The runner's types write and read only what the schema takes | Every example read and written back | `examples.rs` `every_example_read_and_written_back` |
| | Every kind has an example | `examples.rs` `every_kind_has_an_example` |
| The README names the two paths and the signatures' contexts | The README's worked contexts | `signatures.rs` `the_enrollments_worked_bytes`, `the_connections_worked_bytes`, `the_examples_enroll_carries_the_worked_key_and_signature` |
| | The README of version 2 | `signatures.rs` `the_readme_describes_version_2` |
| Version 2 names every end Flume 0.21.0 gives a place | A drop's and a done's end | `ends.rs` `a_drops_and_a_dones_end` |
| | A hand-over at a session's start | `ends.rs` `a_hand_over_at_a_sessions_start` |
| | An end with no key | `ends.rs` `an_end_with_no_key` |
| An end of a form version 2 does not name goes up as its delivery alone | An end of a form unnamed | `ends.rs` `an_end_of_a_form_unnamed` |
| | Content beside a delivery alone | `ends.rs` `content_beside_a_delivery_alone` |
| Version 2 carries a control plane's confirm | A confirm is an act of version 2 | `interface.rs` `a_confirm_is_an_act_of_version_2` |
| Version 2 carries no land and no preview | A land is no act of version 2 | `interface.rs` `a_land_is_no_act_of_version_2` |
| | A preview is no want of version 2 | `interface.rs` `a_preview_is_no_want_of_version_2` |
| Version 1 stays as a release spoke it | Version 1 as released | `files.rs` `version_1_is_as_released`, its digests checked against `v0.7.3` above |
| | Version 1's examples still pass | `examples.rs` `version_1s_examples_still_pass` |

### hosted-control-plane

| requirement | scenario | asserted by (passed in the gate) |
|---|---|---|
| A partition is enrolled once by boomdrive enroll | A partition enrolled | `enrollment.rs` `a_partition_enrolled` |
| | A lapsed token | `enrollment.rs` `a_lapsed_token` |
| | The answer as JSON | `boomdrive`'s `enroll.rs` `the_answer_as_json` |
| | The token never shown | `boomdrive`'s `enroll.rs` `the_token_never_shown` |
| Every connection proves the host's key | A connection proved | `connection.rs` `a_connection_proved` |
| | A key refused | `connection.rs` `a_key_refused` |
| | A control plane that speaks version 1 alone | `connection.rs` `a_control_plane_that_speaks_version_1_alone` |
| A partition's connection carries only its own repositories and work | Each hello its own | `plane.rs` `each_hello_its_own` |
| | Another partition's repository | `plane.rs` `another_partitions_repository` |
| | A repository drained goes up in no hello | `plane.rs` `a_repository_draining_goes_up_in_no_hello` |
| What goes up is version 2's copy, checked before it goes | Flume's places cut | `cut.rs` `flumes_places_cut` |
| | A value Flume adds | `plane.rs` `a_value_flume_adds` |
| Every message the runner is sent is checked before it is read | A pulled with a field unnamed | `connection.rs` `a_pulled_with_a_field_unnamed` |
| | A message of version 1 | `connection.rs` `a_message_of_version_1` |
| | An enrollment's answer on the socket | `connection.rs` `an_enrollments_answer_on_the_socket_is_refused` |
| A report's answer names taken each status that went up as its copy | A status that goes up as its copy | `plane.rs` `a_status_that_passes_goes_up_as_its_copy` |
| | A status that goes up null | `plane.rs` `a_value_flume_adds` |
| | A status with an end by its delivery alone | `plane.rs` `a_status_with_an_end_by_its_delivery_alone` |
| | A drop held for the runner | `plane.rs` `a_drop_held_for_the_runner`, on Flume 0.21.0's fixture |
| Each end of a place's status is copied on its own | An end Flume adds | `plane.rs` `an_end_flume_adds`; `copy::tests` `each_end_is_copied_on_its_own` |
| A control plane's confirm is given to the runner as any act | A confirm carried | `plane.rs` `a_confirm_carried` |
| | A confirm the runner refuses | `plane.rs` `a_confirm_the_runner_refuses` |

The tasks also name these tests, and each passed in the gate:

- `boomdrive`'s `tests/confirms.rs` (1.1);
- `boomdrive-hosted:lint` (2.3);
- `copy::tests`' `a_copy_whose_ends_all_pass_is_whole` and `a_failure_outside_the_ends_fails_whole` (5.1);
- `boomdrive`'s `tests/readme.rs` `the_readme_names_version_2_of_the_protocol` (7.2);
- `boomdrive-control:lint` (6.4).
