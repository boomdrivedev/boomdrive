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

- **WHEN** the NixOS module is enabled for `alice`, with crew turned on with its host, its drives and its hosts, and again with `services.sluice.user = "bob"`
- **THEN** Flume's, sluice's and crew's `user` are `alice` in the first, and sluice's is `bob` in the second

### Requirement: Each tool's package is the set's unless the host gives its own

The modules SHALL set `services.flume.package`, `services.sluice.package` and `services.crew.package` to the set's package of that tool for the host's system by default, whether or not the runner is enabled. A host's own value SHALL win, and SHALL change nothing of the tool's file.

#### Scenario: The set's packages

- **WHEN** a system turns on `services.boomdrive` and crew with its host, its drives and its hosts, naming no tool's package
- **THEN** the system profile holds the set's `boomdrive`, `flume`, `sluice` and `crew`, on nix-darwin and on NixOS

#### Scenario: A host's own crew

- **WHEN** `services.crew.package` is a package of the host's own
- **THEN** the system profile holds it and not the set's crew, and crew's activation links the same file it links with the set's

#### Scenario: No sluice of the set's

- **WHEN** `services.sluice.package` is `null`
- **THEN** the system profile holds no sluice of the set's, and sluice's file is still linked

### Requirement: The Boomdrive module writes no file of the other tools'

The modules' own activation SHALL link the runner's file, write Pending You's folder where crew's companions are installed, run herdr's own `plugin link` for the plugins `services.boomdrive.herdr.plugins` lists, write or remove Claude Code's managed telemetry document on nix-darwin, and do nothing else of a tool's. Flume's, sluice's and crew's files, crew's source in Flume's folder among them, SHALL each be written by that tool's own module, as it writes them without Boomdrive's.

#### Scenario: The tools' activation is their own

- **WHEN** a system with `services.boomdrive` and crew turned on is set beside one that imports Flume's, sluice's and crew's own modules alone, each turned on with the same settings, packages and user, and Flume's with crew's source declared as the modules declare it
- **THEN** on nix-darwin their activations are the same but for the runner's own link, Pending You's folder, herdr's plugins' links and the managed telemetry document's step, and on NixOS their user activations are the same but for the runner's own

### Requirement: The modules hold no option that installs a tool its own module installs

The modules SHALL hold no option of their own that installs a tool whose own module installs it: setting `services.boomdrive.tools` SHALL fail evaluation, naming `services.flume.package` and `services.sluice.package`. Their own options SHALL install herdr, Ghostty, portless, worktrunk, the launcher, Pending You, Plannotator and plannotator-tui. The README's install with nix SHALL name each option that installs a tool of the set, and the shell, and no `services.boomdrive.tools`.

#### Scenario: The tools option is gone

- **WHEN** a system sets `services.boomdrive.tools = [ ]`
- **THEN** evaluation fails, naming `services.flume.package` and `services.sluice.package`

#### Scenario: The README's words

- **WHEN** the README's install with nix is read
- **THEN** it names `services.flume.package`, `services.sluice.package`, `services.crew.package`, `services.boomdrive.herdr.package`, `services.boomdrive.herdr.plugins`, `services.boomdrive.ghostty.package`, `services.boomdrive.portless.package`, `services.boomdrive.worktrunk.package`, `services.boomdrive.launcher.package`, `services.boomdrive.pendingyou.package`, `services.boomdrive.plannotator.package`, `services.boomdrive.plannotator-tui.package` and `nix develop github:boomdrivedev/boomdrive`, and no `services.boomdrive.tools`

### Requirement: A crew table written out wins whole

Where the host writes `settings.control_plane.crew`, the file's `[control_plane.crew]` SHALL be that table, whole, with none of crew's drives merged into it.

#### Scenario: A table written out wins

- **WHEN** `settings` names a crew control plane whose table is the host `host-a` with the partition `garden` at `example/garden` alone, and crew's module is on with the host `host-a` and the drives `home` on `host-a` and `away` on `host-b`
- **THEN** `settings.control_plane.crew` is the table written, the partition `garden` alone, and the file is that table's rendering, on nix-darwin and on NixOS

### Requirement: No crew table beside another control plane

Where `settings` names no control plane, or one whose `type` is not `crew`, the modules SHALL write no `[control_plane.crew]`, whatever crew's module holds.

#### Scenario: A hosted control plane beside crew

- **WHEN** `accounts` names the account `home`, whose drive `garden` keeps the owner `agentplot`'s `flume`, and the account `work` with no drive, and `settings` names a hosted control plane at `https://control.example` with the partitions `home` and `work`, and crew's module is on with the host `host-a` and the drive `home` on `host-a`
- **THEN** the file is `nix/tests/files/hosted.toml`, byte for byte, on nix-darwin and on NixOS

#### Scenario: No control plane beside crew

- **WHEN** neither `accounts` nor `settings` sets anything, and crew's module is on with the host `host-a` and the drive `home` on `host-a`
- **THEN** the file is `nix/tests/files/empty.toml`, byte for byte, on nix-darwin and on NixOS

### Requirement: The README writes no crew table beside crew's drives

The README's install with nix SHALL set the runner's `control_plane.type = "crew"` beside `services.crew` and write no `control_plane.crew`. Its words SHALL say the module writes `[control_plane.crew]` from crew's drives whose `machine` is `services.crew.host`, and that a table written out wins.

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

A drive's `machine` SHALL name the host its work runs on, as crew's drives name a host. A drive whose `machine` is unset or is `services.crew.host` is a drive of this host. A drive whose `machine` names another host SHALL give Flume no stream and no repository's table, and its repositories SHALL still be written in the runner's file.

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

With `services.boomdrive.enable`, the modules SHALL put `services.boomdrive.herdr.package`, `services.boomdrive.ghostty.package` and `services.boomdrive.launcher.package` in the system profile. Each SHALL be the set's for the host's system by default, the launcher made for each drive of this host; a host's own package SHALL replace each, installed as given, and `null` SHALL install none. With the runner not enabled, the modules SHALL install none of them.

#### Scenario: The way in with the runner

- **WHEN** a system turns on `services.boomdrive`, names none of the three and has no drive
- **THEN** the system profile holds the set's `herdr`, `ghostty` and `boomdrive-launcher`, on nix-darwin and on NixOS

