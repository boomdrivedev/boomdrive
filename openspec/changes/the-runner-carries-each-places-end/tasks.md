# Tasks

Work through the tasks in order. A task is true once the check it names passes, run as it names it. The groups are headings for reading.

- This change takes Flume `v0.20.0` (`9ab0e2c`), which carries the bolt `a-tidied-places-end-stays-readable` (`72107a2`) and Flume record 0060, and follows its spellings: `flume launch --recipient` and `flume inbox <place> --recipient` (design Context).
- Every test runs on the crate's fakes and stubs, in a temporary folder, with neutral names and paths (core.41). No test runs a Flume of the host's: every Flume a test runs is a fake, a stub, or the set's on a scratch host.
- No task adds a crate, a tool, a tracked file type, a configuration key, a field of the report or a message of the protocol (core.7, core.8). Nothing outside the files proposal.md's Touches lists is changed.
- Task 1.1 writes record 0030, core.53 and the amendments to the details of core.1, core.13 and core.15. No later task writes them again.

## 1. The set's Flume holds a place's end for its caller

- [x] 1.1 Record 0030 and core.53 say that the runner is one more recipient, `--recipient boomdrive`, of each launch that names recipients, and none of one that names none, and that it collects a place's held record by `flume inbox <record> --recipient boomdrive` once its report holds it, under the least Flume `0.20.0`, as the user's answer on agentplot-design's card of 2026-11-08 ("Runner as one more recipient now; one shared inbox in weir next") and agentplot-design's words carrying it decide. The details of core.1 and core.15 say the same, and this change's proposal, design and specs say it (design Decision 1; Task notes 1.1). True when `sluice exec main -- moon run root:adrs root:changes` passes.
- [x] 1.2 The set names Flume `v0.20.0`, the first release that holds a place's end for a recipient no pane holds, its tag moved with the lock, Flume's git crates' hashes at that tag and README's override example. `flume::LEAST` is that release's version (design Decisions 5 and 6; Task notes 1.2). True when `devenv shell -- cargo test -p boomdrive --test family each_tools_version_is_its_tags` passes.

## 2. Flume's command line

- [x] 2.1 Every `flume launch` of a launch that names recipients names `--recipient boomdrive` after them, from `flume::RECIPIENT`, and one of a launch that names none names no `--recipient`. The launch's own recipients are passed as it gives them. This proves the work-into-flume spec's "The runner is one more recipient" at the command line (design Decision 1; Task notes 2.1). True when `devenv shell -- cargo test -p boomdrive --test flume -- a_launch_with_every_flag a_launch_naming_no_stream_priority_or_intent the_runner_is_one_more_recipient flume_runs_with_no_recipient_of_the_runners_own` passes.
- [x] 2.2 `Flume::collect(record)` runs `flume --json inbox <record> --recipient boomdrive`, naming no repository, and reads the answer by Flume's exit as every call does. `FakeFlume` and `tests/once.rs`'s `Watching` implement it as design Decision 8 has it (design Decisions 3 and 8). True when `devenv shell -- cargo test -p boomdrive --test flume a_collection_names_one_record_and_the_runner` passes.
- [x] 2.3 `flume::runs_with` takes `flume --version`'s first line at or above `LEAST`, a suffix after its patch ignored. It refuses one below, or one naming no version, with design Decision 5's words, and `flume::recipients` names the runner after a launch's recipients and none where it names none (design Decisions 1 and 5). True when `devenv shell -- cargo test -p boomdrive --lib flume::` passes.
- [x] 2.4 The set's Flume, on a scratch host, refuses a launch naming `--recipient boomdrive` beside another recipient, and an inbox naming a place it does not hold with `--recipient boomdrive`, each with its reason on exit 1. `flume::runs_with` takes the set's `--version`. This proves the family-packages spec's "The set's Flume takes the runner as a recipient" (design Decision 6). True when `devenv shell -- cargo test -p boomdrive --test family the_sets_flume_takes_the_runner_as_a_recipient` passes.

## 3. Launches name the runner their caller

- [x] 3.1 The control pass's launches that name recipients name the runner one more. This proves the work-into-flume spec's "A unit's launch", "A launch naming no stream, priority or intent", "A launch's environment and brief" and "The runner is one more recipient" (design Decision 1). True when `devenv shell -- cargo test -p boomdrive --test offers -- a_units_launch a_launch_naming_no_stream_priority_or_intent a_launchs_environment_and_brief the_runner_is_one_more_recipient` passes.
- [x] 3.2 A bolt and its unit, which name no recipient, are launched naming none of the runner's. This proves the work-into-flume spec's "A bolt and its unit" and "A bolt counts against its stream's room within its pass" (design Decision 1). True when `devenv shell -- cargo test -p boomdrive --test bolts -- a_bolt_and_its_unit a_bolt_counts_against_its_streams_room_within_its_pass` passes.

