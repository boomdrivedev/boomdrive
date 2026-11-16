# Proposal

## Why

The brief, 2026-11-16: "The runner is the only thing that packs and prunes the repository copies it keeps on a host. Git's automatic cleanup is off in them, so a fetch or an agent's commit never starts one mid-work, and long-lived copies stay small and fast." Sources: row 669 of boomdrive-cloud's `design/research/flywheel-next-ideas.md`; the rule a blueprints agent drafted for that row, amending core.14; the clones spec; record 0051 and core.14 as amended, in this change's first commit.

A host's clone of a repository is its one clone (core.14), and it lives as long as the host keeps the repository. Flume's places, sluice's refs and every agent's commits write into it. Nothing of the runner, Flume or sluice says who packs or prunes it, so git's own collection does. A fetch, or any commit in a place, that finds more than git's bound of loose objects or packs starts `git gc` or `git maintenance` in the clone before it returns. On a scratch clone holding 7,003 loose objects, one commit in a worktree packed them all. That collection also packs refs, expires refs' logs and prunes worktrees, each of which is another tool's (core.14).

## What Changes

- **Git's own collection is off in every clone.** The runner sets `gc.auto` to `0` and `maintenance.auto` to `false` in a clone's own config: in a new clone before its first fetch, and in each kept clone at every keeping pass, before its fetch, where the clone's config says otherwise. So no fetch, commit or tool's call in the clone, or in a worktree of it, starts git's collection. A setting that cannot be written is named and fails the pass, and the repository is still fetched and its view kept.
- **A keeping pass packs a clone once what arrived passes git's bound.** Once a pass has kept and let go of every repository and kept the workspaces, it reads each kept clone's loose objects and packs. It packs a clone holding more than 6,700 loose objects or more than 50 packs, git's own `gc.auto` and `gc.autoPackLimit`. What the clone reaches goes into one pack, and what it does not reach but was written within two weeks into a cruft pack. The rest is dropped, and the clone's commit graph is written. The pass says `<address>: packed <n> loose objects and <m> packs`. Below both bounds it does nothing and says nothing.
- **A pack drops only what nothing of the clone reaches, past two weeks.** No object a ref, a ref's log, or a worktree's HEAD or index reaches is dropped, and none written within two weeks, git's own `gc.pruneExpire`.
- **A pack touches objects alone.** It packs no ref, expires no ref's log and prunes no worktree.
- **A pack waits on nothing and holds no lock.** It holds no clone's lock, so a launch into the clone never waits on it. The pass reads its stop before each clone, and keeps a landing told while it packs between two packs. A pack that fails is named, `<address>: not packed: <git's words>`, and fails the pass, and every other clone is still packed.
- **A dropped repository's clone is not packed**, as it is not fetched.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `clones`: git's own collection is off in every clone; a keeping pass packs each kept clone past git's bound, dropping only what nothing of it reaches past two weeks and touching no ref, log or worktree; a pack that fails is named; a dropped repository's clone is not packed.

## Impact

- `crates/boomdrive/src/clone.rs`: the two settings at a clone's making and at its keep, the count of a clone's loose objects and packs, the bound, and the pack.
- `crates/boomdrive/src/pass.rs`: the settings turned off before each kept clone's fetch, and the packing at the end of the keeping pass.
- `crates/boomdrive/tests/packs.rs`, a new test file of the crate's: the spec's new scenarios.
- `README.md`: what a keeping pass does to the clones it keeps.
- `docs/adr/0051-*.md` and `docs/architecture/core.md`: record 0051 and core.14 as amended, in this change's first commit, carry the construct stage's choices.
- Once this lands, an occasion's first pass on each host packs every clone past the bound, each costing what its history holds.
- Nothing else changes: no crate, no tool, no tracked file type, no key of the configuration, no message of the protocol, no control plane's implementation.
