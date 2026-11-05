# Design

## Context

See proposal.md for why. This change takes one write out of a pass, and keeps the rest of a repository's keeping going when the HEAD write that stays fails.

What stands:

- **This repository**, at `00bda37`.
  - A pass (`crates/boomdrive/src/pass.rs`) keeps each configured repository: its clone made or fetched, then `settle`, then the old view's let-go and the view. `settle` reads origin's default (`clone::default_of`), points HEAD at it and makes the branch where the clone lacks it (`clone::settle_head`), then levels it (`clone::level_facts`, `clone::level`, `clone::level_to`). Any error of `settle` returns from `keep` with the pass failed, before the view.
  - Levelling runs `update-ref refs/heads/<default> <origin's tip> <the clone's own>` where the move is a fast-forward no worktree holds, and says `<default> moved to origin/<default> at <commit>`. Otherwise it says `<default> left: …`.
  - Every other read of the runner's is at origin's refs. The view is made and moved at `refs/remotes/origin/<default>`, and `refs/boomdrive/view` records that commit (`view::facts`). An inbox is read at `refs/remotes/origin/<branch>` (`inbox::read`). The work a dropped clone holds is counted against `--remotes=origin` (`lacks`). `branch` asks sluice, and `clone::origin_commit` reads origin by `ls-remote`. A launch's base goes to Flume as given (`flume::LaunchArgs::base`; core.15).
- **sluice**, read in its statement and its code at 0.66.0:
  - Clause 124: for a branch whose policy names a remote, the host's own ref is sluice's copy, levelled to each landing sluice reads and written at no other time. Any other hand's write is refused, judged by the ref's own value before and after, and logged with the command and both revisions. S58 is a tool moving the copy to the remote's revision by hand, refused.
  - `crates/sluice/src/protect.rs`: the `reference-transaction` hook, in `prepared`, refuses an update to a branch the policy names that moves it from where it stands. It passes a creation (`a creation`), an update that leaves the value as it was, and any ref outside `refs/heads/`.
  - Record 0069: which remote a branch is on is read from every revision of it the host holds, and a copy that lags decides nothing. This was the cause herdr.13's amendment of 2026-10-26 worked around.
- **Flume**, read in its statement and its code:
  - Clause 9 cuts a place from its base's current revision, never from a copy that may lag it.
  - C.1 and `base_revision`: under sluice, the base is `origin/<base>` after one fetch where the policy lands it on a remote, else the local `<base>`. With no merge queue bound, `history.head` reads the local `refs/heads/<base>`.
  - Its binding is read at `origin/<default>` after a fetch.
- **crew** cuts a landing's place from `origin/bolt/<bolt>` after a fetch, and never reads the host's copy (the fix's brief).

There are no area pages, so `docs/architecture/core.md` binds every path this change touches.

## Goals / Non-Goals

**Goals:**

- The runner never writes a branch sluice keeps, so sluice's protection never refuses a pass (sluice's 124; core.14).
- Every branch the runner reads, it reads at origin's tracking ref after its fetch.
- A HEAD that cannot be settled stops neither the repository's view nor any other repository.

**Non-Goals:**

- **Removing a branch levelling left.** A clone's own default branch stays where the last levelling put it. Where sluice keeps the branch, the ref is now sluice's copy, and removing it is no tool's but sluice's (core.14).
- **Flume's base with no merge queue.** Flume reads it at the host's own ref (C.1), which no longer follows origin's in a clone the runner keeps. It is asked of Flume (What this change asks of others).
- **Asking sluice whether it keeps a branch.** No command of sluice's answers it, and the runner needs no answer once it writes no branch (Decision 1).

## Decisions

### 1. Levelling goes everywhere

A pass no longer moves the clone's own default branch, where sluice keeps the branch or not. `LevelFacts`, `LevelAct`, `level`, `level_facts` and `level_to` go from `clone.rs`, and their lines go from the pass.

Levelling was for readers of the host's default branch. The runner reads none. Flume, under sluice, reads the base at origin after its own fetch, and sluice reads the remote. The one reader left is Flume, for a base no merge queue lands, where clause 9 already says it should not read a copy that may lag.

Alternatives:

- **Levelling only where sluice keeps no branch of that name.** Not taken: the runner would read sluice's policy at each of the branch's revisions, its tables and patterns, as sluice and Flume already do. A third judge of sluice's rule drifts from sluice's own, and sluice's protection still refuses what the runner misjudges.
- **Levelling kept, and the hook's refusal named and passed over.** Not taken: each pass while the copy lags would be S58's tool, refused and logged as a `protected` event in sluice's records at every timer.

