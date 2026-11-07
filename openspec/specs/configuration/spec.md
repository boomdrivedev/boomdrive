# configuration Specification

## Purpose
The runner's configuration on a host is one TOML file. It names the repositories the host keeps by their forge address, where their views go and how often they are fetched. This spec covers where the file is read from, what it may say, and its defaults and refusals. It also requires that a file which cannot be read never counts as one that names no repository (core.4, core.5; crew record 0052 part 2).

## Requirements

### Requirement: The configuration is one file the runner reads

The runner SHALL read its configuration from the file `--config <file>` names. Where none is named, it SHALL read `$XDG_CONFIG_HOME/boomdrive/config.toml`, or `~/.config/boomdrive/config.toml` where `XDG_CONFIG_HOME` is unset or empty. A file that does not exist or is not valid TOML SHALL be refused. The refusal SHALL name the file, the line and what is wrong, and SHALL never quote the file's text.

#### Scenario: The default file

- **WHEN** `boomdrive run --once` runs with no `--config`, `XDG_CONFIG_HOME` unset and `HOME` set to `/h`
- **THEN** it reads `/h/.config/boomdrive/config.toml`

#### Scenario: The file named

- **WHEN** `boomdrive run --once --config /etc/boomdrive.toml` runs with `XDG_CONFIG_HOME` set
- **THEN** it reads `/etc/boomdrive.toml` and no other file

#### Scenario: No file

- **WHEN** `boomdrive run` starts and the configuration file does not exist
- **THEN** it exits 2 naming the path it looked for, and makes, fetches, moves and removes nothing

### Requirement: The configuration names each repository by its forge address

The configuration SHALL list the repositories the host keeps under `repositories`. Each entry SHALL be a GitHub address `owner/name`. The owner SHALL be letters, digits and inner hyphens. The name SHALL be letters, digits, `-`, `_` and `.`, but not `.`, not `..`, and not ending in `.git`. `repositories` SHALL be required, so a host that keeps none says `repositories = []`. Two entries that are equal ignoring case SHALL be refused, naming both.

#### Scenario: An address with no owner

- **WHEN** the configuration says `repositories = ["flume"]`
- **THEN** it is refused, naming the entry `flume` and the form `owner/name`

#### Scenario: One repository twice

- **WHEN** the configuration says `repositories = ["agentplot/flume", "AgentPlot/flume"]`
- **THEN** it is refused, naming both entries

#### Scenario: No list

- **WHEN** the configuration sets `repository_root` and has no `repositories`
- **THEN** it is refused, naming `repositories` as required

#### Scenario: Keeping none

- **WHEN** the configuration says `repositories = []`
- **THEN** it is read as a host that keeps no repository

### Requirement: The repository root is a setting with a default

The configuration SHALL set the directory each kept repository's view is made under with `repository_root`. The value SHALL be an absolute path or one that begins with `~/`, where `~` is the host user's home. Where `repository_root` is absent, the repository root SHALL be `~/Code`. Any other value, a relative path or one naming a variable among them, SHALL be refused, naming it.

#### Scenario: No views setting

- **WHEN** the configuration names `agentplot/flume` and no `repository_root`, and `HOME` is `/h`
- **THEN** the repository's view is `/h/Code/agentplot/flume`

#### Scenario: A home-relative root

- **WHEN** the configuration says `repository_root = "~/src"` and `HOME` is `/h`
- **THEN** the repository root is `/h/src`

#### Scenario: A relative root

- **WHEN** the configuration says `repository_root = "Code"` or `repository_root = "$HOME/Code"`
- **THEN** it is refused, naming the value and saying a repository root is absolute or begins with `~/`

### Requirement: How often repositories are fetched is a setting with a default

The configuration SHALL set how often every kept repository is fetched with `fetch_interval`: a whole number of seconds, minutes or hours, written `<n>s`, `<n>m` or `<n>h`, more than zero. Where it is absent, the interval SHALL be two minutes. Any other value SHALL be refused, naming it.

#### Scenario: No period

- **WHEN** the configuration has no `fetch_interval`
- **THEN** the runner fetches every two minutes

#### Scenario: A period of nothing

- **WHEN** the configuration says `fetch_interval = "0m"` or `fetch_interval = "soon"`
- **THEN** it is refused, naming the value and the form `<n>s`, `<n>m` or `<n>h`

### Requirement: The configuration holds nothing the runner does not read

The configuration SHALL be refused, naming the key, when it holds any key this spec does not name. In particular, no key SHALL hold a token, key or credential: git's own credential helper on the host answers for every fetch (core.4).

#### Scenario: A misspelt key

- **WHEN** the configuration says `repositories_root = "~/Code"`
- **THEN** it is refused, naming `repositories_root` as a key the runner does not read

#### Scenario: A token

- **WHEN** the configuration says `token = "..."`
- **THEN** it is refused, naming `token` as a key the runner does not read, and the value appears in no output

