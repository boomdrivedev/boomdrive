# Tasks

Build the groups in order: each stands on the ones before it. A task is true once the check it names passes, run as it names it.

- Every test runs on its crate's fakes, stubs or scratch host, in a temporary folder, with neutral names and paths (core.41). No test reads the user's configuration, touches a real place, clone or state repository, or reaches the network.
- No task adds a crate, a tool or a tracked file type (core.7, core.8). Nothing outside the files proposal.md's Touches lists is changed.

## 1. The interface records each launch's repository and the repositories drained

- [x] 1.1 `boomdrive_control::Launched` gains `repository: Option<String>`, `#[serde(default, skip_serializing_if = "Option::is_none")]`, and `Hello` gains `draining: Vec<String>`, `#[serde(default)]`, each documented as design Decisions 1 and 3 say; every construction of either in every crate's sources and tests names the new field (Task notes 1.1). `crates/boomdrive-control/tests/json.rs`'s `a_report` and `a_hello` hold the new fields, and `a_report_from_before_reads` proves the control-plane-interface spec's "A report from before". True when `devenv shell -- cargo test -p boomdrive-control --test json a_report_from_before_reads -- --exact` passes.

## 2. What has ended, and what holds a repository

- [x] 2.1 `facts::ended` reads a place's entry ended where its phase is `tidied` or `gone` (design Decision 2). True when `devenv shell -- cargo test -p boomdrive --lib facts::tests::a_place_that_has_ended -- --exact` passes, reading `tidied` and `gone` ended and `live`, `over` and a delivered wait not.
- [x] 2.2 `report::holding` gives each launch of a report, with its place, that holds an address (design Decision 2). True when `devenv shell -- cargo test -p boomdrive --lib report::tests::the_places_that_hold_a_repository -- --exact` passes, giving a launched place not ended, of the address, and none ended, not listed, of another address, held or refused, or recording no repository.

## 3. The control pass drains, and follows a place only through its clone

- [x] 3.1 `Keeping` gains `Dropped(PathBuf)` and `Clones` gains `dropped()`, `Configured` giving both, and the fakes follow: `FakeClones::drop_named` keeps a made clone as `Dropped`, and `FakeClones::gone` forgets the address (design Decision 3; Task notes 3.1). The unit test `a_clone_the_configuration_drops_is_not_kept` becomes `a_clone_the_configuration_drops_is_dropped`. True when `devenv shell -- cargo test -p boomdrive --lib carry::tests::a_clone_the_configuration_drops_is_dropped -- --exact` passes.
- [x] 3.2 The report keeps each launch's repository from its first answer, and gains it for a launch of before at the first pull that gives it (design Decision 1). True when `devenv shell -- cargo test -p boomdrive --test offers a_launchs_repository_kept_with_its_answer -- --exact` passes, proving the work-into-flume spec's "A launch's repository kept with its answer".
- [x] 3.3 The hello names the repositories drained apart from those kept (design Decision 3). True when `devenv shell -- cargo test -p boomdrive --test offers a_repository_draining_in_the_hello -- --exact` passes, proving the spec's "A repository draining in the hello".
- [x] 3.4 An offer for a repository the runner neither keeps nor drains is refused before its mark is read (design Decision 4). True when `devenv shell -- cargo test -p boomdrive --test offers a_mark_in_a_repository_not_kept -- --exact` passes, proving the spec's "A mark in a repository not kept".
- [x] 3.5 An offer in a repository drained is refused but where its mark is found, and a branch command or an inbox there is answered as one in a repository not kept (design Decision 4). True when `devenv shell -- cargo test -p boomdrive --test offers a_repository_drained -- --exact` passes, proving the spec's "A repository drained".
- [ ] 3.6 A place's status is carried through its repository's clone, named or dropped, always with `--repository`, and a landing in a dropped repository is told to no keeper (design Decision 5). `crates/boomdrive/tests/statuses.rs` loses `a_place_whose_repository_the_runner_keeps_no_more` and `a_place_found_tidied_by_its_mark` with the requirement they proved. True when `devenv shell -- cargo test -p boomdrive --test statuses a_place_in_a_repository_drained -- --exact` passes, proving the spec's "A place in a repository drained" and "A landing in a repository not kept".
- [ ] 3.7 A launch whose repository's clone does not stand has no status asked for (design Decision 5). True when `devenv shell -- cargo test -p boomdrive --test statuses a_place_in_a_repository_with_no_clone -- --exact` passes, proving the spec's "A place in a repository with no clone".
- [ ] 3.8 sluice's entry is read in the clone of the launch's repository, named or dropped (design Decision 5). True when `devenv shell -- cargo test -p boomdrive --test entries an_entry_carried_from_a_repository_drained -- --exact` passes, proving the tool-records spec's "An entry carried from a repository drained".

