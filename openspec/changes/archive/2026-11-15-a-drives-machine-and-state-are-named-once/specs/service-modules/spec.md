## MODIFIED Requirements

### Requirement: The options are the runner's file, its accounts always written

`services.boomdrive.accounts` and `services.boomdrive.settings` SHALL hold the runner's file: `accounts` its `accounts`, and `settings` every other key, each named as the file names it. Each module SHALL render them as TOML at evaluation, a key of `settings` written only where it is set, so the runner's own default holds, and `accounts`, each account's `drives` and each drive's `repositories` always. One `accounts` and `settings` SHALL render one text on either platform.

#### Scenario: Only accounts

- **WHEN** neither `accounts` nor `settings` sets anything
- **THEN** the file's settings are an empty `[accounts]` table alone, after the module's header line

#### Scenario: A host under crew

- **WHEN** `accounts` names the account `own`, naming the state repository `example/state`, whose drive `home` keeps the owner `agentplot`'s `flume` and `sluice`, and `settings` names `repository_root = "~/Code"`, `fetch_interval = "2m"`, `code_workspaces = true` and a crew control plane for the host `host-a` with the partition `home` at `example/state`
- **THEN** the file is `nix/tests/files/crew.toml`, byte for byte, on nix-darwin and on NixOS

#### Scenario: A host under a hosted control plane

- **WHEN** `accounts` names the account `home`, whose drive `garden` keeps the owner `agentplot`'s `flume`, and the account `work` with no drive, and `settings` names a hosted control plane at `https://control.example` with the partitions `home`, serving `agentplot/flume`, and `work`, serving none
- **THEN** the file is `nix/tests/files/hosted.toml`, byte for byte

#### Scenario: The runner reads each file

- **WHEN** the runner reads each file under `nix/tests/files`
- **THEN** it reads it, refusing no key, with the accounts, drives, repositories and control plane its `accounts` and `settings` named

### Requirement: A crew table written out wins whole

Where the host writes `settings.control_plane.crew`, the file's `[control_plane.crew]` SHALL be that table, whole, with none of the accounts' drives merged into it.

#### Scenario: A table written out wins

- **WHEN** `settings` names a crew control plane whose table is the host `host-a` with the partition `garden` at `example/garden` alone, `accounts` names the account `own`, naming the state repository `example/state`, whose drive `home` keeps the owner `agentplot`'s `flume` and `sluice`, and crew's module is on with the host `host-a` and crew's own keys for the drive `home`
- **THEN** `settings.control_plane.crew` is the table written, the partition `garden` alone, and the file is that table's rendering, on nix-darwin and on NixOS

### Requirement: The README writes no crew table beside crew's drives

The README's install with nix SHALL set the runner's `control_plane.type = "crew"` beside `services.crew` and write no `control_plane.crew`. Its words SHALL say the module writes `[control_plane.crew]` from the drives of `services.boomdrive.accounts` whose `machine` is `services.crew.host`, or that name none, under an account that names its `state_repository`, and that a table written out wins.

#### Scenario: The README's crew table

- **WHEN** the README's install with nix is read
- **THEN** a nix block in it sets `type = "crew"`, no nix block in it names `control_plane.crew` or `partitions`, and its words name `services.crew.host` and say a table written out wins

## REMOVED Requirements

### Requirement: Under crew, the runner's crew table is written from crew's drives on this host

**Reason**: The runner's crew table is read from the host's accounts, which name each drive's machine and its account's crew state repository once (record 0032). "Under crew, the runner's crew table is written from the accounts' drives on this host" holds what stands of it.

**Migration**: A host under crew names each account's `state_repository` and each drive's `machine` in `services.boomdrive.accounts`, and drops each drive's `machine` and `state` from `services.crew.settings`. The table the modules write from them is the same.

### Requirement: Under crew, a host the runner cannot serve fails evaluation

**Reason**: Its words named crew's drives. "Under crew, a host the runner cannot serve from its accounts fails evaluation" holds it, naming the accounts' drives.

**Migration**: None: the failures are the same, their words naming the accounts.

## ADDED Requirements

### Requirement: Each account names its crew state repository

`services.boomdrive.accounts.<account>.state_repository` SHALL be declared, unset by default: the account's crew state repository, an address `<owner>/<name>`. A value that is no address of a GitHub repository SHALL fail the option's type. It SHALL be no key of the runner's file (core.5; record 0028).

