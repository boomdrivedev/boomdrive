## MODIFIED Requirements

### Requirement: Under crew, the runner's crew table is written from the accounts' drives on this host

Where an account names crew, crew's module is on and the host writes no `settings.control_plane.crew`, the modules SHALL write it from the accounts: `host` as `services.crew.host`, and a partition for each drive of this host that crew's settings set up, under an account naming crew and its `state_repository`, holding the drive's name as `label` and that `state_repository` alone, accounts and drives in the order of their names (records 0023 and 0032).

#### Scenario: The accounts' drives on this host

- **WHEN** `accounts` names the account `own`, naming the state repository `example/state` and `control_plane = "crew"`, whose drive `home` keeps the owner `agentplot`'s `flume` and `sluice`, and `settings` names `repository_root = "~/Code"`, `fetch_interval = "2m"` and `code_workspaces = true` with no `control_plane.crew`, and crew's module is on with the host `host-a` and crew's own keys for the drive `home`
- **THEN** the file is `nix/tests/files/crew.toml`, byte for byte, on nix-darwin and on NixOS

#### Scenario: The accounts' drives in their order

- **WHEN** each account names `control_plane = "crew"`, `settings` names no `control_plane.crew`, and crew's module and the accounts are those of "Each drive's machine and state repository, given to crew"
- **THEN** `settings.control_plane.crew` is the host `host-a` with the partitions `garden` at `example/state`, `home` at `example/state` and `field` at `example/work`, in that order, each holding `label` and `state_repository` alone, on nix-darwin and on NixOS

#### Scenario: Only the drives crew's settings set up

- **WHEN** `settings` names no `control_plane.crew`, `accounts` names the accounts of "Each drive's machine and state repository, given to crew", each naming `control_plane = "crew"`, and crew's module is on with the host `host-a`, the hosts `host-a` and `host-b`, and crew's own keys for the drive `home` alone
- **THEN** `settings.control_plane.crew` is the host `host-a` with the one partition `home` at `example/state`, and the system fails no assertion, on nix-darwin and on NixOS

### Requirement: Under crew, a host the runner cannot serve from its accounts fails evaluation

With `services.boomdrive.enable` and an account naming `control_plane = "crew"`, evaluation SHALL fail where `settings.control_plane.crew` is written neither by the host nor by the modules, naming it and `services.crew`; and where the modules would write the table and it holds no drive, naming `services.crew.host`, its value, and each `machine` the accounts' drives that crew's settings set up name.

#### Scenario: Under crew with neither

- **WHEN** an account names `control_plane = "crew"`, `settings` names no `control_plane.crew`, and crew's module is off
- **THEN** evaluation fails, naming `settings.control_plane.crew` and `services.crew`, on nix-darwin and on NixOS

#### Scenario: No drive of the accounts on this host

- **WHEN** an account names `control_plane = "crew"`, `settings` names no `control_plane.crew`, `accounts` names the one account `work`, naming the state repository `example/away`, whose one drive `away` names the `machine` `host-b`, and crew's module is on with the host `host-a` and crew's own keys for the drive `away`
- **THEN** evaluation fails, naming `services.crew.host`, `host-a` and `host-b`, on nix-darwin and on NixOS

#### Scenario: No drive crew runs on this host

- **WHEN** an account names `control_plane = "crew"`, `settings` names no `control_plane.crew`, `accounts` names the account `work` of "No drive of the accounts on this host" and the account `own`, naming the state repository `example/state`, whose drive `home`, naming no machine, keeps the owner `agentplot`'s `flume`, and crew's module is on with the host `host-a` and crew's own keys for the drive `away` alone
- **THEN** evaluation fails, naming `services.crew.host`, `host-a` and `host-b`, on nix-darwin and on NixOS

### Requirement: Under crew, each account with a drive crew's settings set up names its state repository

With `services.boomdrive.enable` and crew's module on, evaluation SHALL fail where an account that names `control_plane = "crew"` and no `state_repository` holds a drive crew's settings set up, its words naming `services.boomdrive.accounts.<account>.state_repository` for each such account. An account none of whose drives crew's settings set up SHALL need none (records 0028 and 0032 as amended on 2026-11-18; core.19).

#### Scenario: An account under crew with no state repository

- **WHEN** `settings` names no `control_plane.crew`, `accounts` names the account `own`, naming `control_plane = "crew"` and no state repository, whose drive `home` keeps the owner `agentplot`'s `flume` and `sluice`, and crew's module is on with the host `host-a` and the drive `home`, whose `machine` `host-a` and `state` `example/state` crew's settings write
- **THEN** evaluation fails, naming `services.boomdrive.accounts.own.state_repository`, on nix-darwin and on NixOS