## 4. The keeper drains a dropped repository

- [ ] 4.1 The keeper reads the report file once a pass, keeps a dropped clone and view while a launch holds the repository, and names each place and launch that holds it (design Decision 2; Task notes 4.1). True when `devenv shell -- cargo test -p boomdrive --test dropped dropped_while_a_place_is_live -- --exact` passes, proving the clones spec's "Dropped while a place is live".
- [ ] 4.2 A drained repository whose place is tidied is removed at the next pass. True when `devenv shell -- cargo test -p boomdrive --test dropped the_place_tidied -- --exact` passes, proving the clones spec's "The place tidied".
- [ ] 4.3 A drained repository whose place stands not listed is removed at the next pass. True when `devenv shell -- cargo test -p boomdrive --test dropped the_place_no_longer_listed -- --exact` passes, proving the clones spec's "The place no longer listed".
- [ ] 4.4 A report file that cannot be read lets no dropped repository go, and names it. True when `devenv shell -- cargo test -p boomdrive --test dropped a_report_that_cannot_be_read -- --exact` passes, proving the clones spec's "A report that cannot be read".

## 5. crew's implementation gives a launch only for a repository kept or drained

- [ ] 5.1 A launch no runner answered is given only for a repository the hello keeps, never one it drains (design Decision 6). True when `devenv shell -- cargo test -p boomdrive-crew --test claims an_unanswered_launch_in_a_repository_draining -- --exact` passes, proving the crew-control-plane spec's "A repository draining".
- [ ] 5.2 A launch the runner answered is given at the address of a repository the hello keeps, else drains (design Decision 6). True when `devenv shell -- cargo test -p boomdrive-crew --test claims a_launch_answered_in_a_repository_draining -- --exact` passes, proving the spec's "A launch answered in a repository draining".
- [ ] 5.3 A launch the runner answered whose `Repo` the hello neither keeps nor drains is given nothing, and the arm that gave it with its `Repo` as written goes (design Decision 6). `crates/boomdrive-crew/tests/claims.rs` loses `a_launch_answered_before_its_repository_left_the_hello` and `a_launch_the_report_answers_is_given_whatever_the_hello_names` with the requirement they proved. True when `devenv shell -- cargo test -p boomdrive-crew --test claims a_launch_answered_before_its_repository_was_renamed -- --exact` passes, proving the spec's "A launch answered before its repository was renamed".
- [ ] 5.4 A launch naming this host resolves its `Repo` among the kept repositories, then the drained (design Decision 6). True when `devenv shell -- cargo test -p boomdrive-crew --test claims a_launch_naming_this_host_in_a_repository_draining -- --exact` passes, proving the spec's "A launch naming this host in a repository draining".

## 6. The hosted implementation sends no drained repository

- [ ] 6.1 No partition's hello names a repository drained, and no report a launch's repository (design Decision 3; Task notes 6.1). True when `devenv shell -- cargo test -p boomdrive-hosted --test plane a_repository_draining_goes_up_in_no_hello -- --exact` passes, proving the hosted-control-plane spec's "A repository drained goes up in no hello".

## 7. The README and the specs

- [ ] 7.1 README says a dropped repository is drained while a place the runner launched there has not ended, its place followed through its clone, and that no rename is followed, in place of the paragraph that follows a place by its location alone. True when `sluice exec main -- moon run root:format` passes.
- [ ] 7.2 The spec delta archives into specs that pass openspec's strict check. True when, after `openspec archive a-dropped-repository-is-drained-and-no-rename-is-followed --yes`, `sluice exec main -- moon run root:specs` passes.
