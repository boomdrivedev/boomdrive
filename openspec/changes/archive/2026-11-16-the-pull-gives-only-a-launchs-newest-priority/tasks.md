# Tasks

A task is true once the check it names passes, run as it names it.

- Every test runs over a bare state repository of `crates/boomdrive-crew/tests/common`, with neutral names and paths (core.41).
- No task adds a crate, a tool or a tracked file type (core.7, core.8). Nothing outside the files proposal.md's Touches lists is changed.

## 1. A priority is given only while it is its launch's newest

- [x] 1.1 A priority the runner does not hold is given only while no later request that reads as a priority names its `Of` (design Decisions 1 and 3). True when `devenv shell -- cargo test -p boomdrive-crew --test priorities` passes with `moot_priorities_the_runner_does_not_hold`, `moot_priorities_naming_no_host` and `a_launch_reordered_many_times_is_given_its_newest_priority_alone` proving the crew-control-plane spec's "Moot priorities the runner does not hold" and "Moot priorities naming no host".
- [x] 1.2 A priority the runner holds is given still (design Decision 3). True when the same command passes with `a_priority_the_runner_holds` proving the spec's "A priority the runner holds".
- [x] 1.3 A later priority that cannot be read hides no earlier one (design Decision 2). True when the same command passes with `a_later_priority_that_cannot_be_read` proving the spec's "A later priority that cannot be read", under its "A priority that cannot be read makes no earlier one moot".

## 2. The gate

- [x] 2.1 Every scenario of the crew-control-plane spec still holds. True when `sluice exec main -- moon run boomdrive-crew:test` passes.
- [x] 2.2 The code passes its lint and format. True when `sluice exec main -- moon run boomdrive-crew:lint boomdrive-crew:format` passes.
- [x] 2.3 Record 0002 holds the ruling. True when `sluice exec main -- moon run root:format root:adrs` passes.
- [x] 2.4 The spec delta archives into specs that pass openspec's strict check. True when, after `openspec archive the-pull-gives-only-a-launchs-newest-priority --yes`, `sluice exec main -- moon run root:specs root:changes` passes.
