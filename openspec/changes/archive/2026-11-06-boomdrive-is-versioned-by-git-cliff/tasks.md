# Tasks

Build the groups in order. Each group ends green on the moon tasks it names. A task is true once the moon task it names passes, run as it names it.

- Run each check as `devenv shell -- moon run <project>:<task>`, or through sluice while iterating, as CLAUDE.md says.
- The tests make their scratch repositories in a temporary directory, with neutral names and no configuration of the host's git (core.41).
- Nothing outside the files proposal.md's Touches lists is changed.

## 1. The release's tools

- [x] 1.1 The devenv shell holds git-cliff, the flake's nixpkgs' build that the workflow runs as `nix shell --inputs-from . nixpkgs#git-cliff`, and convco, each asserted to be the version devenv.nix pins, and `root:toolchain` checks both (core.8). True when `devenv shell -- moon run root:toolchain` passes.
- [x] 1.2 `treefmt.nix` formats `.versionrc` as YAML, and `cliff.toml` is TOML's (core.7). True when `devenv shell -- moon run root:format` passes.

## 2. The version and its notes

- [x] 2.1 `cliff.toml` and `release.sh` give the next tag and its notes. True when `devenv shell -- moon run root:releases` passes with `check-releases.sh` proving the releases spec's "The first release", "A feature", "A fix", "A breaking change before 1.0.0", "Nothing to release" and "A release's notes".
- [x] 2.2 `devenv shell -- release-preview` prints what the workflow would release from the checkout: the notes `release.sh notes` gives for the tag `release.sh next` gives, by the git-cliff of 1.1. True when `devenv shell -- moon run root:releases` passes with `check-releases.sh` comparing the two in a scratch repository.

## 3. The convention

- [x] 3.1 `check-commits.sh` checks the commits since the newest release tag under `.versionrc`, skipping merges and reverts, and `root:commits` runs it on the tree. True when `devenv shell -- moon run root:commits root:releases` passes with `check-releases.sh` proving the spec's "A commit that is not conventional", "Merges, reverts and several scopes" and "Before the newest release".

## 4. The workflow

- [x] 4.1 `.github/workflows/release.yml` runs on a push to `main` alone, one run at a time, takes the tag from `release.sh next`, publishes it with `gh release create` on the pushed commit with `release.sh notes`'s notes and no file, and runs `cache` at the tag. True when `devenv shell -- moon run root:workflow root:releases` passes with `check-releases.sh` proving the spec's "The workflow's trigger" and "A release".

## 5. The record, the rule and the docs

- [x] 5.1 Record 0023 and rule core.48 carry the decision, and CLAUDE.md and the README say how boomdrive is released. True when `devenv shell -- moon run root:format root:adrs` passes.

## 6. The change's checks

- [x] 6.1 Run the checks of every moon project this change touches. True when `devenv shell -- moon run root:format root:toolchain root:adrs root:specs root:changes root:commits root:releases root:workflow` passes.
- [x] 6.2 The spec delta archives into specs that pass openspec's strict check. True when, after `openspec archive boomdrive-is-versioned-by-git-cliff --yes`, `devenv shell -- moon run root:specs root:changes` passes on the tree.
