# Clones

## MODIFIED Requirements

### Requirement: A repository is fetched when the runner is told of its landing

When the running runner is told that a kept repository has had a landing, it SHALL fetch that repository, settle its HEAD and keep its view at once, without waiting for the next pass and without fetching any other repository. A landing told of a repository the configuration does not name SHALL clone and fetch nothing, and SHALL be named.

#### Scenario: Told between passes

- **WHEN** the runner keeps `agentplot/flume` and `agentplot/sluice`, fetches every hour, and is told of a landing on `agentplot/flume` a minute after a pass, origin's `main` having moved
- **THEN** within seconds `agentplot/flume` is fetched and its view moved to the new `origin/main`, and `agentplot/sluice` is not fetched

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

## ADDED Requirements

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

- **WHEN** a clone made before the rename has its `main` guarded as sluice guards a branch it keeps and behind origin's, its view is clean under the old repository root, and the repository root changes
- **THEN** the pass removes the old view, makes the view under the new root at `origin/main`, writes nothing to the clone's `main`, and fails nothing

### Requirement: A HEAD that cannot be settled leaves the view kept

Where the runner cannot point the clone's HEAD at origin's default branch, or cannot make that branch where the clone lacks it, it SHALL name the repository with git's words, and the pass SHALL fail. It SHALL still keep the repository's view at `origin/<default>`, and every repository after it in the same pass.

#### Scenario: A new default the clone refuses

- **WHEN** origin's HEAD moves to a new branch `trunk`, and the clone refuses any write that makes its own `trunk`
- **THEN** the pass names `HEAD not settled on trunk` with git's words and fails, and the view is moved to `origin/trunk`

## REMOVED Requirements

### Requirement: The clone's own default branch follows origin's

**Reason**: Where sluice keeps the branch, the clone's own ref of it is sluice's copy, which sluice alone writes and which its protection guards (sluice's clause 124). Moving it refused the pass. Every reader of the runner's reads `origin/<default>`, so the move served none of them (record 0019; core.14).

**Migration**: Nothing on a host moves. A clone's own default branch stays where the last levelling put it, and sluice levels it where it keeps the branch. A reader of the host's default branch reads `origin/<default>` after a fetch.
