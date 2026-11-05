# Tasks

Build the groups in order. Each group stands on the ones before it, and ends green on the moon tasks it names. A task is true once the moon task it names passes, run as it names it.

- Run each check as `devenv shell -- moon run <project>:<task>`, or through sluice while iterating, as CLAUDE.md says.
- Every test runs on the scratch host of `crates/boomdrive/tests/common`, with neutral names and paths (core.41). No test reads the user's configuration, touches a real clone or view, or reaches the network.
- No task adds a crate, a tool or a tracked file type (core.7, core.8). Nothing outside the files proposal.md's Touches lists is changed.

## 1. The runner moves no branch of its clone

- [x] 1.1 A pass moves no branch of the clone, says nothing of the clone's own branches, still points HEAD at origin's default and makes it where the clone lacks it, and keeps the view and every later repository where HEAD cannot be settled (design Decisions 1 to 3; Task notes 1.1). True when `devenv shell -- moon run boomdrive:test` passes with `tests/clones.rs` proving the clones spec's "Behind origin", "Ahead of origin" and "Origin's default is not main", and `tests/loop.rs` its "Told between passes".
- [x] 1.2 A stand-in for sluice's protection in the scratch host shows that no write of the runner's reaches a guarded branch, and that a refused creation fails nothing but its own step (design Decision 4; Task notes 1.2). True when, in `devenv shell -- moon run boomdrive:test`, `tests/clones.rs` proves the clones spec's "Sluice keeps the branch" and "A new default the clone refuses", and `tests/views.rs` its "A new repository root while sluice keeps the branch".
- [x] 1.3 README says what a pass does to the clone's branches. True when `devenv shell -- moon run root:format` passes.
- [x] 1.4 Run this group's checks. True when `devenv shell -- moon run boomdrive:format boomdrive:lint boomdrive:typecheck boomdrive:test` passes.

## 2. The change's checks

- [x] 2.1 Run the checks of every moon project this change touches. True when `devenv shell -- moon run boomdrive:format boomdrive:lint boomdrive:typecheck boomdrive:test root:format root:adrs root:specs root:changes` passes.
- [x] 2.2 The spec delta archives into specs that pass openspec's strict check. True when, after `openspec archive the-runner-reads-origin-not-sluices-copy --yes`, `devenv shell -- moon run root:specs root:changes` passes on the tree.
