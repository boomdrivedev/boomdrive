# Verify: a-runner-reports-to-an-observer-it-takes-no-work-from

The work on `change/a-runner-reports-to-an-observer-it-takes-no-work-from` holds, read at 56f43c5, and the change is archived. Every task is ticked. The gate, run once through sluice, passed every task. The change has 13 requirements and 34 scenarios, and every scenario is met: a test asserts each one's outcome, and for one scenario the code holds part of it, as noted below.

## Checks

`sluice exec main` ran the gate once, at 56f43c5, against main at 2ea1a48. It ran in sluice's sandbox with `network = "localhost"`, and exited 0.

- It ran the 23 `check-fast` tasks the branch affects, and all 23 passed:
  - the format, lint, typecheck and test tasks of `boomdrive`, `boomdrive-control`, `boomdrive-crew` and `boomdrive-hosted`;
  - `nix:typecheck` and `nix:test`, which said "every case holds";
  - `root:format`, `root:specs`, `root:changes`, `root:releases` and `root:versions`.
- The site's tasks did not run, since the branch changes nothing the `site` project owns.
- 1157 tests ran. None failed and none was ignored.
- `root:changes` found this change, the one open change, valid under `openspec validate --strict`, and found that it archives to specs that pass the strict check (core.61).
- `root:specs` passed all 27 specs.
- sluice recorded the 23 results under 56f43c5.

moon warned that the sandbox could not write its output archives. The tasks' results were unaffected. Its other warning, no internet connection, only skipped its dependency install.

## Tasks

All 15 tasks of `tasks.md`, 1.1 to 8.1, are ticked `[x]`, and none is open. Each test a task names ran in the gate and passed:

| Task | Tests |
|---|---|
| 1.1 | `interface::an_observing_hello_and_an_observed_report_are_of_version_3` |
| 1.2 | `files::version_3_is_version_2_but_for_its_hold_and_observing` |
| 1.3 | `refused::a_field_the_version_does_not_name` |
| 1.4 | `examples::every_example_read_and_written_back` |
| 2.1 | `table::tests::an_observer_read`, `an_observer_off_the_host`, `an_observer_with_a_token`, `an_observers_key_name` |
| 2.2 | `enrollment::an_observer_enrolled`, `key::an_observers_key_kept_by_systemd_creds` |
| 3.1 | `fake::tests::an_observer_holds_what_it_is_handed` |
| 4.1 | `observer::the_observing_hello_first`, `an_offer_from_an_observer`, `a_report_from_an_observer`, `a_terminal_from_an_observer`, `an_observers_outage` |
| 4.2 | `observer::the_places_observed`, `the_observing_hello_names_the_partitions_repositories`, `a_report_answered_and_let_go`, `a_pulled_wakes_nothing`, `an_observer_down_holds_up_nothing`, `back_again_the_newest_alone`, `the_hosted_observers` |
| 5.1 | `config::tests::an_observer_beside_crew`, `an_observer_of_no_control_plane`, `an_observer_off_the_host`; `local_layer::an_observer_in_the_local_layer` |
| 5.2 | `enroll::an_observer_enrolled_beside_crew`, `no_observer_named` |
| 6.1 | `control::a_part_observed`, `an_observer_of_no_part`, `an_observer_unreachable`; `once::one_pass_observes_nothing` |
| 7.1 | `examples::the_observer_table_reads` |
| 7.2, 7.3 | `root:format` |
| 8.1 | `root:specs`, `root:changes` |

What the tasks build is in the tree:

- **Version 3.**
  - `protocol/v3/` gains `observing_hello`, `observed_report` and `observed_launch`.
  - Its `hello` and `report` kinds each take a `oneOf` of the working form and the observing one, and `message_refused`'s description names an observing connection.
  - It gains the three examples Decision 1 lists.
  - No file under `protocol/v1/` or `protocol/v2/` changes.
- **Types.** `src/protocol/v3.rs` reads each kind's body as the observing form first, then the working one. `Observing` writes and reads `true` alone.
- **The observer.** `src/observer.rs` holds `open`, `observers` and `Observing`:
  - The hello names only the partition's repositories of those kept, with no capacity or lease.
  - The report holds the share's launches alone, each cut by `plane::cut_launch`, the function `Partition::upward` uses.
  - Every answer is dropped.
- **The connection.** `src/connection.rs`'s observing role refuses a `pulled` holding work, and the four terminal messages, with `OBSERVES`, saying one line for each. It wakes nothing, and its lines say "the observer".
- **The table and enrollment.** `src/table.rs`'s `Role` sets:
  - the table's name in a refusal;
  - the loopback-only `url`;
  - the key's `observer.` prefix;
  - the enroll command a refusal names.
- **The interface.** `boomdrive-control` holds `Observer`, `Observed` and the fake `Observing`.
- **The runner.**
  - `config.rs` holds `Config::observer`, refused where the file names no `[control_plane]`.
  - `planes.rs` holds `observers`, `read_observer` and `enroll_observer`.
  - `run.rs` opens the observers once the parts are opened, pairs each with its part by label, and says once each that has no part.
  - `carry.rs`'s `Carrier::observed_pass` hands the observer the pass's hello and share after the send and the confirms. `run --once` passes `None`.
- **The command.** `boomdrive enroll --observer` is in `main.rs` and `enroll.rs`.
- **The docs.**
  - README's hosted section has a "Reporting to an observer" part. `tests/examples/observer.toml` is its example.
  - The protocol's README names the observing hello and the observed report. *Version 2* says version 2 has no observing connection.
  - Record 0052's amendment of 2026-11-18, and core.42's and core.10's details, are written.

