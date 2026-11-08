# Verify

The verify stage's report on `change/a-dropped-launch-is-never-offered` at `ed2aa39`.

**Verdict: the work falls short. The whole checks fail, so the change goes back to its code stage.**

The code stage archived the change in `ed2aa39`, as its task 4.2 asks. That is why this report sits in the archived change's directory and not under `openspec/changes/a-dropped-launch-is-never-offered/`. A directory there holding only this file would be an open change with no proposal, and `root:changes` and `root:specs` would fail on it.

## The whole checks

Run once, as the repository's instructions name them: `devenv shell -- sluice exec main`, exit 1.

```
Tasks: 12 completed, 2 failed
failed: boomdrive-crew:lint, boomdrive-crew:test
recorded 14 results under their keys for ed2aa39
```

| task | outcome |
|---|---|
| `root:format`, `root:adrs`, `root:changes`, `root:specs` | pass |
| `boomdrive:format`, `boomdrive:lint`, `boomdrive:typecheck`, `boomdrive:test` | pass |
| `boomdrive-crew:format`, `boomdrive-crew:typecheck` | pass |
| `nix:typecheck`, `nix:test` | pass |
| **`boomdrive-crew:lint`** | **fail** |
| **`boomdrive-crew:test`** | **fail** |

### `boomdrive-crew:lint`

clippy `-D warnings` refuses the new test file:

```
error: unnecessary use of `clone` to create a slice from a reference
   --> crates/boomdrive-crew/tests/drops.rs:183:26
    |
183 |     main_holding(&state, &[kit.clone()], &["u"]);
    |                          ^^^^^^^^^^^^^^ help: try: `std::slice::from_ref(&kit)`
    |
    = note: `-D clippy::cloned-ref-to-slice-refs` implied by `-D warnings`
```

This is in `a_claimed_launch_dropped`, task 1.5's test.

### `boomdrive-crew:test`

An existing test breaks under the new behaviour: `tests/read.rs`'s `a_retry_of_the_submission`, which proves the crew-control-plane spec's scenario "A retry of the submission".

```
thread 'a_retry_of_the_submission' panicked at crates/boomdrive-crew/tests/read.rs:183:5:
assertion `left == right` failed
  left: [Act(madswan/14 on madswan/12, Retry { submission: true }), Act(madswan/15 on madswan/12, Drop), Act(madswan/16 on madswan/12, Priority(7))]
 right: [Act(madswan/13 on madswan/12, Retry { stage: "code", words: "Four." }), Act(madswan/14 …), Act(madswan/15 … Drop), Act(madswan/16 … Priority(7))]
```

The test's `requests.rec` holds launch 12 for `deneb`, and acts 13 to 16 on it. Act 15 drops 12. The test pulls with no report, so the runner has not launched 12. Under this change's "A launch a drop names is offered only where the runner launched it", launch 12 is no longer given. The test's `pulled.wants[1..]`, which skipped launch 12, now skips act 13 instead.

The spec itself still holds. Its scenario "A retry of the submission" names no drop, and the code gives what the new requirement asks. What is wrong is the test's fixture, which drops the launch its acts are on. One fix is to give the pull a report answering 12 `launched`. Another is to take the drop out of the fixture, or have it name another launch. Either fix touches `tests/read.rs`, which proposal.md's Touches does not list.

cargo stops at the first test binary that fails, so the binaries after `read` in `boomdrive-crew:test` did not run in this gate. Their outcome is unknown here. The binaries that ran before it passed (18, 16, 11 and 11 tests).

Once both are fixed, the whole checks need to run again before this change holds.

## The work against the change

### Tasks

Every task in `tasks.md` is ticked, 1.1 to 4.2. Each task's own named check is narrower than the gate. Task 1.6 runs `--test claims` alone and task 4.1 runs `root:format` alone, so neither caught the failures above. The task notes also say a task is true only "once the check it names passes", and no task names the crate's whole tests or clippy.

### Requirements and scenarios

On reading the code at `ed2aa39`, each requirement of the delta is built as the design gives it:

| Requirement (delta) | Where it is built | Its scenarios' tests (`tests/drops.rs` unless named) |
|---|---|---|
| Each request for this host is a want under the lease number 1 (MODIFIED) | `pull_partition`'s walk | `read.rs` `only_this_hosts_requests` |
| A launch naming no host is claimed before it is given (MODIFIED) | the dropped-launch skip before `kits` | `claims.rs` `claimed_alone`, `another_claim_seen`, `taken_elsewhere` |
| A claim stands until the runner answers its launch (MODIFIED) | `Crew::report`'s claim filter, `own_claims` | `claims.rs` `a_claim_waiting_for_its_streams_room`, `a_claim_found_again_after_a_stop`; `a_claimed_launch_dropped` |
| A launch the runner answered is given whatever the hello names (MODIFIED) | the skip, by `Pulling::launched` | `claims.rs` `a_launch_answered_before_its_repository_left_the_hello`, `a_launch_the_report_answers_is_given_whatever_the_hello_names` |
| A request whose work has left crew's plan is given only while the runner holds it | `Pulling::current`, as built | `work_gone_that_the_runner_does_not_hold` |
| A drop is given to the runner that holds its launch whatever the plan holds | `Pulling::current`'s drop clause | `a_drop_written_as_its_work_leaves_the_plan`, `a_kits_drop_goes_to_the_runner_holding_its_launch` |
| A launch is dropped as crew reads it | `Main::dropped`, read in `Store::main` | `a_drop_naming_another_host` |
| A launch a drop names is offered only where the runner launched it | `Pulling::launched` and the skip in `pull_partition` | `a_held_launch_dropped`, `a_kits_launch_dropped_before_it_is_claimed`, `a_dropped_launch_the_runner_launched` |
| A dropped launch the runner holds unlaunched is let go | `write::let_go`'s first loop | `a_held_launch_dropped_is_let_go`, `a_drop_answered_from_the_report_its_work_gone`, `a_dropped_launch_the_runner_launched_stays_until_its_place_is_over` |

Each test asserts what its scenario's THEN says, and each is named as design Decision 6's table names it. The archived delta matches `openspec/specs/crew-control-plane/spec.md`, and `root:specs` passes. README's section on crew's table and record 0002's amendment of 2026-11-07 say what the proposal says they will.

This change broke one scenario's test, "A retry of the submission" in `tests/read.rs`. Until that test is fixed, the gate does not show the spec's existing scenario holding.

### Scope

The change touches the files proposal.md's Touches lists, its own OpenSpec files and the main spec it archives into. Nothing else is changed. No rule changes, and none is departed from.

## What the code stage must do

1. Fix `crates/boomdrive-crew/tests/drops.rs:183` for clippy, for example with `std::slice::from_ref(&kit)`.
2. Fix the fixture of `crates/boomdrive-crew/tests/read.rs`'s `a_retry_of_the_submission`, so that its drop no longer withholds the launch its acts are on. Either have the runner's report answer 12 `launched`, or have the drop not name 12.
3. Run the whole checks, `sluice exec main`, until they pass.