## 4. Each place's end, held and collected

- [x] 4.1 `facts::pending_for(entry, name)` is true only where the entry's phase is `tidied` and its `pending_for` names `name` (design Decision 7). True when `devenv shell -- cargo test -p boomdrive --lib facts::` passes.
- [x] 4.2 Once the report file holds the statuses and entries carried, `Work::collect` collects each record pending for the runner, before the report is sent. This proves the work-into-flume spec's "A landing held for the runner" and "A drop held for the runner" (design Decisions 2 and 3; Task notes 4.2). True when `devenv shell -- cargo test -p boomdrive --test statuses -- a_landing_held_for_the_runner a_drop_held_for_the_runner` passes.
- [x] 4.3 The report file holds the held record when the inbox for it runs. This proves the rest of "A landing held for the runner" (design Decision 3; Task notes 4.3). True when `devenv shell -- cargo test -p boomdrive --test once a_held_record_is_collected_once_its_report_holds_it` passes.
- [x] 4.4 Only a record pending for the runner is collected, and a tidied record pending for a recipient alone is carried as before. This proves the work-into-flume spec's "A record pending for a recipient too", "A record pending for no end of the runner's", "A place tidied" and "A place tidied while the pass answers its wants" (design Decision 3). True when `devenv shell -- cargo test -p boomdrive --test statuses -- a_record_pending_for_a_recipient_too a_record_pending_for_no_end_of_the_runners a_place_tidied a_place_tidied_while_the_pass_answers_its_wants` passes.
- [x] 4.5 A collection Flume refuses, or cannot be run for, is said in one line and made again at the next pass. This proves the work-into-flume spec's "A collection Flume could not run" and "A collection Flume refuses" (design Decision 3). True when `devenv shell -- cargo test -p boomdrive --test statuses -- a_collection_flume_could_not_run a_collection_flume_refuses` passes.
- [x] 4.6 No record is collected for a launch whose lease the control plane refused. This proves the work-into-flume spec's "A lease refused before its end was collected" and "A lease refused" (design Decision 3; core.10). True when `devenv shell -- cargo test -p boomdrive --test leases -- a_lease_refused_before_its_end_was_collected a_lease_refused` passes.
- [x] 4.7 An offer after a place Flume tidied, landed, and holds for the runner is launched at the next pass. This proves the work-into-flume spec's "A dependent of a landing held for the runner" and "A dependent held, then launched" (design Decision 2). True when `devenv shell -- cargo test -p boomdrive --test after -- a_dependent_of_a_landing_held_for_the_runner a_dependent_held_then_launched` passes.

## 5. The drain

- [x] 5.1 `report::ended` reads a place tidied or gone as ended only where its record is not pending for `flume::RECIPIENT` (design Decision 4). True when `devenv shell -- cargo test -p boomdrive --lib report::` passes.
- [x] 5.2 A drained repository's clone and view stay until the place's held record is collected. This proves the clones spec's "The place's end held for the runner", "The place tidied" and "The place no longer listed" (design Decision 4; Task notes 5.2). True when `devenv shell -- cargo test -p boomdrive --test dropped -- the_places_end_held_for_the_runner the_place_tidied the_place_no_longer_listed` passes.

## 6. The least Flume

- [ ] 6.1 Below the least Flume no offer is launched, and the runner says so once until Flume's version changes. `Least` is kept by the runner and by `Bench`, beside `Reach`. This proves the work-into-flume spec's "A Flume older than the least", "A Flume at the least" and "Said once while it lasts" (design Decision 5; Task notes 6.1). True when `devenv shell -- cargo test -p boomdrive --test offers -- a_flume_older_than_the_least a_flume_at_the_least said_once_while_it_lasts` passes.
- [ ] 6.2 `tests/pass.rs` builds its `Carrier` with a `Least`. A Flume whose `--version` printed nothing, read as missing, has its launches tried as before (design Decision 5). True when `devenv shell -- cargo test -p boomdrive --test pass` passes.

## 7. What the runner says of it

- [ ] 7.1 README's control pass section says that the runner is one more recipient, `--recipient boomdrive`, of each launch that names recipients, so Flume holds each such place's end for it. It also says the runner carries the held record as the place's status and collects it by `flume inbox <record> --recipient boomdrive` once its report holds it, that a drained clone waits for the collection, and that no offer is launched below the least Flume, `0.20.0` (design Decisions 1 to 5). True when `sluice exec main -- moon run root:format` passes.

## 8. The specs

- [ ] 8.1 The change's deltas archive into specs that pass openspec's strict check. True when, after `devenv shell -- openspec archive the-runner-carries-each-places-end --yes`, `sluice exec main -- moon run root:specs` passes.
