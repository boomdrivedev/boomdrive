# clones Specification

## Purpose
How the runner keeps each repository its configuration names: one bare clone in its own data directory, hidden from the user. The clone is fetched over https through the host's credential helper, on the runner's timer and when it is told of a landing, and its own default branch is kept at origin's. This spec also covers how the runner lets go of a repository the configuration drops (crew record 0052 part 2; swancloud's `names.3` and `herdr.13` as they move to the runner; core.5).

## Requirements

### Requirement: Each kept repository has one bare clone in the runner's data directory

The runner SHALL keep each configured repository in the bare-container layout under its data directory. Its data directory is `$XDG_DATA_HOME/boomdrive`, or `~/.local/share/boomdrive` where `XDG_DATA_HOME` is unset or empty. The container is `<data>/repositories/<owner>/<name>/`. It holds `<name>.git`, a bare repository whose origin is `https://github.com/<owner>/<name>.git`, and `.git`, a gitfile naming `./<name>.git`. Its places are left to Flume.

#### Scenario: A new repository is cloned

- **WHEN** a pass runs with `agentplot/flume` configured and no clone of it
- **THEN** `<data>/repositories/agentplot/flume/flume.git` is a bare repository whose `remote.origin.url` is `https://github.com/agentplot/flume.git`, holding every branch of origin as `refs/remotes/origin/*`, and `<data>/repositories/agentplot/flume/.git` reads `gitdir: ./flume.git`

#### Scenario: The clone is not under the views root

- **WHEN** a pass has cloned `agentplot/flume` with the repository root `~/Code`
- **THEN** nothing of the clone's is under `~/Code` but the view

### Requirement: A clone that fails leaves nothing

A clone SHALL be made in a folder of its own beside its container and moved into place only once its first fetch has succeeded. A clone that fails SHALL leave nothing in the data directory. It SHALL be named with git's words and tried again at the next pass. Every other repository SHALL still be kept in that pass.

#### Scenario: Origin cannot be reached

- **WHEN** a pass clones `agentplot/flume` and origin answers no fetch
- **THEN** no folder of `agentplot/flume`'s is left in the data directory, the failure is named, every other configured repository is kept in the same pass, and the next pass tries the clone again

### Requirement: A folder the runner did not make is never touched

The runner SHALL know a clone as its own by the address it recorded in the clone when it made it, under `boomdrive.address` and no other key (record 0035). A folder at a container's path that is not a clone the runner made SHALL be left untouched and named, and no clone SHALL be made over it. A folder in the data directory that is not a clone the runner made SHALL never be fetched or removed.

#### Scenario: A folder already there

- **WHEN** `<data>/repositories/agentplot/flume` exists, holding a folder the runner did not make, and `agentplot/flume` is configured
- **THEN** the pass leaves the folder as it was, names it, and makes no clone and no view of `agentplot/flume`

#### Scenario: A stranger in the data directory

- **WHEN** `<data>/repositories/someone/else` is a folder the runner did not make and no configured address is `someone/else`
- **THEN** the pass neither fetches nor removes it

#### Scenario: A clone recording its address under another name

- **WHEN** `<data>/repositories/agentplot/flume` holds a bare repository `flume.git` whose own git config records `other.address` as `agentplot/flume` and no `boomdrive.address`, and `agentplot/flume` is configured
- **THEN** the pass names the container as not a clone the runner made, makes no clone and no view of `agentplot/flume`, and the bare repository's config still records `other.address` and no `boomdrive.address`

### Requirement: A clone is fetched over https and never asks

Every fetch SHALL reach origin over https, with git never prompting. Where the configuration names no `[forge]`, a fetch SHALL go through the host's git credential helper, with no credential of the runner's own. A fetch whose URL the host's git configuration rewrites to ssh SHALL not be run: the repository SHALL be named with the URL it would have used, so a fetch never waits on an ssh agent or on someone at the keyboard (core.34).

#### Scenario: Git asks no one

- **WHEN** the runner fetches a clone, its configuration naming no `[forge]`
- **THEN** git runs with `GIT_TERMINAL_PROMPT=0` and is given no credential, so the host's credential helper answers or the fetch fails without asking

#### Scenario: The host rewrites https to ssh

