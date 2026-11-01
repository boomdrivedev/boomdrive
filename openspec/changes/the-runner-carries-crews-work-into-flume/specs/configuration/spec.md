# Configuration

## RENAMED Requirements

- FROM: `### Requirement: The views root is a setting with a default`
- TO: `### Requirement: The repository root is a setting with a default`

## MODIFIED Requirements

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

The configuration SHALL be refused, naming the key, when it holds any key this spec does not name. In particular:

- no key SHALL hold a token, key or credential: git's own credential helper on the host answers for every fetch (core.4);
- no key SHALL name a stream: streams, and the stream a repository's launches run in, are each host's Flume configuration, and a refusal of `stream` or `streams` SHALL say so (core.5);
- no key SHALL name a kind, a kinds file or a kind's source: where a host's kinds come from is its Flume configuration, and a refusal of `kind`, `kinds` or `kinds_file` SHALL say so (core.5);
- a refusal of `views` or `fetch_every` SHALL name the key that holds it, `repository_root` or `fetch_interval` (core.12).

#### Scenario: A misspelt key

- **WHEN** the configuration says `repositories_root = "~/Code"`
- **THEN** it is refused, naming `repositories_root` as a key the runner does not read

#### Scenario: A token

- **WHEN** the configuration says `token = "..."`
- **THEN** it is refused, naming `token` as a key the runner does not read, and the value appears in no output

#### Scenario: A stream named

- **WHEN** the configuration says `stream = "work"`, or holds a `[streams]` table
- **THEN** it is refused, naming the key as one the runner does not read and saying streams are Flume's host configuration

#### Scenario: A kinds file named

- **WHEN** the configuration holds `kinds = { crew = "/k/kinds.toml" }`
- **THEN** it is refused, naming `kinds` as a key the runner does not read and saying kinds are Flume's host configuration

#### Scenario: A key of before

- **WHEN** the configuration says `views = "~/Code"` and `fetch_every = "2m"`
- **THEN** it is refused, naming `views` as held by `repository_root` and `fetch_every` as held by `fetch_interval`

## ADDED Requirements

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
- **THEN** `flywheel run --once` keeps the repositories and runs no `flume` command

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
