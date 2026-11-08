# Verify: each-pass-ends-with-a-settle

The work on `change/each-pass-ends-with-a-settle`, read at 3cbcfcb, meets its change, so the change stays archived, with this report beside it.

- Every task is ticked.
- The gate, run through sluice, passed all 17 `check-fast` tasks it ran for 3cbcfcb against `main` at 3f428cf. 626 tests passed, with none failed or ignored.
- The change has 6 requirements and 23 scenarios. A test that passed in the gate asserts the outcome of each scenario.
- The code stage archived the change at 3cbcfcb, as task 5.1 says to. `root:specs` and `root:changes` passed on the archived tree.
- The branch was cut from `bolt/the-runner-keeps-the-hosts-watch`, which stands at 3f428cf, the same commit as `main`. `git merge-tree` merges the bolt and the branch with no conflict.

## Tasks

All 11 tasks of `tasks.md`, groups 1 to 5, are ticked `[x]`, and none is open. What they build is in the tree, as the design and its Task notes name it:

- **The set's Flume** (1.1). `flake.nix` and `flake.lock` name Flume `v0.17.1` and sluice `v0.74.3`, from the chore's commit 4dd74ca. `nix/packages/flume-git-crates.nix` carries the hashes of Flume's git crates. `family.rs`'s `each_tools_version_is_its_tags` passed.
- **Flume's settle** (2.1 to 2.3).
  - `src/settle.rs` gives `bound`, `every` and `Settles::heard` as Decisions 2 and 3 have them. `every` rounds up to whole seconds and never gives less than `1s`.
  - `flume::Cli::settle_every` runs `--json settle --every <n>s` through `Cli::run`, so it unsets `FLUME_RECIPIENT` and reads the answer by Flume's exit. It belongs to `Cli` alone and is on no trait.
  - `tests/fixtures/flume-0.17.1/settled.json` is the set's Flume's answer on a scratch host, and its `README.md` says how it was read. They name no host, account or person's path (core.41).
- **Each turn of passes ends with a settle** (3.1 to 3.5).
  - `Scratch::boomdrive()` puts the scratch host's stub `flume` first on `PATH`, and `fakes::Stub::new()` answers `settle`.
  - `Programs` has moved from `Control` to `Runner`. `Runner::tools` and `carry_for` set it, and `Control::open` builds its relays from it.
  - `Runner::once` settles after its keeping pass and its control pass, with `self.config.fetch_interval`.
  - `Runner::keep` notes whether a turn ran a pass. It checks the stop after the control pass, then settles once, with the latest `fetch_interval` read, before it waits. A keeping pass whose configuration did not read still counts as a pass and settles. A landing between passes runs no settle.
  - `Runner::settle` passes `poll_interval` only while `carries()`, so a control plane that could not be opened leaves the bound at `fetch_interval`.
- **README** (4.1). "Running on a host" says each turn of passes ends with `flume --json settle --every <bound>`. It gives the bound, the three lines the runner says, that a refused settle stops nothing, and that the host runs attended once the watch lapses. The control pass's section says a control pass that follows a keeping pass at once shares its settle. The `--once` paragraph says a refused or unanswered settle is no failure.
- **The specs** (5.1). The deltas are archived into `openspec/specs/running`, `configuration` and `family-packages`, and `root:specs` passed.

Record 0029, record 0003's amendment, and the amendments to core.13's rule and details and to core.1's details are in 156324f, beside the design. They read as the code does.

The only paths changed are those the proposal's Touches lists, plus the change's own files: `git diff --stat 3f428cf HEAD` shows nothing else. No crate, tool, tracked file type, configuration key or protocol message is added. `crates/boomdrive-control`, `-crew` and `-hosted` are unchanged.

## Requirements and scenarios

