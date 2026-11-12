# Design

## Context

See proposal.md for why. This change carries out record 0024 as amended on 2026-11-11, which this change's commit writes with core.48 and core.47 as amended.

What stands, at `b574550`:

- The version Boomdrive's tree carries is `Cargo.toml`'s `[workspace.package] version = "0.0.0"`. Each of the four crates takes it, `version.workspace = true`. `Cargo.lock` gives it again in each crate's entry, the four entries with no `source`. No other tracked file carries Boomdrive's version. The site's `package.json` names none, and the site's commits release nothing (core.48).
- The runner reads its version from `CARGO_PKG_VERSION`: clap's `--version` (`src/main.rs`), the start line (`src/run.rs`) and the hello's `runner_version` (`src/hello.rs`). Its tests read the same: `tests/running.rs`, `tests/hello.rs` and `tests/dropped.rs`. `"boomdrive 0.0.0"` in the other crates' tests is a hello's sample value, not the runner's own.
- `nix/packages/boomdrive.nix` versions the package by `Cargo.toml`'s workspace version. Its source is `Cargo.toml`, `Cargo.lock` and `crates/` alone. buildRustPackage builds against the lock, and cargo refuses a lock whose crate entries differ from their manifests (read 2026-11-11: `cargo metadata --locked --offline` on a copy whose `Cargo.toml` alone said `0.8.0` refused to update the lock, and passed once the four entries said it too).
- `release.sh next` runs `git-cliff --bumped-version`, and `release.sh notes <tag>` runs `git-cliff --unreleased --tag <tag> --strip all`. `check-commits.sh` takes the newest release by `git describe --tags --abbrev=0 --match 'v[0-9]*'`. `release-preview` runs `git-cliff --unreleased --bump`.
- The workflow `release` checks out with `persist-credentials: false`, finds the tag with `release.sh next`, and publishes it with `gh release create "$TAG" --target "$GITHUB_SHA"`. That makes the tag on the pushed commit through GitHub's API. Then it runs `cache` at the tag with `gh workflow run`. `check-releases.sh` reads those lines, and fails where the workflow holds `git push`, `git commit` or `git tag`.
- git-cliff 2.14.2, the devenv shell's, was run on 2026-11-11 in scratch repositories whose tag names a commit cut from `main`, off its history. Without `--unreleased`, `--bumped-version` reads the whole history as unreleased: after `v0.1.0` on such a commit and a `fix:` on `main`, it gives `v0.2.0`. With `--unreleased` it gives `v0.1.1`, and gives `v0.1.0`, which is tagged, where only a `chore:` follows. With `--unreleased`, every scenario of the releases spec on a tag of `main` gives what it gives today. `git describe` finds no tag there.
- Flume's `set-version.sh` and its `ci.yml` release job, and crew's `set-version.sh` and `release.yml`, make the family's release commit: version written, committed as `chore(release): v<x.y.z>`, tagged, the tag pushed. Each runs the version and the notes with `--unreleased`. Each checks the convention from the newest tag by `git tag --list 'v*' --sort=-v:refname`.

There are no area pages for these paths, so `docs/architecture/core.md` binds every path this change touches.

## Goals / Non-Goals

**Goals:**

- A build from a release tag reports that release's version, by the tree alone: `boomdrive --version`, the start line, the hello and the set's package (blueprints principle 14).
- `main`'s tree names `0.0.0`, and the gate fails a candidate whose doesn't.
- The release commit changes no line but the version's, and the gate proves the writer that makes it.
- The newest release, its next version, its notes and the commits held to the convention are read alike for a tag on `main` and for a tag on a release commit.

**Non-Goals:**

- **A build or a test in the workflow `release`.** It still builds nothing and publishes no file (core.37, core.48). The binaries stay the cache's (record 0020) and the attested release's of record 0012.
- **The runner's code.** It already reports `CARGO_PKG_VERSION`.
- **sluice's version in the set.** `nix/packages/sluice.nix` gives sluice its tag where sluice's tagged tree says `0.0.0`, and leaves sluice's package as it is where the tree carries a version. sluice's own unit makes its tree carry it.
- **The tags made before this change.** Each stays on the commit of `main` it names. No tag is moved or made again (core.48).
- **A `release` branch,** which Flume moves for its marketplace. Boomdrive has no plugin whose installer reads a branch.

