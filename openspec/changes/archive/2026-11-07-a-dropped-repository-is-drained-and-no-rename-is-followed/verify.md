# Verify: a-dropped-repository-is-drained-and-no-rename-is-followed

The work on `change/a-dropped-repository-is-drained-and-no-rename-is-followed`, read at 48540d9, holds.

- Every task is ticked.
- The whole checks pass: `sluice exec main` ran all 21 `#check-fast` tasks at 48540d9, and 665 tests passed, with none failed or ignored.
- The delta has 20 requirements and 33 scenarios, beside 2 requirements removed, and a test asserts the outcome of every scenario.
- The code stage archived the change at 48540d9, before this stage ran. That commit splits four of the delta's requirements that ran past openspec's 500-character limit into requirements of their own, each sentence kept, and merges the deltas into `openspec/specs/`. This report is added to the archived change's directory, `openspec/changes/archive/2026-11-07-a-dropped-repository-is-drained-and-no-rename-is-followed/`. `openspec/changes/` holds no other open change.

The branch was cut from `bolt/the-runner-keeps-the-hosts-watch`, and its work starts at 15c7fbe, the change's proposal.

## Tasks

All 22 tasks of `tasks.md`, groups 1 to 7, are ticked `[x]`, and none is open. Each names the test, or the moon task, that makes it true. Those tests all ran in the gate (below).

What the tasks build is in the tree:

- **The interface.** `Launched` has `repository: Option<String>`, `#[serde(default, skip_serializing_if = "Option::is_none")]`, and `Hello` has `draining: Vec<String>`, `#[serde(default)]`. Every construction in every crate names them. `INTERFACE_VERSION` stays 1, and no file under `crates/boomdrive-hosted/protocol/` changed (core.42).
- **What has ended.** `facts::ended` reads a phase that is exactly `{"tidied": …}` or `{"gone": …}` as ended. `report::holding` gives each launch, with its place, whose recorded repository parses to the address, answered `launched` at a place, whose carried status is not "not listed" and does not read the place tidied or gone. A launch with no status carried holds its repository.
- **The control pass.** `Keeping::Dropped(container)` and `Clones::dropped()` are in `carry.rs`, and the fakes follow. `Work::offer` takes the order of design Decision 4:
  1. An offer the report answers gains its repository where it has none.
  2. An offer under a lapsed lease, or with no listing, waits.
  3. An offer in a repository neither kept nor drained is refused before any mark is read.
  4. An offer whose mark the listing holds is answered `launched`.
  5. An offer in a drained repository is refused.
  `Work::answer` records the repository at the first answer, and never changes it after. `branch` and `inbox` read `Dropped` as `NotNamed`. `carry_statuses` and `carry_entries` follow `Ready` and `Dropped` clones, and skip the rest. A status is always asked with `--repository`, and a landing is told to the keeper only where `Ready`.
- **The hello.** `hello::draining` gives each dropped address that `report::holding` finds held in the report read before the pull. The pass sets it on the hello.
- **The keeper.** `pass::pass` reads the report file once, lazily, before it lets go of a dropped clone. Where the file cannot be read, it names each dropped clone `not let go: <why>` and fails the pass. `let_go` keeps a clone that a launch holds and names `held by the place <place> of <id>, which has not ended`.
- **crew's implementation.** `pull_partition` resolves a launch naming this host against the hello's repositories, then against its drained ones, else takes its `Repo` as written. A launch naming no host that no runner answered resolves against the hello's repositories alone. One the runner answered resolves against the repositories, then the drained ones, and is given nothing where neither has it. A shared name gives nothing and is said in both cases.
- **The hosted implementation.** `Plane::hello` builds `repositories` from the hello's `repositories` alone. `taken_back` gives `repository: None`.
- **The README.** It says a dropped repository is drained, that its place is followed through its clone, and that no rename is followed.

The branch changes only files the proposal's Touches names, beside the change itself. Two files listed there, `crates/boomdrive-crew/tests/read.rs` and `crates/boomdrive-hosted/tests/connection.rs`, needed no change.

## Checks

The gate ran once, `devenv shell -- sluice exec main`, on the branch at 48540d9 with a clean worktree. It ran under sluice's sandbox with `network = "localhost"`, and was not run again. It exited 0 with `passed: 21 tasks`:

