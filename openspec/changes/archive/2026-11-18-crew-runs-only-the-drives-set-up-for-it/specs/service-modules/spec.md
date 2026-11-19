## RENAMED Requirements

- FROM: `### Requirement: With crew's module on, the modules give crew each drive's machine and its account's state repository`
- TO: `### Requirement: With crew's module on, the modules give each drive crew's settings set up its machine and its account's state repository`

- FROM: `### Requirement: Under crew, each account with a drive names its state repository`
- TO: `### Requirement: Under crew, each account with a drive crew's settings set up names its state repository`

## MODIFIED Requirements

### Requirement: With crew's module on, the modules give each drive crew's settings set up its machine and its account's state repository

Where `services.crew.enable` is true, whether or not `services.boomdrive.enable` is, the modules SHALL give each drive the host's crew settings set up, `services.crew.settings.drives.<drive>`, that an account naming its `state_repository` names: its `machine`, the drive's, or `services.crew.host` where the drive names none; and its `state`, the account's `state_repository`. They SHALL add no drive to crew's settings, and give no other key (record 0032 as amended on 2026-11-18).

#### Scenario: Each drive's machine and state repository, given to crew

- **WHEN** crew's module is on with the host `host-a`, the hosts `host-a` and `host-b`, and crew's own keys for the drives `garden`, `home`, `away` and `field`, and `accounts` names the account `own`, naming the state repository `example/state`, with the drives `garden`, whose `machine` is `host-a`, and `home`, naming none, and the account `work`, naming `example/work`, with the drives `away`, whose `machine` is `host-b`, and `field`, naming none
- **THEN** crew's drives `garden` and `home` each have the `machine` `host-a` and the `state` `example/state`, `away` the `machine` `host-b` and the `state` `example/work`, and `field` the `machine` `host-a` and the `state` `example/work`; crew's teams file evaluates; the system fails no assertion; and the modules' definitions of crew's drive `home`'s `machine` and `state` are placed at `services.boomdrive.accounts`, on nix-darwin and on NixOS

#### Scenario: Given without the runner

- **WHEN** a system imports the platform's `default` module, leaves `services.boomdrive.enable` unset, and turns on crew and names the accounts as "Each drive's machine and state repository, given to crew" does
- **THEN** crew's drives have the same `machine` and `state`, on nix-darwin and on NixOS

#### Scenario: An account naming no state repository gives crew nothing

- **WHEN** crew's module is on with the host `host-a` and the drive `home`, whose `machine` `host-a` and `state` `example/state` crew's settings write, and `accounts` names the account `own`, naming no state repository, whose drive `home` keeps the owner `agentplot`'s `flume`
- **THEN** crew's drive `home` has the `machine` `host-a` and the `state` `example/state`, crew's teams file evaluates, and the system fails no assertion, on nix-darwin and on NixOS

#### Scenario: A drive crew's settings do not configure

- **WHEN** `accounts` names the accounts of "Each drive's machine and state repository, given to crew", and crew's module is on with the host `host-a`, the hosts `host-a` and `host-b`, and crew's own keys for the drive `home` alone
- **THEN** crew's drives are `home` alone, with the `machine` `host-a` and the `state` `example/state`; crew's teams file evaluates; and the system fails no assertion, on nix-darwin and on NixOS

### Requirement: A host writes neither a drive's machine nor its state in crew's settings

The modules SHALL give each drive's `machine` and `state` at the options' own priority, in definitions placed at `services.boomdrive.accounts`. So evaluation of crew's settings SHALL fail where a host also writes the `machine` or the `state` of a drive the modules give, and crew's failure names `services.boomdrive.accounts` among the places that write it (record 0032).

#### Scenario: A machine written twice

- **WHEN** `accounts` and crew's settings are those of "Each drive's machine and state repository, given to crew", and crew's settings also give the drive `home` the `machine` `host-a`
- **THEN** evaluation of crew's settings fails, and the modules' definition of crew's drive `home`'s `machine` is placed at `services.boomdrive.accounts`, on nix-darwin and on NixOS