## Decisions

### 1. `set-version.sh` writes the version where Boomdrive's tree carries it

`sh set-version.sh <major.minor.patch> [<root>]` takes the family's form, Flume's and crew's script by name, arguments and refusals. `<root>` defaults to the folder the script is in. It writes the version into two places:

- `Cargo.toml`: the first `version = "…"` line under `[workspace.package]`.
- `Cargo.lock`: the `version = "…"` line of each `[[package]]` entry that has no `source`, the workspace's crates.

Each file is rewritten through a temporary file beside it and moved into place. A version not `[0-9]+\.[0-9]+\.[0-9]+` is refused with exit 2, and a wrong count of arguments with the usage line and exit 2. Where either file has nowhere to write the version, the script exits 1 and names that file. That file is left as it was. A crate's own `Cargo.toml` carries no version of its own, so it is not written. `check-versions.sh` holds each crate to the workspace's.

Paths not taken: a subcommand of `release.sh`, which would give the family's one writer a second name; a version given to the build by the environment, as sluice's and timber's releases did, which a build from the tag's tree alone never learns (principle 14).

### 2. `release.sh commit <tag>` makes the release commit, touching nothing else

`release.sh commit <tag>` refuses a tag that is not `v<major>.<minor>.<patch>` (exit 2), and a tag that already exists (exit 1, naming it). Otherwise it does these steps:

1. Copies HEAD's committed `Cargo.toml` and `Cargo.lock` (`git show HEAD:<file>`) into a temporary folder, and runs `set-version.sh <tag without v> <folder>` on them.
2. Writes the two files as blobs (`git hash-object -w`). It builds a tree from HEAD's under a temporary index (`GIT_INDEX_FILE`, `git read-tree HEAD`, `git update-index --cacheinfo`, each with its mode in HEAD), and writes it (`git write-tree`).
3. Commits that tree with HEAD as its one parent and the message `chore(release): <tag>` (`git commit-tree`), by git's own identity.
4. Tags the commit `<tag>`, a lightweight tag as Flume's and crew's are. It says `release: <tag> names <commit>, cut from <HEAD>` on standard error.

It reads HEAD's committed files, never the working tree. So nothing uncommitted in a checkout reaches a release. It changes no file of the working tree, no entry of the index and no branch. The temporary folder and index are removed on exit. The workflow gives git its identity, `github-actions[bot]`, through the step's environment. `check-releases.sh` gives the scratch repositories theirs.

Paths not taken: the release commit's commands inline in the workflow, as Flume's and crew's are, which the gate could only read and never run; a commit made in the checkout, which in a run by hand moves the branch the checkout is on; a worktree of its own, which needs the whole tree checked out to write two files.

### 3. The newest release is the newest tag by version

A release commit is off `main`'s history, so a tag's commit is no longer one that `main`'s history reaches.

- `release.sh next` runs `git-cliff --unreleased --bumped-version`. `--unreleased` starts git-cliff's range at its newest tag, wherever the tag's commit is. That range is the commits `main` took since the commit the release was cut from. Where the version it gives is tagged already, `next` still prints nothing.
- `release.sh notes` and `release-preview` already pass `--unreleased`.
- `check-commits.sh` takes the newest of `git tag --list 'v*' --sort=-v:refname` that is `v<major>.<minor>.<patch>`, and checks `<tag>..HEAD`. A tag on a release commit excludes everything its parent reaches, so the range is the same commits git-cliff reads. Where there is no tag, it checks every commit, as before.

The tags made before this change name commits of `main`, and are read as any tag: the newest by version starts the range.

Path not taken: git-cliff's `--use-branch-tags`, which reads only tags on the branch's history, and so would never see a release commit's tag.

### 4. The workflow `release` makes, pushes and publishes the release commit

Its one job, after the checkout (`fetch-depth: 0`, `persist-credentials: false`) and nix:

