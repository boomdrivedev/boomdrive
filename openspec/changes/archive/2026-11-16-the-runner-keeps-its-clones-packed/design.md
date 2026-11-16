# Design

## Context

Record 0051 decides that the runner alone packs and prunes a clone it keeps, with git's own collection off in it, and core.14 as amended says so, with core.13's details naming packing as no tool's housekeeping. Both are in this change's first commit and carry the construct stage's choices below. Rules relied on: core.13, core.14, core.17, core.34, core.35, core.40, core.49 and core.58. No rule is departed from. The design reaches no control plane (core.2).

`crates/boomdrive/src/clone.rs` makes a clone in `make_in`: `git init --bare`, then the clone's own keys, `remote.origin.url`, `remote.origin.fetch`, `remote.origin.followRemoteHEAD` and `boomdrive.address`, each by `git config`, then its first fetch. `crates/boomdrive/src/pass.rs` runs a keeping pass in `pass`: it keeps each repository the configuration names under the clone's lock (`keep_configured`, then `keep`, which fetches a found clone, settles its HEAD and keeps its view), calling `between` after each to keep a landing told meanwhile, then lets go of each dropped repository, keeps the workspaces and clears partial clones. It reads `stop` before each repository. `Runner::keep` in `run.rs` runs `pass` on the keeping loop, and `Runner::once` runs it for `run --once`. Every git call that reaches no remote goes through `Git::run` and runs to its end (core.58).

The family: Flume runs `git worktree prune` on a place's repository, and sluice in its checks' worktrees, and neither packs or prunes objects. sluice's protection lets git's packing of refs through (sluice record 0110). timber's test fixture turns `maintenance.auto` off in its own scratch repositories. So the runner is the one owner, and no other tool's design changes.

## Goals / Non-Goals

**Goals:**

- No fetch, commit or tool's call in a clone, or in a worktree of it, starts git's collection.
- A clone's objects stay within git's own bounds, packed at the runner's pass.
- Nothing a clone reaches is lost, and nothing another tool keeps there but its objects' storage is changed (core.14).
- No launch waits on a pack, and no repository's fetch waits on another's pack.

**Non-Goals:**

- No packing of refs, expiry of refs' logs or pruning of worktrees: each is its own tool's.
- No key of the configuration: the bounds and the cutoff are git's own defaults, fixed.
- No change to a dropped repository's clone, which is neither fetched nor packed.
- Nothing about a host's own `git maintenance` registration or a person's `git gc`, which are neither the runner's nor the family's.

## Decisions

### Decision 1: Git's collection is off by two keys of the clone's own config

The clone's own config gives `gc.auto` as `0` and `maintenance.auto` as `false`. `gc.auto` at `0` makes `git gc --auto` do nothing, whichever command runs it, and `git maintenance run --auto`'s `gc` task with it. `maintenance.auto` at `false` makes `git maintenance run --auto`, which a fetch, a commit and other commands run after their work, do nothing. On a scratch clone holding 7,003 loose objects, a commit in a worktree packed them all with neither key set, and none with both. Every worktree of the clone, the view and each place, reads the same config, so both hold in them. Local config wins over the host's global config.

`make_in` sets both with the clone's other keys, before its first fetch. A new function of `clone.rs` turns them off in a kept clone: it reads each with `git config --local --type=int --get gc.auto` and `git config --local --type=bool --get maintenance.auto`, and writes each, by `git config --local`, only where it is missing or reads otherwise, so `no` reads as `false` and a steady pass writes no config. A clone's config written each pass would take its `config.lock` each pass, and a place's own `git config` could then fail.

`keep` in `pass.rs` calls it for a found clone, after its drive is recorded and before its fetch, since the fetch is the first command of the pass that would start a collection. Where a write fails, the pass says `<address>: git's own collection not turned off: <git's words>`, the outcome is failed, and the keep goes on: the fetch, the HEAD and the view as before. Not taken: skipping the fetch, since a fetch with git's collection on is how every clone was kept until now, while a repository left unfetched is a view left behind.

Not taken: `gc.autoPackLimit` at `0` as well, which `gc.auto` at `0` already makes moot; and `extensions.worktreeConfig`, which a place could use to set its own `gc.auto`, since no tool of the family does.

### Decision 2: An occasion comes past git's own bounds

`clone.rs` reads a clone's counts by `git count-objects -v`, its `count`, the loose objects, and its `packs`. A clone's occasion has come where it holds more than 6,700 loose objects or more than 50 packs, git's own defaults for `gc.auto` and `gc.autoPackLimit`: the bounds at which git's collection would have started. The decision is one pure function of the counts, with the two bounds as constants beside it, unit tested at and past each bound.

`count-objects` counts every loose object exactly, reading each of the 256 folders of `objects/`, which costs little next to a fetch. After an occasion the clone holds no loose object and at most two packs (Decision 3), so what the next occasion counts is what arrived since: loose objects from small fetches, under git's `fetch.unpackLimit` of 100, and from commits in places, and one pack for each larger fetch.

