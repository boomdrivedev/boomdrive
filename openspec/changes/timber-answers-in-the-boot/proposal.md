# Proposal

## Why

The brief: "Make bolt/boomdrives-hosts-carry-timber pass the checks of main that failed on its submission. [sluice] #207 failed on main: nix:test." Its log says: "nix: Every tool installed answers: the set's timber, in the boot's system profile, has no answer".

The bolt carries timber-comes-with-boomdrives-flake, under which the modules install timber with the runner. main has since landed a-booted-nixos-system-proves-the-set, whose boot runs each tool the modules install and holds, in the booted-system spec's scenario "Every tool installed answers", that each of the set's packages in the boot's system profile has an answer. Its list of answers was written before timber was in the set, so main and the bolt together install timber and ask it nothing.

## What Changes

- The boot's answers, in `nix/boot.nix` and in the nix project's test of it, `nix/tests/boot.nix`, hold timber's after crew's: `timber --version`, holding the version of the flake's `aarch64-linux` package of timber. timber's own statement has `timber --version` print `timber <version>, revision <revision>, built by <build>` on one line, the version its tree's `Cargo.toml` gives, so its answer holds the set's version as every other tool's does.
- The booted-system spec's scenario "Each tool's answer" names `timber` among the answers.

### What stays

- Every other answer, its command and its order; the requirement's words; how the script runs each answer; the scenario "Every tool installed answers", which now holds for timber.

### Not taken

- **timber at the end of the list.** The answers follow the family's order, the runner, Flume, sluice, crew and timber, then the tools the set brings, as the flake's description names them.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `booted-system`: timber answers in the booted system.

## Impact

- `nix/boot.nix` and `nix/tests/boot.nix`: timber among the tools.
- `openspec/specs/booted-system/spec.md`, by this change's delta once archived.
- No record or rule changes: the requirement already asks every tool the modules install to answer, and record 0020's amendment of 2026-11-14 names no tool.
