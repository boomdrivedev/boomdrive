# Tasks

Work through the tasks in order. A task is true once the check it names passes, run as it names it. The groups are headings for reading.

- Every case evaluates against no store, as `nix/check.sh` does, with core.41's neutral names. No task builds a package or writes to the store. Only 1.1's lock reaches the network, in the devenv shell, outside the sandbox.
- No task moves an input other than crew's, or adds a crate, a tool or a tracked file type (core.7, core.8). The runner's code and its file do not change (core.5). Nothing outside the paths proposal.md's Touches lists is changed.

## 1. The set's crew, by crew's names

- [ ] 1.1 `flake.nix` names crew `v0.59.0`, and the lock moves with it alone. `flake.nix` takes `inputs.crew.packages.<system>.crew` unchanged, and `nix/packages/crew.nix` is removed (design Decisions 1 and 2; Task notes 1.1). True when `sluice exec main -- moon run nix:typecheck` passes, proving "Every system's packages" and "Each tag is the lock's".
- [ ] 1.2 `options.nix` writes the runner's crew table from crew's `drives` whose `machine` is `services.crew.host`, in label order, and its failures, descriptions and comments name crew's drives. `settings.nix`'s fixtures give crew's settings by crew's names, with `hosts`. The cases on crew's partitions take crew's drives, and `packages.nix`'s crew case holds the set's crew to crew's flake's own derivation (design Decisions 2, 3, 5 and 7; Task notes 1.2). True when `sluice exec main -- moon run nix:test` passes, proving:
  - "crew through its own flake";
  - "crew's drives on this host" and "A table written out wins", each on nix-darwin and on NixOS;
  - "A hosted control plane beside crew" and "No control plane beside crew", each on nix-darwin and on NixOS;
  - "crew's drives in label order", "Under crew with neither" and "No drive of crew's on this host", each on nix-darwin and on NixOS;
  - "The set's packages" and "A host's own crew", each on nix-darwin and on NixOS;
  - "The tools' user on NixOS", and "The tools' activation is their own" on nix-darwin and on NixOS.
- [ ] 1.3 `tools.nix` holds a host on the set's crew to crew's teams file at version 6, holds `hosts` and the drive's `runs` and `review` to what the host wrote, and holds crew's old names to fail evaluation (design Decision 6; Task notes 1.3). True when `sluice exec main -- moon run nix:test` passes, proving "A host on the set's crew", "crew's keys stay the host's", "crew's old way of running" and "crew's old batch names", each on nix-darwin and on NixOS.
- [ ] 1.4 The README's **Installing with nix** writes `services.crew.settings` by crew's names. Its words name crew's drives and hosts where they named crew's partitions, and say the module writes none of crew's `hosts`, `runs` or `review`. `tools.nix` gains "The README's crew settings" (design Decisions 6 and 8; Task notes 1.4). True when `sluice exec main -- moon run nix:test` passes, proving "The README's crew settings", with "The README's crew table", "The README's words", "The README's accounts" and "The README's lines" still holding; and `sluice exec main -- moon run root:format` passes.
- [ ] 1.5 The nix files this change adds or changes keep to the nix project's format and lint (Task notes 1.5). True when `sluice exec main -- moon run nix:format` and `sluice exec main -- moon run nix:lint` pass.

## 2. The specs

- [ ] 2.1 The change's deltas archive into specs that pass openspec's strict check. True when, after `devenv shell -- openspec archive the-sets-crew-pin-takes-its-new-names --yes`, `sluice exec main -- moon run root:specs` passes.
