---
number: 24
title: Boomdrive is versioned by git-cliff from its Conventional Commits, and its release is a tag of main and its notes
status: accepted
date: 2026-11-06
decision-makers:
- Chuck Swanberg
---

# Boomdrive is versioned by git-cliff from its Conventional Commits, and its release is a tag of main and its notes

Status: accepted by the user on 2026-11-07, on their card. Proposed on 2026-11-06 by the code stage of the chore `boomdrive-is-versioned-by-git-cliff`, from its brief. Its amendment of 2026-11-11, below, is proposed.

Deciders: the brief of the chore, in the bolt `boomdrive-releases-are-versioned`, 2026-11-06, for what is versioned and how; the code stage, for its form, each choice below with the path it did not take.
Sources: the brief, quoted below; agentplot/sluice's, agentplot/flume's and agentplot/timber's `cliff.toml`, `.versionrc`, `devenv.nix` and `.github/workflows/ci.yml` at origin/main on 2026-11-06, each public; agentplot/timber's record 0004, cited by id; records 0012, 0014 and 0020; rules core.7, core.8, core.37, core.44 and core.47; GitHub's documentation on the events a workflow's `GITHUB_TOKEN` triggers; git-cliff 2.14.2 and convco 0.7.2, run on this repository's history on 2026-11-06.

## Context and problem statement

The brief, in its words: "boomdrive's versions, tags and release notes come from its Conventional Commits through git-cliff, as sluice's, flume's and timber's do: a cliff.toml, a release-preview task in its devenv, and CI that tags the version git-cliff finds and writes its notes; where its binaries are published stays records 0012 and 0020's".

Boomdrive has no version but Cargo.toml's `0.0.0`, no tag and no release. Its commits are Conventional Commits already: convco reads every one of its 233 commits that is not a merge. Its one workflow, `cache`, pushes the flake's packages to the family's cache at each push to `main` and each tag `v*` (record 0020). sluice, flume and timber each make their next version and its notes with git-cliff at each push to `main`, under one `cliff.toml` with their own links, and tag it. flume tags a release commit off `main` with the version written into its tree; sluice and timber tag the pushed commit, and their version lives only in the tag. Each checks in CI that its commits since its last release are conventional, since a commit git-cliff cannot read bumps nothing and is in no notes. Unlike theirs, boomdrive's CI builds and tests nothing: `main` holds only what sluice's gate passed (core.47).

## Considered options

For where the version lives:

1. **In the tag alone, on the commit of `main` that was pushed,** as sluice's and timber's.
2. In a release commit off `main`, the tag naming it, as flume's.
3. In the tree, committed to `main` through the gate before each tag.

For where the commits are held to the convention:

1. **In the gate, a root task over the commits since the newest release tag.**
2. In CI after the push, as sluice's, flume's and timber's: a check that fails once the commit is on `main` already.
3. In a commit-msg hook, which boomdrive's devenv installs none of, and which a session can skip.

For the tag's binaries in the family's cache:

1. **The workflow `release` runs `cache` at each tag it makes.**
2. Nothing: GitHub runs no workflow on a tag a workflow's `GITHUB_TOKEN` made, so `cache` would never run at a tag.
3. The tag made with a token of its own, whose push runs `cache`: a credential the repository would hold for this alone.

## Decision outcome

**Option 1, the version in the tag alone.** At each push to `main`, the workflow `release` asks git-cliff for the next version from the commits since the newest tag `v<version>` in the pushed commit's history, under `cliff.toml`. Where that version is not tagged yet, it publishes a GitHub release of the tag `v<version>`, made on the pushed commit, with git-cliff's notes for the same commits. The repository never holds its version, and nothing is committed by the workflow. A release commit off `main` (option 2) is a commit the gate did not pass, whose paths `cache` would push at its tag (core.47); a version committed through the gate (option 3) is a commit for each release and a version a branch can race to.

**`cliff.toml` is sluice's,** its links boomdrive's: breaking changes, features, fixes, performance, refactors and documentation, in that order, each line its scope and its subject; chores, tests, CI and builds in no notes; a commit git-cliff cannot read, a merge or a `Revert "…"` among them, in no notes and no bump. The first version is `0.1.0`. Before `1.0.0`, a feature or a breaking change bumps the minor version and anything else in the notes the patch. A push whose commits are all left out of the notes releases nothing.

**`release.sh` holds the workflow's two asks of git-cliff,** the next tag and its notes, so the gate's test runs what the workflow runs. **`devenv shell -- release-preview`** shows what the workflow would release from a checkout, by the same git-cliff: the flake's nixpkgs' build, which the workflow runs as `nix shell --inputs-from . nixpkgs#git-cliff`, and the shell's packages asserted to be its version (core.8).

