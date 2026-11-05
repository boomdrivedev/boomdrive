# Views

## ADDED Requirements

### Requirement: Each owner's views have a VS Code workspace where the configuration asks

Where `code_workspaces` is true, each owner of a kept repository SHALL have one VS Code workspace, the file `<repository_root>/<owner>/<owner>.code-workspace`. Its `folders` SHALL list exactly that owner's kept repositories, in the configuration's order, each by its view's path from the file's folder, `{"path": "<name>"}`, whether or not its view stands. A file the runner makes SHALL hold `folders` and an empty `settings`. Otherwise the runner SHALL write no workspace.

#### Scenario: The first workspaces

- **WHEN** a pass keeps `agentplot/flume`, `afterthought/crew` and `agentplot/sluice`, in that order, with `code_workspaces = true` and the repository root `/h/Code`, and no workspace is there
- **THEN** `/h/Code/agentplot/agentplot.code-workspace` holds `{"folders": [{"path": "flume"}, {"path": "sluice"}], "settings": {}}`, and `/h/Code/afterthought/afterthought.code-workspace` holds `{"folders": [{"path": "crew"}], "settings": {}}`

#### Scenario: No workspace asked for

- **WHEN** a pass keeps `agentplot/flume` with a configuration that has no `code_workspaces`
- **THEN** no file under `/h/Code` ends in `.code-workspace`

#### Scenario: A kept repository with no view

- **WHEN** `code_workspaces` is true and the configuration keeps `agentplot/flume` and `agentplot/empty`, whose origin has no branch
- **THEN** after the pass `/h/Code/agentplot/empty` does not exist, and the workspace's `folders` are `[{"path": "flume"}, {"path": "empty"}]`

### Requirement: A workspace's folders follow the configuration and the rest of it is kept

At every pass where `code_workspaces` is true, the runner SHALL set each workspace's `folders` to its owner's kept repositories, and SHALL keep every other member of the file as it was, in its order. It SHALL write the file only where its `folders` differ, as a whole file written beside it and renamed over it, and SHALL then say one line naming it. A pass that is stopped SHALL write no workspace.

#### Scenario: The user's settings kept

- **WHEN** `/h/Code/agentplot/agentplot.code-workspace` holds `folders` listing `flume`, then `settings` holding `"editor.tabSize": 2`, then `extensions` holding a recommendation, and `agentplot/sluice` is added to the configuration
- **THEN** after the next pass its `folders` list `flume` and `sluice`, and its `settings` and `extensions` are as they were, in that order

#### Scenario: Folders edited by hand

- **WHEN** the user removes `{"path": "flume"}` from the workspace's `folders`
- **THEN** the next pass puts it back, and says one line naming the file

#### Scenario: A dropped repository leaves its workspace

- **WHEN** `agentplot/sluice` is dropped from the configuration while its view holds an uncommitted change, and `agentplot/flume` is still kept
- **THEN** after the next pass the workspace's `folders` list only `flume`, and `/h/Code/agentplot/sluice` is as it was

#### Scenario: A quiet pass leaves the workspaces

- **WHEN** every workspace's `folders` already list its owner's kept repositories
- **THEN** the pass leaves each file byte for byte as it was, and says nothing of it

### Requirement: A workspace the runner cannot read is left as it is

Where an owner's workspace path holds a link, a folder, or a file that is not a JSON object the runner can read, a comment or a trailing comma among the reasons, the runner SHALL leave it as it is. It SHALL name it at every pass, with what is wrong and never the file's text. Such a workspace SHALL not fail the pass. A workspace that cannot be read or written SHALL be named, and the pass SHALL fail.

#### Scenario: A comment in the workspace

- **WHEN** `/h/Code/agentplot/agentplot.code-workspace` holds a line `// mine`, and `agentplot/sluice` is added to the configuration
- **THEN** after the next pass the file is byte for byte as it was, one line names it, and the pass does not fail

#### Scenario: A link at the workspace's path

- **WHEN** `/h/Code/agentplot/agentplot.code-workspace` is a symbolic link to a JSON file elsewhere
- **THEN** after the next pass the link and its target are as they were, one line names the path, and the pass does not fail

#### Scenario: A workspace that cannot be written

- **WHEN** no file can be made in `/h/Code/agentplot`, and the workspace's `folders` differ from its owner's kept repositories
- **THEN** the pass names the file and fails, and the file is as it was

## RENAMED Requirements

- FROM: `### Requirement: A removed view takes its owner's folder only when that is empty`
- TO: `### Requirement: A removed view takes its owner's folder only when nothing but a workspace no owner keeps is left there`

## MODIFIED Requirements

### Requirement: A removed view takes its owner's folder only when nothing but a workspace no owner keeps is left there

When the runner removes a view, it SHALL also remove the view's owner folder, `<repository_root>/<owner>`, if that folder is then empty. Where `code_workspaces` is true and the folder then holds only `<owner>.code-workspace`, and that is not the workspace of an owner the configuration keeps, the runner SHALL remove the file and then the folder. It SHALL remove nothing else under the repository root.

#### Scenario: The owner's last view

- **WHEN** the runner removes `/h/Code/agentplot/flume` and `/h/Code/agentplot` then holds nothing
- **THEN** `/h/Code/agentplot` is removed, and `/h/Code` is not

#### Scenario: The owner's folder holds more

- **WHEN** the runner removes `/h/Code/agentplot/flume` and `/h/Code/agentplot` holds another folder
- **THEN** `/h/Code/agentplot` and what it holds are left

#### Scenario: The owner's last view, with its workspace

- **WHEN** `code_workspaces` is true, and `agentplot/flume`, the configuration's last repository of `agentplot`, is dropped holding nothing the forge lacks, and `/h/Code/agentplot` holds its view and `agentplot.code-workspace`
- **THEN** after the next pass `/h/Code/agentplot` is gone

#### Scenario: An owner still kept keeps its workspace

- **WHEN** `code_workspaces` is true, `agentplot/flume` is dropped holding nothing the forge lacks, and `agentplot/empty`, whose origin has no branch, is still kept
- **THEN** after the next pass `/h/Code/agentplot/agentplot.code-workspace` lists only `empty`, with its other members as they were

#### Scenario: A workspace not asked for

- **WHEN** the configuration has no `code_workspaces`, `agentplot/flume` is dropped holding nothing the forge lacks, and `/h/Code/agentplot` holds its view and `agentplot.code-workspace`
- **THEN** after the next pass the view is gone, and `/h/Code/agentplot` holds `agentplot.code-workspace` as it was

#### Scenario: A new root, the old owner's workspace

- **WHEN** `code_workspaces` is true, the repository root changes from `/h/Code` to `/h/src`, and `/h/Code/agentplot` holds only the clean view `flume` and `agentplot.code-workspace`
- **THEN** after the next pass `/h/Code/agentplot` is gone, and `/h/src/agentplot/agentplot.code-workspace` lists `flume`
