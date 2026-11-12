# Proposal

## Why

The Boomdrive launcher opens one stream, Flume's default. A click on `Boomdrive` runs `flume open` with no name, so Flume shows the host's default stream's own session (Flume's statement 186). On a host of record 0028, each drive's places run in the drive's own stream, so the launcher never shows them. A person types `flume open <session>` in a terminal to see a drive's agents.

The brief of this change: "A person opens any drive's agents from the Boomdrive launcher, one entry per drive of the host, each showing that drive's stream with flume open: a menu-bar menu on macOS and one action per drive in the desktop entry on Linux." Its sources:

- **Record 0023** (accepted 2026-11-06) gives the person one way in, a Boomdrive app that runs `flume open`, in `/Applications/Nix Apps` on macOS and as a desktop entry on Linux.
- **Record 0028** (accepted 2026-11-07) gives each drive one Flume stream, named after the drive unless the drive names another. A drive of another machine has no stream on this host.
- **Flume's statement 186.** A person opens a session by its name, never a stream. A stream's own session is the one Flume's settings name for it, else the stream's name. Asked for no name, Flume shows the default stream's own.

The pieces are ready:

- The module knows each drive of this host and its stream (`nix/modules/drives.nix`), and Flume's module at the set's tag, `v0.21.2`, declares a stream's `session`.
- Flume `v0.21.2` takes `flume open [--] [<session>]`.
- nixpkgs' `makeDesktopItem` writes a desktop entry's `Actions`.

## What Changes

- **The script opens one session.** `boomdrive-launcher <session>` runs `flume open -- <session>`. With no argument it runs `flume open`, as today. Flume's refusal is shown as today.
- **The launcher is made for drives.** `nix/packages/launcher.nix` takes `drives`, a list of each drive's name and session, empty in the set's own package.
  - **macOS.** `Boomdrive.app` becomes a menu-bar app: a status item `Boomdrive` whose menu holds one entry for each drive, then `Default stream`, then `Quit Boomdrive`. A drive's entry runs the script with its session, and `Default stream` runs it with none. The app is `nix/packages/launcher.m`, Objective-C, compiled by the flake's own nixpkgs. The drives are in its `Info.plist`.
  - **Linux.** `boomdrive.desktop` gains one action for each drive, which runs the script with the drive's session. A click on the entry itself opens the default stream, as today.
- **The modules give the launcher this host's drives.** With `services.boomdrive.enable`, the default `services.boomdrive.launcher.package` is the set's launcher made, by its `override`, for each drive of this host, in name order. A drive's session is its stream's `session` in Flume's settings, else the stream's name. A drive of another machine has no entry. A host's own package is installed as given.
- **clang-format joins the gate.** treefmt formats `nix/packages/*.m` with clang-format, at the version `devenv.nix` pins and `check-toolchain.sh` asserts (core.7, core.8).
- **Words.** The README's install with nix, CLAUDE.md and `openspec/config.yaml` say the launcher has an entry for each drive.
- **Records and rules.** Record 0023 gains an amendment that carries this change's choices. The details of core.13, core.44, core.46 and core.47 are amended in the commit that writes this change (design, Rules amended).

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `way-in`: the launcher's script opens the session it is given; the launcher is a menu-bar app on macOS and a desktop entry with an action for each drive on Linux, made for the drives it is given.
- `service-modules`: the launcher the modules install is the set's made for each drive of this host, each entry opening its stream's own session.
- `binary-cache`: the launcher made for a host's drives is the flake's own launcher by its override, its compiled menu the set's path the cache holds.

## Impact

- `nix/packages/`: `launcher.sh`, `launcher.nix`, and `launcher.m`, which is added.
- `nix/modules/`: `accounts.nix` gives the drives of this host to `drives.nix` and `apps.nix`; `apps.nix` makes the launcher's default.
- `nix/tests/`: `way-in.sh`, `way-in.nix`, `apps.nix` and `cache.nix`.
- `treefmt.nix`, `devenv.nix` and `check-toolchain.sh`: clang-format.
- `README.md`, `CLAUDE.md` and `openspec/config.yaml`.
- `docs/adr/0023-…` and `docs/architecture/core.md`, in the commit that writes this change.
- The runner's code does not change. No crate is added. One file type, `.m`, is added, with its formatter (core.7, core.8).
- Outside this repository: none. Flume's `open` and its module's `session` are used as Flume `v0.21.2` gives them.

## Touches

```
nix/packages/**
nix/modules/**
nix/tests/**
treefmt.nix
devenv.nix
check-toolchain.sh
README.md
CLAUDE.md
openspec/config.yaml
docs/adr/0023-the-boomdrive-flake-configures-the-whole-host-each-tool-by-its-own-module-and-each-repository-named-once-by-its-owner.md
docs/architecture/core.md
```
