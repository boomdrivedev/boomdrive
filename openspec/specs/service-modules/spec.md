# service-modules Specification

## Purpose
The flake's nix-darwin and NixOS modules for the runner's service: their options, which are the runner's file under its own names, the file they render and link, the launchd agent and systemd user unit they keep and restart, the tools they put on `PATH`, and what they leave to the host, its credentials and enrollment among them (core.46; record 0020).

## Requirements

### Requirement: No credential reaches the store

Evaluation SHALL fail, naming the key's path, where a key of `settings` at any depth is named `token`, `key`, `secret` or `password`, or ends in `_token`, `_key`, `_secret` or `_password`. No option of the modules SHALL hold a token, a key or an enrollment.

#### Scenario: A token in the hosted table

- **WHEN** `settings.control_plane.hosted.token` is set
- **THEN** evaluation fails, naming `control_plane.hosted.token` and saying a partition's key is made on the host by `boomdrive enroll`

### Requirement: On nix-darwin the runner is a launchd agent of the primary user's login

With `services.boomdrive.enable`, the nix-darwin module SHALL declare `launchd.user.agents.boomdrive`, loaded in `system.primaryUser`'s login. It SHALL run `<package>/bin/boomdrive run --config <the file>`, start at load, be kept alive at most once in 30 seconds, run as interactive, leave its process group when stopped, be given `HOME` and `PATH` alone by the module, beside any variable the host gives the job, and log to `logFile`. Evaluation SHALL fail where the primary user has no home.

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

Each module's activation SHALL link the service's user's `~/.config/boomdrive/config.toml` to the module's file, so `boomdrive enroll` and `boomdrive run --once` run by hand read what the service reads. A link there SHALL be replaced, and a file or folder there that is no link SHALL be moved aside once, to `config.toml.before-nix`, and named in its output. It SHALL leave the runner's local layer, `config.local.toml` beside it, as it is (record 0009 as amended 2026-11-06; core.18).

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

#### Scenario: A local layer beside it

- **WHEN** the activation runs for a home whose `.config/boomdrive` holds a `config.local.toml`
- **THEN** it links `config.toml` to the module's file, and `config.local.toml` is there as it was, its text unchanged

### Requirement: The runner finds its tools on the profiles' PATH

Each module SHALL put `package` in the system profile, and each tool's own module SHALL put that tool's package there, the set's unless the host gives its own. The service's `PATH` SHALL be the user's profiles, then the system's, then the operating system's folders, so the runner, what it launches and a person's shell run one copy of each tool. git SHALL be left to the host.

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

### Requirement: The options are the runner's file, its accounts always written

`services.boomdrive.accounts` and `services.boomdrive.settings` SHALL hold the runner's file: `accounts` its `accounts`, and `settings` every other key, each named as the file names it. Each module SHALL render them as TOML at evaluation, a key of `settings` written only where it is set, so the runner's own default holds, and `accounts`, each account's `drives` and each drive's `repositories` always. One `accounts` and `settings` SHALL render one text on either platform.

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

The modules' own activation SHALL link the runner's file, run herdr's own `plugin link` for the plugins `services.boomdrive.herdr.plugins` lists, and do nothing else of a tool's. Flume's, sluice's and crew's files SHALL each be written and linked by that tool's own module, from its own `settings`, as it writes them when imported without Boomdrive's.

#### Scenario: The tools' activation is their own

- **WHEN** a system with `services.boomdrive` and crew turned on is set beside one that imports Flume's, sluice's and crew's own modules alone, each turned on with the same settings, packages and user
- **THEN** on nix-darwin their activations are the same but for the runner's own link and herdr's plugins' links, and on NixOS their user activations are the same but for the runner's own

### Requirement: The modules hold no option that installs a tool its own module installs

The modules SHALL hold no option of their own that installs a tool whose own module installs it: setting `services.boomdrive.tools` SHALL fail evaluation, naming `services.flume.package` and `services.sluice.package`. Their own options SHALL install herdr, Ghostty and the launcher. The README's install with nix SHALL name each option that installs a tool of the set, and the shell, and no `services.boomdrive.tools`.

#### Scenario: The tools option is gone

- **WHEN** a system sets `services.boomdrive.tools = [ ]`
- **THEN** evaluation fails, naming `services.flume.package` and `services.sluice.package`

#### Scenario: The README's words

