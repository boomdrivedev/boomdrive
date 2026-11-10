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

Every fetch SHALL reach origin over https, through the host's git credential helper, with no credential of the runner's own and with git never prompting. A fetch whose URL the host's git configuration rewrites to ssh SHALL not be run: the repository SHALL be named with the URL it would have used, so a fetch never waits on an ssh agent or on someone at the keyboard.

#### Scenario: Git asks no one

- **WHEN** the runner fetches a clone
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

A place SHALL have ended once the status the report carries of it stands not listed, or reads the place's phase `tidied` or `gone` and does not name `boomdrive` among those its record is pending for. A place with no status carried SHALL not have ended. So a place whose record Flume holds for the runner SHALL not have ended until the runner has collected it (record 0030).

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