1. `tag=$(nix shell --inputs-from . nixpkgs#git-cliff -c sh release.sh next)`, as now.
2. Where there is a tag, in one step:
   1. `release.sh notes "$TAG"` into `$RUNNER_TEMP/notes.md`. It runs before the release commit is tagged, since `--unreleased` would then start at the new tag.
   2. `sh release.sh commit "$TAG"`, with `GIT_AUTHOR_NAME`, `GIT_AUTHOR_EMAIL`, `GIT_COMMITTER_NAME` and `GIT_COMMITTER_EMAIL` the bot's, `github-actions[bot]` and `41898282+github-actions[bot]@users.noreply.github.com`, in the step's environment.
   3. `git -c credential.helper= -c 'credential.helper=!gh auth git-credential' push origin "refs/tags/$TAG"`: the tag alone, with no branch and no force. The token is the step's `GH_TOKEN`, handed to this one command by gh's own helper.
   4. `gh release create "$TAG" --verify-tag --title "$TAG" --notes-file "$RUNNER_TEMP/notes.md"`, with no file.
   5. `gh workflow run cache.yml --ref "$TAG"`, as now.

`contents: write` covers the tag's push and the release, and `actions: write` covers `cache`'s dispatch, as now. One run at a time, none cancelled, so each run reads the tags the run before it pushed.

Paths not taken: `persist-credentials: true`, as Flume's and crew's checkouts have it, which leaves the token in `.git/config` for every later step; the commit and the tag made through GitHub's API, which is longer and which the gate cannot run.

### 5. The gate holds `main`'s tree to `0.0.0`

`check-versions.sh [<root>]`, the task `root:versions`, reads the tree at `<root>`, by default the folder it is in. It fails in each of these cases, naming the file, the place and both versions:

- `Cargo.toml`'s `[workspace.package]` gives no version, or one other than `0.0.0`.
- A member that `[workspace] members` lists has a `Cargo.toml` with no `version.workspace = true`.
- `Cargo.lock`'s entries with no `source` are not exactly the members' package names, or one gives a version other than the workspace's.

It passes otherwise. It is a check of the tree, run by the gate on every candidate for `main` and for a bolt, and never at a tag. A release commit never passes through the gate.

`root:versions` declares its inputs: `/Cargo.toml`, `/Cargo.lock`, `/crates/*/Cargo.toml` and `/check-versions.sh`. `root:releases`'s inputs gain `/set-version.sh`, `/check-versions.sh`, `/Cargo.toml`, `/Cargo.lock` and `/crates/*/Cargo.toml`, since its scratch repositories copy them.

Path not taken: the tree's check folded into `root:releases`. That would make one task two checks (core.7), and the releases test proves the scripts in scratch repositories, as it proves `check-commits.sh` beside `root:commits`.

### 6. The gate proves the writer, the commit and the reads in scratch repositories

`check-releases.sh` adds these cases to the ones it has. Each scratch repository is made as now, its git reading no configuration of the host's, with neutral names (core.41).

- **set-version.sh.** On a scratch copy of the tree's `Cargo.toml` and `Cargo.lock`, `set-version.sh 98.76.54 <copy>` passes. Afterwards each place of Decision 1 gives `98.76.54`, and a `diff` of each file against the tree's shows as many lines removed as added, every added line giving `"98.76.54"`. `set-version.sh 1.2 <copy>` exits 2 and leaves both files as they were. A `Cargo.toml` with no `[workspace.package]` version makes it exit 1, naming `Cargo.toml`, and leaves that file as it was.
- **check-versions.sh.** It passes on a scratch copy of the tree's `Cargo.toml`, `Cargo.lock` and each crate's `Cargo.toml`. It fails on the same copy once `set-version.sh 0.8.0` has written it, naming `Cargo.toml` and `0.8.0`. It fails on a copy whose `Cargo.lock` alone gives one crate `0.8.0`, naming that crate.
- **release.sh commit.** It runs in a repository whose `main` commits the tree's `Cargo.toml`, `Cargo.lock` and crates' manifests beside a file. `release.sh commit v0.2.0` makes the tag `v0.2.0`. Its commit's one parent is `main`'s tip, and its subject is `chore(release): v0.2.0`. Its tree differs from `main`'s in `Cargo.toml` and `Cargo.lock` alone, and both give `0.2.0` where Decision 1 writes. `main`'s tip, the checked-out branch, the working tree and `git status` are as they were. A second `release.sh commit v0.2.0` exits 1, naming the tag, and makes no commit. `release.sh commit 0.2.0` exits 2.
- **next, notes and the convention after a release commit.** In a repository whose `v0.1.0` names a release commit cut from `main`, there are three cases. A `fix:` on `main` gives `release.sh next` `v0.1.1`. A `chore:` alone gives it nothing, saying `v0.1.0 is released already`. `release.sh notes v0.1.1` after `fix(nix): a fix` holds that fix and nothing of the commits before the cut. `check-commits.sh` passes where `Add a feature` precedes the cut and only conventional commits follow it. It fails, naming the commit, where `Add a feature` follows it.
- **The workflow.** It holds each line of Decision 4 once. It holds no `nix build`, `cargo` or `upload-artifact`, and no `git commit` or `git tag`. Its one `git push` pushes `refs/tags/$TAG`, with no `--force`, `-f` or branch.

