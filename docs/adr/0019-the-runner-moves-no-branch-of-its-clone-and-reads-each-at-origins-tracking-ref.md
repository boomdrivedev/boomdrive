---
number: 19
title: The runner moves no branch of its clone and reads each at origin's tracking ref, since the clone's own ref of a branch sluice keeps is sluice's copy
status: proposed
date: 2026-11-05
decision-makers:
- Chuck Swanberg
---

# The runner moves no branch of its clone and reads each at origin's tracking ref, since the clone's own ref of a branch sluice keeps is sluice's copy

Status: proposed on 2026-11-05 by the code stage of the fix `the-runner-reads-origin-not-sluices-copy`. The direction is the user's, in their words below. That levelling goes everywhere, and not only where sluice keeps the branch, is the stage's choice, made on sluice's statement and this repository's rules. The record is accepted when the user says yes to it.

Deciders: the user, for the direction; the code stage, for how far it goes.
Sources: the user's brief of the fix `the-runner-reads-origin-not-sluices-copy`, 2026-11-05; sluice's statement, clause 124 and its scenario S58, sluice record 0069, and its `crates/sluice/src/protect.rs`; Flume's statement, clause 9 and section C.1, and its `base_revision` and `history.head`; afterthought/swancloud's rule herdr.13 as amended 2026-10-26, cited by id (core.41); Decision 6 of the archived change `the-runner-keeps-its-repositories`; record 0003; rules core.1, core.14 and core.34; the clones spec.

## Context and problem statement

On 2026-11-05 a hand pass of the runner on a host exited 1 for the clone of agentplot/boomdrive-cloud. Its levelling ran `update-ref refs/heads/main <origin's tip> <the clone's own>`. sluice's branch protection refused it, and the clone's log for `main` records the refusal as a `protected` event. sluice had run in that clone: places submitted from it, and sluice installed its `reference-transaction` hook there. The pass stopped keeping that repository at the refusal, so its view stayed under the old repository root. Every other clone was kept.

The two tools' statements disagree about who moves that ref.

- **The runner.** Its levelling moved the clone's own `refs/heads/<default>` to `origin/<default>` after each fetch, as a fast-forward no worktree held (the clones spec; design Decision 6, carrying herdr.13 as amended 2026-10-26). The amendment was made so that whatever reads the host's default branch reads origin's. Its cause was sluice reading a host's `main` that lagged and refusing a bolt for that copy's policy.
- **sluice, clause 124.** For a branch whose policy names a remote, the remote's revision is the branch. The host's own ref of that name is sluice's copy. sluice levels it to each landing it reads, writes it at no other time, and reads nothing from it. A write to the copy by any other hand is refused and logged with the command and both revisions. S58 has this case: a tool that moves the copy to the remote's revision by hand is refused. Which remote a branch is on is now read from every revision of it the host holds, and a copy that lags decides nothing (record 0069). That is the cause the amendment worked around.
- **core.14** said the runner alone fetches the clone and moves its default branch and its view.

The user, 2026-11-05: "The direction I expect is that sluice owns the host's copy, and the runner reads `refs/remotes/origin/<default>`, after its fetch, everywhere it now reads the clone's own default branch: views, places, launches, and the facts it records. It then stops moving that ref, at least where sluice keeps the branch."

## What reads the clone's own default branch

Read from the code at this record's commit:

- **The runner reads it nowhere but in levelling itself.** A view is made and moved at `origin/<default>`, and the commit recorded for it, `refs/boomdrive/view`, is that one. An inbox is read at `origin/<branch>`. The work a dropped clone holds is counted against origin's refs. `branch` asks sluice, and checks origin by `ls-remote`. A launch's base goes to Flume as the control plane gave it (core.15).
- **Flume** cuts a place from its base's current revision: `origin/<base>` after its own fetch where sluice's policy lands the base on a remote, and `refs/heads/<base>` where sluice lands it locally or no merge queue is bound (clause 9, C.1). It reads the binding at `origin/<default>`.
- **sluice** reads the remote, and owns the copy (124).
- **crew** cuts a landing's place from `origin/bolt/<bolt>` after a fetch, and never reads the host's copy.

Levelling served one reader beside sluice: Flume, for a base no merge queue lands, where it reads the host's own `refs/heads/<base>`.

## Considered options

1. **Levelling goes.** The runner moves no branch of its clone. Where the clone lacks origin's default branch, it still makes it at origin's tip, so HEAD names a branch that holds a commit.
2. **Levelling stays only where sluice keeps no branch of that name.** The runner would read sluice's policy at each of the branch's revisions to tell.
3. **Levelling stays, and a refusal is named and passed over.** The runner tries the fast-forward at every pass and takes the hook's refusal as sluice's ownership.

## Decision outcome

Option 1.

- **The runner moves no branch of its clone.** It reads every branch at origin's tracking ref, after its own fetch. The clone's own ref of a branch sluice keeps is sluice's copy, and sluice levels it (124). Any other branch of the clone is whoever works in the clone's, and the runner leaves it as it stands (core.14).
- **It still makes the default branch where the clone lacks it.** This happens once, at origin's tip, when the clone is made or origin names a new default. So the bare clone's HEAD names a branch that holds a commit, as in any clone git makes, and git run in the container answers. sluice's protection passes a creation, since a ref with no value on the host is no copy it guards.
- **A HEAD that cannot be settled fails no other step.** Where pointing HEAD or making the default branch fails, the pass names it with git's words and counts it as a failed git call. It then still keeps the repository's view, which reads only `origin/<default>`, and every repository after it.

Option 2 would make the runner a second judge of sluice's policy. sluice already reads branch tables, patterns and the policy at every revision the host holds, and Flume reads them for a base's remote. A third reading would drift from sluice's own, and sluice's protection would still refuse whatever the runner misjudged. Option 3 would make every pass, while the copy lags, the tool S58 describes: refused, and logged as a `protected` event in sluice's records, on every timer until sluice next levels the copy.

Option 1 costs one reader: Flume, cutting a place from a base no merge queue lands. It reads the host's `refs/heads/<base>`, which now stays where the clone was made, or where levelling last left it. In a clone the runner keeps, origin holds that base, and the host's ref of it is a copy that may lag. Flume's clause 9 says a place is never cut from such a copy, but its C.1 reads the local base there. That is Flume's to reconcile, by reading `origin/<base>` after a fetch where the repository has origin and no merge queue lands the base locally. It is asked of Flume, not worked around here.

### Consequences

- core.14 is amended in this record's commit. The runner alone fetches the clone and moves its view, moves no branch of the clone, and reads each at origin's tracking ref.
- Record 0003's line on the host's one clone cites this record.
- The clones spec's requirement that the clone's own default branch follows origin's gives way to one that it is never moved. Its purpose, and the running spec's landing, no longer say the default branch is kept. That is the OpenSpec change `the-runner-reads-origin-not-sluices-copy`.
- A pass says no `<default> moved to origin/<default>` line and no `<default> left` line again. A HEAD that cannot be settled is said as `HEAD not settled on <default>: <git's words>`.
- A clone's own default branch, levelled until this release, stays where the last levelling put it. Where sluice keeps the branch, sluice levels it at the next landing it reads. The runner removes none.
- herdr.13 in afterthought/swancloud says the views timer moves a host's default branch after its fetch. The runner no longer carries that part. Amending herdr.13 is swancloud's, asked through the orchestrator.
- Flume is asked to read a base no merge queue lands at `origin/<base>`, as above.
