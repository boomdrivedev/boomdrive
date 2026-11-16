---
number: 51
title: The runner alone packs and prunes its clones, and git's own collection is off in them
status: accepted
date: 2026-11-16
decision-makers:
- the construct stage of change/the-runner-keeps-its-clones-packed
---

# The runner alone packs and prunes its clones, and git's own collection is off in them

Status: accepted. The direction is the change's brief, quoted below, which carries row 669 of boomdrive-cloud's `design/research/flywheel-next-ideas.md` and the rule a blueprints agent drafted for it. Its form here is the construct stage's own choice, each point cheap to change.

Deciders: the construct stage of `change/the-runner-keeps-its-clones-packed`, from its brief: "The runner is the only thing that packs and prunes the repository copies it keeps on a host. Git's automatic cleanup is off in them, so a fetch or an agent's commit never starts one mid-work, and long-lived copies stay small and fast."

Sources: boomdrive-cloud `design/research/flywheel-next-ideas.md`, row 669; the rule drafted for that row in the blueprints session's handoff `tool-others.md`, ROW 669, which amends core.14; flywheel-next `design/models/statechart/model.md`, section 11 (a host while `alive`) and 12.15 (its 327 and 328); boomdrivedev/sluice `crates/sluice/src/protect.rs` and its record 0110; Flume's `crates/flume/src/profiles/place/git_worktree/mod.rs`; git 2.48's `git repack`, `git prune`, `git commit-graph` and `git count-objects`, and its `gc.auto`, `gc.autoPackLimit`, `gc.pruneExpire` and `maintenance.auto`; the clones spec; rules core.13, core.14, core.34, core.35, core.38 and core.58; the OpenSpec change `the-runner-keeps-its-clones-packed`, whose design gives each point below.

## Context and problem statement

A host's clone of a repository is its one clone (core.14). It lives as long as the host keeps the repository, and Flume's places, sluice's refs and every agent's commits write into it. No rule, record or spec of the runner, Flume or sluice says who packs or prunes it. The clones spec only fetches. Flume and sluice run `git worktree prune` and nothing that packs. Today git's own collection is the only upkeep: a fetch, or any commit in a place, that finds more loose objects than `gc.auto` or more packs than `gc.autoPackLimit` starts `git gc` or `git maintenance` in the clone, in the middle of whatever started it. On a scratch clone with 7,003 loose objects, one commit in a worktree packed all of them before it returned.

That collection also does more than pack. It packs refs into `packed-refs`, expires refs' logs, and prunes worktrees whose folders are gone. Each touches what another tool made in the clone, which core.14 rules out for the runner. sluice's protection lets git's packing of refs through (sluice record 0110) only because a fetch's maintenance ran it.

flywheel-next's model had each host switch git's collection off in every repository it kept, pack between ticks once what had arrived passed a bound, drop what nothing referenced past a cutoff, and consolidate packs below what a reader can open.

## Considered options

1. **The runner turns git's collection off and packs at its own pass, by git's own commands for objects alone** (chosen).
2. **Git's collection left on**, as now.
3. **The runner runs `git gc --auto` at its pass**, with git's collection off otherwise.
4. **`git maintenance` on the host's scheduler**, registered for each clone.
5. **The model's two kinds of occasion**: the loose objects packed alone at most occasions, and the packs consolidated once per so many.

## Decision outcome

Option 1.