- **The crates.** `format`, `lint`, `typecheck` and `test` passed for `boomdrive`, `boomdrive-control`, `boomdrive-crew` and `boomdrive-hosted`.
- **The flake.** `nix:typecheck` and `nix:test` passed.
- **The root.** `root:format`, `root:specs` and `root:changes` passed. `root:specs` passed all 16 main specs under `--strict`, the 6 this change merges among them.

Each test cargo ran passed: 665 in all, none failed or ignored. Among them are the 20 tests the tasks name, each reported `ok`.

sluice did not record the pass under 48540d9. This report was written into the worktree while the run went on, and sluice saw it as beyond the head. The run itself was not affected.

## Requirements and scenarios

Each scenario's test. "Unchanged" marks a scenario of a modified requirement whose test stood before this change and still passes.

### Clones

| requirement | scenario | test |
|---|---|---|
| A dropped repository is removed once it holds nothing the forge lacks | Dropped with nothing of its own | `tests/dropped.rs` `dropped_with_nothing_of_its_own` (unchanged) |
| | Emptied later | `tests/dropped.rs` `emptied_later` (unchanged) |
| | The view's folder is not the view any more | `tests/dropped.rs` `the_views_folder_is_not_the_view` (unchanged) |
| A dropped repository is drained while a place the runner launched there has not ended | Dropped while a place is live | `tests/dropped.rs` `dropped_while_a_place_is_live`: over two passes, the view's HEAD and the clone's `origin/main` stay as they were though origin moved, and the one line names the place and launch |
| A place has ended once the status carried of it says Flume holds it no more | The place tidied | `tests/dropped.rs` `the_place_tidied`: held, then removed once the record `<place>@<at>` reads tidied and landed |
| | The place no longer listed | `tests/dropped.rs` `the_place_no_longer_listed` |
| A report file that cannot be read lets no dropped repository go | A report that cannot be read | `tests/dropped.rs` `a_report_that_cannot_be_read`: the pass fails, one line names the file, and the view and the container stand |

### Control plane interface

| requirement | scenario | test |
|---|---|---|
| The runner's report is reported whole | A report sent twice | `boomdrive-control` `tests/fake.rs` `a_report_sent_twice` (unchanged) |
| | A report from before | `tests/json.rs` `a_report_from_before_reads`: both read, the report serialises back byte for byte, and both round-trip; `a_report` and `a_hello` round-trip the new fields |

### crew's control plane

| requirement | scenario | test (`boomdrive-crew` `tests/claims.rs`) |
|---|---|---|
| A launch naming no host is given only where this runner keeps its repository | A kit named by its name alone | `a_kit_named_by_its_name_alone` (unchanged) |
| | A repository not kept | `a_repository_not_kept` (unchanged) |
| A repository draining gives no launch no runner answered | A repository draining | `an_unanswered_launch_in_a_repository_draining`: with room on the host, nothing is given, written or said |
| A launch the runner answered is given while the runner keeps or drains its repository | A launch answered in a repository draining | `a_launch_answered_in_a_repository_draining`, answered both by the branch and by the report: the offer names `example/kit`, its act is given, the branch's tip does not move, and nothing is said |
| No old name of a repository is matched to a new one | A launch answered before its repository was renamed | `a_launch_answered_before_its_repository_was_renamed`, by the branch and by the report |
| A launch naming this host takes its repository from those kept, then those drained | A launch naming this host in a repository draining | `a_launch_naming_this_host_in_a_repository_draining` |

The removed requirement, "A launch the runner answered is given whatever the hello names", is gone from `openspec/specs/crew-control-plane/spec.md`. Its two tests are gone with it.

### Hosted control plane

| requirement | scenario | test (`boomdrive-hosted` `tests/plane.rs`) |
|---|---|---|
| A partition's connection carries only its own repositories and work | Each hello its own | `each_hello_its_own` (unchanged) |
| | Another partition's repository | `another_partitions_repository` (unchanged) |
| | A repository drained goes up in no hello | `a_repository_draining_goes_up_in_no_hello`: `home`'s hello has `repositories: []` and no `draining` key; `work`'s names `example/service`; `home`'s report's `l-1` has no `repository` key |

### Tool records

