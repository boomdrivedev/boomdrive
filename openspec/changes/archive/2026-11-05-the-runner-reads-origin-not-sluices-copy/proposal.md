# Proposal

## Why

On 2026-11-05 a hand pass of the runner on a host exited 1 for one clone, agentplot/boomdrive-cloud's. Its levelling ran `update-ref refs/heads/main <origin's tip> <the clone's own>`, and sluice's branch protection refused it. sluice had run in that clone and installed its `reference-transaction` hook there. The pass stopped keeping that repository at the refusal, so its view stayed under the old repository root. Every other clone was kept.

The runner and sluice both moved that ref.

- **The runner.** The clones spec's "The clone's own default branch follows origin's" moved the clone's `refs/heads/<default>` to `origin/<default>` after each fetch. It carried afterthought/swancloud's herdr.13 as amended 2026-10-26, so that whatever reads a host's default branch reads origin's. core.14 said the runner alone moves the clone's default branch.
- **sluice, clause 124.** For a branch whose policy names a remote, the host's own ref of that name is sluice's copy. sluice levels it to each landing it reads, writes it at no other time, and refuses and logs any other hand's write. Since sluice's record 0069, a copy that lags decides nothing, which was the cause herdr.13's amendment worked around.

The user, 2026-11-05: "The direction I expect is that sluice owns the host's copy, and the runner reads `refs/remotes/origin/<default>`, after its fetch, everywhere it now reads the clone's own default branch: views, places, launches, and the facts it records. It then stops moving that ref, at least where sluice keeps the branch."

## What Changes

- **Rule core.14 and record 0018**, in this change's commit beside its design. The runner alone fetches the clone and moves its view. It reads every branch at origin's tracking ref, and moves no branch of the clone, whose own ref of a branch sluice keeps is sluice's copy.
- **Levelling goes.** A pass no longer moves the clone's own default branch, and says no `<default> moved to origin/<default>` or `<default> left` line. This holds everywhere, not only where sluice keeps the branch.
- **HEAD stays origin's.** A pass still points the clone's HEAD at origin's default branch, and makes that branch at origin's tip where the clone lacks it. sluice's protection passes a creation.
- **A HEAD that cannot be settled fails no other step.** Its failure is named with git's words and fails the pass. The repository's view is still kept, at `origin/<default>`, and so is every repository after it.
- **Every reader already reads origin.** A view, the commit recorded for it, an inbox and the work a dropped clone holds are read at origin's refs. A launch's base goes to Flume as given. Nothing else in the runner reads the clone's own default branch.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `clones`: the clone's own default branch is never moved, and every branch is read at origin's tracking ref; the default branch is made where the clone lacks it; a HEAD that cannot be settled leaves the view kept; a landing told of keeps the HEAD and the view, not the default branch.

## Impact

- `docs/architecture/core.md` amends core.14. `docs/adr/` gains record 0018, and record 0003's line on the host's one clone cites it.
- `crates/boomdrive`:
  - `src/clone.rs` loses levelling: `LevelFacts`, `LevelAct`, `level`, `level_facts` and `level_to`;
  - `src/pass.rs` settles HEAD and keeps the view, and a HEAD that cannot be settled no longer stops the view;
  - tests under `crates/boomdrive/tests/`, with a stand-in for sluice's protection in the scratch host.
- `README.md`: what a pass does to the clone's branches.
- Nothing else changes: no crate, no tool, no tracked file type, no message of the protocol, no control plane's implementation.
- Outside this repository:
  - Flume cuts a place from a base no merge queue lands at the host's own `refs/heads/<base>` (its C.1), which no longer follows origin's in a clone the runner keeps. That is Flume's to reconcile with its clause 9.
  - herdr.13 in afterthought/swancloud still has the host's default branch moved after a fetch. That is swancloud's to amend.
  - Both are told to the orchestrator.

## Touches

```
docs/architecture/core.md
docs/adr/0003-the-runner-starts-no-tools-process-shares-the-hosts-one-clone-and-asks-nothing-of-boom.md
docs/adr/0018-the-runner-moves-no-branch-of-its-clone-and-reads-each-at-origins-tracking-ref.md
crates/boomdrive/src/clone.rs
crates/boomdrive/src/pass.rs
crates/boomdrive/tests/clones.rs
crates/boomdrive/tests/views.rs
crates/boomdrive/tests/loop.rs
crates/boomdrive/tests/running.rs
crates/boomdrive/tests/common/**
README.md
```
