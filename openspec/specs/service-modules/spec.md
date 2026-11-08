# service-modules Specification

## Purpose
The flake's nix-darwin and NixOS modules for the runner's service: their options, which are the runner's file under its own names, the file they render and link, the launchd agent and systemd user unit they keep and restart, the tools they put on `PATH`, and what they leave to the host, its credentials and enrollment among them (core.46; record 0020).

## Requirements

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

On nix-darwin, the agent's program SHALL name the package and the file, so nix-darwin, which reloads an agent whose plist changed, reloads it when either changes and leaves it running when neither does. On NixOS, the module's user activation SHALL, for the service's user alone, start the unit where it is not running, restart it where the running runner's command line is not the unit's, and do nothing else.

#### Scenario: A new file on nix-darwin

- **WHEN** two configurations differ only in `settings.fetch_interval`
- **THEN** their agents' programs differ, in the file alone, and their plists differ

#### Scenario: A new package on nix-darwin

- **WHEN** two configurations differ only in `package`
- **THEN** their agents' programs differ, in the package alone, and their plists differ

#### Scenario: Nothing else changed on nix-darwin

- **WHEN** two configurations differ only in `services.flume.package`
- **THEN** their agents' plists are the same, byte for byte

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

#### Scenario: A file moved aside before

- **WHEN** the activation finds a file at `~/.config/boomdrive/config.toml` that is no link, and one at `config.toml.before-nix`
- **THEN** it changes neither, names both paths, and fails the link alone

#### Scenario: Already linked

- **WHEN** the activation finds the link to the module's file
- **THEN** it changes nothing and says nothing

### Requirement: The runner finds its tools on the profiles' PATH

Each module SHALL put `package` in the system profile, and each tool's own module SHALL put that tool's package there, the set's unless the host gives its own. The service's `PATH` SHALL be the user's profiles, then the system's, then the operating system's folders, so the runner, what it launches and a person's shell run one copy of each tool. git and herdr SHALL be left to the host.

#### Scenario: The set's tools

- **WHEN** the module is enabled with no tool's package named
- **THEN** the system profile holds the package, the set's Flume and the set's sluice

#### Scenario: A host's own tools

- **WHEN** the module is enabled with `services.flume.package` and `services.sluice.package` each `null`
- **THEN** the system profile holds the package and no Flume or sluice of the set's

#### Scenario: The user's profile first

- **WHEN** the module is enabled for `alice`
- **THEN** the service's `PATH` names `/etc/profiles/per-user/alice/bin` and `alice`'s `~/.nix-profile/bin` before the system profile's `bin`, and holds no package's own folder

### Requirement: On nix-darwin the agent waits for the Nix store

The nix-darwin module's agent SHALL run its command behind `/bin/wait4path /nix/store`, the shell handing its process to the runner by `exec`, so an agent loaded before the Nix store's volume is mounted waits for it, and a stop's signal reaches the runner.

#### Scenario: The agent waits for the Nix store

- **WHEN** the module is enabled, with no `settings` or with a crew control plane's
- **THEN** the agent's program is `/bin/sh -c` with a line that runs `/bin/wait4path /nix/store`, then `exec`s the package's `boomdrive run --config` with the file

### Requirement: The modules bring each tool's own module

`darwinModules.default` and `nixosModules.default`, each the same value as the platform's `boomdrive`, SHALL import the runner's module and Flume's, sluice's and crew's, each the module that tool's flake gives at the release tag the lock names. No file of this tree SHALL declare an option of `services.flume`, `services.sluice` or `services.crew`.

#### Scenario: Each tool's options are its own flake's

- **WHEN** a system imports the platform's `default` module
- **THEN** every option of `services.flume`, `services.sluice` and `services.crew` is declared in a file of that tool's input, and none in this tree, on nix-darwin and on NixOS

### Requirement: The flake re-exports each tool's module

The flake SHALL give each of Flume's, sluice's and crew's modules, as that tool's flake gives it, as `darwinModules.<tool>` and `nixosModules.<tool>`, beside `boomdrive` and `default`.

#### Scenario: The tools' modules re-exported

