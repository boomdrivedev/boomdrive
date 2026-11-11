# Proposal

## Why

Record 0023, as amended on 2026-11-08, puts worktrunk in the set. The user chose it on their card "Boomdrive flake: beyond Pending You and Plannotator, which of your tools should it also install?": "worktrunk (wt), portless (the .localhost dev addresses)". The record says:

- **What it is for.** The flake installs worktrunk's `wt`, which crew and sluice's worktree driver call.
- **Overridable as herdr is.** It is the set's by default. A host replaces it through the modules' options, or drops it for its own.

Today the set holds no worktrunk, and no module installs one. On a host that takes the flake:

- crew's `wt remove` and its `wt submit`, sluice's driver that worktrunk runs as `wt submit`, find a `wt` only where the host installed one itself;
- the flake's shell puts crew and sluice on `PATH` with no `wt` beside them.

The pieces are ready:

- **worktrunk's flake.** worktrunk publishes a flake at its release tag `v0.80.0`. It gives `packages.<system>.worktrunk` for `aarch64-darwin`, `aarch64-linux` and `x86_64-linux`, the three systems this flake gives. Its `mainProgram` is `wt` and its version is `0.80.0`.
- **The release sluice checked.** sluice's tree at the tag the lock names, `v0.78.1`, pins worktrunk `0.80.0` in its `devenv.nix`. Its comment calls it "the version they are tested against", the version its worktree driver's tests drive.
- **The checks can read it.** worktrunk's package, with the flake's `nixpkgs` and `rust-overlay` followed, evaluates on each of the three systems under the nix project's checks, offline against no store. Seen on 2026-11-11 with a scratch lock.

This change is the unit `worktrunk-comes-with-the-flake` of the bolt `the-flake-brings-its-companion-tools`.

## What Changes

- **worktrunk in the set.** worktrunk's flake joins the inputs at `v0.80.0`, following the flake's `nixpkgs` and `rust-overlay`. `packages.<system>.worktrunk` is worktrunk's flake's package, made by `nix/packages/worktrunk.nix`.
  - Its build is given its tag, `v0.80.0`, as its `git describe`, as worktrunk's release build gets it from git. So `wt --version` says its tag's version (core.44).
  - worktrunk's flake gives its build the commit's short id, `b49ca7e`, in its place.
- **Held to sluice's pin.** The test holds worktrunk's version to the worktrunk sluice's `devenv.nix` pins at sluice's locked tag, and to its own tag. `tags.nix` holds it to a release tag, as it holds Flume, sluice, crew and herdr.
- **The modules.** With `services.boomdrive.enable`, they install `services.boomdrive.worktrunk.package`. It is the set's by default, a host's own replaces it, and `null` installs none. Without the runner they install none.
- **The shell.** `devShells.<system>.default` holds the set's worktrunk beside the runner, Flume, sluice, crew and herdr.
- **Words.**
  - The README's install with nix names worktrunk in the set, its option, and the shell's `wt`. It says worktrunk's shell integration is the person's to install with worktrunk's own command.
  - CLAUDE.md and `openspec/config.yaml` name worktrunk in the set.
- **Records and rules**, amended in the commit that writes this change (design, Rules amended):
  - record 0023 gains the writing agent's choices for worktrunk;
  - core.44's and core.46's details name worktrunk.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `family-packages`: the set gains `worktrunk`, through its own flake at the worktrunk sluice's tree pins, named by a release tag.
- `service-modules`: the modules install worktrunk with the runner, replaceable or dropped; the README names its option.
- `binary-cache`: worktrunk is built and pushed with the rest, and the modules install the flake's own.
- `way-in`: the flake's shell holds worktrunk.

## Impact

- `flake.nix` and `flake.lock`: worktrunk's input, which brings worktrunk's own `crane` and `flake-utils` to the lock, and the package `worktrunk`.
- `nix/packages/worktrunk.nix`, added.
- `nix/modules/apps.nix`: the option `services.boomdrive.worktrunk.package`.
- `nix/shell.nix`: the set's worktrunk.
- `nix/tests/`: `typecheck.nix`, `packages.nix`, `tags.nix`, `apps.nix`, `cache.nix` and `way-in.nix`.
- `README.md`, `CLAUDE.md` and `openspec/config.yaml`.
- `docs/adr/0023-…md` and `docs/architecture/core.md`, in the commit that writes this change.
- The runner's code does not change and calls no `wt`. No crate, tool of the gate or tracked file type is added (core.7, core.8). The checks run no worktrunk: they evaluate it.
- `.github/workflows/cache.yml` does not change: it builds every package the flake gives.
- Outside this repository: afterthought/swancloud, once it takes the flake, either drops its hosts' own worktrunk for the set's or sets `services.boomdrive.worktrunk.package` to its own or to `null` (record 0023).

## Touches

```
flake.nix
flake.lock
nix/**
README.md
CLAUDE.md
openspec/config.yaml
docs/adr/0023-the-boomdrive-flake-configures-the-whole-host-each-tool-by-its-own-module-and-each-repository-named-once-by-its-owner.md
docs/architecture/core.md
```
