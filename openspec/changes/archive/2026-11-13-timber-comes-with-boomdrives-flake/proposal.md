# Proposal

## Why

timber delivers what the merge queue lands: it publishes a landed revision, promotes it with proof, and serves the host's registries at one fixed address. Its v0.6.0 is released, and no host has it. The user asked on 2026-11-12, as the first step of Timber's MVP, that the hosts get it. The planner queued it as this unit (afterthought/crew-state's `plan.rec`, agentplot/main `f577b1e0`):

> Boomdrive's flake carries timber, its CLI and its herdr and Claude Code plugins from one pin of timber's flake, as it carries sluice and flume, so every host the set configures has timber on its path.

Record 0020 made this flake the way the family's host tools reach a host, and asked timber to carry each release's version in its tagged tree. timber's record 0031 did so, citing this flake's set. Today the set holds no timber:

- a host that takes the flake has no `timber` on its `PATH`;
- timber's herdr pane, which shows every repository the host knows, is linked nowhere;
- timber's Claude Code plugin, which keeps a session in a timber repository publishing only through `timber publish`, is in no marketplace a host has.

The pieces are ready:

- **timber's flake at `v0.6.0`.** It gives `packages.<system>.default` for `aarch64-darwin`, `aarch64-linux` and `x86_64-linux`, among others. Its package is `timber` `0.6.0`, its program `timber`. Its tagged tree carries its version, and the binary says it beside the revision it was built from.
- **Its plugins in the same tree.** `plugins/herdr` is its herdr plugin, `agentplot.timber` `0.6.0`, asking herdr `0.9.3` or later. `plugins/claude-code` is its Claude Code plugin, `timber` `0.6.0`.
- **The checks can read it.** With the flake's `nixpkgs` and `rust-overlay` followed, the package evaluates on each of the three systems. Its `Cargo.lock` holds no git source, and the locked rust-overlay carries the Rust `1.98.1` its `.prototools` names. Seen on 2026-11-13.

## What Changes

- **timber in the set.** timber's flake joins the inputs at `v0.6.0`, following the flake's `nixpkgs` and `rust-overlay`.
  - `packages.<system>.timber` is its flake's `default`, unchanged.
  - `packages.<system>.timber-herdr-plugin` is its source's `plugins/herdr`, made as Flume's and sluice's are.
  - `tags.nix` holds timber to a release tag, as it holds Flume, sluice, crew, herdr, portless and worktrunk.
  - The test holds both plugins' manifests to the set's timber's version.
- **The set's marketplace.** `boomdrive-claude-plugins` lists a third plugin, `timber`: its source's `plugins/claude-code`, copied whole. The change installs it into no Claude Code home. That install is the unit `the-companions-load-in-claude-codes-sessions`, which installs the set's marketplace into each home (record 0044, point 3).
- **The modules.** With `services.boomdrive.enable`:
  - they install `services.boomdrive.timber.package`, the set's by default, a host's own replacing it and `null` installing none;
  - `services.boomdrive.herdr.plugins`' default gains `timber-herdr-plugin`, after sluice's, whether or not crew is on.

  Without the runner they install none. They start no `timber serve` and write no file of timber's.
- **The shell.** `devShells.<system>.default` holds the set's timber.
- **The cache.** Its workflow builds and pushes `timber` and `timber-herdr-plugin` with the rest, with no change of its own.
- **Words.**
  - The README's install with nix names timber, its two plugins, its option, the shell's `timber`, and that timber's serving process is timber's own.
  - CLAUDE.md and `openspec/config.yaml` name timber in the set.
- **Records and rules**, amended in the commit that writes this change (design, Rules amended):
  - record 0023 gains an amendment of 2026-11-13 holding the brief's ask and this design's choices;
  - record 0044 gains one naming timber's plugin in the set's marketplace;
  - core.13's, core.44's and core.46's details name timber.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `family-packages`: the set gains `timber` and `timber-herdr-plugin`, through timber's own flake at a release tag, and the set's marketplace lists timber's Claude Code plugin.
- `service-modules`: the modules install timber with the runner, replaceable or dropped, and link its herdr plugin; the README names its option.
- `binary-cache`: timber and its herdr plugin are built and pushed with the rest, and the modules install the flake's own.
- `way-in`: the flake's shell holds timber.

## Impact

- `flake.nix` and `flake.lock`: timber's input, which adds the one node `timber` to the lock, and the packages `timber` and `timber-herdr-plugin`.
- `nix/packages/claude-plugins.nix`: the plugin `timber`.
- `nix/modules/apps.nix`: the option `services.boomdrive.timber.package` and the plugins' default.
- `nix/shell.nix`: the set's timber.
- `nix/tests/`:
  - `timber.nix`, added;
  - `default.nix`, `typecheck.nix`, `tags.nix`, `packages.nix`, `companions.nix`, `apps.nix`, `nixos.nix`, `cache.nix`, `workflow.nix` and `way-in.nix`, changed.
- `README.md`, `CLAUDE.md` and `openspec/config.yaml`.
- `docs/adr/0023-…md`, `docs/adr/0044-…md` and `docs/architecture/core.md`, in the commit that writes this change.
- **What does not change.**
  - The runner's code does not change and runs no `timber`.
  - No crate, gate tool or tracked file type is added (core.7, core.8).
  - The checks evaluate timber and never build or run it.
  - `.github/workflows/cache.yml` builds every package the flake gives, so it does not change.
  - The quick start and `herdr-plugins` do not change (design Decision 9).
- **Outside this repository.**
  - The unit `the-companions-load-in-claude-codes-sessions` finds `timber` among the marketplace's plugins (design Decision 4).
  - afterthought/swancloud takes timber with its next move of the Boomdrive input, and drops any timber of its own, or sets the option to its own or to `null`.

## Touches

```
flake.nix
flake.lock
nix/**
README.md
CLAUDE.md
openspec/config.yaml
docs/adr/0023-the-boomdrive-flake-configures-the-whole-host-each-tool-by-its-own-module-and-each-repository-named-once-by-its-owner.md
docs/adr/0044-crews-companions-come-with-the-flake-and-claude-code-loads-them-from-the-sets-own-marketplace.md
docs/architecture/core.md
```
