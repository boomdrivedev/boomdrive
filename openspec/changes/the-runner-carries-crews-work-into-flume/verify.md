# Verify: the-runner-carries-crews-work-into-flume

The work on `change/the-runner-carries-crews-work-into-flume`, read at 9fa3ae2, falls short, and the change is not archived. Every task is ticked. Every check task 10.1 names passes, and so does the gate run fresh through sluice. The change has 83 requirements and 150 scenarios. Of the scenarios, 144 are proved by a test that asserts their outcome. Three are contradicted by the code and its tests on purpose, and three are proved only in part. Archiving the change as written breaks `root:specs` on main. Two further requirements are not met in the code.

## What falls short

1. **Archiving breaks `root:specs`.** The configuration delta's MODIFIED "The configuration holds nothing the runner does not read" has 736 characters of requirement text. openspec warns above 500, and `--strict` makes that warning an error. `root:changes` validates the delta with `openspec validate --changes --strict`, which does not apply the length rule, so it passes. A trial archive on a copy of `openspec/` merged every delta cleanly. `openspec validate --specs --strict` then failed `spec/configuration`, its `requirements[4]` "Requirement text is very long (>500 characters)". The landed requirement is two sentences; the change's bulleted refusals of streams, kinds and keys of before make it long. Its text is to be shortened. One way is to move the refusals into a requirement of their own, whose scenarios already exist ("A stream named", "A kinds file named", "A key of before").

2. **Three scenarios say what the code and its tests do not.** Each is a deliberate choice in the code or the interface, so either the scenario's THEN or the code moves:
   - control-plane-interface "A pull that cannot be read" and running "A control plane that cannot be read" both say no `flume` command runs. Design Decision 5, step 2, reads `flume --json status` and each tool's `--version` for the hello before the pull (`src/carry.rs:144`, `src/hello.rs`). `offers::a_pull_that_cannot_be_read` asserts that `flume --json status` ran (`[Call::Status]`). `control::a_control_plane_that_cannot_be_read` allows `status` and `--version`. The spec and the design disagree.
   - crew-control-plane "A run standing" says the pull gives no offer for `planner`. `crew/roles::a_run_standing` asserts the pull gives `madswan/planner/1`. That is the standing run given again as a settled want, as the shared case "a settled want is still given" requires (`crew/src/roles.rs:307-313`). The scenario means no new run of `planner`.

3. **`flywheel run --once` exits 1 where the running spec says 0.** The spec says it exits 0 when every git call of the keeping pass succeeded and the control pass both pulled and reported, and 1 otherwise. `Runner::once` (`src/run.rs:296-307`) exits 1 whenever the control pass is marked failed. `src/carry.rs` marks it failed in more cases than a pull or report that did not happen:
   - a launch or act Flume could not run (`:314`, `:435`);
   - Flume's status could not be read (`:151`);
   - sluice could not run (`:582`, `:750`);
   - origin's branch could not be read (`:590`);
   - a place's status or an inbox could not be read (`:686`, `:816`);
   - the report could not be saved (`:873`).

   `offers::flume_is_not_there` asserts the pass failed although it pulled and reported. `Runner::once`'s doc comment and README's lines 76 to 81 say what the spec says. `src/main.rs:27-28`'s help still says "0 where every git call succeeded, 1 where one failed", with no word of the control pass. The spec, the code and the help text are to agree.

4. **One partition never read withholds every partition's let-go.** crew's `report` (`crew/src/lib.rs:738-806`) pushes a partition whose `<label>/main` has not been read onto `unsent`, and goes on to write the others. It then returns `Err(Unsent)` for the whole call, dropping the `let_go` it computed for the partitions it wrote. Take "One partition missing", with `swancloud/main` unreadable and `madswan` read:
   - every report writes `madswan/runner/<host>`, yet comes back `Unsent`;
   - the runner keeps the launches crew has already removed from `madswan`'s branch;
   - every control pass is marked failed for as long as `swancloud` stays unread.

   That contradicts "A partition that cannot be read hides only its own work" and "A launch is let go once its work has left crew's plan". No test reports with one partition unread; `crew/read::one_partition_missing` only pulls.

