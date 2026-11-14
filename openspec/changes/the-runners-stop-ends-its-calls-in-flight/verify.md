# Verify: the-runners-stop-ends-its-calls-in-flight

Verified on `change/the-runners-stop-ends-its-calls-in-flight` at `b9a6ee5f`, cut from `bolt/the-runner-needs-no-hand-on-a-host`, on 2026-11-14.

**Outcome: the work falls short. The gate fails on one lint error, so it goes back to its code stage.** The code is otherwise complete: every requirement and scenario of the three deltas is met, every task is ticked, and every test the change adds passes.

The code stage archived the change in `b9a6ee5f`, as task 5.1 asks, before this stage ran. That is why this report sits in the archived directory. This stage archived nothing.

## The whole checks

`sluice exec main` ran once on `b9a6ee5f`, the checks it affects against `main` (`08100b2`). Result: 24 tasks passed and 1 failed. The run took 5m 24s.

| task | result |
|---|---|
| `boomdrive:format`, `typecheck`, `test` | pass |
| `boomdrive:lint` | **fail** |
| `boomdrive-control:*`, `boomdrive-crew:*`, `boomdrive-hosted:*` (format, lint, typecheck, test) | pass |
| `nix:format`, `lint`, `typecheck`, `test` | pass |
| `root:changes`, `format`, `releases`, `specs`, `versions` | pass |

`boomdrive:lint` fails because clippy runs with `-D warnings`:

```
error: this function has too many arguments (8/7)
   --> crates/boomdrive/src/planes.rs:425:1
425 | / pub fn open(
...
433 | |     halt: &Halt,
434 | | ) -> Result<Vec<Part>, String> {
    = note: `-D clippy::too-many-arguments` implied by `-D warnings`
error: could not compile `boomdrive` (lib) due to 1 previous error
error: could not compile `boomdrive` (lib test) due to 1 previous error
```

Task 3.2 adds `halt: &Halt` to `planes::open`, which gives it an eighth argument. `Opener`, `crew` and `hosted` stay at seven, so clippy flags only `open`. Task 3.2's own check, `cargo test -p boomdrive --test planes --test readme --test module_files`, passes. The gate's lint task still fails, and core.7 makes it binding.

## Tasks

All ten tasks are ticked `[x]`.

| task | holds | evidence |
|---|---|---|
| 1.1 weir at `5e3f128` | yes, with one note | `Cargo.toml` and `git-crates.nix` name the four crates at `5e3f1286…` with their hash, and the comment cites weir record 0008 and record 0042 as amended on 2026-11-12. `git::bounded` and `Store::hosts` read `Stopped::AtLimit` and `Stopped::Halted` as the task has it. `status` is read only where `stopped` is none. `nix:test` and `typecheck` pass. See note 2. |
| 2.1 `Git` carries a halt | yes | `Git::halted_by`; `trying_for` keeps the halt; `bounded` takes `&Halt`. `a_call_to_a_remote_halted`, `a_call_to_a_remote_begun_after_its_halt` and `a_git_carries_the_halt_it_is_given` pass. |
| 2.2 the runner's halt and its stopper | yes | `Runner::start` makes the halt and gives it to `git`. `Stopper::stop` sets the flag, then the halt, then sends `Event::Stop` and stops the bells. `Scratch::silent` gives a `Silent`. `a_stop_halts_every_try` and `stopped_with_a_fetch_in_flight` pass. |
| 2.3 `run --once` stops | yes | `main` calls `stop_on_signals` for both forms. `once` reads the stop after its keeping pass and after its parts' passes, then says `boomdrive: stopped` and gives `Failed`. `once_stopped_with_a_fetch_in_flight` passes. |
| 3.1 `Store` carries a halt | yes | `Store.halt`; `branch` uses `.with_halt`; `hosts` runs to `try_for` and the halt and uses `weir_bounded::halted` words. `a_stores_branches_carry_its_halt`, `a_listing_halted_in_flight`, `a_read_halted_in_flight` and `a_write_halted_in_flight` pass. |
| 3.2 crew's implementation takes the halt | yes, but fails lint | `Crew::halted_by`, `open`, `parts`, `Opener`, `planes::open`, `Control::open` and `parts_halted_by` take or pass the halt. `a_pull_after_the_runners_stop_is_unread` passes. Its eighth argument on `planes::open` is the lint failure above. |
| 3.3 control test | yes | `stopped_with_a_read_in_flight` passes. |
| 4.1 README | yes | The running section, the `--once` paragraph and the crew section say what the task asks. `readme` tests pass. |
| 4.2 core.58 | yes | `grep -c "weir's .5e3f128. or later" docs/architecture/core.md` prints `1`. The details rule out a pin whose `weir-bounded` holds no halt, and the source names weir's change and this change's brief. |
| 5.1 specs archive strictly | yes, with one note | `root:specs` passes on the archived specs. See note 1. |