| spec | requirement | scenario | asserted by |
|---|---|---|---|
| running | Each turn of passes ends with a settle of every place | Passes back to back share one settle | `control.rs` `passes_back_to_back_share_one_settle` |
| | | Each control pass ends with a settle | `control.rs` `each_control_pass_ends_with_a_settle` |
| | | Each keeping pass ends with a settle | `loop.rs` `each_keeping_pass_ends_with_a_settle` |
| | | A pass on a broken configuration settles | `loop.rs` `a_file_broken_while_running` |
| | The settle's bound is the runner's interval | The shorter interval | `control.rs` `one_control_pass` (`--every 60s`); `settle::tests::the_bound_is_the_shorter_interval` |
| | | A control plane that could not be opened | `control.rs` `a_word_not_carried` (only `--every 120s`) |
| | The settle is the runner's own and said only when it changes | A refusal said once until Flume settles again | `settle::tests::a_refusal_is_said_once_until_flume_settles_again` |
| | | A settle with no answer | `settle::tests::a_settle_with_no_answer_is_said_once` |
| | boomdrive run --once makes one pass | One control pass | `control.rs` `one_control_pass` (the settle comes last and is the only one) |
| | | A control plane that cannot be read | `control.rs` `a_control_plane_that_cannot_be_read` (`status`, `--version` and one `settle`) |
| | | A settle Flume refuses | `control.rs` `a_settle_flume_refuses` (says the line once, exits 0) |
| | | Every repository kept, A clone failed, Left for the user, Flume's status refused | unchanged by this change; their existing tests passed in the gate |
| configuration | The configuration may name the control plane | No control plane | `control.rs` `no_control_plane` (only `--every 120s`, with sluice and herdr asked nothing) |
| | | crew named, No type | unchanged; their existing tests passed |
| family-packages | The set is the one the runner's tests pass with | The set's Flume keeps the runner's watch | `family.rs` `the_sets_flume_keeps_the_runners_watch` (`watch` is `{"bound":{"seconds":60}}`, and no host tool ran) |
| | | Each tool's version, Flume's status of a scratch host, A submission sluice has no record of, Not the host's tools | unchanged; `family.rs`'s existing tests passed against the set at v0.17.1 and v0.74.3 |

`flume.rs` `a_settle_as_the_hosts_watch` holds the argv, the unset `FLUME_RECIPIENT`, the answer given as printed, and a refusal on exit 1 given with Flume's reason (design Decision 6).

`each_keeping_pass_ends_with_a_settle` checks for at least one settle after each pass, and that Flume is asked nothing else, as design Decision 7 has it. It does not count exactly one per pass. Exactly one comes from `keep`'s structure, which calls `settle` once per turn, and from the two control-pass tests, which do count exactly.

## Checks

- **The gate.** `sluice exec main` at 3cbcfcb ran the 17 `check-fast` tasks that 3cbcfcb affects against `main` at 3f428cf, and all passed:
  - `boomdrive:format`, `lint`, `typecheck` and `test`;
  - `boomdrive-crew:typecheck`, `lint` and `test`;
  - `boomdrive-hosted:test`;
  - `nix:format`, `lint`, `typecheck` and `test`;
  - `root:format`, `adrs`, `specs`, `changes` and `releases`.

  sluice recorded the 17 results under their keys for 3cbcfcb.
  - 626 tests passed, with none failed or ignored. Among them are every test the tasks name, the six unit tests of `settle::`, and `family.rs`'s tests against the set's own Flume v0.17.1 and sluice v0.74.3.
  - sluice did not run the tasks it judged 3cbcfcb leaves unaffected: `boomdrive-control`'s four, `boomdrive-crew:format`, `boomdrive-hosted`'s format, lint and typecheck, and `root:toolchain`. This change touches none of their inputs.
  - moon warned that it could not write its shared cache outside the sandbox, and the OTLP exporter failed to shut down. Neither touched a task's result.
- **Outside this repository.** As the proposal and record 0029 say, Flume's statement, section 9, still says each pass, keeping or control, runs the settle. Under record 0029, two passes run back to back share one settle. That is agentplot/blueprints' to amend, through the orchestrator. Record 0029 is `proposed` until the user accepts it.
