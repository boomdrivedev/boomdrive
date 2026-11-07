# Verify: a-bolt-is-a-launch

Verified on 2026-11-07, on `change/a-bolt-is-a-launch` at 698d525.

**Verdict: the work falls short. The whole checks fail on one lint error.** Every requirement and scenario of the specs has a test that passes, and every task is ticked. But task 2.1's own check, `boomdrive:lint`, fails at the head of the branch, so the change goes back to its code stage.

The code stage archived the change itself, as task 4.3 asks (698d525). So this report sits in the archived directory, `openspec/changes/archive/2026-11-07-a-bolt-is-a-launch/`, because `openspec/changes/a-bolt-is-a-launch/` no longer exists. This stage archives nothing more.

## The whole checks

`devenv shell -- sluice exec main`, run once on 698d525, exited 1: **21 tasks completed, 1 failed**, `boomdrive:lint`.

```
error: the borrowed expression implements the required traits
   --> crates/boomdrive/tests/bolts.rs:225:46
    |
225 |     let acted = bench.report().acts[&Id::new(&format!("{BOLT}/send"))].clone();
    |                                              ^^^^^^^^^^^^^^^^^^^^^^^ help: change this to: `format!("{BOLT}/send")`
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.98.0/index.html#needless_borrows_for_generic_args
    = note: `-D clippy::needless-borrows-for-generic-args` implied by `-D warnings`
error: could not compile `boomdrive` (test "bolts") due to 1 previous error
```

These passed: `boomdrive:format`, `boomdrive:typecheck` and `boomdrive:test`; `boomdrive-control`, `boomdrive-crew` and `boomdrive-hosted`, each with its format, lint, typecheck and test; `nix:typecheck` and `nix:test`; `root:format`, `root:adrs`, `root:specs` and `root:changes`; and the layer's other tasks.

The fix is the one clippy names: `Id::new(format!("{BOLT}/send"))` in `sending`, `crates/boomdrive/tests/bolts.rs:225`.

## Tasks

All 12 tasks are ticked `[x]`. Each test a task names ran in the gate's `boomdrive:test` or `boomdrive-control:test` and passed:

| task | the check it names | outcome |
|---|---|---|
| 1.1 | `commands`: `a_branch_command`, `a_clone_not_yet_made`, `a_boom_command` | pass |
| 1.2 | `commands`: `a_command_given_twice`, `a_branch_command_an_earlier_release_carried_out` | pass |
| 1.3 | `leases a_command_whose_lease_lapsed` | pass |
| 1.4 | `statuses a_pass_that_answers_a_command_reads_the_listing_once` | pass |
| 2.1 | `sluice every_fixture_is_read`, and `moon run boomdrive:lint` | the test passes; **the lint fails**, on `tests/bolts.rs`, which task 3.1 added after 2.1 |
| 2.2 | `rules no_crate_asks_sluice_for_a_branch` | pass |
| 3.1 | `bolts`: `a_bolt_and_its_unit`, `a_bolt_counts_against_its_streams_room_within_its_pass` | pass |
| 3.2 | `bolts`: `a_bolts_send`, `a_send_flume_refuses` | pass |
| 4.1 | `boomdrive-control --test json` | pass |
| 4.2 | `readme`, `the_readme_names_no_branch_command` among it | pass |
| 4.3 | `root:specs` after the archive | pass |

Task 2.1 is ticked, but its check fails as the task names it, so it is not true at the head of the branch.

## Specs

Each scenario has a test whose assertions match its WHEN and THEN.

**commands** (new; `openspec/specs/commands/spec.md` matches the delta):

- *A command is refused, naming its tool*: `Work::command`, in `crates/boomdrive/src/carry.rs`, refuses every command not already in the report under a live lease. The words are `this binary does not carry <tool>'s commands`, with `sluice` for `CommandSort::Branch`. It reads neither repository nor clone, runs nothing, and sets no text and no origin commit. Scenarios: *A branch command* (`a_branch_command`), *A clone not yet made* (`a_clone_not_yet_made`, which also covers a repository the configuration does not keep), and *A boom command* (`a_boom_command`).
- *A command is answered once*: the report's answer stands, under the lease given now, and its time stands. Scenarios: *A command given twice* (`a_command_given_twice`, across a restarted control plane and then a lost report file), and *A branch command an earlier release carried out* (`a_branch_command_an_earlier_release_carried_out`, which keeps `done`, sluice's text and origin commit `C`, and runs nothing).

**work-into-flume**:

- *A bolt is a launch*: no code reads a kind. *A bolt and its unit* (`a_bolt_and_its_unit`) checks the exact `flume --json launch` argument vectors and that no `sluice` call is made. *A bolt counts against its stream's room within its pass* (`a_bolt_counts_against_its_streams_room_within_its_pass`) checks that the unit is left unanswered, then launched at the next pass.
- *A bolt's send is a retry of its submission*: *A bolt's send* (`a_bolts_send`, `flume --json retry P --submission`, answered `done`); *A send Flume refuses* (`a_send_flume_refuses`, answered `refused` with Flume's reason as given, and no `sluice` call).
- *Work under a lapsed lease waits for a live one*, modified: *A command whose lease lapsed* (`a_command_whose_lease_lapsed`). With a lapsed lease the command gets no answer and the pass writes no line. Under a live lease it is refused, naming sluice, and no `sluice` call is made.
- *The statuses are carried against Flume's listing…*, modified: `Work::ran` is no longer set by a command. *A pass that answers a command reads the listing once* (`a_pass_that_answers_a_command_reads_the_listing_once`).

**branch-commands**: retired. `openspec/specs/branch-commands/` is gone, and each removed requirement names where its content went.

## Design, rules and scope

- Decision 4: `Sluice::branch`, `Cli::branch`, `Printed`, `clone::origin_commit`, sluice 0.61.1's three branch fixtures and the fixtures README's section on them are all gone. The stub sluice's `then` hook went with them. No crate's `src` holds `"branch", "create"` or `"branch", "delete"`.
- Decision 5: `CommandSort::Branch` and `Commanded::origin_commit` keep their shapes, and their documentation says what the task asks (core.36, core.42).
- Decision 6: the listing is read again only after a launch or an act of Flume's (record 0027).
- Decision 7: core.52 and its details are in `docs/architecture/core.md`. Record 0002 has its amendment of 2026-11-07, proposed, which retires settled point 3.
- Decision 8: the README's control pass says a bolt is a launch, its send a retry naming its submission, and that every command is refused, naming its tool. It names no `sluice branch create`.
- Scope: every path changed since 3f428cf is in the proposal's Touches list or under the change's own OpenSpec paths.