- **WHEN** the README's install with nix is read
- **THEN** it names `services.flume.package`, `services.sluice.package`, `services.crew.package`, `services.boomdrive.herdr.package`, `services.boomdrive.herdr.plugins`, `services.boomdrive.ghostty.package`, `services.boomdrive.launcher.package` and `nix develop github:agentplot/boomdrive`, and no `services.boomdrive.tools`

### Requirement: Under crew, the runner's crew table is written from crew's partitions on this host

Where `settings.control_plane.type` is `crew`, crew's module is on and the host writes no `settings.control_plane.crew`, the modules SHALL write it: `host` as `services.crew.host`, and one entry of `partitions` for each of crew's partitions whose `machine` is that host, in crew's order, each its `label` and its `state` as `state_repository` alone. `settings.control_plane.crew` SHALL read that table (record 0023; core.18).

#### Scenario: crew's partitions on this host

- **WHEN** `accounts` names the account `own`, whose drive `home` keeps the owner `agentplot`'s `flume` and `sluice`, and `settings` names `repository_root = "~/Code"`, `fetch_interval = "2m"`, `code_workspaces = true` and `control_plane.type = "crew"` with no `control_plane.crew`, and crew's module is on with the host `host-a` and the partitions `home`, whose `machine` is `host-a` and `state` `example/state`, and `away`, whose `machine` is `host-b` and `state` `example/away`
- **THEN** the file is `nix/tests/files/crew.toml`, byte for byte, on nix-darwin and on NixOS

#### Scenario: crew's order

- **WHEN** `control_plane.type = "crew"` is set with no `control_plane.crew`, and crew's module is on with the host `host-a` and the partitions `garden` on `host-a` at `example/garden`, `away` on `host-b` at `example/away` and `home` on `host-a` at `example/state`, in that order
- **THEN** `settings.control_plane.crew` is the host `host-a` with the partitions `garden` at `example/garden`, then `home` at `example/state`, each holding `label` and `state_repository` alone, on nix-darwin and on NixOS

### Requirement: A crew table written out wins whole

Where the host writes `settings.control_plane.crew`, the file's `[control_plane.crew]` SHALL be that table, whole, with none of crew's partitions merged into it.

#### Scenario: A table written out wins

- **WHEN** `settings` names a crew control plane whose table is the host `host-a` with the partition `garden` at `example/garden` alone, and crew's module is on with the host `host-a` and the partitions `home` on `host-a` and `away` on `host-b`
- **THEN** `settings.control_plane.crew` is the table written, the partition `garden` alone, and the file is that table's rendering, on nix-darwin and on NixOS

### Requirement: No crew table beside another control plane

Where `settings` names no control plane, or one whose `type` is not `crew`, the modules SHALL write no `[control_plane.crew]`, whatever crew's module holds.

#### Scenario: A hosted control plane beside crew

- **WHEN** `accounts` names the account `home`, whose drive `garden` keeps the owner `agentplot`'s `flume`, and the account `work` with no drive, and `settings` names a hosted control plane at `https://control.example` with the partitions `home` and `work`, and crew's module is on with the host `host-a` and the partition `home` on `host-a`
- **THEN** the file is `nix/tests/files/hosted.toml`, byte for byte, on nix-darwin and on NixOS

#### Scenario: No control plane beside crew

- **WHEN** neither `accounts` nor `settings` sets anything, and crew's module is on with the host `host-a` and the partition `home` on `host-a`
- **THEN** the file is `nix/tests/files/empty.toml`, byte for byte, on nix-darwin and on NixOS

### Requirement: A host under crew that the runner cannot serve fails evaluation

With `services.boomdrive.enable`, evaluation SHALL fail where `settings.control_plane.type` is `crew` and `settings.control_plane.crew` is neither written by the host nor written by the modules, its words naming `settings.control_plane.crew` and `services.crew`. Evaluation SHALL fail where the modules would write the table and no partition's `machine` is `services.crew.host`, its words naming `services.crew.host`, its value, and each `machine` the partitions name.

#### Scenario: Under crew with neither

- **WHEN** `settings` names `control_plane.type = "crew"` and no `control_plane.crew`, and crew's module is off
- **THEN** evaluation fails, naming `settings.control_plane.crew` and `services.crew`, on nix-darwin and on NixOS

#### Scenario: No partition on this host

- **WHEN** `settings` names `control_plane.type = "crew"` and no `control_plane.crew`, and crew's module is on with the host `host-a` and the one partition `away`, whose `machine` is `host-b`
- **THEN** evaluation fails, naming `services.crew.host`, `host-a` and `host-b`, on nix-darwin and on NixOS

