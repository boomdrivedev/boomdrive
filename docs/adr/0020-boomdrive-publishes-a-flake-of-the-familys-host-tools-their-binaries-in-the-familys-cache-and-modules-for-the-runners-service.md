---
number: 20
title: Boomdrive publishes a flake of the family's host tools, their binaries in the family's cache, and modules for the runner's service
status: accepted
date: 2026-11-05
decision-makers:
- Chuck Swanberg
---

# Boomdrive publishes a flake of the family's host tools, their binaries in the family's cache, and modules for the runner's service

Status: accepted by the user on 2026-11-06, approving the change's design ("Approve: start its code"). Proposed on 2026-11-05 by the construct stage of the change `boomdrive-publishes-a-flake-of-the-family-and-the-runners-service`. That this repository publishes a flake, with a package set and service modules, that the flake distributes the runner's, Flume's and sluice's binaries through the user's Cachix cache, which cache that is, and where the workflow reads its token from, are the user's decisions, in the words below. How the set is made, proven and pushed, and what the modules write, are the stage's choices, made on the written rules.

Deciders: the user, for the flake, what it holds, and its binaries through Cachix; the construct stage, for its form.
Sources: the brief of the change, 2026-11-05, and the user's words in it; the user's words on reading the change's design, 2026-11-05, and on approving it, 2026-11-06; records 0009, 0012, 0016, 0017, 0018 and 0019; rules core.4, core.7, core.8, core.12, core.13, core.17, core.18, core.32, core.37, core.38 and core.41; agentplot/sluice's and agentplot/timber's `flake.nix` and workflows, and agentplot/flume's release workflow and `set-version.sh`, each public; a host's configuration's runner module and check, read 2026-11-05; Cachix's public API for the cache `boomdrive` and `cachix/cachix-action`'s README, read 2026-11-05; GitHub's hosted runner images; nixpkgs' NixOS `switch-to-configuration`, and nix-darwin's `launchd.user.agents`, each public.

## Context and problem statement

The user's idea, 2026-11-05, in the brief's words: Boomdrive publishes a public flake, holding "a package set that pins the family together: the runner, Flume, sluice and herdr's plugins, at versions known to work with each other", and "nix-darwin and NixOS modules for the runner's service". Anyone with nix then installs the whole setup in a few lines, and a host's configuration imports the modules instead of carrying its own. Asked when to design it, the user answered "After the runner's service runs on [a host]". Reading a host's configuration's design for retiring its views the same day, they wrote: "I think I would like to see this combined with the Boom Drive flake. I haven't seen that design yet so I don't know if someone forgot about it". That design turns on `code_workspaces` and widens the runner's repositories in the host's own copy of the runner's file. The user wants those settings to come through this flake's module.

Reading the change's design, the user ruled how the set reaches a host. In their words: "the whole point of the flake is to distribute all of the binaries so it should be the runner, flume, and sluice"; "I have a Cachix open-source binary cache that we can use to publish the binaries so that people don't have to rebuild them"; and "Let's set up publishing to cachix". Approving the design on 2026-11-06, they named the cache, writing `{ cachix.push = "boomdrive"; }` against its name, had Cachix sign with the cache's own key ("the token is all the workflow needs"), and ruled where the workflow reads its token: "Configure the GitHub Actions to use a service account token from 1Password to read the variable".

Record 0012 already has this repository publish a flake, "its packages and a module", and says both install routes "install the same attested releases". core.37 rules out "a nix flake that installs what the release workflow did not attest", and a check against a key of our own in place of the workflow's identity. No release of the family carries an attestation today, and the runner has no release at all.

The family packages its tools three ways today. sluice and timber publish flakes that build from source, and report the version a release build is given in the environment, so a pure build of their tag says `0.0.0`. Flume publishes no flake. Its tag names a release commit with its version written into its tree. A host's configuration packages Flume and sluice from their release archives, patching sluice's, which name the Nix store of its build, and compiles the runner from source on each host. No tool pushes its binaries to a cache. Cachix gives a public cache `boomdrive`, which no file of the family names yet.

## Considered options

For how the set reaches a host:

1. Each package a derivation of the source the flake's lock names by its hash, built for each system by this repository's workflow on GitHub's hosted runners and pushed to the family's Cachix cache, which the flake names; a host that trusts the cache substitutes, and one that does not builds the same paths.
2. Each tool's release archive, by the digest GitHub publishes, verified by an attestation at each bump once the releases carry one; the runner from this tree until it has a release.
3. Each package built on each host from the same locked source, with no cache.

For how a set is proven: by the runner's tests run with the set's own binaries in the gate; by fixtures of each version's answers alone; or by a host running it.

For the modules: nix-darwin's and NixOS's own service options, or home-manager's.

For the file: written into the store and named by the service, and linked at the runner's own path; or written at activation as a real file in the user's home.

## Decision outcome

Option 1, by the user's ruling, proven by the runner's tests with the set's binaries, through each platform's own service options, with the file in the store.