#### Scenario: A host's own herdr

- **WHEN** `services.boomdrive.herdr.package` is a package of the host's own
- **THEN** the system profile holds it and not the set's herdr, and the activation links the plugins with it, on nix-darwin and on NixOS

#### Scenario: No Ghostty of the set's

- **WHEN** `services.boomdrive.ghostty.package` is `null`
- **THEN** the system profile holds no Ghostty of the set's, and `services.flume.settings.host.terminal_app` is still `ghostty`, on nix-darwin and on NixOS

#### Scenario: A host's own launcher

- **WHEN** `services.boomdrive.launcher.package` is a package of the host's own, and the host has the drives `garden` and `home`
- **THEN** the system profile holds that package as given, on nix-darwin and on NixOS

#### Scenario: Not without the runner

- **WHEN** a system imports the platform's module and leaves `services.boomdrive` off
- **THEN** its system profile holds none of the set's `herdr`, `ghostty` and `boomdrive-launcher`, on nix-darwin and on NixOS

### Requirement: The modules link the set's herdr plugins with herdr's own command

With `services.boomdrive.enable`, each activation SHALL run, as the runner's user, the installed herdr's own `herdr plugin link <plugin>` for each package of `services.boomdrive.herdr.plugins`, in order, with no `HERDR_*` variable of the caller's. The list's default SHALL be the set's `flume-herdr-plugin` and `sluice-herdr-plugin`, then, with crew on, its companions' plugins. The modules SHALL write no file of herdr's. With no herdr package or no plugin, none SHALL be linked.

#### Scenario: The plugins linked on nix-darwin

- **WHEN** a nix-darwin system whose primary user is `alice` turns on `services.boomdrive`
- **THEN** its activation runs, as `alice`, the step that links the set's `flume-herdr-plugin` and `sluice-herdr-plugin` with the set's herdr

#### Scenario: The plugins linked on NixOS

- **WHEN** a NixOS system turns on `services.boomdrive` for `alice`
- **THEN** `alice`'s user activation runs the step that links the set's `flume-herdr-plugin` and `sluice-herdr-plugin` with the set's herdr, and another user's runs none

#### Scenario: The companions' plugins with crew

- **WHEN** a system turns on `services.boomdrive` and `services.crew` and names no plugin
- **THEN** its activation links the set's `flume-herdr-plugin`, `sluice-herdr-plugin`, `pendingyou-herdr-plugin` and `plannotator-herdr-plugin`, in that order, on nix-darwin and on NixOS

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

### Requirement: The modules install portless, replaceable or dropped

With `services.boomdrive.enable`, the modules SHALL put `services.boomdrive.portless.package` in the system profile. It SHALL be the set's portless for the host's system by default. A host's own package SHALL replace it, and `null` SHALL install none. With the runner not enabled, the modules SHALL install no portless.

#### Scenario: portless with the runner

- **WHEN** a system turns on `services.boomdrive` and names no portless
- **THEN** the system profile holds the set's `portless`, on nix-darwin and on NixOS

#### Scenario: A host's own portless

- **WHEN** `services.boomdrive.portless.package` is a package of the host's own
- **THEN** the system profile holds it and not the set's portless, on nix-darwin and on NixOS

#### Scenario: No portless of the set's

- **WHEN** `services.boomdrive.portless.package` is `null`
- **THEN** the system evaluates, and its profile holds no portless of the set's, on nix-darwin and on NixOS

#### Scenario: No portless without the runner

- **WHEN** a system imports the platform's module and leaves `services.boomdrive` off
- **THEN** its system profile holds no portless of the set's, on nix-darwin and on NixOS

### Requirement: portless's proxy and its trust are the person's

The modules SHALL start no proxy of portless's. No launchd daemon or agent, systemd unit or activation step of theirs SHALL run portless. They SHALL write no file of portless's and trust no certificate authority. The README's install with nix SHALL say that portless's proxy is started by its first `portless run` at a terminal or by `portless service install`, and that `portless trust` is the person's.

#### Scenario: No proxy of the modules'

- **WHEN** a system turns on `services.boomdrive` and names no portless
- **THEN** no launchd daemon or agent, no systemd unit and no activation step of the system names the set's portless, on nix-darwin and on NixOS

#### Scenario: The README's proxy

- **WHEN** the README's install with nix is read
- **THEN** it names `portless service install` and `portless trust`

### Requirement: The modules install worktrunk, replaceable or dropped

With `services.boomdrive.enable`, the modules SHALL put `services.boomdrive.worktrunk.package` in the system profile. It SHALL be the set's `worktrunk` for the host's system by default, a host's own package SHALL replace it, and `null` SHALL install none. With the runner not enabled, the modules SHALL install no worktrunk. The modules SHALL write no file of worktrunk's.

#### Scenario: worktrunk with the runner

- **WHEN** a system turns on `services.boomdrive` and names no worktrunk
- **THEN** the system profile holds the set's `worktrunk`, on nix-darwin and on NixOS

#### Scenario: A host's own worktrunk

- **WHEN** `services.boomdrive.worktrunk.package` is a package of the host's own
- **THEN** the system profile holds it and not the set's worktrunk, on nix-darwin and on NixOS

#### Scenario: No worktrunk of the set's

- **WHEN** `services.boomdrive.worktrunk.package` is `null`
- **THEN** the system evaluates and its profile holds no worktrunk of the set's, on nix-darwin and on NixOS

#### Scenario: No worktrunk without the runner

- **WHEN** a system imports the platform's module and leaves `services.boomdrive` off
- **THEN** its system profile holds no worktrunk of the set's, on nix-darwin and on NixOS

### Requirement: The launcher has an entry for each drive of this host

With `services.boomdrive.enable`, the launcher the modules make SHALL be the set's `boomdrive-launcher` made, by its override, for one entry for each drive of this host, in the order of the drives' names, each entry's `drive` the drive's name, and the set's own `boomdrive-launcher` where this host has no drive. A drive of this host is one whose `machine` is unset or is `services.crew.host`. A drive of another machine SHALL have no entry.

#### Scenario: Each drive of this host in the launcher

