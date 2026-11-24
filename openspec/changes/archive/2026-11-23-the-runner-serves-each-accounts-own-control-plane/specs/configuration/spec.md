## MODIFIED Requirements

### Requirement: The configuration names its accounts, each account's drives and each drive's repositories

The configuration SHALL name what the host keeps under `accounts`: a table of accounts, each holding `drives`, a table of drives, each holding `repositories`, the drive's repositories in the order kept, and MAY hold `control_plane`. `accounts`, each account's `drives` and each drive's `repositories` SHALL be required, and each MAY be empty, so a host that keeps none says `accounts = {}`. The runner SHALL keep every drive's repositories, in the file's order (record 0028; core.5).

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

### Requirement: The App is named only in the managed layer of a host that holds it

Where every account names a control plane that holds the App, as the hosted control plane does, `[forge]` SHALL be refused, naming it. Where any account names another plane, or none, `[forge]` MAY be named. The local layer SHALL not set `[forge]` (record 0007 as amended on 2026-11-09; record 0054; core.17, core.33).

#### Scenario: The App beside a hosted control plane

- **WHEN** the configuration names `[forge]`, and its one account says `control_plane = "hosted"`
- **THEN** it is refused, naming `forge` and saying the hosted control plane holds the App

#### Scenario: The App beside crew and a hosted control plane

- **WHEN** the configuration names `[forge]`, the account `own` says `control_plane = "crew"` and the account `cloud` says `control_plane = "hosted"`
- **THEN** it reads, holding `[forge]`

#### Scenario: The App in the local layer

- **WHEN** the local layer says `[forge]` with `app_id = 1`
- **THEN** the local layer is not taken, and the runner names `forge`

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

- **WHEN** the managed layer's account `own` says `control_plane = "hosted"`, its `[control_plane.hosted]` names a `url` and the partition `home`, and the local layer names the hosted partition `work` alone
- **THEN** the table handed to the hosted implementation holds the managed layer's `url` and the partition `work` alone

## ADDED Requirements

### Requirement: The control-plane table holds the table of each plane the accounts name

The configuration MAY hold a `[control_plane]` table: `poll_interval` and the table of each implementation an account names, each handed whole to its implementation. It SHALL be refused where it holds `type`, naming `accounts.<account>.control_plane`; where it holds a table no account names, or none; and where it holds any other key. Where no account names a control plane, the runner SHALL make no control pass (record 0054; core.19).

#### Scenario: crew named

- **WHEN** the account `own` says `control_plane = "crew"`, and the configuration holds `[control_plane]` with a `[control_plane.crew]` table
- **THEN** the runner opens crew's implementation with that table

#### Scenario: A type of before

- **WHEN** the configuration holds `[control_plane]` with `type = "crew"` and a `[control_plane.crew]` table
- **THEN** it is refused, naming `type` and saying each account names its control plane by `accounts.<account>.control_plane`

#### Scenario: A table no account names

- **WHEN** the account `own` says `control_plane = "crew"`, and `[control_plane]` holds `[control_plane.crew]` and `[control_plane.hosted]`
- **THEN** it is refused, naming `[control_plane.hosted]` as a table no account names

#### Scenario: No control plane

- **WHEN** the configuration holds no `[control_plane]` and names no `fetch_interval`
- **THEN** `boomdrive run --once` keeps the repositories, makes no control pass, and runs no `flume` command but the settle that ends its pass, `flume --json settle --every 120s`

### Requirement: Each account names its control plane

An account MAY hold `control_plane`, the word of the implementation that serves it, `crew` or `hosted`. It SHALL be refused, naming `accounts.<account>.control_plane`, where it is not a string, where it names an implementation this binary does not carry, naming those it carries, or where `[control_plane]` holds no table of it. An account that names none SHALL be served by no control plane. The local layer SHALL not set it (record 0054; core.17, core.19).

#### Scenario: crew and a cloud side by side

- **WHEN** the account `own` says `control_plane = "crew"`, the account `cloud` says `control_plane = "hosted"`, and `[control_plane]` holds `[control_plane.crew]` and `[control_plane.hosted]`
- **THEN** it reads, `own` served by crew and `cloud` by the hosted control plane

#### Scenario: A plane with no table

- **WHEN** the account `own` says `control_plane = "hosted"`, and `[control_plane]` holds `[control_plane.crew]` alone
- **THEN** it is refused, naming `accounts.own.control_plane` and `[control_plane.hosted]`

#### Scenario: A plane this binary does not carry

- **WHEN** the account `own` says `control_plane = "cloud"`
- **THEN** it is refused, naming `accounts.own.control_plane` and the words `crew` and `hosted`

#### Scenario: A plane in the local layer

- **WHEN** the local layer says `control_plane = "hosted"` in the account `own`
- **THEN** the local layer is not taken, and the runner names `accounts.own.control_plane`

## REMOVED Requirements

### Requirement: The configuration may name the control plane

**Reason**: `[control_plane]`'s `type` named the one plane a runner served; each account names its own (record 0054).
**Migration**: Remove `type`; write `control_plane` in each account its plane serves.

### Requirement: The managed layer may name an observer

**Reason**: Record 0048 is superseded by record 0054; the runner reports to no observer.
**Migration**: Remove `[observer]`; name the local copy of the cloud as an account's hosted control plane.