- **WHEN** the host's git configuration says `url."git@github.com:".insteadOf = "https://github.com/"` and a pass keeps `agentplot/flume`
- **THEN** no fetch of `agentplot/flume` is run, and it is named with `git@github.com:agentplot/flume.git` as the URL git would have used

### Requirement: Every kept repository is fetched on the timer

At every pass, the runner SHALL fetch each kept repository from origin, pruning the branches origin no longer has. A fetch that fails SHALL be named with git's words, and every other repository SHALL still be fetched.

#### Scenario: Origin gains a commit

- **WHEN** origin's `main` of a kept repository gains a commit
- **THEN** after the next pass the clone's `refs/remotes/origin/main` is that commit

#### Scenario: Origin deletes a branch

- **WHEN** origin deletes the branch `fix/one` of a kept repository
- **THEN** after the next pass the clone has no `refs/remotes/origin/fix/one`

#### Scenario: One fetch fails

- **WHEN** a pass keeps `agentplot/flume` and `agentplot/sluice` and the fetch of `agentplot/flume` fails
- **THEN** that failure is named with git's words, and `agentplot/sluice` is fetched in the same pass

### Requirement: A repository is fetched when the runner is told of its landing

When the running runner is told that a kept repository has had a landing, it SHALL fetch that repository, settle its HEAD and keep its view at once, under its lock, without waiting for the next pass and without fetching any other repository. Told during a keeping pass, it SHALL keep the landing before the pass's next repository. A landing told of a repository the configuration does not name SHALL clone and fetch nothing, and SHALL be named.

#### Scenario: Told between passes

- **WHEN** the runner keeps `agentplot/flume` and `agentplot/sluice`, fetches every hour, and is told of a landing on `agentplot/flume` a minute after a pass, origin's `main` having moved
- **THEN** within seconds `agentplot/flume` is fetched and its view moved to the new `origin/main`, and `agentplot/sluice` is not fetched

#### Scenario: Told during a keeping pass

- **WHEN** a keeping pass keeps `agentplot/flume`, then `agentplot/sluice`, then `agentplot/timber`, and the runner is told of a landing on `agentplot/timber` while the pass keeps `agentplot/flume`
- **THEN** `agentplot/timber`'s landing is kept after `agentplot/flume` and before `agentplot/sluice`

#### Scenario: Told of a repository it does not keep

- **WHEN** the runner is told of a landing on `someone/else`, which its configuration does not name
- **THEN** nothing is cloned or fetched, and the notice is named as for a repository the configuration does not name

### Requirement: The default branch is origin's

A clone's default branch SHALL be the branch origin names as its HEAD. Every fetch SHALL keep the clone's `refs/remotes/origin/HEAD` current, and the clone's own HEAD SHALL name the same branch. Where the clone lacks that branch, the runner SHALL make it at origin's tip, so HEAD names a commit. A repository whose origin names no default branch, or has none, SHALL be named. It gets no view until origin has one and is tried again at the next pass.

#### Scenario: Origin's default is not main

- **WHEN** origin's HEAD names `trunk`
- **THEN** the clone's HEAD names `refs/heads/trunk`, the clone's `trunk` is made at `origin/trunk`, and its view is detached at `origin/trunk`

#### Scenario: An empty origin

- **WHEN** origin of a configured repository has no branch
- **THEN** the pass names it as having no default branch, makes no view, and the pass after origin gains one makes the view

### Requirement: A dropped repository is no longer fetched

A clone the runner made whose address the configuration no longer names SHALL never be fetched again, by the timer or by a landing told of it.

#### Scenario: Dropped while holding work

- **WHEN** `agentplot/flume` is dropped from the configuration while its clone holds work the forge lacks, and origin's `main` then gains a commit
- **THEN** after the next pass the clone's `refs/remotes/origin/main` has not moved

### Requirement: A dropped repository is removed once it holds nothing the forge lacks

At every pass, a dropped repository's clone and view SHALL be removed, the view first and then the container, once neither holds work the forge lacks and the runner drains the repository no more. Until then both SHALL be left untouched and named with each thing that holds them.

#### Scenario: Dropped with nothing of its own

- **WHEN** `agentplot/flume` is dropped from the configuration, its view clean and detached at `origin/main`, its clone holding only what origin has
- **THEN** after the next pass its view's folder and its container are gone, and it is named as removed

