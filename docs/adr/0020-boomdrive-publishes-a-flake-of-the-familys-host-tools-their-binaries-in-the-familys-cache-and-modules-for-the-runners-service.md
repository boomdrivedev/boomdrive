---
number: 20
title: Boomdrive publishes a flake of the family's host tools, their binaries in the family's cache, and modules for the runner's service
status: accepted
date: 2026-11-05
decision-makers:
- Chuck Swanberg
---

# Boomdrive publishes a flake of the family's host tools, their binaries in the family's cache, and modules for the runner's service

Status: accepted by the user on 2026-11-06, approving the change's design ("Approve: start its code"). Proposed on 2026-11-05 by the construct stage of the change `boomdrive-publishes-a-flake-of-the-family-and-the-runners-service`. That this repository publishes a flake, with a package set and service modules, that the flake distributes the runner's, Flume's and sluice's binaries through the user's Cachix cache, which cache that is, and where the workflow reads its token from, are the user's decisions, in the words below. How the set is made, proven and pushed, and what the modules write, are the stage's choices, made on the written rules. The set gives no `x86_64-darwin`, which nixpkgs 26.11 builds nothing for: the code stage proposed it, and the user accepted it on 2026-11-05, answering "Leave it out: three systems".

Amended in part by record 0023, accepted on 2026-11-06: the flake configures the whole host, the runner, Flume, sluice and crew each by its own module, re-exported from the tool's own flake; each repository is named once, under its owner with its stream; herdr and Ghostty are in the set, with a Boomdrive app and a shell; crew's partitions are written once, in crew's module.

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
- **Its binaries in the family's cache.** This repository's workflow builds every package for each system the flake gives, `aarch64-darwin`, `aarch64-linux` and `x86_64-linux`, on GitHub's hosted runners, and pushes them to `boomdrive` on Cachix, at each push to `main` and each tag `v*`. The runner's package is built and pushed with Flume's and sluice's. `main` holds only what the gate passed, so nothing is pushed that the gate did not pass. Cachix signs what is pushed with the cache's own key. The push's token is kept in 1Password, and the workflow loads it through a service account whose token is the one secret of an environment that admits `main` and tags `v*` alone.
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

## Amendment, 2026-11-06: the nix-darwin agent waits for the Nix store

Status: accepted on 2026-11-06. The fix `the-darwin-agent-waits-for-the-store` asks, in its brief, that the agent wait for the store; its code stage chose the form, on the written rules.

Deciders: the fix's brief, for the wait; the fix's code stage, for its form.
Sources: the fix's brief, 2026-11-06; nix-darwin's `modules/launchd/default.nix` and `modules/system/launchd.nix`, and nixpkgs' `nixos/lib/utils.nix`, each at this flake's lock; the change `the-darwin-agent-waits-for-the-store`, its design; rules core.13 and core.46.

### Context and problem statement

The module set the agent's `ProgramArguments` to the runner's store path, with `RunAtLoad` and `KeepAlive`. nix-darwin puts `/bin/wait4path /nix/store` in front of a job's program only where the job is declared with `command` or `script`. At boot, launchd can start the agent before macOS has mounted the Nix store's volume. The exec then fails, and launchd may never try it again, so the runner stays down until it is loaded by hand. A host's configuration cannot wrap the job from outside, since the module owns it.

### Decision outcome

- **The agent is declared with nix-darwin's `command`.** launchd runs `/bin/sh -c '/bin/wait4path /nix/store && exec <package>/bin/boomdrive run --config <file>'`. The shell's `exec` hands the job's process to the runner, so a stop's signal reaches the runner alone, and `AbandonProcessGroup` still leaves a `flume launch` in flight. The plist still names the package and the file, so nix-darwin reloads the agent when either changes and leaves it running when neither does.
- **Not taken: the same shell written into the module's own `ProgramArguments`.** It gives the same plist and the same signals, but copies nix-darwin's wrapper into the module, and the copy would not follow a change nix-darwin makes to how a job waits. The tests read the program launchd gets, so a lock that changes nix-darwin's wrapper fails the gate where it no longer waits for the store and then `exec`s.
- **The NixOS unit is unchanged.** NixOS mounts the store in its first stage, before systemd, which runs from the store, starts.

Nothing else of the decision above changes. core.46 is amended in this amendment's commit: the modules start the runner only once the host's Nix store is mounted.

## Amendment, 2026-11-13: the x86_64-linux job frees its runner's disk before Nix is installed

Status: accepted on 2026-11-13. The fix `main--cache-x86-build-has-room` asks, in its brief, that the `x86_64-linux` build have room, "free the hosted runner's disk before Nix builds (remove the preinstalled toolchains it doesn't use, by a pinned maintained action or a step of its own) or build it on a larger runner"; its code stage chose the form, on the written rules.

