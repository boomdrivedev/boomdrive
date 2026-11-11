## MODIFIED Requirements

### Requirement: The version is git-cliff's, from the commits since the newest release

`release.sh next` SHALL print the tag `v<version>` of the version git-cliff makes under `cliff.toml` from the commits since the newest tag `v<major>.<minor>.<patch>` by version, in the history or on a release commit off it, `v0.1.0` where there is none. Before `1.0.0`, a feature or a breaking change SHALL bump the minor version, and any other commit the notes keep the patch. Where that version is tagged already, it SHALL print nothing and say so on standard error, and SHALL write nothing.

#### Scenario: The first release

- **WHEN** a history with no release tag holds a `feat:` and a `fix:` commit
- **THEN** `release.sh next` prints `v0.1.0`

#### Scenario: A feature

- **WHEN** the newest release tag is `v0.1.0` and a `feat:` commit follows it
- **THEN** `release.sh next` prints `v0.2.0`

#### Scenario: A fix

- **WHEN** the newest release tag is `v0.1.0` and a `fix:` commit follows it
- **THEN** `release.sh next` prints `v0.1.1`

#### Scenario: A breaking change before 1.0.0

- **WHEN** the newest release tag is `v0.1.0` and a `fix!:` commit follows it
- **THEN** `release.sh next` prints `v0.2.0`

#### Scenario: Nothing to release

- **WHEN** the newest release tag is `v0.1.0` and only `chore:` and `test:` commits follow it
- **THEN** `release.sh next` prints nothing

#### Scenario: A fix after a release commit

- **WHEN** `v0.1.0` names a release commit cut from `main`, off its history, and a `fix:` commit follows on `main`
- **THEN** `release.sh next` prints `v0.1.1`

#### Scenario: Nothing to release after a release commit

- **WHEN** `v0.1.0` names a release commit cut from `main`, and only a `chore:` commit follows on `main`
- **THEN** `release.sh next` prints nothing, and says `v0.1.0` is released already

### Requirement: The notes are git-cliff's, for the same commits

`release.sh notes <tag>` SHALL print git-cliff's notes, under `cliff.toml`, of the commits since the newest release tag, headed by the tag: breaking changes, features, fixes, performance, refactors and documentation, each a group of its own in that order, each commit a line of its scope, its subject and a link to it. A chore, a test, a build, CI, a style change or a commit git-cliff cannot read SHALL be in no group, and no line SHALL hold a `Co-Authored-By` trailer.

#### Scenario: A release's notes

- **WHEN** the newest release tag is `v0.1.0`, followed by `feat(boomdrive): a feature`, `fix(nix): a fix`, `docs(adr, architecture): a record` and `chore: a chore`, the feature carrying a `Co-Authored-By` trailer, and `release.sh notes v0.2.0` runs
- **THEN** the notes are headed `v0.2.0`, hold `**boomdrive**: A feature` under Features, `**nix**: A fix` under Fixes and `**adr, architecture**: A record` under Documentation, in that order, and hold nothing of the chore or the trailer

#### Scenario: The notes after a release commit

- **WHEN** `v0.1.0` names a release commit cut from a commit of `main` that follows `feat: a start`, `fix(nix): a fix` follows on `main`, and `release.sh notes v0.1.1` runs
- **THEN** the notes hold `**nix**: A fix` under Fixes, and nothing of the start

### Requirement: Each push to main that git-cliff finds a new version in is released

The workflow `release` SHALL run on a push to `main` alone, one run at a time. Given a tag by `release.sh next`, it SHALL make the notes with `release.sh notes` and the release commit with `release.sh commit`, push the tag alone with no branch and no force, publish the tag's GitHub release with the notes and no file, and then run `cache` at the tag. It SHALL build nothing, make no commit of its own and push no branch.

#### Scenario: The workflow's trigger

- **WHEN** the workflow `release` is read
- **THEN** it runs on a push to `main` alone, one run at a time, none cancelled

#### Scenario: A release

- **WHEN** the workflow `release` is read
- **THEN** it takes the tag from `release.sh next`, writes `release.sh notes`'s notes before it runs `release.sh commit`, pushes `refs/tags/$TAG` alone, publishes it with `gh release create` on the existing tag with the notes and no file, and runs `cache` at the tag with `gh workflow run`

#### Scenario: Nothing built, committed or pushed but the tag

- **WHEN** the workflow `release` is read
- **THEN** it holds no `nix build`, `cargo`, `upload-artifact`, `git commit` or `git tag`, and its one `git push` pushes the tag with no force

### Requirement: Every commit since the newest release is conventional

The gate's `root:commits` SHALL fail where a commit since the newest release tag `v<major>.<minor>.<patch>` by version, every commit where there is none, is not a Conventional Commit under `.versionrc`, naming it, and SHALL pass otherwise. A merge commit, and a commit whose subject opens `Revert "`, SHALL not be checked. A scope SHALL be allowed to name several parts of the tree, separated by commas.

