## RENAMED Requirements

- FROM: `### Requirement: The README writes no crew table beside crew's partitions`
- TO: `### Requirement: The README writes no crew table beside crew's drives`

## REMOVED Requirements

### Requirement: Under crew, the runner's crew table is written from crew's partitions on this host

**Reason**: crew's teams file at version 6, which the set's crew reads, names its drives, keyed by label, and no partitions (crew record 0054; record 0032). "Under crew, the runner's crew table is written from crew's drives on this host" holds what stands of it.

**Migration**: A host writes `services.crew.settings.drives`, each by its label, in place of `partitions`. The runner's table the modules write from them is the same.

### Requirement: A host under crew that the runner cannot serve fails evaluation

**Reason**: Its words named crew's partitions. "Under crew, a host the runner cannot serve fails evaluation" holds it, naming crew's drives.

**Migration**: None: the failures are the same, their words naming crew's drives.

## MODIFIED Requirements

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

### Requirement: A drive of another machine gives Flume nothing

A drive's `machine` SHALL name the host its work runs on, as crew's drives name a host. A drive whose `machine` is unset or is `services.crew.host` is a drive of this host. A drive whose `machine` names another host SHALL give Flume no stream and no repository's table, and its repositories SHALL still be written in the runner's file.

#### Scenario: A drive of another machine

- **WHEN** `services.crew.host` is `host-a`, and the drive `garden` of the account `work` sets `machine = "host-b"` and keeps the owner `example`'s `alpha`
- **THEN** Flume's `settings` names no stream `garden` and no `example/alpha`, and the runner's file's `[accounts.work.drives.garden]` holds `repositories = ["example/alpha"]`

#### Scenario: A drive of this machine

- **WHEN** `services.crew.host` is `host-a`, and the drive `garden` sets `machine = "host-a"` and keeps the owner `example`'s `alpha`
- **THEN** Flume's `settings.streams` names `garden`, and `settings.repos."example/alpha".stream` is `garden`

## ADDED Requirements

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