The branch changes only what the proposal's Touches lists. Its merge base with `bolt/a-hosted-control-plane-can-hold-a-waiting-place` is f0e481e, as the design says.

## Scenarios

Each scenario's test was read against its WHEN and THEN. `x::y` is the test `y` in the crate's `tests/x.rs`, and `src/m.rs::y` is a unit test in that module. The crate is `boomdrive-hosted` unless the row names it.

### runner-protocol (3 requirements, 9 scenarios)

| Scenario | Proved by | Note |
|---|---|---|
| Version 3 is version 2 but for its hold | `files::version_3_is_version_2_but_for_its_hold_and_observing` |  |
| Version 3's examples are version 2's | `files::version_3_is_version_2_but_for_its_hold_and_observing` |  |
| A field the version does not name | `refused::a_field_the_version_does_not_name` | adds `reason` to the observing hello and to an observed launch |
| A time not in UTC to the second | `forms::a_time_not_in_utc_to_the_second` | unchanged by this change |
| An integer past what JavaScript reads exactly | `forms::an_integer_past_what_javascript_reads_exactly` | unchanged by this change |
| The observing connection's examples | `interface::an_observing_hello_and_an_observed_report_are_of_version_3` |  |
| An observing hello that offers room or holds a lease | `interface::an_observing_hello_and_an_observed_report_are_of_version_3` |  |
| An observed report that names a lease or an act | `interface::an_observing_hello_and_an_observed_report_are_of_version_3` |  |
| No observing connection in version 1 or 2 | `interface::an_observing_hello_and_an_observed_report_are_of_version_3` |  |

### hosted-control-plane (7 requirements, 15 scenarios)

| Scenario | Proved by | Note |
|---|---|---|
| The observer read | `src/table.rs::an_observer_read` |  |
| An observer off the host | `src/table.rs::an_observer_off_the_host` |  |
| A token in the observer table | `src/table.rs::an_observer_with_a_token` |  |
| An observer enrolled beside crew | `enrollment::an_observer_enrolled`; `boomdrive` `enroll::an_observer_enrolled_beside_crew` | Split as task note 5.2 gives it. The hosted test asserts the request at the observer's `runner/enroll`, its `enroll` passing version 3, the key under `observer.home` and host `h-1`. The command's test asserts that under `type = "crew"`, `--observer` reaches the observer's enrollment and is refused as unreachable, not as crew's. |
| Kept apart from a partition of the same label | `key::an_observers_key_kept_by_systemd_creds` | `--name=boomdrive.observer.home.localhost` |
| No observer named | `boomdrive` `enroll::no_observer_named` | exit 2, naming `[observer]` |
| The observing hello | `observer::the_observing_hello_first`, `observer::the_observing_hello_names_the_partitions_repositories` |  |
| The places observed | `observer::the_places_observed` | the share holds act `home/2`, and the report holds no act |
| An offer from an observer | `observer::an_offer_from_an_observer` |  |
| A terminal from an observer | `observer::a_terminal_from_an_observer` | The test asserts the four `message_refused` answers, their lines, and that no `terminal_closed` is sent. "Makes no key" holds in the code: `refuse_work` sends the refusal and does nothing else. |
| A report's answer that lets go | `observer::a_report_answered_and_let_go` |  |
| A pulled wakes nothing | `observer::a_pulled_wakes_nothing` | asserts the wake the observer was opened beside is not called |
| An observer down | `observer::an_observer_down_holds_up_nothing` | each handing under 100 ms |
| Back again | `observer::back_again_the_newest_alone` |  |
| An observer's outage | `observer::an_observers_outage` | also asserts no wake |

### configuration (1 requirement, 4 scenarios)

| Scenario | Proved by | Note |
|---|---|---|
| An observer beside crew | `boomdrive` `src/config.rs::an_observer_beside_crew` |  |
| An observer of no control plane | `boomdrive` `src/config.rs::an_observer_of_no_control_plane` |  |
| An observer in the local layer | `boomdrive` `local_layer::an_observer_in_the_local_layer` |  |
| An observer off the host | `boomdrive` `src/config.rs::an_observer_off_the_host` |  |

### control-plane-interface (1 requirement, 2 scenarios)

| Scenario | Proved by | Note |
|---|---|---|
| An observer takes what it is handed | `boomdrive-control` `src/fake.rs::an_observer_holds_what_it_is_handed` |  |
| The hosted observers | `observer::the_hosted_observers` |  |

### running (1 requirement, 4 scenarios)

| Scenario | Proved by | Note |
|---|---|---|
| A part observed | `boomdrive` `control::a_part_observed` | each held hello equals the pass's, the last share equals the last report, and nothing of `work`'s is held |
| An observer unreachable | `boomdrive` `control::an_observer_unreachable` | the hosted observer at a loopback port with nothing on it; launched and reported within 10 s |
| An observer of no part | `boomdrive` `control::an_observer_of_no_part` |  |
| One pass | `boomdrive` `once::one_pass_observes_nothing` |  |

## Rules

The diff was read against the rules the design cites:

- core.2: only `boomdrive-hosted` names the protocol, and `planes.rs` alone reaches it.
- core.19, core.1: the observer is handed no wake, and its connection refuses every want and terminal.
- core.26: the observing hello names the partition's repositories alone.
- core.42: version 1 and 2 are unchanged, and every observing form is closed.
- core.49: `observe` returns at once.
- core.54: no end is confirmed on an observer's answer.

No line names a value: a refusal of work names its kind and id alone.

## Archive

The change is archived with `openspec archive a-runner-reports-to-an-observer-it-takes-no-work-from --yes`, this report with it.
