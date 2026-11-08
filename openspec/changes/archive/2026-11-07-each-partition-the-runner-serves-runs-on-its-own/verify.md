# Verify: each-partition-the-runner-serves-runs-on-its-own

The work on `change/each-partition-the-runner-serves-runs-on-its-own`, read at 2673525, holds. The gate passes, run once through sluice. Every one of the 21 tasks is ticked. The change's deltas hold 24 requirements and 61 scenarios. A test asserts the outcome of each scenario, and reading the code finds each requirement met.

Task 7.1 has the code stage archive the change, and it did so in 2673525. This report is added to that archive. `main` stands at 3f428cf, the branch's merge base, and so does `bolt/the-runner-serves-each-partition-apart`.

## Checks

`sluice exec main` ran once, at 2673525 against main 3f428cf, under sluice's sandbox with `network = "localhost"`.
- All 22 `check-fast` tasks passed, and their results were recorded under 2673525:
  - format, lint, typecheck and test of `boomdrive`, `boomdrive-control`, `boomdrive-crew` and `boomdrive-hosted`;
  - `nix:typecheck` and `nix:test`;
  - `root:adrs`, `root:changes`, `root:specs` and `root:format`.
- 60 test suites ran 689 tests. None failed, and none was ignored.
- moon warned that the sandbox could not write its output archives or its shared cache, and that it found no internet connection. Each task ran afresh, and none of the results was affected.

## Tasks

All 21 tasks of `tasks.md`, groups 1 to 7, are ticked `[x]`, and none is open. Each task's check names tests, and every one of those tests ran and passed in the gate. The README test for 6.1 passed under `root:format`, and the archived specs for 7.1 passed under `root:specs`.

Record 0026's amendment and the details of core.9, core.23 and core.49 are on the branch, as the tasks' preamble says.

The branch touches only what the proposal's Touches names, with four exceptions. `crates/boomdrive/tests/code_workspaces.rs`, `tests/common/mod.rs`, `tests/module_files.rs` and `tests/readme.rs` each change only a call, to pass the arguments that `pass::pass` and the openers now take. Tasks 4.3 and 5.1 changed those signatures, and the preamble says such a task changes its callers in the same task.

## Specs

Every scenario below is a test that passed in the gate. The tests work in seconds where a scenario names minutes.

### running

- **Each part of the control plane is carried on its own.** The scenarios are `control::a_slow_part`, `control::a_failing_part` and `control::a_keeping_pass_under_way`. `Runner::keep` spawns each part's thread once the first keeping pass ends. The loop runs no control pass.
- **The runner's lines of a part's pass name the part.** The scenario is `acts::acts_left_for_the_next_pass`. `Carrier::line` and `listing_of` name the label, and `Reach` names it on both of its lines.
- **boomdrive run keeps the repositories until it is stopped.** The scenarios are:
  - `running::stopped_while_waiting`;
  - `control::stopped_with_a_parts_pass_under_way`;
  - `run::tests::passes_on_the_timer`;
  - `loop::a_failure_does_not_stop_it`.

  `Stopper::stop` stops every bell. The scope waits for each part's thread before `boomdrive: stopped` is said.
- **boomdrive run --once makes one pass.** The scenarios are:
  - `running::every_repository_kept` and `running::a_clone_failed`;
  - `dropped::left_for_the_user`;
  - `control::one_control_pass` and `control::flumes_status_refused`;
  - `control::a_control_plane_that_cannot_be_read` and `control::one_part_unread`.

  `Runner::once` runs each part's pass on a scoped thread. It fails where no pass returned `reported`.
- **boomdrive run makes control passes on a timer of their own.** The scenarios are:
  - `run::tests::on_the_control_timer` and `control::control_passes_on_their_own_timer`;
  - `leases::a_lapsed_lease` and `carry::tests::a_lapsed_lease`;
  - `leases::a_pass_that_did_not_report` and `carry::tests::a_pass_that_did_not_report`;
  - `control::each_part_on_its_own_timer`.
- **A control pass runs at once when its control plane wakes the runner.** The scenarios are `control::woken_by_the_control_plane` and `control::one_part_woken`. The unit test `run::tests::a_wake_is_read_by_its_parts_pass_under_way` also covers it.
- **A control plane that cannot be read is said once until it is read again.** The scenarios are `control::three_passes_unread_then_read` and `control::each_parts_reach_its_own`.

### clones

- **A clone's keep and a launch into it take turns on the clone's lock.** The scenarios are `offers::a_launch_waits_for_its_own_repositorys_fetch_alone` and `offers::two_parts_launch_into_one_clone_at_once`. `locks::tests` holds the lock's own cases.
  - Only three places take a keep: `pass::pass`, its let-go and `pass::keep_one`.
  - Only a launch in `carry.rs` takes the lock shared.
  - No lock is held across a pass, a pull or a report.
