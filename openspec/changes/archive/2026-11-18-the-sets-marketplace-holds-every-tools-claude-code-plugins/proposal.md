# Proposal

## Why

The set's Claude Code marketplace, `boomdrive-claude-plugins`, holds only Pending You's, Plannotator's and timber's plugins. Record 0044 as amended on 2026-11-18 has it hold every plugin a tool of the set ships, from the same pin as the tool and at the set's version of it, so a host that takes its tools through the set takes their Claude Code plugins from the set too ("The whole point of the boomdrive flake is that everything gets cached.", the user, 2026-11-18). Flume ships `flume`, sluice `sluice` and `sluice-board`, crew `crew` and worktrunk `worktrunk`, and the marketplace lists none of them.

## What Changes

- **Five more plugins.** The marketplace lists `flume`, `sluice`, `sluice-board`, `crew` and `worktrunk` beside `pendingyou`, `plannotator` and `timber`, each at `./plugins/<name>` and at the set's version of its tool.
- **Each from its tool's pin.** Flume's `plugins/claude` and sluice's `plugins/sluice` and `plugins/sluice-board` are copied whole from the tree the set builds each tool from. crew's is the `plugin` folder of the set's crew package, which crew's own flake builds. worktrunk's is the `plugins/worktrunk` of the tree at worktrunk's tag, since its package's source holds only what its build compiles.
- **Each held to its tool's version.** The flake's test and the marketplace's build fail where a plugin's manifest says another version than its tool's, naming the plugin and both versions. worktrunk's manifest names none, so Claude Code reads the marketplace's entry, the set's worktrunk's version.
- **Record 0044's amendment of 2026-11-18** names crew's and worktrunk's plugins and how each is taken, and core.44's details name the marketplace's plugins of the set's tools.
- **The README** names the marketplace's plugins.

No module installs the marketplace yet: that is record 0044's point 3, the change `every-claude-code-session-has-boomdrives-plugins`.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `family-packages`: the set's marketplace lists every Claude Code plugin a tool of the set ships, each of its tool's pin and held to its tool's version.

## Impact

- `nix/packages/claude-plugins.nix`: the copied plugins in one list, each copied and checked alike.
- `flake.nix`: the marketplace given Flume, sluice, crew, worktrunk and worktrunk's input.
- `nix/tests/companions.nix`: "The marketplace's plugins" lists eight. `nix/tests/plugins.nix`, wired in `nix/tests/default.nix`, holds the tools' plugins' cases.
- `README.md`, `CLAUDE.md` and `openspec/config.yaml` name the marketplace's plugins.
- `docs/adr/0044-…`: its amendment of 2026-11-18 names crew's and worktrunk's plugins. `docs/architecture/core.md`: core.44's details.
- No input moves, and no crate, tool or tracked file type is added (core.7, core.8, core.45). The runner's code does not change.

## Touches

```
flake.nix
nix/packages/claude-plugins.nix
nix/tests/companions.nix
nix/tests/default.nix
nix/tests/plugins.nix
README.md
CLAUDE.md
openspec/config.yaml
docs/adr/0044-crews-companions-come-with-the-flake-and-claude-code-loads-them-from-the-sets-own-marketplace.md
docs/architecture/core.md
```
