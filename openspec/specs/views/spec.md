# views Specification

## Purpose
The one view of each kept repository, which the user browses. It is a worktree of the runner's clone under the views root, detached at `origin/<default>` and never on a branch. It is moved only while it holds no uncommitted change and stands where the runner last put it, so a view the user moved stays where they put it (crew record 0052 part 2; swancloud's `herdr.14` as it moves to the runner, carried whole).

## Requirements

### Requirement: Each kept repository has one view under the repository root

Each kept repository whose origin has a default branch SHALL have one view at `<repository_root>/<owner>/<name>`. The view is a worktree of the repository's clone, detached at `origin/<default>`. Where the view's folder is missing, the runner SHALL make it, with any folders above it. The runner SHALL make no other checkout of the repository.

#### Scenario: The first view

- **WHEN** a pass clones `agentplot/flume`, whose origin's default branch is `main`, with the repository root `/h/Code`
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

### Requirement: A view moves with the repository root

When the configured repository root no longer gives the path where the runner made a repository's view, the runner SHALL remove the view at the old path once it holds nothing the forge lacks (clones spec), and then make the view at the new path. While the old view holds work the forge lacks, it SHALL be left untouched and named, and no view SHALL be made at the new path.

#### Scenario: A new root, the old view clean

- **WHEN** the repository root changes from `/h/Code` to `/h/src` and `/h/Code/agentplot/flume` is clean and detached at a commit origin's refs reach
- **THEN** after the next pass `/h/Code/agentplot/flume` is gone and `/h/src/agentplot/flume` is the view, detached at `origin/main`

#### Scenario: A new root, the old view holding work

- **WHEN** the repository root changes from `/h/Code` to `/h/src` and `/h/Code/agentplot/flume` holds an uncommitted change
- **THEN** after the next pass `/h/Code/agentplot/flume` is as it was and named, and `/h/src/agentplot/flume` does not exist

### Requirement: An old view that is gone holds nothing back

An old view whose folder is gone, or is no longer a worktree of the clone, SHALL not hold the new view back. The runner SHALL forget it, leave any folder there untouched, and make the view at the new path.

#### Scenario: A new root, the old folder replaced

- **WHEN** the repository root changes from `/h/Code` to `/h/src` and `/h/Code/agentplot/flume` is now a folder of the user's that is not a worktree of the clone
- **THEN** after the next pass that folder is as it was and `/h/src/agentplot/flume` is the view

### Requirement: Each drive has a VS Code workspace at the repository root where the configuration asks

Where `code_workspaces` is true, each drive of the configuration SHALL have one VS Code workspace, the file `<repository_root>/<drive>.code-workspace`. Its `folders` SHALL list exactly the drive's kept repositories, in the configuration's order, each by its view's path from the root, `{"path": "<owner>/<name>"}`, whether or not its view stands. A file the runner makes SHALL hold `folders` and an empty `settings`. Otherwise the runner SHALL write no workspace.

#### Scenario: The first workspaces

- **WHEN** a pass keeps the drive `home`, naming `agentplot/flume` and `example/crew`, and the drive `work`, naming `agentplot/sluice`, with `code_workspaces = true` and the repository root `/h/Code`, and no workspace is there
- **THEN** `/h/Code/home.code-workspace` holds `{"folders": [{"path": "agentplot/flume"}, {"path": "example/crew"}], "settings": {}}`, `/h/Code/work.code-workspace` holds `{"folders": [{"path": "agentplot/sluice"}], "settings": {}}`, and no owner's folder holds a workspace

#### Scenario: No workspace asked for

- **WHEN** a pass keeps `agentplot/flume` with a configuration that has no `code_workspaces`
- **THEN** no file under `/h/Code` ends in `.code-workspace`

#### Scenario: A kept repository with no view