#### Scenario: A commit that is not conventional

- **WHEN** the newest release tag is followed by a commit `Add a feature`
- **THEN** the check fails, naming that commit

#### Scenario: Merges, reverts and several scopes

- **WHEN** the newest release tag is followed by a merge commit, a commit `git revert` wrote and a commit `docs(adr, architecture): a record`
- **THEN** the check passes

#### Scenario: Before the newest release

- **WHEN** a commit `Add a feature` precedes the newest release tag, and only conventional commits follow it
- **THEN** the check passes

#### Scenario: Before a release commit

- **WHEN** a commit `Add a feature` precedes the commit of `main` that the release commit of `v0.1.0` was cut from, and only conventional commits follow it on `main`
- **THEN** the check passes

#### Scenario: After a release commit

- **WHEN** `v0.1.0` names a release commit cut from `main`, and a commit `Add a feature` follows on `main`
- **THEN** the check fails, naming that commit

## ADDED Requirements

### Requirement: A release's tag names its release commit, which writes its version into the tree

`release.sh commit <tag>` SHALL make, from HEAD's committed files, a commit whose one parent is HEAD and whose subject is `chore(release): <tag>`. Its tree SHALL be HEAD's but for the version `set-version.sh` writes, and it SHALL be tagged `<tag>`. It SHALL change no file of the working tree, no entry of the index and no branch. It SHALL refuse a tag that exists, with exit 1, and a tag that is not `v<major>.<minor>.<patch>`, with exit 2, making nothing.

#### Scenario: The release commit

- **WHEN** `main` holds the tree's `Cargo.toml`, `Cargo.lock` and crates' manifests, and `release.sh commit v0.2.0` runs on it
- **THEN** the tag `v0.2.0` names a commit whose one parent is `main`'s tip, whose subject is `chore(release): v0.2.0`, which changes `Cargo.toml` and `Cargo.lock` alone, and in which each gives `0.2.0` where Boomdrive's version is

#### Scenario: main is left as it was

- **WHEN** `release.sh commit v0.2.0` has run on `main`
- **THEN** `main`'s tip, the branch checked out, the working tree and `git status` are as they were before it ran

#### Scenario: A tag that exists

- **WHEN** `v0.2.0` is a tag, and `release.sh commit v0.2.0` runs
- **THEN** it exits 1, naming `v0.2.0`, and makes no commit

#### Scenario: What is not a tag

- **WHEN** `release.sh commit 0.2.0` runs
- **THEN** it exits 2, and makes no commit and no tag

### Requirement: main's tree names 0.0.0

The gate's `root:versions` SHALL fail where `Cargo.toml`'s `[workspace.package]` gives a version other than `0.0.0`, or none, where a workspace crate does not take the workspace's version, or where a crate's entry in `Cargo.lock` gives another, naming the file or the crate and both versions, and SHALL pass otherwise.

#### Scenario: main's tree

- **WHEN** `root:versions` runs on the tree
- **THEN** it passes

#### Scenario: A version where main's should be

- **WHEN** `check-versions.sh` runs on a copy of the tree's manifests that `set-version.sh 0.8.0` wrote
- **THEN** it fails, naming `Cargo.toml` and `0.8.0`

#### Scenario: A crate's entry that differs

- **WHEN** `check-versions.sh` runs on a copy of the tree's manifests whose `Cargo.lock` alone gives the crate `boomdrive-control` `0.8.0`
- **THEN** it fails, naming `boomdrive-control` and `0.8.0`

### Requirement: set-version.sh writes a release's version and nothing else

`set-version.sh <version> [<root>]` SHALL write the version into each place `root:versions` reads in `Cargo.toml` and `Cargo.lock`, and change no other line. It SHALL refuse a version that is not `<major>.<minor>.<patch>`, with exit 2, and a file with no place to write it, with exit 1, naming that file and leaving it as it was.

#### Scenario: A release's version and nothing else

- **WHEN** `set-version.sh 98.76.54` runs on a copy of the tree's `Cargo.toml` and `Cargo.lock`
- **THEN** each place `root:versions` reads gives `98.76.54`, and every line it changed, in either file, gives `"98.76.54"`, as many added as removed

#### Scenario: What is not a version

- **WHEN** `set-version.sh 1.2` runs on a copy of the tree's `Cargo.toml` and `Cargo.lock`
- **THEN** it exits 2, and both files are as they were

#### Scenario: Nowhere to write it

- **WHEN** `set-version.sh 98.76.54` runs on a copy whose `Cargo.toml` gives no version under `[workspace.package]`
- **THEN** it exits 1, naming `Cargo.toml`, and that file is as it was
