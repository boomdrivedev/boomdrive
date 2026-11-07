# Proposal

## Why

Boomdrive has no version but Cargo.toml's `0.0.0`, no tag and no release, so a host pins a commit of `main` where it would pin a release. sluice, flume and timber each make their next version and its notes with git-cliff from their Conventional Commits at each push to `main`, and tag it. The brief of the chore asks the same of boomdrive: "a cliff.toml, a release-preview task in its devenv, and CI that tags the version git-cliff finds and writes its notes; where its binaries are published stays records 0012 and 0020's".

## What Changes

- **`cliff.toml`**, sluice's with boomdrive's links: the next version and the notes from the Conventional Commits since the newest tag `v*`, the first `v0.1.0`.
- **`release.sh`**, the workflow's two asks of git-cliff: `next`, the tag of the next version or nothing where it is tagged already, and `notes <tag>`, its notes.
- **The workflow `release`**, `.github/workflows/release.yml`: at each push to `main`, it publishes a GitHub release of the tag `release.sh next` gives, on the pushed commit, with `release.sh notes`'s notes and no file, then runs `cache` at the tag, which GitHub would not run for a tag the workflow's token made. It builds nothing and commits nothing (records 0012, 0020).
- **`root:commits`**, in the gate's fast layer: convco over the commits since the newest release tag, under **`.versionrc`**, which admits a scope naming several parts of the tree. Merges and reverts are skipped.
- **`devenv shell -- release-preview`**: what the workflow would release from the checkout, by the same git-cliff, the flake's nixpkgs' build the workflow runs. git-cliff and convco are pinned and checked by `root:toolchain` (core.8).
- **`root:releases`** tests `release.sh` and `root:commits`'s script in scratch repositories, and reads the workflow; **`root:workflow`** lints it with actionlint.
- **Record 0023 and rule core.48** carry the decision. CLAUDE.md and the README say how boomdrive is released.

## Capabilities

### New Capabilities

- `releases`: the next version and its notes from git-cliff, the workflow that tags and publishes them, and the gate's check that the commits are conventional.

### Modified Capabilities

None.

## Impact

- New files at the root: `cliff.toml`, `.versionrc`, `release.sh`, `check-commits.sh`, `check-releases.sh`, `.github/workflows/release.yml`.
- `devenv.nix` gains git-cliff and convco, pinned, and the script `release-preview`; `check-toolchain.sh` checks both; `moon.yml` gains `root:commits`, `root:releases` and `root:workflow`, which `.moon/workspace.yml`'s layers name; `treefmt.nix` formats `.versionrc` as YAML.
- `docs/adr/0023-…` and `docs/architecture/core.md`'s core.48; `CLAUDE.md` and `README.md`.
- No crate, no package of the flake and nothing of the workflow `cache` changes. The runner's build is given no version, so its `--version` stays `0.0.0` (record 0023, Consequences).

## Touches

```
cliff.toml
.versionrc
release.sh
check-commits.sh
check-releases.sh
.github/workflows/release.yml
devenv.nix
check-toolchain.sh
moon.yml
.moon/workspace.yml
treefmt.nix
docs/adr/0023-boomdrive-is-versioned-by-git-cliff-from-its-conventional-commits-and-its-release-is-a-tag-and-notes.md
docs/architecture/core.md
CLAUDE.md
README.md
```