**Option 1, the convention held in the gate.** `root:commits` runs convco over the commits since the newest release tag in the candidate's history, every commit where there is none, under `.versionrc`, which admits a scope naming several parts of the tree, as `docs(adr, architecture)`. Merges and commits `git revert` wrote are skipped, as git-cliff leaves them out. A check after the push (option 2) finds a commit `main` cannot lose; a hook (option 3) is skipped by a session that does not run it.

**Option 1, `cache` run at each tag.** After the release, `release` runs `cache` at the tag by `workflow_dispatch`, which GitHub runs for a `GITHUB_TOKEN`, so the tag's packages reach the family's cache as record 0020 says. The tag is a commit of `main`, which the environment `cachix` admits as it admits every tag `v*`. A token of its own (option 3) is a secret the repository would keep for one push.

**The release builds nothing.** It publishes no file: where the binaries are published stays records 0012's and 0020's, the family's cache now and the attested release record 0012 names to come (core.37, core.47).

### Consequences

- Rule core.48 holds this record.
- The first push to `main` after this lands releases `v0.1.0`, with notes of every commit before it that the notes keep.
- The runner's build is given no version: the flake's runner at a tag reports Cargo.toml's `0.0.0`, as it does at any commit, and so does the `runner_version` its hello sends. Giving the build its tag's version, with the cache's paths still those of a commit the gate passed (core.47), is a change of its own.
- The repository is public with its whole history (record 0014 as amended on 2026-11-13), so its tags and releases are public and each stands on the commit it names.

## Amendment, 2026-11-11: a release's tag names a release commit that writes its version into the tree

Status: proposed on 2026-11-11 by the construct stage of the change `a-boomdrive-release-reports-its-own-version`, on its brief from the design agent. It moves where the version lives from option 1 above, which the user accepted on 2026-11-07, to option 2. So it stands proposed until the user says yes to it. How it is built is the stage's own choice, each part cheap to change, with the path it did not take in the change's design.

The brief, in its words: "A boomdrive release's tag names a release commit, off main, that writes the release's version into the tree: the workspace's Cargo.toml and every file that names boomdrive's version. A build from the tag therefore reports that version and names its packages by it, with no variable or argument from the builder. main's tree names 0.0.0. This is the family's one way, as Flume's release, crew's code.10 and sluice's a-sluice-release-reports-its-own-version have it."

The design agent found the defect that day. Boomdrive's newest tag, `v0.7.3`, names a landing commit whose `Cargo.toml` says `0.0.0`. So the runner built from any tag says `boomdrive 0.0.0`: in its `--version`, in its hello's `runner_version` and in the set's package. The same form of release made a sluice built at `v0.88.0` report `0.78.1` on a host. Blueprints' principle 14 has a tool say its own. The consequence above left this to a change of its own. Flume's tag `v0.21.2` names `chore(release): v0.21.2`, whose `Cargo.toml` says `0.21.2`. crew's code.10 takes Flume's form, and sluice's unit does the same.

- **The version lives in a release commit, which the tag names.** At each push to `main` that git-cliff finds a new version in, the workflow `release` cuts a commit from the pushed one. The commit writes the version into every file that carries Boomdrive's, `Cargo.toml`'s `[workspace.package]` and `Cargo.lock`'s entries for the workspace's crates, by `set-version.sh`, the family's writer by name. Its subject is `chore(release): v<version>`. The tag `v<version>` names it, and is pushed alone, with no branch and no force. `main` is never committed to, and its tree names `0.0.0`. A build from the tag reports the version by the tree alone.
- **The gate proves what a release commit changes.** Option 2 was set aside above because a release commit is one the gate did not pass, whose paths `cache` would push. Its tree is now the pushed commit's, which the gate passed, with the version lines `set-version.sh` writes. `root:releases` proves, at every candidate, that the writer changes no other line of the tree's own files. `root:versions` holds `main`'s tree to `0.0.0` and each crate to the workspace's version. The cache takes the tag's paths on that proof (core.47 as amended).
- **`release.sh commit <tag>` makes the release commit.** It reads HEAD's committed files and builds the commit under a temporary index. It leaves the checkout and every branch as they were, so the gate's test runs what the workflow runs, as it does for `next` and `notes`.
- **The newest release is the newest tag by version.** A release commit is off `main`'s history, which neither `git describe` nor git-cliff's walk of the history reaches. `release.sh next` asks git-cliff with `--unreleased`, as `notes` and `release-preview` already do. `check-commits.sh` starts at the newest tag `v<major>.<minor>.<patch>` by version. The tags made before this amendment name commits of `main` and are read as any tag. None is moved.
- **The push takes its credential for one command.** The checkout keeps no credential. The tag's push is given gh's credential helper, with the step's token.

### Consequences of the amendment

- Good: the runner at a tag, its hello and the set's package say the release's version, with nothing given by the builder (principle 14).
- Good: one form of release across Flume, crew, sluice and Boomdrive.
- Bad: each release is one more commit, off `main`, that no gate ran. Its difference from a gated commit is proven, and no more.
- Bad: a run that pushes the tag and fails to publish leaves a tag with no release, which the next run does not mend.