- **WHEN** the flake's `darwinModules` and `nixosModules` are listed, and a system imports the platform's `sluice` module alone
- **THEN** each gives `boomdrive`, `crew`, `default`, `flume` and `sluice`, and the system declares `services.sluice` from sluice's input and no `services.boomdrive`

### Requirement: The Boomdrive module is taken once however often it is imported

The platform's `boomdrive` module SHALL carry the key `github:agentplot/boomdrive#<platform>Modules.boomdrive`, so a system that imports it more than once, as `default` and as `boomdrive`, takes it and the tools' modules it imports once.

#### Scenario: Imported twice

- **WHEN** a system imports the platform's `default` module and its `boomdrive` module
- **THEN** it evaluates, failing no assertion, on nix-darwin and on NixOS

### Requirement: The Boomdrive module turns on Flume's and sluice's modules and leaves crew's to the host

With `services.boomdrive.enable`, the modules SHALL set `services.flume.enable` and `services.sluice.enable` true by default, a host's own value winning. They SHALL leave `services.crew.enable` to the host. On NixOS they SHALL set `services.flume.user`, `services.sluice.user` and `services.crew.user` to `services.boomdrive.user` by default, a host's own value winning.

#### Scenario: A host with the runner alone

- **WHEN** a system turns on `services.boomdrive` and sets no option of Flume's, sluice's or crew's
- **THEN** `services.flume.enable` and `services.sluice.enable` are true, `services.crew.enable` is false, and the system fails no assertion, on nix-darwin and on NixOS

#### Scenario: A host that keeps its own Flume file

- **WHEN** a system turns on `services.boomdrive` and sets `services.flume.enable = false`
- **THEN** its activation links no file of Flume's, and its system profile holds no Flume of the set's

#### Scenario: The tools' user on NixOS

- **WHEN** the NixOS module is enabled for `alice`, with crew turned on with its host and partitions, and again with `services.sluice.user = "bob"`
- **THEN** Flume's, sluice's and crew's `user` are `alice` in the first, and sluice's is `bob` in the second

### Requirement: Each tool's package is the set's unless the host gives its own

The modules SHALL set `services.flume.package`, `services.sluice.package` and `services.crew.package` to the set's package of that tool for the host's system by default, whether or not the runner is enabled. A host's own value SHALL win, and SHALL change nothing of the tool's file.

#### Scenario: The set's packages

- **WHEN** a system turns on `services.boomdrive` and crew with its host and partitions, naming no tool's package
- **THEN** the system profile holds the set's `boomdrive`, `flume`, `sluice` and `crew`, on nix-darwin and on NixOS

#### Scenario: A host's own crew

- **WHEN** `services.crew.package` is a package of the host's own
- **THEN** the system profile holds it and not the set's crew, and crew's activation links the same file it links with the set's

#### Scenario: No sluice of the set's

- **WHEN** `services.sluice.package` is `null`
- **THEN** the system profile holds no sluice of the set's, and sluice's file is still linked

### Requirement: The Boomdrive module writes no file of the other tools'

The modules' own activation SHALL link the runner's file alone. Flume's, sluice's and crew's files SHALL each be written and linked by that tool's own module, from its own `settings`, as it writes them when imported without Boomdrive's.

#### Scenario: The tools' activation is their own

- **WHEN** a system with `services.boomdrive` and crew turned on is set beside one that imports Flume's, sluice's and crew's own modules alone, each turned on with the same settings, packages and user
- **THEN** on nix-darwin their activations are the same but for the runner's own link, and on NixOS their user activations are the same but for the runner's own

### Requirement: The modules hold no option that installs another tool

The modules SHALL hold no option of their own that installs another tool: setting `services.boomdrive.tools` SHALL fail evaluation, naming `services.flume.package` and `services.sluice.package`. The README's install with nix SHALL name each tool's `package` option, and no `services.boomdrive.tools`.

#### Scenario: The tools option is gone

- **WHEN** a system sets `services.boomdrive.tools = [ ]`
- **THEN** evaluation fails, naming `services.flume.package` and `services.sluice.package`

#### Scenario: The README's words

- **WHEN** the README's install with nix is read
- **THEN** it names `services.flume.package`, `services.sluice.package` and `services.crew.package`, and no `services.boomdrive.tools`