5. **No implementation can wake the runner.** The running spec's "A control pass runs at once when its control plane wakes the runner" is met by the loop, which acts on `Event::Wake` (`src/run.rs:361-366`). But `planes::Opener` (`src/planes.rs:20-21`) hands an implementation no `Notices`, and `Control::open` runs before the event channel exists (`src/run.rs:206-207`). Design Decision 5 says an implementation "may send through `Notices`"; none can. `control::woken_by_the_control_plane` calls `notices.wake()` itself, and with an hour's interval it cannot tell a timer kept from the woken pass's start from one kept from the pass before. crew never wakes, so nothing breaks today. Either the opener carries a way to wake the runner, or the spec says the requirement waits for an implementation that pushes.

## Tasks

All 39 tasks of `tasks.md`, groups 1 to 10, are ticked `[x]`, and none is open. Each group's closing task names the checks that ran it. Those checks pass at 9fa3ae2 (below).

What the tasks build stands in the tree:
- `crates/flywheel-control` and `crates/flywheel-crew` are members of the cargo workspace and moon projects of their own in `.moon/workspace.yml`, with `language: 'rust'`.
- Each crate has the tasks format, lint, typecheck and test, each tagged `check-fast` and naming `/devenv.nix`, `/devenv.lock` and `/.prototools` among its inputs. `flywheel-crew`'s name `/crates/flywheel-control/**/*` too, and `flywheel`'s name both new crates' sources and `/README.md` for its README test.
- `devenv.nix` pins recutils at 1.9, asserted against `pkgs.recutils.version` as git is, and `check-toolchain.sh` checks `recfix --version` against it.
- `treefmt.nix` excludes `crates/*/tests/fixtures/**` by name, with its reason.
- `Cargo.toml` takes weir-branch, weir-capture and weir-signals at `0b6eb5abb806a51fec128b01f97c4dcff6170219`.
- `docs/architecture/core.md` holds core.5 amended and core.12, and `docs/adr/0002` holds the decision, both in 22d7630 beside the design.
- README carries the `[control_plane]` table in a fenced `toml` block, the runner's branch, and that kinds and streams are Flume's host configuration. CLAUDE.md's opening paragraph says what the runner now carries.

The tree touches only what the proposal's Touches names, beside the change itself and its record.

## Checks

Run through sluice, so its sandbox and `network = "localhost"` applied:

- `sluice exec main -- moon run flywheel-control:format flywheel-control:lint flywheel-control:typecheck flywheel-control:test flywheel-crew:format flywheel-crew:lint flywheel-crew:typecheck flywheel-crew:test flywheel:format flywheel:lint flywheel:typecheck flywheel:test root:format root:toolchain root:adrs root:specs root:changes`, task 10.1's command, passed: 17 tasks, 14 of them replayed from moon's cache.
- `sluice exec main`, the gate, then ran every `check-fast` task afresh and passed all 17. It recorded their results under 9fa3ae2. There were 438 tests, none failed and none ignored. `root:toolchain` reported rustc, cargo, clippy and rustfmt at 1.98.1, git at 2.55.0, recutils at 1.9 and openspec at 1.14.3. `root:changes` validated this change under `--strict`.

moon warned that the sandbox could not write its output archives or shared cache. The tasks' results were unaffected.

## Trial archive

`openspec archive the-runner-carries-crews-work-into-flume --yes`, run with the pinned openspec on a copy of `openspec/` in a scratch directory, applied every delta: + 71, ~ 12, → 3. It created branch-commands, control-plane-interface, crew-control-plane, terminal-relay, tool-records and work-into-flume. It updated clones, configuration, running and views, and the three RENAMED headers each found their requirement. `openspec validate --specs --strict` then passed nine of the ten specs and failed configuration (finding 1). The archive also warned, without blocking, that the Why section passes 1000 characters and that the change has more than 10 deltas.

## Scenarios

Each scenario's test was read against its WHEN and THEN. A test named `x::y` is the function `y` in `crates/flywheel/tests/x.rs`, and `src/m.rs::y` a unit test of that module of `crates/flywheel`. `crew/` and `control/` mark the same in `crates/flywheel-crew` and `crates/flywheel-control`. A note marks each scenario not met or met only in part.

### branch-commands (5 requirements, 7 scenarios)

| Scenario | Proved by | Note |
|---|---|---|
| No gate for the branch | `branches::no_gate_for_the_branch` |  |
| A branch made | `branches::a_branch_made` |  |
| A branch already there | `branches::a_branch_already_there` |  |
| A repository not kept | `branches::a_repository_not_kept` |  |
| A clone not yet made | `branches::a_clone_not_yet_made` |  |
| A command given twice | `branches::a_command_given_twice` |  |
| A boom command | `branches::a_boom_command` |  |