#### Scenario: Emptied later

- **WHEN** a dropped repository is held only by its branch `wip`, and `wip` is then deleted
- **THEN** the next pass removes its view and its container

#### Scenario: The view's folder is not the view any more

- **WHEN** `agentplot/flume` is dropped and its view's folder has been replaced by a folder that is not a worktree of the clone
- **THEN** the folder is left untouched, and the container is removed once the clone holds nothing the forge lacks

### Requirement: Work the forge lacks is what the runner would lose

A clone or view SHALL hold work the forge lacks when it has any of these. A commit on any ref of the clone that origin's refs, as last fetched, do not reach; origin's own refs and the runner's are not counted. A worktree of the clone other than its view. In the view, an uncommitted or untracked change, or a HEAD that origin's refs do not reach. Files git ignores SHALL not count.

#### Scenario: A branch the forge lacks

- **WHEN** `agentplot/flume` is dropped while the clone's branch `wip` holds a commit origin's refs do not reach
- **THEN** its clone and view are left as they were and named as held by `wip`, at every pass until `wip` is gone or reached

#### Scenario: A change in the view

- **WHEN** `agentplot/flume` is dropped while its view holds an uncommitted change to a tracked file, or an untracked file
- **THEN** its clone and view are left as they were and named as held by the view's changes

#### Scenario: A commit in the view

- **WHEN** `agentplot/flume` is dropped while its view's detached HEAD is a commit made there that origin's refs do not reach
- **THEN** its clone and view are left as they were and named as held by that commit

#### Scenario: A place in the clone

- **WHEN** `agentplot/flume` is dropped while a worktree of its clone other than its view exists, a Flume place under `places/` among them
- **THEN** its clone and view are left as they were and named as held by that worktree

#### Scenario: Only ignored files

- **WHEN** `agentplot/flume` is dropped while its view holds only files its `.gitignore` ignores
- **THEN** after the next pass its view and its container are gone

### Requirement: The runner moves no branch of the clone

The runner SHALL read every branch of a kept repository at origin's tracking ref, `refs/remotes/origin/<branch>`, as its last fetch left it. It SHALL NOT move or delete any branch of the clone, its default branch among them, whether or not sluice keeps it: the clone's own ref of a branch sluice keeps is sluice's copy, which sluice levels. A pass SHALL say nothing of the clone's own branches.

#### Scenario: Behind origin

- **WHEN** origin's `main` moves two commits ahead of the clone's `main`
- **THEN** after the next pass the clone's `main` is where it was, the pass says nothing of it, and the view is moved to `origin/main`

#### Scenario: Sluice keeps the branch

- **WHEN** the clone's `main` is guarded as sluice's protection guards a branch it keeps, refusing any write that moves it, and origin's `main` moves ahead of it
- **THEN** the pass writes nothing to the clone's `main`, so nothing is refused, it fails nothing, and the view is moved to `origin/main`

#### Scenario: Ahead of origin

- **WHEN** the clone's `main` holds a commit origin's `main` lacks, and origin's `main` moves on
- **THEN** the clone's `main` is left where it is, the pass says nothing of it, and the view is moved to `origin/main`

#### Scenario: A new repository root while sluice keeps the branch

- **WHEN** the runner's clone has its `main` guarded as sluice guards a branch it keeps and behind origin's, its view is clean under the old repository root, and the repository root changes
- **THEN** the pass removes the old view, makes the view under the new root at `origin/main`, writes nothing to the clone's `main`, and fails nothing

### Requirement: A HEAD that cannot be settled leaves the view kept

Where the runner cannot point the clone's HEAD at origin's default branch, or cannot make that branch where the clone lacks it, it SHALL name the repository with git's words, and the pass SHALL fail. It SHALL still keep the repository's view at `origin/<default>`, and every repository after it in the same pass.

#### Scenario: A new default the clone refuses

- **WHEN** origin's HEAD moves to a new branch `trunk`, and the clone refuses any write that makes its own `trunk`
- **THEN** the pass names `HEAD not settled on trunk` with git's words and fails, and the view is moved to `origin/trunk`

### Requirement: A clone's keep and a launch into it take turns on the clone's lock