### Requirement: The README writes no crew table beside crew's partitions

The README's install with nix SHALL set the runner's `control_plane.type = "crew"` beside `services.crew` and write no `control_plane.crew`. Its words SHALL say the module writes `[control_plane.crew]` from crew's partitions whose `machine` is `services.crew.host`, and that a table written out wins.

#### Scenario: The README's crew table

- **WHEN** the README's install with nix is read
- **THEN** a nix block in it sets `type = "crew"`, no nix block in it names `state_repository`, and its words name `services.crew.host` and say a table written out wins

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

### Requirement: The modules install herdr, Ghostty and the launcher, each replaceable or dropped

With `services.boomdrive.enable`, the modules SHALL put `services.boomdrive.herdr.package`, `services.boomdrive.ghostty.package` and `services.boomdrive.launcher.package` in the system profile. Each SHALL be the set's for the host's system by default, a host's own package SHALL replace it, and `null` SHALL install none. With the runner not enabled, the modules SHALL install none of them.

#### Scenario: The way in with the runner

- **WHEN** a system turns on `services.boomdrive` and names none of the three
- **THEN** the system profile holds the set's `herdr`, `ghostty` and `boomdrive-launcher`, on nix-darwin and on NixOS

#### Scenario: A host's own herdr

- **WHEN** `services.boomdrive.herdr.package` is a package of the host's own
- **THEN** the system profile holds it and not the set's herdr, and the activation links the plugins with it, on nix-darwin and on NixOS

#### Scenario: No Ghostty of the set's

- **WHEN** `services.boomdrive.ghostty.package` is `null`
- **THEN** the system profile holds no Ghostty of the set's, and `services.flume.settings.host.terminal_app` is still `ghostty`, on nix-darwin and on NixOS

#### Scenario: Not without the runner

- **WHEN** a system imports the platform's module and leaves `services.boomdrive` off
- **THEN** its system profile holds none of the set's `herdr`, `ghostty` and `boomdrive-launcher`, on nix-darwin and on NixOS

### Requirement: The modules link the set's herdr plugins with herdr's own command

With `services.boomdrive.enable`, each activation SHALL run, as the runner's user, the installed herdr's own `herdr plugin link <plugin>` for each package of `services.boomdrive.herdr.plugins`, in order, with no `HERDR_*` variable of the caller's. The list's default SHALL be the set's two herdr plugins. The modules SHALL write no file of herdr's. With no herdr package, or no plugin, none SHALL be linked.

#### Scenario: The plugins linked on nix-darwin

- **WHEN** a nix-darwin system whose primary user is `alice` turns on `services.boomdrive`
- **THEN** its activation runs, as `alice`, the step that links the set's `flume-herdr-plugin` and `sluice-herdr-plugin` with the set's herdr

#### Scenario: The plugins linked on NixOS

- **WHEN** a NixOS system turns on `services.boomdrive` for `alice`
- **THEN** `alice`'s user activation runs the step that links the set's `flume-herdr-plugin` and `sluice-herdr-plugin` with the set's herdr, and another user's runs none

#### Scenario: Each plugin linked

- **WHEN** the activation's plugin step runs with herdr and two plugins, in a shell whose `HERDR_SOCKET_PATH` and `HERDR_ENV` are set
- **THEN** herdr is run as `herdr plugin link <plugin>` for each plugin, in order, with neither variable set, and the step exits 0

#### Scenario: No herdr of the set's

- **WHEN** `services.boomdrive.herdr.package` is `null`, or `services.boomdrive.herdr.plugins` is `[ ]`
- **THEN** the activation links no plugin, on nix-darwin and on NixOS

### Requirement: A plugin herdr refuses to link stops nothing else

A link herdr refuses SHALL be said in one line naming the plugin and herdr's words, and SHALL stop no other plugin's link and nothing else of the activation.

#### Scenario: A link herdr refuses

- **WHEN** herdr refuses the first plugin's link, saying `manifest not found`
- **THEN** one line names that plugin and `manifest not found`, the second plugin is still linked, and the step exits 0

### Requirement: The modules name herdr and Ghostty Flume's terminal and window