### clones (1 requirement, 2 scenarios)

| Scenario | Proved by | Note |
|---|---|---|
| A new repository is cloned | `clones::a_new_repository_is_cloned` |  |
| The clone is not under the views root | `views::the_clone_is_not_under_the_repository_root` |  |

### configuration (7 requirements, 20 scenarios)

| Scenario | Proved by | Note |
|---|---|---|
| An address with no owner | `src/config.rs::an_address_with_no_owner` |  |
| One repository twice | `src/config.rs::one_repository_twice` |  |
| No list | `src/config.rs::no_list` |  |
| Keeping none | `src/config.rs::keeping_none` |  |
| No views setting | `src/config.rs::no_views_setting` |  |
| A home-relative root | `src/config.rs::a_home_relative_root` |  |
| A relative root | `src/config.rs::a_relative_root` |  |
| No period | `src/config.rs::no_period` |  |
| A period of nothing | `src/config.rs::a_period_of_nothing` |  |
| A misspelt key | `src/config.rs::a_misspelt_key` |  |
| A token | `src/config.rs::a_token` |  |
| A stream named | `src/config.rs::a_stream_named` |  |
| A kinds file named | `src/config.rs::a_kinds_file_named` |  |
| A key of before | `src/config.rs::a_key_of_before` |  |
| crew named | `planes::crew_named` |  |
| No type | `src/config.rs::no_type` |  |
| No control plane | `control::no_control_plane` |  |
| No poll interval | `src/config.rs::no_poll_interval` |  |
| A poll interval of nothing | `src/config.rs::a_poll_interval_of_nothing` |  |
| A poll interval changed while running | `control::a_poll_interval_changed_while_running` |  |

### control-plane-interface (8 requirements, 11 scenarios)

| Scenario | Proved by | Note |
|---|---|---|
| Only crew's crate names crew | `rules::only_crews_crate_names_crew` |  |
| Wants in order | `control/fake::wants_in_order` |  |
| A pull that cannot be read | `offers::a_pull_that_cannot_be_read`, `control/fake::a_pull_that_cannot_be_read` | **Not met as written** (finding 2) |
| An offer's lease kept with its answer | `offers::an_offers_lease_kept_with_its_answer` |  |
| An offer with every field | `control/fake::an_offer_with_every_field` |  |
| A report sent twice | `control/fake::a_report_sent_twice` |  |
| A report not sent | `offers::a_report_not_sent`, `control/fake::a_report_not_sent` |  |
| A launch let go | `leases::a_launch_let_go`, `control/fake::a_launch_let_go` |  |
| The hosted control plane named | `control::the_hosted_control_plane_named` |  |
| The fake control plane passes | `control/fake::shared_wants_in_order`, `control/fake::shared_a_report_comes_back`, `control/fake::shared_a_report_twice`, `control/fake::shared_a_settled_want_is_given` |  |
| crew's implementation passes | `crew/shared::shared_wants_in_order`, `crew/shared::shared_a_report_comes_back`, `crew/shared::shared_a_report_twice`, `crew/shared::shared_a_settled_want_is_given` |  |

### crew-control-plane (21 requirements, 38 scenarios)