The runner SHALL hold one lock for each repository it keeps, its address compared without case. The keeper SHALL hold it alone while it keeps or lets go of the repository, as SHALL a landing's keep. A launch into the clone SHALL hold it shared with other launches, from finding the clone made to the end of `flume launch`. No other command or read SHALL take it, and none SHALL be held across a pass (record 0026; core.14).

#### Scenario: A launch waits for its own repository's fetch alone

- **WHEN** the keeper holds `agentplot/flume`'s lock for its fetch, and a control pass offers one launch into `agentplot/flume` and one into `agentplot/sluice`
- **THEN** the launch into `agentplot/sluice` is made at once, and the one into `agentplot/flume` once the fetch is done

#### Scenario: Two parts launch into one clone at once

- **WHEN** `home`'s pass is launching into `agentplot/flume`, and `work`'s pass offers a launch into `agentplot/flume`
- **THEN** `work`'s launch is made without waiting for `home`'s to end

### Requirement: A launch finds its clone again under the lock

Holding a clone's lock, a control pass SHALL find the clone again before it launches into it, and SHALL leave an offer whose clone is no longer there unanswered, as one whose clone is not yet made (record 0021; core.14).

#### Scenario: A clone let go before the launch

- **WHEN** a control pass finds `agentplot/flume`'s clone made, and the keeper removes it, as dropped, before the pass takes its lock
- **THEN** the offer into `agentplot/flume` is left unanswered, and no `flume launch` is run for it

### Requirement: A dropped repository is drained while a place the runner launched there has not ended

The runner SHALL drain a dropped repository while its report holds a launch answered `launched` at a place, naming that repository as the one it was answered in, whose place has not ended. While it drains a repository, its clone and view SHALL be kept as they are, neither fetched nor moved, and named at every pass as held by each such place, with its launch.

#### Scenario: Dropped while a place is live

- **WHEN** `agentplot/flume` is dropped holding nothing the forge lacks, and the report holds launch `a` launched at `P` in `agentplot/flume`, whose carried status reads `P` live
- **THEN** after the next pass its view and its container are as they were, and it is named as held by `P` of `a`

### Requirement: A place has ended once the status carried of it says Flume holds it no more

A place SHALL have ended once the status the report carries of it stands not listed, or reads the place's phase `tidied` or `gone` and does not name `boomdrive` among those its record is pending for. A place with no status carried SHALL not have ended. So a place whose record Flume holds for the runner SHALL not have ended until the runner has confirmed the ends Flume owes it and carried the status that follows (record 0030 as amended on 2026-11-09).

#### Scenario: The place tidied

- **WHEN** the runner drains `agentplot/flume` for `P` alone, and the status the report carries of `P` then reads its record `P@<at>` tidied and landed
- **THEN** the next pass removes its view and its container

#### Scenario: The place no longer listed

- **WHEN** the runner drains `agentplot/flume` for `P` alone, and the status the report carries of `P` then stands not listed
- **THEN** the next pass removes its view and its container

#### Scenario: The place's end held for the runner

- **WHEN** the runner drains `agentplot/flume` for `P` alone, and the status the report carries of `P` reads its record `P@<at>` tidied and landed, pending for `boomdrive`
- **THEN** the next pass keeps its view and its container, named as held by `P`; and once the carried status reads `P@<at>` pending for `boomdrive` no more, the pass after removes them

### Requirement: A report file that cannot be read lets no dropped repository go

Where there is no report file, no repository SHALL be drained. Where the report file cannot be read, no dropped repository SHALL be removed that pass, and each SHALL be named with the report file's fault.

#### Scenario: A report that cannot be read

- **WHEN** `agentplot/flume` is dropped holding nothing the forge lacks, and the report file does not read as a report
- **THEN** after the next pass its view and its container are as they were, and it is named with the report file's path

### Requirement: The data directory is Boomdrive's alone

The runner's data directory SHALL be `$XDG_DATA_HOME/boomdrive`, or `~/.local/share/boomdrive` where `XDG_DATA_HOME` is unset or empty, and no other folder, whatever else is beside it. The runner SHALL read no folder on disk to choose it (record 0035; core.35, core.36).

#### Scenario: The data directory is the runner's own

- **WHEN** the runner starts with `XDG_DATA_HOME` set to a folder that holds a folder `other`, itself holding `repositories/` and `control/report.json`, and no folder `boomdrive`
- **THEN** its data directory is `boomdrive` in that folder, and it still is once that folder is made

