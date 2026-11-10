# releases Specification

## Purpose
Boomdrive's versions, tags and release notes, made by git-cliff from the Conventional Commits since the newest release tag. It covers how the next version is found, what its notes hold, the workflow `release` that tags it on the commit of `main` that was pushed and publishes its notes, building nothing, and the gate's check that every commit since the newest release is conventional (core.48; record 0024).

## Requirements

### Requirement: The version is git-cliff's, from the commits since the newest release

`release.sh next` SHALL print the tag `v<version>` of the version git-cliff makes under `cliff.toml` from the commits since the newest tag `v<major>.<minor>.<patch>` in the history, `v0.1.0` where the history holds none. Before `1.0.0`, a feature or a breaking change SHALL bump the minor version, and any other commit the notes keep the patch. A commit that changes only the site's paths, `site/` and `.github/workflows/pages.yml`, SHALL bump nothing (core.48; record 0038). Where that version is tagged already, `release.sh next` SHALL print nothing and say so on standard error. It SHALL write nothing.

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

#### Scenario: A change to the site alone

- **WHEN** the newest release tag is `v0.1.0` and only a `feat(site): a page` commit follows it, changing a file under `site/` and nothing else
- **THEN** `release.sh next` prints nothing

#### Scenario: A change to the site and the runner

- **WHEN** the newest release tag is `v0.1.0` and a `fix: both` commit follows it, changing a file under `site/` and a file outside it
- **THEN** `release.sh next` prints `v0.1.1`

### Requirement: The notes are git-cliff's, for the same commits

`release.sh notes <tag>` SHALL print git-cliff's notes, under `cliff.toml`, of the commits since the newest release tag, headed by the tag: breaking changes, features, fixes, performance, refactors and documentation, each a group of its own in that order, each commit a line of its scope, its subject and a link to it. A chore, a test, a build, CI, a style change or a commit git-cliff cannot read SHALL be in no group, nor SHALL a commit that changes only the site's paths, `site/` and `.github/workflows/pages.yml` (core.48; record 0038), and no line SHALL hold a `Co-Authored-By` trailer.

#### Scenario: A release's notes

- **WHEN** the newest release tag is `v0.1.0`, followed by `feat(boomdrive): a feature`, `fix(nix): a fix`, `docs(adr, architecture): a record` and `chore: a chore`, the feature carrying a `Co-Authored-By` trailer, and `release.sh notes v0.2.0` runs
- **THEN** the notes are headed `v0.2.0`, hold `**boomdrive**: A feature` under Features, `**nix**: A fix` under Fixes and `**adr, architecture**: A record` under Documentation, in that order, and hold nothing of the chore or the trailer

#### Scenario: The site's commit in no notes

- **WHEN** the newest release tag is `v0.1.0`, followed by `fix(nix): a fix` and by `feat(site): a page`, which changes only a file under `site/`, and `release.sh notes v0.1.1` runs
- **THEN** the notes hold `**nix**: A fix` under Fixes, and nothing of the page

### Requirement: Each push to main that git-cliff finds a new version in is released

The workflow `release` SHALL run on a push to `main` and on no other event. It SHALL take the tag from `release.sh next`, and where it is given one, publish a GitHub release of that tag, made on the pushed commit, with `release.sh notes`'s notes and no file, and then run the workflow `cache` at the tag. It SHALL build nothing and commit nothing. One run SHALL release at a time.

#### Scenario: The workflow's trigger

- **WHEN** the workflow `release` is read
- **THEN** it runs on a push to `main` alone, one run at a time, none cancelled

#### Scenario: A release

- **WHEN** the workflow `release` is read
- **THEN** it takes the tag from `release.sh next`, publishes it with `gh release create` on the pushed commit with `release.sh notes`'s notes and no file, and runs `cache` at the tag with `gh workflow run`

### Requirement: Every commit since the newest release is conventional

The gate's `root:commits` SHALL fail where a commit since the newest release tag in the history, every commit where there is none, is not a Conventional Commit under `.versionrc`, naming it, and SHALL pass otherwise. A merge commit, and a commit whose subject opens `Revert "`, SHALL not be checked. A scope SHALL be allowed to name several parts of the tree, separated by commas.

#### Scenario: A commit that is not conventional

- **WHEN** the newest release tag is followed by a commit `Add a feature`
- **THEN** the check fails, naming that commit

#### Scenario: Merges, reverts and several scopes

- **WHEN** the newest release tag is followed by a merge commit, a commit `git revert` wrote and a commit `docs(adr, architecture): a record`
- **THEN** the check passes

#### Scenario: Before the newest release

- **WHEN** a commit `Add a feature` precedes the newest release tag, and only conventional commits follow it
- **THEN** the check passes
