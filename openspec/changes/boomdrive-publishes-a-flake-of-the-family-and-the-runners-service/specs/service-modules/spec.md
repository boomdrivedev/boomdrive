# Service modules

## Purpose

The flake's nix-darwin and NixOS modules for the runner's service: their options, which are the runner's file under its own names, the file they render and link, the launchd agent and systemd user unit they keep and restart, the tools they put on `PATH`, and what they leave to the host, its credentials and enrollment among them (core.46; record 0020).

## ADDED Requirements

### Requirement: The options are the runner's file

`services.boomdrive.settings` SHALL hold the runner's file, each key named as the file names it, among them `repositories`, `repository_root`, `fetch_interval`, `code_workspaces` and `control_plane`. Each module SHALL render it as TOML at evaluation, a key written only where it is set and `repositories` always, so the runner's own defaults hold for the rest. One `settings` SHALL render one text on either platform.

#### Scenario: Only repositories

- **WHEN** `settings` sets nothing
- **THEN** the file's settings are `repositories = []` alone, after the module's header line

#### Scenario: A host under crew

- **WHEN** `settings` names `repository_root = "~/Code"`, `fetch_interval = "2m"`, `code_workspaces = true`, the repositories `agentplot/flume` and `agentplot/sluice`, and a crew control plane for the host `host-a` with the partition `home` at `example/state`
- **THEN** the file is `nix/tests/files/crew.toml`, byte for byte, on nix-darwin and on NixOS

#### Scenario: A host under a hosted control plane

- **WHEN** `settings` names a hosted control plane at `https://control.example` with the partitions `home`, serving `agentplot/flume`, and `work`, serving none
- **THEN** the file is `nix/tests/files/hosted.toml`, byte for byte

#### Scenario: The runner reads each file

- **WHEN** the runner reads each file under `nix/tests/files`
- **THEN** it reads it, refusing no key, with the repositories and control plane its `settings` named

### Requirement: No credential reaches the store

Evaluation SHALL fail, naming the key's path, where a key of `settings` at any depth is named `token`, `key`, `secret` or `password`, or ends in `_token`, `_key`, `_secret` or `_password`. No option of the modules SHALL hold a token, a key or an enrollment.

#### Scenario: A token in the hosted table

- **WHEN** `settings.control_plane.hosted.token` is set
- **THEN** evaluation fails, naming `control_plane.hosted.token` and saying a partition's key is made on the host by `boomdrive enroll`

### Requirement: On nix-darwin the runner is a launchd agent of the primary user's login

With `services.boomdrive.enable`, the nix-darwin module SHALL declare `launchd.user.agents.boomdrive`, loaded in `system.primaryUser`'s login. It SHALL run `<package>/bin/boomdrive run --config <the file>`, start at load, be kept alive at most once in 30 seconds, run as interactive, leave its process group when stopped, have only `HOME` and `PATH` in its environment, and log to `logFile`. Evaluation SHALL fail where the primary user has no home.

#### Scenario: The agent

- **WHEN** the module is enabled for the primary user `alice`, whose home is `/Users/alice`
- **THEN** the agent runs the package's `boomdrive run --config` with the file, `RunAtLoad`, `KeepAlive`, `ThrottleInterval` 30, `ProcessType` `Interactive` and `AbandonProcessGroup`, with `HOME` `/Users/alice`, and logs to `/Users/alice/Library/Logs/boomdrive.log`

#### Scenario: No home for the primary user

- **WHEN** the module is enabled and `users.users.alice.home` is not set
- **THEN** evaluation fails, naming the primary user and the option

### Requirement: On NixOS the runner is a systemd user unit of its user

With `services.boomdrive.enable`, the NixOS module SHALL declare `systemd.user.services.boomdrive` for `services.boomdrive.user` alone, wanted by `default.target`. It SHALL run `<package>/bin/boomdrive run --config <the file>`, restart always after 30 seconds with no limit on starts, and stop only its main process. The user SHALL linger, so the unit runs from boot. Where `settings` names a hosted control plane, evaluation SHALL fail on a systemd older than 256.

#### Scenario: The unit

- **WHEN** the module is enabled for the user `alice`
- **THEN** the unit runs the package's `boomdrive run --config` with the file, with `ConditionUser=alice`, `Restart=always`, `RestartSec=30s`, `StartLimitIntervalSec=0` and `KillMode=process`, and `alice` lingers

#### Scenario: A hosted control plane on an old systemd

- **WHEN** `settings` names a hosted control plane and the system's systemd is 255
- **THEN** evaluation fails, saying `systemd-creds` of systemd 256 or later keeps a partition's key

### Requirement: A changed file or package restarts the runner, and nothing else does

On nix-darwin, the agent's arguments SHALL name the package and the file, so nix-darwin reloads the agent when either changes and leaves it running when neither does. On NixOS, the module's user activation SHALL, for the service's user alone, start the unit where it is not running, restart it where the running runner's command line is not the unit's, and do nothing else.

#### Scenario: A new file on nix-darwin

- **WHEN** two configurations differ only in `settings.fetch_interval`
- **THEN** their agents' arguments differ, in the file alone

#### Scenario: Not running on NixOS

- **WHEN** the activation runs for `alice` and the unit is not running
- **THEN** it starts the unit

#### Scenario: Nothing changed on NixOS

- **WHEN** the activation runs for `alice` and the running runner's command line is the unit's
- **THEN** it neither starts nor restarts the unit

#### Scenario: A new file on NixOS

- **WHEN** the activation runs for `alice` and the running runner's command line names another file
- **THEN** it restarts the unit

#### Scenario: Another user's activation

- **WHEN** the activation runs for `bob`
- **THEN** it starts, restarts and links nothing

### Requirement: The file is at the runner's own path for commands run by hand

Each module's activation SHALL link the service's user's `~/.config/boomdrive/config.toml` to the module's file, so `boomdrive enroll` and `boomdrive run --once` run by hand read what the service reads. A link there SHALL be replaced. A file or folder there that is no link SHALL be moved aside once, to `config.toml.before-nix`, and named in the activation's output.

#### Scenario: An empty home

- **WHEN** the activation runs for a home with no `.config/boomdrive`
- **THEN** `~/.config/boomdrive/config.toml` is a link to the module's file

#### Scenario: A person's own file

- **WHEN** the activation finds a file at `~/.config/boomdrive/config.toml` that is no link
- **THEN** it moves it to `config.toml.before-nix`, links the module's file, and names both paths

#### Scenario: Already linked

- **WHEN** the activation finds the link to the module's file
- **THEN** it changes nothing and says nothing

### Requirement: The runner finds its tools on the profiles' PATH

Each module SHALL put `package` and each of `tools` in the system profile. `tools` SHALL default to the set's Flume and sluice for the host's system, and `[ ]` SHALL install none. The service's `PATH` SHALL be the user's profiles, then the system's, then the operating system's folders, so the runner, what it launches and a person's shell run one copy of each tool. git and herdr SHALL be left to the host.

#### Scenario: The set's tools

- **WHEN** the module is enabled with no `tools`
- **THEN** the system profile holds the package, the set's Flume and the set's sluice

#### Scenario: A host's own tools

- **WHEN** the module is enabled with `tools = [ ]`
- **THEN** the system profile holds the package and no Flume or sluice of the module's

#### Scenario: The user's profile first

- **WHEN** the module is enabled for `alice`
- **THEN** the service's `PATH` names `/etc/profiles/per-user/alice/bin` and `alice`'s `~/.nix-profile/bin` before the system profile's `bin`, and holds no package's own folder