- **WHEN** the account `own` has the drive `home` and the account `work` the drive `garden`, each keeping one repository and naming no machine
- **THEN** the installed launcher is made for `garden` with the session `garden`, then `home` with the session `home`, on nix-darwin and on NixOS

#### Scenario: A drive of another machine has no entry

- **WHEN** `services.crew.host` is `host-a`, the drive `garden` sets `machine = "host-b"`, and the drive `home` names no machine
- **THEN** the installed launcher is made for `home` alone, on nix-darwin and on NixOS

#### Scenario: No drive, the set's launcher

- **WHEN** a system turns on `services.boomdrive` with no account
- **THEN** the installed launcher is the set's `boomdrive-launcher`'s store path, on nix-darwin and on NixOS

### Requirement: A drive's entry opens its stream's own session

An entry's `session` SHALL be the drive's stream's own session as Flume names it: `services.flume.settings.streams.<stream>.session` where that is set, and the drive's stream's name otherwise (Flume's statements 107 and 186).

#### Scenario: A stream's own session named

- **WHEN** the drive `home` keeps one repository and `services.flume.settings.streams.home.session` is `house`
- **THEN** the installed launcher's entry `home` has the session `house`, on nix-darwin and on NixOS

### Requirement: The README says the launcher opens each drive

The README's install with nix SHALL say that the launcher the modules install is a menu-bar app on macOS and a desktop entry on Linux, holding an entry for each drive of the host and `Default stream`.

#### Scenario: The README's launcher

- **WHEN** the README's install with nix is read
- **THEN** it names `menu bar`, `each drive` and `Default stream`

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

### Requirement: With the runner and crew's module on, the modules give crew each kept repository's folders

With `services.boomdrive.enable` and `services.crew.enable`, each module SHALL give `services.crew.settings.repositories` one entry for each address of `services.boomdrive.cloneDirs`, and no other. Each entry's `dir` SHALL be that address's folder in `services.boomdrive.cloneDirs`, and its `checkout` SHALL be the runner's view of the address.

#### Scenario: crew's kept repositories on nix-darwin

- **WHEN** the module is enabled for the primary user `alice`, whose home is `/Users/alice`, crew's module is on with its host and partitions, and the account `own`'s drive `home` keeps the owner `agentplot`'s `flume` and `sluice`
- **THEN** `services.crew.settings.repositories` gives `agentplot/flume` the `dir` `/Users/alice/.local/share/boomdrive/repositories/agentplot/flume` and the `checkout` `/Users/alice/Code/agentplot/flume`, gives `agentplot/sluice` the same under its own name, and gives nothing else

#### Scenario: crew's kept repositories on NixOS

- **WHEN** the module is enabled for the user `alice`, whose home is `/home/alice`, crew's module is on with its host and partitions, and the account `own`'s drive `home` keeps the owner `agentplot`'s `flume` and `sluice`
- **THEN** `services.crew.settings.repositories` gives `agentplot/flume` the `dir` `/home/alice/.local/share/boomdrive/repositories/agentplot/flume` and the `checkout` `/home/alice/Code/agentplot/flume`, gives `agentplot/sluice` the same under its own name, and gives nothing else

#### Scenario: crew's kept repository of another machine's drive

- **WHEN** crew's module is on with the host `host-a`, and the account `work`'s drive `garden`, on the machine `host-b`, keeps the owner `example`'s `alpha`
- **THEN** `services.crew.settings.repositories` gives `example/alpha` the `dir` `services.boomdrive.cloneDirs` gives it and the `checkout` `Code/example/alpha` under the user's home, on either platform

#### Scenario: crew's checkouts under the host's repository root

- **WHEN** crew's module is on, the drive `home` keeps `agentplot/flume`, and `services.boomdrive.settings.repository_root` is `~/src`, and again `/srv/code`
- **THEN** `services.crew.settings.repositories` gives `agentplot/flume` the `checkout` `src/agentplot/flume` under the user's home in the first, and `/srv/code/agentplot/flume` in the second, on either platform

### Requirement: crew's checkout is the runner's view by the runner's own rule

The view the modules give crew SHALL be the runner's by the runner's own rule, `<repository_root>/<owner>/<name>`. Its root SHALL be `services.boomdrive.settings.repository_root`, or `~/Code` where that is unset, and `~/` SHALL be the home of the runner's user.

#### Scenario: The runner's own view rule

- **WHEN** the module's view rule and the runner's `config::view_path` are given one home, one address and one `repository_root`, the root unset, beginning with `~/`, `~/` alone, absolute, or absolute ending in `/`
- **THEN** each gives the same folder, as text

#### Scenario: A repository root the runner refuses

- **WHEN** the module's view rule and the runner's reading of its file are given a `repository_root` that is neither absolute nor begins with `~/`, such as `src` or `$HOME/Code`
- **THEN** the rule's evaluation fails and the runner refuses the file

### Requirement: Without the runner, crew's module or a kept repository, the modules give crew's kept repositories nothing

The modules SHALL give `services.crew.settings.repositories` nothing where `services.boomdrive.enable` or `services.crew.enable` is not set, or where `services.boomdrive.cloneDirs` names no address, so the host's own value stands, and crew's file names none where the host names none.

#### Scenario: crew's module off

- **WHEN** the module is enabled, the drive `home` keeps `agentplot/flume` and `agentplot/sluice`, and crew's module is not turned on
- **THEN** `services.crew.settings.repositories` is `null`, on either platform

#### Scenario: The runner not enabled

- **WHEN** the platform's module is imported with crew's module on and the drive `home`, and `services.boomdrive.enable` is not set
- **THEN** `services.crew.settings.repositories` is `null`, on either platform

#### Scenario: No repository kept beside crew

- **WHEN** the module is enabled, crew's module is on, and `services.boomdrive.accounts` names no repository
- **THEN** `services.crew.settings.repositories` is `null`, on either platform

### Requirement: No host writes a kept repository's folders

The modules SHALL give each entry of crew's kept repositories at their own priority, so evaluation of `services.crew.settings.repositories` fails where a host gives an address of `services.boomdrive.cloneDirs` another `dir` or another `checkout`. A host's own entry for an address `services.boomdrive.cloneDirs` does not name SHALL stand beside the modules' entries.

#### Scenario: A host that gives a kept repository another folder

