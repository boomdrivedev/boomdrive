# Tasks

Build the tasks in order. A task is true once the check it names passes, run as it names it. The groups are headings, for reading.

- Every case evaluates against no store, as `nix/check.sh` does, with core.41's neutral names. No task builds a package, writes to the store or reaches the network.
- No task adds a crate, a tool or a tracked file type (core.7, core.8). The runner's code and every file under `nix/tests/files` stay as they are (design Decision 7). Nothing outside the paths proposal.md's Touches lists is changed.

## 1. The table written from crew's partitions

- [x] 1.1 `settings.control_plane.crew`'s default is the table written from crew's partitions where the control plane's `type` is `crew` and `services.crew.enable` is true, and `null` otherwise. The table's `host` is `services.crew.host`, and its `partitions` are each of crew's partitions whose `machine` is that host, in crew's order, each `label` and `state_repository` alone. `nix/tests/settings.nix` gains `crewUnwritten`, `crewFleet`, `crewOrder` and `crewElsewhere` (design Decisions 1 to 3 and 8; Task notes 1.1). True when `sluice exec main -- moon run nix:test` passes, proving the service-modules delta's "crew's partitions on this host", on nix-darwin and on NixOS, and "crew's order".
- [x] 1.2 A `settings.control_plane.crew` the host writes is the file's table, whole, with none of crew's partitions merged into it (design Decision 4; Task notes 1.2). True when `sluice exec main -- moon run nix:test` passes, proving the delta's "A table written out wins" on nix-darwin and on NixOS.
- [x] 1.3 Under a hosted control plane, or with none, the file holds no crew table, whatever crew's module holds (design Decision 5; Task notes 1.3). True when `sluice exec main -- moon run nix:test` passes, proving the delta's "A hosted control plane beside crew" and "No control plane beside crew" on nix-darwin and on NixOS.

## 2. Hosts the runner cannot serve

- [x] 2.1 With `services.boomdrive.enable`, evaluation fails where the control plane's `type` is `crew` and `settings.control_plane.crew` is null, the host having written none and crew's module being off. The words name `settings.control_plane.crew` and `services.crew` (design Decision 6; Task notes 2.1). True when `sluice exec main -- moon run nix:test` passes, proving the delta's "Under crew with neither" on nix-darwin and on NixOS.
- [ ] 2.2 With `services.boomdrive.enable`, evaluation fails where the module would write the crew table and no partition's `machine` is `services.crew.host`. The words name `services.crew.host`, its value and each `machine` the partitions name (design Decision 6; Task notes 2.2). True when `sluice exec main -- moon run nix:test` passes, proving the delta's "No partition on this host" on nix-darwin and on NixOS.

## 3. The words

- [ ] 3.1 The README's **Installing with nix** sets the runner's `control_plane.type = "crew"` beside `services.crew` and writes no `control_plane.crew`. Its words say the module writes `[control_plane.crew]` from crew's partitions whose `machine` is `services.crew.host`, and that a table written out wins, whole. `nix/tests/lib.nix` gives the README's section and its nix blocks, which `cache.nix` takes from it (Task notes 3.1). True when `sluice exec main -- moon run nix:test` passes, proving the delta's "The README's crew table" and still the binary-cache spec's "The README's lines" and the service-modules spec's "The README's words".
- [ ] 3.2 The nix files this change touched are formatted and lint clean (Task notes 3.2). True when `sluice exec main -- moon run nix:format` and `sluice exec main -- moon run nix:lint` pass.
- [ ] 3.3 The spec delta archives into specs that pass openspec's strict check (Task notes 3.3). True when, after `devenv shell -- openspec archive crews-partitions-give-the-runner-its-crew-table --yes`, `sluice exec main -- moon run root:specs` passes.