### 2. HEAD stays origin's, and the default branch is made where the clone lacks it

`settle_head` stays as it is. It points HEAD at `refs/heads/<default>`, and where the clone lacks that branch, makes it at `origin/<default>` with `update-ref <ref> <tip> ""`. That happens once, when the clone is made or origin names a new default. So a bare clone's HEAD names a branch that holds a commit, as in any clone git makes, and git run in the container answers. sluice's protection passes it: a ref with no value on the host is no copy it guards. No branch the clone has is moved.

Alternatives:

- **No branch made at all, HEAD left on an unborn branch.** Not taken: git run in the container fails on its HEAD, and a clone made before this change and one made after would differ in shape for no reader's sake.

### 3. A HEAD that cannot be settled fails no other step

`keep` reads origin's default first. Where that read fails, the repository is named, `default branch not read: <git's words>`, and nothing more is kept of it, since the view has no target. Where origin names none, the pass names it, as before.

Then `settle` points HEAD and makes the branch. Where either fails, the pass says `HEAD not settled on <default>: <git's words>` and counts a failed git call, so `run --once` exits 1. It then goes on to the old view's let-go and the view, which read only `origin/<default>`, and to every repository after.

### 4. The tests

Every test runs on the scratch host (`tests/common`), with neutral names and paths (core.41). A stand-in for sluice's protection, `Scratch::guard`, writes a `reference-transaction` hook into a scratch clone. In `prepared`, it refuses every update to `refs/heads/<branch>` that moves or deletes it where it stands, and, where asked, every creation too. Each refusal is a line of a file the test reads. It runs no sluice, and judges by the ref's value on the host, as sluice's does.

- `tests/clones.rs`:
  - `behind_origin` and `ahead_of_origin` show the clone's `main` left, and nothing said of it.
  - `sluice_keeps_the_default_branch` shows a guarded `main` behind origin's: nothing is refused, the pass is kept and the view moves. A move by hand afterwards is refused, so the guard is live.
  - `a_head_that_cannot_be_settled` shows a creation refused: named, failed, and the view and the next repository kept.
- `tests/views.rs`: `a_new_root_while_sluice_keeps_the_default_branch` has a clone made before the rename, its `main` guarded and behind, and a changed root. The old view is removed and the new one made at origin's tip.
- The lines other tests expect lose `main moved to origin/main at …`.

### Rules relied on

- **core.14**, as amended in this change's commit: the runner moves the clone's view and no branch of it, and reads each branch at origin's tracking ref.
- **core.1**: what a branch sluice keeps holds is sluice's to decide.
- **core.34**: HEAD and the made branch are written through the host's `git`.
- **core.41**: the tests name no host, no partition and no person's path.

### Rules departed from

None. core.14 is amended by record 0018, which departs from herdr.13 as amended 2026-10-26.

## What this change asks of others

- **Flume.** C.1 reads a base no merge queue lands at the host's own `refs/heads/<base>`. In a clone the runner keeps, origin holds that base and the host's ref is a copy that may lag, which clause 9 rules out. Flume is asked to read it at `origin/<base>` after a fetch where the repository has origin and no merge queue lands the base locally. Until then, a place there is cut from where the clone's branch stands.
- **afterthought/swancloud.** herdr.13 still says the views timer moves a host's default branch after its fetch. The runner no longer carries that part. Amending herdr.13 is swancloud's.

Both are told to the orchestrator, not done here.

## Risks / Trade-offs

- **A branch no merge queue lands lags in the clone.** Flume's places there are cut from it until Flume reads origin. The view is not affected.
- **A clone's own default branch stays where levelling left it.** A person running git in the container sees `main` behind `origin/main`.
- **A hook that refuses creations fails every pass** while origin's default is missing from the clone. It is named each time, and the view is kept.

## Migration Plan

Nothing on a host moves. At the first pass of this release, no default branch is written. A clone whose pass levelling stopped has its view kept again. A clone made before the rename is carried over first, as before (record 0016). A view a changed root left behind is removed once it holds nothing the forge lacks, and the view is made under the new root.

## After deploy

Nothing. No step of this change is outside the gate.

## Task notes

- **1.1** Remove levelling from `src/clone.rs`, with its unit tests. Rework `pass::keep` and `pass::settle` as Decision 3 has them. The lines are `HEAD names <default>, origin's default branch`, `HEAD not settled on <default>: <words>`, `default branch not read: <words>` and `origin names no default branch, so it has no view`, each after `<owner>/<name>: `.
- **1.2** `Scratch::guard(bare, branch, creations)` writes the hook and gives the file of refusals. The tests Decision 4 names prove the scenarios their requirement names.