With `services.boomdrive.enable`, the modules SHALL set `services.flume.settings.host.multiplexer` to `herdr` and `services.flume.settings.host.terminal_app` to `ghostty` by default, each a host's own value winning. They SHALL do so whether or not the host keeps the set's herdr and Ghostty. Where the host's `services.flume.settings.host` gives `terminal`, the word Flume reads as `multiplexer` and refuses beside it, they SHALL set no `multiplexer`.

#### Scenario: Flume's terminal and window

- **WHEN** a system turns on `services.boomdrive` and names no key of Flume's `host`
- **THEN** `services.flume.settings.host` gives `multiplexer` `herdr` and `terminal_app` `ghostty`, on nix-darwin and on NixOS

#### Scenario: A host's own window

- **WHEN** a system turns on `services.boomdrive` and sets `services.flume.settings.host.terminal_app = "terminal"`
- **THEN** `services.flume.settings.host` gives `terminal_app` `terminal` and `multiplexer` `herdr`

#### Scenario: The word of before

- **WHEN** a system turns on `services.boomdrive` and sets `services.flume.settings.host.terminal = "herdr"`
- **THEN** `services.flume.settings.host` gives `terminal` `herdr` and no `multiplexer`

### Requirement: The modules name the runner's data folder by the runner's own rule

Each module SHALL declare `services.boomdrive.dataDir`, whether or not the runner is enabled, as the runner's data folder by the runner's own rule. That is `$XDG_DATA_HOME/boomdrive` where the runner's job gives an absolute `XDG_DATA_HOME`, else `~/.local/share/boomdrive` under the runner's user's home: the primary user's on nix-darwin, `services.boomdrive.user`'s on NixOS. It SHALL never name the folder of before the rename (record 0035).

#### Scenario: The data folder on nix-darwin

- **WHEN** the module is enabled for the primary user `alice`, whose home is `/Users/alice`, and the job is given no `XDG_DATA_HOME`
- **THEN** `services.boomdrive.dataDir` is `/Users/alice/.local/share/boomdrive`

#### Scenario: The data folder on NixOS

- **WHEN** the module is enabled for the user `alice`, whose home is `/home/alice`, and the job is given no `XDG_DATA_HOME`
- **THEN** `services.boomdrive.dataDir` is `/home/alice/.local/share/boomdrive`

#### Scenario: The data folder of another user on NixOS

- **WHEN** the module is enabled for the user `bob`, whose home is `/home/bob`
- **THEN** `services.boomdrive.dataDir` is `/home/bob/.local/share/boomdrive`

#### Scenario: The data folder with the runner not enabled

- **WHEN** the nix-darwin module is imported for the primary user `alice` and `services.boomdrive.enable` is not set
- **THEN** `services.boomdrive.dataDir` is `/Users/alice/.local/share/boomdrive`

#### Scenario: The runner's own rule

- **WHEN** the module's rule and the runner's `data_dir` are given one `HOME` and one `XDG_DATA_HOME`, the latter unset, absolute, absolute ending in `/`, empty or relative
- **THEN** each gives the same folder

### Requirement: The data folder follows the job's own XDG_DATA_HOME

The `XDG_DATA_HOME` the data folder follows SHALL be the one the job's declaration gives: the agent's `EnvironmentVariables` on nix-darwin and the unit's `environment` on NixOS. An empty or relative one SHALL be taken as unset, as the runner takes it.

#### Scenario: A job given its own XDG_DATA_HOME

- **WHEN** the host gives the runner's job `XDG_DATA_HOME` `/srv/data`, on nix-darwin in `launchd.user.agents.boomdrive.serviceConfig.EnvironmentVariables` and on NixOS in `systemd.user.services.boomdrive.environment`
- **THEN** `services.boomdrive.dataDir` is `/srv/data/boomdrive` on either platform, and the nix-darwin agent's `HOME` and `PATH` are the module's

#### Scenario: An XDG_DATA_HOME the runner takes as unset

- **WHEN** the host gives the runner's job `XDG_DATA_HOME` `data`, or an empty one
- **THEN** `services.boomdrive.dataDir` is `~/.local/share/boomdrive` under the user's home, on either platform

### Requirement: No host sets the runner's data folder

`services.boomdrive.dataDir` SHALL be read-only, so evaluation of it fails where a host sets it, at any priority.

#### Scenario: A host that sets the data folder

- **WHEN** a host sets `services.boomdrive.dataDir = "/srv/boomdrive"`
- **THEN** evaluation of the option fails, on either platform, as nixpkgs fails a read-only option set beside its value

### Requirement: The README names the runner's data folder

