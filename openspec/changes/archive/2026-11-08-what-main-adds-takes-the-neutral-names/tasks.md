# Tasks

Work through the tasks in order. A task is done once the check it names passes, run as it names it. The verify stage runs the repository's whole checks once every task is done.

- Each name a task replaces takes the scrub's neutral name for its role (design Decision 1). No file of this change, and no commit, names a value it replaces.
- A Rust file a task edits is formatted with `devenv shell -- cargo fmt -p <crate>`.
- No task adds a check, a test, a task, a crate, a tool or a tracked file type, and no rule in `docs/architecture/` changes.

## 1. The tests main adds take the neutral names

- [x] 1.1 The runner's tests `crates/boomdrive/tests/bolts.rs` and `commands.rs`, and the unit test of `crates/boomdrive/src/carry.rs` that keeps a dropped clone, name `example/crew`, ids, marks and the stream under `home`, each test proving the scenario it proved. True when `sluice exec main -- moon run boomdrive:test` passes.
- [x] 1.2 The tests the merge resolved, the interface's `crates/boomdrive-control/tests/json.rs`, crew's `crates/boomdrive-crew/tests/claims.rs` and the runner's `crates/boomdrive/tests/control.rs`, take `main`'s tests with the names under `home`, the hosts `host-a` and `host-b` and the repositories under `example`. True when `sluice exec main -- moon run boomdrive-control:test boomdrive-crew:test boomdrive:test` passes.

## 2. The docs main adds cite, and name no one's host

- [x] 2.1 Record 0029 cites the planner's signal by its date, with no state repository or signal id (design Decision 3). True when `sluice exec main -- moon run root:adrs` passes.
- [x] 2.2 The archived changes `a-bolt-is-a-launch` and `a-dropped-repository-is-drained-and-no-rename-is-followed` name `example/crew`, ids and the stream under `home` and the host `host-a`, and crew's follow-up names agentplot/crew (design Decisions 1 and 2). True when `sluice exec main -- moon run root:format` passes.

## 3. The specs take the neutral names

- [x] 3.1 This change's deltas modify `work-into-flume`'s *A bolt is a launch* and *A bolt's send is a retry of its submission*, `crew-control-plane`'s *A launch the runner answered is given while the runner keeps or drains its repository* and *A launch naming this host takes its repository from those kept, then those drained*, and `commands`' *A command is refused, naming its tool*, each with the neutral names alone (design Decision 4). True when `sluice exec main -- moon run root:changes` passes.
- [x] 3.2 The change is archived, so the specs take its deltas. True when `sluice exec main -- moon run root:specs` passes.
