## MODIFIED Requirements

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
