# Proposal

## Why

The set brought timber on 2026-11-13 with an option of the Boomdrive module's own, `services.boomdrive.timber.package`, since timber's flake gave no module. Record 0023's amendment of that day says what follows: "timber gives no nix-darwin or NixOS module; once it gives one, the set re-exports it as it does Flume's, and the option goes."

timber's flake now gives one. Its record 0035, accepted on 2026-11-14, gives `services.timber` as `darwinModules.timber` and `nixosModules.timber`:

- `enable`, `package` (timber's flake's own for the host's system by default, `null` installing none), `settings` (the host's bindings file, `~/.config/timber/config.toml`, under the file's own names) and, on NixOS, `user`;
- with `enable`, the package in the system profile and the file linked for the user who runs timber, a file of the host's own there moved once to `config.toml.before-nix`;
- no `timber serve`, no service and no other tool's file;
- the key `github:boomdrivedev/timber#<platform>Modules.timber`, so a system that imports it directly and through Boomdrive's declares `services.timber` once.

timber's clause 118, as amended at `boomdrivedev/blueprints` 573ec1a, asks: "Boomdrive's set takes that module from timber's flake and writes no file of timber's." timber's record 0035 names the rest of it Boomdrive's: "Boomdrive's set taking the module, setting `services.timber.package` and `user`, and dropping `services.boomdrive.timber.package` wait on a release that carries it, and are agentplot/boomdrive's."

The set pins timber `v0.7.0` since the chore `the-set-pins-flume-v0-24-0-and-timber-v0-7-0`, and that tag carries the module. Today the set still installs timber by its own option, and a host that wants timber's bindings written imports timber's module beside the set's by hand. This change is the unit `the-set-takes-timbers-own-module` of the bolt `a-hosts-set-takes-its-tools-new-releases`. Its brief:

> Boomdrive's flake re-exports timber's own module as it does Flume's and sluice's, and services.boomdrive.timber.package goes.

## What Changes

- **Re-exported and imported.** `darwinModules.timber` and `nixosModules.timber` are timber's flake's own modules, beside `boomdrive`, `crew`, `default`, `flume` and `sluice`. The platform's `boomdrive` module imports timber's after crew's, so `services.timber` is timber's input's alone.
- **Turned on with the runner.** With `services.boomdrive.enable`, `services.timber.enable` is true by default, whether or not crew is on, a host's `false` winning. Without the runner, timber's module stays off.
- **Its package and user.** `services.timber.package` is the set's `timber` by default, whether or not the runner is enabled, a host's own or `null` winning. On NixOS, `services.timber.user` is the runner's user by default.
- **Its file is its module's.** With timber's module on, each switch links `~/.config/timber/config.toml` to the file `services.timber.settings` renders, the header alone where the host sets nothing, which binds as no file does (timber's 118). The Boomdrive module writes none of it.
- **BREAKING: the option goes.** `services.boomdrive.timber.package` is removed. Setting it fails evaluation, naming `services.timber.package` and `services.timber.enable`.
- **What stays.** timber's herdr plugin stays in `services.boomdrive.herdr.plugins`' default, and its Claude Code plugin in the set's marketplace. No module of the set's starts `timber serve`.
- **Words.** The README's install with nix names timber's module among the tools' own, its file, `services.timber.package` and `services.timber.enable`. No check reads them (blueprints principle 16).
- **Records and rules.** Record 0023 gains an amendment of 2026-11-15 holding the brief's ask and this design's choices. core.18's and core.46's details name timber's module. Both are written in the commit that writes this change.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `service-modules`: the flake re-exports timber's own module and the Boomdrive module imports it, turns it on with the runner, gives it the set's package and, on NixOS, the runner's user. timber's module installs timber and links its file. The Boomdrive module's own `services.boomdrive.timber.package` is gone.

## Impact

- `flake.nix`: timber among the tools whose modules are imported and re-exported. The lock does not change.
- `nix/modules/tools.nix`: timber's package and its `enable` with the runner.
- `nix/modules/nixos.nix`: timber's user.
- `nix/modules/apps.nix`: `timber.package` removed from its options and its installs, and its header.
- `nix/modules/options.nix`: the removed option's failure.
- `nix/tests/`: `lib.nix`, `tools.nix`, `apps.nix`, `darwin.nix`, `nixos.nix` and `cache.nix`.
- `README.md`.
- `docs/adr/0023-…md` and `docs/architecture/core.md`, in the commit that writes this change.
- **What does not change.**
  - The runner's code does not change and runs no `timber`.
  - No crate, gate tool or tracked file type is added (core.7, core.8).
  - The set's packages, the lock, the cache's workflow, the boot's check and the shell do not change.
  - CLAUDE.md and `openspec/config.yaml` already say each tool's own module is re-exported and the modules install timber with the runner.
- **On a host.**
  - A host that set `services.boomdrive.timber.package` fails evaluation until it moves the value to `services.timber.package`, or sets `services.timber.enable = false`.
  - A host that imported timber's module beside the set's evaluates as before, the module taken once by its key.
  - A host with a `~/.config/timber/config.toml` of its own finds it moved once to `config.toml.before-nix` at its next switch, unless it writes its bindings in `services.timber.settings` or turns timber's module off (design Risks).
- **Outside this repository.** afterthought/swancloud takes the change with its next move of the Boomdrive input, and drops any import of timber's module of its own, which still evaluates.

## Touches

```
flake.nix
nix/**
README.md
docs/adr/0023-the-boomdrive-flake-configures-the-whole-host-each-tool-by-its-own-module-and-each-repository-named-once-by-its-owner.md
docs/architecture/core.md
```
