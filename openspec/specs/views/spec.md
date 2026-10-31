# views Specification

## Purpose
The one view of each kept repository, which the user browses. It is a worktree of the runner's clone under the views root, detached at `origin/<default>` and never on a branch. It is moved only while it holds no uncommitted change and stands where the runner last put it, so a view the user moved stays where they put it (crew record 0052 part 2; swancloud's `herdr.14` as it moves to the runner, carried whole).

## Requirements

### Requirement: Each kept repository has one view under the views root

Each kept repository whose origin has a default branch SHALL have one view at `<views>/<owner>/<name>`. The view is a worktree of the repository's clone, detached at `origin/<default>`. Where the view's folder is missing, the runner SHALL make it, with any folders above it. The runner SHALL make no other checkout of the repository.

#### Scenario: The first view

- **WHEN** a pass clones `agentplot/flume`, whose origin's default branch is `main`, with the views root `/h/Code`
- **THEN** `/h/Code/agentplot/flume` is a worktree of the clone whose HEAD is detached at `origin/main`, and `git symbolic-ref HEAD` there fails

#### Scenario: A view the user deleted

- **WHEN** the user removes the folder `/h/Code/agentplot/flume` of a kept repository
- **THEN** the next pass makes the view there again, detached at `origin/main`

### Requirement: A view is never made over a folder that is there

Where the view's folder exists and is not a worktree of the repository's clone, the runner SHALL leave it untouched and name it, at every pass. It SHALL make no view of that repository elsewhere.

#### Scenario: A folder of the user's

- **WHEN** `/h/Code/agentplot/flume` holds a folder of the user's that is not a worktree of the clone
- **THEN** the pass leaves the folder as it was, names it, and the clone is still fetched

### Requirement: The runner never puts a view on a branch

The runner SHALL make and move a view only as a detached HEAD. It SHALL never check out a branch in a view.

#### Scenario: Made and moved detached

- **WHEN** a view is made and later moved by the runner
- **THEN** after each, the view's HEAD is detached and names no branch

### Requirement: A view follows origin while it stands where the runner last put it

After each fetch, the runner SHALL move a view to `origin/<default>` only when all of these hold: the view's HEAD is detached; it is at the commit the runner last put it at; and it holds no uncommitted change to a tracked file. Otherwise the view SHALL be left without a word, since the user has moved it or is working in it. Untracked files SHALL not hold a view: they stay in it as they are. A move that git refuses SHALL be named with git's words.

#### Scenario: Origin moves on

- **WHEN** origin's `main` gains a commit and the view is clean and detached where the runner last put it
- **THEN** after the next pass the view is detached at the new `origin/main`

#### Scenario: The user moved it

- **WHEN** the user runs `git switch --detach HEAD~1` in the view and origin's `main` then gains a commit
- **THEN** after the next pass the view is still where the user put it, and nothing is said of it

#### Scenario: Back where the runner put it

- **WHEN** the user moves the view back to the commit the runner last put it at
- **THEN** the next pass moves it to `origin/main` again

#### Scenario: An uncommitted change

- **WHEN** the view holds an uncommitted change to a tracked file and origin's `main` gains a commit
- **THEN** after the next pass the view is where it was, with the change as it was

#### Scenario: On a branch

- **WHEN** the user switches the view onto a branch
- **THEN** the runner leaves it on that branch at every pass

#### Scenario: An untracked file

- **WHEN** the view, clean and where the runner last put it, holds an untracked file that the new `origin/main` does not add, and origin's `main` gains a commit
- **THEN** after the next pass the view is at the new `origin/main` and the untracked file is still there

### Requirement: A view moves with its views root

When the configured views root no longer gives the path where the runner made a repository's view, the runner SHALL remove the view at the old path once it holds nothing the forge lacks (clones spec), and then make the view at the new path. While the old view holds work the forge lacks, it SHALL be left untouched and named, and no view SHALL be made at the new path.

#### Scenario: A new root, the old view clean

- **WHEN** the views root changes from `/h/Code` to `/h/src` and `/h/Code/agentplot/flume` is clean and detached at a commit origin's refs reach
- **THEN** after the next pass `/h/Code/agentplot/flume` is gone and `/h/src/agentplot/flume` is the view, detached at `origin/main`

#### Scenario: A new root, the old view holding work

- **WHEN** the views root changes from `/h/Code` to `/h/src` and `/h/Code/agentplot/flume` holds an uncommitted change
- **THEN** after the next pass `/h/Code/agentplot/flume` is as it was and named, and `/h/src/agentplot/flume` does not exist

### Requirement: An old view that is gone holds nothing back

An old view whose folder is gone, or is no longer a worktree of the clone, SHALL not hold the new view back. The runner SHALL forget it, leave any folder there untouched, and make the view at the new path.

#### Scenario: A new root, the old folder replaced

- **WHEN** the views root changes from `/h/Code` to `/h/src` and `/h/Code/agentplot/flume` is now a folder of the user's that is not a worktree of the clone
- **THEN** after the next pass that folder is as it was and `/h/src/agentplot/flume` is the view

### Requirement: A removed view takes its owner's folder only when that is empty

When the runner removes a view, it SHALL also remove the view's owner folder, `<views>/<owner>`, if that folder is then empty. It SHALL remove nothing else under the views root.

#### Scenario: The owner's last view

- **WHEN** the runner removes `/h/Code/agentplot/flume` and `/h/Code/agentplot` then holds nothing
- **THEN** `/h/Code/agentplot` is removed, and `/h/Code` is not

#### Scenario: The owner's folder holds more

- **WHEN** the runner removes `/h/Code/agentplot/flume` and `/h/Code/agentplot` holds another folder
- **THEN** `/h/Code/agentplot` and what it holds are left