- **WHEN** `code_workspaces` is true and the drive `home` names `agentplot/flume` and `agentplot/empty`, whose origin has no branch
- **THEN** after the pass `/h/Code/agentplot/empty` does not exist, and the workspace's `folders` are `[{"path": "agentplot/flume"}, {"path": "agentplot/empty"}]`

#### Scenario: A drive keeping none

- **WHEN** `code_workspaces` is true and the drive `garden` says `repositories = []`
- **THEN** `/h/Code/garden.code-workspace` holds `{"folders": [], "settings": {}}`

#### Scenario: A new root

- **WHEN** `code_workspaces` is true, the drive `home` names `agentplot/flume`, and the repository root changes from `/h/Code` to `/h/src`
- **THEN** after the next pass `/h/src/home.code-workspace` lists `agentplot/flume`, and `/h/Code/home.code-workspace` is as it was

### Requirement: A workspace's folders follow the configuration and the rest of it is kept

At every pass where `code_workspaces` is true, the runner SHALL set each workspace's `folders` to its drive's kept repositories, and SHALL keep every other member of the file as it was, in its order. It SHALL write the file only where its `folders` differ, as a whole file written beside it and renamed over it, and SHALL then say one line naming it. A pass that is stopped SHALL write no workspace.

#### Scenario: The user's settings kept

- **WHEN** `/h/Code/home.code-workspace` holds `folders` listing `agentplot/flume`, then `settings` holding `"editor.tabSize": 2`, then `extensions` holding a recommendation, and `agentplot/sluice` is added to the drive `home`
- **THEN** after the next pass its `folders` list `agentplot/flume` and `agentplot/sluice`, and its `settings` and `extensions` are as they were, in that order

#### Scenario: Folders edited by hand

- **WHEN** the user removes `{"path": "agentplot/flume"}` from the drive `home`'s workspace's `folders`
- **THEN** the next pass puts it back, and says one line naming the file

#### Scenario: A dropped repository leaves its workspace

- **WHEN** `agentplot/sluice` is dropped from the drive `home` while its view holds an uncommitted change, and `agentplot/flume` is still kept there
- **THEN** after the next pass the workspace's `folders` list only `agentplot/flume`, and `/h/Code/agentplot/sluice` is as it was

#### Scenario: A repository moved to another drive

- **WHEN** `agentplot/sluice` moves from the drive `home`, which still names `agentplot/flume`, to the drive `work`
- **THEN** after the next pass `home`'s workspace lists only `agentplot/flume`, `work`'s lists `agentplot/sluice`, and the view `/h/Code/agentplot/sluice` is where it was

#### Scenario: A quiet pass leaves the workspaces

- **WHEN** every workspace's `folders` already list its drive's kept repositories
- **THEN** the pass leaves each file byte for byte as it was, and says nothing of it

### Requirement: A workspace the runner cannot read is left as it is

Where a drive's workspace path holds a link, a folder, or a file that is not a JSON object the runner can read, a comment or a trailing comma among the reasons, the runner SHALL leave it as it is. It SHALL name it at every pass, with what is wrong and never the file's text. Such a workspace SHALL not fail the pass. A workspace that cannot be read or written SHALL be named, and the pass SHALL fail.

#### Scenario: A comment in the workspace

- **WHEN** `/h/Code/home.code-workspace` holds a line `// mine`, and `agentplot/sluice` is added to the drive `home`
- **THEN** after the next pass the file is byte for byte as it was, one line names it, and the pass does not fail

#### Scenario: A link at the workspace's path

- **WHEN** `/h/Code/home.code-workspace` is a symbolic link to a JSON file elsewhere
- **THEN** after the next pass the link and its target are as they were, one line names the path, and the pass does not fail

#### Scenario: A workspace that cannot be written

- **WHEN** no file can be made in `/h/Code`, and the drive `home`'s workspace's `folders` differ from the drive's kept repositories
- **THEN** the pass names the file and fails, and the file is as it was

### Requirement: A removed view takes its owner's folder only when that is empty