- **WHEN** crew's module is on, the drive `home` keeps `agentplot/flume`, and a host gives `services.crew.settings.repositories."agentplot/flume".dir = "/srv/flume"`, and again `.checkout = "/srv/flume"`
- **THEN** evaluation of `services.crew.settings.repositories` fails in each, on either platform

#### Scenario: A host's own kept repository

- **WHEN** crew's module is on, the drive `home` keeps `agentplot/flume` and `agentplot/sluice`, and a host gives `services.crew.settings.repositories."example/own"` the `dir` `/srv/own` and the `checkout` `/srv/own/main`
- **THEN** `services.crew.settings.repositories` gives `example/own` as the host gave it, beside `agentplot/flume` and `agentplot/sluice` as the module gives them, on either platform

### Requirement: The README names crew's kept repositories

The README's **Installing with nix** SHALL say that, with the runner and crew's module on, the module gives `services.crew.settings.repositories` each address of `services.boomdrive.cloneDirs`, its `dir` the clone's folder and its `checkout` the runner's view. It SHALL say a host writes none, an entry it gives another folder failing evaluation, and that a repository of the host's own that the runner does not keep stays the host's to name.

#### Scenario: The README's kept repositories

- **WHEN** the README's **Installing with nix** is read
- **THEN** it names `services.crew.settings.repositories`

### Requirement: With crew on, the modules declare crew's source to Flume

Where `services.crew.enable` is true, whether or not `services.boomdrive.enable` is, the modules SHALL give Flume's module the source `crew`, `services.flume.sources.crew`, with `kinds_dir` set to `~/.local/share/flume/kinds/crew` and `roles` set to `~/.local/state/crew/flume/roles.toml`, and no other key (record 0034, point 2; crew record 0057, point 1).

#### Scenario: crew's source with crew on

- **WHEN** a system turns on `services.boomdrive` and crew, with its host and drives, and sets no option of `services.flume.sources`
- **THEN** `services.flume.sources.crew` is exactly `kinds_dir = "~/.local/share/flume/kinds/crew"` and `roles = "~/.local/state/crew/flume/roles.toml"`, Flume's activation links a source rendered as `flume-source-crew.toml`, and the system fails no assertion, on nix-darwin and on NixOS

#### Scenario: crew's source without the runner

- **WHEN** a system imports the platform's `default` module, leaves `services.boomdrive.enable` unset, and turns on crew with its host and drives
- **THEN** `services.flume.sources.crew` names the same `kinds_dir` and `roles`, on nix-darwin and on NixOS

### Requirement: With crew off, no source of crew's

Where `services.crew.enable` is false, the modules SHALL give Flume's module no source of crew's.

#### Scenario: No source of crew's with crew off

- **WHEN** a system turns on `services.boomdrive` and leaves crew off
- **THEN** `services.flume.sources` holds no `crew`, and Flume's activation links no `flume-source-crew.toml`, on nix-darwin and on NixOS

### Requirement: crew's source names what the set's crew writes

The `roles` and `kinds_dir` the modules give crew's source SHALL be the roles file and the kinds folder the set's crew writes, each under the home of the user it runs as, written under `~`.

#### Scenario: The paths the set's crew writes

- **WHEN** crew's `plugin/lib/source.py`, at the crew the lock names, is read for its `ROLES_FILE` and `KINDS_DIR`, each `Path.home() / "<path>"`
- **THEN** the source's `roles` is `~/` followed by `ROLES_FILE`'s path, and its `kinds_dir` is `~/` followed by `KINDS_DIR`'s

### Requirement: A host's own value in crew's source wins

The modules SHALL give each key of crew's source at the options' default priority, so a value a host sets in `services.flume.sources.crew` wins over the module's, and the key it does not set keeps the module's.

#### Scenario: A host's own kinds folder for crew

- **WHEN** a system turns on `services.boomdrive` and crew, with its host and drives, and sets `services.flume.sources.crew.kinds_dir = "/srv/kinds/crew"`
- **THEN** the source's `kinds_dir` is `/srv/kinds/crew`, its `roles` is `~/.local/state/crew/flume/roles.toml`, and the system fails no assertion, on nix-darwin and on NixOS

### Requirement: The README names crew's source

The README's **Installing with nix** SHALL say that, with crew's module on, the module declares crew's source to Flume, `services.flume.sources.crew`, naming the roles file crew writes and its kinds folder, and that a host writes no file for it. No nix block in it SHALL set `services.flume.sources`.

#### Scenario: The README's words on crew's source

- **WHEN** the README's **Installing with nix** is read
- **THEN** it says `services.flume.sources.crew`, `~/.local/state/crew/flume/roles.toml` and `~/.local/share/flume/kinds/crew`, and no nix block in it sets `services.flume.sources`

### Requirement: The App's key is named by a path outside the store

`services.boomdrive.settings.forge` SHALL be declared, unset by default: its `app_id` a positive integer and its `key_file` a string, so a Nix path value fails the option's type and a key is never copied into the store. Evaluation SHALL fail, naming `services.boomdrive.settings.forge.key_file` and the Nix store, where `key_file` begins with the store directory. Set, it SHALL be rendered as the file's `[forge]`, which the runner reads (record 0007 as amended on 2026-11-11; core.46).

#### Scenario: The App named

- **WHEN** `settings.forge` is `{ app_id = 1; key_file = "~/.config/boomdrive/app.pem"; }`
- **THEN** the file holds `[forge]` with `app_id = 1` and that `key_file`, and the runner reads the App `1` from it

#### Scenario: A key file in the store

- **WHEN** `settings.forge.key_file` is `"/nix/store/abc-app.pem"`
- **THEN** evaluation fails, naming `services.boomdrive.settings.forge.key_file` and the Nix store

#### Scenario: A key given as a Nix path

- **WHEN** `settings.forge.key_file` is a Nix path value
- **THEN** evaluation fails on the option's type

### Requirement: Under crew, the runner's crew table is written from crew's drives on this host

Where `settings.control_plane.type` is `crew`, crew's module is on and the host writes no `settings.control_plane.crew`, the modules SHALL write it: `host` as `services.crew.host`, and one entry of `partitions` for each of crew's drives whose `machine` is that host, in the order of their labels, each its `label` and its `state` as `state_repository` alone. `settings.control_plane.crew` SHALL read that table (record 0023; record 0032; core.18).

