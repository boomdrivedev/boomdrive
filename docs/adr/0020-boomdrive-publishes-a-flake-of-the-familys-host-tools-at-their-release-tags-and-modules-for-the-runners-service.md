---
number: 20
title: Boomdrive publishes a flake of the family's host tools at their release tags, and modules for the runner's service
status: proposed
date: 2026-11-05
decision-makers:
- Chuck Swanberg
---

# Boomdrive publishes a flake of the family's host tools at their release tags, and modules for the runner's service

Status: proposed on 2026-11-05 by the construct stage of the change `boomdrive-publishes-a-flake-of-the-family-and-the-runners-service`. That this repository publishes a flake, with a package set and service modules, is the user's decision, in the words below. How the set is built, how it is proven, and what the modules write are the stage's choices, made on the written rules. How the set is built amends core.37's details, and that part is the user's to rule on at the change's review. The record is accepted when the user says yes to it.

Deciders: the user, for the flake and what it holds; the construct stage, for its form; the user, at the change's review, for the set being built from source.
Sources: the brief of the change, 2026-11-05, and the user's words in it; records 0009, 0012, 0016, 0017, 0018 and 0019; rules core.4, core.7, core.8, core.12, core.13, core.17, core.18, core.32, core.37, core.38 and core.41; agentplot/sluice's and agentplot/timber's `flake.nix` and release workflows, and agentplot/flume's release workflow and `set-version.sh`, each public; afterthought/swancloud's rules packages.4, packages.5, packages.6 and files.1, its records 0042, 0044 and 0050, and its runner's module and check, cited by id (core.41); nixpkgs' NixOS `switch-to-configuration`, and nix-darwin's `launchd.user.agents`, each public.

## Context and problem statement

The user's idea, 2026-11-05, in the brief's words: Boomdrive publishes a public flake, holding "a package set that pins the family together: the runner, Flume, sluice and herdr's plugins, at versions known to work with each other", and "nix-darwin and NixOS modules for the runner's service". Anyone with nix then installs the whole setup in a few lines, and a host's configuration imports the modules instead of carrying its own. Asked when to design it, the user answered "After the runner's service runs on [a host]". Reading a host's configuration's design for retiring its views the same day, they wrote: "I think I would like to see this combined with the Boom Drive flake. I haven't seen that design yet so I don't know if someone forgot about it". That design turns on `code_workspaces` and widens the runner's repositories in the host's own copy of the runner's file. The user wants those settings to come through this flake's module.

Record 0012 already has this repository publish a flake, "its packages and a module", and says both install routes "install the same attested releases". core.37 rules out "a nix flake that installs what the release workflow did not attest". No release of the family carries an attestation today, and the runner has no release at all.

The family packages its tools three ways today. sluice and timber publish flakes that build from source, and report the version a release build is given in the environment, so a pure build of their tag says `0.0.0`. Flume publishes no flake. Its tag names a release commit with its version written into its tree. A host's configuration packages Flume and sluice from their release archives, patching sluice's, which name the Nix store of its build, and builds the runner from source, which the user chose knowing it compiles on each host (swancloud packages.4 and packages.5).

## Considered options

For how the set is built:

1. From source at each tool's release tag, through the tool's own flake where it publishes one, locked by its hash in this flake's lock.
2. From each tool's release archive, by the digest GitHub publishes, verifying an attestation at each bump once the releases carry one; the runner from this tree until it has a release.
3. Each tool's flake as it stands, taken whole.

For how a set is proven: by the runner's tests run with the set's own binaries in the gate; by fixtures of each version's answers alone; or by a host running it.

For the modules: nix-darwin's and NixOS's own service options, or home-manager's.

For the file: written into the store and named by the service, and linked at the runner's own path; or written at activation as a real file in the user's home.

## Decision outcome

Option 1, proven by the runner's tests with the set's binaries, through each platform's own service options, with the file in the store.

- **Built from source at release tags.** The set is the runner at this tree, and Flume and sluice each at a release tag, with their herdr plugins from the same tags. Each is built on the host from the source the flake's lock names by its hash, so nothing prebuilt runs. The runner's package is this tree's. sluice's is sluice's own flake's. Flume's is a recipe of this flake's until Flume publishes a flake of its own. Each reports its tag's version.
- **core.37 is amended.** A flake installs no release binaries the release workflow did not attest, and a package nix builds on the host from a source its lock names by hash is no release binary. This keeps core.37's purpose, that nothing runs whose making cannot be checked, for the route where the host makes it.
- **Proven in the gate.** A set is the one the runner's tests pass with. The gate's shell gets the set's Flume and sluice, built from the same lock, by path. The runner's tests run each, check its version against its tag, and read its answers a scratch host gives with the runner's own readers. A set moves by one change that moves the tags and the lock together.
- **Overridden as nix overrides.** A tool's version is moved by overriding its input, and a package replaced through the module's options. An override is the host's, and the runner's own floors still judge it (core.38).
- **Modules of each platform's own.** The nix-darwin module is a launchd agent of the primary user's login, through `launchd.user.agents`. The NixOS module is a systemd user unit of a named user, lingering. Their options are one set, `services.boomdrive`. Its `settings` is the runner's file, each key named as the file names it (core.12, core.17).
- **The file in the store.** The module renders the file in Nix, and the service runs `boomdrive run --config <the file>`, so a changed file or package changes the service and restarts it. The same file is linked at the runner's own path, so a command run by hand reads what the service reads. It is the host's level of record 0009. A key naming a credential fails evaluation, since the store is readable by every user (core.4).
- **The host keeps its own.** Its accounts, which hosts run the runner, its repositories, its partitions and control plane, its git and herdr, its secrets and its enrollments stay the host's configuration. Enrollment is `boomdrive enroll <label>` by hand, its token on standard input and its key in the host's credential store (core.32).

Option 2 needs no compiler on a host, and is what a host's configuration does today. But it re-packages every tool here, patching sluice's binaries on each platform, and no release it would fetch carries an attestation, so it meets core.37 no better. Option 3 gives a sluice that says `0.0.0`, below any floor the runner names, and no Flume at all.

Fixtures alone prove the runner reads what a version printed once. The set's binaries in the gate prove the versions the flake names. A host running it proves nothing before it lands.

home-manager's options would bring home-manager into every host that imports the modules, where nix-darwin and NixOS each run a user's service themselves. A real file written at activation needs a writer and a restart of the module's own on each platform, and a store link holds nothing secret, since no key may name one.

### Consequences

- core.7 is amended: the flake is a moon project of its own, as each crate is. core.44, core.45 and core.46 are added. Each is in this record's commit.
- A host that takes the set compiles the runner, Flume and sluice at each bump, unless a cache it trusts serves them. A public cache is not in this record.
- The gate evaluates the flake and builds Flume and sluice, but not the runner's package. The runner's crates are the ones its checks compile, and the first host that builds the package after a landing is the first to build it with nix. The release workflow of record 0012 builds the flake on each system before it publishes.
- Flume is asked to publish a flake, and sluice and timber to carry each release's version in the tagged tree, as Flume's release commit does. Until then, this flake builds Flume by a recipe of its own and gives sluice its tag's version.
- A host's configuration imports the modules in place of its own, and drops its own package of the runner.

Rules: core.44, core.45 and core.46; core.7 and core.37 amended.
