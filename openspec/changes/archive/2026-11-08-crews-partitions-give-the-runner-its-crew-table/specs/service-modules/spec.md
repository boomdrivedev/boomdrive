# Service modules

## ADDED Requirements

### Requirement: Under crew, the runner's crew table is written from crew's partitions on this host

Where `settings.control_plane.type` is `crew`, crew's module is on and the host writes no `settings.control_plane.crew`, the modules SHALL write it: `host` as `services.crew.host`, and one entry of `partitions` for each of crew's partitions whose `machine` is that host, in crew's order, each its `label` and its `state` as `state_repository` alone. `settings.control_plane.crew` SHALL read that table (record 0023; core.18).

#### Scenario: crew's partitions on this host

- **WHEN** `settings` names `repository_root = "~/Code"`, `fetch_interval = "2m"`, `code_workspaces = true`, the account `own` whose drive `home` keeps `agentplot/flume` and `agentplot/sluice`, and `control_plane.type = "crew"` with no `control_plane.crew`, and crew's module is on with the host `host-a` and the partitions `home`, whose `machine` is `host-a` and `state` `example/state`, and `away`, whose `machine` is `host-b` and `state` `example/away`
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

- **WHEN** `settings` names the account `home`, whose drive `garden` keeps `agentplot/flume`, the account `work` with no drive, and a hosted control plane at `https://control.example` with the partitions `home` and `work`, and crew's module is on with the host `host-a` and the partition `home` on `host-a`
- **THEN** the file is `nix/tests/files/hosted.toml`, byte for byte, on nix-darwin and on NixOS

#### Scenario: No control plane beside crew

- **WHEN** `settings` sets nothing, and crew's module is on with the host `host-a` and the partition `home` on `host-a`
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