#### Scenario: An account whose drives crew's settings do not set up

- **WHEN** `settings` names no `control_plane.crew`, `accounts` names the account `own`, naming the state repository `example/state` and `control_plane = "crew"`, whose drive `home` keeps the owner `agentplot`'s `flume`, and the account `work`, naming `control_plane = "crew"` and no state repository, whose drive `field` keeps the owner `agentplot`'s `sluice`, and crew's module is on with the host `host-a`, the hosts `host-a` and `host-b`, and crew's own keys for the drive `home` alone
- **THEN** the system fails no assertion, crew's drives are `home` alone, `settings.control_plane.crew` is the host `host-a` with the one partition `home` at `example/state`, and Flume is given the streams `home` and `field`, on nix-darwin and on NixOS

### Requirement: Under crew, each drive of crew's is a drive of the accounts

With `services.boomdrive.enable`, an account naming `control_plane = "crew"` and crew's module on, evaluation SHALL fail where `services.crew.settings.drives` names a drive that no account of `services.boomdrive.accounts` names, its words naming each such drive and `services.boomdrive.accounts` (record 0032).

#### Scenario: A drive of crew's no account names

- **WHEN** `settings` names no `control_plane.crew`, `accounts` names the account `own`, naming the state repository `example/state` and `control_plane = "crew"`, whose drive `home` keeps the owner `agentplot`'s `flume` and `sluice`, and crew's module is on with the host `host-a`, crew's own keys for the drive `home`, and the drive `stray`, whose `machine` `host-a` and `state` `example/state` crew's settings write
- **THEN** evaluation fails, naming `stray` and `services.boomdrive.accounts`, on nix-darwin and on NixOS

### Requirement: No crew table beside another control plane

Where no account names `control_plane = "crew"`, the modules SHALL write no `[control_plane.crew]`, whatever crew's module holds.

#### Scenario: A hosted control plane beside crew

- **WHEN** `accounts` names the account `home`, whose drive `garden` keeps the owner `agentplot`'s `flume`, and the account `work` with no drive, each naming `control_plane = "hosted"`, and `settings` names a hosted control plane at `https://control.example` with the partitions `home` and `work`, and crew's module is on with the host `host-a` and the drive `home` on `host-a`
- **THEN** the file is `nix/tests/files/hosted.toml`, byte for byte, on nix-darwin and on NixOS

#### Scenario: No control plane beside crew

- **WHEN** neither `accounts` nor `settings` sets anything, and crew's module is on with the host `host-a` and the drive `home` on `host-a`
- **THEN** the file is `nix/tests/files/empty.toml`, byte for byte, on nix-darwin and on NixOS

### Requirement: The README writes no crew table beside crew's drives

The README's install with nix SHALL set an account's `control_plane = "crew"` beside `services.crew` and write no `control_plane.crew`. Its words SHALL say the module writes `[control_plane.crew]` from the drives of `services.boomdrive.accounts` that crew's settings set up, whose `machine` is `services.crew.host` or that name none, under an account that names its `state_repository`, and that a table written out wins.

#### Scenario: The README's crew table

- **WHEN** the README's install with nix is read
- **THEN** a nix block in it sets `control_plane = "crew"` in an account, no nix block in it names `control_plane.crew` or `partitions`, and its words name `services.crew.host` and say a table written out wins

## ADDED Requirements

### Requirement: Each account's control plane is written into its account

`services.boomdrive.accounts.<account>.control_plane` SHALL be `null`, `"crew"` or `"hosted"`, `null` where unset, and SHALL be written as the account's `control_plane` where it is not `null`. `settings.control_plane.type` SHALL fail evaluation, naming `services.boomdrive.accounts.<account>.control_plane` (record 0054).

#### Scenario: An account's plane written

- **WHEN** the account `own` names `control_plane = "crew"` and the account `cloud` names `control_plane = "hosted"`
- **THEN** the file's `[accounts.own]` says `control_plane = "crew"` and its `[accounts.cloud]` says `control_plane = "hosted"`, on nix-darwin and on NixOS

#### Scenario: A type of before

- **WHEN** `settings` names `control_plane.type = "crew"`
- **THEN** evaluation fails, naming `services.boomdrive.accounts.<account>.control_plane`, on nix-darwin and on NixOS
