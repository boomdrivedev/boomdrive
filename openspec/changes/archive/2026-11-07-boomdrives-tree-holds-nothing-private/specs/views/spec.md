## MODIFIED Requirements

### Requirement: Each owner's views have a VS Code workspace where the configuration asks

Where `code_workspaces` is true, each owner of a kept repository SHALL have one VS Code workspace, the file `<repository_root>/<owner>/<owner>.code-workspace`. Its `folders` SHALL list exactly that owner's kept repositories, in the configuration's order, each by its view's path from the file's folder, `{"path": "<name>"}`, whether or not its view stands. A file the runner makes SHALL hold `folders` and an empty `settings`. Otherwise the runner SHALL write no workspace.

#### Scenario: The first workspaces

- **WHEN** a pass keeps `agentplot/flume`, `example/crew` and `agentplot/sluice`, in that order, with `code_workspaces = true` and the repository root `/h/Code`, and no workspace is there
- **THEN** `/h/Code/agentplot/agentplot.code-workspace` holds `{"folders": [{"path": "flume"}, {"path": "sluice"}], "settings": {}}`, and `/h/Code/example/example.code-workspace` holds `{"folders": [{"path": "crew"}], "settings": {}}`

#### Scenario: No workspace asked for

- **WHEN** a pass keeps `agentplot/flume` with a configuration that has no `code_workspaces`
- **THEN** no file under `/h/Code` ends in `.code-workspace`

#### Scenario: A kept repository with no view

- **WHEN** `code_workspaces` is true and the configuration keeps `agentplot/flume` and `agentplot/empty`, whose origin has no branch
- **THEN** after the pass `/h/Code/agentplot/empty` does not exist, and the workspace's `folders` are `[{"path": "flume"}, {"path": "empty"}]`