- **One owner.** The runner alone packs and prunes a clone it keeps. Flume and sluice do neither in it today, so no other tool's design changes. Not taken: option 2, under which whichever command crosses git's bound pays for the collection mid-work, and no record chose it.
- **Git's collection is off.** The runner sets `gc.auto` to `0` and `maintenance.auto` to `false` in the clone's own config: in a new clone with its other keys, before its first fetch, and at every keeping pass, before its fetch, where the clone's own config says otherwise. It reads each by git's own type, so `no` reads as `false`, and writes nothing where the clone already says so, so a steady pass writes no config. Every worktree of the clone, a view or a place, reads the same config. A setting that cannot be written is named, `<address>: git's own collection not turned off: <git's words>`, and fails the pass, and the repository is still fetched and its view kept, since a fetch with git's collection on is how every clone was kept before.
- **An occasion comes past git's own bounds.** Once a keeping pass has kept and let go of every repository and kept the workspaces, it reads each kept clone's loose objects and packs by `git count-objects -v`, and packs the clone where it holds more than 6,700 loose objects or more than 50 packs: git's own defaults for `gc.auto` and `gc.autoPackLimit`, the bounds at which its collection would have started. Below both, nothing is done or said. The bounds are fixed: no key of the configuration sets them (core.17), as no key sets core.58's limit. Not taken: a configuration key, which no host has needed.
- **An occasion packs the clone whole.** It runs `git repack -d -l --cruft --cruft-expiration=2.weeks.ago`, which writes what the clone reaches into one pack and what it does not reach, but was written within two weeks, into a cruft pack. It then runs `git prune --expire=2.weeks.ago`, which drops the loose objects the repack left, those nothing reaches written more than two weeks before. It then runs `git commit-graph write --reachable`, which git's collection also wrote. Two weeks is git's own `gc.pruneExpire`. What the clone reaches is git's own reckoning: every ref, every ref's log, and each worktree's HEAD and index. After an occasion the clone holds no loose object and at most two packs, so what the next occasion counts is what arrived since. The line is `<address>: packed <n> loose objects and <m> packs`, each count as `count-objects` gave it before the pack, with `object` and `pack` for one. Not taken: option 5, which costs less per occasion on a large clone, but leaves the unreachable objects of the last two weeks loose, so they count again at every pass until their cutoff. It can come later without changing a bound or a line.
- **Objects alone.** An occasion packs no ref, expires no ref's log and prunes no worktree. Those are each tool's own (core.14), and they stay bounded by what origin and the tools hold: a branch origin deletes is pruned at the fetch, and a place's branch, log and worktree go with the place. Not taken: option 3, whose `gc` packs refs, expires logs, prunes worktrees and collects `rerere`'s records, each to be turned off by a setting on the call.
- **At the pass, holding no lock.** The runner packs on its keeping loop, one clone after another, after every repository is kept, so no pack delays another repository's fetch. It reads its stop before each clone, and keeps a landing told while it packs between two packs. It takes no clone's lock to pack (record 0026), so a launch into the clone never waits on a pack: git's repack and prune are as safe beside a launch's writes as git's own collection was, since what a launch writes is younger than the cutoff. A pack runs to its end, as a git call that reaches no remote does (core.58). Not taken: option 4, a process on the host's scheduler, outside the runner's pass, which the runner would have to register in each clone and nothing would bound.
- **A failure is named, and the pass goes on.** Where `count-objects` or a step of an occasion fails, the clone is named, `<address>: not packed: <git's words>`, its later steps are not run, and the pass fails. Every other clone is still packed, and the next pass tries again, its occasion still there. A repack cut short leaves its temporary files, which a later prune removes once they are past the cutoff.
- **What is packed.** A clone of a repository the configuration keeps, every pass, `run --once`'s among them. A dropped repository's clone, held or drained, is left as it is, as its fetch is (the clones spec).

### Consequences

- No fetch, commit or tool's call in a clone starts git's collection, and a clone's objects stay within git's own bounds as before, packed at the runner's pass.
- A clone's refs, refs' logs and worktrees are no longer packed, expired or pruned by anything of git's own accord. sluice's protection keeps passing git's packing of refs where a person or another host's tool runs it.
- An occasion costs what the clone's history holds. On a large clone it can delay a landing told meanwhile, and a SIGTERM during it, until it ends. On the running runner's first keeping pass, whose end starts the control parts, it delays their first passes too. Clones were collected by git at the same bounds until now, so few are past them at any one pass.
- Packing is the runner's upkeep of its own clone, which git's commands change (core.34), and no tool's housekeeping run of the runner's accord (core.13).
- A host that registered a clone for `git maintenance` on its own scheduler, or a person who runs `git gc` in it, still collects it, and is no tool's of the family.
- core.14 is amended with this record, and core.13's details name packing as not ruled out. The change `the-runner-keeps-its-clones-packed` builds it, and the clones spec says it.
