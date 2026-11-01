# branch-commands Specification

## Purpose
How the runner makes a bolt's branch when its control plane sends that command: by sluice, in the runner's clone of the bolt's repository, under the command's lease. Any host that keeps the repository may take the command. A branch already there counts as made, read as a fact from git and not from sluice's words. It is the first of the tool commands a control plane sends (core.1, core.10; flywheel-cloud record 0017 and core.19; the domain model's Command card).

## Requirements

### Requirement: A bolt's branch is made by sluice in the clone

For a branch command naming `branch` and `from` in a repository the runner keeps with its clone made, the runner SHALL run `sluice branch create <branch> --from <from> --json` in the clone's container, whatever the room. It SHALL carry sluice's standard output as the command's answer, as given, answering `done` where sluice exits 0, and `refused` otherwise, with sluice's `message` or standard error as words.

#### Scenario: No gate for the branch

- **WHEN** the stub `sluice` refuses for want of a `[branches]` table, and the forge holds no `bolt/x`
- **THEN** the command is answered `refused` with sluice's words, and has no origin commit

### Requirement: A branch command carries the commit origin's branch stands at

After sluice answers, whichever way, the runner SHALL read `git ls-remote origin refs/heads/<branch>`, and carry the commit origin's branch stands at as the command's origin commit, or none where origin holds no such branch. It SHALL read nothing from sluice's words for it.

#### Scenario: A branch made

- **WHEN** a command asks for `bolt/x` from `main` in `afterthought/crew`, and the stub `sluice` exits 0 and the forge then holds `bolt/x` at `C`
- **THEN** the command is answered `done`, its answer the stub's output as printed, and its origin commit `C`

#### Scenario: A branch already there

- **WHEN** the forge already holds `bolt/x` at `C`, and the stub `sluice` exits 2 printing `{"error":{"kind":"refused","message":"bolt/x is already there, at C","next":[]}}`
- **THEN** the command is answered `refused`, its answer that text as printed, and its origin commit `C`

### Requirement: A branch command is taken only where its repository is kept

A branch command for a repository the configuration does not name SHALL be answered `refused`, naming the repository. One whose clone is not yet made SHALL be left untaken, so that another host keeping the repository may take it (core.5).

#### Scenario: A repository not kept

- **WHEN** a command asks for a branch in a repository the configuration does not name
- **THEN** it is answered `refused`, naming the repository, and no `sluice` command is run

#### Scenario: A clone not yet made

- **WHEN** a command asks for a branch in a repository the configuration names whose clone is not yet made
- **THEN** it is left untaken, and no `sluice` command is run

### Requirement: A command is carried out once

A command the runner's report holds settled SHALL be answered from its report, and its tool SHALL not be run again, across a restart too (core.10).

#### Scenario: A command given twice

- **WHEN** the same branch command is pulled at two passes, the runner restarting between them
- **THEN** the stub `sluice` is run once

### Requirement: A command this binary does not carry is refused

A command of any sort other than a branch SHALL be answered `refused`, naming its tool and saying this binary does not carry it.

#### Scenario: A boom command

- **WHEN** a command names the tool `boom`
- **THEN** it is answered `refused`, naming `boom`, and nothing is run
