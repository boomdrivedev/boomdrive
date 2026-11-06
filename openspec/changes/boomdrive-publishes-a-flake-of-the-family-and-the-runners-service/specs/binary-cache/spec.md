# Binary cache

## Purpose

The family's binary cache, `boomdrive` on Cachix, through which the set reaches a host as binaries. It covers the flake's `nixConfig`, the workflow that builds every package of every system on GitHub's hosted runners and pushes it at each push to `main` and each tag, the push's token, and what a host that trusts the cache, or does not, gets (core.47; record 0020).

## ADDED Requirements

### Requirement: The flake names the family's cache

`flake.nix`'s `nixConfig` SHALL name, as its one extra substituter, `https://boomdrive.cachix.org`, and, as its one extra trusted public key, `boomdrive.cachix.org-1:vHLgmasPg2jKqkvPXWOOrYzspoGL2xxsW4fAuHBtiag=`. The README's install with nix SHALL give a host's configuration the same substituter and key, for its own nix settings.

#### Scenario: The flake's nixConfig

- **WHEN** `flake.nix` is read
- **THEN** its `nixConfig` names that substituter and that key, and no other

#### Scenario: The README's lines

- **WHEN** the README's install with nix is read
- **THEN** it names the substituter and the key `nixConfig` names

### Requirement: The workflow builds and pushes every package of every system

The workflow `cache` SHALL run on a push to `main`, on a push of a tag `v*`, and by hand, and on no other push. For each system the flake gives, a job on GitHub's hosted runner of that system SHALL build every package the flake gives for it, the runner's among them, and push their store paths and runtime closures to the cache. A job whose build fails SHALL push nothing.

#### Scenario: Each system has its job

- **WHEN** the workflow is read beside the flake's packages
- **THEN** it has one job for each system the flake gives, `x86_64-linux` on `ubuntu-24.04`, `aarch64-linux` on `ubuntu-24.04-arm`, `aarch64-darwin` on `macos-15` and `x86_64-darwin` on `macos-15-intel`, and none for another

#### Scenario: The runner is pushed with its tools

- **WHEN** a commit lands on `main`
- **THEN** each system's job builds `boomdrive`, `flume`, `sluice` and both herdr plugins of that commit, and pushes them to the cache

#### Scenario: A package that does not build

- **WHEN** one package of a system fails to build
- **THEN** that system's job fails and pushes nothing, and the other systems' jobs push theirs

#### Scenario: A branch's push

- **WHEN** a branch other than `main` is pushed
- **THEN** the workflow does not run

### Requirement: The push's token is the workflow's alone

The workflow SHALL read the cache's auth token only from the secret `CACHIX_AUTH_TOKEN` of the repository's environment `cachix`, which admits `main` and tags `v*` alone. A job SHALL fail before it builds, naming the secret, where the token is empty. No file of the tree, option, store path or log SHALL hold the token.

#### Scenario: No token

- **WHEN** the workflow runs and `CACHIX_AUTH_TOKEN` is not set
- **THEN** each job fails before it builds, naming `CACHIX_AUTH_TOKEN`

### Requirement: A host substitutes the set, or builds the same paths

The modules' default packages SHALL be the flake's own, made with its locked nixpkgs whatever the host's `pkgs`, so they are the store paths the cache holds. A host whose nix trusts the cache SHALL substitute them, and one that does not SHALL build the same paths from the source the lock names. The modules SHALL add no substituter or key to a host's nix settings.

#### Scenario: A host's own nixpkgs

- **WHEN** a module is enabled in a system whose `pkgs` carries an overlay of the host's own
- **THEN** the system profile holds the flake's own `boomdrive`, `flume` and `sluice`, the store paths the flake gives

#### Scenario: The host's trust

- **WHEN** a module is enabled
- **THEN** the system's nix substituters and trusted public keys name nothing of the cache's
