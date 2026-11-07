# Verify: a-dropped-repository-is-drained-and-no-rename-is-followed

The work on `change/a-dropped-repository-is-drained-and-no-rename-is-followed`, read at 059cef4, holds.

- **Tasks.** All 23 tasks are ticked.
- **Checks.** The whole checks pass. `sluice exec main` ran all 25 `#check-fast` tasks at 059cef4, and sluice recorded their results under that commit. 675 tests passed, none failed and none ignored.
- **Scenarios.** The delta has 20 requirements and 33 scenarios, and it removes 2 requirements. A test asserts the outcome of every scenario.
- **The base.** The branch holds `bolt/the-runner-keeps-the-hosts-watch` at 478303b, after #20 (a-bolt-is-a-launch) and #21 (each-pass-ends-with-a-settle). It took that commit by a merge, 3f43146, with no rebase. Measured from 478303b, the branch changes only the files the proposal's Touches names, plus the change itself and the six main specs it merges into.
- **The archive.** The code stage archived the change at 059cef4, before this stage ran, and merged the deltas into `openspec/specs/` over 478303b. This report goes into the archived change's directory, `openspec/changes/archive/2026-11-07-a-dropped-repository-is-drained-and-no-rename-is-followed/`. No other change is open in `openspec/changes/`.

## Tasks

All 23 tasks in `tasks.md`, groups 1 to 7, are ticked `[x]`, and none is open. Each one names the test or moon task that makes it true, and all of those ran in the gate (see Checks).

What the tasks build is in the tree:

- **The interface.** `Launched` has `repository: Option<String>`, with `#[serde(default, skip_serializing_if = "Option::is_none")]`. `Hello` has `draining: Vec<String>`, with `#[serde(default)]`. Every construction of either, in every crate, names the new field. `INTERFACE_VERSION` stays 1, and no file under `crates/boomdrive-hosted/protocol/` changed (core.42).
- **What has ended.** `facts::ended` reads a place as ended where its phase is exactly `{"tidied": …}` or `{"gone": …}`.
- **What holds a repository.** `report::holding` gives each launch, with its place, that meets all of these:
  - its recorded repository parses to the address;
  - it was answered `launched` at a place;
  - the status carried of that place is not "not listed" and does not read it tidied or gone.

  A launch with no status carried still holds its repository.
- **The control pass.** `Keeping::Dropped(container)` and `Clones::dropped()` are in `carry.rs`, and the fakes follow them. `Work::offer` handles an offer in the order design Decision 4 gives:
  1. An offer the report already answers gains its repository where it has none.
  2. An offer under a lapsed lease, or with no listing, waits.
  3. An offer in a repository neither kept nor drained is refused before any mark is read.
  4. An offer whose mark is in the listing is answered `launched`.
  5. An offer in a drained repository is refused.

  The rest of the pass:
  - `Work::answer` records the repository at the first answer and never changes it afterwards.
  - `inbox` treats `Dropped` as `NotNamed`.
  - `Work::command`, which came in with #20, refuses every command and names its tool, which is sluice for a branch command. It reads no repository and runs nothing (core.52), so no command ever reads `Keeping::Dropped`.
  - `carry_statuses` and `carry_entries` follow `Ready` and `Dropped` clones and skip the rest.
  - A status is always asked for with `--repository`.
  - A landing is told to the keeper only where the clone is `Ready`.
- **The hello.** `hello::draining` gives each dropped address that `report::holding` finds held in the report read before the pull, and the pass sets it on the hello.
- **The keeper.** `pass::pass` reads the report file once, and only when it is about to let go of a dropped clone.
  - Where the file cannot be read, it names each dropped clone `not let go: <why>` and fails the pass.
  - `let_go` keeps a clone that a launch holds, and names it `held by the place <place> of <id>, which has not ended`.
- **crew's implementation.** `pull_partition` resolves a launch's `Repo` as follows:
  - A launch naming this host resolves against the hello's repositories, then its drained ones, and otherwise takes its `Repo` as written.
  - A launch naming no host, which no runner answered, resolves against the hello's repositories only.
  - A launch naming no host, which the runner answered, resolves against the repositories, then the drained ones, and is given nothing where neither has it.
  - Where two addresses share a name, nothing is given and the clash is logged, for both kinds of launch.
- **The hosted implementation.** `Plane::hello` builds `repositories` from the hello's `repositories` alone, and `taken_back` gives `repository: None`.
- **The README.** It says three things in the clones paragraph and in the control pass's list:
  - a dropped repository is drained;
  - its place is followed through its clone;
  - no rename is followed.

  Beside that, it keeps #20's bolt paragraph and the listing read again only where the pass ran a launch or an act.

Two files listed in Touches needed no change: `crates/boomdrive-crew/tests/read.rs` and `crates/boomdrive-hosted/tests/connection.rs`.

