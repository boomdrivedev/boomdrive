# Tasks

## 1. The install

- [x] 1.1 `install.sh` takes `--yes` and the enroll command's three flags, together or none, and refuses any other flag with exit 2 (Decision 1). True when `sluice exec main -- moon run root:install` passes, proving "A flag that is not the install's", "Part of the enroll command" and "No enroll command".
- [x] 1.2 It runs on macOS on Apple silicon alone, and stops on Linux and on an Intel Mac (Decision 3). True when `sluice exec main -- moon run root:install` passes, proving "Linux" and "A Mac that is not Apple silicon".
- [x] 1.3 It stops where Nix sets up the runner, or where another runner is on the `PATH`. It goes on to enroll where this release's runner is on the `PATH` (Decision 4). True when `sluice exec main -- moon run root:install` passes, proving "Nix sets up the runner", "Another runner on the PATH" and "This release's runner on the PATH".
- [x] 1.4 It asks before it installs Nix, and runs Nix's installer only at the pinned digest (Decision 5). True when `sluice exec main -- moon run root:install` passes, proving "No Nix, and no answer", "An installer whose digest is not the pinned one" and "No Nix, and yes".
- [x] 1.5 It asks before Nix uses the cache, and writes the lines where the host's Nix reads them (Decision 5). True when `sluice exec main -- moon run root:install` passes. It proves "Nix that uses the cache" and "Nix that does not use the cache, and no answer". It proves "Nix that does not use the cache, and yes", "Determinate's Nix" and "A Nix configuration the install cannot write".
- [x] 1.6 It adds the set at its tag and gives the token to `boomdrive enroll` alone (Decision 6). True when `sluice exec main -- moon run root:install` passes, proving "The token on enroll's standard input alone" in each case that enrolls.
- [x] 1.7 Each case runs under the shell's `sh` and macOS's `/bin/sh`, and treefmt's shellcheck and shfmt hold both files (Decision 7). True when `sluice exec main -- moon run root:install root:format` passes.

## 2. The release

- [x] 2.1 `set-version.sh` writes `install.sh`'s `version=`, and `root:versions` holds it to `0.0.0` (Decision 6). True when `sluice exec main -- moon run root:versions root:releases` passes, proving "The install's version written" and "An install of another version".
- [x] 2.2 The release commit changes `Cargo.toml`, `Cargo.lock` and `install.sh` alone, and the workflow `release` attaches the release commit's `install.sh` (Decisions 2 and 6). True when `sluice exec main -- moon run root:releases root:workflow` passes, proving "The release commit" and "A release".

## 3. The words

- [x] 3.1 Record 0012 gains its amendment of 2026-11-27, and README and CLAUDE.md give the install's command. True when `sluice exec main -- moon run root:format` passes.
