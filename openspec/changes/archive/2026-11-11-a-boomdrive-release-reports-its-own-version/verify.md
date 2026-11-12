# Verify

The verify stage of `a-boomdrive-release-reports-its-own-version`, 2026-11-11, on `change/a-boomdrive-release-reports-its-own-version` at `7d58170`.

**Outcome: the work holds.** The whole checks pass, every requirement and scenario of the change's two delta specs is met, and every task is ticked done.

The code stage archived the change as its task 6.1 asks, in `7d58170`. So this report sits beside the change in its archived folder, `openspec/changes/archive/2026-11-11-a-boomdrive-release-reports-its-own-version/`.

## The whole checks

`sluice exec main` ran once on `7d58170`, the gate's `#check-fast` in its sandbox. It exited 0: 36 tasks completed, 34 passed and recorded under their keys, and none failed.

| project | tasks |
|---|---|
| `boomdrive`, `boomdrive-control`, `boomdrive-crew`, `boomdrive-hosted` | format, lint, typecheck, test |
| `nix` | format, lint, typecheck, test (`nix: test: every case holds`) |
| `root` | adrs, changes, commits (`no errors in 366 commits`), format, releases, specs (`22 passed, 0 failed`), toolchain, versions (`Cargo.toml, its crates and Cargo.lock give 0.0.0`), workflow |
| `site` | install, build, check, prose, test, vocab, workflow |

moon warned that it could not archive `boomdrive:test`'s outputs into its cache. That is a cache write, and the task passed.

## The releases spec

Each scenario is proven by a line of `root:releases` (`check-releases.sh`) that passed in the run above, unless the row says otherwise.

| requirement | scenario | proven by |
|---|---|---|
| The version is git-cliff's | The first release, A feature, A fix, A breaking change before 1.0.0, Nothing to release | the cases kept from before, each passing |
| | A fix after a release commit | `'fix: a fix' after v0.1.0's release commit releases v0.1.1` |
| | Nothing to release after a release commit | `a chore after v0.1.0's release commit releases nothing` (it also checks that standard error says `v0.1.0 is released already`) |
| The notes are git-cliff's | A release's notes | the case kept from before |
| | The notes after a release commit | `the notes after v0.1.0's release commit hold the fix under Fixes and nothing of the start` |
| Each push to main is released | The workflow's trigger | `runs on a push to main alone`, `runs one at a time, none cancelled` |
| | A release | `writes release.sh's notes, makes the release commit of its tag, pushes the tag, publishes it with its notes and no file, then runs cache at it`, which also checks the order of the lines |
| | Nothing built, committed or pushed but the tag | `builds, attaches and commits nothing`, `one push pushes the release's tag alone, with no force` |
| Every commit since the newest release is conventional | A commit that is not conventional, Merges, reverts and several scopes, Before the newest release | the cases kept from before |
| | Before a release commit | `check-commits.sh passes 'Add a feature' before v0.1.0's release commit` |
| | After a release commit | `check-commits.sh fails 'Add a feature' after v0.1.0's release commit, naming it` |
| A release's tag names its release commit | The release commit | `release.sh commit v0.2.0 tags a commit cut from main, chore(release): v0.2.0, changing Cargo.toml and Cargo.lock alone to give 0.2.0, and says so` |
| | main is left as it was | `leaves main, the branch checked out, the index, the working tree and git status as they were`, with a change left uncommitted in the working tree |
| | A tag that exists | `exits 1 where v0.2.0 is a tag, naming it, and makes no commit` (loose objects counted) |
| | What is not a tag | `release.sh commit 0.2.0 exits 2 and makes no commit and no tag` |
| main's tree names 0.0.0 | main's tree | `root:versions` passes on the tree; `check-versions.sh passes a copy of the tree's manifests` |
| | A version where main's should be | `fails a copy that set-version.sh 0.8.0 wrote, naming Cargo.toml and 0.8.0` |
| | A crate's entry that differs | `fails a Cargo.lock that gives boomdrive-control 0.8.0, naming the crate and 0.8.0` |
| set-version.sh writes a release's version and nothing else | A release's version and nothing else | `writes Cargo.toml's version and each crate's in Cargo.lock`, and `changes no line of` each file `but its versions` |
| | What is not a version | `set-version.sh 1.2 exits 2 and leaves both files as they were` |
| | Nowhere to write it | `exits 1 on a Cargo.toml with no version, naming it and leaving it as it was` |

The site's three scenarios, which the change did not touch, pass as before.

The archived `openspec/specs/releases/spec.md` holds each requirement above. Its Purpose says the workflow `release` tags a release commit, cut from the commit of `main` that was pushed, which writes the version into the tree, and publishes its notes, building nothing (design, Risks; task 6.1).

## The family-packages spec

| requirement | scenario | proven by |
|---|---|---|
| Each package is made from its release tag's source | The runner at its tree's version | `nix:test`'s case "The runner at its tree's version", which holds `packages.<system>.boomdrive.version` to `Cargo.toml`'s `[workspace.package]` version for each system; `nix/packages/boomdrive.nix` reads that version |
| | Flume at its tag, sluice through its own flake, The runner from this tree, An overridden input | `nix:test`'s cases kept from before |

Beyond the gate, the stage ran `set-version.sh 0.8.0` on a scratch copy of the tree's `Cargo.toml`, `Cargo.lock` and `crates`. `cargo metadata --locked --offline` then accepted the written lock, giving each of the four crates `0.8.0`. So a build from a release commit's tree runs against its lock, as design's Context says cargo requires.

## Tasks

All of tasks 1.1 to 6.1 are ticked, and each names a check that passed in the run above:

- 1.1: `root:releases`.
- 1.2: `root:versions` and `root:releases`.
- 2.1, 2.2: `root:releases`.
- 2.3: `root:commits` and `root:releases`.
- 3.1: `root:workflow` and `root:releases`.
- 4.1: `nix:test`.
- 5.1: `root:format`.
- 6.1: `root:specs`.

The changed files since the plan's commit `b25a2d9` are the ones proposal.md's Impact lists, and no others:

- `.github/workflows/release.yml`, `CLAUDE.md`, `README.md` and `cliff.toml`.
- `check-commits.sh`, `check-releases.sh`, `check-versions.sh`, `release.sh` and `set-version.sh`.
- `moon.yml` and `nix/tests/packages.nix`.
- The change's own folder and the two specs.

No crate's code changed. The task notes' constraints hold:

- **No new tool or file type.** The scripts are shell, using git, awk and diff, which treefmt's shellcheck and shfmt already claim (core.7, core.8).
- **One new gate task.** `root:versions` is the only task added to the gate. It is tagged `check-fast` with the inputs design Decision 5 names.
- **Neutral scratch repositories.** `check-releases.sh` builds every repository in its scratch directory, with neutral names (core.41).

## Rules

The rules the change relies on hold in the code:

- **core.48.** `main`'s tree names `0.0.0`. Only `release.sh commit` writes a version, cut from HEAD. The tag is pushed alone, with no force. The workflow builds nothing.
- **core.47.** The tag's paths differ from the gated commit's by `set-version.sh`'s lines alone, and `root:releases` proves that.
- **core.4.** No credential is in the tree. The workflow's checkout keeps `persist-credentials: false`, and gh's helper is given only to the one push.

Record 0024's amendment of 2026-11-11 is written in `b25a2d9`, and core.48 and core.47 cite it. It stands **proposed**: it reverses the choice the user accepted on 2026-11-07, so it waits on the user's yes, as design Decision 9 says.
