# Verify: the-runners-stop-ends-its-try-in-flight

The work on `change/the-runners-stop-ends-its-try-in-flight` was read at e29f653. The gate passes, the work meets every requirement and scenario of its change, and every task is ticked. The change stays archived, with this report beside it.

- The gate ran once through sluice against `main` at 678ccfc. All 18 of its tasks passed: 729 tests passed and none failed.
- The change's deltas add 6 requirements and 10 scenarios. A test that passed in the gate asserts the outcome of each scenario.
- All 9 tasks are ticked. The code stage archived the change at e29f653, as task 5.1 says to, so this report sits beside the archived change.
- `bolt/the-runners-stop-ends-its-try-in-flight` is at 57b1d1e, the branch's base. `main` has moved on to 678ccfc. What it gained, `devenv.nix` and one record, touches no path of this change.

## Checks

- **The gate.** `sluice exec main` at e29f653 ran the 18 `check-fast` tasks that e29f653 affects against `main` at 678ccfc. It exited 0, and sluice recorded 18 results under their keys for e29f653.
  - Passed:
    - `boomdrive-bounded:format`, `lint`, `typecheck` and `test`;
    - `boomdrive-crew:format`, `lint`, `typecheck` and `test`;
    - `boomdrive:format`, `lint`, `typecheck` and `test`;
    - `nix:typecheck` and `test`;
    - `root:format`, `adrs`, `specs` (23 of 23) and `changes`, which found no open change to validate.
  - moon warned three ways: it could not write its shared cache or its output archives outside the sandbox, and it had no internet connection for a dependency install. None of these touched a task's result.
- The gate was run once, as this stage's brief says, and nothing was fixed here.

## Tasks

All 9 tasks of `tasks.md`, groups 1 to 5, are ticked `[x]`, and none is open. What they build is in the tree, as the design names it:

- **The halt** (1.1, 1.2):
  - `boomdrive-bounded` holds `Halt`, a `Clone`, `Debug` and `Default` handle to one shared `AtomicBool`, with `halt` and `halted`. It also holds `Stopped`, either `AtLimit` or `Halted`.
  - `output(command, limit, halt)` reads the halt before it spawns. Where the halt is set, it returns `Halted` with no status and empty pipes. Otherwise it reads the halt at each poll beside the limit, and at whichever comes first ends the group by `end_group` with `GRACE`.
  - `Ended.status` is `Option<ExitStatus>`, and `Ended.stopped` is `Option<Stopped>`.
  - `halted(call, remote)` gives `<call> with <remote> was stopped as the runner stopped`.
  - The existing tests pass `&Halt::default()` and assert `Some(Stopped::AtLimit)`, and the four new tests the tasks name are present.
- **The runner's halt** (2.1 to 2.3):
  - `Git` carries a private `halt`, never set by `new` or `default`, and `halted_by` gives a git with another. `call_remote` runs `bounded` to the limit and the halt. A halted call fails with git's last line, then `; `, then `halted(…)`, and its standard output is not read.
  - `Runner::start` makes the halt, gives it to its git, and passes it to `Control::open` and on to `planes::open`. `Stopper::stop` sets the stop flag, then the halt, then sends `Event::Stop` and stops every bell.
  - `main` calls `stop_on_signals` for `run --once` as for `run`. `Runner::once` reads the stop after its keeping pass and after its parts' passes. Where it is stopped, it says `boomdrive: stopped` and gives `Outcome::Failed`, which `main` exits 1 on. It begins no part's pass and no settle.
  - `Scratch::silent` returns a `Silent` with `taken(within)` and `closed(within)`.
- **crew's listing** (3.1, 3.2):
  - `Store` gains `pub halt`, never set by `Store::new`. `Store::hosts` runs its `ls-remote` to `try_for` and `halt`, and a halted listing fails with `halted("git ls-remote", &self.url)` after git's last line.
  - `Crew` gains a private halt and `halted_by`, and `Crew::store` gives its store the crew's halt. `open` and `parts` take `halt: &Halt` and give it to each part's `Crew`.
  - `Opener` takes `halt: &Halt`. The `crew` entry passes it on and the `hosted` entry ignores it.
  - `planes::open` takes `[control_plane]`'s table, `&ControlPlaneTable`, the configuration file, the data directory, the say, the wake and the halt: six arguments, which clippy's `too_many_arguments` passes with no allowance. `run.rs`'s `Control::open`, `tests/planes.rs`, `tests/readme.rs` and `tests/module_files.rs` give it the table the configuration parsed, and the three tests give it `&Halt::default()`.
- **What the runner says of it** (4.1): README's running section says that the runner, sent SIGTERM or SIGINT, ends a fetch or a listing in flight as its limit would, asked to end and killed two seconds later, and begins none after. It also says that `--once` stops the same way and exits 1. README's crew section says a listing in flight is ended at the stop and its partition named unread. Neither adds a module option or changes a stop timeout.
- **The specs** (5.1): the deltas are archived into `openspec/specs/clones`, `crew-control-plane` and `running`, two requirements each, and `root:specs` passes 23 of 23.

Record 0042's amendment of 2026-11-11 and core.58 as amended, with its details, are in the tree as the plan wrote them. `nix/modules/` is unchanged, so `AbandonProcessGroup` and `KillMode=process` stand.

Against the branch's base, 57b1d1e, the branch changes only these paths:

- the files the proposal's Touches lists;
- the change's own files;
- the three specs its archive reconciled.

No crate, tool, tracked file type, configuration key, `report.json` field, crew file, protocol message or module option is added, and `boomdrive-control` is untouched.

## Requirements and scenarios

| spec | requirement | scenario | asserted by |
|---|---|---|---|
| clones | A stop of the runner ends its fetch in flight | What a halted call started ends with it | `boomdrive-bounded` `a_halted_process_is_ended_with_its_group` |
| | | A halted call is given its grace | `boomdrive-bounded` `a_halted_process_is_given_its_grace` |
| | | A call begun after its halt | `boomdrive-bounded` `a_process_begun_after_its_halt_runs_nothing` |
| | A fetch ended by the runner's stop is named | A halted fetch's words | `git::tests` `a_call_to_a_remote_halted` |
| | | A fetch begun after the stop | `git::tests` `a_call_to_a_remote_begun_after_its_halt` |
| crew-control-plane | A stop of the runner ends its listing in flight | A listing in flight when the runner stops | `read.rs` `a_listing_in_flight_when_the_runner_stops` |
| | A listing ended by the runner's stop leaves its partition unread | A listing's words at the runner's stop | `read.rs` `a_listing_in_flight_when_the_runner_stops` |
| | | A pull after the runner's stop | `read.rs` `a_pull_after_the_runners_stop_is_unread` |
| running | A stop of boomdrive run ends its try in flight | Stopped with a fetch in flight | `running.rs` `stopped_with_a_fetch_in_flight` |
| | boomdrive run --once stops as boomdrive run does | Stopped with a fetch in flight | `running.rs` `once_stopped_with_a_fetch_in_flight` |

The words `halted` gives are asserted by `boomdrive-bounded` `a_halted_calls_words_name_its_call_and_remote`. The tests that tasks 2.2 and 2.3 name beside these, `stopped_while_waiting`, `every_repository_kept` and `a_clone_failed`, also passed in the gate.

## Outcome

The gate passes and the work holds. The change is archived at e29f653, and this report is committed beside it.
