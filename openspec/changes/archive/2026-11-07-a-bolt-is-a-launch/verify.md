# Verify: a-bolt-is-a-launch

Verified on 2026-11-07, on `change/a-bolt-is-a-launch` at e129fa5.

**Verdict: the work holds and the whole checks pass.** Every requirement and scenario of the specs has a test whose assertions match it, and that test passes. Every task is ticked and its check passes as the task names it. The change is archived.

The code stage archived the change itself, as task 4.3 asks (e129fa5). So this report sits in the archived directory, `openspec/changes/archive/2026-11-07-a-bolt-is-a-launch/`, because `openspec/changes/a-bolt-is-a-launch/` no longer exists. The archive already holds the specs as they reconcile: `commands` made, `branch-commands` retired and `work-into-flume` merged. This report's commit completes the archive.

## The whole checks

`devenv shell -- sluice exec main`, run once on e129fa5, exited 0: **22 tasks completed, none failed**. sluice recorded the 22 results under their keys for e129fa5.

| project | tasks, each passed |
|---|---|
| `boomdrive` | format, lint, typecheck, test |
| `boomdrive-control` | format, lint, typecheck, test |
| `boomdrive-crew` | format, lint, typecheck, test |
| `boomdrive-hosted` | format, lint, typecheck, test |
| `nix` | typecheck, test |
| `root` | format, adrs, specs, changes |

moon warned once that it could not archive `boomdrive:test`'s outputs into its cache, and the OTLP metrics exporter failed to shut down at exit. Neither is a check, and neither changed the outcome.

## Tasks

All 12 tasks are ticked `[x]`. Each test a task names ran in the gate's `boomdrive:test` or `boomdrive-control:test` and passed. Each moon task a task names passed in the gate.

| task | the check it names | outcome |
|---|---|---|
| 1.1 | `commands`: `a_branch_command`, `a_clone_not_yet_made`, `a_boom_command` | pass |
| 1.2 | `commands`: `a_command_given_twice`, `a_branch_command_an_earlier_release_carried_out` | pass |
| 1.3 | `leases a_command_whose_lease_lapsed` | pass |
| 1.4 | `statuses a_pass_that_answers_a_command_reads_the_listing_once` | pass |
| 2.1 | `sluice every_fixture_is_read`, and `moon run boomdrive:lint` | pass |
| 2.2 | `rules no_crate_asks_sluice_for_a_branch` | pass |
| 2.3 | `moon run boomdrive:lint`, with `tests/bolts.rs:225` making the send's id as `Id::new(format!("{BOLT}/send"))` | pass |
| 3.1 | `bolts`: `a_bolt_and_its_unit`, `a_bolt_counts_against_its_streams_room_within_its_pass` | pass |
| 3.2 | `bolts`: `a_bolts_send`, `a_send_flume_refuses` | pass |
| 4.1 | `boomdrive-control --test json` | pass |
| 4.2 | `readme`, `the_readme_names_no_branch_command` among it | pass |
| 4.3 | `root:specs` after the archive | pass |

## Specs

**commands** (new; `openspec/specs/commands/spec.md` matches the delta):

- *A command is refused, naming its tool*: `Work::command`, in `crates/boomdrive/src/carry.rs`, refuses every command the report does not already hold, under a live lease. Its words are `this binary does not carry <tool>'s commands`, with `sluice` for `CommandSort::Branch`. It reads neither repository nor clone, runs nothing, and `commanded` sets no text and no origin commit. Scenarios: *A branch command* (`a_branch_command`), *A clone not yet made* (`a_clone_not_yet_made`) and *A boom command* (`a_boom_command`).
- *A command is answered once*: an answer the report holds stands, under the lease given now, and its time stands. Scenarios: *A command given twice* (`a_command_given_twice`) and *A branch command an earlier release carried out* (`a_branch_command_an_earlier_release_carried_out`, which keeps `done`, sluice's text and origin commit `C`, and runs nothing).

**work-into-flume**:

- *A bolt is a launch*: no code reads a kind. *A bolt and its unit* (`a_bolt_and_its_unit`) checks the two exact `flume --json launch` argument vectors, both answered `launched`, and that no `sluice` call is made. *A bolt counts against its stream's room within its pass* (`a_bolt_counts_against_its_streams_room_within_its_pass`) checks that the bolt is launched and the unit left unanswered, then launched at the next pass on `bolt/x` in stream `madswan`.
- *A bolt's send is a retry of its submission*: *A bolt's send* (`a_bolts_send`, exactly `flume --json retry P --submission`, answered `done`). *A send Flume refuses* (`a_send_flume_refuses`, answered `refused` with Flume's reason as given, and no `sluice` call).
- *Work under a lapsed lease waits for a live one*, modified: *A command whose lease lapsed* (`a_command_whose_lease_lapsed`).
- *The statuses are carried against Flume's listing…*, modified: a command no longer sets `Work::ran`. *A pass that answers a command reads the listing once* (`a_pass_that_answers_a_command_reads_the_listing_once`).

**branch-commands**: retired. `openspec/specs/branch-commands/` is gone, and each removed requirement names where its content went.

## Design, rules and scope

- Decision 4: `Sluice::branch`, `Cli::branch`, `Printed`, `clone::origin_commit`, sluice 0.61.1's three branch fixtures and the fixtures README's section on them are all gone. No crate's `src`, and not the README, holds `sluice branch`, `"branch", "create"` or `"branch", "delete"`.
- Decision 5: `CommandSort::Branch` and `Commanded::origin_commit` keep their shapes. Their documentation says what task 4.1 asks (core.36, core.42).
- Decision 6: the listing is read again only after a launch or an act of Flume's (record 0027).
- Decision 7: core.52 and its details are in `docs/architecture/core.md`. Record 0002 has its amendment of 2026-11-07, *a bolt is a launch*, proposed, which retires settled point 3.
- Decision 8: the README's control pass says a bolt is a launch and its send a retry naming its submission. It also says every command is refused, naming its tool, and it names no `sluice branch create`.
- Scope: every path changed since the base, 3f428cf (`origin/bolt/the-runner-keeps-the-hosts-watch`), is in the proposal's Touches list or under the change's own OpenSpec paths.
