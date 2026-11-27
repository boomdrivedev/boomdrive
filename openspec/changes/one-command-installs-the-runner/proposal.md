# Proposal

## Why

The control plane's page copies one whole `boomdrive enroll` command (the change a-host-enrolls-from-one-command). But the host must have the runner first. The first-run mockup draws one command that installs the runner and enrolls the host. The enroll change named the install as the next change of this repository.

A release builds no binary (core.48). The set reaches a host from the family's cache, through the flake (core.37, core.47). The runner also needs Flume, sluice and herdr to do work. So the install adds the set from the flake, and installs Nix first where the host has none.

## What Changes

- **One install script, `install.sh`, at the root of the tree.** It finds the system. It stops on a host where Nix sets up the runner. It installs Nix where the host has none, and sets Nix to use the family's cache. It adds the set at the release's tag to the person's Nix profile. Then it runs the enroll command that it was given, with the token on the same standard input.
- **It asks first.** Before it installs Nix or changes Nix's settings, it says what it will do and asks. `--yes` is the answer yes. The control plane's page puts `--yes` in the command that it copies.
- **Each release attaches the script.** The release commit writes the release's version into the script, as it does into `Cargo.toml` and `Cargo.lock`. The workflow `release` attaches the script from the release commit to the GitHub release. GitHub serves it at `https://github.com/boomdrivedev/boomdrive/releases/latest/download/install.sh`, and at `releases/download/v<version>/install.sh` for one release.
- **The gate tests the script's choices.** A new task, `root:install`, runs the script with stubs of the programs that it runs, under the shell's `sh` and under macOS's `/bin/sh`.

## Capabilities

### New Capabilities

- `install`: the install script, its choices, what it runs and where GitHub serves it.

### Modified Capabilities

- `releases`: the release commit writes the version into `install.sh` too, and the workflow `release` attaches that one file to the release.

## Impact

- New: `install.sh`, `check-install.sh`, and the task `root:install` in `moon.yml`.
- `set-version.sh`, `check-versions.sh`, `release.sh`, `check-releases.sh` and `.github/workflows/release.yml`: the version in `install.sh`, and the file on the release.
- `docs/adr/0012-*.md`: an amendment of 2026-11-27, which names the install as the script of this change.
- `README.md` and `CLAUDE.md`: the install's command.
- The control plane's page changes its machine step to copy the install's command. That change follows this one and cites it.
