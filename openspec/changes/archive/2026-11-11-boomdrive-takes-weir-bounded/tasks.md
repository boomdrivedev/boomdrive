# Tasks

A task is true once the check it names passes, run as it names it.

- weir is taken at `9cba691`, weir's main with weir record 0008 landed. weir-bounded's public surface holds boomdrive-bounded's, so the swap changes no word, limit or grace (record 0042's amendment of 2026-11-11).
- Every test runs in a temporary folder, with neutral names, and reaches no network: a silent state repository is a `TcpListener` of the test's own on `127.0.0.1` (core.4, core.41).

## 1. weir at 9cba691

- [x] 1.1 `Cargo.toml` takes weir-signals, weir-capture, weir-branch and weir-bounded at `9cba69196e9269a36b02024861340355bdbe6d4b`, its comment citing weir record 0008 and record 0042's amendment, and `Cargo.lock` moves them alone. `nix/packages/git-crates.nix` names the four at that revision with fetchgit's hash there. True when `sluice exec main -- moon run nix:test` passes.

## 2. weir-bounded in place of boomdrive-bounded

- [x] 2.1 `crates/boomdrive` and `crates/boomdrive-crew` take `weir-bounded.workspace = true` in place of `boomdrive-bounded`, each comment citing weir record 0008 and record 0042's amendment. `git::bounded` and `Store::hosts` call `weir_bounded::output` and `weir_bounded::stopped`, and `TRY_FOR` is `weir_bounded::TRY_FOR`. `crates/boomdrive-bounded` is removed, `.moon/workspace.yml` lists it no more, and no task of `boomdrive` or `boomdrive-crew` takes its sources. `tests/rules.rs`'s `the_crates_depend_as_decision_1_has_them` has no crate of the tree's own holding the limit, and the binary's crate and crew's implementation depending on weir-bounded. True when `sluice exec main -- moon run boomdrive:test` passes.
- [x] 2.2 The clones spec's and crew-control-plane spec's limits hold as before: `git::tests` `a_call_to_a_remote_stopped_at_its_limit` and `a_git_tries_for_the_familys_limit`, `tests/clones.rs`'s `a_fetch_whose_origin_never_answers` and `a_clone_whose_origin_never_answers`, and `tests/read.rs`'s `a_state_repository_that_never_answers_the_listing` and `a_stores_listing_tries_for_thirty_seconds`. True when `sluice exec main -- moon run boomdrive:test boomdrive-crew:test` passes.

## 3. crew's read and write of a state repository

- [x] 3.1 `tests/read.rs` gains `a_state_repository_that_never_answers_the_read`, whose `Store::main("home")` of a silent state repository fails within forty seconds with words ending weir-branch's stopped fetch at `30s`, naming the URL, and `a_state_repository_that_never_answers_a_write`, whose `write::write` to it fails the same way without forming a file. The two run side by side in one binary. They prove the crew-control-plane spec's "A state repository that never answers the read" and "A state repository that never answers a write". True when `devenv shell -- cargo test -p boomdrive-crew --test read -- never_answers` passes.

## 4. What the runner says of it

- [x] 4.1 Record 0042 gains its amendment of 2026-11-11, and core.58's rule and details say weir's crate holds the limit and weir-branch's calls take it. README's crew section says weir-branch's read and write stop at the limit, and README's and CLAUDE.md's lists of projects drop `boomdrive-bounded`. True when `sluice exec main -- moon run root:format root:adrs` passes.

## 5. The specs

- [x] 5.1 The change's delta archives into specs that pass openspec's strict check. True when, after `devenv shell -- openspec archive boomdrive-takes-weir-bounded --yes`, `sluice exec main -- moon run root:specs` passes.
