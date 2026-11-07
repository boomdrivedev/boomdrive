# Configuration

## RENAMED Requirements

- FROM: `### Requirement: Whether each owner's views have a VS Code workspace is a setting`
- TO: `### Requirement: Whether each drive has a VS Code workspace is a setting`

## ADDED Requirements

### Requirement: The configuration names its accounts, each account's drives and each drive's repositories

The configuration SHALL name what the host keeps under `accounts`: a table of accounts, each holding `drives`, a table of drives, each holding `repositories`, the drive's repositories in the order kept. `accounts`, each account's `drives` and each drive's `repositories` SHALL be required, and each MAY be empty, so a host that keeps none says `accounts = {}`. The runner SHALL keep every drive's repositories, in the file's order (record 0028; core.5).

#### Scenario: One drive

- **WHEN** the configuration holds `[accounts.own.drives.home]` with `repositories = ["agentplot/flume"]`
- **THEN** it is read as keeping `agentplot/flume` in the drive `home` of the account `own`

#### Scenario: Drives of two accounts

- **WHEN** the account `own`'s drive `home` names `agentplot/flume` and `afterthought/crew`, and then the account `client`'s drive `garden` names `agentplot/sluice`
- **THEN** the runner keeps `agentplot/flume`, `afterthought/crew` and `agentplot/sluice`, in that order

#### Scenario: A drive with no list

- **WHEN** `[accounts.own.drives.home]` holds no `repositories`
- **THEN** it is refused, naming `accounts.own.drives.home.repositories` as required

#### Scenario: A drive keeping none

- **WHEN** the account `own`'s drive `home` says `repositories = []`
- **THEN** it is read with the drive `home` keeping no repository

### Requirement: An account's and a drive's name is of one form, and a drive's is given once on the host

An account's and a drive's name SHALL be 1 to 32 lower-case letters, digits and `-`, starting with a letter; any other SHALL be refused, naming it and the form. A drive's name is its stream's, its crew branch's and its workspace's, so it SHALL be given once on the host: a drive two accounts name SHALL be refused, naming the drive and both accounts (records 0017 and 0028).

#### Scenario: One drive in two accounts

- **WHEN** the accounts `own` and `client` each hold a drive `home`
- **THEN** it is refused, naming `home`, `own` and `client`

#### Scenario: A name out of form

- **WHEN** the configuration holds `[accounts.own.drives.Home]`, or `[accounts.my_own.drives.home]`
- **THEN** it is refused, naming `Home`, or `my_own`, and the form

## MODIFIED Requirements

### Requirement: The configuration names each repository by its forge address

Each entry of a drive's `repositories` SHALL be a GitHub address `owner/name`. The owner SHALL be letters, digits and inner hyphens. The name SHALL be letters, digits, `-`, `_` and `.`, but not `.`, not `..`, and not ending in `.git`. A repository SHALL belong to one drive: two entries that are equal ignoring case, in one drive or in two, SHALL be refused, naming both and their drives (record 0028).

#### Scenario: An address with no owner

- **WHEN** the drive `home` says `repositories = ["flume"]`
- **THEN** it is refused, naming the entry `flume` and the form `owner/name`

#### Scenario: One repository twice

- **WHEN** the drive `home` says `repositories = ["agentplot/flume", "AgentPlot/flume"]`
- **THEN** it is refused, naming both entries

#### Scenario: One repository in two drives

- **WHEN** the drive `home` names `agentplot/flume` and the drive `work` names `AgentPlot/flume`
- **THEN** it is refused, naming both entries and the drives `home` and `work`

#### Scenario: No list

- **WHEN** the configuration sets `repository_root` and has no `accounts`
- **THEN** it is refused, naming `accounts` as required

#### Scenario: Keeping none

- **WHEN** the configuration says `accounts = {}`
- **THEN** it is read as a host that keeps no repository

### Requirement: The configuration holds nothing the runner does not read

The configuration SHALL be refused, naming the key by its path, when it holds any key this spec does not name, at its top, in an account or in a drive. In particular, no key SHALL hold a token, key or credential: git's own credential helper on the host answers for every fetch (core.4).

#### Scenario: A misspelt key

- **WHEN** the configuration says `repositories_root = "~/Code"`
- **THEN** it is refused, naming `repositories_root` as a key the runner does not read

#### Scenario: A token

- **WHEN** the configuration says `token = "..."`
- **THEN** it is refused, naming `token` as a key the runner does not read, and the value appears in no output

#### Scenario: A key in a drive

- **WHEN** the account `own`'s drive `home` says `machine = "elsewhere"`
- **THEN** it is refused, naming `accounts.own.drives.home.machine` as a key the runner does not read

#### Scenario: A key in an account

- **WHEN** the account `own` says `state = "example/state"`
- **THEN** it is refused, naming `accounts.own.state` as a key the runner does not read

### Requirement: A refused stream, kind or key of before says where it is held

No key, at the top, in an account or in a drive, SHALL name a stream, a kind, a kinds file or a kind's source: these are each host's Flume configuration (core.5). A refusal of `stream` or `streams` SHALL say streams are, and one of `kind`, `kinds` or `kinds_file` that kinds are. A refusal of `views`, `fetch_every` or a top-level `repositories` SHALL name the key that holds it now: `repository_root`, `fetch_interval` or `accounts` (core.12; record 0028).

#### Scenario: A stream named

- **WHEN** the configuration says `stream = "work"`, or holds a `[streams]` table
- **THEN** it is refused, naming the key as one the runner does not read and saying streams are Flume's host configuration

#### Scenario: A stream in a drive

- **WHEN** the account `own`'s drive `home` says `stream = "home"`
- **THEN** it is refused, naming `accounts.own.drives.home.stream` as a key the runner does not read and saying streams are Flume's host configuration

#### Scenario: A kinds file named

- **WHEN** the configuration holds `kinds = { crew = "/k/kinds.toml" }`
- **THEN** it is refused, naming `kinds` as a key the runner does not read and saying kinds are Flume's host configuration

#### Scenario: A key of before

- **WHEN** the configuration says `views = "~/Code"` and `fetch_every = "2m"`
- **THEN** it is refused, naming `views` as held by `repository_root` and `fetch_every` as held by `fetch_interval`

#### Scenario: A flat list of before

- **WHEN** the configuration says `repositories = ["agentplot/flume"]` at its top
- **THEN** it is refused, naming `repositories` as held by `accounts`, each repository named under its drive

### Requirement: Whether each drive has a VS Code workspace is a setting

The configuration MAY set `code_workspaces`: true where the runner keeps a VS Code workspace for each drive of its configuration (views spec), false where it keeps none. Where it is absent, it SHALL be false. Any value but `true` or `false` SHALL be refused, naming the key and the value. It SHALL be read again at every pass, as `accounts` is.

#### Scenario: No workspaces setting

- **WHEN** the configuration keeps `agentplot/flume` and has no `code_workspaces`
- **THEN** it is read with `code_workspaces` false

#### Scenario: Workspaces asked for in words

- **WHEN** the configuration says `code_workspaces = "yes"`
- **THEN** it is refused, naming `code_workspaces` and saying it is `true` or `false`

#### Scenario: Workspaces asked for while running

- **WHEN** `code_workspaces = true` is added to the configuration of a running runner that keeps `agentplot/flume` in the drive `home`
- **THEN** the next pass writes `/h/Code/home.code-workspace`, listing `agentplot/flume`
