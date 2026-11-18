# Tasks

Work through the tasks in order. A task is true once the check it names passes, run as it names it. The groups are headings for reading.

- Record 0052's amendment of 2026-11-18, and core.42's and core.10's details, are already written with the change. core.19 was amended with record 0048.
- Version 3 is unreleased, so it gains the observing connection (design Decision 1). No file under `crates/boomdrive-hosted/protocol/v1/` or `protocol/v2/` changes (core.42). A task that would edit one is wrong.
- Every test runs on the crates' fakes, in a temporary folder, with neutral names and paths (core.41). No task runs a real Flume, sluice, control plane or observer, or reads the host's own credential store: a test's key is in `key::Memory`, or a `systemd-creds` stub.
- No line the runner says, and no failure a check gives, names a value of a message: kinds, ids and paths alone (core.11).
- No task adds a crate, a tool or a tracked file type (core.7, core.8). Nothing outside the paths proposal.md's Touches lists is changed. A task that relies on a choice the design names cites its Decision.

## 1. Version 3 carries an observing connection

- [x] 1.1 Version 3's `hello.schema.json` gains `observing_hello`, and its `report.schema.json` gains `observed_report` and `observed_launch`. Its `from-runner.schema.json`'s `hello` and `report` kinds each take either form, and `shared.schema.json`'s `message_refused` description names an observing connection. Its examples gain `from-runner/hello-observing.json`, `from-runner/report-observed.json` and `to-runner/pulled-answering-an-observing-hello.json`. `tests/interface.rs` gains `an_observing_hello_and_an_observed_report_are_of_version_3`. This proves the runner-protocol spec's "The observing connection's examples", "An observing hello that offers room or holds a lease", "An observed report that names a lease or an act" and "No observing connection in version 1 or 2" (design Decision 1; Task notes 1.1). True when `devenv shell -- cargo test -p boomdrive-hosted --test interface an_observing_hello_and_an_observed_report_are_of_version_3` passes.
- [x] 1.2 `tests/files.rs`'s `version_3_is_version_2_but_for_its_hold` becomes `version_3_is_version_2_but_for_its_hold_and_observing`. It sets aside the observing forms and the two kinds' choice, and holds each to its working form as design Decision 9 gives it. Version 3's examples are version 2's, the hold's and the three of the observing connection, exactly. This proves the runner-protocol spec's "Version 3 is version 2 but for its hold" and "Version 3's examples are version 2's" (design Decision 9; Task notes 1.2). True when `devenv shell -- cargo test -p boomdrive-hosted --test files version_3_is_version_2_but_for_its_hold_and_observing` passes.
- [x] 1.3 `tests/refused.rs`'s `a_field_the_version_does_not_name` adds a field to the observing hello and to the observed report's launch, each refused. This proves the runner-protocol spec's "A field the version does not name" (design Decision 1; Task notes 1.3). True when `devenv shell -- cargo test -p boomdrive-hosted --test refused a_field_the_version_does_not_name` passes.
- [x] 1.4 `src/protocol/v3.rs` gains `ObservingHello`, `ObservedReport` and `ObservedLaunch`, and `FromRunner`'s `Hello` and `Report` each take either form. `src/check.rs` gains `observing_hello()` and `observed_report()`. This keeps the runner-protocol spec's "Every example read and written back" with the new examples among them (design Decision 1; Task notes 1.4). True when `devenv shell -- cargo test -p boomdrive-hosted --test examples` passes.

## 2. The observer table and its enrollment

- [x] 2.1 `src/table.rs`'s `Table` gains its role, and `Table::read_observer` reads `[observer]`: a `url` at a loopback address alone, and its partitions, each refusal naming `[observer]`. `key_name` gives an observer's key the label `observer.<label>`. This proves the hosted-control-plane spec's "The observer read", "An observer off the host" and "A token in the observer table" (design Decisions 3 and 4; Task notes 2.1). True when `devenv shell -- cargo test -p boomdrive-hosted --lib table::tests` passes.
- [ ] 2.2 `src/lib.rs` gains `enroll_observer`, an observer's partition enrolled as a hosted partition is, its key under `observer.<label>`. `tests/enrollment.rs` gains `an_observer_enrolled`, and `tests/key.rs` gains `an_observers_key_kept_by_systemd_creds`. This proves the hosted-control-plane spec's "An observer enrolled beside crew", the enrollment itself, and "Kept apart from a partition of the same label" (design Decision 4; Task notes 2.2). True when `devenv shell -- cargo test -p boomdrive-hosted --test enrollment --test key observer` passes.

## 3. The interface's observer

