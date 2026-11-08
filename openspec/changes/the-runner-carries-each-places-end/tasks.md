# Tasks

Work through the tasks in order. A task is true once the check it names passes, run as it names it. The groups are headings for reading.

- This change waits on the Flume unit that agentplot-planner places ahead of it, which releases what design.md's "What Flume lacks" names. Task 1.1 takes that release; where Flume spells a flag otherwise than this change asks, follow Flume's, and change this change's specs and README to match (design Context).
- Every test runs on the crate's fakes and stubs, in a temporary folder, with neutral names and paths (core.41). No test runs a Flume of the host's: every Flume a test runs is a fake, a stub, or the set's on a scratch host.
- No task adds a crate, a tool, a tracked file type, a configuration key, a field of the report or a message of the protocol (core.7, core.8). Nothing outside the files proposal.md's Touches lists is changed.
- Record 0030, core.53, and the amendments to the details of core.1, core.13 and core.15 are in the commit that holds this change. No task writes them again.

## 1. The set's Flume holds a place's end for its caller

- [ ] 1.1 The set names the first Flume release that holds a place's end for its caller, its tag moved with the lock, Flume's git crates' hashes at that tag and README's override example. `flume::LEAST` is that release's version (design Decisions 5 and 6; Task notes 1.1). True when `devenv shell -- cargo test -p boomdrive --test family each_tools_version_is_its_tags` passes.

## 2. Flume's command line

- [ ] 2.1 Every `flume launch` ends `--mark <mark> --caller boomdrive`, from `flume::CALLER`. The launch's `--recipient`s are those it gives, and none of the runner's own. This proves the work-into-flume spec's "Every launch names the runner its caller" at the command line (design Decision 1; Task notes 2.1). True when `devenv shell -- cargo test -p boomdrive --test flume -- a_launch_with_every_flag a_launch_naming_no_stream_priority_or_intent every_launch_names_the_runner_its_caller flume_runs_with_no_recipient_of_the_runners_own` passes.
- [ ] 2.2 `Flume::collect(record)` runs `flume --json inbox --caller boomdrive <record>`, naming no repository, and reads the answer by Flume's exit as every call does. `FakeFlume` and `tests/once.rs`'s `Watching` implement it as design Decision 8 has it (design Decisions 3 and 8). True when `devenv shell -- cargo test -p boomdrive --test flume a_collection_names_the_runners_caller_and_one_record` passes.
- [ ] 2.3 `flume::runs_with` takes `flume --version`'s first line at or above `LEAST`, a suffix after its patch ignored. It refuses one below, or one naming no version, with design Decision 5's words (design Decision 5). True when `devenv shell -- cargo test -p boomdrive --lib flume::` passes.
- [ ] 2.4 The set's Flume, on a scratch host, refuses a launch with `--caller boomdrive` and an inbox with `--caller boomdrive` for a place it does not hold, each with its reason on exit 1. `flume::runs_with` takes the set's `--version`. This proves the family-packages spec's "The set's Flume takes the runner's caller" (design Decision 6). True when `devenv shell -- cargo test -p boomdrive --test family the_sets_flume_takes_the_runners_caller` passes.

## 3. Launches name the runner their caller

- [ ] 3.1 The control pass's launches name the runner their caller. This proves the work-into-flume spec's "A unit's launch", "A launch naming no stream, priority or intent", "A launch's environment and brief" and "Every launch names the runner its caller" (design Decision 1). True when `devenv shell -- cargo test -p boomdrive --test offers -- a_units_launch a_launch_naming_no_stream_priority_or_intent a_launchs_environment_and_brief every_launch_names_the_runner_its_caller` passes.
- [ ] 3.2 A bolt and its unit are launched naming the runner their caller. This proves the work-into-flume spec's "A bolt and its unit" and "A bolt counts against its stream's room within its pass" (design Decision 1). True when `devenv shell -- cargo test -p boomdrive --test bolts -- a_bolt_and_its_unit a_bolt_counts_against_its_streams_room_within_its_pass` passes.

## 4. Each place's end, held and collected

