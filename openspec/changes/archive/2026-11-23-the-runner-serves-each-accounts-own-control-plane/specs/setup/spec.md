## MODIFIED Requirements

### Requirement: The runner's file holds the keys Nix writes

Setup SHALL write the runner's managed layer, `config.toml`, holding `accounts`, each account it set up naming the implementation as its `control_plane`, and `[control_plane]`, holding the implementation's own table: crew's `host`, and a partition for each drive with its `label` and `state_repository`. Past its first line, it SHALL be byte for byte the file Nix's module renders for the same accounts and control plane (record 0009 as amended 2026-11-09).

#### Scenario: One drive under crew

- **WHEN** setup runs with the account `own`, the drive `home`, the state repository `example/state` and the repositories `agentplot/flume` and `agentplot/sluice`, on the host `host-a`
- **THEN** the runner's file, past its first line, is `nix/tests/files/setup.toml` past its first line, byte for byte

#### Scenario: A second drive

- **WHEN** setup, having set up the drive `home` of `own`, runs for the drive `work` of `own` keeping `example/app`
- **THEN** the runner's file keeps the drives `home` and `work`, each with its repositories, and crew's table names both as partitions