- [ ] 3.1 `boomdrive-control` gains `Observer` and `Observed`, and its `fake` module a fake observer, `Observing`, that holds what it is handed. This proves the control-plane-interface spec's "An observer takes what it is handed" (design Decision 5; Task notes 3.1). True when `devenv shell -- cargo test -p boomdrive-control an_observer_holds_what_it_is_handed` passes.

## 4. The hosted observer

- [ ] 4.1 `src/connection.rs`'s `Dial` and `Connection` take an observing role. Its hello and report are checked against the observing forms, and a `pulled` holding a want or a report is refused and said, as is every terminal message. Its wake does nothing, and its lines name the observer. `tests/observer.rs`, new, holds the connection's tests. This proves the hosted-control-plane spec's "The observing hello", "An offer from an observer", "A terminal from an observer" and "An observer's outage" (design Decision 6; Task notes 4.1). True when `devenv shell -- cargo test -p boomdrive-hosted --test observer -- the_observing_hello_first an_offer_from_an_observer a_report_from_an_observer a_terminal_from_an_observer an_observers_outage` passes.
- [ ] 4.2 `src/observer.rs`, new, holds `open` and `Observing`. `Observing` gives the observing hello of its partition's repositories and the observed report of the share's launches, cut by the function `plane.rs`'s `Partition::upward` now shares. It waits on nothing and drops every answer. `src/lib.rs` names the module. This proves the hosted-control-plane spec's "The places observed", "A report's answer that lets go", "A pulled wakes nothing", "An observer down" and "Back again", and the control-plane-interface spec's "The hosted observers" (design Decisions 2, 6 and 8; Task notes 4.2). True when `devenv shell -- cargo test -p boomdrive-hosted --test observer` passes.

## 5. The configuration and the command

- [ ] 5.1 `crates/boomdrive/src/config.rs` holds `[observer]` as `Config::observer`, read by `planes::read_observer`. It is refused where the file names no `[control_plane]`, and a local layer that sets it is not taken. `planes.rs` gains `read_observer`. This proves the configuration spec's "An observer beside crew", "An observer of no control plane", "An observer in the local layer" and "An observer off the host" (design Decision 3; Task notes 5.1). True when `devenv shell -- cargo test -p boomdrive --lib --test local_layer an_observer` passes.
- [ ] 5.2 `boomdrive enroll --observer <label>` enrolls the observer's partition through `planes::enroll_observer`, whatever `[control_plane]`'s `type`, and is refused with exit 2 where no `[observer]` or no such partition is named. This proves the hosted-control-plane spec's "No observer named", and the command's half of "An observer enrolled beside crew" (design Decision 4; Task notes 5.2). True when `devenv shell -- cargo test -p boomdrive --test enroll -- an_observer_enrolled_beside_crew no_observer_named` passes.

## 6. The runner hands each pass to its observer

- [ ] 6.1 `planes.rs` gains `observers`. `run.rs` opens the observers at start beside the parts, gives each part the observer of its label in `keep`, and says once each observer of no part. `carry.rs`'s `Carrier::pass` hands the part's observer its hello and share once its report is sent. `Runner::observe_with` puts a test's observers in place. This proves the running spec's "A part observed", "An observer unreachable", "An observer of no part" and "One pass" (design Decisions 7 and 8; Task notes 6.1). True when `devenv shell -- cargo test -p boomdrive --test control --test once -- a_part_observed an_observer_of_no_part an_observer_unreachable one_pass_observes_nothing` passes.

## 7. What the runner says of it

- [ ] 7.1 `crates/boomdrive/tests/examples/observer.toml` is README's example of `[observer]`, and `tests/examples.rs`'s `the_observer_table_reads` reads it (design Decision 10; Task notes 7.1). True when `devenv shell -- cargo test -p boomdrive --test examples the_observer_table_reads` passes.
- [ ] 7.2 README's hosted section describes an observer as design Decision 10 says: its table and its example's copy, its enrollment, what it is sent, that it is given and waits on nothing, and its lines (design Decision 10). True when `sluice exec main -- moon run root:format` passes.
- [ ] 7.3 `crates/boomdrive-hosted/protocol/README.md` describes the observing hello and the observed report. Its messages table and a paragraph name them, and its list of what version 3 has no message for drops an observing connection. *Version 2* names the observing connection among how version 3 differs (design Decision 10). True when `sluice exec main -- moon run root:format` passes.

## 8. The specs

- [ ] 8.1 The change's deltas archive into specs that pass openspec's strict check. True when, after `devenv shell -- openspec archive a-runner-reports-to-an-observer-it-takes-no-work-from --yes`, `sluice exec main -- moon run root:specs root:changes` passes.
