# Tasks

Build the tasks in order. A task is true once the check it names passes, run as it names it. The groups are headings, for reading.

- Every test of the runner runs on the scratch host of `crates/boomdrive/tests/fakes`, and every test of crew's implementation on its scratch state repository, with neutral names and paths (core.41), and no real Flume or sluice.
- No task adds a crate, a tool or a tracked file type (core.7, core.8). Nothing outside the paths proposal.md's Touches lists is changed.
- Record 0031 and core.55 are already written with the change. A task that relies on a choice the design names cites its Decision.

## 1. The interface

- [x] 1.1 `boomdrive-control`'s `Want` gains `Preview(Preview)`: `id`, `lease`, `repository`, `branch`, `revision` and `done`, written as `{"preview": {…}}`. `Want::id` gives its id, and `lib.rs` re-exports `Preview` (design Decision 1). True when `devenv shell -- cargo test -p boomdrive-control --test json a_preview_of_each_kind` passes, proving the control-plane-interface spec's "A preview of each kind".
- [x] 1.2 `Report` gains `previews: BTreeMap<Id, Previewed>`, serde `default` and skipped where empty. `Previewed` is as design Decision 9 has it. `Report::leases` names each preview's lease, and `Outcome`'s documentation says a preview is `held`, `done` or `refused`. `shared.rs`'s report takes `previews`. True when `devenv shell -- cargo test -p boomdrive-control --test json` passes with:
  - `a_report_holding_a_preview`, writing and reading back a report holding a preview answered `held` with a carried status, under `"previews"`;
  - `a_report_from_before_reads`, finding a report's JSON with no `previews` read as holding none and written back with no `previews`;
  - `a_previews_lease_among_the_reports_leases`, finding a report holding launch `a` under lease 3 and preview `p` under lease 5 give `a` with 3 and `p` with 5 as its leases.

  These prove the control-plane-interface spec's "A report holding a preview", "A report from before" and "A preview's lease among the report's leases".

## 2. Version 1 carries no preview

- [x] 2.1 The hosted implementation takes a preview as design Decision 11 has it:
  - `own_wants` withholds one naming a repository its account is not served;
  - `prefixed_want` gives its id the partition's prefix;
  - `part` takes the partition's previews, and `taken_back` gives none;
  - `upward` is unchanged, so no `previews` goes up.

  `tests/interface.rs` gains `a_preview_is_no_want_of_version_1`, which checks a preview want, as `boomdrive-control`'s types write it, against version 1's `want` with `common::refused`. `the_interfaces_values_pass`'s documentation says every want sort but a preview passes. True when `devenv shell -- cargo test -p boomdrive-hosted --test interface` passes, proving the runner-protocol spec's "A preview is no want of version 1" and "The interface's values pass".

## 3. The control pass takes a preview

- [ ] 3.1 `carry.rs` takes `Want::Preview` (design Decisions 7 and 8):
  - in the first round, a preview the report holds `done` or `refused` is answered from the report, its lease the pull's, and one under a lapsed lease is left untaken;
  - a preview not marked done in a repository the runner neither keeps nor drains, or one it drains, is answered `refused` with `this host's configuration does not keep <repository>`;
  - one whose clone is not yet made is left unanswered;
  - `forget` drops a preview;
  - a preview whose report the control plane refuses is named as left on this host as sluice holds it.

  Every `Report { … }` the crate's sources and tests write field by field names `previews`. `tests/fakes/mod.rs` gains `preview(id, branch, revision, done)` (Task notes 3.1 and group 5). True when `devenv shell -- cargo test -p boomdrive --test previews` passes with `a_preview_in_a_repository_not_kept`, `a_preview_in_a_repository_whose_clone_is_not_yet_made` and `a_preview_whose_lease_lapsed`, proving the previews spec's scenarios of those names.

## 4. sluice's word of a preview

