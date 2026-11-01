# Clones

## MODIFIED Requirements

### Requirement: Each kept repository has one bare clone in the runner's data directory

The runner SHALL keep each configured repository in the bare-container layout under its data directory. Its data directory is `$XDG_DATA_HOME/flywheel`, or `~/.local/share/flywheel` where `XDG_DATA_HOME` is unset or empty. The container is `<data>/repositories/<owner>/<name>/`. It holds `<name>.git`, a bare repository whose origin is `https://github.com/<owner>/<name>.git`, and `.git`, a gitfile naming `./<name>.git`. Its places are left to Flume.

#### Scenario: A new repository is cloned

- **WHEN** a pass runs with `agentplot/flume` configured and no clone of it
- **THEN** `<data>/repositories/agentplot/flume/flume.git` is a bare repository whose `remote.origin.url` is `https://github.com/agentplot/flume.git`, holding every branch of origin as `refs/remotes/origin/*`, and `<data>/repositories/agentplot/flume/.git` reads `gitdir: ./flume.git`

#### Scenario: The clone is not under the views root

- **WHEN** a pass has cloned `agentplot/flume` with the repository root `~/Code`
- **THEN** nothing of the clone's is under `~/Code` but the view