- **A launch finds its clone again under the lock.** The scenario is `offers::a_clone_let_go_before_the_launch`.
- **A repository is fetched when the runner is told of its landing.** The scenarios are `loop::told_between_passes`, `loop::told_during_a_keeping_pass` and `loop::told_of_a_repository_it_does_not_keep`.

### control-plane-interface

- **An implementation serves its control plane as parts.** The scenarios are `crew/read::crews_partitions_as_parts` and `hosted/plane::the_hosted_partitions_as_parts`.
- **A part says which ids are its own.** The scenarios are `crew/read::crews_ids_by_partition` and `hosted/plane::the_hosted_ids_by_partition`.
- **The control plane is reached through one interface.** The scenario is `rules::only_crews_crate_names_crew`. No source of `crates/boomdrive` but `planes.rs` names an implementation.
- **The configuration chooses the implementation.** The scenarios are `control::the_hosted_control_plane_named`, `control::a_word_not_carried` and `planes::a_word_not_carried`.
- **Every implementation passes the shared cases.** `each_id_given_is_owned` runs over the fake (`fake::shared_each_id_given_is_owned`), crew's implementation (`crew/shared::shared_each_id_given_is_owned`) and the hosted implementation (`hosted/shared::each_id_given_is_owned`).

### crew-control-plane

- **The shared cache of a release before is removed.** The scenario is `crew/read::the_shared_cache_of_a_release_before`.
- **crew's records are read at one tip of each partition's drive branch.** The scenarios are `crew/read::a_file_missing_at_the_tip` and `crew/read::two_partitions_of_one_state_repository`.
- **A partition that cannot be read hides only its own work.** The scenarios are:
  - `crew/read::one_partition_missing`;
  - `crew/read::no_partition_read`;
  - `crew/read::one_partition_missing_at_a_report`, with `crew/letgo::one_partition_missing_at_a_report` beside it.

### hosted-control-plane

- **A pull gives each partition's wants under its own ids.** The scenarios are `hosted/plane::ids_under_their_partition`, `hosted/plane::one_partition_unreachable` and `hosted/plane::woken`.
- **A report goes to each partition, answered by its id.** The scenarios are `hosted/plane::a_lease_refused`, `hosted/plane::a_report_resent` and `hosted/plane::a_report_unanswered_by_another_partition`.

### work-into-flume

- **Each part saves its share of the one report file.** The scenario is `once::a_parts_save_keeps_every_other_parts_share`. `report::tests` covers the same ground with `a_save_keeps_every_other_parts_entries` and `parts_saving_at_once_keep_each_others_entries`.
- **A part the report file does not hold takes its share from its pull.** The scenarios are `once::a_file_from_the_release_before` and `once::a_part_the_file_does_not_hold`.
- **Room is read from Flume's status and counted down within a pass.** The scenarios are:
  - `room::room_for_one`;
  - `room::a_stream_with_a_queue`;
  - `room::a_host_with_no_room`;
  - `room::unbounded`;
  - `room::each_part_counts_its_own_room`.
- **Each launch, act and command is carried out once.** The scenarios are `once::a_retry_not_made_twice` and `once::a_launch_found_by_its_mark`.

## Notes

None of these holds the change back.

- **A save to a file from the release before keeps `parts` unnamed.** The requirement says a save adds its label to `parts`. `ReportFile::save_share` leaves a file with no `parts` as it is, so the file goes on holding every part's share. That keeps the requirement's own reading rule. Naming one part there would hide every other part's unreported answers, which core.10 rules out. `report::tests::a_file_with_no_parts_holds_every_part` holds this behaviour.
- **A launch that waits on the keeper counts its room at its turn.** When the keeper held the clone, the launch counts its room when its turn comes. Where the keeper then let the clone go, no launch is made, and its room stays counted for the rest of that pass. This can leave a later offer of the same pass for the next pass. core.9 allows that, since it rules out only a launch beyond room.
- **core.49's details do not name the hosted departure.** Design Decision 9 says core.49's details "also name the hosted account's drives, sharing its connection". The details on the branch name a clone's lock and the report file's lock, and do not name that departure. Design Decision 8 states the departure and its reason, as do record 0026's amendment and the README, which is what CLAUDE.md asks of a departure. Writing it into core.49's details is the design's to amend.
- **The design and the code type `say_for` differently.** Design Decision 1 types `say_for` as `&dyn Fn() -> Say`. The code hands it the part's label, as `&dyn Fn(&str) -> Say`, the same way it hands `wake_for`.
