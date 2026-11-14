# Tasks

A task is true once the check it names passes, run as it names it.

## 1. The boot

- [x] 1.1 The boot's answers hold timber's, `timber --version` with the set's timber version, after crew's. True when `nix/tests/boot.nix`'s cases "Each tool's answer" and "Every tool installed answers" hold, with `sluice exec main -- moon run nix:test` passing.

## 2. The spec

- [x] 2.1 The change's delta passes openspec's strict check. True when `sluice exec main -- moon run root:changes` passes.
- [x] 2.2 The change's delta archives into specs that pass openspec's strict check, the requirement "Each tool of the set answers in the booted system" naming `timber` among its answers. True when, after `devenv shell -- openspec archive timber-answers-in-the-boot --yes`, `sluice exec main -- moon run root:specs` passes.
