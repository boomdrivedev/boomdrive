# Verify: a-put-away-preview-leaves-the-runners-report

The work on `change/a-put-away-preview-leaves-the-runners-report`, read at 06e50c4, meets its change, which is archived with this report beside it.

- The gate ran once, through sluice (`sluice exec main`), and passed. It ran the 13 `check-fast` tasks that 06e50c4 affects against `main` (cee700c). 629 tests passed, none failed and none was ignored.
- Every task of `tasks.md` is ticked.
- The change has 8 requirements and 21 scenarios across two specs. For each scenario, a test that passed in the gate asserts its outcome.
- The branch was cut from `bolt/a-put-away-preview-leaves-the-runners-report`, which stands at ecbe5fa, an ancestor of 06e50c4. `git merge-tree` merges the branch with the bolt and with `main` with no conflict.

## The gate

`devenv shell -- sluice exec main` exited 0, and sluice recorded 13 results for 06e50c4. The tasks were:

- `boomdrive`: format, lint, typecheck and test;
- `boomdrive-crew`: format, lint, typecheck and test;
- `nix`: typecheck and test;
- `root`: format, adrs and changes.

sluice judged the rest of the layer unaffected by the change. moon warned that it could not archive `boomdrive:test`'s outputs into its cache, and the OTLP exporter failed to shut down. Neither touches a result.

## Tasks

All 7 tasks of `tasks.md`, in groups 1 to 3, are ticked `[x]`, and none is open. What they build is in the tree, as the design names it:

- **The runner drops a preview let go** (1.1). `tests/previews.rs` gains `a_preview_let_go`. The runner's code is unchanged, as the task says.
- **`bolt/<b>` leaves the plan with its `Bolt` record** (2.1). `Main::plans` gains the arm `["bolt", bolt]`, and its documentation names `bolt/<bolt>` beside `fix/<bolt>/<name>` (Decision 3).
- **A preview request is given while crew still asks for it, or while the runner holds it** (2.2). `read.rs` gains:
  - `Request::preview_key`, the request's `Host`, `Repo` and `Revision` as written;
  - `Request::preview_done`, whether a preview that reads is marked done;
  - `Main::newest_preview`.

  `Pulling::current` hands a preview request that reads to `Pulling::preview_current`, which follows the Task notes' order:
  - held, given;
  - else not newest, not given;
  - else a done, given where an earlier bring-up of its key is held;
  - else a bring-up, given while its work is in the plan.

  A preview request that does not read falls to the old rule, as Decision 2 says.
- **A done answered is let go with what it put away** (2.3). `let_go` sweeps the report's previews twice:
  - first each done, with each bring-up before it of its key;
  - then each other bring-up whose work has left the plan and which is not up, a done no longer counted in that rule (Decisions 1 and 4).
- **The lint and the documentation** (2.4, 3.2). `let_go`, `Pulling::current`, `Pulling::preview_current`, `Main::plans` and the new functions cite record 0031 as amended on 2026-11-09. Both crates' lint passed in the gate.
- **The words** (3.1). The README's section on crew's implementation says, beside `previews.rec`, that a preview "leaves the runner's branch once its done is answered, with what the done put away; or once its work has left crew's plan and it is no longer up, a bolt's work leaving with its `Bolt` record". `tests/readme.rs` checks for that phrase.

Record 0031's amendment of 2026-11-09, proposed, is in 5ade074. It records Decisions 1 to 4 and the paths not taken, and reads as the code does. No rule changes: core.55 stands.

**Paths changed against Touches.** `git diff --name-only main...HEAD` changes only the paths that the proposal's Touches lists, and the change's own directory. `openspec/specs/` changes at the archive.

## Scenarios

### control-plane-interface

| requirement | scenario | test |
|---|---|---|
| A report's answer names what is let go and what is refused | A report not sent | `boomdrive` `offers.rs` `a_report_not_sent` |
| | A launch let go | `boomdrive` `leases.rs` `a_launch_let_go` |
| | A preview let go | `boomdrive` `previews.rs` `a_preview_let_go` |

`a_preview_let_go` brings `home/30` up and has the fake let it go. It then asserts four things:

- the line `home/30: let go by the control plane`;
- no `home/30` in the runner's report, nor in the last report sent;
- no `release` call at either later pass;
- one `preview` call in all.

### crew-control-plane

| requirement | scenario | test (`boomdrive-crew`) |
|---|---|---|
| A preview request the runner does not hold is given only while it is crew's newest of its host, repository and revision | A preview and its done the runner does not hold | `read.rs` `a_preview_and_its_done_the_runner_does_not_hold` |
| | A done written before a later bring-up | `read.rs` `a_done_of_a_preview_written_before_a_later_bring_up` |
| A done is given while the runner holds a bring-up of it, whatever the plan holds | A done of a landed bolt's preview | `read.rs` `a_done_of_a_landed_bolts_preview` |
| A done answered is let go with the previews it put away | A preview put away, its work still in the plan | `letgo.rs` `a_preview_put_away_its_work_still_in_the_plan` |
| | A bring-up written after the done stays | `letgo.rs` `a_preview_brought_up_after_its_done_stays` |
| A preview request is a preview of what it names | A preview | `read.rs` `a_preview` |
| | A preview marked done | `read.rs` `a_preview_marked_done` |
| A launch is let go once its work has left crew's plan | A unit landed and gone from the plan | `letgo.rs` `a_unit_landed_and_gone_from_the_plan` |
| | A unit still in the plan | `letgo.rs` `a_unit_still_in_the_plan` |
| | A bolt landed and gone from the plan | `letgo.rs` `a_bolt_landed_and_gone_from_the_plan` |
| | A fix of main | `letgo.rs` `a_fix_of_main` |
| A request whose work has left crew's plan is given only while the runner holds it | Work gone that the runner does not hold | `drops.rs` `work_gone_that_the_runner_does_not_hold` |
| | A preview of a bolt gone that the runner does not hold | `read.rs` `a_preview_of_a_bolt_gone_that_the_runner_does_not_hold` |
| A preview is let go once its work has left crew's plan and it holds no preview up | A preview put away, its work gone | `letgo.rs` `a_preview_put_away_its_work_gone` |
| | A preview sluice no longer lists, its work gone | `letgo.rs` `a_preview_sluice_no_longer_lists_its_work_gone` |
| | A preview still up, its work gone | `letgo.rs` `a_preview_still_up_its_work_gone` |
| | A refused preview, its bolt landed | `letgo.rs` `a_refused_preview_its_bolt_landed` |
| | A refused preview, its bolt still in the plan | `letgo.rs` `a_refused_preview_its_bolt_still_in_the_plan` |

Each modified requirement's delta keeps every scenario the main spec holds under it, so the archive drops none. The main spec's "A preview whose work has left the plan" falls under a requirement this change does not modify, and still holds. `letgo.rs` `a_preview_whose_work_has_left_the_plan` gives `home/30` and, by the done's rule, `home/31`.

## Findings

None that sends the change back.
