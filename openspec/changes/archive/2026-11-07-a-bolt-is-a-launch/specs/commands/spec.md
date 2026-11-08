## Purpose

How the runner answers the commands its control plane sends, each under a lease of its own. This binary carries no tool's command, so a command of any sort is refused, naming its tool, and is answered once. A branch command is among them, refused naming sluice: a bolt is a launch of its caller's kind with no stages, whose branch sluice makes and closes at Flume's asking, and it is sent by a retry of its place's submission (core.1, core.10, core.16, core.52; record 0002 as amended on 2026-11-07; Flume's 202 to 206).

## ADDED Requirements

### Requirement: A command is refused, naming its tool

The runner SHALL answer `refused` every command its control plane sends under a live lease, with the words `this binary does not carry <tool>'s commands`, where a branch command's tool is sluice. It SHALL run no tool for it, and SHALL read neither its repository nor its clone, so a branch command in a repository the configuration does not keep, or whose clone is not yet made, is refused just as one whose clone is made (core.16, core.52).

#### Scenario: A branch command

- **WHEN** a command asks for `bolt/x` from `main` in `example/crew`, which the configuration names with its clone made
- **THEN** it is answered `refused` with the words `this binary does not carry sluice's commands`, no text and no origin commit, and no `sluice` command is run

#### Scenario: A clone not yet made

- **WHEN** a command asks for a branch in a repository the configuration names whose clone is not yet made
- **THEN** it is answered `refused`, naming sluice, rather than left untaken, and no `sluice` command is run

#### Scenario: A boom command

- **WHEN** a command names the tool `boom`
- **THEN** it is answered `refused`, naming `boom`, and nothing is run

### Requirement: A command is answered once

A command the runner's report holds answered SHALL be answered from its report, under the lease the control plane gives now, across a restart too, and its answer's time SHALL stand. A branch command an earlier release carried out SHALL keep its answer as the report holds it, sluice's text and its origin commit among it, and SHALL run nothing (core.10, core.36).

#### Scenario: A command given twice

- **WHEN** the same branch command is pulled at two passes, the runner restarting between them
- **THEN** it is answered `refused` at the first pass, and the second pass reports that answer with its time as it stood

#### Scenario: A branch command an earlier release carried out

- **WHEN** the report file holds the branch command `branch/1` answered `done`, with sluice's text and the origin commit `C`, as an earlier release answered it, and a pull gives `branch/1` again
- **THEN** it is reported `done`, its text and its origin commit `C` as the report held them, and no `sluice` command is run