- [ ] 4.1 `crates/boomdrive/tests/fixtures/sluice-0.74.3/` holds `preview-starting.json`, `preview-no-table.json`, `previews.json`, `release.json` and `release-nothing.json`. Each is the set's sluice's own answer, captured on a scratch host byte for byte, with a README saying how and what stood, as design Decision 14 and Task notes 4.1 have it. `tests/sluice.rs`'s `every_fixture_is_read` reads them. True when `devenv shell -- cargo test -p boomdrive --test sluice every_fixture_is_read` passes.
- [ ] 4.2 `sluice.rs` gains `preview_carried` and `release_carried` (design Decision 4). True when `devenv shell -- cargo test -p boomdrive --test sluice` passes with:
  - `a_preview_carried`: `preview-starting.json` carried as its `name`, `branch`, `revision`, `state`, `made` and `addresses` in that order, each as the fixture prints it;
  - `a_failed_previews_output_is_left_out`: `previews.json`'s failed row carried with no `output`;
  - `a_release_carried`: `release.json` carried as its `preview` and `released`.

  These prove the previews spec's "A failed preview's output left out".
- [ ] 4.3 The `Sluice` trait and `Cli` gain `preview`, `previews` and `release`, each run in the clone's container (design Decisions 2, 3, 5 and 6; Task notes 4.3):
  - `preview` is awaited no longer than the patience, `PREVIEW_PATIENCE`, 20 seconds, which `Cli` takes another of for a test. sluice is then sent SIGINT and awaited. An exit with no document after the interrupt is an answer that sluice holds the preview waiting.
  - `libc` becomes a dependency of `crates/boomdrive`.
  - The stub `sluice` waits where a test asks.

  True when `devenv shell -- cargo test -p boomdrive --test sluice` passes with:
  - `a_preview_is_asked_in_the_clones_container`: `preview main R1 --json`, run in the container;
  - `an_ask_past_the_patience_is_interrupted_and_waits`: patience one second, the stub waiting, the call returning within a few seconds as waiting;
  - `previews_are_listed_in_the_clones_container`: `previews --json`;
  - `a_release_is_asked_by_name`: `release <name> --json`.

## 5. The control pass brings up, carries and puts away a preview

- [ ] 5.1 A preview not marked done, in a repository kept with its clone made, is asked of sluice in the second round, once, and answered as design Decision 2 has it, with its lines. True when `devenv shell -- cargo test -p boomdrive --test previews` passes with `a_preview_brought_up`, `a_preview_sluice_refuses` and `a_preview_given_twice`, proving the previews spec's scenarios of those names.
- [ ] 5.2 A preview sluice holds waiting is answered `held`, said once, and asked again at each pass until sluice answers it (design Decision 3). True when `devenv shell -- cargo test -p boomdrive --test previews a_preview_waiting_at_sluices_bound` passes, proving the previews spec's "A preview waiting at sluice's bound".
- [ ] 5.3 After the rounds, sluice's listing of each preview held or brought up is read once a pass per repository and carried as its status, until sluice lists it no more. A listing that cannot be read leaves each status as it was, said in one line, and a preview's command sets no `ran` (design Decision 5). True when `devenv shell -- cargo test -p boomdrive --test previews` passes with `a_preview_comes_up`, `a_preview_released_by_a_person`, `a_listing_that_cannot_be_read` and `one_listing_for_two_previews_of_one_repository`, proving the previews spec's scenarios of those names.
- [ ] 5.4 A preview marked done puts away, by one `sluice release`, only what the report holds of its repository and revision asked of sluice. A held preview so put away is answered `refused` and asked no more. A done with none there, or no clone standing, is refused, running nothing (design Decision 6). True when `devenv shell -- cargo test -p boomdrive --test previews` passes with `a_preview_put_away`, `a_done_of_a_preview_never_asked_for`, `a_held_preview_put_away` and `a_bring_up_and_its_done_in_one_pull`, proving the previews spec's scenarios of those names.
- [ ] 5.5 Once the next pass is due, the second round runs no preview's command but its first, leaves the rest to that pass, and says how many it left, its count apart from the acts' (design Decision 7). True when `devenv shell -- cargo test -p boomdrive --test previews` passes with `a_launch_ahead_of_a_preview` and `previews_wait_for_the_next_pass_once_it_is_due`, proving the previews spec's scenarios of those names, and `devenv shell -- cargo test -p boomdrive --test acts acts_on_places_wait_for_the_next_pass_once_it_is_due` still passes, its acts' count untouched.
- [ ] 5.6 A preview whose report the control plane refuses leaves the report, its preview left as sluice holds it and named, with nothing run (design Decision 8). True when `devenv shell -- cargo test -p boomdrive --test previews a_previews_lease_refused` passes, proving the previews spec's "A preview's lease refused".
- [ ] 5.7 The runner's sources pass the lint, `libc`'s use among them. True when `sluice exec main -- moon run boomdrive:lint` passes.