| requirement | scenario | test (`tests/entries.rs`) |
|---|---|---|
| Each submission Flume names is carried from sluice's own answer | An entry carried | `an_entry_carried` (unchanged) |
| | No submission | `no_submission` (unchanged) |
| | An entry carried from a repository drained | `an_entry_carried_from_a_repository_drained`: the hello drains `example/kit`, sluice is run once as `status 412 --json` in its container, and the entry is carried |

### Work into Flume

| requirement | scenario | test |
|---|---|---|
| An offer is taken only for a repository the runner keeps | A repository not kept | `tests/offers.rs` `a_repository_not_kept` (unchanged) |
| | A clone not yet made | `tests/offers.rs` `a_clone_not_yet_made` (unchanged) |
| | A mark in a repository not kept | `tests/offers.rs` `a_mark_in_a_repository_not_kept`: in a repository with no clone, and in one dropped with nothing launched there, each with a place under its mark, each refused with the words, and no `flume` command but the listing run |
| Flume's status of each place is carried as Flume gave it | The status byte for byte | `tests/statuses.rs` `the_status_byte_for_byte` (unchanged) |
| | Only the host changed | `tests/statuses.rs` `only_the_host_changed` (unchanged) |
| A landing is told to the keeper once | A landing told once | `tests/statuses.rs` `a_landing_told_once` (unchanged) |
| | A landing in a repository not kept | `tests/statuses.rs` `a_place_in_a_repository_drained`: the landing is carried and no keeper is told |
| The hello names the repositories drained apart from those kept | A repository draining in the hello | `tests/offers.rs` `a_repository_draining_in_the_hello` |
| Nothing more is launched in a repository drained | A repository drained | `tests/offers.rs` `a_repository_drained`: `b` is refused and no `flume launch` is run. The test also shows that `c`, under its mark, is answered `launched` at its place and records `example/kit`, and that a branch command and an inbox there are answered as not kept, with no sluice run |
| A place whose repository has no clone is asked of Flume no more | A place in a repository with no clone | `tests/statuses.rs` `a_place_in_a_repository_with_no_clone`: no `StatusOf` call, the carried status is equal to before, and no landing is told |
| A place in a repository drained is followed to its end through its clone | A place in a repository drained | `tests/statuses.rs` `a_place_in_a_repository_drained`: `StatusOf(Some(container), P)` at the landing and at the tidy, the tidied record `P@1793559694` carried, listed, and the next hello naming nothing drained |
| Each launch's repository is kept with its answer | A launch's repository kept with its answer | `tests/offers.rs` `a_launchs_repository_kept_with_its_answer`: `a` records `agentplot/flume` in the file; `b` of before gains `Agentplot/Flume` as its offer wrote it, and is not launched again; a later offer naming another repository does not change `a`'s |

The removed requirement, "A place is followed to its end whether or not its repository is kept", is gone from `openspec/specs/work-into-flume/spec.md`. Its two tests are gone with it.

Unit tests the tasks name: `facts::tests::a_place_that_has_ended`, `report::tests::the_places_that_hold_a_repository` and `carry::tests::a_clone_the_configuration_drops_is_dropped`.

## Rules

No rule or record changes, as the proposal says. Record 0021 and core.5's "Not ruled out" already allow a dropped repository's clone and view to be kept, with nothing launched there, while a place the runner launched there is live. The work keeps the rules the design relies on:

- **core.5.** A drained clone is neither fetched nor moved.
- **core.9.** Work is taken only for a repository the runner keeps.
- **core.10.** A launch found by its mark is not made twice.
- **core.36.** The report gains a field, read as empty where it is missing.
- **core.42.** Version 1 is unchanged.
- **core.51.** An act's status check still names no repository: `act_on`'s `status_of(None, …)`.

## Notes that do not hold the change back

- **Two comments still describe the behavior this change removes.**
  - The doc comment on `Flume::status_of`, `crates/boomdrive/src/flume.rs:91-92`, says a place is asked "where the runner keeps its clone, else by its location alone". The `None` form now serves only `act_on`'s check (core.51).
  - `crates/boomdrive/tests/flume.rs:323` still reads "A place whose repository the runner keeps no more, by its location alone (record 0021)".

  The code is right. A later change can bring the words in line.
- **Following a dropped clone.** `carry_statuses` and `carry_entries` follow any dropped clone that stands, drained or held only by work the forge lacks, as design Decision 5 chooses. The specs require the follow where the repository is named or drained, and forbid none.