#### Scenario: A state repository of no address

- **WHEN** `accounts.own.state_repository = "example"` is set
- **THEN** evaluation of `services.boomdrive.accounts` fails

### Requirement: With crew's module on, the modules give crew each drive's machine and its account's state repository

Where `services.crew.enable` is true, whether or not `services.boomdrive.enable` is, the modules SHALL give `services.crew.settings.drives.<drive>` for each drive of each account that names its `state_repository`, this host's and others': its `machine`, the drive's `machine`, or `services.crew.host` where the drive names none; and its `state`, the account's `state_repository`. They SHALL give no other key of crew's drive, and nothing of an account that names no `state_repository` (record 0032).

#### Scenario: Each drive's machine and state repository, given to crew

- **WHEN** crew's module is on with the host `host-a`, the hosts `host-a` and `host-b`, and crew's own keys for the drives `garden`, `home`, `away` and `field`, and `accounts` names the account `own`, naming the state repository `example/state`, with the drives `garden`, whose `machine` is `host-a`, and `home`, naming none, and the account `work`, naming `example/work`, with the drives `away`, whose `machine` is `host-b`, and `field`, naming none
- **THEN** crew's drives `garden` and `home` each have the `machine` `host-a` and the `state` `example/state`, `away` the `machine` `host-b` and the `state` `example/work`, and `field` the `machine` `host-a` and the `state` `example/work`; crew's teams file evaluates; the system fails no assertion; and the modules' definition of `services.crew.settings` is placed at `services.boomdrive.accounts`, on nix-darwin and on NixOS

#### Scenario: Given without the runner

- **WHEN** a system imports the platform's `default` module, leaves `services.boomdrive.enable` unset, and turns on crew and names the accounts as "Each drive's machine and state repository, given to crew" does
- **THEN** crew's drives have the same `machine` and `state`, on nix-darwin and on NixOS

#### Scenario: An account naming no state repository gives crew nothing

- **WHEN** crew's module is on with the host `host-a` and the drive `home`, whose `machine` `host-a` and `state` `example/state` crew's settings write, and `accounts` names the account `own`, naming no state repository, whose drive `home` keeps the owner `agentplot`'s `flume`
- **THEN** crew's drive `home` has the `machine` `host-a` and the `state` `example/state`, crew's teams file evaluates, and the system fails no assertion, on nix-darwin and on NixOS

#### Scenario: A drive crew's settings do not configure

- **WHEN** `accounts` names the accounts of "Each drive's machine and state repository, given to crew", and crew's module is on with the host `host-a` and crew's own keys for the drive `home` alone
- **THEN** evaluation of crew's settings fails, on nix-darwin and on NixOS

### Requirement: A host writes neither a drive's machine nor its state in crew's settings

The modules SHALL give each drive's `machine` and `state` at the options' own priority, in a definition placed at `services.boomdrive.accounts`. So evaluation of crew's settings SHALL fail where a host also writes the `machine` or the `state` of a drive the accounts give, and crew's failure names `services.boomdrive.accounts` among the places that write it (record 0032).

#### Scenario: A machine written twice

- **WHEN** `accounts` and crew's settings are those of "Each drive's machine and state repository, given to crew", and crew's settings also give the drive `home` the `machine` `host-a`
- **THEN** evaluation of crew's settings fails, and the modules' definition of `services.crew.settings` is placed at `services.boomdrive.accounts`, on nix-darwin and on NixOS

#### Scenario: A state written twice

- **WHEN** `accounts` and crew's settings are those of "Each drive's machine and state repository, given to crew", and crew's settings also give the drive `home` the `state` `example/state`
- **THEN** evaluation of crew's settings fails, and the modules' definition of `services.crew.settings` is placed at `services.boomdrive.accounts`, on nix-darwin and on NixOS

### Requirement: Under crew, the runner's crew table is written from the accounts' drives on this host

Where `settings.control_plane.type` is `crew`, crew's module is on and the host writes no `settings.control_plane.crew`, the modules SHALL write it from the accounts: `host` as `services.crew.host`, and one partition for each drive of this host under an account that names its `state_repository`, holding the drive's name as `label` and its account's `state_repository` alone, accounts and each account's drives in the order of their names (records 0023 and 0032).

#### Scenario: The accounts' drives on this host