#### Scenario: crew's drives on this host

- **WHEN** `accounts` names the account `own`, whose drive `home` keeps the owner `agentplot`'s `flume` and `sluice`, and `settings` names `repository_root = "~/Code"`, `fetch_interval = "2m"`, `code_workspaces = true` and `control_plane.type = "crew"` with no `control_plane.crew`, and crew's module is on with the host `host-a` and the drives `home`, whose `machine` is `host-a` and `state` `example/state`, and `away`, whose `machine` is `host-b` and `state` `example/away`
- **THEN** the file is `nix/tests/files/crew.toml`, byte for byte, on nix-darwin and on NixOS

#### Scenario: crew's drives in label order

- **WHEN** `control_plane.type = "crew"` is set with no `control_plane.crew`, and crew's module is on with the host `host-a` and the drives `garden` on `host-a` at `example/garden`, `away` on `host-b` at `example/away` and `home` on `host-a` at `example/state`
- **THEN** `settings.control_plane.crew` is the host `host-a` with the partitions `garden` at `example/garden`, then `home` at `example/state`, each holding `label` and `state_repository` alone, on nix-darwin and on NixOS

### Requirement: Under crew, a host the runner cannot serve fails evaluation

With `services.boomdrive.enable`, evaluation SHALL fail where `settings.control_plane.type` is `crew` and `settings.control_plane.crew` is neither written by the host nor written by the modules, its words naming `settings.control_plane.crew` and `services.crew`. Evaluation SHALL fail where the modules would write the table and no drive of crew's has `services.crew.host` as its `machine`, its words naming `services.crew.host`, its value, and each `machine` crew's drives name.

#### Scenario: Under crew with neither

- **WHEN** `settings` names `control_plane.type = "crew"` and no `control_plane.crew`, and crew's module is off
- **THEN** evaluation fails, naming `settings.control_plane.crew` and `services.crew`, on nix-darwin and on NixOS

#### Scenario: No drive of crew's on this host

- **WHEN** `settings` names `control_plane.type = "crew"` and no `control_plane.crew`, and crew's module is on with the host `host-a` and the one drive `away`, whose `machine` is `host-b`
- **THEN** evaluation fails, naming `services.crew.host`, `host-a` and `host-b`, on nix-darwin and on NixOS

### Requirement: A host on the set loads crew's teams file at version 6, by crew's names

Where crew's module is on, the set's crew SHALL render crew's teams file at version 6, by crew's names: each drive's `runs`, each main-level role `"standing"` or `"on-demand"`; its `review`'s `after_changes` and `after_hours`; and `hosts`, each with its `ssh` name and `kind`. A drive naming `flume`, or `review`'s `every` or `hours`, SHALL fail evaluation (crew record 0064 as amended on 2026-11-09).

#### Scenario: A host on the set's crew

- **WHEN** crew's module is on with the host `host-a`; the drive `home`, whose `machine` is `host-a` and `state` `example/state`, whose `runs` names `design`, `planner` and `operator` `"standing"` and `reviewer` and `ops` `"on-demand"`, and whose `review` is `after_changes = 5` and `after_hours = 4`; and the hosts `host-a`, `ssh = "host-a"` and `kind = "darwin"`, and `host-b`, `ssh = "host-b"` and `kind = "nixos"`
- **THEN** the system fails no assertion, and crew's teams file's `version` is 6, on nix-darwin and on NixOS

#### Scenario: crew's old way of running

- **WHEN** crew's module is on with the host `host-a` and the drive `home`, which names `flume`, each of its five roles `false`, in place of `runs`
- **THEN** evaluation fails, on nix-darwin and on NixOS

#### Scenario: crew's old batch names

- **WHEN** crew's module is on with the host `host-a` and the drive `home`, whose `review` is `every = 5` and `hours = 4`
- **THEN** evaluation fails, on nix-darwin and on NixOS

### Requirement: The modules write none of crew's hosts, runs or review

The modules SHALL write no key of crew's `hosts`, and none of a drive's `runs` or `review`. Each is the host's own, written in `services.crew.settings` (core.57; record 0036, point 4).

#### Scenario: crew's keys stay the host's

- **WHEN** a system turns on `services.boomdrive` and crew's module with the host `host-a`, the drive `home` and the hosts `host-a` and `host-b`, as "A host on the set's crew" writes them
- **THEN** crew's `hosts`, and the drive `home`'s `runs` and `review`, are exactly those written, on nix-darwin and on NixOS

### Requirement: The README writes crew's settings by crew's names

The README's install with nix SHALL write `services.crew.settings` by crew's names: `drives`, keyed by label, each drive's `runs` and its `review`'s `after_changes` and `after_hours`, and `hosts`, each with its `ssh` name and its `kind`. No nix block of it SHALL name crew's `partitions`, a drive's `flume`, or `review`'s `every` or `hours`.

#### Scenario: The README's crew settings

- **WHEN** the README's install with nix is read
- **THEN** a nix block in it that names `services.crew` names `drives`, `runs`, `after_changes`, `after_hours`, `hosts`, `ssh` and `kind`, and no nix block in it names `partitions`, a key `flume`, `every =` or `hours =`

### Requirement: The modules install crew's companions with the runner and crew, each replaceable or dropped

With `services.boomdrive.enable` and `services.crew.enable`, the modules SHALL put `services.boomdrive.pendingyou.package`, `services.boomdrive.plannotator.package` and `services.boomdrive.plannotator-tui.package` in the system profile, and `pendingyou-signin` beside a Pending You. Each SHALL be the set's by default, a host's own replacing it and `null` installing none. Without the runner or crew, none SHALL be installed and no folder of Pending You's written.

#### Scenario: The companions with the runner and crew

- **WHEN** a system turns on `services.boomdrive` and `services.crew` and names none of the three
- **THEN** the system profile holds the set's `pendingyou`, `plannotator` and `plannotator-tui`, and `pendingyou-signin`, on nix-darwin and on NixOS

#### Scenario: A host's own Plannotator

- **WHEN** `services.boomdrive.plannotator.package` is a package of the host's own
- **THEN** the system profile holds it and not the set's Plannotator, on nix-darwin and on NixOS

