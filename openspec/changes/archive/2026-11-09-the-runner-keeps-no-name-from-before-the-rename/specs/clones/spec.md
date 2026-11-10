# clones

## MODIFIED Requirements

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

## ADDED Requirements

### Requirement: The data directory is Boomdrive's alone

The runner's data directory SHALL be `$XDG_DATA_HOME/boomdrive`, or `~/.local/share/boomdrive` where `XDG_DATA_HOME` is unset or empty, and no other folder, whatever else is beside it. The runner SHALL read no folder on disk to choose it (record 0035; core.35, core.36).

#### Scenario: The data directory is the runner's own

- **WHEN** the runner starts with `XDG_DATA_HOME` set to a folder that holds a folder `other`, itself holding `repositories/` and `control/report.json`, and no folder `boomdrive`
- **THEN** its data directory is `boomdrive` in that folder, and it still is once that folder is made

## REMOVED Requirements

### Requirement: A data directory from before the rename is kept

**Reason**: Record 0035, accepted on 2026-11-09 on the user's words ("When we cut over everything should be Boom Drive"), drops every reading of the runner's name from before the rename. The data directory is Boomdrive's alone (core.36).

**Migration**: Each host moves its data directory to `boomdrive` by hand, with the runner stopped and no place live, before it runs the release, as record 0035's chore says and the release's notes name.

### Requirement: A clone's records from before the rename are carried over

**Reason**: Record 0035 drops the carry-over. A clone is the runner's only by `boomdrive.address`, and its view's commit is read only at `refs/boomdrive/view` (core.36).

**Migration**: The same chore rewrites each moved clone's records and its view's ref under `boomdrive`, and repairs the link of each worktree the moved clones hold, before the host runs the release.