#### Scenario: A state written twice

- **WHEN** `accounts` and crew's settings are those of "Each drive's machine and state repository, given to crew", and crew's settings also give the drive `home` the `state` `example/state`
- **THEN** evaluation of crew's settings fails, and the modules' definition of crew's drive `home`'s `state` is placed at `services.boomdrive.accounts`, on nix-darwin and on NixOS

### Requirement: The modules bring each tool's own module

`darwinModules.default` and `nixosModules.default`, each the same value as the platform's `boomdrive`, SHALL import the runner's module and Flume's, sluice's, crew's and timber's, each the module that tool's flake gives at the release tag the lock names. Each option of `services.flume`, `services.sluice`, `services.crew` and `services.timber` SHALL be declared in its tool's input; this tree SHALL declare one again only to add to its submodule type a module giving facts the set knows (core.57).

#### Scenario: Each tool's options are its own flake's

- **WHEN** a system imports the platform's `default` module
- **THEN** every option of `services.flume`, `services.sluice`, `services.crew` and `services.timber` is declared in a file of that tool's input; none is declared again in a file of this tree but `services.crew.settings` and its `drives`, each in `nix/modules/crew-drives.nix`, setting no description, default or example; and every other option of crew's settings, and every option of a drive of crew's, is declared in crew's input alone, on nix-darwin and on NixOS

### Requirement: Under crew, the runner's crew table is written from the accounts' drives on this host

Where `settings.control_plane.type` is `crew`, crew's module is on and the host writes no `settings.control_plane.crew`, the modules SHALL write it from the accounts: `host` as `services.crew.host`, and a partition for each drive of this host that crew's settings set up, under an account naming its `state_repository`, holding the drive's name as `label` and that `state_repository` alone, accounts and drives in the order of their names (records 0023 and 0032).

#### Scenario: The accounts' drives on this host

- **WHEN** `accounts` names the account `own`, naming the state repository `example/state`, whose drive `home` keeps the owner `agentplot`'s `flume` and `sluice`, and `settings` names `repository_root = "~/Code"`, `fetch_interval = "2m"`, `code_workspaces = true` and `control_plane.type = "crew"` with no `control_plane.crew`, and crew's module is on with the host `host-a` and crew's own keys for the drive `home`
- **THEN** the file is `nix/tests/files/crew.toml`, byte for byte, on nix-darwin and on NixOS

#### Scenario: The accounts' drives in their order

- **WHEN** `control_plane.type = "crew"` is set with no `control_plane.crew`, and crew's module and the accounts are those of "Each drive's machine and state repository, given to crew"
- **THEN** `settings.control_plane.crew` is the host `host-a` with the partitions `garden` at `example/state`, `home` at `example/state` and `field` at `example/work`, in that order, each holding `label` and `state_repository` alone, on nix-darwin and on NixOS

#### Scenario: Only the drives crew's settings set up

- **WHEN** `control_plane.type = "crew"` is set with no `control_plane.crew`, `accounts` names the accounts of "Each drive's machine and state repository, given to crew", and crew's module is on with the host `host-a`, the hosts `host-a` and `host-b`, and crew's own keys for the drive `home` alone
- **THEN** `settings.control_plane.crew` is the host `host-a` with the one partition `home` at `example/state`, and the system fails no assertion, on nix-darwin and on NixOS

### Requirement: Under crew, a host the runner cannot serve from its accounts fails evaluation

With `services.boomdrive.enable` and `settings.control_plane.type` `crew`, evaluation SHALL fail where `settings.control_plane.crew` is written neither by the host nor by the modules, naming it and `services.crew`; and where the modules would write the table and it holds no drive, naming `services.crew.host`, its value, and each `machine` the accounts' drives that crew's settings set up name.

#### Scenario: Under crew with neither

- **WHEN** `settings` names `control_plane.type = "crew"` and no `control_plane.crew`, and crew's module is off
- **THEN** evaluation fails, naming `settings.control_plane.crew` and `services.crew`, on nix-darwin and on NixOS

#### Scenario: No drive of the accounts on this host

