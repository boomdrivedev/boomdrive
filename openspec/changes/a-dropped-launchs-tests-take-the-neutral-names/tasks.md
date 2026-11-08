# Tasks

Build the groups in order. A task is true once the check it names passes, run as it names it.

- Every test runs over a bare state repository in a temporary folder, through `crates/boomdrive-crew/tests/common`, with neutral names: partition `home`, hosts `host-a` and `host-b`, repository `example/crew` (core.41).
- No task adds a crate, a tool, a check or a tracked file type (core.7, core.8). Nothing outside the files proposal.md's Touches lists is changed.

## 1. The drops' tests take the neutral names

- [ ] 1.1 `tests/common` and crew's other tests take boomdrives-tree-holds-nothing-private's `ed66819` hunk for hunk, and `drops.rs` and the launch `read.rs`'s retry of the submission holds take the same names (design Decision 1). True when `sluice exec main -- moon run boomdrive-crew:test` passes, `drops.rs`'s tests proving the crew-control-plane spec's scenarios under "A claim stands until the runner answers its launch", "A drop is given to the runner that holds its launch whatever the plan holds", "A launch is dropped as crew reads it", "A launch a drop names is offered only where the runner launched it" and "A dropped launch the runner holds unlaunched is let go", each with the names its scenario gives.
- [ ] 1.2 crew's tests pass their format and lint. True when `sluice exec main -- moon run boomdrive-crew:format boomdrive-crew:lint boomdrive-crew:typecheck` passes.

## 2. The scenarios and the archived change take them

- [ ] 2.1 a-dropped-launch-is-never-offered's archived design, tasks and delta take the same names (design Decision 3). True when `sluice exec main -- moon run root:format root:changes` passes.
- [ ] 2.2 The delta archives into specs that pass openspec's strict check, the claim requirement's two older scenarios in boomdrives-tree-holds-nothing-private's words (design Decision 2). True when, after `openspec archive a-dropped-launchs-tests-take-the-neutral-names --yes`, `sluice exec main -- moon run root:specs root:changes` passes.
