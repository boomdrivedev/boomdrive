## MODIFIED Requirements

### Requirement: A branch command carries the commit origin's branch stands at

After sluice answers, whichever way, the runner SHALL read `git ls-remote origin refs/heads/<branch>`, and carry the commit origin's branch stands at as the command's origin commit, or none where origin holds no such branch. It SHALL read nothing from sluice's words for it.

#### Scenario: A branch made

- **WHEN** a command asks for `bolt/x` from `main` in `example/crew`, and the stub `sluice` exits 0 and the forge then holds `bolt/x` at `C`
- **THEN** the command is answered `done`, its answer the stub's output as printed, and its origin commit `C`

#### Scenario: A branch already there

- **WHEN** the forge already holds `bolt/x` at `C`, and the stub `sluice` exits 2 printing `{"error":{"kind":"refused","message":"bolt/x is already there, at C","next":[]}}`
- **THEN** the command is answered `refused`, its answer that text as printed, and its origin commit `C`