| Scenario | Proved by | Note |
|---|---|---|
| The table read | `crew/src/config.rs::the_table_read` |  |
| No host | `crew/src/config.rs::no_host` |  |
| A label twice | `crew/src/config.rs::a_label_twice` |  |
| A kinds file named | `crew/src/config.rs::a_kinds_file_named` |  |
| A token in crew's table | `crew/src/config.rs::a_token_in_crews_table` |  |
| A file missing at the tip | `crew/read::a_file_missing_at_the_tip` |  |
| One partition missing | `crew/read::one_partition_missing` |  |
| No partition read | `crew/read::no_partition_read` |  |
| Only this host's requests | `crew/read::only_this_hosts_requests` |  |
| A kit named by its name alone | `crew/claims::a_kit_named_by_its_name_alone` |  |
| A repository not kept | `crew/claims::a_repository_not_kept` |  |
| Claimed alone | `crew/claims::claimed_alone` |  |
| Another's claim seen | `crew/claims::another_claim_seen` |  |
| Taken elsewhere | `crew/claims::taken_elsewhere` |  |
| No room on the host | `crew/claims::no_room_on_the_host` |  |
| A claim waiting for its stream's room | `crew/claims::a_claim_waiting_for_its_streams_room` |  |
| A claim found again after a stop | `crew/claims::a_claim_found_again_after_a_stop` |  |
| Claimed again in name order | `crew/claims::both_claiming_before_either_checks` |  |
| An act on another host's launch | `crew/claims::an_act_naming_no_host_goes_to_the_runner_holding_its_launch` |  |
| A launch's every field | `crew/read::a_launchs_every_field` |  |
| A kit's launch naming no stream | `crew/read::a_kits_launch_naming_no_stream` |  |
| A retry of the submission | `crew/read::a_retry_of_the_submission` |  |
| An act crew does not write | `crew/read::an_act_crew_does_not_write` | Partial: asserts the want; the runner's refusal is `after::an_act_on_anothers_launch` |
| Nothing changed | `crew/write::nothing_changed` |  |
| Only the runner's branch | `crew/write::only_the_runners_branch` |  |
| The first report | `crew/write::the_first_report` |  |
| A place's status as Flume gave it | `crew/write::a_places_status_as_flume_gave_it` |  |
| Nothing pending | `crew/roles::nothing_pending` |  |
| A superseded turn answers nothing | `crew/roles::a_superseded_turn_answers_nothing` |  |
| Another host's place asks the planner | `crew/roles::another_hosts_place_asks_the_planner` |  |
| A role's work landed | `crew/roles::a_roles_work_landed` |  |
| A burst of asks | `crew/roles::a_burst_of_asks` |  |
| A run standing | `crew/roles::a_run_standing` | **Not met as written** (finding 2) |
| A run waiting for room keeps its id | `crew/roles::a_run_waiting_for_room_keeps_its_id` |  |
| A turn delivered | `crew/roles::a_turn_delivered` |  |
| A unit landed and gone from the plan | `crew/letgo::a_unit_landed_and_gone_from_the_plan` |  |
| A unit still in the plan | `crew/letgo::a_unit_still_in_the_plan` | Partial: asserts the `Place` stays, not its last `Status` |
| A fix of main | `crew/letgo::a_fix_of_main` |  |

### running (6 requirements, 13 scenarios)

| Scenario | Proved by | Note |
|---|---|---|
| Stopped while waiting | `running::stopped_while_waiting` |  |
| Passes on the timer | `src/run.rs::passes_on_the_timer` |  |
| A failure does not stop it | `loop::a_failure_does_not_stop_it` |  |
| Every repository kept | `running::every_repository_kept` |  |
| A clone failed | `running::a_clone_failed` |  |
| Left for the user | `dropped::left_for_the_user` |  |
| One control pass | `control::one_control_pass` |  |
| A control plane that cannot be read | `control::a_control_plane_that_cannot_be_read` | **Not met as written** (finding 2) |
| A quiet pass | `running::a_quiet_pass` |  |
| A view moved | `running::a_view_moved` |  |
| On the control timer | `src/run.rs::on_the_control_timer` | Exact times in the unit test; `control::control_passes_on_their_own_timer` runs the loop |
| Woken by the control plane | `control::woken_by_the_control_plane` | Partial: the test wakes the runner itself (finding 5) |
| The configuration broken while running | `control::the_configuration_broken_while_running` |  |

### terminal-relay (4 requirements, 7 scenarios)

| Scenario | Proved by | Note |
|---|---|---|
| A launch not held | `terminals::a_launch_not_held` |  |
| No agent running | `terminals::no_agent_running` |  |
| A screen sent once while it stands | `terminals::a_screen_sent_once_while_it_stands` |  |
| Input passed in | `terminals::input_passed_in` |  |
| The pane gone | `terminals::the_pane_gone` |  |
| The launch let go | `terminals::the_launch_let_go` |  |
| Nothing written | `terminals::nothing_written` |  |

### tool-records (7 requirements, 9 scenarios)

| Scenario | Proved by | Note |
|---|---|---|
| An entry carried | `entries::an_entry_carried` |  |
| No submission | `entries::no_submission` |  |
| A failed row without its log | `entries::a_failed_row_without_its_log` |  |
| A landed entry | `entries::a_landed_entry` |  |
| An unchanged tip | `inboxes::an_unchanged_tip` |  |
| An inbox carried | `inboxes::an_inbox_carried` |  |
| An inbox weir refuses | `inboxes::an_inbox_weir_refuses` |  |
| A repository not kept | `inboxes::a_repository_not_kept` |  |
| No such branch | `inboxes::no_such_branch` |  |

