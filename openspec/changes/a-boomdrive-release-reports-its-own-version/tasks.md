# Tasks

Work through the tasks in order. A task is true once the check it names passes, run as it names it. The groups are headings for reading.

- Record 0024's amendment of 2026-11-11 and core.48's and core.47's amendments are in the commit that holds this change. No task writes them again.
- `check-releases.sh` makes every scratch repository and copy in its temporary folder, with its git reading no configuration of the host's and neutral names (core.41). No case runs against the tree's own history or tags.
- No task adds a crate, a tool or a tracked file type (core.7, core.8): the new scripts are shell, which treefmt's shellcheck and shfmt already claim, and they use git, awk and diff alone. The one task added to the gate is `root:versions` (design Decision 5). No crate's code changes. Nothing outside the files proposal.md's Impact lists is changed.

## 1. Where the version is carried

- [x] 1.1 `set-version.sh <major.minor.patch> [<root>]` writes the version into `Cargo.toml`'s `[workspace.package]` version and into the version of each `Cargo.lock` entry with no `source`. It changes no other line. It refuses what is not a version with exit 2, and a file with no place to write it with exit 1, naming that file and leaving it as it was (design Decision 1). `check-releases.sh` proves the releases spec's "A release's version and nothing else", "What is not a version" and "Nowhere to write it" on scratch copies (design Decision 6). `root:releases`'s inputs in `moon.yml` gain `/set-version.sh`, `/Cargo.toml`, `/Cargo.lock` and `/crates/*/Cargo.toml`. True when `sluice exec main -- moon run root:releases` passes.
- [x] 1.2 `check-versions.sh [<root>]` fails where `Cargo.toml`'s workspace version is not `0.0.0`, a member crate's `Cargo.toml` does not take `version.workspace = true`, or `Cargo.lock`'s entries with no `source` are not the members or give another version, naming the file or the crate and both versions (design Decision 5; Task notes 1.1, 1.2). `moon.yml` runs it as `root:versions`, tagged `check-fast`, with the inputs Decision 5 names. `root:releases`'s inputs gain `/check-versions.sh`. `check-releases.sh` proves "A version where main's should be" and "A crate's entry that differs" on scratch copies, and `root:versions` proves "main's tree". True when `sluice exec main -- moon run root:versions root:releases` passes.

## 2. The release commit and the newest release

- [x] 2.1 `release.sh next` runs `git-cliff --unreleased --bumped-version`, and its header says why (design Decision 3). `check-releases.sh` gains a scratch repository whose `v0.1.0` names a release commit cut from `main`. It proves the releases spec's "A fix after a release commit", "Nothing to release after a release commit" and "The notes after a release commit". Every case it proved before still passes. True when `sluice exec main -- moon run root:releases` passes.
- [x] 2.2 `release.sh commit <tag>` makes the release commit from HEAD's committed files under a temporary index, with HEAD its one parent and `chore(release): <tag>` its subject. It tags the commit `<tag>` and says so on standard error. It leaves the working tree, the index and every branch as they were. It refuses a tag that exists with exit 1 and what is not a tag with exit 2 (design Decision 2; Task notes 2.2). Its usage line and header name it. `check-releases.sh` proves "The release commit", "main is left as it was", "A tag that exists" and "What is not a tag" in a scratch repository whose `main` commits the tree's `Cargo.toml`, `Cargo.lock` and crates' manifests. True when `sluice exec main -- moon run root:releases` passes.
- [ ] 2.3 `check-commits.sh` checks the commits since the newest tag `v<major>.<minor>.<patch>` by version, every commit where there is none, and its header says why (design Decision 3). `check-releases.sh` proves "Before a release commit" and "After a release commit", and the three cases it proved before. True when `sluice exec main -- moon run root:commits root:releases` passes.

## 3. The workflow

- [ ] 3.1 `.github/workflows/release.yml` takes the tag from `release.sh next`, as now. Where there is one, it writes `release.sh notes`'s notes and runs `release.sh commit "$TAG"` with the bot's identity in the step's environment. It pushes `refs/tags/$TAG` alone through gh's credential helper given to that one push, publishes it with `gh release create "$TAG" --verify-tag --title "$TAG" --notes-file …` and no file, and runs `cache` at the tag, as design Decision 4 gives each line. The checkout keeps `persist-credentials: false`. Its header comment says the release commit. `check-releases.sh` reads the new lines and proves "The workflow's trigger", "A release" and "Nothing built, committed or pushed but the tag" (Task notes 3.1). True when `sluice exec main -- moon run root:workflow root:releases` passes.

## 4. The flake

- [ ] 4.1 `nix/tests/packages.nix` gains the case "The runner at its tree's version": for each system, `packages.<system>.boomdrive.version` is `Cargo.toml`'s `[workspace.package]` version, read from the tree, proving the family-packages spec's "The runner at its tree's version" (design Decision 7). True when `sluice exec main -- moon run nix:test` passes.

## 5. What the tree says of it

- [ ] 5.1 `README.md`'s "Installing with nix", `CLAUDE.md`'s paragraph on versions and `cliff.toml`'s header say a release's tag names its release commit, cut from the commit of `main` that was pushed, which writes the version into the tree, and that `main` names `0.0.0` (Task notes 5.1). True when `sluice exec main -- moon run root:format` passes.

## 6. The specs

- [ ] 6.1 The change's deltas archive into specs that pass openspec's strict check. After the archive, the clause of `openspec/specs/releases/spec.md`'s Purpose on the workflow `release` says it tags a release commit, cut from the commit of `main` that was pushed, which writes the version into the tree, and publishes its notes, building nothing (design, Risks). True when, after `devenv shell -- openspec archive a-boomdrive-release-reports-its-own-version --yes`, `sluice exec main -- moon run root:specs` passes.