#### Scenario: No Pending You of the set's

- **WHEN** `services.boomdrive.pendingyou.package` is `null`
- **THEN** the system evaluates, its profile holds no Pending You and no `pendingyou-signin`, and its activation writes no folder of Pending You's, on nix-darwin and on NixOS

#### Scenario: No companions without crew

- **WHEN** a system turns on `services.boomdrive` and leaves `services.crew` off
- **THEN** its profile holds none of the set's `pendingyou`, `plannotator` and `plannotator-tui`, its activation writes no folder of Pending You's, and its plugins' default is Flume's and sluice's alone, on nix-darwin and on NixOS

#### Scenario: No companions without the runner

- **WHEN** a system imports the platform's module, turns on `services.crew` and leaves `services.boomdrive` off
- **THEN** its profile holds none of the set's `pendingyou`, `plannotator` and `plannotator-tui`, on nix-darwin and on NixOS

### Requirement: Each activation writes Pending You's folder before it links the plugins

Where the companions are installed and Pending You's package is not `null`, each activation SHALL run, as the runner's user, the step that writes Pending You's folder, `<home>/.config/pendingyou`, before the step that links the plugins.

#### Scenario: The folder written on nix-darwin

- **WHEN** a nix-darwin system whose primary user is `alice`, at `/Users/alice`, turns on `services.boomdrive` and `services.crew`
- **THEN** its activation runs, as `alice`, the step that writes the folder's four files and its wake mod into `/Users/alice/.config/pendingyou`, before the step that links the plugins

#### Scenario: The folder written on NixOS

- **WHEN** a NixOS system turns on `services.boomdrive` and `services.crew` for `alice`
- **THEN** `alice`'s user activation runs the step that writes the folder into her home's `.config/pendingyou`, before the plugins' step, and another user's runs none

### Requirement: Pending You's folder holds its launchers, its manifest and its wake mod

The folder SHALL hold four files and the wake mod: `bin/pendingyou-hook` and `bin/pendingyou-mcp-headers`, each exporting `PENDINGYOU_CONFIG_DIR` as the folder and running the given Pending You; `claude-code.json`, Pending You's manifest, naming the origin `https://www.pendingyou.com` and the folder's helper; `signin/.claude.json`, naming the MCP server `pendingyou` with that helper; and `mod`, the given Pending You's wake mod, copied whole, as Pending You's `init` copies it into the folder.

#### Scenario: The launchers run the given Pending You

- **WHEN** `services.boomdrive.pendingyou.package` is a package of the host's own
- **THEN** both launchers run that package's `pendingyou`, and both export the folder as `PENDINGYOU_CONFIG_DIR`

#### Scenario: The wake mod in the folder

- **WHEN** the folder is read
- **THEN** its `mod` is the given Pending You's `mod`, copied whole

#### Scenario: A crew session's stop check

- **WHEN** the folder's `pendingyou-hook` is read
- **THEN** for `stopcheck` with `CREW_AGENT` set it runs `pendingyou` with its arguments, sends its standard output to `/dev/null` and exits 0, and for every other call it `exec`s `pendingyou` with its arguments

### Requirement: The folder's step writes only what differs

The step SHALL write each file of the folder's source whole, with the source's mode made writable by its owner, 0755 where the source runs and 0644 where it does not, by a temporary file renamed over its path, only where no regular file there has the same bytes and mode, and SHALL replace a link standing at its path. It SHALL make the folder with mode 0700 where it is missing, and say one line for each file it wrote and nothing where none differed.

#### Scenario: A fresh home

- **WHEN** the step runs in a home with no `.config/pendingyou`
- **THEN** the folder is made with mode 0700, holds the four files and the wake mod's as regular files, the launcher and the helper 0755 and the rest 0644, and one line names each

#### Scenario: Nothing differs

- **WHEN** the step runs again with the same source
- **THEN** no file is written and nothing is said

### Requirement: The folder's step writes the wake mod's modules before its hooks

The step SHALL write the wake mod's `hooks/wake.js`, `hooks/register.js` and `hooks/hooks.json` before the folder's other files, in that order, as Pending You's `init` does, so a session that reloads the mod finds the modules its hooks name.

#### Scenario: The wake mod's order

- **WHEN** the step writes the wake mod into a fresh home
- **THEN** it writes `mod/hooks/wake.js`, `mod/hooks/register.js` and `mod/hooks/hooks.json` first, in that order

### Requirement: The folder's step touches nothing else of the folder and stops nothing

The step SHALL read, write and remove nothing under the folder but the source's files. A file it cannot write SHALL be said in one line naming it, and the step SHALL exit 0.

#### Scenario: The command line's own files

- **WHEN** the folder holds a sign-in, a key and a `permission-cards/` folder of the command line's
- **THEN** the step leaves each as it was

#### Scenario: A link where a launcher goes

- **WHEN** `bin/pendingyou-hook` is a link into a store
- **THEN** the step replaces it with a regular file holding the source's bytes

#### Scenario: A file it cannot write

- **WHEN** the folder's `bin` cannot be written
- **THEN** one line names the file not written, and the step exits 0

### Requirement: The modules write no file of Claude Code's but its managed telemetry document

No activation step, launchd job or systemd unit of the modules SHALL run `claude`, name a Claude Code home or write a file of Claude Code's, whatever the host turns on, but the managed telemetry document in Claude Code's managed-settings folder where the host names a collector.

#### Scenario: No Claude Code of the modules'

- **WHEN** a system turns on `services.boomdrive` and `services.crew`, with a collector named and with none
- **THEN** no activation step, launchd job or systemd unit of the system runs `claude` or names `.claude`, `CLAUDE_CONFIG_DIR` or the set's marketplace, on nix-darwin and on NixOS

### Requirement: One sign-in command signs Claude Code's connection and herdr's in

`pendingyou-signin` SHALL export `PENDINGYOU_CONFIG_DIR` as `$HOME/.config/pendingyou`, run the given Pending You's `pendingyou login --app claude-code` with its arguments and `CLAUDE_CONFIG_DIR` the folder's `signin`, then run `pendingyou app login herdr` with its arguments but `--force`. No step of the modules SHALL run it.

