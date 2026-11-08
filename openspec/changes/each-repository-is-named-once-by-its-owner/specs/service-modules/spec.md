## MODIFIED Requirements

### Requirement: The options are the runner's file, its accounts always written

`services.boomdrive.accounts` and `services.boomdrive.settings` SHALL hold the runner's file: `accounts` its `accounts`, each drive's repositories named by owner, and `settings` every other key, each named as the file names it, among them `repository_root`, `fetch_interval`, `code_workspaces` and `control_plane`. Each module SHALL render them as TOML at evaluation, a key of `settings` written only where it is set, and `accounts`, each account's `drives` and each drive's `repositories` always, so the runner's own defaults hold for the rest. One `accounts` and `settings` SHALL render one text on either platform.

#### Scenario: Only accounts

- **WHEN** neither `accounts` nor `settings` sets anything
- **THEN** the file's settings are an empty `[accounts]` table alone, after the module's header line

#### Scenario: A host under crew

- **WHEN** `accounts` names the account `own`, whose drive `home` keeps the owner `agentplot`'s `flume` and `sluice`, and `settings` names `repository_root = "~/Code"`, `fetch_interval = "2m"`, `code_workspaces = true` and a crew control plane for the host `host-a` with the partition `home` at `example/state`
- **THEN** the file is `nix/tests/files/crew.toml`, byte for byte, on nix-darwin and on NixOS

#### Scenario: A host under a hosted control plane

- **WHEN** `accounts` names the account `home`, whose drive `garden` keeps the owner `agentplot`'s `flume`, and the account `work` with no drive, and `settings` names a hosted control plane at `https://control.example` with the partitions `home`, serving `agentplot/flume`, and `work`, serving none
- **THEN** the file is `nix/tests/files/hosted.toml`, byte for byte

#### Scenario: The runner reads each file

- **WHEN** the runner reads each file under `nix/tests/files`
- **THEN** it reads it, refusing no key, with the accounts, drives, repositories and control plane its `accounts` and `settings` named

### Requirement: A flat list of before fails evaluation

Evaluation SHALL fail where `settings` names `repositories` at its top, saying each repository is named under its drive and its owner, `services.boomdrive.accounts.<account>.drives.<drive>.repositories.<owner>` (record 0028), and where `settings` names `accounts`, naming `services.boomdrive.accounts`, so each repository is named in one place. A key in an account or a drive of `accounts` that the module does not declare SHALL fail evaluation, so no file holding it is written.

#### Scenario: The flat list in settings

- **WHEN** `settings.repositories = [ "agentplot/flume" ]` is set
- **THEN** evaluation fails, naming `settings.repositories` and `services.boomdrive.accounts.<account>.drives.<drive>.repositories.<owner>`

#### Scenario: A stream in a drive's settings

- **WHEN** `settings.accounts.own.drives.home.stream = "home"` is set
- **THEN** evaluation fails, naming `settings.accounts` and `services.boomdrive.accounts`

#### Scenario: A key a drive does not hold

- **WHEN** `accounts.own.drives.home.pool = "home"` is set
- **THEN** evaluation fails, and no file is rendered

## ADDED Requirements

### Requirement: Each repository is named once, under its drive and its owner

`services.boomdrive.accounts.<account>.drives.<drive>.repositories.<owner>` SHALL list the names of the owner's repositories the drive keeps. The module SHALL write the drive's `repositories` in the runner's file as their addresses, `<owner>/<name>`, owner by owner in the order of the owners' names, each owner's names in the order given. An owner and a name that make no address of a GitHub repository SHALL fail evaluation, naming the address.

#### Scenario: Owners turned into addresses

- **WHEN** the drive `home` of the account `own` keeps the owner `example`'s `beta` and `alpha`, and the owner `agentplot`'s `flume`
- **THEN** the file's `[accounts.own.drives.home]` holds `repositories = ["agentplot/flume", "example/beta", "example/alpha"]`

#### Scenario: An address of no repository

- **WHEN** the drive `home` of the account `own` keeps the owner `example/inner`'s `alpha`
- **THEN** evaluation fails, naming `example/inner/alpha`

### Requirement: Each drive of this host gives Flume its stream

With `services.boomdrive.enable`, each drive of this host SHALL give Flume's `settings.streams.<stream>`, `<stream>` being its `stream.name`, the drive's name by default: `agents`, its `stream.agents`, 4 by default; `priority`, its `stream.priority`, unset by default; and each label's `reserve`, its `stream.labels.<label>.reserve`, `operation` 1 and `planning` 2 by default, each kept where the drive sets another label. It SHALL give no other key of a stream.

#### Scenario: A drive's stream by default

- **WHEN** the drive `home` of the account `own` keeps the owner `agentplot`'s `flume` and sets no key of its stream
- **THEN** Flume's `settings.streams.home` gives `agents` 4, `labels.operation.reserve` 1 and `labels.planning.reserve` 2, and no `priority`, on nix-darwin and on NixOS

#### Scenario: A drive that writes only what it changes

- **WHEN** the drive `home` sets `stream.priority = 11`, `stream.labels.planning.reserve = 1` and `stream.labels.review.reserve = 1`
- **THEN** Flume's `settings.streams.home` gives `agents` 4, `priority` 11, and the reserves `operation` 1, `planning` 1 and `review` 1