### Requirement: No try of a fetch outlasts its limit

Every fetch of a clone, a pass's, a landing's and a new clone's first, SHALL run in a process group of its own and SHALL be stopped at its limit, thirty seconds, the limit agentplot/sluice record 0109 sets for every try of a call to a remote. At the limit its group SHALL be asked to end, given two seconds, then killed, so nothing it started outlives it (record 0042; core.58).

#### Scenario: What a call started ends with it

- **WHEN** a call is run to its limit, and starts a process that starts another, neither of which ever ends
- **THEN** at its limit both are ended, the call is said stopped, and what each pipe gave until then is kept

#### Scenario: A call asked to end is given its grace

- **WHEN** a call run to its limit, asked to end at its limit, says so and exits
- **THEN** what it said as it ended is kept, and the call is said stopped

#### Scenario: A call that will not end is killed

- **WHEN** a call run to its limit ignores being asked to end
- **THEN** it is killed once its grace has passed, and the call is said stopped

#### Scenario: A call that ends within its limit

- **WHEN** a call run to its limit exits by itself before it
- **THEN** its exit and everything it printed are given, and the call is not said stopped

#### Scenario: The limit is thirty seconds

- **WHEN** the runner fetches a clone with the git it runs on a host
- **THEN** the fetch's limit is thirty seconds and its grace two

### Requirement: A fetch stopped at its limit is named, and the pass goes on

A fetch stopped at its limit SHALL fail with the words `git fetch with origin was still going after <limit>, the most a try may take, and was stopped`, after git's last line on standard error where git wrote one. Its repository SHALL be named not fetched, or not cloned with nothing left in the data directory, as for any failed fetch. Every other repository SHALL still be kept in that pass, and the next pass SHALL fetch it again (record 0042).

#### Scenario: A fetch whose origin never answers

- **WHEN** a pass keeps `agentplot/flume` and `agentplot/sluice`, both cloned, and `agentplot/flume`'s origin takes the connection and never answers
- **THEN** the pass ends within its limit and grace and a margin; a line reads `agentplot/flume: not fetched: git fetch with origin was still going after <limit>, the most a try may take, and was stopped`; and `agentplot/sluice` is fetched and its view moved in the same pass

#### Scenario: A clone whose origin never answers

- **WHEN** a pass keeps `agentplot/flume`, with no clone of it, and `agentplot/sluice`, and `agentplot/flume`'s origin takes the connection and never answers
- **THEN** the pass ends within its limit and grace and a margin; a line reads `agentplot/flume: not cloned: git fetch with origin was still going after <limit>, the most a try may take, and was stopped`; no folder of `agentplot/flume`'s is left in the data directory; and `agentplot/sluice` is kept in the same pass

### Requirement: A fetch goes through the App's token where the configuration names the App

Where the configuration names `[forge]`, a fetch of a served repository, a new clone's first among them, SHALL go through the App's token alone. The call SHALL empty the host's helpers, and its one helper SHALL read the token the runner keeps for the repository from that call's environment, so the token is on no command line (record 0007 as amended on 2026-11-11; core.33, core.34).

#### Scenario: Git is given the App's token alone

- **WHEN** a call to a remote is given the App's token `t`, and the host's git configuration names a helper of its own that answers with another password
- **THEN** git is given `x-access-token` and `t` by the call's own helper, the host's helper is not asked, and no argument of the call holds `t`

### Requirement: A fetch with no token goes with no credential

Where the App is not installed on the repository, or no token could be minted, the fetch SHALL go with the host's helpers emptied and no credential. Where such a fetch fails after a refused mint, the line SHALL give git's words, then `; no forge token: ` and the mint's words (record 0007 as amended on 2026-11-11; core.33).

#### Scenario: Git is given no credential

- **WHEN** a call to a remote is given no credential, and the host's git configuration names a helper of its own
- **THEN** git is given no credential, and the host's helper is not asked

#### Scenario: A repository the App is not installed on

- **WHEN** a pass keeps `agentplot/flume`, its configuration naming `[forge]`, GitHub has no installation of the App on it, and its origin needs no credential
- **THEN** it is fetched and its view moved, and no token is bought

#### Scenario: A mint refused is named after git's words

