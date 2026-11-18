# Clones

## ADDED Requirements

### Requirement: Git's own collection is off in every clone

The runner SHALL set `gc.auto` to `0` and `maintenance.auto` to `false` in each clone's own config: a new clone's before its first fetch, and a kept clone's at each keeping pass before its fetch, writing neither where the config already says so. A setting that cannot be written SHALL be named, `<address>: git's own collection not turned off: <git's words>`, and fail the pass, and the repository SHALL still be fetched (record 0051; core.14).

#### Scenario: A new clone's collection is off

- **WHEN** a pass clones `agentplot/flume`
- **THEN** the bare repository's own config gives `gc.auto` as `0` and `maintenance.auto` as `false`

#### Scenario: A clone made before is turned off at its next pass

- **WHEN** `agentplot/flume`'s clone's own config gives `gc.auto` as `6700` and no `maintenance.auto`
- **THEN** after the next pass its own config gives `gc.auto` as `0` and `maintenance.auto` as `false`, and the pass says nothing of it

#### Scenario: An agent's commit starts no collection

- **WHEN** a pass has kept `agentplot/flume`, whose clone then holds more than 6,700 loose objects, and a commit is made in a worktree of the clone
- **THEN** every loose object the clone held is still loose, the commit's among them, and the clone holds no pack it did not hold before

#### Scenario: A clone whose config cannot be written

- **WHEN** `agentplot/flume`'s clone has `gc.auto` set to `6700` and a `config.lock` beside its config, and origin's `main` gains a commit
- **THEN** a line reads `agentplot/flume: git's own collection not turned off: <git's words>`, the pass fails, and the clone's `refs/remotes/origin/main` is origin's new commit

### Requirement: A keeping pass packs a clone once what arrived passes git's bound

Once a keeping pass has kept and let go of every repository and kept the workspaces, it SHALL pack each clone of a repository the configuration keeps that holds more than 6,700 loose objects or more than 50 packs, as `git count-objects -v` counts them, saying `<address>: packed <n> loose objects and <m> packs`. A packed clone SHALL hold no loose object, at most two packs and a commit graph. Any other clone SHALL be left, and nothing said of it (record 0051).

#### Scenario: Nothing is packed at the bound

- **WHEN** a pass keeps `agentplot/flume`, whose clone holds 50 packs and fewer than 6,700 loose objects
- **THEN** after the pass the clone holds the same packs and loose objects, and the pass says nothing of it

#### Scenario: Loose objects past the bound

- **WHEN** a pass keeps `agentplot/flume`, whose clone holds `<n>` loose objects, more than 6,700, and `<m>` packs
- **THEN** the pass says `agentplot/flume: packed <n> loose objects and <m> packs`, and after it the clone holds no loose object, at most two packs and `objects/info/commit-graph`, and every object it held is still there

#### Scenario: Packs past the bound

- **WHEN** a pass keeps `agentplot/flume`, whose clone holds 51 packs and `<n>` loose objects, fewer than 6,700
- **THEN** the pass says `agentplot/flume: packed <n> loose objects and 51 packs`, and after it the clone holds no loose object and at most two packs

#### Scenario: A dropped repository is not packed

- **WHEN** `agentplot/flume` is dropped from the configuration while its clone's branch `wip` holds a commit origin's refs do not reach, and the clone holds more than 6,700 loose objects
- **THEN** after the next pass the clone holds the same loose objects and packs, and the pass says nothing of packing it

### Requirement: A pack drops only what nothing of the clone reaches, past two weeks

A pack SHALL drop an object only where no ref, ref's log, or worktree's HEAD or index of the clone reaches it, and it was written more than two weeks before, git's own `gc.pruneExpire`. Every other object SHALL stay in the clone (record 0051; core.14).

#### Scenario: An old object nothing reaches is dropped

- **WHEN** a pass packs `agentplot/flume`, whose clone holds a blob nothing reaches written three weeks before, and another nothing reaches written that day
- **THEN** after the pass the first blob is gone from the clone, and the second is still there

#### Scenario: What a place and sluice hold is kept

- **WHEN** a pass packs `agentplot/flume`, while a worktree of its clone holds a blob in its index alone and a commit on its own branch, and the clone's `refs/sluice/heads/x` names a commit origin lacks, each object of them written three weeks before
- **THEN** after the pass each of those objects is still in the clone

### Requirement: A pack touches no ref, log or worktree

A pack SHALL change nothing of the clone but its objects and its commit graph: it SHALL pack no ref into `packed-refs`, expire no ref's log and prune no worktree (record 0051; core.14).

#### Scenario: Refs, logs and worktrees are left as they were

- **WHEN** a pass packs `agentplot/flume`, whose clone has `refs/sluice/heads/x` as a ref of its own file with a log whose one entry is a year old, no `packed-refs`, and a worktree whose folder has been removed
- **THEN** after the pass `refs/sluice/heads/x` is still a file of its own and its log as it was, the clone has no `packed-refs`, and `git worktree list` still names the worktree

### Requirement: Packing reads the runner's stop and lets a landing in

The runner SHALL read its stop before each clone it would pack, and pack none once it is set. It SHALL keep a landing told while the pass packs between two packs. A pack in flight SHALL run to its end, as a git call that reaches no remote does (record 0051; core.58).

#### Scenario: A landing told while the pass packs

- **WHEN** a pass packs `agentplot/flume`, then `agentplot/sluice`, each past the bound, and the runner is told of a landing on `agentplot/timber`, origin's `main` having moved, while it packs `agentplot/flume`
- **THEN** `agentplot/timber`'s view is moved to the new `origin/main` after `agentplot/flume` is said packed and before `agentplot/sluice` is

#### Scenario: A stop while the pass packs

- **WHEN** a pass would pack `agentplot/flume`, then `agentplot/sluice`, each past the bound, and the runner is told to stop while it packs `agentplot/flume`
- **THEN** `agentplot/flume` is said packed, and `agentplot/sluice`'s clone holds the same loose objects and packs as before the pass

### Requirement: A pack that fails is named, and the pass goes on

Where counting a clone's objects or packing it fails, the runner SHALL name it, `<address>: not packed: <git's words>`, run no later step of its pack, and fail the pass. It SHALL still pack every other clone past the bound, and the next pass SHALL try the clone again (record 0051).

#### Scenario: One pack fails

- **WHEN** a pass would pack `agentplot/flume` and `agentplot/sluice`, each past the bound, and no file can be made in `agentplot/flume`'s `objects/pack`
- **THEN** a line reads `agentplot/flume: not packed: <git's words>`, the pass fails, `agentplot/sluice` is said packed, and once a file can be made there again the next pass packs `agentplot/flume`