- **The set.** The runner at this tree, and Flume and sluice each at a release tag, with their herdr plugins from the same tags. Each package is a derivation of the source the flake's lock names by its hash. The runner's is this tree's. sluice's is sluice's own flake's. Flume's is a recipe of this flake's until Flume publishes a flake of its own. Each reports its tag's version.
- **Its binaries in the family's cache.** This repository's workflow builds every package for each of the four systems the flake gives, on GitHub's hosted runners, and pushes them to `boomdrive` on Cachix, at each push to `main` and each tag `v*`. The runner's package is built and pushed with Flume's and sluice's. `main` holds only what the gate passed, so nothing is pushed that the gate did not pass. Cachix signs what is pushed with the cache's own key. The push's token is kept in 1Password, and the workflow loads it through a service account whose token is the one secret of an environment that admits `main` and tags `v*` alone.
- **A host's trust is its own.** The flake's `nixConfig` names the cache and its key, for a command run on the flake. A host's configuration adds the same two lines to its own nix settings, and the modules add neither. The modules' packages are the flake's own, made with its own nixpkgs, so they are the paths the cache holds. A host that does not trust the cache builds the same paths from the locked source.
- **core.37 is amended.** A store path a host's nix substitutes from the family's cache, checked against the cache's key the host trusts, or builds from the locked source, is no release. core.37's check of a release's provenance binds the installer's and `upgrade`'s releases, and a flake still installs no release binary the release workflow did not attest. This narrows record 0012's "Both install routes install the same attested releases", as the user's ruling asks: the flake distributes the binaries, and the installer's releases carry their attestations.
- **Proven in the gate.** A set is the one the runner's tests pass with. The gate's shell gets the set's Flume and sluice, the flake's packages of the same lock, by path, substituted from the cache where the host trusts it. The runner's tests run each, check its version against its tag, and read its answers a scratch host gives with the runner's own readers. A set moves by one change that moves the tags and the lock together. The runner's package, which no gate task builds, is built on every system by the workflow at the push that lands it, its one fixed hash held in the gate to the revision `Cargo.lock` names.
- **Overridden as nix overrides.** A tool's version is moved by overriding its input, and a package replaced through the module's options. An override is the host's, built by the host, and the runner's own floors still judge it (core.38).
- **Modules of each platform's own.** The nix-darwin module is a launchd agent of the primary user's login, through `launchd.user.agents`. The NixOS module is a systemd user unit of a named user, lingering. Their options are one set, `services.boomdrive`. Its `settings` is the runner's file, each key named as the file names it (core.12, core.17).
- **The file in the store.** The module renders the file in Nix, and the service runs `boomdrive run --config <the file>`, so a changed file or package changes the service and restarts it. The same file is linked at the runner's own path, so a command run by hand reads what the service reads. It is the host's level of record 0009. A key naming a credential fails evaluation, since the store is readable by every user (core.4).
- **The host keeps its own.** Its accounts, which hosts run the runner, its repositories, its partitions and control plane, whether it trusts the cache, its git and herdr, its secrets and its enrollments stay the host's configuration. Enrollment is `boomdrive enroll <label>` by hand, its token on standard input and its key in the host's credential store (core.32).

Option 2 needs no compiler on a host, and is what a host's configuration does for Flume and sluice today. But it re-packages every tool here, patching sluice's binaries on each platform, it has no `x86_64-darwin` Flume, and no release it would fetch carries an attestation. Option 3 has every host compile three Rust tools at each bump, which the user's ruling rules out: "so that people don't have to rebuild them". It stays what a host that does not trust the cache gets.

Fixtures alone prove the runner reads what a version printed once. The set's binaries in the gate prove the versions the flake names. A host running it proves nothing before it lands.

home-manager's options would bring home-manager into every host that imports the modules, where nix-darwin and NixOS each run a user's service themselves. A real file written at activation needs a writer and a restart of the module's own on each platform, and a store link holds nothing secret, since no key may name one.

### Consequences

- core.7 is amended: the flake is a moon project of its own, as each crate is, and it owns its workflow. core.44, core.45, core.46 and core.47 are added. Each is in this record's commit.
- The cache is `boomdrive`, signed by `boomdrive.cachix.org-1:vHLgmasPg2jKqkvPXWOOrYzspoGL2xxsW4fAuHBtiag=`. Before the change lands, the user makes the environment `cachix`, stores the cache's token in 1Password, and makes a service account that reads its vault, its token the environment's secret `OP_SERVICE_ACCOUNT_TOKEN`, by the commands the change's design gives.
- Whoever holds the token can push paths every trusting host takes. It is a write token of the cache, read only through the service account by the workflow's job on `main` and tags `v*`.
- Flume is asked to publish a flake, and sluice and timber to carry each release's version in the tagged tree. Until then, this flake builds Flume by a recipe of its own and gives sluice its tag's version, and its workflow pushes all three tools' binaries.
- The family is asked to push each tool's own flake's packages to the same cache at its release tags, once its tagged tree carries its version. This flake then takes them as the tool's flake gives them, and its workflow pushes only the runner and what a tool did not.
- A host's configuration imports the modules in place of its own, trusts the cache in its own nix settings, and drops its own package of the runner.

Rules: core.44, core.45, core.46 and core.47; core.7 and core.37 amended.
