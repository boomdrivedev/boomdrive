# Proposal

## Why

A computer that runs no runner should still be able to install Boomdrive's tools and their herdr plugins from the Boomdrive flake, already built, turning on only the ones it wants. This change is the unit of the bolt `every-machine-takes-boomdrives-tools`. Its brief:

> A machine that runs no runner can still take Boomdrive's tools and their plugins from the Boomdrive flake, already built, turning on only the tools it wants. Sources: boomdrivedev/blueprints design/principles.md 4 and 17.

As the modules read at `2ea1a481` on 2026-11-18, Flume, sluice, crew and timber already come without the runner by their own modules' `enable`. herdr, Ghostty, the launcher, portless, worktrunk, herdr's plugins and Flume's terminal words come only with the runner, and crew's companions only with the runner and crew (design Context).

## What Changes

- **A switch for each tool.** herdr, Ghostty, the launcher, portless and worktrunk each gain `services.boomdrive.<tool>.enable`. Pending You, Plannotator and plannotator-tui gain `services.boomdrive.pendingyou.enable`, `services.boomdrive.plannotator.enable` and `services.boomdrive.plannotator-tui.enable`. Flume, sluice, crew and timber keep their own modules' `enable`. Where a switch is on, with the runner or without, its `package` is installed: the set's by default, a host's own as given, and none for `null`.
- **The runner's switch keeps turning them on.** Each new switch is on by default where `services.boomdrive.enable` is on, and the companions' where crew's module is on too. A host with the runner gets what it got before. Without the runner, each is off until the host turns it on.
- **herdr's plugins follow the tools the set installs.** `services.boomdrive.herdr.plugins` holds by default the set's plugin of each tool the set installs, each only where that tool is on with the set's own package. They are linked wherever herdr is on, with the runner or without.
- **The set's user, with the runner or without.** The plugins' links and Pending You's folder run as the primary user on nix-darwin and as `services.boomdrive.user` on NixOS. Without the runner, nothing else runs as that user: no runner's file, agent, unit or linger. A step with no user named fails evaluation, naming the option. On NixOS, the four tools' `user` is `services.boomdrive.user` by default wherever the host names it.
- **Flume's terminal words follow herdr and Ghostty.** Flume's `host.multiplexer` is `herdr` by default where herdr is on, and its `host.terminal_app` is `ghostty` where Ghostty is on.
- **The launcher holds no drive without the runner.** It is the set's own, with `Default stream` alone.
- **Words.** The README's install with nix, `CLAUDE.md`, `openspec/config.yaml` and the modules' comments say each tool has a switch of its own, which the runner's turns on, and how a host without the runner takes the ones it wants.
- **Records and rules.** Record 0023 gains an amendment of 2026-11-18, and record 0044 one for point 8. core.63 is written, and core.18's and core.46's details are amended. All are in the commit that writes this change.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `service-modules`: each tool of the set is turned on by a switch of its own, which the runner's turns on by default, and a host without the runner installs the ones it turns on, with their herdr plugins, as the set's user.

## Impact

- `nix/modules/apps.nix`: the five switches, the plugins' default and Flume's terminal words.
- `nix/modules/companions.nix`: the three switches.
- `nix/modules/darwin.nix` and `nix/modules/nixos.nix`: the plugins' and Pending You's folder's steps, with the runner or without, as the set's user. On NixOS, also the tools' users.
- `nix/modules/options.nix`, `nix/modules/tools.nix` and `flake.nix`: comments.
- `nix/tests/`: `apps.nix`, `tools.nix`, `darwin.nix`, `nixos.nix` and `lib.nix`.
- `README.md`, `CLAUDE.md` and `openspec/config.yaml`.
- `docs/adr/0023-…md`, `docs/adr/0044-…md` and `docs/architecture/core.md`, in the commit that writes this change.
- **What does not change.**
  - The runner's code, the packages, the lock, the cache's workflow, the boot's check and the shell.
  - No crate, gate tool or tracked file type is added (core.7, core.8).
  - Claude Code's two managed documents and the tools' telemetry keep their conditions.
- **On a host.**
  - A host with the runner that names none of the new switches, and keeps every tool's package the set's, evaluates to the same system.
  - A host with the runner that gives Flume, sluice or timber a package of its own, or turns that tool's module off, and names no `services.boomdrive.herdr.plugins`, no longer links the set's plugin of that tool. It lists its own build's plugin there (design Decision 3).
  - A host without the runner that turns on nothing evaluates to the same system.
- **Outside this repository.** afterthought/swancloud's hosts not flagged `runner` import the module and turn nothing on, so nothing changes for them until swancloud turns on the tools it wants there.

## Touches

```
flake.nix
nix/**
README.md
CLAUDE.md
openspec/config.yaml
docs/adr/0023-the-boomdrive-flake-configures-the-whole-host-each-tool-by-its-own-module-and-each-repository-named-once-by-its-owner.md
docs/adr/0044-crews-companions-come-with-the-flake-and-claude-code-loads-them-from-the-sets-own-marketplace.md
docs/architecture/core.md
```
