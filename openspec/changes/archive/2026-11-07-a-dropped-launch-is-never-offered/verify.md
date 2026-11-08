# Verify

The verify stage's report on `change/a-dropped-launch-is-never-offered` at `2d121e5`.

**Verdict: the work holds. The whole checks pass, every task is ticked, and every requirement and scenario of the change's spec is met. The change stays archived.**

The code stage archived the change in `2d121e5`, as its task 4.2 asks, so this report sits in the archived change's directory and the change's archive is that commit with this report.

## The whole checks

Run once, as the repository's instructions name them: `devenv shell -- sluice exec main`, exit 0. sluice ran the `#check-fast` tasks that `2d121e5` affects against `main` (`3f428cf`), in its sandbox.

```
running the checks 2d121e5 affects against main (3f428cf)
Tasks: 22 completed
passed: 22 tasks
recorded 1 result under its key for 2d121e5
```

| project | tasks, each a pass |
|---|---|
| `root` | `format`, `adrs`, `specs`, `changes` |
| `boomdrive` | `format`, `lint`, `typecheck`, `test` |
| `boomdrive-control` | `format`, `lint`, `typecheck`, `test` |
| `boomdrive-crew` | `format`, `lint`, `typecheck`, `test` |
| `boomdrive-hosted` | `format`, `lint`, `typecheck`, `test` |
| `nix` | `typecheck`, `test` |

The other `#check-fast` tasks, `nix:format`, `nix:lint`, `root:toolchain`, `root:commits`, `root:releases` and `root:workflow`, are not affected by this change's paths, so sluice did not run them. treefmt ran over the tree as the shell started and changed no file. moon warned that it could not write its caches from the sandbox; that touched no task's outcome.

`boomdrive-crew:test` ran every test binary to the end, `tests/drops.rs`'s eleven tests among them, `tests/claims.rs`, and `tests/read.rs` with `a_retry_of_the_submission` and `only_this_hosts_requests`. `boomdrive-control:test` and `boomdrive-hosted:test` ran the shared case `wants_in_order`.

## The work against the change

### Tasks

Every task in `tasks.md` is ticked, 1.1 to 4.2. The gate above runs each task's named check or a wider one: the crate tests 1.1 to 3.3 name are in `boomdrive-crew:test`; 1.7 is `boomdrive-crew:lint`; 1.9 is the three crates' `test`; 4.1 is `root:format`; 4.2 is `root:specs`. Tasks 1.7, 1.8 and 1.9, which this report's earlier run sent back, hold: `tests/drops.rs` hands `main_holding` `std::slice::from_ref(&kit)`, `a_retry_of_the_submission` pulls with launch 12 answered `launched`, and `wants_in_order` acts on launch 3 with `ActSort::Priority(1)`, its module documentation saying its acts hold no drop.

### Requirements and scenarios

Each requirement of the delta is built as the design gives it, and each scenario is proven by the test design Decision 6 names, which asserts what its THEN says:

| Requirement (delta) | Where it is built | Its scenarios' tests (`tests/drops.rs` unless named) |
|---|---|---|
| Each request for this host is a want under the lease number 1 (MODIFIED) | `pull_partition`'s walk | `read.rs` `only_this_hosts_requests` |
| A launch naming no host is claimed before it is given (MODIFIED) | the dropped-launch skip before `kits` | `claims.rs` `claimed_alone`, `another_claim_seen`, `taken_elsewhere` |
| A claim stands until the runner answers its launch (MODIFIED) | `Crew::report`'s claim filter, `own_claims` | `claims.rs` `a_claim_waiting_for_its_streams_room`, `a_claim_found_again_after_a_stop`; `a_claimed_launch_dropped` |
| A launch the runner answered is given whatever the hello names (MODIFIED) | the skip, by `Pulling::launched` | `claims.rs` `a_launch_answered_before_its_repository_left_the_hello`, `a_launch_the_report_answers_is_given_whatever_the_hello_names` |
| A request whose work has left crew's plan is given only while the runner holds it | `Pulling::current`, as built | `work_gone_that_the_runner_does_not_hold` |
| A drop is given to the runner that holds its launch whatever the plan holds | `Pulling::current`'s drop clause | `a_drop_written_as_its_work_leaves_the_plan`, `a_kits_drop_goes_to_the_runner_holding_its_launch` |
| A launch is dropped as crew reads it | `Main::dropped`, read in `Store::main` from every `Act: drop` request's whole `Of` | `a_drop_naming_another_host` |
| A launch a drop names is offered only where the runner launched it | `Pulling::launched` and the skip in `pull_partition` | `a_held_launch_dropped`, `a_kits_launch_dropped_before_it_is_claimed`, `a_dropped_launch_the_runner_launched` |
| A dropped launch the runner holds unlaunched is let go | `write::let_go`'s first loop | `a_held_launch_dropped_is_let_go`, `a_drop_answered_from_the_report_its_work_gone`, `a_dropped_launch_the_runner_launched_stays_until_its_place_is_over` |

Every requirement of the archived delta reads word for word as it does in `openspec/specs/crew-control-plane/spec.md`, and `root:specs` passes. README's section on crew's table and record 0002's amendment of 2026-11-07 say what the proposal says they will.

### Scope

The change touches the files proposal.md's Touches lists, its own OpenSpec files and the main spec it archives into. Nothing else changes. No rule changes and none is departed from: core.1's details already allow an implementation to give what it reads from the control plane's own records, and the runner's code and rules stand as they were.