#### Scenario: The sign-in's two commands

- **WHEN** `pendingyou-signin`'s script is read
- **THEN** given `--force` it runs `pendingyou login --app claude-code --force` with `CLAUDE_CONFIG_DIR` the folder's `signin`, then `pendingyou app login herdr` with no `--force`

### Requirement: The README names the companions and that no Claude Code home is installed into yet

The README's install with nix SHALL name Pending You, Plannotator and plannotator-tui in the set, and their herdr plugins. It SHALL say that they come with the runner where crew's module is on, and that the modules write Pending You's folder and leave its sign-ins, key and state the command line's. It SHALL name `pendingyou-signin` as the person's one sign-in, and say that the companions' Claude Code plugins are installed into no Claude Code home yet.

#### Scenario: The README's companions

- **WHEN** the README's install with nix is read
- **THEN** it names `pendingyou-signin`, `boomdrive-claude-plugins` and `~/.config/pendingyou`, and says that no Claude Code home is installed into yet

### Requirement: Where the host names a collector, the modules write Claude Code's managed telemetry document

With `services.boomdrive.enable` on and `services.boomdrive.claude-code.telemetry.collector` set, the modules SHALL write `10-boomdrive-telemetry.json`, whole, as a regular file of mode 0644, into Claude Code's managed-settings folder: `/Library/Application Support/ClaudeCode/managed-settings.d` on nix-darwin and `/etc/claude-code/managed-settings.d` on NixOS. They SHALL write no other file of Claude Code's managed settings.

#### Scenario: The document on NixOS

- **WHEN** a NixOS system turns on `services.boomdrive` and names the collector `http://localhost:4318`
- **THEN** `/etc` holds `claude-code/managed-settings.d/10-boomdrive-telemetry.json`, mode `0644`, its text the document, and no other entry under `claude-code`

#### Scenario: The document on nix-darwin

- **WHEN** a nix-darwin system turns on `services.boomdrive` and names the collector `http://localhost:4318`
- **THEN** its activation runs, as root, the step that writes the document to `/Library/Application Support/ClaudeCode/managed-settings.d/10-boomdrive-telemetry.json`, and no step writes `managed-settings.json`

### Requirement: Every session on the host reads the document, whatever its home

The document's path SHALL be the platform's managed-settings folder alone, in no Claude Code home, so every Claude Code session on the host reads it, whatever its account and configuration home.

#### Scenario: In no home

- **WHEN** a system names a collector, with crew's module on
- **THEN** the document's path names no home, `.claude` or `CLAUDE_CONFIG_DIR`, on nix-darwin and on NixOS

### Requirement: The managed telemetry document holds the telemetry keys alone

The document SHALL hold `env` and no other key, each key of its `env` one of Claude Code's telemetry keys and each value a string.

#### Scenario: The telemetry keys alone

- **WHEN** a system names a collector and turns on metrics, logs and traces
- **THEN** the document's keys are `env` alone, with string values, and each key of its `env` is one of `CLAUDE_CODE_ENABLE_TELEMETRY`, `CLAUDE_CODE_ENHANCED_TELEMETRY_BETA`, `ENABLE_BETA_TRACING_DETAILED`, `OTEL_METRICS_EXPORTER`, `OTEL_LOGS_EXPORTER`, `OTEL_TRACES_EXPORTER`, `OTEL_EXPORTER_OTLP_METRICS_ENDPOINT`, `OTEL_EXPORTER_OTLP_LOGS_ENDPOINT`, `OTEL_EXPORTER_OTLP_TRACES_ENDPOINT`, `OTEL_EXPORTER_OTLP_METRICS_PROTOCOL`, `OTEL_EXPORTER_OTLP_LOGS_PROTOCOL`, `OTEL_EXPORTER_OTLP_TRACES_PROTOCOL`, `OTEL_METRICS_INCLUDE_SESSION_ID`, `OTEL_METRICS_INCLUDE_ACCOUNT_UUID`, `OTEL_METRICS_INCLUDE_REPOSITORY`, `OTEL_LOG_USER_PROMPTS`, `OTEL_LOG_ASSISTANT_RESPONSES`, `OTEL_LOG_TOOL_DETAILS`, `OTEL_LOG_TOOL_CONTENT` and `OTEL_LOG_RAW_API_BODIES`, on nix-darwin and on NixOS

### Requirement: Each signal is exported as the host names it

`services.boomdrive.claude-code.telemetry.metrics`, `logs` and `traces` SHALL each say whether that signal is exported, `true`, `true` and `false` by default. The document SHALL set `CLAUDE_CODE_ENABLE_TELEMETRY` to `"1"`, each signal's exporter to `"otlp"` where it is on and `"none"` where it is off, and `CLAUDE_CODE_ENHANCED_TELEMETRY_BETA` to `"1"` with traces on and not at all with traces off.

#### Scenario: Metrics and logs by default

- **WHEN** a system names a collector and names no signal
- **THEN** the document's `OTEL_METRICS_EXPORTER` and `OTEL_LOGS_EXPORTER` are `otlp`, its `OTEL_TRACES_EXPORTER` is `none`, and it has no `CLAUDE_CODE_ENHANCED_TELEMETRY_BETA`

#### Scenario: Traces named

- **WHEN** a system names a collector and turns `traces` on and `logs` off
- **THEN** the document's `OTEL_TRACES_EXPORTER` is `otlp`, its `CLAUDE_CODE_ENHANCED_TELEMETRY_BETA` is `1`, and its `OTEL_LOGS_EXPORTER` is `none`

### Requirement: Each signal on goes to the collector by its own endpoint alone

For each signal on, the document SHALL set that signal's endpoint to the collector's address, any trailing `/` dropped, followed by `/v1/<signal>`, and its protocol to `"http/protobuf"`. For a signal off it SHALL set neither. It SHALL set no general endpoint, protocol or header, no headers helper, no detailed tracing endpoint and no resource attributes.

#### Scenario: Each signal's endpoint

- **WHEN** a system names the collector `http://127.0.0.1:4318/` and turns `traces` on and `logs` off
- **THEN** the document's metrics endpoint is `http://127.0.0.1:4318/v1/metrics` and its traces endpoint `http://127.0.0.1:4318/v1/traces`, each protocol `http/protobuf`, and it has no logs endpoint or logs protocol

