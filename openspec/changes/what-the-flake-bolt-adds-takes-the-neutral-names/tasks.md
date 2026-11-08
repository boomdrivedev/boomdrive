# Tasks

Work through the tasks in order. A task is done once the check it names passes, run as it names it. The verify stage runs the repository's whole checks once every task is done.

- Each name a task replaces takes the scrub's neutral name for its role (design Decision 1). No file of this change, and no commit, names a value it replaces.
- A Rust file a task edits is formatted with `devenv shell -- cargo fmt -p <crate>`.
- No task adds a check, a test, a task, a crate, a tool or a tracked file type, and no rule in `docs/architecture/` changes.

## 1. The tests the bolt adds take the neutral names

- [ ] 1.1 The unit tests of `crates/boomdrive/src/config.rs` that keep a drive's repositories name `example/crew`, and crew's table in `crates/boomdrive/tests/planes.rs` names the host `host-a`, each test proving the scenario it proved. True when `sluice exec main -- moon run boomdrive:test` passes.

## 2. The nix and the docs the bolt adds name no one's host

- [ ] 2.1 The comment on a drive's stream defaults in `nix/modules/options.nix` cites record 0028 (design Decision 2). True when `sluice exec main -- moon run nix:format nix:lint` passes.
- [ ] 2.2 The archived changes `a-hosts-configuration-names-its-accounts-and-their-drives`, `the-flake-configures-each-tool-by-its-own-module`, `crews-partitions-give-the-runner-its-crew-table`, `each-repository-is-named-once-by-its-owner` and `herdr-and-ghostty-come-with-the-flake-and-an-app-opens-the-agents` name `example/crew` and a host's facts, and the user's words on a drive of another machine name no host or employer (design Decisions 1 and 3). True when `sluice exec main -- moon run root:format` passes.

## 3. The specs take the neutral names

- [ ] 3.1 This change's deltas modify `configuration`'s *The local layer's repositories add to the managed layer's, and its partitions replace them*, *The managed layer may pin a key the local layer could set* and *The configuration names its accounts, each account's drives and each drive's repositories*, and `views`' *Each drive has a VS Code workspace at the repository root where the configuration asks*, each with the neutral names alone (design Decision 4). True when `sluice exec main -- moon run root:changes` passes.
- [ ] 3.2 The change is archived, so the specs take its deltas. True when `sluice exec main -- moon run root:specs` passes.
