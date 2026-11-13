# Tasks

A task is true once the check it names passes, run as it names it.

## 1. The workflow

- [ ] 1.1 The `x86_64-linux` job's first step removes the runner's preinstalled trees no package's build uses, and says the free space of `/` before and after, before Nix is installed, and the other systems' jobs run no new step. True when `nix/tests/workflow.nix`'s case "The x86_64-linux job has room" holds, with `sluice exec main -- moon run nix:test` passing, and `sluice exec main -- moon run nix:lint` passes, actionlint reading the workflow.

## 2. The record and the rule

- [ ] 2.1 Record 0020's amendment of 2026-11-13 and core.47's details name the step. True when `sluice exec main -- moon run root:adrs` passes.

## 3. The spec

- [ ] 3.1 The change's delta passes openspec's strict check. True when `sluice exec main -- moon run root:changes` passes.
- [ ] 3.2 The change's delta archives into specs that pass openspec's strict check, the requirement "The workflow builds and pushes every package of every system" holding the scenario "The x86_64-linux job has room". True when, after `devenv shell -- openspec archive cache-x86-build-has-room --yes`, `sluice exec main -- moon run root:specs` passes.