When the runner removes a view, it SHALL also remove the view's owner folder, `<repository_root>/<owner>`, if that folder is then empty. It SHALL remove nothing else under the repository root but a drive's workspace that goes with its drive. A workspace an earlier runner kept in an owner's folder, `<owner>.code-workspace`, SHALL be left as it is, with its folder (record 0017).

#### Scenario: The owner's last view

- **WHEN** the runner removes `/h/Code/agentplot/flume` and `/h/Code/agentplot` then holds nothing
- **THEN** `/h/Code/agentplot` is removed, and `/h/Code` is not

#### Scenario: The owner's folder holds more

- **WHEN** the runner removes `/h/Code/agentplot/flume` and `/h/Code/agentplot` holds another folder
- **THEN** `/h/Code/agentplot` and what it holds are left

#### Scenario: An owner's workspace from before

- **WHEN** `code_workspaces` is true, `agentplot/flume` is dropped holding nothing the forge lacks, and `/h/Code/agentplot` holds its view and `agentplot.code-workspace`
- **THEN** after the next pass the view is gone, and `/h/Code/agentplot` holds `agentplot.code-workspace` as it was

#### Scenario: A new root, an owner's workspace from before

- **WHEN** `code_workspaces` is true, the repository root changes from `/h/Code` to `/h/src`, and `/h/Code/agentplot` holds only the clean view `flume` and `agentplot.code-workspace`
- **THEN** after the next pass `/h/Code/agentplot` holds only `agentplot.code-workspace`, as it was, and `/h/src/agentplot/flume` is the view

### Requirement: A drive's workspace goes with its drive

The runner SHALL record in each kept repository's clone the drive the configuration names it under. Where `code_workspaces` is true, once a drive has left the configuration and no clone the runner holds records it, the pass SHALL remove `<repository_root>/<drive>.code-workspace` where it is a file the runner reads as a JSON object, saying one line, and SHALL leave anything else there as it is, naming it. It SHALL remove no other workspace (record 0017).

#### Scenario: The drive's last view

- **WHEN** `code_workspaces` is true, the drive `home` names only `agentplot/flume`, whose view is clean, and `home` is dropped from the configuration
- **THEN** after the next pass the view `/h/Code/agentplot/flume`, its clone and `/h/Code/home.code-workspace` are gone, and one line names the workspace

#### Scenario: A view still holding work

- **WHEN** `code_workspaces` is true and the drive `home`, naming only `agentplot/flume`, is dropped while flume's view holds an uncommitted change
- **THEN** after the next pass `/h/Code/home.code-workspace` is as it was; and once the change is gone, the pass that removes the view removes the workspace

#### Scenario: Its repositories moved

- **WHEN** `code_workspaces` is true and the drive `home` is dropped while `agentplot/flume`, its one repository, is named by the drive `work`
- **THEN** after the next pass `/h/Code/home.code-workspace` is gone, `/h/Code/work.code-workspace` lists `agentplot/flume`, and the view is where it was

#### Scenario: A workspace of the person's own

- **WHEN** `code_workspaces` is true, `/h/Code/notes.code-workspace` is a file of the person's, and the drive `home` is dropped holding nothing the forge lacks
- **THEN** after the next pass `/h/Code/notes.code-workspace` is byte for byte as it was

#### Scenario: A departed drive's workspace the runner cannot read

- **WHEN** `code_workspaces` is true, `/h/Code/home.code-workspace` holds a line `// mine`, and the drive `home` is dropped holding nothing the forge lacks
- **THEN** after the next pass the file is byte for byte as it was, one line names it, and the pass does not fail

#### Scenario: A departed drive's workspace not asked for

- **WHEN** the configuration has no `code_workspaces`, `/h/Code/home.code-workspace` is there, and the drive `home` is dropped holding nothing the forge lacks
- **THEN** after the next pass the view and the clone are gone, and `/h/Code/home.code-workspace` is as it was