Deciders: the fix's brief, for the room; the fix's code stage, for its form.
Sources: the fix's brief, 2026-11-13; the workflow `cache`'s runs 43344112746, 43345882875, 43358584313, 43360246846 and 43361015862; wimpysworld/nothing-but-nix's README and `action.yml` at v12, `b2574ec`, and its test run 43234874879 on `ubuntu-24.04`; jlumbroso/free-disk-space's `action.yml`; the change `cache-x86-build-has-room`; rule core.47.

### Context and problem statement

From v0.8.4 to v0.9.2, every run of `cache` failed on `build (x86_64-linux)` alone, five to eight minutes into the build, with Nix's note that the failure may have been caused by lack of free disk space: the store's and the builds' file system, `/`, had no bytes left. The `aarch64-linux` and `aarch64-darwin` jobs built and pushed the same set each time, so a `x86_64-linux` host got nothing from the cache. An x64 `ubuntu-24.04` runner leaves about 88GiB of its `/` free, and the arm64 runner, whose image holds fewer toolchains, about 106GiB.

### Decision outcome

- **The `x86_64-linux` job's first step frees its disk.** Before Nix is installed, it removes the image's trees that no package's build uses and dpkg does not own: Rust's own, GHC, the tool cache, the Android SDK, .NET and Swift, the trees nothing-but-nix's Carve removes, about 27GiB, and the Azure modules, Miniconda, Gradle, Kotlin, sbt, Heroku and pipx. It says the free space of `/` before and after, so a run's log shows the room its build had. It is the job's one step with a condition, and the other systems' jobs remove nothing.
- **The store and the builds stay on `/`.** Nix is installed and configured as before, and the x64 job gets more of `/` than the arm64 job builds the set in.
- **Not taken: nothing-but-nix.** It pools the runner's free space into a compressed Btrfs `/nix` of up to about 129GiB on x64, the most room of any form. It needs Nix's `build-dir` moved onto that volume; its newest release, v12, was a week old and mends v11's breaking of `cachix/install-nix-action`; and it runs a purge as root in the background of the job that holds the cache's push token. Should a run's log show the step's room is not enough, it is the next form.
- **Not taken: a larger runner.** The binary-cache spec names `ubuntu-24.04` for `x86_64-linux`, and a larger runner is a paid runner the organisation configures.

Nothing else of the decision above changes. core.47's details name the step as not ruled out, in this amendment's commit.

## Amendment, 2026-11-14: the NixOS module is proven by booting aarch64-linux, emulated on GitHub's arm runner

Status: decided on 2026-11-14 by boomdrive-design, on the construct stage's question of the change `a-booted-nixos-system-proves-the-set`. Nothing in it is the user's alone: it costs time and no money, principle 15 and the binary-cache spec decide it, and each choice is cheap to change.

Deciders: boomdrive-design.
Sources: the construct stage's reading of 2026-11-14 in its place's stop-short; actions/runner-images#14062, "Please support KVM on ARM runners", closed by GitHub on 2026-08-20, saying the ARM64 hosted fleet gives no nested KVM device; nixpkgs at this flake's lock (`1d060086`), `nixos/lib/testing/run.nix`, whose `requiredFeatures.kvm` "Can be disabled to allow emulated execution"; WarpBuild's documentation, whose ARM64 runners give no `/dev/kvm`; boomdrivedev/blueprints `design/principles.md` 15; afterthought/swancloud record 0059, decision 3, the pin a box takes.

### Context and problem statement

The NixOS module runs on Willdan's Graviton boxes, which are `aarch64-linux`. The check that proves it boots a NixOS system with the module, and a box's pin moves only to a commit whose boot is green. GitHub's `ubuntu-24.04-arm` runner gives no KVM, the boxes themselves are virtual machines with none, and a boot on `x86_64-linux` proves a system no box runs.

### Decision outcome

- **The boot runs on `ubuntu-24.04-arm`, its guest emulated.** The NixOS test sets `requiredFeatures.kvm` false, so its driver boots the `aarch64-linux` system under QEMU's own emulation on the `aarch64` host. The guest is the boxes' system, built as they build it.
- **It is a job of this repository's workflow,** at each push to `main` and each tag `v*`, and its check on the commit is what a box's pin reads. Its waits allow for an emulated boot, which takes minutes where a KVM boot takes seconds.
- **Never on `x86_64-linux`.** A boot of another system proves nothing a box runs.
- **Not taken: a self-hosted runner on a bare-metal `aarch64-linux` machine with KVM,** such as an EC2 Graviton `.metal` or a dedicated Ampere server. It boots in seconds, but it bills by the hour, gives ops a machine to keep, and on a public repository must take only pushes to `main` and tags. Whether to pay for one is the user's, asked if the emulated boot proves too slow to keep.
- **Not taken: a hosted runner service with `aarch64` KVM.** None was found.

Nothing else of the decision above changes.
