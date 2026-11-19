# Proposal

## Why

`main` brought Rex into the set with the change `the-set-carries-rex-matched-to-flume`, installed with the runner by `services.boomdrive.rex.package`. The bolt `every-machine-takes-boomdrives-tools` gave every other tool of the set a switch of its own with the change `boomdrives-tools-install-on-a-machine-without-the-runner`, so a host without the runner takes the tools it turns on. core.62 rules out a tool of the set that only the runner's switch installs. Merging `main` into the bolt gave Rex its switch in `nix/modules/apps.nix`, as the merge's code needed one. The service-modules spec still says Rex comes with the runner alone. Record 0023, as amended on 2026-11-18 ("Rex has a switch of its own"), decides it.

## What Changes

- **Rex's switch.** `services.boomdrive.rex.enable` is `services.boomdrive.enable` by default, a host's own value winning. Where it is on, with the runner or without, the modules put `services.boomdrive.rex.package` in the system profile. Where it is off, they install no Rex. A host with the runner gets what it got before.
- **Every tool turned on without the runner** includes Rex.
- The modules still name Rex in no setting of Flume's and run no Rex.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `service-modules`: Rex is installed where its own switch is on, with the runner or without, and a host that turns every switch on without the runner gets Rex too.

## Impact

- `nix/modules/apps.nix`: `services.boomdrive.rex.enable`, and Rex in the system profile where it is on. The merge commit holds it.
- `nix/tests/apps.nix`: the case *Rex without the runner*, and Rex in *Every tool turned on without the runner*. The merge commit holds them.
- `docs/adr/0023-…`: its amendment of 2026-11-18, Rex has a switch of its own.
- The runner's code, the set's packages, the lock, the cache's workflow, the boot and the shell do not change. No crate, tool the gate runs or tracked file type is added (core.7, core.8).

## Touches

```
nix/modules/apps.nix
nix/tests/apps.nix
docs/adr/0023-the-boomdrive-flake-configures-the-whole-host-each-tool-by-its-own-module-and-each-repository-named-once-by-its-owner.md
```