### Requirement: The configuration is read again at every pass

The running runner SHALL read its configuration again at the start of every pass. A repository added SHALL be kept from that pass, and one dropped SHALL be let go from that pass. A pass that cannot read the configuration SHALL name the file and what is wrong with it, and SHALL make, fetch, move and remove nothing. The configuration SHALL never be taken to name no repository because it could not be read.

#### Scenario: A repository added while running

- **WHEN** `agentplot/sluice` is added to the configuration of a running runner
- **THEN** the next pass clones it and makes its view

#### Scenario: A file broken while running

- **WHEN** the configuration of a running runner that keeps `agentplot/flume` is made invalid
- **THEN** the next pass names the file's error and removes nothing, `agentplot/flume`'s clone and view among them; and once the file is valid again, the pass after keeps as it says

### Requirement: A refused stream, kind or key of before says where it is held

No key SHALL name a stream, a kind, a kinds file or a kind's source: streams, the stream a repository's launches run in, and where a host's kinds come from are each host's Flume configuration (core.5). A refusal of `stream` or `streams` SHALL say streams are, and one of `kind`, `kinds` or `kinds_file` that kinds are. A refusal of `views` or `fetch_every` SHALL name the key that holds it now, `repository_root` or `fetch_interval` (core.12).

#### Scenario: A stream named

- **WHEN** the configuration says `stream = "work"`, or holds a `[streams]` table
- **THEN** it is refused, naming the key as one the runner does not read and saying streams are Flume's host configuration

#### Scenario: A kinds file named

- **WHEN** the configuration holds `kinds = { crew = "/k/kinds.toml" }`
- **THEN** it is refused, naming `kinds` as a key the runner does not read and saying kinds are Flume's host configuration

#### Scenario: A key of before

- **WHEN** the configuration says `views = "~/Code"` and `fetch_every = "2m"`
- **THEN** it is refused, naming `views` as held by `repository_root` and `fetch_every` as held by `fetch_interval`

### Requirement: The configuration may name the control plane

The configuration MAY hold a `[control_plane]` table. In it, `type`, the word of the implementation to open, SHALL be required, and the table named by `type` SHALL be handed whole to that implementation, which reads its own keys. Where the configuration holds no `[control_plane]`, the runner SHALL keep its repositories and make no control pass.

#### Scenario: crew named

- **WHEN** the configuration holds `[control_plane]` with `type = "crew"` and a `[control_plane.crew]` table
- **THEN** the runner opens crew's implementation with that table

#### Scenario: No type

- **WHEN** the configuration holds `[control_plane]` with `poll_interval = "30s"` and no `type`
- **THEN** it is refused, naming `type` as required

#### Scenario: No control plane

- **WHEN** the configuration holds no `[control_plane]`
- **THEN** `boomdrive run --once` keeps the repositories and runs no `flume` command

### Requirement: How often the control plane is polled is a setting with a default

`[control_plane]`'s `poll_interval` SHALL be how often the runner makes a control pass, a whole number of seconds, minutes or hours written `<n>s`, `<n>m` or `<n>h`, more than zero. Where it is absent, the interval SHALL be one minute. Any other value SHALL be refused, naming it.

#### Scenario: No poll interval

- **WHEN** `[control_plane]` has no `poll_interval`
- **THEN** the runner makes a control pass every minute

#### Scenario: A poll interval of nothing

- **WHEN** `[control_plane]` says `poll_interval = "0s"`
- **THEN** it is refused, naming the value and the form `<n>s`, `<n>m` or `<n>h`

### Requirement: The control-plane table is read when the runner starts

The `[control_plane]` table SHALL be read when the runner starts, and a change to it SHALL take effect when the runner starts again.

#### Scenario: A poll interval changed while running

- **WHEN** `poll_interval` is changed from `1m` to `5m` in the file of a running runner
- **THEN** its control passes keep a one-minute interval until the runner starts again

### Requirement: Whether each owner's views have a VS Code workspace is a setting

The configuration MAY set `code_workspaces`: true where the runner keeps a VS Code workspace for each owner of its kept repositories (views spec), false where it keeps none. Where it is absent, it SHALL be false. Any value but `true` or `false` SHALL be refused, naming the key and the value. It SHALL be read again at every pass, as `repositories` is.

#### Scenario: No workspaces setting

- **WHEN** the configuration names `agentplot/flume` and has no `code_workspaces`
- **THEN** it is read with `code_workspaces` false

#### Scenario: Workspaces asked for in words

- **WHEN** the configuration says `code_workspaces = "yes"`
- **THEN** it is refused, naming `code_workspaces` and saying it is `true` or `false`

#### Scenario: Workspaces asked for while running

- **WHEN** `code_workspaces = true` is added to the configuration of a running runner that keeps `agentplot/flume`
- **THEN** the next pass writes `/h/Code/agentplot/agentplot.code-workspace`, listing `flume`