#### Scenario: No general endpoint

- **WHEN** a system names a collector and turns on every signal
- **THEN** the document holds no `OTEL_EXPORTER_OTLP_ENDPOINT`, `OTEL_EXPORTER_OTLP_PROTOCOL`, key ending in `HEADERS`, `otelHeadersHelper`, `BETA_TRACING_ENDPOINT` or `OTEL_RESOURCE_ATTRIBUTES`

### Requirement: The collector is on the host

`services.boomdrive.claude-code.telemetry.collector` SHALL be `null` or an `http://` address whose host is `localhost`, `127.0.0.1` or `[::1]`, with a port and at most a path, and no user, query or fragment; any other value SHALL fail evaluation, naming the option. A collector named with every signal off SHALL fail evaluation, naming `metrics`, `logs` and `traces`.

#### Scenario: A collector off the host

- **WHEN** a system names the collector `http://collector.example.com:4318`, or `https://localhost:4318`
- **THEN** evaluation fails, naming `services.boomdrive.claude-code.telemetry.collector`

#### Scenario: A collector with no signal

- **WHEN** a system names a collector and turns `metrics`, `logs` and `traces` off
- **THEN** evaluation fails, naming `metrics`, `logs` and `traces`

### Requirement: No session's words leave in what it exports

The document SHALL set each of `OTEL_LOG_USER_PROMPTS`, `OTEL_LOG_ASSISTANT_RESPONSES`, `OTEL_LOG_TOOL_DETAILS`, `OTEL_LOG_TOOL_CONTENT`, `OTEL_LOG_RAW_API_BODIES` and `ENABLE_BETA_TRACING_DETAILED` to `"0"`, whatever the host turns on.

#### Scenario: The content settings off

- **WHEN** a system names a collector and turns on every signal
- **THEN** each of the six keys is `0` in the document, on nix-darwin and on NixOS

### Requirement: Each session carries its account and repository

The document SHALL set each of `OTEL_METRICS_INCLUDE_SESSION_ID`, `OTEL_METRICS_INCLUDE_ACCOUNT_UUID` and `OTEL_METRICS_INCLUDE_REPOSITORY` to `"1"`, so each session's metrics and events carry Claude Code's own session, account and repository attributes.

#### Scenario: The account and repository attributes on

- **WHEN** a system names a collector
- **THEN** each of the three keys is `1` in the document, on nix-darwin and on NixOS

### Requirement: With no collector, or without the runner, the set writes no managed telemetry document

Where `services.boomdrive.claude-code.telemetry.collector` is `null`, or `services.boomdrive.enable` is off, the modules SHALL write no managed telemetry document. On NixOS they SHALL declare no `/etc` entry under `claude-code`. On nix-darwin each activation of a system that imports the module SHALL remove `10-boomdrive-telemetry.json` from Claude Code's managed-settings folder where it stands, and nothing else there.

#### Scenario: No collector on NixOS

- **WHEN** a NixOS system turns on `services.boomdrive` and names no collector
- **THEN** `/etc` holds no entry under `claude-code`

#### Scenario: No collector on nix-darwin

- **WHEN** a nix-darwin system turns on `services.boomdrive` and names no collector
- **THEN** its activation runs, as root, the step that removes `/Library/Application Support/ClaudeCode/managed-settings.d/10-boomdrive-telemetry.json`, and no step writes a document

#### Scenario: Not without the runner

- **WHEN** a system names a collector and leaves `services.boomdrive.enable` off
- **THEN** NixOS's `/etc` holds no entry under `claude-code`, and nix-darwin's activation runs the step that removes the document

### Requirement: The managed step writes the document whole, and nothing else

`activate.sh managed <path> <source>` SHALL write `<source>` to `<path>` whole, as a regular file of mode 0644, by a temporary file beside it renamed over the path, only where no regular file there has the same bytes and mode, replacing a link there. It SHALL make the path's folder, mode 0755, where it is missing, and say one line naming the path where it wrote it. A write that fails SHALL say one line naming the path, and the step SHALL exit 0.

#### Scenario: A fresh host

- **WHEN** the step runs with a source and a path whose folder is missing
- **THEN** the folder is made with mode 0755, the path holds the source's bytes as a regular file of mode 0644, one line names the path, and the step exits 0

#### Scenario: Nothing differs

- **WHEN** the step runs with a source and the path already holds its bytes with mode 0644
- **THEN** nothing is written, nothing is said, and the step exits 0

#### Scenario: A link where the document goes

- **WHEN** the step runs with a source and a link stands at the path
- **THEN** the path holds the source's bytes as a regular file of mode 0644, and the link's target is unchanged

#### Scenario: A folder it cannot write

- **WHEN** the step runs with a source and the path's folder cannot be written
- **THEN** one line names the path not written, and the step exits 0

### Requirement: The managed step removes the document alone

`activate.sh managed <path>` SHALL remove the file or link at `<path>` where one stands, saying one line naming it, and say nothing where none stands. It SHALL read, write and remove nothing else in the folder, and exit 0.

#### Scenario: The document removed

- **WHEN** the step runs with no source, the path holds a file, and the folder holds another document
- **THEN** the path holds nothing, the other document is unchanged, one line names the path, and the step exits 0

#### Scenario: Nothing to remove

- **WHEN** the step runs with no source and nothing stands at the path
- **THEN** nothing is changed, nothing is said, and the step exits 0

### Requirement: The README names the sessions' telemetry

The README's install with nix SHALL name `services.boomdrive.claude-code.telemetry` and its `collector`, `metrics`, `logs` and `traces`, the document's name and its two folders, and that every Claude Code session on the host exports to the collector alone, whatever its home. It SHALL say that nothing is written without a collector or the runner, and that traces are Claude Code's beta, off unless named.

#### Scenario: The README's telemetry

- **WHEN** the README's install with nix is read
- **THEN** a nix block names `services.boomdrive.claude-code.telemetry` with a `collector`, and its words name `metrics`, `logs`, `traces`, `10-boomdrive-telemetry.json`, `/Library/Application Support/ClaudeCode/managed-settings.d` and `/etc/claude-code/managed-settings.d`, and say that traces are off unless named