- [ ] 4.1 `facts::kept_for(entry, name)` is true only where the entry's phase is `tidied` and its `pending_for` names `name` (design Decision 7). True when `devenv shell -- cargo test -p boomdrive --lib facts::` passes.
- [ ] 4.2 Once the report file holds the statuses and entries carried, `Work::collect` collects each record kept for the runner, before the report is sent. This proves the work-into-flume spec's "A landing held for the runner" and "A drop held for the runner" (design Decisions 2 and 3; Task notes 4.2). True when `devenv shell -- cargo test -p boomdrive --test statuses -- a_landing_held_for_the_runner a_drop_held_for_the_runner` passes.
- [ ] 4.3 The report file holds the held record when the inbox for it runs. This proves the rest of "A landing held for the runner" (design Decision 3; Task notes 4.3). True when `devenv shell -- cargo test -p boomdrive --test once a_held_record_is_collected_once_its_report_holds_it` passes.
- [ ] 4.4 Only a record kept for the runner is collected, and a tidied record kept for a recipient alone is carried as before. This proves the work-into-flume spec's "A record kept for a recipient too", "A record kept for no end of the runner's", "A place tidied" and "A place tidied while the pass answers its wants" (design Decision 3). True when `devenv shell -- cargo test -p boomdrive --test statuses -- a_record_kept_for_a_recipient_too a_record_kept_for_no_end_of_the_runners a_place_tidied a_place_tidied_while_the_pass_answers_its_wants` passes.
- [ ] 4.5 A collection Flume refuses, or cannot be run for, is said in one line and made again at the next pass. This proves the work-into-flume spec's "A collection Flume could not run" (design Decision 3). True when `devenv shell -- cargo test -p boomdrive --test statuses -- a_collection_flume_could_not_run a_collection_flume_refuses` passes.
- [ ] 4.6 No record is collected for a launch whose lease the control plane refused. This proves the work-into-flume spec's "A lease refused before its end was collected" and "A lease refused" (design Decision 3; core.10). True when `devenv shell -- cargo test -p boomdrive --test leases -- a_lease_refused_before_its_end_was_collected a_lease_refused` passes.
- [ ] 4.7 An offer after a place Flume tidied, landed, and holds for the runner is launched at the next pass. This proves the work-into-flume spec's "A dependent of a landing held for the runner" and "A dependent held, then launched" (design Decision 2). True when `devenv shell -- cargo test -p boomdrive --test after -- a_dependent_of_a_landing_held_for_the_runner a_dependent_held_then_launched` passes.

## 5. The drain

- [ ] 5.1 `report::ended` reads a place tidied or gone as ended only where its record is not kept for `flume::CALLER` (design Decision 4). True when `devenv shell -- cargo test -p boomdrive --lib report::` passes.
- [ ] 5.2 A drained repository's clone and view stay until the place's held record is collected. This proves the clones spec's "The place's end held for the runner", "The place tidied" and "The place no longer listed" (design Decision 4; Task notes 5.2). True when `devenv shell -- cargo test -p boomdrive --test dropped -- the_places_end_held_for_the_runner the_place_tidied the_place_no_longer_listed` passes.

## 6. The least Flume

- [ ] 6.1 Below the least Flume no offer is launched, and the runner says so once until Flume's version changes. `Least` is kept by the runner and by `Bench`, beside `Reach`. This proves the work-into-flume spec's "A Flume older than the least", "A Flume at the least" and "Said once while it lasts" (design Decision 5; Task notes 6.1). True when `devenv shell -- cargo test -p boomdrive --test offers -- a_flume_older_than_the_least a_flume_at_the_least said_once_while_it_lasts` passes.
- [ ] 6.2 `tests/pass.rs` builds its `Carrier` with a `Least`. A Flume whose `--version` printed nothing, read as missing, has its launches tried as before (design Decision 5). True when `devenv shell -- cargo test -p boomdrive --test pass` passes.

## 7. What the runner says of it

- [ ] 7.1 README's control pass section says that every launch names the runner its caller, `--caller boomdrive`, and that Flume holds each place's end for the runner. It also says the runner carries the held record as the place's status and collects it once its report holds it, that a drained clone waits for the collection, and that no offer is launched below the least Flume, which it names (design Decisions 1 to 5). True when `sluice exec main -- moon run root:format` passes.

## 8. The specs

- [ ] 8.1 The change's deltas archive into specs that pass openspec's strict check. True when, after `devenv shell -- openspec archive the-runner-carries-each-places-end --yes`, `sluice exec main -- moon run root:specs` passes.
