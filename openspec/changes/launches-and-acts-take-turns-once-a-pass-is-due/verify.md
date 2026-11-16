# Verify: launches-and-acts-take-turns-once-a-pass-is-due

The work holds. The gate passes, all nine tasks are ticked, and a test proves each scenario of the two delta specs. `openspec validate launches-and-acts-take-turns-once-a-pass-is-due --strict` finds the change valid.

This report is on `change/launches-and-acts-take-turns-once-a-pass-is-due` at `310f45a`, cut from `main` at `98d1909`.

## The whole checks

`sluice exec main` ran once on `310f45a`. It ran the checks that commit affects against `main` (`98d1909`) and exited 0. All 9 passed, and sluice recorded their results under their keys for `310f45a`:

- `boomdrive`: `format`, `lint`, `typecheck` and `test`;
- `nix`: `typecheck` and `test` ("every case holds");
- `root`: `format`, `adrs` and `changes`.

`boomdrive:test` ran 41 test files and failed no test, among them every test the tasks name. moon also warned that it could not write its output archives under `.moon/cache/outputs`, and that it skipped the Rust dependency install for want of a network. Both warnings come from the sandbox, and no check failed because of them.

## Tasks

All nine tasks are ticked, 1.1 to 4.1, and none is open. Each names `cargo test` runs of tests that passed in `boomdrive:test` above. Task 4.1 names two greps of `README.md`, which print lines 1066 and 1114.

The diff from `98d1909` stays inside proposal.md's Touches and the change's own directory: `carry.rs`, the four test files, `README.md`, record 0002 and `core.md`. It adds no crate, tool or tracked file type. Record 0002's amendment of 2026-11-14 on taking turns, and core.50's and core.55's details, are in the change's first commit, `e2b7a0b`, as the tasks say. The code and tests cite them and change neither.

## The delta, scenario by scenario

### work-into-flume, added

| Scenario | Proven by |
|---|---|
| A pass due from its start makes one launch | `turns.rs` `a_pass_due_from_its_start_makes_one_launch`: Flume is asked to launch `a` alone. `b` and `c` are held with `TURN`, each said in one line. At the next pass `b` is launched and `c` stays held, said in no line |
| A pass that falls due among its launches makes none more | `turns.rs` `a_pass_that_falls_due_among_its_launches_makes_none_more`: the pass falls due once two launches are made. `a` and `b` are launched, and `c` and `d` are held with no `flume launch` |
| A launch held for its turn counts against its stream's room | `turns.rs` `a_launch_held_for_its_turn_counts_against_its_streams_room`: in stream `work`, with room 2, `a` is launched, `b` held, and `c` left unanswered |
| A launch that waited on its clone's keep is held without waiting on it | `offers.rs` `a_launch_that_waited_on_its_clones_keep_is_held_without_waiting_on_it`: `b` into `agentplot/sluice` is launched, `a` is held with `TURN`, and the keeper is still running when the pass ends |
| A drop of a launch held for its turn | `turns.rs` `a_drop_of_a_launch_held_for_its_turn`: `a` is launched, the drop is done, and `b` is done "dropped by b/drop before it was launched", with no `flume launch` for `b` |
| An act waits behind one launch once the pass is due | `turns.rs` `an_act_waits_behind_one_launch_once_the_pass_is_due`: Flume's calls are `launch a`, then `retry submission=true` at `x`'s place, with `b` held. At the next pass they are `launch b`, then `Priority(5)`, with `c` held |
| A pass that falls due among its launches runs its first act next | `turns.rs` `a_pass_that_falls_due_among_its_launches_runs_its_first_act_next`: Flume is asked for `launch a`, `launch b` and `Priority(3)` at `x`'s place, with `c` and `d` held. `y`'s priority is unanswered, and a line says 1 act on a place waits |

### work-into-flume, modified

| Scenario | Proven by |
|---|---|
| A pass past due sends a launch's answer before its act runs | `sends.rs` `a_pass_past_due_sends_a_launchs_answer_before_its_act_runs`: three reports are sent. The first goes after `a`'s launch and before any act, with `a` launched and `b` and `c` held. The second and third, at the answer and at the end, each hold the priority done |
| A pass past due sends each launch's answer before its next launch | `sends.rs` `a_pass_past_due_sends_each_launchs_answer_before_its_next_launch`: over three passes, `a`, `b` and `c` are launched one per pass. Each pass sends once after its last want and once at its end, before the next pass's launch |
| A pass not yet due sends once | `sends.rs` `a_pass_not_yet_due_sends_once_at_its_end`, unchanged, passing |
| An act's answer goes up before the next act runs | `sends.rs` `an_acts_answer_goes_up_before_the_next_act_runs`, unchanged, passing under the send before each command |
| A want let go at a send of the pass is taken no more | `sends.rs` `a_want_refused_at_a_send_of_the_pass_is_taken_no_more`: `c` is refused, and the first report goes before any launch, holding that refusal. `a` is let go, named on its line with its place left. Flume is asked to launch `b` alone, and `a` is not in the report |
| An act whose launch is let go at the send before it runs nothing | `sends.rs` `an_act_whose_launch_is_let_go_at_the_send_before_it_runs_nothing`: `b` is launched, and the send before the priority comes after one launch and no act. No `Act` or `StatusOf` call names `x`'s place. Neither act is in the report or in the end's send |
| A send that fails leaves the pass's answers to its end | `sends.rs` `a_send_that_fails_leaves_the_passs_answers_to_its_end`: two sends are tried, the one before the priority and the end's, and none once the last want is answered. The end's report holds `a` launched and the priority done |

### previews, modified

| Scenario | Proven by |
|---|---|
| A launch ahead of a preview | `previews.rs` `a_launch_ahead_of_a_preview`, unchanged, passing |
| A preview past due waits on no launch held for its turn | `previews.rs` `a_preview_past_due_waits_on_no_launch_held_for_its_turn`: Flume is asked to launch `a` alone, `b` is held with `TURN`, and `sluice preview` runs once, with `home/30` done in the same pass |

## The code against the design

`Work::turn_taken` holds a launch once the pass is due and `launches_run` is above zero. `made` counts every `flume launch` it runs, whatever Flume answers, and a launch found by its mark counts none (Decision 1). `hold_for_turn` answers `held` with `TURN`, said where the answer changed. In `offer`, room is counted at the launch's turn, and between the rounds a launch that waited on its clone's keep is held the same way (Decision 2).

`Work::may_run` saves the share and, once due, sends it before every command:

- `flume launch`, inline and between the rounds;
- Flume's status of a place for an act, and Flume's command on the place;
- `sluice preview` and `sluice release`.

After the send it refuses the want where the want or its launch is in `gone`, and `want` skips an act whose launch is in `gone`. `answer_wants` sends once the last want is answered (Decisions 3 and 4). A save that fails, before a send or after its answer, sets `halted`, and the pass carries out nothing more (core.25). A failed send sets `sends_failed`, which leaves the pass's answers to the send at its end (core.22).

One observation, which breaks no requirement: `offer` calls `may_run` before it learns whether the keeper holds the clone. So once due, an offer that then waits on its keep can send before any command runs. That is at most one send for each such offer, and only where an answer changed since the last send.
