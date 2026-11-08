# Proposal

## Why

Record 0023, accepted by the user on 2026-11-06, has a host add the Boomdrive flake alone and get the runner, Flume, sluice and crew, "each packaged and configured", each by its own module. Flume, sluice and crew each publish a flake with their own module, so each can be used without Boomdrive, and the Boomdrive flake takes each as a locked input and re-exports its module. `services.flume.settings` is Flume's file, `services.sluice.settings` sluice's host settings and `services.crew.settings` crew's, each key named as its file names it. No module writes another tool's file, and the Boomdrive module sets the other tools' module options. In the user's words: "I want to make sure that it's a one-stop shop with the Boom Drive flake"; "I want to make sure the flake also packages and configures crew and sluice"; and, for Flume's module, "Flume's flake, re-exported".

Today the flake's modules write the runner's file alone. They install the set's Flume and sluice through `services.boomdrive.tools`, and nothing writes either tool's file. The set builds Flume by a recipe of its own and holds no crew. Each tool's own module is being built in its own repository now:

- Flume's: flume record 0055, the change `flume-publishes-its-own-flake-and-module`.
- sluice's: sluice record 0098, the change `sluice-publishes-a-module-for-its-host-settings`.
- crew's: crew record 0058, the change `crew-publishes-a-flake-whose-module-writes-its-host-file`.

Each of those designs has the Boomdrive flake re-export its module. This change does that. It is the first unit of the bolt `the-boomdrive-flake-configures-the-whole-host`. The bolt's units that name each repository once under its drive, bring herdr and Ghostty, and write the runner's crew table from crew's partitions set further options of these modules, and come after it.

## What Changes

- **Each tool's flake at its release tag.**
  - Flume's input becomes Flume's flake, at its newest release tag whose flake gives its modules.
  - sluice's input moves to the same kind of tag of sluice's.
  - crew's flake joins the inputs at the same kind of tag of crew's.
  - Each follows the flake's `nixpkgs`, `rust-overlay` and `nix-darwin` where it takes them, so its package is made with the flake's own nixpkgs and is a path the family's cache holds.
- **The set gains crew and takes Flume from Flume's flake.**
  - `packages.<system>.flume` is Flume's flake's own package, and this flake's recipe for Flume and its git crates' hashes go.
  - `packages.<system>.crew` is crew's flake's package, given its tag's version, since crew's tree carries none.
  - sluice's package and the two herdr plugins are made as before.
- **The modules bring each tool's own module.**
  - `darwinModules.boomdrive` and `nixosModules.boomdrive`, each also `default`, import the runner's module with Flume's, sluice's and crew's, each taken from that tool's flake.
  - The flake re-exports each of those three as `darwinModules.<tool>` and `nixosModules.<tool>`.
  - No module of this tree declares an option of another tool's.
- **The Boomdrive module sets the tools' options and writes no file of theirs.**
  - Each tool's `package` defaults to the set's, and a host gives its own through that tool's module.
  - With `services.boomdrive.enable`, Flume's and sluice's modules are turned on, and on NixOS each tool's `user` defaults to the runner's.
  - crew's module is turned on by the host, which names its host and partitions.
  - The activation of each tool's file is that tool's module's own.
- **`services.boomdrive.tools` goes.** Each tool's module installs its package, and setting `tools` fails evaluation, naming the options that replace it.
- **Words.**
  - The README's install with nix names each tool's module and its options.
  - CLAUDE.md and `openspec/config.yaml` name crew in the set.
  - core.46's sentence, as record 0023 left it, reads whole, and its details say each tool is installed by its own module.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `service-modules`: the modules bring Flume's, sluice's and crew's own modules, set their options and write none of their files; each tool's package is the set's unless the host gives its own; `services.boomdrive.tools` goes.
- `family-packages`: the set gains crew; Flume's package is Flume's flake's; every tool of the set but the runner comes through its own flake at a release tag.
- `binary-cache`: crew's package is built and pushed with the rest, and the modules install the flake's own crew.

## Impact

- `flake.nix` and `flake.lock`: Flume as a flake, sluice moved, crew added, each at a release tag giving its modules; the outputs' packages and modules.
- `nix/packages/`: `flume.nix` and `flume-git-crates.nix` removed; `crew.nix` added.
- `nix/modules/`: `tools.nix` added, holding what the Boomdrive module sets of the other tools' options; `options.nix`, `darwin.nix` and `nixos.nix` lose `tools` and import it.
- `nix/tests/`: the cases of the three specs' deltas, in `tools.nix` added and in `packages.nix`, `typecheck.nix`, `tags.nix`, `darwin.nix`, `nixos.nix`, `cache.nix`, `lib.nix` and `settings.nix`; `git-crates.nix` keeps the runner's case alone.
- `README.md`, `CLAUDE.md`, `openspec/config.yaml`, and `docs/architecture/core.md` (core.46's sentence and details).
- The runner's code does not change. No crate, tool or tracked file type is added (core.7, core.8). `devenv.nix` is unchanged: it gives the runner's tests the set's Flume and sluice from the same outputs.
- Outside this repository:
  - This change is built once Flume, sluice and crew each have a release tag whose flake gives its modules (design, Dependencies).
  - sluice's and crew's modules are asked to carry a `key`, as Flume's does, so a host may import a tool's module beside Boomdrive's (design, What this change asks of others).
  - afterthought/swancloud passes deneb's facts as these options once it takes the flake (record 0023).

## Touches

```
flake.nix
flake.lock
nix/packages/**
nix/modules/**
nix/tests/**
README.md
CLAUDE.md
openspec/config.yaml
docs/architecture/core.md
```