### views (4 requirements, 7 scenarios)

| Scenario | Proved by | Note |
|---|---|---|
| The first view | `views::the_first_view` |  |
| A view the user deleted | `views::a_view_the_user_deleted` |  |
| A new root, the old view clean | `views::a_new_root_the_old_view_clean` |  |
| A new root, the old view holding work | `views::a_new_root_the_old_view_holding_work` |  |
| A new root, the old folder replaced | `views::a_new_root_the_old_folder_replaced` |  |
| The owner's last view | `dropped::the_owners_last_view` |  |
| The owner's folder holds more | `dropped::the_owners_folder_holds_more` |  |

### work-into-flume (20 requirements, 36 scenarios)

| Scenario | Proved by | Note |
|---|---|---|
| Repositories kept | `offers::repositories_kept` |  |
| A tool missing | `offers::a_tool_missing` |  |
| Streams as Flume printed them | `offers::streams_as_flume_printed_them` |  |
| A repository not kept | `offers::a_repository_not_kept` |  |
| A clone not yet made | `offers::a_clone_not_yet_made` |  |
| A dependent held, then launched | `after::a_dependent_held_then_launched` |  |
| A dependent of a dropped launch | `after::a_dependent_of_a_dropped_launch` |  |
| Room for one | `room::room_for_one` |  |
| A stream with a queue | `room::a_stream_with_a_queue` |  |
| A host with no room | `room::a_host_with_no_room` |  |
| Unbounded | `room::unbounded` |  |
| A stream the host does not declare | `room::a_stream_the_host_does_not_declare` |  |
| No stream named | `room::no_stream_named` |  |
| A unit's launch | `offers::a_units_launch` |  |
| A launch naming no stream, priority or intent | `offers::a_launch_naming_no_stream_priority_or_intent`, `flume::a_launch_naming_no_stream_priority_or_intent` |  |
| A kind named by its identity | `offers::a_kind_named_by_its_identity` |  |
| A launch's environment and brief | `offers::a_launchs_environment_and_brief` |  |
| Flume refuses | `offers::flume_refuses` |  |
| Flume is not there | `offers::flume_is_not_there` |  |
| A retry with words | `acts::a_retry_with_words` |  |
| A done | `acts::a_done` |  |
| A priority of a held launch | `after::a_priority_of_a_held_launch` |  |
| A drop of a held launch | `after::a_drop_of_a_held_launch` |  |
| A retry of a held launch | `after::a_retry_or_a_done_of_a_held_launch` |  |
| An act on another's launch | `after::an_act_on_anothers_launch` |  |
| A retry not made twice | `once::a_retry_not_made_twice` |  |
| A launch found by its mark | `once::a_launch_found_by_its_mark` |  |
| A lease refused | `leases::a_lease_refused` |  |
| Renewed before it lapses | `src/carry.rs::renewed_before_it_lapses`, `leases::renewed_before_it_lapses` |  |
| The status byte for byte | `statuses::the_status_byte_for_byte` |  |
| Only the host changed | `statuses::only_the_host_changed` |  |
| A place tidied | `statuses::a_place_tidied` |  |
| A landing told once | `statuses::a_landing_told_once` |  |
| The user's own place | `statuses::the_users_own_place` |  |
| A launch said | `statuses::a_launch_said` |  |
| A quiet pass | `statuses::a_quiet_pass` |  |

Every test reaches no network and runs no real Flume, sluice or herdr (design Decision 21). git runs under the scratch host of `crates/flywheel/tests/common`, each state repository is a bare repository the test makes, and each tool is a fake behind a trait or a stub program in the test's own temporary directory.

## Requirements read in the code

