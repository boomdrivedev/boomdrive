# Service modules

## REMOVED Requirements

### Requirement: The options are the runner's file

**Reason**: The runner's file names its repositories under its accounts and their drives, not in a flat `repositories` (record 0028). Its successor, "The options are the runner's file, its accounts always written", renders `accounts` always in place of `repositories`.

**Migration**: A host's `settings` names each repository under its drive, `settings.accounts.<account>.drives.<drive>.repositories`, in place of `settings.repositories`. One that still names `repositories` fails evaluation, saying so.

## ADDED Requirements

### Requirement: The options are the runner's file, its accounts always written

`services.boomdrive.settings` SHALL hold the runner's file, each key named as the file names it, among them `accounts`, `repository_root`, `fetch_interval`, `code_workspaces` and `control_plane`. Each module SHALL render it as TOML at evaluation, a key written only where it is set and `accounts`, each account's `drives` and each drive's `repositories` always, so the runner's own defaults hold for the rest. One `settings` SHALL render one text on either platform.

#### Scenario: Only accounts

- **WHEN** `settings` sets nothing
- **THEN** the file's settings are an empty `[accounts]` table alone, after the module's header line

#### Scenario: A host under crew

- **WHEN** `settings` names `repository_root = "~/Code"`, `fetch_interval = "2m"`, `code_workspaces = true`, the account `own` whose drive `home` keeps `agentplot/flume` and `agentplot/sluice`, and a crew control plane for the host `host-a` with the partition `home` at `example/state`
- **THEN** the file is `nix/tests/files/crew.toml`, byte for byte, on nix-darwin and on NixOS

#### Scenario: A host under a hosted control plane

- **WHEN** `settings` names the account `home`, whose drive `garden` keeps `agentplot/flume`, the account `work` with no drive, and a hosted control plane at `https://control.example` with the partitions `home`, serving `agentplot/flume`, and `work`, serving none
- **THEN** the file is `nix/tests/files/hosted.toml`, byte for byte

#### Scenario: The runner reads each file

- **WHEN** the runner reads each file under `nix/tests/files`
- **THEN** it reads it, refusing no key, with the accounts, drives, repositories and control plane its `settings` named

### Requirement: A flat list of before fails evaluation

Evaluation SHALL fail where `settings` names `repositories` at its top, saying each repository is named under its drive, `settings.accounts.<account>.drives.<drive>.repositories` (record 0028). A key in an account or a drive that the runner does not read SHALL fail evaluation, so no file holding it is written.

#### Scenario: The flat list in settings

- **WHEN** `settings.repositories = [ "agentplot/flume" ]` is set
- **THEN** evaluation fails, naming `settings.repositories` and `settings.accounts.<account>.drives.<drive>.repositories`

#### Scenario: A stream in a drive's settings

- **WHEN** `settings.accounts.own.drives.home.stream = "home"` is set
- **THEN** evaluation fails, and no file is rendered