The touched files are all within proposal.md's Touches list. `boomdrive-control`, `boomdrive-hosted` and the modules are untouched, so neither `AbandonProcessGroup` nor `KillMode=process` moves (design Decision 7).

## Specs

Every scenario is met by a test that the gate ran and passed:

- **clones**:
  - "A fetch in flight at the stop" and "A halted fetch's words": `git::tests::a_call_to_a_remote_halted`
  - "A fetch begun after the stop" and "…says so alone": `a_call_to_a_remote_begun_after_its_halt`
  - "The runner's git carries one halt": `a_git_carries_the_halt_it_is_given`
  - "A new clone halted in flight": `running::stopped_with_a_fetch_in_flight`
- **crew-control-plane**:
  - "A store's branches carry its halt": `a_stores_branches_carry_its_halt`
  - listing in flight and its words: `a_listing_halted_in_flight`
  - read in flight and its words: `a_read_halted_in_flight`
  - write in flight and its words: `a_write_halted_in_flight`
  - "A pull after the runner's stop" and its words: `a_pull_after_the_runners_stop_is_unread`
- **running**:
  - "Stopped with a fetch in flight": `running::stopped_with_a_fetch_in_flight`
  - "Stopped with a part's read in flight": `control::stopped_with_a_read_in_flight`
  - "run --once stopped with a fetch in flight": `running::once_stopped_with_a_fetch_in_flight`
  - The stopper's part of all three: `run::tests::a_stop_halts_every_try`

## Notes

1. **The running delta was reshaped when the change was archived.** The plan's delta MODIFIED two requirements: "boomdrive run keeps the repositories until it is stopped" and "boomdrive run --once makes one pass". `b9a6ee5f` replaced it with one ADDED requirement, "A stop of the runner ends its calls in flight", and the two requirements stayed as they were.
   - Why: on a copy of the tree at `b9a6ee5f^`, the plan's delta passes `openspec validate --strict` and archives. The specs it then yields fail `root:specs`, because both modified requirements run past openspec's strict limit of 500 characters.
   - What changed: the new requirement carries the same behavior and the same three scenarios. The `--once` scenario is renamed "run --once stopped with a fetch in flight" so the two names differ within one requirement. `running.rs`'s doc comment follows the new name.
   - What is missing: `b9a6ee5f`'s message does not say why. This is recorded here, and no fix is asked for.
2. **`Cargo.lock` moved more than the four weir crates.** Besides weir's four sources, the lock moved the `windows-sys` edge of `errno`, `rustix`, `rustls-platform-verifier`, `tempfile` and `winapi-util` from 0.61.2 to 0.52.0. Task 1.1 says the lock "moves the four alone". No package is added, and no package version changes; both `windows-sys` versions were already locked. The edges are Windows-only, so no host build changes. The code stage may restore those edges while it fixes the lint.

## Found work for the code stage

- `boomdrive:lint` fails because `crates/boomdrive/src/planes.rs:425`, `planes::open`, takes 8 arguments where clippy allows 7 under `-D warnings`. The gate must pass whole.
