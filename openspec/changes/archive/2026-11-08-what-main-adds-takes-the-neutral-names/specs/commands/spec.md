## MODIFIED Requirements

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