- **WHEN** `accounts` names the account `own`, naming the state repository `example/state`, whose drive `home` keeps the owner `agentplot`'s `flume` and `sluice`, and `settings` names `repository_root = "~/Code"`, `fetch_interval = "2m"`, `code_workspaces = true` and `control_plane.type = "crew"` with no `control_plane.crew`, and crew's module is on with the host `host-a` and crew's own keys for the drive `home`
- **THEN** the file is `nix/tests/files/crew.toml`, byte for byte, on nix-darwin and on NixOS

#### Scenario: The accounts' drives in their order

- **WHEN** `control_plane.type = "crew"` is set with no `control_plane.crew`, and crew's module and the accounts are those of "Each drive's machine and state repository, given to crew"
- **THEN** `settings.control_plane.crew` is the host `host-a` with the partitions `garden` at `example/state`, `home` at `example/state` and `field` at `example/work`, in that order, each holding `label` and `state_repository` alone, on nix-darwin and on NixOS

### Requirement: Under crew, a host the runner cannot serve from its accounts fails evaluation

With `services.boomdrive.enable` and `settings.control_plane.type` `crew`, evaluation SHALL fail where `settings.control_plane.crew` is written neither by the host nor by the modules, naming it and `services.crew`; and where the modules would write the table and it holds no drive, naming `services.crew.host`, its value, and each `machine` the accounts' drives name.

#### Scenario: Under crew with neither

- **WHEN** `settings` names `control_plane.type = "crew"` and no `control_plane.crew`, and crew's module is off
- **THEN** evaluation fails, naming `settings.control_plane.crew` and `services.crew`, on nix-darwin and on NixOS

#### Scenario: No drive of the accounts on this host

- **WHEN** `settings` names `control_plane.type = "crew"` and no `control_plane.crew`, `accounts` names the one account `work`, naming the state repository `example/away`, whose one drive `away` names the `machine` `host-b`, and crew's module is on with the host `host-a` and crew's own keys for the drive `away`
- **THEN** evaluation fails, naming `services.crew.host`, `host-a` and `host-b`, on nix-darwin and on NixOS

### Requirement: Under crew, each account with a drive names its state repository

With `services.boomdrive.enable`, `settings.control_plane.type` `crew` and crew's module on, evaluation SHALL fail where an account that names a drive names no `state_repository`, its words naming `services.boomdrive.accounts.<account>.state_repository` for each such account (records 0028 and 0032; core.19).

#### Scenario: An account under crew with no state repository

- **WHEN** `settings` names `control_plane.type = "crew"` and no `control_plane.crew`, `accounts` names the account `own`, naming no state repository, whose drive `home` keeps the owner `agentplot`'s `flume` and `sluice`, and crew's module is on with the host `host-a` and the drive `home`, whose `machine` `host-a` and `state` `example/state` crew's settings write
- **THEN** evaluation fails, naming `services.boomdrive.accounts.own.state_repository`, on nix-darwin and on NixOS

### Requirement: Under crew, each drive of crew's is a drive of the accounts

With `services.boomdrive.enable`, `settings.control_plane.type` `crew` and crew's module on, evaluation SHALL fail where `services.crew.settings.drives` names a drive that no account of `services.boomdrive.accounts` names, its words naming each such drive and `services.boomdrive.accounts` (record 0032).

#### Scenario: A drive of crew's no account names

- **WHEN** `settings` names `control_plane.type = "crew"` and no `control_plane.crew`, `accounts` names the account `own`, naming the state repository `example/state`, whose drive `home` keeps the owner `agentplot`'s `flume` and `sluice`, and crew's module is on with the host `host-a`, crew's own keys for the drive `home`, and the drive `stray`, whose `machine` `host-a` and `state` `example/state` crew's settings write
- **THEN** evaluation fails, naming `stray` and `services.boomdrive.accounts`, on nix-darwin and on NixOS

### Requirement: The README names each drive's machine and its account's state repository once

The README's install with nix SHALL write each account's `state_repository` in `services.boomdrive.accounts`, and neither `state` nor `machine` in `services.crew.settings`. Its words SHALL say that, with crew's module on, the module gives crew each drive's `machine` and its account's `state_repository`, and that a host writing either in crew's settings fails evaluation.

#### Scenario: The README's machine and state

- **WHEN** the README's install with nix is read
- **THEN** the nix block in it that names `services.crew` names `state_repository = "example/state"` before `services.crew = {`, and neither `state =` nor `machine =` after it, and its words name `services.boomdrive.accounts.<account>.state_repository`
