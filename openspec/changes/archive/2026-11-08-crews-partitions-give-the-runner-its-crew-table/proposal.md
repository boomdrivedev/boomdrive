# Proposal

## Why

Record 0023, accepted by the user on 2026-11-06, writes crew's partitions once, in crew's module. `services.crew.settings.partitions` lists every partition with its `machine`, and `services.crew.host` names this host to crew. "Where the control plane is crew and no `[control_plane.crew]` is written, the Boomdrive module writes it: the host from `services.crew.host`, and each partition whose `machine` is this host. A table written out wins." The user settled it in the talk of 2026-11-06: "Yes, derive it". Rule core.18's details already allow it.

Today a host under crew writes its partitions twice. It writes them once in `services.crew.settings.partitions` for crew's teams file. It writes them again, with the host's name, in `services.boomdrive.settings.control_plane.crew` for the runner. The README's install with nix shows both. Nothing holds the two the same.

This change is the unit `crews-partitions-give-the-runner-its-crew-table` of the bolt `the-boomdrive-flake-configures-the-whole-host`. It comes after `the-flake-configures-each-tool-by-its-own-module`, which brought crew's own module into the Boomdrive module.

## What Changes

- **The runner's crew table is written from crew's partitions.**
  - Where `services.boomdrive.settings.control_plane.type` is `crew`, crew's module is turned on, and the host writes no `control_plane.crew`, the module writes the table itself.
  - Its `host` is `services.crew.host`.
  - It holds one partition for each of `services.crew.settings.partitions` whose `machine` is that host, in crew's order. Each is that partition's `label`, with its `state` as `state_repository`, and nothing else of it.
  - A partition whose `machine` is another host is left out.
- **A table written out wins.** A `control_plane.crew` the host writes is the file's table, whole. None of crew's partitions is merged into it.
- **Nothing is written beside another control plane.** Under a hosted control plane, or with no `control_plane`, the module writes no crew table, whatever crew's module says.
- **Two hosts that cannot be served fail evaluation.**
  - Under crew, with no table written and crew's module off, evaluation fails. It names `settings.control_plane.crew` and `services.crew`.
  - Under crew, where the module would write the table but no partition's `machine` is `services.crew.host`, evaluation fails. It names `services.crew.host`, its value and the machines the partitions name.
  - Today the runner refuses both files at start.
- **Words.**
  - The README's install with nix writes `control_plane.type = "crew"` and no table. It says how the module writes the table, and that a table written out wins.
  - Record 0023 gains an amendment of 2026-11-08 naming this change's choices, each with the path not taken.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `service-modules`: under crew, the runner's `[control_plane.crew]` is written from crew's partitions whose `machine` is this host, unless the host writes one. A host under crew that neither writes the table nor turns on crew's module fails evaluation, as does one where no partition names its host.

## Impact

- `nix/modules/options.nix`: `control_plane.crew`'s default is the table written from crew's partitions where the control plane is crew and crew's module is on, and two assertions are added. `darwin.nix` and `nixos.nix` render `settings` as now.
- `nix/tests/`: the delta's cases, in `darwin.nix`, `nixos.nix` and `tools.nix`, with `settings.nix` gaining a fleet of two machines' partitions and crew's settings with no table. `cache.nix` takes the README's section from `lib.nix`, which `tools.nix` reads too.
- `README.md`: the install with nix.
- `docs/adr/0023-…md`: the amendment, committed with this change's planning.
- The runner's code does not change, and neither do its file's keys or any file under `nix/tests/files`. crew's table is the one crew's implementation already reads (core.2). No crate, tool or tracked file type is added (core.7, core.8). No rule changes: core.18's details already name this.
- Outside this repository: afterthought/swancloud can drop its hosts' `control_plane.crew` once it takes the flake, since a host's partitions in crew's settings give it (record 0023).

## Touches

```
nix/modules/options.nix
nix/tests/**
README.md
docs/adr/0023-the-boomdrive-flake-configures-the-whole-host-each-tool-by-its-own-module-and-each-repository-named-once-by-its-owner.md
openspec/changes/crews-partitions-give-the-runner-its-crew-table/**
openspec/specs/service-modules/spec.md
```