## Checks

The gate ran once, as `devenv shell -- sluice exec main`, on the branch at 059cef4 with a clean worktree. It was not run again. It exited 0 with `passed: 25 tasks`, and sluice printed `recorded 25 results under their keys for 059cef4`.

- **The crates.** `format`, `lint`, `typecheck` and `test` passed for `boomdrive`, `boomdrive-control`, `boomdrive-crew` and `boomdrive-hosted`.
- **The flake.** `nix:format`, `nix:lint`, `nix:typecheck` and `nix:test` passed.
- **The root.** `root:format`, `root:adrs`, `root:specs`, `root:changes` and `root:releases` passed. `root:specs` passed all 16 main specs under `--strict` (`Totals: 16 passed, 0 failed`), including the 6 this change merges into.

Every test cargo ran passed: 675 in all, with none failed or ignored. They include the 20 tests the tasks name, each reported `ok`. Task 3.9's check, `boomdrive:test`, passed within the gate.

moon logged a warning for some tasks: `Failed to package task outputs into archive` under `.moon/cache/outputs/`. It is moon's output cache failing to write a file, and no task's result depended on it.

## Requirements and scenarios

Each scenario and the test that asserts it. "Unchanged" marks a scenario of a modified requirement whose test existed before this change and still passes.

### Clones

| requirement | scenario | test |
|---|---|---|
| A dropped repository is removed once it holds nothing the forge lacks | Dropped with nothing of its own | `tests/dropped.rs` `dropped_with_nothing_of_its_own` (unchanged) |
| | Emptied later | `tests/dropped.rs` `emptied_later` (unchanged) |
| | The view's folder is not the view any more | `tests/dropped.rs` `the_views_folder_is_not_the_view_any_more` (unchanged) |
| A dropped repository is drained while a place the runner launched there has not ended | Dropped while a place is live | `tests/dropped.rs` `dropped_while_a_place_is_live`: over two passes, the view's HEAD and the clone's `origin/main` stay put though origin moved, and one line names the place and the launch |
| A place has ended once the status carried of it says Flume holds it no more | The place tidied | `tests/dropped.rs` `the_place_tidied`: held, then removed once the record `<place>@<at>` reads tidied and landed |
| | The place no longer listed | `tests/dropped.rs` `the_place_no_longer_listed` |
| A report file that cannot be read lets no dropped repository go | A report that cannot be read | `tests/dropped.rs` `a_report_that_cannot_be_read`: the pass fails, one line names the file, and the view and the container stay |

### Control plane interface

| requirement | scenario | test |
|---|---|---|
| The runner's report is reported whole | A report sent twice | `boomdrive-control` `tests/fake.rs` `a_report_sent_twice` (unchanged) |
| | A report from before | `tests/json.rs` `a_report_from_before_reads`: both files read, the report serializes back byte for byte, and both round-trip. `a_report` and `a_hello` round-trip the new fields |

### crew's control plane

| requirement | scenario | test (`boomdrive-crew` `tests/claims.rs`) |
|---|---|---|
| A launch naming no host is given only where this runner keeps its repository | A kit named by its name alone | `a_kit_named_by_its_name_alone` (unchanged) |
| | A repository not kept | `a_repository_not_kept` (unchanged) |
| A repository draining gives no launch no runner answered | A repository draining | `an_unanswered_launch_in_a_repository_draining`: with room on the host, nothing is given, written or logged |
| A launch the runner answered is given while the runner keeps or drains its repository | A launch answered in a repository draining | `a_launch_answered_in_a_repository_draining`, with the launch answered once on the branch and once in the report. The offer names `example/kit`, its act is given, the branch's tip does not move, and nothing is logged |
| No old name of a repository is matched to a new one | A launch answered before its repository was renamed | `a_launch_answered_before_its_repository_was_renamed`, answered on the branch and in the report |
| A launch naming this host takes its repository from those kept, then those drained | A launch naming this host in a repository draining | `a_launch_naming_this_host_in_a_repository_draining` |

The removed requirement, "A launch the runner answered is given whatever the hello names", is gone from `openspec/specs/crew-control-plane/spec.md`, and its two tests are gone with it.

### Hosted control plane

| requirement | scenario | test (`boomdrive-hosted` `tests/plane.rs`) |
|---|---|---|
| A partition's connection carries only its own repositories and work | Each hello its own | `each_hello_its_own` (unchanged) |
| | Another partition's repository | `another_partitions_repository` (unchanged) |
| | A repository drained goes up in no hello | `a_repository_draining_goes_up_in_no_hello`: `home`'s hello has `repositories: []` and no `draining` key, `work`'s hello names `example/service`, and `l-1` in `home`'s report has no `repository` key |

### Tool records