The bounds are fixed, with no key of the configuration (core.17), as core.58's limit has none.

### Decision 3: An occasion packs the clone whole, in three of git's commands

At an occasion `clone.rs` runs, by `Git::run`, in the bare repository:

1. `git repack -q -d -l --cruft --cruft-expiration=2.weeks.ago`. What the clone reaches goes into one pack, with its bitmap, which git writes by default in a bare repository. What it does not reach, but was written within two weeks, goes into a cruft pack, which records each object's time. The packs and loose objects the new ones hold are removed.
2. `git prune --expire=2.weeks.ago`. The repack leaves loose the objects nothing reaches that were written more than two weeks before, and prune drops them. It also removes a cut-short repack's temporary files once they are past the cutoff.
3. `git commit-graph write --reachable`, which git's collection wrote too, so walks of the clone's history stay fast.

What the clone reaches is git's own reckoning, as `git gc` has it: every ref, every ref's log, and each worktree's HEAD and index. Two weeks is git's own `gc.pruneExpire`. Each step runs to its end (core.58), and the first that fails ends the pack (Decision 6).

The pass then says `<address>: packed <n> loose objects and <m> packs`, the counts read before the pack, with `1 loose object` and `1 pack` for one. A clone below both bounds is said nothing of.

Not taken: an occasion that packs the loose objects alone, `git repack -d`, and consolidates only past the pack bound, as flywheel-next's model did. It costs less per occasion on a large clone, but leaves loose the unreachable objects of the last two weeks, so a clone that holds more than 6,700 of them comes to an occasion at every pass until their cutoff. A cruft pack holds them, so after every occasion nothing is loose. The split can come later without changing a bound or a line.

Not taken: `git gc --auto` with git's collection forced on for the one call, which packs refs, expires refs' logs, prunes worktrees and collects `rerere`'s records, each to be turned off by a setting on the call (core.14).

### Decision 4: A pack touches objects alone

None of the three commands packs a ref, expires a ref's log or prunes a worktree. Loose refs stay bounded by what origin and the tools hold: the fetch prunes a branch origin deleted, and a place's branch, log and worktree go with the place. sluice's protection, which lets git's packing of refs through, is unchanged, and never asked.

### Decision 5: The pass packs last, holding no lock

`pass` packs once it has kept and let go of every repository, kept the workspaces and cleared partial clones, so no pack delays another repository's fetch, view or workspace. It goes through the repositories the configuration keeps, in the configuration's order, each with a clone among the pass's clones. A dropped repository's clone is not packed, held or drained, as it is not fetched. Where the local layer was not taken, a repository only it names is not kept (core.17), and so not packed.

Before each clone, `pass` reads `stop`, and packs none once it is set. After each clone it packs, it calls `between`, so a landing told while it packs is kept between two packs, and finds the clones again where `between` kept one, as the keeping round does. A pack in flight runs to its end: it reaches no remote (core.58), and a stop waits for it.

A pack takes no clone's lock. A launch into the clone holds the lock shared, and the keeper alone while it keeps, and no other command takes it (record 0026). git's repack and prune are as safe beside a launch's writes as git's own collection was, beside the same writes, since what a launch writes is younger than the cutoff. The packs run on the keeping loop's thread, as every keep and every landing's keep, so no keep or let go of the same clone runs beside its pack.

`run --once`'s pass packs as any. The running runner's first keeping pass packs too, so its packs delay the parts' first control passes, which start at its end: clones were collected at the same bounds by git until now, so few are past them at any one pass. Not taken: no packing on the first pass, which would leave `run --once` never packing.

### Decision 6: A pack that fails is named

Where `count-objects` or a step of the pack fails, the pass says `<address>: not packed: <git's words>`, runs no later step, and its outcome is failed. Every other clone is still packed. The next pass reads the counts again and packs the clone, its occasion still there. A repack cut short, by a failure or a kill, leaves its temporary files in `objects/pack`, which git reads past, and which a later prune removes once they are past the cutoff.

## Risks / Trade-offs

- **An occasion costs what the clone's history holds.** On a large clone a whole repack takes from seconds to minutes. A landing told meanwhile, and a SIGTERM, wait for it, and on the running runner's first pass the parts' first control passes wait too. Mitigation: occasions come at git's own bounds, as often as git's collection came, and a repack reuses the deltas its packs hold.
- **A host that runs `git maintenance` on its own scheduler for a clone, or a person's `git gc`, still collects it.** Neither is a tool of the family's, and either is as safe as before.
- **A place that sets its own `gc.auto` in a worktree's own config** collects in that worktree. No tool of the family does.
- **Objects a person staged and unstaged, or a commit amended away, are dropped two weeks after they were written**, as git's own collection dropped them.

## Migration Plan

None by hand. At its first keeping pass after the upgrade, the runner turns git's collection off in every kept clone, saying nothing, and packs each clone past the bound. A clone whose config cannot be written is named at every pass until it can be.
