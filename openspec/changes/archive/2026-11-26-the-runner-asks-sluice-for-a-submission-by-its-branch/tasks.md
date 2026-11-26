# Tasks

## 1. The ask names the branch

- [x] 1.1 `facts::submission` gives the number and the `target` of the submission that Flume's status names, and none where either is missing (Decision 1). True when `devenv shell -- cargo test -p boomdrive --lib facts::` and `devenv shell -- cargo test -p boomdrive --test facts` pass.
- [x] 1.2 `Sluice::entry` runs `sluice status <id> --branch <branch> --json` (Decision 1). True when `devenv shell -- cargo test -p boomdrive --test sluice` passes, proving "An entry carried", and `devenv shell -- cargo test -p boomdrive --test family` passes, proving the family-packages spec's "A submission sluice has no record of".
- [x] 1.3 The runner asks for the number on Flume's `target`. A final entry stops the asks only on its own branch (Decisions 1 and 2). True when `devenv shell -- cargo test -p boomdrive --test entries` passes, proving "A number on two branches" and "The same number on another branch".

## 2. A refusal is asked for no more

- [x] 2.1 A launch keeps sluice's refusal of its entry in `entry_refused`. The runner does not ask for that number on that branch again (Decision 3). True when `devenv shell -- cargo test -p boomdrive --test entries` passes, proving "A refused entry is asked for no more" and "Another entry after a refusal".

## 3. The set's sluice

- [x] 3.1 A family test runs each of the runner's asks against the set's sluice. In its clone, `main` and `next` each hold a submission 1. The test fails at exit 64 or a refused entry (Decision 4). True when `devenv shell -- cargo test -p boomdrive --features stand-in --test family` passes, proving "A number on two branches, from the set's sluice" and "Each ask's command line is read". The test fails with `Cli::entry`'s ask of before.

## 4. The gate

- [x] 4.1 The projects' tasks pass. True when `sluice exec main` passes.