| requirement | scenario | test (`tests/entries.rs`) |
|---|---|---|
| Each submission Flume names is carried from sluice's own answer | An entry carried | `an_entry_carried` (unchanged) |
| | No submission | `no_submission` (unchanged) |
| | An entry carried from a repository drained | `an_entry_carried_from_a_repository_drained`: the hello drains `example/kit`, sluice runs once as `status 412 --json` in its container, and the entry is carried |

### Work into Flume

| requirement | scenario | test |
|---|---|---|
| An offer is taken only for a repository the runner keeps | A repository not kept | `tests/offers.rs` `a_repository_not_kept` (unchanged) |
| | A clone not yet made | `tests/offers.rs` `a_clone_not_yet_made` (unchanged) |
| | A mark in a repository not kept | `tests/offers.rs` `a_mark_in_a_repository_not_kept`: two cases, a repository with no clone and one dropped with nothing launched there, each with a place under its mark. Each is refused with the words, and no `flume` command runs but the listing |
| Flume's status of each place is carried as Flume gave it | The status byte for byte | `tests/statuses.rs` `the_status_byte_for_byte` (unchanged) |
| | Only the host changed | `tests/statuses.rs` `only_the_host_changed` (unchanged) |
| A landing is told to the keeper once | A landing told once | `tests/statuses.rs` `a_landing_told_once` (unchanged) |
| | A landing in a repository not kept | `tests/statuses.rs` `a_place_in_a_repository_drained`: the landing is carried and no keeper is told |
| The hello names the repositories drained apart from those kept | A repository draining in the hello | `tests/offers.rs` `a_repository_draining_in_the_hello` |
| Nothing more is launched in a repository drained | A repository drained | `tests/offers.rs` `a_repository_drained`: `b` is refused and no `flume launch` runs. The test also covers three more cases (below the table) |
| A place whose repository has no clone is asked of Flume no more | A place in a repository with no clone | `tests/statuses.rs` `a_place_in_a_repository_with_no_clone`: no `StatusOf` call, the carried status is unchanged, and no landing is told |
| A place in a repository drained is followed to its end through its clone | A place in a repository drained | `tests/statuses.rs` `a_place_in_a_repository_drained`: `StatusOf(Some(container), P)` runs at the landing and at the tidy, the tidied record `P@1793559694` is carried and listed, and the next hello names nothing drained |
| Each launch's repository is kept with its answer | A launch's repository kept with its answer | `tests/offers.rs` `a_launchs_repository_kept_with_its_answer`: `a` records `agentplot/flume` in the file. `b`, from an older report, gains `Agentplot/Flume` as its offer wrote it and is not launched again. A later offer naming another repository does not change `a`'s |

The three further cases `a_repository_drained` covers:

- `c`, under its mark, is answered `launched` at its place and records `example/kit`.
- An inbox in the drained repository is answered as one in a repository not kept, and no sluice runs.
- A branch command there is refused naming sluice, `this binary does not carry sluice's commands`, as every command is (core.52).

The removed requirement, "A place is followed to its end whether or not its repository is kept", is gone from `openspec/specs/work-into-flume/spec.md`, and its two tests are gone with it. The main spec also holds #20's requirements, including "A pass that answers a command reads the listing once", and this change's delta does not conflict with them: no requirement of the delta names a branch command.

The unit tests the tasks name are `facts::tests::a_place_that_has_ended`, `report::tests::the_places_that_hold_a_repository` and `carry::tests::a_clone_the_configuration_drops_is_dropped`.

## Rules

No rule or record changes, as the proposal says. Record 0021, and the "Not ruled out" of core.5, already allow a dropped repository's clone and view to be kept while a place the runner launched there is live, with nothing launched there. The work keeps the rules the design relies on:

- **core.5.** A drained clone is neither fetched nor moved.
- **core.9.** Work is taken only for a repository the runner keeps.
- **core.10.** A launch found by its mark is not made twice.
- **core.36.** The report gains a field, read as empty where it is missing.
- **core.42.** Version 1 of the protocol is unchanged.
- **core.51.** An act's status check still names no repository: `act_on` calls `status_of(None, …)`.
- **core.52.** A branch command in a drained repository is refused naming sluice, and nothing of the repository is read.

## Notes that do not hold the change back

- **Two comments still describe the behavior this change removes.** The code is right, and a later change can bring the words in line.
  - The doc comment on `Flume::status_of`, at `crates/boomdrive/src/flume.rs:93-94`, says a place is asked "where the runner keeps its clone, else by its location alone". The `None` form now serves only `act_on`'s check (core.51).
  - `crates/boomdrive/tests/flume.rs:335` still reads "A place whose repository the runner keeps no more, by its location".
- **Following a dropped clone.** `carry_statuses` and `carry_entries` follow any dropped clone that is still there, whether it is drained or held only by work the forge lacks, as design Decision 5 chooses. The specs require the follow where the repository is named or drained, and forbid none.
