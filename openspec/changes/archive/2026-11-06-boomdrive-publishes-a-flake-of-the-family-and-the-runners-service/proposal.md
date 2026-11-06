# Proposal

## Why

The user's idea, 2026-11-05, in the brief's words: Boomdrive publishes a public flake holding "a package set that pins the family together: the runner, Flume, sluice and herdr's plugins, at versions known to work with each other", and "nix-darwin and NixOS modules for the runner's service". Anyone with nix then installs the whole setup in a few lines, and a host's configuration imports the modules instead of carrying its own. Asked when to design it, the user answered "After the runner's service runs on [a host]". It runs there now, as a launchd agent a host's configuration writes with a module of its own.

The same day, reading that configuration's design for retiring its views, the user wrote: "I think I would like to see this combined with the Boom Drive flake. I haven't seen that design yet so I don't know if someone forgot about it". That design turns on `code_workspaces` and widens the runner's repositories in the host's own copy of the runner's file. The user wants those settings to come through this flake's module.

Reading this change's design, the user ruled how the set reaches a host: "the whole point of the flake is to distribute all of the binaries so it should be the runner, flume, and sluice"; "I have a Cachix open-source binary cache that we can use to publish the binaries so that people don't have to rebuild them"; and "Let's set up publishing to cachix". So the flake distributes built binaries of the runner, Flume and sluice through the user's Cachix cache, and a host that trusts it compiles nothing. Approving the design on 2026-11-06, the user named the cache `boomdrive`, signed by Cachix's own key ("the token is all the workflow needs"), and ruled that the workflow reads its token from 1Password: "Configure the GitHub Actions to use a service account token from 1Password to read the variable".

Record 0012 already says this repository publishes a flake. Nothing builds it yet. A host that runs the runner today packages it, compiles it, writes its file, keeps its service and restarts it, all in its own configuration.

## What Changes

- **Record 0020 and rules core.44 to core.47**, with core.7 and core.37 amended, in this change's commit beside its design.
- **A flake at the root, the rest under `nix/`,** as one moon project, `nix`, with its own format, lint, type check and test in the fast layer. It owns the flake's workflow too.
- **The package set.**
  - The runner, built from this tree.
  - Flume and sluice, each from the source of a release tag, sluice's through its own flake and Flume's through a recipe of this flake's until Flume publishes one.
  - The two tools' herdr plugins, from the same tags.
  - Each package reports its tag's version, on `aarch64-darwin`, `aarch64-linux` and `x86_64-linux`. nixpkgs 26.11 builds nothing for `x86_64-darwin`, so the set gives it none.
  - A tool's version moves by its input. A package is replaced through the modules' options.
- **The binaries, through the family's cache.**
  - A workflow, `.github/workflows/cache.yml`, builds every package for each system on GitHub's hosted runners, and pushes them to `boomdrive` on Cachix. It runs at each push to `main` and each tag `v*`, the runner's package among them.
  - The flake's `nixConfig` names the cache and its key as a substituter.
  - A host whose nix trusts the cache substitutes the set. A host that does not builds the same paths from source.
  - The modules' packages are the flake's own, made with its own nixpkgs, so they are the paths the cache holds.
  - The push's token is kept in 1Password and loaded by the workflow through a service account, whose token is the one GitHub secret, `OP_SERVICE_ACCOUNT_TOKEN` of the environment `cachix`. Cachix signs what is pushed with the cache's own key.
- **The set is proven in the gate.** The shell gives the runner's tests the set's Flume and sluice, from the same lock, by path, substituted from the cache. The tests run each, check its version against its tag, and read the answers a scratch host gives with the runner's own readers.
- **Two modules, one set of options, `services.boomdrive`.**
  - nix-darwin: a launchd agent of the primary user's login.
  - NixOS: a systemd user unit of a named user, lingering.
  - `settings` is the runner's file, each key as the file names it: `repositories`, `repository_root`, `fetch_interval`, `code_workspaces`, `control_plane`.
  - The file is rendered in Nix, named by the service as `--config`, and linked at the runner's own path. A changed file or package restarts the runner.
  - The runner and the set's Flume and sluice go in the system profile, and the service's `PATH` is the user's profiles, then the system's.
  - A key named for a credential fails evaluation. Enrollment stays `boomdrive enroll <label>` on the host.
- **The README** gains how to install with nix and trust the cache, and **CLAUDE.md** names the new project.

## Capabilities

### New Capabilities

- `family-packages`: the flake's package set, how each tool's package is made and versioned, how a set is proven, and how a host overrides one.
- `binary-cache`: the family's cache, the workflow that builds and pushes the set to it, the flake's `nixConfig`, and what a host that trusts it, or does not, gets.
- `service-modules`: the nix-darwin and NixOS modules, their options, the file they write, the service they keep and what they leave to the host.

### Modified Capabilities

None. The runner's own behaviour, its file and its command line do not change.

## Impact

- `docs/architecture/core.md` gains core.44 to core.47 and amends core.7 and core.37, and `docs/adr/` gains record 0020, in this change's commit.
- New: `flake.nix`, `flake.lock`, `.github/workflows/cache.yml`, and `nix/`, holding the packages, the modules, their tests and the project's `moon.yml`.
- `.moon/workspace.yml` lists the `nix` project.
- `devenv.nix` pins nix, nixfmt, statix, deadnix and actionlint for the project's tasks, fetches the flake's inputs from its lock for the gate, pulls from the family's cache, and gives the set's Flume and sluice to the runner's tests by path, never on `PATH`.
- `check-toolchain.sh` holds the new pins. `treefmt.nix` excludes the rendered files the tests compare byte for byte.
- `crates/boomdrive/tests/` gains a test of the set's tools, and one reading each file the modules render. `crates/boomdrive/moon.yml`'s test reads those files and the lock.
- `README.md` and `CLAUDE.md`.
- On GitHub and in 1Password: the user makes the environment `cachix`, the cache's token in the vault `GitHub-Actions`, and a service account whose token is the environment's secret `OP_SERVICE_ACCOUNT_TOKEN`, by the commands of the design's After deploy.
- Outside this repository: Flume is asked for a flake of its own; sluice and timber to carry each release's version in the tagged tree; the family to push each tool's own binaries to the same cache, as recommended; and a host's configuration to import the modules, with options named as here, and to trust the cache (design, What this change asks of others).

## Touches

```
docs/architecture/core.md
docs/adr/0020-boomdrive-publishes-a-flake-of-the-familys-host-tools-their-binaries-in-the-familys-cache-and-modules-for-the-runners-service.md
flake.nix
flake.lock
nix/**
.github/workflows/cache.yml
.moon/workspace.yml
devenv.nix
devenv.lock
treefmt.nix
check-toolchain.sh
crates/boomdrive/moon.yml
crates/boomdrive/tests/family.rs
crates/boomdrive/tests/module_files.rs
crates/boomdrive/tests/common/**
README.md
CLAUDE.md
openspec/config.yaml
```