### 7. The runner's package takes its tree's version

`nix/packages/boomdrive.nix` already reads `Cargo.toml`'s workspace version. A case of `nix/tests/packages.nix`, "The runner at its tree's version", holds `packages.<system>.boomdrive.version` to that version for each system. So a change that gives the package any other version fails the flake's test. At a release tag, the tree's version is the release's (Decision 2), so the package and its store path are named by it. No recipe changes.

### 8. What reaches the family's cache

At each tag, `cache` builds the flake at the tag's release commit. Flume's and sluice's paths are main's, and the runner's is new. The release commit is not a commit the gate passed. Its tree is the pushed commit's, which the gate passed, with the version lines `set-version.sh` writes. The gate proves `set-version.sh` changes no other line (Decision 6). core.47 as amended admits those paths, and nothing else of a commit the gate did not pass. The environment `cachix` admits tags `v*` as before.

### 9. The record and the rules

Record 0024's amendment of 2026-11-11 moves where the version lives from its option 1 to its option 2. It answers the objection the record gave against option 2 with Decisions 6 and 8, and keeps the rest of the record. core.48's rule and details say the release commit. core.47's details admit the release commit's paths. The amendment is proposed. It reverses a choice the user accepted on 2026-11-07, and stands proposed until they say yes to it. The construct stage writes it, and the rules with it, in the commit that holds this change. No task writes them again.

Blueprints holds no statement of Boomdrive's, so no clause is amended. Principle 14, which the brief cites, stands as it is.

## Risks / Trade-offs

- **A release commit the gate never ran.** Its tree differs from a gated one only by `set-version.sh`'s lines, which the gate proves on the tree's own files at every candidate. A change that makes a new file carry Boomdrive's version fails `check-versions.sh` only where it is one the check reads. Such a change extends Decision 1's places and the check together.
- **A push refused by GitHub.** A tag ruleset could refuse the bot's push. The step then fails before the release is published and before `cache` runs. The next push to `main` finds the version untagged and makes it again.
- **A tag with no release.** A run that pushes the tag and then fails to publish leaves a tag with no release. The next run finds that version tagged and prints nothing. It releases the version after it once a commit bumps it. Mending that tag's release is ops' work, and no tag is moved (core.48).
- **The releases spec's Purpose** says the workflow tags the commit of `main` that was pushed. openspec's deltas carry no Purpose for a spec that stands. So task 6.1 rewrites that sentence once the change is archived, as a step of this change.

## Task notes

- **1.1, 1.2.** The scratch copies are made under `$scratch`, as `check-releases.sh`'s other cases are. `check-versions.sh` reads `[workspace] members` from `Cargo.toml`, each member's `Cargo.toml` under `<root>`, and `Cargo.lock`, with awk alone, as `set-version.sh` does. No tool is added (core.8).
- **2.2.** `release.sh commit` finds `set-version.sh` beside itself (`$(dirname "$0")`), so the scratch repositories copy it as they copy `release.sh`.
- **3.1.** The workflow's step names stay "Find the next version" and "Release it". The new lines are the ones Decision 4 gives, laid out as yamlfmt lays them, so `check-releases.sh` reads each by a basic regular expression as it reads the others.
- **5.1.** `README.md`'s "Installing with nix" says each landing that git-cliff finds a new version in is released as a tag `v<version>` on a release commit, cut from that commit of `main`, which writes the version into the tree, so the runner at the tag says it; `main`'s runner says `0.0.0`. `CLAUDE.md`'s paragraph on versions says the same in place of "The version lives only in the tag". `cliff.toml`'s header says the newest tag names a release commit off `main`, which only `--unreleased` starts the range at.
