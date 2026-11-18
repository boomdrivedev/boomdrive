# Tasks

A task is true once the check it names passes, run as it names it.

## 1. The script and the check

- [x] 1.1 `activate.sh`'s `case` stands in a function of its own, called inside `$( )`, and the activation's cases hold. True when `sluice exec main -- moon run nix:test` passes, and `sluice exec main -- moon run root:format` passes, shfmt leaving the file as it is.
- [x] 1.2 `nix:mac-sh` reads every script under `nix/modules` and `nix/packages` with bash 3.2.57 in POSIX mode, and fails on `activate.sh` as `23e41810` holds it. True when `sluice exec main -- moon run nix:mac-sh` passes, and `sh nix/mac-sh.sh` in the devenv shell, with `23e41810`'s `activate.sh` in place, exits 1 naming its line 138.
- [x] 1.3 `devenv.nix` pins bash 3.2.57 and gives it by `BOOMDRIVE_MAC_SH`, on no PATH. True when `sluice exec main -- moon run root:toolchain` passes, saying `macOS's /bin/sh is 3.2.57`.

## 2. The record and the rule

- [x] 2.1 Record 0020's amendment of 2026-11-15 and core.8's details name the build. True when `sluice exec main -- moon run root:adrs` passes.

## 3. The spec

- [x] 3.1 The change's delta passes openspec's strict check. True when `sluice exec main -- moon run root:changes` passes.
- [x] 3.2 The change's delta archives into specs that pass openspec's strict check, the requirement "A Mac's /bin/sh reads every script the modules and packages give it" holding its two scenarios. True when, after `devenv shell -- openspec archive activate-sh-parses-under-macos-sh --yes`, `sluice exec main -- moon run root:specs` passes.
