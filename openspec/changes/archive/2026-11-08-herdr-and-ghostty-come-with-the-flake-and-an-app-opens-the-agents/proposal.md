# Proposal

## Why

Record 0023, accepted by the user on 2026-11-06, puts herdr and Ghostty in the set and gives the person one way in:

- **herdr.** The flake installs herdr at the version Flume's terminal profile was checked against, with Flume's and sluice's herdr plugins. In the user's words, "In the set, overridable".
- **Ghostty.** The flake installs Ghostty, which the module names Flume's terminal app where the host names none. In the user's words, "Install Ghostty too".
- **A way in.** The module installs a Boomdrive app, in `/Applications/Nix Apps` on macOS and as a desktop entry on Linux, which runs `flume open`. The flake gives a shell, its `PATH` the set's tools, that starts herdr on entering. In the user's words, "An app to click" and "A shell".
- **Each is overridable.** A host replaces herdr or Ghostty through the modules' options, or drops it for its own.
- **Each runs at the person's asking.** The runner still starts no multiplexer (core.13) and names none (core.29).

The user's ask, 2026-11-06: "Is there a way to make it so that when somebody installs this, it could also install Herdr and maybe give them one easy thing to click to open up the preconfigured Herdr instance? either that or have a shell they drop into that is preconfigured to launch herdr or something".

Today herdr and git are the host's own, and the set holds neither herdr nor Ghostty (core.44 as record 0012 left it). Nothing links the set's herdr plugins: the plugin package's comment leaves linking to the host. Flume's file names no terminal or window, so a fresh Mac that takes the flake has no way to see its agents.

The pieces are ready:

- herdr publishes a flake at its release tag `v0.9.5`, the herdr Flume's tree pins at its set tag `v0.18.0`.
- The flake's locked nixpkgs carries Ghostty `1.4.0`, the release Flume's window profile was checked against: built from source on Linux, and Ghostty's own release on macOS.
- Flume `v0.18.0` gives `flume open [<session>]` (Flume's statement 186), and its module declares `host.multiplexer` and `host.terminal_app`.

This change is the unit `herdr-and-ghostty-come-with-the-flake-and-an-app-opens-the-agents` of the bolt `the-boomdrive-flake-configures-the-whole-host`, after the unit that brings each tool's own module.

## What Changes

- **herdr in the set.** herdr's flake joins the inputs at `v0.9.5`, following the flake's `nixpkgs` and `rust-overlay`. `packages.<system>.herdr` is herdr's flake's package, unchanged. The test holds herdr's version to the herdr Flume's tag pins, and each of the set's plugins' `min_herdr_version` at or below it.
- **Ghostty in the set.** `packages.<system>.ghostty` is the flake's nixpkgs' Ghostty: `ghostty` on Linux, and `ghostty-bin`, Ghostty's own release, on macOS. The test holds it to `1.4.0`.
- **The Boomdrive launcher.**
  - `packages.<system>.boomdrive-launcher`, of this tree, runs `flume open` with the user's profiles first on `PATH`.
  - Where Flume refuses, it shows Flume's words: in an alert on macOS, on standard error elsewhere.
  - It is `Boomdrive.app` on macOS and the desktop entry `boomdrive.desktop` on Linux.
- **The shell.** `devShells.<system>.default` holds the set's runner, Flume, sluice, crew and herdr. Entered interactively, outside herdr, it starts herdr, and leaves the person in the shell when herdr ends.
- **The modules.** With `services.boomdrive.enable`:
  - **Installs.** They install `services.boomdrive.herdr.package`, `services.boomdrive.ghostty.package` and `services.boomdrive.launcher.package`. Each is the set's by default, replaced by a host's own, and none where `null`.
  - **Links the plugins.** Each activation runs herdr's own `plugin link` for each package of `services.boomdrive.herdr.plugins`, as the runner's user. The default is the set's two plugins.
  - **Names Flume's terminal and window.** They set Flume's `host.multiplexer` to `herdr` and `host.terminal_app` to `ghostty` by default, a host's own value winning. Where the host writes Flume's older `host.terminal`, they set no `multiplexer`.
- **Words.**
  - The README's install with nix names herdr, Ghostty, the launcher, the shell and their options. git alone stays the host's own.
  - CLAUDE.md and `openspec/config.yaml` name them in the set.
  - core.13's, core.29's, core.44's and core.46's details are amended in the commit that writes this change (design, Rules amended).

## Capabilities

### New Capabilities

- `way-in`: the Boomdrive launcher, an app on macOS and a desktop entry on Linux that runs `flume open`, and the flake's shell that starts herdr.

### Modified Capabilities

- `family-packages`: the set gains `herdr`, through its own flake at the herdr Flume's terminal profile was checked against, `ghostty`, from the flake's nixpkgs at the release Flume's window profile was checked against, and `boomdrive-launcher`, of this tree; git alone is the host's own; herdr is named by a release tag.
- `service-modules`: the modules install herdr, Ghostty and the launcher, each replaceable or dropped; they link the set's herdr plugins with herdr's own command; they name herdr and Ghostty Flume's terminal and window by default; git alone is left to the host.
- `binary-cache`: herdr, Ghostty and the launcher are built and pushed with the rest, and the modules install the flake's own.

## Impact

- `flake.nix` and `flake.lock`: herdr's input; the packages `herdr`, `ghostty` and `boomdrive-launcher`; `devShells`.
- `nix/packages/`: `launcher.nix` and `launcher.sh` added; `herdr-plugin.nix`'s header.
- `nix/shell.nix` and `nix/shell-hook.sh`, added.
- `nix/modules/`:
  - `way-in.nix` added, holding herdr's, Ghostty's and the launcher's options and Flume's terminal and window.
  - `darwin.nix` and `nixos.nix` import it and link the plugins at activation.
  - `activate.sh` gains `plugins`.
- `nix/tests/`:
  - `way-in.nix` added to `default.nix`'s `test`.
  - `way-in.sh` added, run by `nix/check.sh test`.
  - `activate.sh`, `packages.nix`, `typecheck.nix`, `tags.nix`, `tools.nix` and `cache.nix` changed.
- `nix/check.sh`, `README.md`, `CLAUDE.md` and `openspec/config.yaml`.
- `docs/architecture/core.md`, in the commit that writes this change.
- The runner's code does not change, and runs no herdr (core.29). No crate, tool of the gate or tracked file type is added (core.7, core.8): the cases run stubs of herdr, Flume and `osascript`.
- Outside this repository: afterthought/swancloud, once it takes the flake, drops its hosts' own herdr and its link of the set's plugins, or keeps its own through these options (record 0023).

## Touches

```
flake.nix
flake.lock
nix/**
README.md
CLAUDE.md
openspec/config.yaml
docs/architecture/core.md
```