- **WHEN** `settings` names `control_plane.type = "crew"` and no `control_plane.crew`, `accounts` names the one account `work`, naming the state repository `example/away`, whose one drive `away` names the `machine` `host-b`, and crew's module is on with the host `host-a` and crew's own keys for the drive `away`
- **THEN** evaluation fails, naming `services.crew.host`, `host-a` and `host-b`, on nix-darwin and on NixOS

#### Scenario: No drive crew runs on this host

- **WHEN** `settings` names `control_plane.type = "crew"` and no `control_plane.crew`, `accounts` names the account `work` of "No drive of the accounts on this host" and the account `own`, naming the state repository `example/state`, whose drive `home`, naming no machine, keeps the owner `agentplot`'s `flume`, and crew's module is on with the host `host-a` and crew's own keys for the drive `away` alone
- **THEN** evaluation fails, naming `services.crew.host`, `host-a` and `host-b`, on nix-darwin and on NixOS

### Requirement: Under crew, each account with a drive crew's settings set up names its state repository

With `services.boomdrive.enable`, `settings.control_plane.type` `crew` and crew's module on, evaluation SHALL fail where an account that names no `state_repository` holds a drive crew's settings set up, its words naming `services.boomdrive.accounts.<account>.state_repository` for each such account. An account none of whose drives crew's settings set up SHALL need none (records 0028 and 0032 as amended on 2026-11-18; core.19).

#### Scenario: An account under crew with no state repository

- **WHEN** `settings` names `control_plane.type = "crew"` and no `control_plane.crew`, `accounts` names the account `own`, naming no state repository, whose drive `home` keeps the owner `agentplot`'s `flume` and `sluice`, and crew's module is on with the host `host-a` and the drive `home`, whose `machine` `host-a` and `state` `example/state` crew's settings write
- **THEN** evaluation fails, naming `services.boomdrive.accounts.own.state_repository`, on nix-darwin and on NixOS

#### Scenario: An account whose drives crew's settings do not set up

- **WHEN** `settings` names `control_plane.type = "crew"` and no `control_plane.crew`, `accounts` names the account `own`, naming the state repository `example/state`, whose drive `home` keeps the owner `agentplot`'s `flume`, and the account `work`, naming no state repository, whose drive `field` keeps the owner `agentplot`'s `sluice`, and crew's module is on with the host `host-a`, the hosts `host-a` and `host-b`, and crew's own keys for the drive `home` alone
- **THEN** the system fails no assertion, crew's drives are `home` alone, `settings.control_plane.crew` is the host `host-a` with the one partition `home` at `example/state`, and Flume is given the streams `home` and `field`, on nix-darwin and on NixOS

### Requirement: The README writes no crew table beside crew's drives

The README's install with nix SHALL set the runner's `control_plane.type = "crew"` beside `services.crew` and write no `control_plane.crew`. Its words SHALL say the module writes `[control_plane.crew]` from the drives of `services.boomdrive.accounts` that crew's settings set up, whose `machine` is `services.crew.host` or that name none, under an account that names its `state_repository`, and that a table written out wins.

#### Scenario: The README's crew table

- **WHEN** the README's install with nix is read
- **THEN** a nix block in it sets `type = "crew"`, no nix block in it names `control_plane.crew` or `partitions`, and its words name `services.crew.host` and say a table written out wins

### Requirement: The README names each drive's machine and its account's state repository once

The README's install with nix SHALL write each account's `state_repository` in `services.boomdrive.accounts`, and neither `state` nor `machine` in `services.crew.settings`. Its words SHALL say that, with crew's module on, the module gives each drive crew's settings set up its `machine` and its account's `state_repository`, and nothing of another drive, that an account needs one only for such a drive, and that a host writing either in crew's settings fails evaluation.

#### Scenario: The README's machine and state

- **WHEN** the README's install with nix is read
- **THEN** the nix block in it that names `services.crew` names `state_repository = "example/state"` before `services.crew = {`, and neither `state =` nor `machine =` after it, and its words name `services.boomdrive.accounts.<account>.state_repository`
