# clones

## Purpose

How the runner keeps each repository its configuration names: one bare clone in its own data directory, hidden from the user. The clone is fetched over https through the host's credential helper, on the runner's timer and when it is told of a landing, and its own default branch is kept at origin's. This spec also covers how the runner lets go of a repository the configuration drops (crew record 0052 part 2; swancloud's `names.3` and `herdr.13` as they move to the runner; core.5).

## ADDED Requirements

### Requirement: Each kept repository has one bare clone in the runner's data directory

The runner SHALL keep each configured repository in the bare-container layout under its data directory. Its data directory is `$XDG_DATA_HOME/flywheel`, or `~/.local/share/flywheel` where `XDG_DATA_HOME` is unset or empty. The container is `<data>/repositories/<owner>/<name>/`. It holds `<name>.git`, a bare repository whose origin is `https://github.com/<owner>/<name>.git`, and `.git`, a gitfile naming `./<name>.git`. Its places are left to Flume.

#### Scenario: A new repository is cloned

- **WHEN** a pass runs with `agentplot/flume` configured and no clone of it
- **THEN** `<data>/repositories/agentplot/flume/flume.git` is a bare repository whose `remote.origin.url` is `https://github.com/agentplot/flume.git`, holding every branch of origin as `refs/remotes/origin/*`, and `<data>/repositories/agentplot/flume/.git` reads `gitdir: ./flume.git`

#### Scenario: The clone is not under the views root

- **WHEN** a pass has cloned `agentplot/flume` with the views root `~/Code`
- **THEN** nothing of the clone's is under `~/Code` but the view

### Requirement: A clone that fails leaves nothing

A clone SHALL be made in a folder of its own beside its container and moved into place only once its first fetch has succeeded. A clone that fails SHALL leave nothing in the data directory. It SHALL be named with git's words and tried again at the next pass. Every other repository SHALL still be kept in that pass.

#### Scenario: Origin cannot be reached

- **WHEN** a pass clones `agentplot/flume` and origin answers no fetch
- **THEN** no folder of `agentplot/flume`'s is left in the data directory, the failure is named, every other configured repository is kept in the same pass, and the next pass tries the clone again

### Requirement: A folder the runner did not make is never touched

The runner SHALL know a clone as its own by the address it recorded in the clone when it made it. A folder at a container's path that is not a clone the runner made SHALL be left untouched and named, and no clone SHALL be made over it. A folder in the data directory that is not a clone the runner made SHALL never be fetched or removed.

#### Scenario: A folder already there

- **WHEN** `<data>/repositories/agentplot/flume` exists, holding a folder the runner did not make, and `agentplot/flume` is configured
- **THEN** the pass leaves the folder as it was, names it, and makes no clone and no view of `agentplot/flume`

#### Scenario: A stranger in the data directory

- **WHEN** `<data>/repositories/someone/else` is a folder the runner did not make and no configured address is `someone/else`
- **THEN** the pass neither fetches nor removes it

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

When the running runner is told that a kept repository has had a landing, it SHALL fetch that repository and keep its default branch and its view at once, without waiting for the next pass and without fetching any other repository. A landing told of a repository the configuration does not name SHALL clone and fetch nothing, and SHALL be named.

#### Scenario: Told between passes

- **WHEN** the runner keeps `agentplot/flume` and `agentplot/sluice`, fetches every hour, and is told of a landing on `agentplot/flume` a minute after a pass, origin's `main` having moved
- **THEN** within seconds `agentplot/flume` is fetched and its view moved to the new `origin/main`, and `agentplot/sluice` is not fetched

#### Scenario: Told of a repository it does not keep

- **WHEN** the runner is told of a landing on `someone/else`, which its configuration does not name
- **THEN** nothing is cloned or fetched, and the notice is named as for a repository the configuration does not name

### Requirement: The default branch is origin's

A clone's default branch SHALL be the branch origin names as its HEAD. Every fetch SHALL keep the clone's `refs/remotes/origin/HEAD` current, and the clone's own HEAD SHALL name the same branch. A repository whose origin names no default branch, or has none, SHALL be named. It gets no view until origin has one and is tried again at the next pass.

#### Scenario: Origin's default is not main

- **WHEN** origin's HEAD names `trunk`
- **THEN** the clone's HEAD names `refs/heads/trunk` and its view is detached at `origin/trunk`

#### Scenario: An empty origin

- **WHEN** origin of a configured repository has no branch
- **THEN** the pass names it as having no default branch, makes no view, and the pass after origin gains one makes the view

### Requirement: The clone's own default branch follows origin's

After each fetch, the runner SHALL move the clone's own default branch to `origin/<default>` when that move is a fast-forward and no worktree has the branch checked out. A default branch that holds commits origin lacks, or that a worktree holds, SHALL be left where it is and named.

#### Scenario: Behind origin

- **WHEN** origin's `main` moves two commits ahead of the clone's `main` and no worktree holds `main`
- **THEN** after the next pass the clone's `main` is `origin/main`

#### Scenario: Held by a worktree

- **WHEN** a worktree of the clone has `main` checked out and origin's `main` moves ahead
- **THEN** the clone's `main` is left where it was and named as held by that worktree

#### Scenario: Ahead of origin

- **WHEN** the clone's `main` holds a commit origin's `main` lacks
- **THEN** it is left where it is and named with how many commits it is ahead

### Requirement: A dropped repository is no longer fetched

A clone the runner made whose address the configuration no longer names SHALL never be fetched again, by the timer or by a landing told of it.

#### Scenario: Dropped while holding work

- **WHEN** `agentplot/flume` is dropped from the configuration while its clone holds work the forge lacks, and origin's `main` then gains a commit
- **THEN** after the next pass the clone's `refs/remotes/origin/main` has not moved

### Requirement: A dropped repository is removed once it holds nothing the forge lacks

At every pass, a dropped repository's clone and view SHALL be removed, the view first and then the container, once neither holds work the forge lacks. Until then both SHALL be left untouched and named with each thing that holds them.

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