Beyond the scenarios, these hold by reading:
- **The launch.** `flume --json launch` names `--repository`, `--kind`, `--name`, then each flag only where the launch gives it, and `--mark flywheel:<id>`. No source passes `--kinds`, and none passes `--force` to Flume; the one `--force` is git's `worktree add` in `src/view.rs`, on main before this change. Every `flume` call runs with `FLUME_RECIPIENT` unset, and the brief's file is removed once Flume answers.
- **Room.** Room is counted down within a pass, for the stream and the host. `null` is unbounded, a stream with a queue has none, and an unlisted stream is launched so that Flume's refusal is the answer.
- **Statuses.** A place's status is read again only when its listing entry differs as a JSON value. The hello slices Flume's host part and leaves out `pools`, `addresses`, `bases` and `terminal_kept`.
- **Leases.** The next control pass is due by the midpoint between the end of the pass and the earliest expiry (`src/carry.rs::next_due`).
- **crew's records.** crew's four descriptors in `crew/src/write.rs` equal the four string constants of crew's `plugin/lib/runner.py` at `9913f249`, byte for byte. Every lease crew gives is number 1 with no expiry. The runner's branch is written by weir-branch's write: `recfix --check` on each file, one commit, and a push of `<label>/runner/<host>` alone, leased and never forced.
- **The name-order reclaim.** A withdrawn launch is claimed again no sooner than `pulls + place`, its place counted from 1 among the claimants seen (`crew/src/lib.rs:583-593`).

## Rules

- **core.1.** Work starts only as an offer's `flume launch`, an act's Flume command or a command's `sluice branch create`. Each result is carried as the tool gave it, and the runner judges only whether a place has landed.
- **core.2.** Of `crates/flywheel/src` and all of `crates/flywheel-control`, only `src/planes.rs` names `flywheel_crew`, which `rules.rs` holds, reading the tree as Task notes 9.2 has it. `flywheel-control` depends on no implementation.
- **core.3.** `no_listener` reads every crate's sources and `Cargo.lock`, and finds no listener. Every connection is git's or the tools' own.
- **core.4.** Neither table holds a credential. `token` is refused in the runner's configuration and in crew's table, and its value is never printed. `ghp_n0t4r34lt0k3n` in the configuration tests is a stand-in that proves a value is never echoed, as before.
- **core.5.** No key names a stream or a kind, and no `--kinds` is passed. The amended detail is in 22d7630 with the design.
- **core.6.** Nothing commercial appears in the diff.
- **core.7.** The three crates are moon projects of their own, one task per check. `root:format` passed over every tracked file, with Flume's and sluice's fixtures excluded by name.
- **core.8.** recutils joins git and openspec among the pins `check-toolchain.sh` checks.
- **core.9, core.10, core.11, core.12.** These are held by the room, lease, once-only, tool-records, terminal-relay and configuration scenarios above, with the exceptions findings 3 and 4 name.

## Observations

None of these holds the change back on its own. Each was read in the code and not run, and the code stage may weigh them while it is there:
- **A refused lease's offer, given again, is taken again.** The runner keeps no record of the ids a report refused. An offer given again under a refused lease is found by its mark and answered launched (`src/carry.rs:247-253`), and an act on it waits rather than being refused. `control/fake` keeps offering a refused id, and `leases::a_lease_refused` takes the offer out of the pull.
- **Held words for a place Flume dropped.** An offer waiting on a launch whose place Flume dropped after the launch reads "launched at P, not landed" (`src/carry.rs:340-349`). Only a drop before the launch reads "dropped".
- **A lapsed lease makes control passes run back to back.** A past expiry leaves no time (`src/carry.rs::left`), so `src/carry.rs::next_due` falls to the end of the pass. A failed pull keeps the saved report's expiries, so a control plane whose leases expire would see the runner pass back to back while it is unreachable. crew's leases never expire.
- **A brief left by a killed launch.** A runner killed while `flume launch` runs leaves the brief's file (`src/flume.rs:127-140`). After a restart the launch is found by its mark, and the file stays.
- **A failed `git ls-remote` settles a branch command.** It settles the command with no origin commit and words naming the failure (`src/carry.rs:587-614`), and origin is not read again.
- **A launch request crew cannot read is never let go.** Such a request naming this host is given as an act the runner refuses (`crew/src/read.rs:419`). `let_go` lets an act go only by its `Of`, so the answer stays in `answers.rec`.
- **A claim stranded by a narrowed configuration.** A claim stays on the runner's branch, and every other runner reads it as an answer, when the runner's configuration stops keeping the claimed launch's repository (`crew/src/lib.rs:308-310`).
- **Refusals inside `[control_plane]`.** `kinds` or `stream` written directly under `[control_plane]` gets the plain unknown-key refusal, without "Flume's host configuration" (`src/config.rs:385-399`). At the top level, and in crew's table, the words are given.
- **Old wording.** "No views setting" and "The clone is not under the views root" keep the old name in their titles. The views spec's Purpose, which no delta touches, still says "under the views root".
