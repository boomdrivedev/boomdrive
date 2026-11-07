# Views

## RENAMED Requirements

- FROM: `### Requirement: Each owner's views have a VS Code workspace where the configuration asks`
- TO: `### Requirement: Each drive has a VS Code workspace at the repository root where the configuration asks`

## REMOVED Requirements

### Requirement: A removed view takes its owner's folder only when nothing but a workspace no owner keeps is left there

**Reason**: The runner keeps no workspace in an owner's folder: each drive's is at the repository root (record 0017 as revised). So a removed view's owner folder never holds a workspace of the runner's to take with it. Its successor, "A removed view takes its owner's folder only when that is empty", leaves a workspace of before where it is.

**Migration**: A workspace an earlier runner kept in an owner's folder is left as it is, with its folder and its settings, for the person to remove. Each drive's workspace is written at the root where `code_workspaces` is true.

## MODIFIED Requirements

### Requirement: Each drive has a VS Code workspace at the repository root where the configuration asks

Where `code_workspaces` is true, each drive of the configuration SHALL have one VS Code workspace, the file `<repository_root>/<drive>.code-workspace`. Its `folders` SHALL list exactly the drive's kept repositories, in the configuration's order, each by its view's path from the root, `{"path": "<owner>/<name>"}`, whether or not its view stands. A file the runner makes SHALL hold `folders` and an empty `settings`. Otherwise the runner SHALL write no workspace.

#### Scenario: The first workspaces

- **WHEN** a pass keeps the drive `home`, naming `agentplot/flume` and `afterthought/crew`, and the drive `work`, naming `agentplot/sluice`, with `code_workspaces = true` and the repository root `/h/Code`, and no workspace is there
- **THEN** `/h/Code/home.code-workspace` holds `{"folders": [{"path": "agentplot/flume"}, {"path": "afterthought/crew"}], "settings": {}}`, `/h/Code/work.code-workspace` holds `{"folders": [{"path": "agentplot/sluice"}], "settings": {}}`, and no owner's folder holds a workspace

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

## ADDED Requirements

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