## 6. crew's implementation gives a preview and writes its answer

- [ ] 6.1 `read::want` gives `Act: preview` as a preview, its `Repo` resolved as a launch naming this host's, `Branch` and `Revision` required, and `Done` read as `Submission` is (design Decision 10). `tests/common/mod.rs` gains `preview(…)` and the descriptor's fields (Task notes 6.1). True when `devenv shell -- cargo test -p boomdrive-crew --test read` passes with `a_preview`, `a_preview_marked_done` and `a_preview_with_no_revision`, proving the crew-control-plane spec's scenarios of those names.
- [ ] 6.2 A preview naming no `Host` is given nothing, written nothing of, and named in one line (design Decision 10). True when `devenv shell -- cargo test -p boomdrive-crew --test read a_preview_naming_no_host` passes, proving the crew-control-plane spec's "A preview naming no host".
- [ ] 6.3 A preview the report or this host's `previews.rec` answers counts as the runner's in `Pulling`'s `reported` and `branch_answers`, so it is given while its work has left the plan (design Decision 10). True when `devenv shell -- cargo test -p boomdrive-crew --test letgo a_preview_whose_work_has_left_the_plan` passes, proving the crew-control-plane spec's "A preview whose work has left the plan".
- [ ] 6.4 `write.rs` gains crew's descriptor of a `Preview`, `PREVIEWS`, and writes `previews.rec` as design Decision 10 has it. The file is written where the partition's report holds a preview, and as the descriptor alone where the tip holds one and the report none. True when `devenv shell -- cargo test -p boomdrive-crew --test write` passes with:
  - `a_preview_brought_up_is_written_in_previews_rec`;
  - `a_branch_with_no_preview`, finding no `previews.rec`;
  - `nothing_changed` and `only_the_runners_branch`, `previews.rec` among the files checked by `recfix --check` where written.

  These prove the crew-control-plane spec's "A preview brought up" and "A branch with no preview".
- [ ] 6.5 `previews.rec` is read with the runner's other files and checked as they are. `read_back` takes the hello and gives each `Preview` back as its preview (design Decision 10; Task notes 6.5). True when `devenv shell -- cargo test -p boomdrive-crew --test write a_preview_read_back` passes, proving the crew-control-plane spec's "A preview read back".
- [ ] 6.6 `let_go` names a preview once its work has left the plan and it holds no preview up, and the report's write leaves it out of `previews.rec` (design Decision 10). True when `devenv shell -- cargo test -p boomdrive-crew --test letgo` passes with `a_preview_put_away_its_work_gone` and `a_preview_still_up_its_work_gone`, proving the crew-control-plane spec's scenarios of those names.
- [ ] 6.7 crew's implementation passes the lint. True when `sluice exec main -- moon run boomdrive-crew:lint` passes.

## 7. The words

- [ ] 7.1 The README's control pass says how a preview is brought up, carried and put away, and that no output of its command is carried (design Decision 13). Its section on crew's implementation names `previews.rec` beside the runner's three files. `tests/readme.rs` gains `the_readme_names_how_a_preview_is_brought_up_and_put_away`, which fails where the README names neither `sluice preview <branch> <revision>` nor `sluice release <name>`. True when `devenv shell -- cargo test -p boomdrive --test readme` passes.
- [ ] 7.2 `sluice.rs`'s and `carry.rs`'s module documentation say the runner asks sluice for a preview, its listing and its release, and cite core.55. `want.rs`'s and `report.rs`'s documentation of `Preview` and `Previewed` say version 1 of the hosted protocol names neither (core.42). True when `sluice exec main -- moon run boomdrive-control:lint` passes.
- [ ] 7.3 The spec deltas archive into specs that pass openspec's strict check, the previews spec made. True when, after `devenv shell -- openspec archive the-runner-brings-up-a-preview-its-control-plane-asks-for --yes`, `sluice exec main -- moon run root:specs` passes.