#### Scenario: A stream named other than its drive

- **WHEN** the drive `home` keeps the owner `agentplot`'s `flume` and sets `stream.name = "garden"`
- **THEN** Flume's `settings.streams` names `garden` and no `home`, and Flume's `settings.repos."agentplot/flume".stream` is `garden`

### Requirement: A drive's stream is Flume's alone, and only with the runner

The module SHALL give Flume nothing of a drive where `services.boomdrive.enable` is false, and a drive's stream SHALL change nothing of the runner's file.

#### Scenario: With the runner off

- **WHEN** `accounts` names the drive `home` keeping the owner `agentplot`'s `flume`, and `services.boomdrive.enable` is false
- **THEN** Flume's `settings` holds no stream and no repository's table

#### Scenario: A drive's stream changes nothing of the runner's

- **WHEN** two configurations differ only in the drive `home`'s `stream.agents`
- **THEN** their runner's files are the same text, and on nix-darwin their agents' plists are the same, byte for byte

### Requirement: Each repository is given its drive's stream in Flume's settings

With `services.boomdrive.enable`, for each drive of this host, the module SHALL give Flume's `settings.repos."<owner>/<name>".stream` the drive's stream, for each address the drive keeps. It SHALL name each repository by its address and never by a folder, and SHALL give no other key of a repository's table.

#### Scenario: Each address given its drive's stream

- **WHEN** the account `own`'s drive `home` keeps the owner `agentplot`'s `flume` and `sluice`, and the account `work`'s drive `garden` keeps the owner `example`'s `alpha`
- **THEN** Flume's `settings.repos` names exactly `agentplot/flume` and `agentplot/sluice`, each with the stream `home`, and `example/alpha`, with the stream `garden`, on nix-darwin and on NixOS

### Requirement: A drive of another machine gives Flume nothing

A drive's `machine` SHALL name the host its work runs on, as crew's partitions name a host. A drive whose `machine` is unset or is `services.crew.host` is a drive of this host. A drive whose `machine` names another host SHALL give Flume no stream and no repository's table, and its repositories SHALL still be written in the runner's file.

#### Scenario: A drive of another machine

- **WHEN** `services.crew.host` is `host-a`, and the drive `garden` of the account `work` sets `machine = "host-b"` and keeps the owner `example`'s `alpha`
- **THEN** Flume's `settings` names no stream `garden` and no `example/alpha`, and the runner's file's `[accounts.work.drives.garden]` holds `repositories = ["example/alpha"]`

#### Scenario: A drive of this machine

- **WHEN** `services.crew.host` is `host-a`, and the drive `garden` sets `machine = "host-a"` and keeps the owner `example`'s `alpha`
- **THEN** Flume's `settings.streams` names `garden`, and `settings.repos."example/alpha".stream` is `garden`

### Requirement: A drive that names a machine needs crew's host

Evaluation SHALL fail where a drive names a `machine` and `services.crew.host` is not set, naming `services.crew.host` and the drive.

#### Scenario: No host to compare with

- **WHEN** the drive `garden` of the account `work` sets `machine = "host-b"`, and `services.crew.host` is not set
- **THEN** evaluation fails, naming `services.crew.host` and `accounts.work.drives.garden.machine`

### Requirement: A host's own Flume settings win over the module's

The module SHALL give each stream's keys and each repository's `stream` to Flume's options at their default priority, so a value a host sets in `services.flume.settings` wins, and a key of a stream the module does not give, set there, is kept beside the module's.

#### Scenario: A host's own room and rule

- **WHEN** the drive `home` keeps the owner `agentplot`'s `flume` and sets no key of its stream, and the host sets `services.flume.settings.streams.home.agents = 6`, `services.flume.settings.streams.home.labels.operation.keep_free = 1` and `services.flume.settings.repos."agentplot/flume".stream = "other"`
- **THEN** Flume's `settings.streams.home` gives `agents` 6, the reserves `operation` 1 and `planning` 2, and `labels.operation.keep_free` 1, `settings.repos."agentplot/flume".stream` is `other`, and the system fails no assertion

### Requirement: A drive, a repository and a stream are each given once

Evaluation SHALL fail where one drive's name is given under two accounts, where one address is kept by two drives or twice by one, or where one stream is given by two drives, each failure naming what is given twice.

#### Scenario: A drive's name under two accounts

- **WHEN** the accounts `own` and `work` each name the drive `home`
- **THEN** evaluation fails, naming `home`, `own` and `work`

#### Scenario: A repository in two drives

- **WHEN** the drives `home` and `garden` each keep the owner `agentplot`'s `flume`
- **THEN** evaluation fails, naming `agentplot/flume`, `home` and `garden`

#### Scenario: One stream for two drives

- **WHEN** the drive `garden` sets `stream.name = "home"` beside the drive `home`
- **THEN** evaluation fails, naming the stream `home` and the drives `home` and `garden`

### Requirement: The README names each drive's repositories by owner

The README's install with nix SHALL name `services.boomdrive.accounts`, each drive's repositories by owner and its stream's keys, and Flume's own `services.flume.settings.streams` for any other key of a stream, and SHALL name no `settings.accounts`.

#### Scenario: The README's accounts

- **WHEN** the README's install with nix is read
- **THEN** it names `services.boomdrive.accounts` and `services.flume.settings.streams`, and no `settings.accounts`
