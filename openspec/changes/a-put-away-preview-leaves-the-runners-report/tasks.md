# Tasks

Build the tasks in order. A task is true once the check it names passes, run as it names it. The groups are headings, for reading.

- Every test of the runner runs on the scratch host of `crates/boomdrive/tests/fakes`, and every test of crew's implementation on its scratch state repository, with neutral names and paths (core.41), and no real Flume or sluice.
- No task adds a crate, a tool or a tracked file type (core.7, core.8). Nothing outside the paths proposal.md's Touches lists is changed.
- Record 0031's amendment is already written with the change. A task that relies on a choice the design names cites its Decision.

## 1. The runner drops a preview let go

- [x] 1.1 `crates/boomdrive/tests/previews.rs` gains `a_preview_let_go`, as Task notes 1.1 have it. The runner's code is unchanged: `carry.rs`'s `forget` already drops a preview let go and runs nothing against it. True when `devenv shell -- cargo test -p boomdrive --test previews a_preview_let_go` passes, proving the control-plane-interface spec's "A preview let go".

## 2. crew's implementation lets a put-away preview go

- [x] 2.1 `Main::plans` (`crates/boomdrive-crew/src/read.rs`) reads `bolt/<bolt>` as in the plan while `plan.rec` holds `Bolt: <bolt>`, and its documentation says so (design Decision 3; Task notes 2.1). `tests/letgo.rs` gains `a_bolt_landed_and_gone_from_the_plan`, as `a_unit_landed_and_gone_from_the_plan` is:
  - `main_with` of `bolt/b`, the plan holding the bolt `b`;
  - then the plan holding no bolt, the place still listed, its let-go empty;
  - then the place tidied, its let-go `["home/12", "home/13"]`, and `answers.rec` and `places.rec` holding neither.

  True when `devenv shell -- cargo test -p boomdrive-crew --test letgo a_bolt_landed_and_gone_from_the_plan` passes, proving the crew-control-plane spec's "A bolt landed and gone from the plan".
- [ ] 2.2 `Pulling::current` (`crates/boomdrive-crew/src/lib.rs`) gives a preview request that reads by its own rule (design Decision 2; Task notes 2.2):
  - where the runner holds it, it is given;
  - otherwise it is given only while it is the newest of its `Host`, `Repo` and `Revision`;
  - a bring-up so given only while its work is in the plan;
  - a done so given whatever the plan holds, but only while the runner holds a bring-up of its key.

  `read.rs` gains `Request::preview_key` and `Main::newest_preview`.

  `tests/read.rs` changes `a_preview_marked_done`: its pull is handed a report holding `home/30` answered `done`, and gives `home/30` and `home/31`, done. It gains:
  - `a_preview_and_its_done_the_runner_does_not_hold`: requests 30 and 31, its done, of `unit/u`, the plan holding `u`, no report and no branch. The pull gives neither.
  - `a_done_of_a_landed_bolts_preview`: requests 30 of `bolt/y` and 31, its done, a report holding `home/30` answered `done` and listed, the plan holding no bolt. The pull gives `home/30` and `home/31`.
  - `a_done_of_a_preview_written_before_a_later_bring_up`: requests 30, 31 and 32, the report holding `home/30` answered `done`, the plan holding `u`. The pull gives `home/30` and `home/32`, and not `home/31`.
  - `a_preview_of_a_bolt_gone_that_the_runner_does_not_hold`: request 30 of `bolt/y`, no report and no branch, the plan holding no bolt. The pull gives nothing.

  `tests/letgo.rs`'s `a_preview_whose_work_has_left_the_plan` finds its pull gives `home/30` and `home/31`, by the done's rule.

  True when `devenv shell -- cargo test -p boomdrive-crew preview` passes. It proves the crew-control-plane spec's "A preview marked done", "A preview and its done the runner does not hold", "A done of a landed bolt's preview", "A done written before a later bring-up" and "A preview of a bolt gone that the runner does not hold".
- [ ] 2.3 `let_go` (`crates/boomdrive-crew/src/write.rs`) lets each done the report holds go, whatever the plan holds. With it go the bring-ups the report holds before it of its `Host`, `Repo` and `Revision`. It lets a bring-up go by today's rule, its work gone and nothing up, with no done counted in that rule (design Decisions 1 and 4; Task notes 2.3).

  In `tests/letgo.rs`, `a_preview_put_away_its_work_gone` keeps only its second half, the plan holding no `u`, its let-go `["home/30", "home/31"]` and `previews.rec` the descriptor alone. Its first half, the plan holding `u` and nothing let go, holds no more. The file gains:
  - `a_preview_put_away_its_work_still_in_the_plan`, the plan holding `u`, its let-go `["home/30", "home/31"]`;
  - `a_preview_brought_up_after_its_done_stays`;
  - `a_preview_sluice_no_longer_lists_its_work_gone`;
  - `a_refused_preview_its_bolt_landed`;
  - `a_refused_preview_its_bolt_still_in_the_plan`.

  `a_preview_still_up_its_work_gone` stands as it is.

  True when `devenv shell -- cargo test -p boomdrive-crew --test letgo preview` passes. It proves the crew-control-plane spec's "A preview put away, its work still in the plan", "A bring-up written after the done stays", "A preview put away, its work gone", "A preview sluice no longer lists, its work gone", "A preview still up, its work gone", "A refused preview, its bolt landed" and "A refused preview, its bolt still in the plan".
- [ ] 2.4 crew's implementation passes the lint, its new functions documented, `let_go`'s and `Pulling::current`'s documentation citing record 0031's amendment. True when `sluice exec main -- moon run boomdrive-crew:lint` passes.

## 3. The words

- [ ] 3.1 The README's section on crew's implementation says when a preview leaves the runner's branch, beside `previews.rec` (design Decision 6). It leaves once its done is answered, with what the done put away. It also leaves once its work has left the plan and it is no longer up, a bolt's work leaving with its `Bolt` record. `tests/readme.rs`'s `the_readme_names_how_a_preview_is_brought_up_and_put_away` also fails where the flattened README lacks `leaves the runner's branch once its done is answered`. True when `devenv shell -- cargo test -p boomdrive --test readme the_readme_names_how_a_preview_is_brought_up_and_put_away` passes.
- [ ] 3.2 The runner's tests that tasks 1.1 and 3.1 touched pass the lint. True when `sluice exec main -- moon run boomdrive:lint` passes.