- **WHEN** a pass keeps `agentplot/flume`, its configuration naming `[forge]` whose `key_file` does not exist, and the fetch of `agentplot/flume` fails
- **THEN** a line reads `agentplot/flume: not fetched: <git's words>; no forge token: [forge] key_file <path> cannot be read: <why>`

### Requirement: A stop of the runner ends its fetch in flight

Every fetch of a clone, a pass's, a landing's and a new clone's first, SHALL also end at the runner's stop, through the runner's halt, `weir-bounded`'s, which the runner's git carries. A fetch in flight when the halt is set SHALL have its group asked to end, given two seconds, then killed, as at its limit, so nothing it started outlives it. A fetch begun once the halt is set SHALL run no git (record 0042 as amended on 2026-11-12; core.58).

#### Scenario: A fetch in flight at the stop

- **WHEN** a fetch whose remote never answers runs with a limit of sixty seconds, and its git's halt is set 200 ms in
- **THEN** it fails within five seconds, its limit not reached

#### Scenario: A fetch begun after the stop

- **WHEN** a fetch begins with its git's halt already set
- **THEN** it fails at once and runs no git

#### Scenario: The runner's git carries one halt

- **WHEN** a git is made by `Git::default()`, and another by `halted_by` a halt
- **THEN** the first's halt is never set, and the second's is that halt, set when it is set; `trying_for` keeps a git's halt

### Requirement: A fetch ended at the stop fails with a halted call's words

A fetch ended at the runner's stop, or begun after it, SHALL fail with `weir-bounded`'s words for a halted call, `git fetch with origin was stopped as its caller asked`, after git's last line on standard error and `; ` where git wrote one, and never with the limit's words. Its standard output SHALL not be read as an answer (record 0042 as amended on 2026-11-12).

#### Scenario: A halted fetch's words

- **WHEN** a fetch writes `fatal: half` on standard error and is halted before it ends
- **THEN** it fails with `fatal: half; git fetch with origin was stopped as its caller asked`

#### Scenario: A fetch begun after the stop says so alone

- **WHEN** a fetch begins with its git's halt already set
- **THEN** it fails with `git fetch with origin was stopped as its caller asked` alone

### Requirement: A fetch ended at the stop is named as any failed fetch

A repository whose fetch was ended at the runner's stop, or begun after it, SHALL be named not fetched, or not cloned with nothing left in the data directory, with the halted words, as for any failed fetch, and the runner's next start SHALL fetch it again (record 0042 as amended on 2026-11-12).

#### Scenario: A new clone halted in flight

- **WHEN** the runner is stopped while its first fetch of `agentplot/flume`, with no clone of it, is in flight
- **THEN** a line reads `agentplot/flume: not cloned: git fetch with origin was stopped as its caller asked`, and no folder of `agentplot/flume`'s is left in the data directory

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

### Requirement: A fetch on a cloud host goes through the control plane's token

Where the configuration's control plane holds the App, the running runner SHALL open its forge tokens at start, and a fetch of a served repository, a new clone's first among them, SHALL go through the token it gives, by the call's own helper. Where it gives none, the fetch SHALL go with no credential, and where such a fetch fails after a refused ask, its line SHALL give git's words, then `; no forge token: ` and the ask's words (record 0053; core.33, core.34).

#### Scenario: A cloud host's fetch given the control plane's token

- **WHEN** a pass keeps `agentplot/flume` on a cloud host, its control plane giving a token for it, and its origin needs no credential
- **THEN** it is fetched and its view moved, and the control plane was asked for `agentplot/flume`'s token

#### Scenario: A repository the App's installation does not cover

- **WHEN** a pass keeps `agentplot/flume` on a cloud host, its control plane saying the App's installation does not cover it, and its origin needs no credential
- **THEN** it is fetched and its view moved, and no line names a forge token

#### Scenario: The running runner asks its control plane

- **WHEN** `boomdrive run --once` keeps `agentplot/flume` on a scratch host whose configuration names the hosted control plane at a loopback port with nothing on it, its partition `home` served `agentplot/flume` with no key of `home` in the host's store, and the fetch of `agentplot/flume` fails
- **THEN** a line reads `agentplot/flume: not fetched: <git's words>; no forge token: ` and words naming `boomdrive enroll home`
