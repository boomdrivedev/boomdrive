# Verify: the-runners-stop-ends-its-calls-in-flight

Verified on `change/the-runners-stop-ends-its-calls-in-flight` at `4c05b3b9`, cut from `bolt/the-runner-needs-no-hand-on-a-host` and holding it at `afba153a`, on 2026-11-15.

**Outcome: the work holds.** The whole checks pass, every requirement and scenario of the three deltas is met by a test the gate ran and passed, and all thirteen tasks are ticked and true.

The code stage archived the change in `4c05b3b9`, as tasks 5.1 and 5.2 ask, before this stage ran, so this report sits in the archived directory, and the change stays archived with it.

## The whole checks

`sluice exec main` ran once on `4c05b3b9`: 34 tasks passed and none failed, in 2m 42s, their results recorded under their keys for `4c05b3b`.

| project | tasks | result |
|---|---|---|
| `boomdrive` | format, lint, typecheck, test | pass |
| `boomdrive-control` | format, lint, typecheck, test | pass |
| `boomdrive-crew` | format, lint, typecheck, test | pass |
| `boomdrive-hosted` | format, lint, typecheck, test | pass |
| `nix` | format, lint, typecheck, test | pass |
| `root` | adrs, changes, commits, format, releases, specs, toolchain, versions, workflow | pass |
| `site` | install, check, build, prose, vocab, test, workflow | pass |

`root:format` ran, in 2.8s, and moon reported no shortage of open files, the failure of sluice's #230. moon warned that it could not archive task outputs into `.moon/cache`, which the sandbox does not let it write, and that it skipped Rust's dependency install with no network. No task's outcome turns on either.

sluice also said, before it ran the checks, that the branch would conflict with main (`98d1909`) on record 0002, `docs/adr/0002-…-as-given.md`, and `openspec/specs/work-into-flume/spec.md`. That conflict is the base's: this change touches neither file, and `git merge-tree` finds `afba153a` conflicting with main on the same two. The branch merges into the bolt's tip, `3ebf70ae`, with no conflict. Main's conflict is for the bolt's landing into main, not for this change's code stage.

## Tasks

| task | holds | evidence |
|---|---|---|
| 1.1 weir at `5e3f128` | yes | `Cargo.toml` and `nix/packages/git-crates.nix` name the four crates at `5e3f1286…`, the comment citing weir record 0008 and record 0042 as amended on 2026-11-12. `git::bounded` and `Store::hosts` read `Stopped::AtLimit` and `Stopped::Halted`, and `status` only where `stopped` is none. `nix:test` and both crates' typecheck pass. |
| 1.2 the lock moves weir alone | yes | `git diff 5f05ade9^ -- Cargo.lock \| grep -c '^[-+][^-+]'` prints `8`, weir's four sources; the `windows-sys` edges name `0.61.2`. The workspace's typecheck passes. |
| 2.1 `Git` carries a halt | yes | `Git::halted_by`; `trying_for` keeps the halt; `bounded` takes `&Halt` and `call_remote` passes its own. `a_call_to_a_remote_halted`, `a_call_to_a_remote_begun_after_its_halt` and `a_git_carries_the_halt_it_is_given` pass. |
| 2.2 the runner's halt and its stopper | yes | `Runner::start` makes the halt and gives it to its git and its control plane. `Stopper::stop` sets the flag, then the halt, then sends `Event::Stop` and stops the bells. `Scratch::silent` gives a `Silent`. `a_stop_halts_every_try`, `stopped_with_a_fetch_in_flight` and `stopped_while_waiting` pass. |
| 2.3 `run --once` stops | yes | `main` calls `stop_on_signals` for both forms. `once` reads the stop after its keeping pass and after its parts' passes, says `boomdrive: stopped` and gives `Failed`, so it exits 1. `once_stopped_with_a_fetch_in_flight`, `every_repository_kept` and `a_clone_failed` pass. |
| 3.1 `Store` carries a halt | yes | `Store.halt`; `branch` gives `.with_halt(self.halt.clone())`; `hosts` runs to `try_for` and the halt, failing with `weir_bounded::halted("git ls-remote", &self.url)`. `a_stores_branches_carry_its_halt`, `a_listing_halted_in_flight`, `a_read_halted_in_flight` and `a_write_halted_in_flight` pass. |
| 3.2 crew's implementation takes the halt | yes | `Crew::halted_by`, `open`, `parts`, `Opener`, `planes::open` (with clippy's `too_many_arguments` allowed, as `pass.rs`'s long signatures are), `Control::open` and `parts_halted_by` take or pass it; the `hosted` entry ignores it. `a_pull_after_the_runners_stop_is_unread`, and the `planes`, `readme` and `module_files` tests, pass. |
| 3.3 control test | yes | `stopped_with_a_read_in_flight` passes. |
| 4.1 README | yes | The running section, its `--once` paragraph and the crew section say what the task asks, adding no module option and moving no stop timeout. The `readme` tests pass. |
| 4.2 core.58 | yes | `grep -c "weir's .5e3f128. or later" docs/architecture/core.md` prints `1`. Its details rule out a pin whose `weir-bounded` holds no halt, and its source names weir's change and this change's brief. |
| 5.1 specs archive strictly | yes | `root:specs` passes on the archived specs. |
| 5.2 the hold's archive taken in | yes | `03a23b25` merges `afba153a`; the task's grep prints `6` on `openspec/specs/crew-control-plane/spec.md`, the hold's two requirements, then this change's four, each once. `root:specs` and the workspace's typecheck pass. |
| 5.3 `root:format` hashes no ignored tree | yes | `moon.yml`'s `root:format` takes `/target`, `/result`, `/.devenv`, `/.moon/cache`, `/site/node_modules`, `/site/dist` and `/site/.astro` out of its inputs, each by a negated glob after `/**/*`, and its comment says why, naming sluice's #230. In this place, `target/debug/deps` holding 26,693 entries, `moon run root:format --force --log trace` opened 1,117 files and the task's grep prints `0`. The gate's `root:format` passes. |

Every file the change differs from `afba153a` in is in proposal.md's Touches, `moon.yml` among them, or the change's own directory. `boomdrive-control`, `boomdrive-hosted` and the modules are untouched, so `AbandonProcessGroup` and `KillMode=process` stay as they are (design Decision 7). Each requirement and scenario of the three deltas appears exactly once in its spec under `openspec/specs/`, and each of the three specs only gains lines against `afba153a`.

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

The running delta adds one requirement, "A stop of the runner ends its calls in flight", with three scenarios, the `--once` scenario named "run --once stopped with a fetch in flight", and leaves the running spec's "boomdrive run keeps the repositories until it is stopped" and "boomdrive run --once makes one pass" as they are. The added requirement carries the stop's behavior for both, within openspec's strict limit of 500 characters a requirement's text is held to.
