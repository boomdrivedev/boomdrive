# configuration Specification

## Purpose
The runner's configuration on a host is one TOML file. It names the repositories the host keeps by their forge address, where their views go and how often they are fetched. This spec covers where the file is read from, what it may say, and its defaults and refusals. It also requires that a file which cannot be read never counts as one that names no repository (core.4, core.5; crew record 0052 part 2).

## Requirements

### Requirement: The configuration is one file the runner reads

The runner SHALL read its configuration from the file `--config <file>` names. Where none is named, it SHALL read `$XDG_CONFIG_HOME/flywheel/config.toml`, or `~/.config/flywheel/config.toml` where `XDG_CONFIG_HOME` is unset or empty. A file that does not exist or is not valid TOML SHALL be refused. The refusal SHALL name the file, the line and what is wrong, and SHALL never quote the file's text.

#### Scenario: The default file

- **WHEN** `flywheel run --once` runs with no `--config`, `XDG_CONFIG_HOME` unset and `HOME` set to `/h`
- **THEN** it reads `/h/.config/flywheel/config.toml`

#### Scenario: The file named

- **WHEN** `flywheel run --once --config /etc/flywheel.toml` runs with `XDG_CONFIG_HOME` set
- **THEN** it reads `/etc/flywheel.toml` and no other file

#### Scenario: No file

- **WHEN** `flywheel run` starts and the configuration file does not exist
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

- **WHEN** the configuration sets `views` and has no `repositories`
- **THEN** it is refused, naming `repositories` as required

#### Scenario: Keeping none

- **WHEN** the configuration says `repositories = []`
- **THEN** it is read as a host that keeps no repository

### Requirement: The views root is a setting with a default

The configuration SHALL set where views are kept with `views`. The value SHALL be an absolute path or one that begins with `~/`, where `~` is the host user's home. Where `views` is absent, the views root SHALL be `~/Code`. Any other value, a relative path or one naming a variable among them, SHALL be refused, naming it.

#### Scenario: No views setting

- **WHEN** the configuration names `agentplot/flume` and no `views`, and `HOME` is `/h`
- **THEN** the repository's view is `/h/Code/agentplot/flume`

#### Scenario: A home-relative root

- **WHEN** the configuration says `views = "~/src"` and `HOME` is `/h`
- **THEN** the views root is `/h/src`

#### Scenario: A relative root

- **WHEN** the configuration says `views = "Code"` or `views = "$HOME/Code"`
- **THEN** it is refused, naming the value and saying a views root is absolute or begins with `~/`

### Requirement: How often repositories are fetched is a setting with a default

The configuration SHALL set how often every kept repository is fetched with `fetch_every`: a whole number of seconds, minutes or hours, written `<n>s`, `<n>m` or `<n>h`, more than zero. Where it is absent, the period SHALL be two minutes. Any other value SHALL be refused, naming it.

#### Scenario: No period

- **WHEN** the configuration has no `fetch_every`
- **THEN** the runner fetches every two minutes

#### Scenario: A period of nothing

- **WHEN** the configuration says `fetch_every = "0m"` or `fetch_every = "soon"`
- **THEN** it is refused, naming the value and the form `<n>s`, `<n>m` or `<n>h`

### Requirement: The configuration holds nothing the runner does not read

The configuration SHALL be refused, naming the key, when it holds any key this spec does not name. In particular, no key SHALL hold a token, key or credential: git's own credential helper on the host answers for every fetch (core.4).

#### Scenario: A misspelt key

- **WHEN** the configuration says `view = "~/Code"`
- **THEN** it is refused, naming `view` as a key the runner does not read

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
