## MODIFIED Requirements

### Requirement: Each push to main that git-cliff finds a new version in is released

The workflow `release` SHALL run on a push to `main` alone, one run at a time. Given a tag by `release.sh next`, it SHALL make the notes with `release.sh notes` and the release commit with `release.sh commit`. It SHALL push the tag alone with no force. It SHALL publish the tag's GitHub release with the notes and the release commit's `install.sh` as its one file, then run `cache` at the tag. It SHALL build nothing, make no commit of its own and push no branch.

#### Scenario: The workflow's trigger

- **WHEN** the workflow `release` is read
- **THEN** it runs on a push to `main` alone, one run at a time, none cancelled

#### Scenario: A release

- **WHEN** the workflow `release` is read
- **THEN** it takes the tag from `release.sh next`, and writes `release.sh notes`'s notes before it runs `release.sh commit`. It takes `install.sh` from the tag with `git cat-file`, and pushes `refs/tags/$TAG` alone. It publishes the tag with `gh release create` with the notes and `install.sh`, and runs `cache` at the tag with `gh workflow run`

#### Scenario: Nothing built, committed or pushed but the tag

- **WHEN** the workflow `release` is read
- **THEN** it holds no `nix build`, `cargo`, `upload-artifact`, `git commit` or `git tag`, and its one `git push` pushes the tag with no force

### Requirement: A release's tag names its release commit, which writes its version into the tree

`release.sh commit <tag>` SHALL make a commit from HEAD's committed files, and tag it `<tag>`. Its one parent SHALL be HEAD, and its subject `chore(release): <tag>`. Its tree SHALL be HEAD's but for the version that `set-version.sh` writes. It SHALL change no working file, index entry or branch. It SHALL refuse a tag that exists, with exit 1, and one that is not `v<major>.<minor>.<patch>`, with exit 2, making nothing.

#### Scenario: The release commit

- **WHEN** `main` holds the tree's `Cargo.toml`, `Cargo.lock` and crates' manifests, and `release.sh commit v0.2.0` runs on it
- **THEN** the tag `v0.2.0` names a commit whose one parent is `main`'s tip, and whose subject is `chore(release): v0.2.0`. The commit changes `Cargo.toml`, `Cargo.lock` and `install.sh` alone, and each gives `0.2.0` where Boomdrive's version is

#### Scenario: main is left as it was

- **WHEN** `release.sh commit v0.2.0` has run on `main`
- **THEN** `main`'s tip, the branch checked out, the working tree and `git status` are as they were before it ran

#### Scenario: A tag that exists

- **WHEN** `v0.2.0` is a tag, and `release.sh commit v0.2.0` runs
- **THEN** it exits 1, naming `v0.2.0`, and makes no commit

#### Scenario: What is not a tag

- **WHEN** `release.sh commit 0.2.0` runs
- **THEN** it exits 2, and makes no commit and no tag

## ADDED Requirements

### Requirement: install.sh carries Boomdrive's version

`install.sh`'s `version=` SHALL carry Boomdrive's version. `set-version.sh` SHALL write it and change no other line of the file. The gate's `root:versions` SHALL fail where it gives a version other than `0.0.0`, or none, naming `install.sh` and both versions (the install spec).

#### Scenario: The install's version written

- **WHEN** `set-version.sh 98.76.54` runs on a copy of the tree's manifests and `install.sh`
- **THEN** `install.sh` gives `version=98.76.54`, and no other line of it changed

#### Scenario: An install of another version

- **WHEN** `check-versions.sh` runs on a copy of the tree whose `install.sh` gives `version=0.8.0`
- **THEN** it fails, naming `install.sh` and `0.8.0`
