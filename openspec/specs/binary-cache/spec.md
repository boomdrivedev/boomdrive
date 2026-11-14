# binary-cache Specification

## Purpose
The family's binary cache, `boomdrive` on Cachix, through which the set reaches a host as binaries. It covers the flake's `nixConfig`, the workflow that builds every package of every system on GitHub's hosted runners and pushes it at each push to `main` and each tag, the push's token, loaded from 1Password, and what a host that trusts the cache, or does not, gets (core.47; record 0020).

## Requirements

### Requirement: The flake names the family's cache

`flake.nix`'s `nixConfig` SHALL name, as its one extra substituter, `https://boomdrive.cachix.org`, and, as its one extra trusted public key, `boomdrive.cachix.org-1:vHLgmasPg2jKqkvPXWOOrYzspoGL2xxsW4fAuHBtiag=`. The README's install with nix SHALL give a host's configuration the same substituter and key, for its own nix settings.

#### Scenario: The flake's nixConfig

- **WHEN** `flake.nix` is read
- **THEN** its `nixConfig` names that substituter and that key, and no other

#### Scenario: The README's lines

- **WHEN** the README's install with nix is read
- **THEN** it names the substituter and the key `nixConfig` names

### Requirement: The workflow builds and pushes every package of every system

The workflow `cache` SHALL run on a push to `main`, on a push of a tag `v*`, and by hand, and on no other push. For each system the flake gives, a job on GitHub's hosted runner of that system SHALL build every package the flake gives for it, the runner's among them, and push their store paths and runtime closures to the cache. A job whose build fails SHALL push nothing. Its one other job SHALL be the boot, `boot (aarch64-linux)`, which pushes nothing (booted-system).

#### Scenario: Each system has its job

- **WHEN** the workflow is read beside the flake's packages
- **THEN** its jobs are `build` and `boot` alone, and `build` has one job for each system the flake gives, `x86_64-linux` on `ubuntu-24.04`, `aarch64-linux` on `ubuntu-24.04-arm`, and `aarch64-darwin` on `macos-15`, and none for another

#### Scenario: The runner is pushed with its tools

- **WHEN** a commit lands on `main`
- **THEN** each system's job builds `boomdrive`, `boomdrive-launcher`, `boomdrive-claude-plugins`, `flume`, `sluice`, `crew`, `herdr`, `ghostty`, `portless`, `worktrunk`, `pendingyou`, `plannotator`, `plannotator-tui` and the four herdr plugins of that commit, and pushes them to the cache

#### Scenario: A package that does not build

- **WHEN** one package of a system fails to build
- **THEN** that system's job fails and pushes nothing, and the other systems' jobs push theirs

#### Scenario: A branch's push

- **WHEN** a branch other than `main` is pushed
- **THEN** the workflow does not run

### Requirement: The push's token is the workflow's alone

The workflow SHALL read the cache's auth token only from 1Password, through a service account whose token is the secret `OP_SERVICE_ACCOUNT_TOKEN` of the repository's environment `cachix`, which admits `main` and tags `v*` alone. Each system's build job SHALL fail before it builds, naming what is missing, where either token is empty. No other job SHALL read either token. No file of the tree, option, store path or log SHALL hold either token.

#### Scenario: No service account token

- **WHEN** the workflow runs and the environment gives no `OP_SERVICE_ACCOUNT_TOKEN`
- **THEN** each system's build job fails before it builds, naming `OP_SERVICE_ACCOUNT_TOKEN`, and the boot, which reads no token, runs as it would

#### Scenario: No token

- **WHEN** the workflow runs and the `CACHIX_AUTH_TOKEN` it loads from 1Password is empty
- **THEN** each system's build job fails before it builds, naming `CACHIX_AUTH_TOKEN`, and the boot, which reads no token, runs as it would

### Requirement: A host substitutes the set, or builds the same paths

The modules' default packages SHALL be the flake's own, made with its locked nixpkgs whatever the host's `pkgs`, so they are the store paths the cache holds: the runner's, each tool's module's, and those of the modules' own options, the herdr plugins among them. A host whose nix trusts the cache SHALL substitute them, and one that does not SHALL build the same paths from the source the lock names. The modules SHALL add no substituter or key to a host's nix settings.

#### Scenario: A host's own nixpkgs

- **WHEN** a module is enabled, with crew turned on, in a system whose `pkgs` carries an overlay of the host's own
- **THEN** the system profile holds the flake's own `boomdrive`, `flume`, `sluice`, `crew`, `herdr`, `ghostty`, `portless`, `worktrunk`, `boomdrive-launcher`, `pendingyou`, `plannotator` and `plannotator-tui`, the store paths the flake gives, and the plugins it links are the flake's own `pendingyou-herdr-plugin` and `plannotator-herdr-plugin` beside Flume's and sluice's

#### Scenario: The host's trust

- **WHEN** a module is enabled
- **THEN** the system's nix substituters and trusted public keys name nothing of the cache's

### Requirement: A host's launcher is the flake's own by its override

The launcher the modules make for a host's drives SHALL be the flake's own `boomdrive-launcher` by its override, made with the flake's locked nixpkgs whatever the host's `pkgs`, so its compiled menu is the flake's own launcher's path, which the cache holds.

#### Scenario: A host's drives in the launcher

- **WHEN** a module is enabled, with crew turned on, in a system whose `pkgs` carries an overlay of the host's own, and the system has the drive `home`
- **THEN** the installed launcher is the flake's own `boomdrive-launcher` made for `home` with the session `home`, and on nix-darwin its compiled menu is the flake's own launcher's

### Requirement: The x86_64-linux job has room for the set

Before Nix is installed, the workflow's `x86_64-linux` job SHALL remove from its hosted runner the preinstalled trees that no package's build uses and dpkg does not own, the Android SDK, .NET, GHC, Swift, Rust's own and the tool cache among them, so the store and the builds have room for the set, and SHALL say the free space of `/` before and after. No other system's job SHALL remove anything.

#### Scenario: The x86_64-linux job has room

- **WHEN** the workflow is read
- **THEN** its first step, run only where the system is `x86_64-linux`, removes `/usr/local/lib/android`, `/usr/share/dotnet`, `/opt/ghc`, `/usr/local/.ghcup`, `/usr/share/swift`, `~/.cargo`, `~/.rustup` and `/opt/hostedtoolcache` and says the free space of `/` before and after, before Nix is installed, and no other step has a condition
