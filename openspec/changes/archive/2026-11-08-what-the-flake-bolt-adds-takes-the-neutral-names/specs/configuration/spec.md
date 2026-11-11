## MODIFIED Requirements

### Requirement: The local layer's repositories add to the managed layer's, and its partitions replace them

A drive both layers name under one account SHALL keep the managed layer's repositories, then each the local layer names that the managed layer does not, a repository both name kept once and refused in neither. A drive or an account only the local layer names SHALL be declared by it, after the managed layer's. A control plane's `partitions` the local layer sets SHALL replace the managed layer's whole (record 0009 as amended 2026-11-06; record 0028 as amended 2026-11-07).

#### Scenario: A repository both layers name

- **WHEN** the managed layer keeps `agentplot/flume` in the drive `home`, and the local layer names `AgentPlot/flume` and `agentplot/sluice` in the drive `home`
- **THEN** the runner keeps `agentplot/flume` once and `agentplot/sluice`, and refuses neither layer

#### Scenario: A drive only the local layer names

- **WHEN** the managed layer keeps `agentplot/flume` in the account `own`'s drive `home`, and the local layer names `example/crew` in the account `own`'s drive `work` and `agentplot/sluice` in the account `client`'s drive `garden`
- **THEN** the runner keeps the drives `home`, `work` and `garden`, in that order, each with its repository

#### Scenario: A repository the local layer drops

- **WHEN** a running runner keeps `agentplot/flume` from its managed layer and `agentplot/sluice` from its local layer, and the local layer is rewritten to name no repository in the drive
- **THEN** the next pass lets go of `agentplot/sluice` as a repository dropped, and keeps `agentplot/flume`

#### Scenario: The local layer's partitions

- **WHEN** the managed layer names `[control_plane]` with `type = "hosted"` and `[control_plane.hosted]` with a `url` and the partition `home`, and the local layer names the hosted partition `work` alone
- **THEN** the table handed to the hosted implementation holds the managed layer's `url` and the partition `work` alone

### Requirement: The managed layer may pin a key the local layer could set

The managed layer MAY set `pin`, a list of keys in TOML's dotted spelling, each a key the local layer may set, such as `"control_plane.hosted.partitions"`, or a table holding one, such as `"accounts"` or `"accounts.own.drives.home"`, which pins each such key under it. A key the local layer sets that a pin holds SHALL be passed over and named with its pin, and the rest of the local layer SHALL be taken (flume record 0032, point 4; core.17).

#### Scenario: Repositories pinned

- **WHEN** the managed layer says `pin = ["accounts"]` and keeps `agentplot/flume` in the drive `home`, and the local layer names `agentplot/sluice` in the drive `home` and `example/crew` in the drive `work`
- **THEN** the runner keeps `agentplot/flume` alone, and names `accounts.own.drives.home.repositories` and `accounts.own.drives.work.repositories` as passed over, pinned by `accounts`

#### Scenario: A table pinned

- **WHEN** the managed layer says `pin = ["control_plane"]`, keeps no repository in the drive `home` and names the hosted partition `home`, and the local layer names `agentplot/sluice` in the drive `home` and the hosted partition `work`
- **THEN** the runner keeps `agentplot/sluice`, the hosted table names the partition `home` alone, and the runner names `control_plane.hosted.partitions` as passed over, pinned by `control_plane`

### Requirement: The configuration names its accounts, each account's drives and each drive's repositories

The configuration SHALL name what the host keeps under `accounts`: a table of accounts, each holding `drives`, a table of drives, each holding `repositories`, the drive's repositories in the order kept. `accounts`, each account's `drives` and each drive's `repositories` SHALL be required, and each MAY be empty, so a host that keeps none says `accounts = {}`. The runner SHALL keep every drive's repositories, in the file's order (record 0028; core.5).

#### Scenario: One drive

- **WHEN** the configuration holds `[accounts.own.drives.home]` with `repositories = ["agentplot/flume"]`
- **THEN** it is read as keeping `agentplot/flume` in the drive `home` of the account `own`

#### Scenario: Drives of two accounts

- **WHEN** the account `own`'s drive `home` names `agentplot/flume` and `example/crew`, and then the account `client`'s drive `garden` names `agentplot/sluice`
- **THEN** the runner keeps `agentplot/flume`, `example/crew` and `agentplot/sluice`, in that order

#### Scenario: A drive with no list

- **WHEN** `[accounts.own.drives.home]` holds no `repositories`
- **THEN** it is refused, naming `accounts.own.drives.home.repositories` as required

#### Scenario: A drive keeping none

- **WHEN** the account `own`'s drive `home` says `repositories = []`
- **THEN** it is read with the drive `home` keeping no repository
