# Proposal

## Why

crew's roles run in Flume only where the host declares crew as a source of kinds: a file `~/.config/flume/sources/crew.toml` naming the roles file crew writes and crew's kinds folder (crew record 0057, point 1; Flume rule core.45 and record 0040). crew writes the roles file itself, `~/.local/state/crew/flume/roles.toml`, and keeps its kinds folder, `~/.local/share/flume/kinds/crew`. Neither crew nor the runner may write a file in Flume's folder (core.18), so today each host writes that source by hand, or keeps its own writer of it.

Record 0034, accepted on 2026-11-09, point 2, settles who declares it: "Where crew is in the set, the Boomdrive module declares crew's source through Flume's module, naming the roles file crew writes and its kinds folder." Flume's module now writes each source a host declares, from `services.flume.sources` (Flume's change `flumes-module-writes-the-hosts-sources`, landed before v0.21.0). The set's lock names Flume v0.21.2, which has that option, so the set can declare crew's source with no input moved.

This change is the unit `the-set-declares-crews-source-to-flume` that record 0034 names, after Flume's unit.

## What Changes

- **crew's source declared.** Where crew's module is on, whether or not the runner is, the Boomdrive module gives Flume's module `services.flume.sources.crew`: `kinds_dir = "~/.local/share/flume/kinds/crew"` and `roles = "~/.local/state/crew/flume/roles.toml"`, and no other key. Flume's module renders it and links it at `~/.config/flume/sources/crew.toml`. Where crew's module is off, no source of crew's is declared.
- **A host's own value wins.** Each key is given at the options' default priority, as the module's other values for Flume are, so a key a host sets in `services.flume.sources.crew` wins.
- **The paths are crew's.** A test holds the two paths to the ones the set's crew writes, read from crew's own source at the tag the lock names, so a crew pin that moves them fails the gate.
- **Words.**
  - The README's install with nix says the module declares crew's source, and that a host writes no file for it.
  - CLAUDE.md's paragraph on the flake says the same in a clause.
  - core.57 names crew's source among the facts the set gives through a tool's module option, citing record 0034, point 2, in the commit that writes this change.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `service-modules`: with crew's module on, the modules declare crew's source of kinds to Flume's module, naming the roles file and kinds folder the set's crew writes, each key a host's own value overrides; with crew's module off, they declare none; Flume's module writes the source's file, so the Boomdrive module still writes no file of the other tools'.

## Impact

- `nix/modules/`:
  - `sources.nix`, added, declares crew's source through Flume's module.
  - `darwin.nix` and `nixos.nix` import it beside `drives.nix`, and their header comments say so.
- `nix/tests/`:
  - `sources.nix`, added to `default.nix`'s `test`.
  - `lib.nix`'s systems of the tools' own modules declare the same source, so the cases that compare activations compare like with like.
- `README.md` and `CLAUDE.md`.
- `docs/architecture/core.md`, in the commit that writes this change.
- The runner's code and its file do not change (core.5). No flake input or tag moves. No crate, tool or tracked file type is added (core.7, core.8).
- Outside this repository: a host that takes the set with crew on, afterthought/swancloud's among them, drops its own `sources/crew.toml` and any writer of it in the switch that takes this change. Flume's module moves a file of the host's own at that path aside once.

## Touches

```
nix/modules/sources.nix
nix/modules/darwin.nix
nix/modules/nixos.nix
nix/tests/sources.nix
nix/tests/default.nix
nix/tests/lib.nix
README.md
CLAUDE.md
docs/architecture/core.md
```
