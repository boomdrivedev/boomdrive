## MODIFIED Requirements

### Requirement: Each drive of this host gives Flume its stream

With `services.boomdrive.enable`, each drive of this host SHALL give Flume's `settings.streams.<stream>`, `<stream>` being its `stream.name`, the drive's name by default: `agents`, its `stream.agents`, from 1, 4 by default; `priority`, its `stream.priority`, unset by default; and each label's `owed`, its `stream.labels.<label>.owed`, `operation` 1 and `planning` 2 by default, each kept where the drive sets another label. It SHALL give no other key of a stream.

#### Scenario: A drive's stream by default

- **WHEN** the drive `home` of the account `own` keeps the owner `agentplot`'s `flume` and sets no key of its stream
- **THEN** Flume's `settings.streams.home` gives `agents` 4, `labels.operation.owed` 1 and `labels.planning.owed` 2, and no `priority`, on nix-darwin and on NixOS

#### Scenario: A drive that writes only what it changes

- **WHEN** the drive `home` sets `stream.priority = 11`, `stream.labels.planning.owed = 1` and `stream.labels.review.owed = 1`
- **THEN** Flume's `settings.streams.home` gives `agents` 4, `priority` 11, and the labels owed `operation` 1, `planning` 1 and `review` 1

#### Scenario: A stream named other than its drive

- **WHEN** the drive `home` keeps the owner `agentplot`'s `flume` and sets `stream.name = "garden"`
- **THEN** Flume's `settings.streams` names `garden` and no `home`, and Flume's `settings.repos."agentplot/flume".stream` is `garden`

#### Scenario: A stream of no agents

- **WHEN** the drive `home` sets `stream.agents = 0`
- **THEN** evaluation fails at the drive's `stream.agents`, on nix-darwin and on NixOS

### Requirement: A host's own Flume settings win over the module's

The module SHALL give each stream's keys and each repository's `stream` to Flume's options at their default priority, so a value a host sets in `services.flume.settings` wins, and a key of a stream the module does not give, set there, is kept beside the module's.

#### Scenario: A host's own room and rule

- **WHEN** the drive `home` keeps the owner `agentplot`'s `flume` and sets no key of its stream, and the host sets `services.flume.settings.streams.home.agents = 6`, `services.flume.settings.streams.home.labels.operation.reserved = 1` and `services.flume.settings.repos."agentplot/flume".stream = "other"`
- **THEN** Flume's `settings.streams.home` gives `agents` 6, the labels owed `operation` 1 and `planning` 2, and `labels.operation.reserved` 1, `settings.repos."agentplot/flume".stream` is `other`, and the system fails no assertion

## ADDED Requirements

### Requirement: A label's `reserve` fails evaluation, naming `owed`

Evaluation SHALL fail where a drive's label sets `reserve`, Flume's word before `owed` (Flume record 0063), whether or not the runner is enabled, naming each such option and `owed`, and the module SHALL give Flume nothing of that label.

#### Scenario: A label's old word

- **WHEN** the drive `home` of the account `own` sets `stream.labels.review.reserve = 1`, with the runner enabled and again with it not enabled
- **THEN** evaluation fails, naming `accounts.own.drives.home.stream.labels.review.reserve` and saying to set `owed` in its place, on nix-darwin and on NixOS
