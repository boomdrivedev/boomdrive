# Proposal

## Why

The brief, 2026-11-11: "A boomdrive release's tag names a release commit, off main, that writes the release's version into the tree: the workspace's Cargo.toml and every file that names boomdrive's version. A build from the tag therefore reports that version and names its packages by it, with no variable or argument from the builder. main's tree names 0.0.0. This is the family's one way, as Flume's release, crew's code.10 and sluice's a-sluice-release-reports-its-own-version have it."

The design agent read boomdrive's newest tag that day: `v0.7.3` names a landing commit of `main`, whose `Cargo.toml` says `0.0.0`. So the runner built from any tag says `boomdrive 0.0.0`. That is what its `--version` prints, what its start line and its hello's `runner_version` say, and what the set's `boomdrive` package is versioned as. Record 0024 chose this. Its consequences say: "Giving the build its tag's version, with the cache's paths still those of a commit the gate passed (core.47), is a change of its own." This is that change. The same defect in sluice made a sluice built at `v0.88.0` report `0.78.1` on a host. Blueprints' principle 14 has a tool say its own.

Flume's tag `v0.21.2` names `chore(release): v0.21.2`, a commit cut from `main` whose `Cargo.toml` says `0.21.2`, written by its `set-version.sh`. crew's code.10 takes the same form from Flume, and sluice's unit `a-sluice-release-reports-its-own-version` does too.

## What Changes

- **A release's tag names its release commit.** At each push to `main` that git-cliff finds a new version in, the workflow `release` cuts a commit from the pushed one. The commit writes the version into every file that carries Boomdrive's, `Cargo.toml`'s `[workspace.package]` and `Cargo.lock`'s entries for the workspace's four crates. Its subject is `chore(release): v<version>`. The tag `v<version>` names that commit and is pushed alone, with no branch and no force. Nothing is committed to `main`, whose tree keeps `0.0.0`.
- **`set-version.sh <major.minor.patch> [<root>]`** writes a release's version into those files and no other line, as Flume's and crew's do. It refuses what is not a version, and a file it finds no version to write in.
- **`release.sh commit <tag>`** makes the release commit from HEAD's committed files and tags it. It leaves the checkout, its index and every branch as they were, so the gate's test of it runs what the workflow runs.
- **The newest release is the newest tag `v<major>.<minor>.<patch>` by version.** A release commit is off `main`'s history, so neither `git describe` nor git-cliff's walk of the history reaches its tag. `release.sh next` asks git-cliff with `--unreleased`, which starts the range at the newest tag, as `release.sh notes` already does. `check-commits.sh` checks the commits since the newest tag by version. Each tag made before this change names a commit of `main` and is read as any tag.
- **The workflow `release`** takes the tag and its notes, then makes the release commit with `release.sh commit`. It pushes the tag alone, with gh's credential helper given to that one push, and publishes the release on the tag with the notes and no file. Then it runs `cache` at the tag, as before. It still builds nothing.
- **The gate holds the version's files to `main`'s.** A new task, `root:versions`, runs `check-versions.sh`. The check fails where `Cargo.toml`'s workspace version is not `0.0.0`, a crate does not take it, or a crate's entry in `Cargo.lock` gives another version. `root:releases` proves `set-version.sh`, `check-versions.sh` and `release.sh commit` in scratch repositories.
- **The runner's package takes its tree's version.** A case of the flake's test holds the `boomdrive` package's version to the workspace's in `Cargo.toml`. That is `0.0.0` at `main` and the release's at its tag.
- **Record 0024 is amended, and core.48 and core.47 with it,** in the commit that holds this change.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `releases`: the newest release is the newest tag by version; a release's tag names its release commit, which writes the version into the tree; `main`'s tree names `0.0.0`, and `set-version.sh` writes a release's version and nothing else; the workflow pushes the tag of the release commit; the commits since the newest tag by version are held to the convention.
- `family-packages`: the runner's package's version is its tree's.

## Impact

- New files: `set-version.sh` and `check-versions.sh`, at the root.
- `release.sh`: `next` asks with `--unreleased`; a new `commit <tag>`.
- `check-commits.sh`: the newest release tag by version.
- `check-releases.sh`: the new and changed scenarios, scratch repositories with release commits off `main`, and the workflow's new lines.
- `moon.yml`: the task `root:versions`; `root:releases`'s inputs gain `set-version.sh`, `check-versions.sh`, `Cargo.toml`, `Cargo.lock` and each crate's `Cargo.toml`.
- `.github/workflows/release.yml`: the release commit, the tag's push and `--verify-tag`.
- `nix/tests/packages.nix`: the case "The runner at its tree's version".
- `cliff.toml`'s, `release.sh`'s and the workflow's header comments; `README.md`'s "Installing with nix"; `CLAUDE.md`'s paragraph on versions.
- `docs/adr/0024-…`: an amendment of 2026-11-11, proposed. `docs/architecture/core.md`: core.48 and core.47, rule and details. Both are written in the commit that holds this change.
- `openspec/specs/releases` and `openspec/specs/family-packages`, through this change's deltas.
- No crate's code changes. The runner already reads its version from `CARGO_PKG_VERSION`, which each crate takes from the workspace (`version.workspace = true`). Its tests read the same, so they pass at any version. No crate, tool, tracked file type, configuration key, report field or protocol message is added.
- `nix/packages/sluice.nix` is unchanged. Where sluice's tagged tree carries its version, it already takes sluice's package as sluice's flake gives it. That waits on sluice's own unit.
