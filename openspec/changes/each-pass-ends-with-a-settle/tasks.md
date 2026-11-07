# Tasks

Work through the tasks in order. A task is true once the check it names passes, run as it names it. The groups are headings for reading.

- Every test runs on the crate's stubs and fakes, in a temporary folder, with neutral names and paths (core.41). No test runs a Flume of the host's: every settle a test makes runs a stub, or the set's Flume on a scratch host (design Decision 7).
- No task adds a crate, a tool or a tracked file type (core.7, core.8). Nothing outside the files proposal.md's Touches lists is changed.
- Record 0029, record 0003's amendment and core.13's and core.1's amendments are in the commit that holds this change; no task writes them again.

## 1. The set's Flume keeps a watch

- [x] 1.1 The set names Flume v0.17.1, which takes `settle --every`, its tag moved with the lock (design Decision 5; Task notes 1.1). True when `devenv shell -- cargo test -p boomdrive --test family each_tools_version_is_its_tags` passes.

## 2. Flume's settle, as the runner asks it

- [x] 2.1 `src/settle.rs` gives the bound, its argument and what the runner says of its settles: `bound`, `every` and `Settles::heard` (design Decisions 2 and 3; Task notes 2.1), proving the running spec's "A refusal said once until Flume settles again" and "A settle with no answer". True when `devenv shell -- cargo test -p boomdrive --lib settle::` passes.
- [x] 2.2 `flume::Cli::settle_every` runs `flume --json settle --every <n>s` and reads its answer by Flume's exit, over the fixture `tests/fixtures/flume-0.17.1/settled.json` read from the set's Flume on a scratch host, with its `README.md` (design Decisions 6 and 7; Task notes 2.2). True when `devenv shell -- cargo test -p boomdrive --test flume a_settle_as_the_hosts_watch` passes.
- [ ] 2.3 The set's Flume keeps the runner's watch on a scratch host, proving the family-packages spec's "The set's Flume keeps the runner's watch" (design Decision 7). True when `devenv shell -- cargo test -p boomdrive --test family the_sets_flume_keeps_the_runners_watch` passes.

## 3. Each turn of passes ends with a settle

- [ ] 3.1 Every run of the binary in the tests runs a stub `flume` that answers the settle: `Scratch` writes its own stub and puts it first on `Scratch::boomdrive()`'s `PATH`, and `tests/fakes/mod.rs`'s `Stub::new()` answers `settle` with the fixture (design Decision 7; Task notes 3.1). True when `devenv shell -- cargo test -p boomdrive --test running` passes.
- [ ] 3.2 The runner's programs are the runner's, set by `Runner::tools` or `carry_for`, and `tests/loop.rs`'s `Running::start` hands the runner a `fakes::Stub` as its `flume` (design Decision 6; Task notes 3.2). True when `devenv shell -- cargo test -p boomdrive --test loop` passes.
- [ ] 3.3 `run --once` settles after its passes, on the bound of Decision 2, says a refused settle and is not failed by it, proving the running spec's "One control pass", "A control plane that cannot be read", "A settle Flume refuses", "The shorter interval" and "A control plane that could not be opened", and the configuration spec's "No control plane" (design Decisions 1 to 3; Task notes 3.3). True when `devenv shell -- cargo test -p boomdrive --test control -- one_control_pass a_control_plane_that_cannot_be_read no_control_plane a_word_not_carried a_settle_flume_refuses` passes.
- [ ] 3.4 Each turn of `boomdrive run`'s loop ends with one settle, after its passes and before it waits, and a runner told to stop begins none, proving the running spec's "Passes back to back share one settle" and "Each control pass ends with a settle" (design Decisions 1 and 4; Task notes 3.4). True when `devenv shell -- cargo test -p boomdrive --test control -- passes_back_to_back_share_one_settle each_control_pass_ends_with_a_settle` passes.
- [ ] 3.5 A runner with no control plane settles after each keeping pass, and after a pass whose configuration did not read, proving the running spec's "Each keeping pass ends with a settle" and "A pass on a broken configuration settles" (design Decisions 1 and 7). True when `devenv shell -- cargo test -p boomdrive --test loop -- each_keeping_pass_ends_with_a_settle a_file_broken_while_running` passes.

## 4. What the runner says of it

- [ ] 4.1 README's "Running on a host" says each turn of passes ends with `flume settle --every <bound>`, its bound, what the runner says of it, and that a host whose runner stops runs attended once the watch lapses; its control pass's section says a control pass that follows a keeping pass at once shares its settle (design Decision 6). True when `sluice exec main -- moon run root:format` passes.

## 5. The specs

- [ ] 5.1 The change's deltas archive into specs that pass openspec's strict check. True when, after `devenv shell -- openspec archive each-pass-ends-with-a-settle --yes`, `sluice exec main -- moon run root:specs` passes.