The README's **Installing with nix** SHALL name `services.boomdrive.dataDir` and what it holds. It SHALL say the option is read-only and is read wherever a host names the folder, a clone's folder in it being read from `services.boomdrive.cloneDirs`. It SHALL say a host gives the job its own `XDG_DATA_HOME`, since one the job takes from elsewhere is not seen.

#### Scenario: The README's data folder

- **WHEN** the README's **Installing with nix** is read
- **THEN** it names `services.boomdrive.dataDir` and says it is read-only

### Requirement: The modules state each kept repository's clone folder by the runner's own rule

Each module SHALL declare `services.boomdrive.cloneDirs`, whether or not the runner is enabled. It SHALL hold one entry for each address of every drive of every account in `services.boomdrive.accounts`, a drive of another machine's among them, and no other. Each entry SHALL be keyed by its address as written, `<owner>/<name>`, and its value SHALL be the runner's clone folder of it by the runner's own rule, `<services.boomdrive.dataDir>/repositories/<owner>/<name>`.

#### Scenario: Each kept repository's clone folder on nix-darwin

- **WHEN** the module is enabled for the primary user `alice`, whose home is `/Users/alice`, and the account `own`'s drive `home` keeps the owner `agentplot`'s `flume` and `sluice`
- **THEN** `services.boomdrive.cloneDirs` is `agentplot/flume` at `/Users/alice/.local/share/boomdrive/repositories/agentplot/flume` and `agentplot/sluice` at `/Users/alice/.local/share/boomdrive/repositories/agentplot/sluice`, and nothing else

#### Scenario: Each kept repository's clone folder on NixOS

- **WHEN** the module is enabled for the user `alice`, whose home is `/home/alice`, and the account `own`'s drive `home` keeps the owner `agentplot`'s `flume` and `sluice`
- **THEN** `services.boomdrive.cloneDirs` is `agentplot/flume` at `/home/alice/.local/share/boomdrive/repositories/agentplot/flume` and `agentplot/sluice` at `/home/alice/.local/share/boomdrive/repositories/agentplot/sluice`, and nothing else

#### Scenario: The clone folder of another machine's drive

- **WHEN** crew's host is `host-a`, and the account `work`'s drive `garden`, on the machine `host-b`, keeps the owner `example`'s `alpha`
- **THEN** `services.boomdrive.cloneDirs` gives `example/alpha` its folder under `services.boomdrive.dataDir`, on either platform

#### Scenario: The clone folders follow the data folder

- **WHEN** the host gives the runner's job `XDG_DATA_HOME` `/srv/data`, and the drive `home` keeps `agentplot/flume`
- **THEN** `services.boomdrive.cloneDirs` gives `agentplot/flume` at `/srv/data/boomdrive/repositories/agentplot/flume`, on either platform

#### Scenario: No repository kept

- **WHEN** `services.boomdrive.accounts` names no repository
- **THEN** `services.boomdrive.cloneDirs` is empty, on either platform

#### Scenario: The clone folders with the runner not enabled

- **WHEN** the nix-darwin module is imported for the primary user `alice`, the drive `home` keeps `agentplot/flume`, and `services.boomdrive.enable` is not set
- **THEN** `services.boomdrive.cloneDirs` gives `agentplot/flume` at `/Users/alice/.local/share/boomdrive/repositories/agentplot/flume`

#### Scenario: The runner's own clone rule

- **WHEN** the module's rule and the runner's `clone::container` are given one data folder and one address, its owner and name each as written
- **THEN** each gives the same folder

### Requirement: No host sets a clone's folder

`services.boomdrive.cloneDirs` SHALL be read-only, so evaluation of it fails where a host sets it, or an entry of it, at any priority.

#### Scenario: A host that sets a clone's folder

- **WHEN** a host sets `services.boomdrive.cloneDirs."agentplot/flume" = "/srv/flume"`
- **THEN** evaluation of the option fails, on either platform, as nixpkgs fails a read-only option set beside its value

### Requirement: The README names each kept repository's clone folder

The README's **Installing with nix** SHALL name `services.boomdrive.cloneDirs`, what it holds and how each entry is keyed. It SHALL say the option is read-only and is read wherever a host names a clone's folder, and that it holds only the repositories `services.boomdrive.accounts` names.

#### Scenario: The README's clone folders

- **WHEN** the README's **Installing with nix** is read
- **THEN** it names `services.boomdrive.cloneDirs`
