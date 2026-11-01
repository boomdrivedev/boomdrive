# Views

## RENAMED Requirements

- FROM: `### Requirement: Each kept repository has one view under the views root`
- TO: `### Requirement: Each kept repository has one view under the repository root`
- FROM: `### Requirement: A view moves with its views root`
- TO: `### Requirement: A view moves with the repository root`

## MODIFIED Requirements

### Requirement: Each kept repository has one view under the repository root

Each kept repository whose origin has a default branch SHALL have one view at `<repository_root>/<owner>/<name>`. The view is a worktree of the repository's clone, detached at `origin/<default>`. Where the view's folder is missing, the runner SHALL make it, with any folders above it. The runner SHALL make no other checkout of the repository.

#### Scenario: The first view

- **WHEN** a pass clones `agentplot/flume`, whose origin's default branch is `main`, with the repository root `/h/Code`
- **THEN** `/h/Code/agentplot/flume` is a worktree of the clone whose HEAD is detached at `origin/main`, and `git symbolic-ref HEAD` there fails

#### Scenario: A view the user deleted

- **WHEN** the user removes the folder `/h/Code/agentplot/flume` of a kept repository
- **THEN** the next pass makes the view there again, detached at `origin/main`

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

### Requirement: A removed view takes its owner's folder only when that is empty

When the runner removes a view, it SHALL also remove the view's owner folder, `<repository_root>/<owner>`, if that folder is then empty. It SHALL remove nothing else under the repository root.

#### Scenario: The owner's last view

- **WHEN** the runner removes `/h/Code/agentplot/flume` and `/h/Code/agentplot` then holds nothing
- **THEN** `/h/Code/agentplot` is removed, and `/h/Code` is not

#### Scenario: The owner's folder holds more

- **WHEN** the runner removes `/h/Code/agentplot/flume` and `/h/Code/agentplot` holds another folder
- **THEN** `/h/Code/agentplot` and what it holds are left
