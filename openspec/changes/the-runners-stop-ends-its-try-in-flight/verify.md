# Verify: the-runners-stop-ends-its-try-in-flight

The work on `change/the-runners-stop-ends-its-try-in-flight` was read at bf3cfd6. The gate fails: `boomdrive:lint` refuses `planes::open`, which this change gave an eighth argument. The work meets every requirement and scenario of its change, and every task is ticked. Since the gate fails, this stage archives nothing, and the change goes back to its code stage.

- The gate ran once through sluice against `main` at 57b1d1e. Seventeen of its eighteen tasks passed and `boomdrive:lint` failed. 729 tests passed and none failed.
- The change's deltas add 6 requirements and 10 scenarios. A test that passed in the gate asserts the outcome of each scenario.
- All 8 tasks are ticked. The code stage archived the change at bf3cfd6, as task 5.1 says to. This report sits beside the archived change, because the change's directory is there now.
- `main` and `bolt/the-runners-stop-ends-its-try-in-flight` are both at 57b1d1e, the branch's base.

## Checks

- **The gate.** `sluice exec main` at bf3cfd6 ran the 18 `check-fast` tasks that bf3cfd6 affects against `main` at 57b1d1e. It exited 1, and sluice recorded 18 results under their keys for bf3cfd6.
  - **Failed: `boomdrive:lint`.** clippy, under `-D warnings`, refuses the library and its tests:

    ```
    error: this function has too many arguments (8/7)
       --> crates/boomdrive/src/planes.rs:206:1
        |
    206 | / pub fn open(
    207 | |     word: &str,
    208 | |     table: &toml::Table,
    209 | |     file: &Path,
    ...   |
    214 | |     halt: &Halt,
    215 | | ) -> Result<Vec<Part>, String> {
        | |______________________________^
        = note: `-D clippy::too-many-arguments` implied by `-D warnings`
    error: could not compile `boomdrive` (lib) due to 1 previous error
    error: could not compile `boomdrive` (lib test) due to 1 previous error
    ```

    Task 3.1 gives `planes::open` the runner's halt as an argument of its own, beside its seven. The `Opener` type and the `crew` and `hosted` entries take seven each, which clippy passes. The fix is the code stage's to choose, and no task's check named `boomdrive:lint`.
  - Passed:
    - `boomdrive-bounded:format`, `lint`, `typecheck` and `test`;
    - `boomdrive-crew:format`, `lint`, `typecheck` and `test`;
    - `boomdrive:format`, `typecheck` and `test`;
    - `nix:typecheck` and `test`;
    - `root:format`, `adrs`, `specs` (23 of 23) and `changes`.
  - moon warned that it could not write its shared cache or its output archives outside the sandbox. It also had no internet connection for a dependency install. None of these touched a task's result.
- The gate was run once, as this stage's brief says, and nothing was fixed here.

## Tasks

All 8 tasks of `tasks.md`, groups 1 to 5, are ticked `[x]`, and none is open. What they build is in the tree, as the design names it:

- **The halt** (1.1, 1.2):
  - `boomdrive-bounded` holds `Halt`, a `Clone`, `Debug` and `Default` handle to one shared `AtomicBool`, with `halt` and `halted`, and `Stopped`, either `AtLimit` or `Halted`.
  - `output(command, limit, halt)` reads the halt before it spawns. Where the halt is set, it returns `Halted` with no status and empty pipes. Otherwise it reads the halt at each poll beside the limit, and at whichever comes first ends the group by `end_group` with `GRACE`.
  - `Ended.status` is `Option<ExitStatus>`, and `Ended.stopped` is `Option<Stopped>`.
  - `halted(call, remote)` gives `<call> with <remote> was stopped as the runner stopped`.
  - The existing tests pass `&Halt::default()` and assert `Some(Stopped::AtLimit)`, and the four new tests the tasks name are present.
- **The runner's halt** (2.1 to 2.3):
  - `Git` carries a private `halt`, never set by `new` or `default`, and `halted_by` gives a git with another. `call_remote` runs `bounded` to the limit and the halt. A halted call fails with git's last line, then `; `, then `halted(…)`, and its standard output is not read.
  - `Runner::start` makes the halt, gives it to its git, and passes it to `Control::open` and on to `planes::open`. `Stopper::stop` sets the stop flag, then the halt, then sends `Event::Stop` and stops every bell.
  - `main` calls `stop_on_signals` for `run --once` as for `run`. `Runner::once` reads the stop after its keeping pass and after its parts' passes. Where it is stopped, it says `boomdrive: stopped` and gives `Outcome::Failed`, beginning no part's pass and no settle.
  - `Scratch::silent` returns a `Silent` with `taken(within)` and `closed(within)`. The origin reads each connection on a thread of its own until its far end closes it.
  - The noting `flume` of `once_stopped_with_a_fetch_in_flight` overwrites the scratch's stub, which `Scratch::new` made runnable, so a settle would have written the file the test checks.
- **crew's listing** (3.1):
  - `Store` gains `pub halt`, never set by `Store::new`. `Store::hosts` runs its `ls-remote` to `try_for` and `halt`, and a halted listing fails with `halted("git ls-remote", &self.url)` after git's last line.
  - `Crew` gains a private halt and `halted_by`, and `Crew::store` gives its store the crew's halt. `open` and `parts` take `halt: &Halt` and give it to each part's `Crew`.
  - `Opener` and `planes::open` take `halt: &Halt`. The `crew` entry passes it on and the `hosted` entry ignores it.
  - crew's test `State::parts_halted_by` gives `parts` a halt of the test's. `tests/planes.rs`, `tests/readme.rs` and `tests/module_files.rs` give `planes::open` `&Halt::default()`.
- **What the runner says of it** (4.1): README's running section says the runner, sent SIGTERM or SIGINT, ends a fetch or a listing in flight as its limit would, asked to end and killed two seconds later, and begins none after. It also says that `--once` stops the same way and exits 1. README's crew section says a listing in flight is ended at the stop and its partition named unread. Neither adds a module option or changes a stop timeout.
- **The specs** (5.1): the deltas are archived into `openspec/specs/clones`, `crew-control-plane` and `running`, and `root:specs` passes 23 of 23.

Record 0042's amendment of 2026-11-11 and core.58 as amended, with its details, are in the tree as the plan wrote them. `nix/modules/` is unchanged, so `AbandonProcessGroup` and `KillMode=process` stand.

Against `main` at 57b1d1e, the branch changes only these paths:

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

The words `halted` gives are asserted by `boomdrive-bounded` `a_halted_calls_words_name_its_call_and_remote`. The tests that tasks 2.2 and 2.3 name beside these, `stopped_while_waiting`, `every_repository_kept` and `a_clone_failed`, passed in the gate as well.

## What goes back

The code stage makes `boomdrive:lint` pass with `planes::open` taking the runner's halt, then runs the gate again. Nothing else in the work falls short. Once the gate passes, the change can stay archived with this report beside it.
