# Proposal

## Why

Flume carries a second terminal profile, Rex's (Flume record 0068), which drives `rex` from the `PATH`. Boomdrive's set carries herdr, the first, and no Rex, so a host that takes every tool through the set, as afterthought-design says the user does, has no Rex to try Flume's Rex terminal with unless it installs one itself. Record 0023, as amended on 2026-11-18 at ff6b3a0d, puts Rex in the set beside herdr: from Rex's own flake, built into the family's cache, at the release the pinned Flume's Rex profile is tested against, moved whenever the set moves Flume's pin, as Ghostty is held to Flume's window. A host gets Rex installed; which terminal Flume opens stays its Flume settings.

Flume v0.29.0, the set's Flume, takes Rex 0.1.0-preview.14 from Rex's flake by the URL that release keeps for good, `https://pkgs.superlogical.com/rex/nix/preview/0.1.0-preview.14.tar.gz`, in its `devenv.yaml`, locked in its `devenv.lock`. Flume's flake is unchanged.

## What Changes

- **The pin.** `flake.nix` names Rex's flake by the URL Flume's tree locks at the tag the lock names for Flume, its `nixpkgs` following the flake's, and the lock gains a `rex` node locking it by the hash Flume's `devenv.lock` does (core.44, core.45).
- **The package.** The flake gives `rex` for each of its three systems, Rex's flake's own `rex`, unchanged: Rex's release binary, its version the release's, `0.1.0-preview.14`. Rex's flake packages exactly the set's three systems, and allows Rex alone of unfree packages in its own nixpkgs.
- **Held to Flume's.** The flake's test reads Flume's `devenv.lock` at the tag the lock names for Flume and fails where `flake.nix` names another URL for Rex than it, or the lock locks another URL or hash, naming both. So a move of Flume's pin to a tag whose tree locks another Rex fails the gate until the set's Rex moves with it.
- **The module installs it.** `services.boomdrive.rex.package`, the set's `rex` by default, a host's own replacing it and `null` installing none, is put in the system profile with the runner, as worktrunk is. The modules name Rex in no setting of Flume's, so Flume's `host.multiplexer` stays `herdr` by default, and write no file of Rex's and start no server of it.
- **The shell.** The flake's shell's `PATH` gains Rex beside herdr, since Flume in the shell drives whichever terminal its settings name from the `PATH`.
- **The cache.** The workflow builds and pushes `rex` with every other package, since it builds every package the flake gives.
- **The README** names Rex among the set, its option and its profile line.
- **Record 0023** is amended on 2026-11-18 with how Rex comes with the flake; core.44's, core.45's and core.46's details say what they allow of it.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `family-packages`: the set gives `rex`, Rex's flake's own package at the release its URL names, and holds that URL and its hash to the ones Flume's `devenv.lock` locks at the tag the lock names for Flume.
- `service-modules`: the modules install Rex with the runner by `services.boomdrive.rex.package`, replaceable or dropped, and name it in no Flume setting; the README names the option.
- `binary-cache`: the workflow's builds and a host's profile hold the set's `rex`.
- `way-in`: the flake's shell's `PATH` holds Rex.

## Impact

- `flake.nix`, `flake.lock`: the input `rex` and the package `rex`. No other input moves.
- `nix/modules/apps.nix`: the option `services.boomdrive.rex.package`, in the system profile with the runner.
- `nix/shell.nix`: Rex on the shell's `PATH`.
- `nix/tests/packages.nix`: "Rex through its own flake", "Rex is Flume's" and "Rex other than Flume's".
- `nix/tests/apps.nix`: Rex with the runner, a host's own, none, and none without the runner.
- `nix/tests/typecheck.nix`, `nix/tests/cache.nix`, `nix/tests/way-in.nix`: `rex` among the set, the flake's own paths and the shell's tools.
- `README.md`, `CLAUDE.md`, `openspec/config.yaml`: Rex among the set.
- `docs/adr/0023-…`: its amendment of 2026-11-18, how Rex comes with the flake.
- `docs/architecture/core.md`: core.44's, core.45's and core.46's details.
- The runner's code and its file do not change. No crate, tool the gate runs or tracked file type is added (core.7, core.8).
- Outside this repository: every host that switches to a set with this change gets `rex` in its system profile, Rex's release binary, substituted from the family's cache or downloaded from Superlogical's where its nix does not trust the cache. A host that wants none sets `services.boomdrive.rex.package = null`.

## Touches

```
flake.nix
flake.lock
nix/modules/apps.nix
nix/shell.nix
nix/tests/packages.nix
nix/tests/apps.nix
nix/tests/typecheck.nix
nix/tests/cache.nix
nix/tests/way-in.nix
README.md
CLAUDE.md
openspec/config.yaml
docs/adr/0023-the-boomdrive-flake-configures-the-whole-host-each-tool-by-its-own-module-and-each-repository-named-once-by-its-owner.md
docs/architecture/core.md
```
