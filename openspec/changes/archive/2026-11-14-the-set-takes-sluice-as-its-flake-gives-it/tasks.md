# Tasks

A task is true once the check it names passes, run as it names it.

## 1. The set

- [x] 1.1 `flake.nix` pins sluice at `v0.94.8` and `flake.lock` locks it. True when `nix/tests/tags.nix`'s check holds, with `sluice exec main -- moon run nix:typecheck` passing.
- [x] 1.2 The set's `sluice` is sluice's flake's `default`, unchanged, at the locked tag's version, and `nix/packages/sluice.nix` and `nix/tests/tree-9.9.9` are gone. True when `nix/tests/packages.nix`'s case "sluice through its own flake" holds, with `sluice exec main -- moon run nix:test` passing.
- [x] 1.3 The runner's tests pass with the set's sluice `0.94.8`. True when `sluice exec main -- moon run boomdrive:test` passes.

## 2. The spec

- [x] 2.1 The change's delta passes openspec's strict check. True when `sluice exec main -- moon run root:changes` passes.
- [x] 2.2 The change's delta archives into specs that pass openspec's strict check, the scenario "sluice through its own flake" naming a tag whose tree carries its version. True when, after `devenv shell -- openspec archive the-set-takes-sluice-as-its-flake-gives-it --yes`, `sluice exec main -- moon run root:specs` passes.
