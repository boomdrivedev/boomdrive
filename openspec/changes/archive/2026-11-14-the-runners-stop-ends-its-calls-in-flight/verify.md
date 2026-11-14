# Verify: the-runners-stop-ends-its-calls-in-flight

Verified on `change/the-runners-stop-ends-its-calls-in-flight` at `86a7f1ec`, cut from `bolt/the-runner-needs-no-hand-on-a-host`, on 2026-11-14.

**Outcome: the work holds.** The whole checks pass, every requirement and scenario of the three deltas is met by a test the gate ran and passed, and all eleven tasks are ticked and true.

The code stage archived the change in `86a7f1ec`, as task 5.1 asks, before this stage ran, so this report sits in the archived directory, and the change stays archived with it.

## The whole checks

`sluice exec main` ran once on `86a7f1ec`: 25 tasks passed and none failed, in 3m 18s, their results recorded under their keys for `86a7f1e`.

| project | tasks | result |
|---|---|---|
| `boomdrive` | format, lint, typecheck, test | pass |
| `boomdrive-control` | format, lint, typecheck, test | pass |
| `boomdrive-crew` | format, lint, typecheck, test | pass |
| `boomdrive-hosted` | format, lint, typecheck, test | pass |
| `nix` | format, lint, typecheck, test | pass |
| `root` | changes, format, releases, specs, versions | pass |

`boomdrive:lint` passes: `planes::open`, which task 3.2 gives an eighth argument, carries `#[allow(clippy::too_many_arguments)]` (`af3b9949`), as `pass.rs`'s three long signatures and crew's `write::texts` do. moon warned that it could not archive the outputs of `boomdrive:test` and `nix:test` into `.moon/cache`, which the sandbox does not let it write. Neither task's outcome turns on that.

## Tasks

| task | holds | evidence |
|---|---|---|
| 1.1 weir at `5e3f128` | yes | `Cargo.toml` and `nix/packages/git-crates.nix` name the four crates at `5e3f1286…`, the comment citing weir record 0008 and record 0042 as amended on 2026-11-12. `git::bounded` and `Store::hosts` read `Stopped::AtLimit` and `Stopped::Halted`, and `status` only where `stopped` is none. `nix:test` and both crates' typecheck pass. |
| 1.2 the lock moves weir alone | yes | `git diff 5f05ade9^ -- Cargo.lock \| grep -c '^[-+][^-+]'` prints `8`, weir's four sources; the `windows-sys` edges name `0.61.2` again (`41ac5d98`). `cargo check --workspace --tests --locked` passes. |
| 2.1 `Git` carries a halt | yes | `Git::halted_by`; `trying_for` keeps the halt; `bounded` takes `&Halt` and `call_remote` passes its own. `a_call_to_a_remote_halted`, `a_call_to_a_remote_begun_after_its_halt` and `a_git_carries_the_halt_it_is_given` pass. |
| 2.2 the runner's halt and its stopper | yes | `Runner::start` makes the halt and gives it to its git and its control plane. `Stopper::stop` sets the flag, then the halt, then sends `Event::Stop` and stops the bells. `Scratch::silent` gives a `Silent`. `a_stop_halts_every_try`, `stopped_with_a_fetch_in_flight` and `stopped_while_waiting` pass. |
| 2.3 `run --once` stops | yes | `main` calls `stop_on_signals` for both forms. `once` reads the stop after its keeping pass and after its parts' passes, says `boomdrive: stopped` and gives `Failed`, so it exits 1. `once_stopped_with_a_fetch_in_flight`, `every_repository_kept` and `a_clone_failed` pass. |
| 3.1 `Store` carries a halt | yes | `Store.halt`; `branch` gives `.with_halt(self.halt.clone())`; `hosts` runs to `try_for` and the halt, failing with `weir_bounded::halted("git ls-remote", &self.url)`. `a_stores_branches_carry_its_halt`, `a_listing_halted_in_flight`, `a_read_halted_in_flight` and `a_write_halted_in_flight` pass. |
| 3.2 crew's implementation takes the halt | yes | `Crew::halted_by`, `open`, `parts`, `Opener`, `planes::open`, `Control::open` and `parts_halted_by` take or pass it; the `hosted` entry ignores it. `a_pull_after_the_runners_stop_is_unread`, and the `planes`, `readme` and `module_files` tests, pass. |
| 3.3 control test | yes | `stopped_with_a_read_in_flight` passes. |
| 4.1 README | yes | The running section, its `--once` paragraph and the crew section say what the task asks, adding no module option and moving no stop timeout. The `readme` tests pass. |
| 4.2 core.58 | yes | `grep -c "weir's .5e3f128. or later" docs/architecture/core.md` prints `1`. Its details rule out a pin whose `weir-bounded` holds no halt, and its source names weir's change and this change's brief. |
| 5.1 specs archive strictly | yes | `root:specs` passes on the archived specs, which `86a7f1ec` syncs as `b9a6ee5f` did (see the note). |

Every file the change touches is in proposal.md's Touches. `boomdrive-control`, `boomdrive-hosted` and the modules are untouched, so `AbandonProcessGroup` and `KillMode=process` stay as they are (design Decision 7).

## Specs

Every scenario is met by a test the gate ran and passed:

- **clones**:
  - "A fetch in flight at the stop" and "A halted fetch's words": `git::tests::a_call_to_a_remote_halted`
  - "A fetch begun after the stop" and "A fetch begun after the stop says so alone": `git::tests::a_call_to_a_remote_begun_after_its_halt`
  - "The runner's git carries one halt": `git::tests::a_git_carries_the_halt_it_is_given`
  - "A new clone halted in flight": `running::stopped_with_a_fetch_in_flight`
- **crew-control-plane**:
  - "A store's branches carry its halt": `a_stores_branches_carry_its_halt`
  - "A listing in flight when the runner stops" and "A listing's words at the runner's stop": `a_listing_halted_in_flight`
  - "A read in flight when the runner stops" and "A read's words at the runner's stop": `a_read_halted_in_flight`
  - "A write in flight when the runner stops" and "A write's words at the runner's stop": `a_write_halted_in_flight`
  - "A pull after the runner's stop" and "A pull's words after the runner's stop": `a_pull_after_the_runners_stop_is_unread`
- **running**:
  - "Stopped with a fetch in flight": `running::stopped_with_a_fetch_in_flight`
  - "Stopped with a part's read in flight": `control::stopped_with_a_read_in_flight`
  - "run --once stopped with a fetch in flight": `running::once_stopped_with_a_fetch_in_flight`
  - the stopper's part of all three: `run::tests::a_stop_halts_every_try`

## Note

The running delta took its archived shape at the first archive, `b9a6ee5f`. The plan's delta modified two requirements of the running spec, "boomdrive run keeps the repositories until it is stopped" and "boomdrive run --once makes one pass", which once synced run past openspec's strict limit of 500 characters and fail `root:specs`. The archived delta adds one requirement in their place, "A stop of the runner ends its calls in flight", with the same behavior and the same three scenarios, the `--once` scenario named "run --once stopped with a fetch in flight", and leaves the two requirements as they were. `86a7f1ec` archives that same delta again, and the specs it syncs are `b9a6ee5f`'s.
