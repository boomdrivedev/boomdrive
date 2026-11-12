# Proposal

## Why

The set pins crew v0.50.2. Its teams file is version 5: a list of `partitions`, each naming `flume` for how its roles run and `review`'s `every` and `hours` for its batch. crew has since renamed them (agentplot/crew docs/adr/0064-crew-reads-a-hosts-drives-from-its-own-file-alone-and-no-fleets-file.md, amendment of 2026-11-09, accepted). Its teams file is now version 6:

- `drives`, keyed by label, in place of `partitions` (crew record 0054).
- `runs`, naming each main-level role `"standing"` or `"on-demand"`, in place of `flume`.
- `review`'s `after_changes` and `after_hours`, in place of `every` and `hours`.
- `hosts`, each host crew reaches, with its `ssh` name and its `kind`, every drive's `machine` among them. crew reads them from its own file alone, and no fleet's file (crew record 0064).

crew v0.59.0 reads that file. It also reports its own version: its tree's `plugin/release.json` gives `0.59.0` at the tag, so its flake's package says so. The set no longer needs `nix/packages/crew.nix`, which gave crew's package its tag's version because crew's tree carried none (core.44).

Record 0032 says crew's rename of "partition" to "drive" moves "in the change that moves crew's version". This is that change. A host on the set then loads crew's teams file at version 6.

## What Changes

- **The pin.** `flake.nix` names crew `v0.59.0`, and the lock moves with it (core.45).
- **crew's package as crew's flake builds it.** `flake.nix` takes `inputs.crew.packages.<system>.crew` unchanged, and `nix/packages/crew.nix` is removed. Its version is `0.59.0`, from crew's own tree.
- **The runner's crew table from crew's drives.** Under crew, with no table written, the module writes `[control_plane.crew]` from crew's `drives` whose `machine` is `services.crew.host`, in the order of their labels, each as its `label` and its `state` as `state_repository`. Its failures and its options' words name crew's drives, never partitions. The runner's own table keeps its key `partitions` (crew-control-plane spec).
- **crew's keys stay the host's.** The module writes no key of crew's `hosts`, `runs` or `review`. Each host crew reaches, with its `ssh` name and `kind`, and how each drive's roles run and batch, is the host's own fact, written in `services.crew.settings` (core.57; record 0036, point 4).
- **The tests write crew's settings by crew's names.** The fixtures give each drive by label with `runs`, `review.after_changes` and `review.after_hours`, beside `hosts` naming `host-a` and `host-b`. New cases hold that a host on the set's crew loads crew's teams file at version 6, and that crew's old names, `flume`, `every` and `hours`, fail evaluation.
- **The README.** Its install with nix writes `services.crew.settings` by crew's names: `drives`, `runs`, `review`'s `after_changes` and `after_hours`, and `hosts` with `ssh` and `kind`. Its words name crew's drives where they named crew's partitions.

**BREAKING**: a host's `services.crew.settings` written by version 5's names fails evaluation at the switch that takes this change. The host writes them by crew's names in that switch, by hand, as record 0032 has it.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `service-modules`: the runner's crew table is written from crew's drives, in label order. The module's failures name crew's drives. A host on the set loads crew's teams file at version 6, by crew's names, and the module writes none of crew's `hosts`, `runs` or `review`. The README writes crew's settings by those names.
- `family-packages`: crew's package is crew's flake's own, unchanged, its version from crew's tagged tree.

## Impact

- `flake.nix`, `flake.lock`: crew at `v0.59.0`, taken unchanged.
- `nix/packages/crew.nix`: removed.
- `nix/modules/`:
  - `options.nix` reads crew's `drives` for the runner's crew table. Its header comment, the `machine` option's description and its two failures' words name crew's drives.
  - `tools.nix`'s header comment names crew's host, drives and hosts.
- `nix/tests/`:
  - `settings.nix`'s crew fixtures move to crew's names.
  - `packages.nix`'s crew case holds the package unchanged.
  - `tools.nix`, `darwin.nix` and `nixos.nix` rename their cases on crew's partitions to crew's drives. `tools.nix` gains the cases of a host on the set's crew, crew's old names, and the README's crew settings.
  - `cache.nix`'s comment names crew's drives.
- `README.md`'s **Installing with nix**.
- The runner's code and its file do not change (core.5). No crate, tool or tracked file type is added (core.7, core.8). No rule changes.
- Outside this repository: a host that takes the set with crew on, afterthought/swancloud's among them, writes its `services.crew.settings` by crew's version-6 names in the switch that takes this change, as crew record 0064's consequences already ask of it.

## Touches

```
flake.nix
flake.lock
nix/packages/crew.nix
nix/modules/options.nix
nix/modules/tools.nix
nix/tests/settings.nix
nix/tests/packages.nix
nix/tests/tools.nix
nix/tests/darwin.nix
nix/tests/nixos.nix
nix/tests/cache.nix
README.md
```
