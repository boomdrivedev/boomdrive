# Family packages

## RENAMED Requirements

- FROM: `### Requirement: Each package is made from the runner's tree or its tool's own flake at its release tag`
- TO: `### Requirement: Each package is made from this tree, its tool's own flake at its release tag, or the flake's nixpkgs`

## MODIFIED Requirements

### Requirement: The flake gives the family's host tools as packages

The flake SHALL give, for each of `aarch64-darwin`, `aarch64-linux` and `x86_64-linux`, the packages `boomdrive`, `boomdrive-launcher`, `crew`, `flume`, `flume-herdr-plugin`, `ghostty`, `herdr`, `sluice` and `sluice-herdr-plugin`, with `default` the runner's. It SHALL give no package of git, which is the host's own, and none of a tool a repository pins.

#### Scenario: Every system's packages

- **WHEN** the flake's outputs are evaluated for each of the three systems
- **THEN** each gives the nine packages and `default`, each a derivation, and `default` is `boomdrive`, and the flake gives no other system

#### Scenario: No tool of the host's own

- **WHEN** the flake's packages are listed for any system
- **THEN** none is git

### Requirement: The flake and its lock name one set

Each tool of the set that comes through its own flake, Flume, sluice, crew and herdr, SHALL be named in `flake.nix` by a release tag, and the lock SHALL lock that tag. A tag in `flake.nix` that the lock does not lock SHALL fail the flake's type check, naming the tool and both tags. A tool of the set named by anything but a release tag SHALL fail it, naming the tool and what names it.

#### Scenario: A tag moved without the lock

- **WHEN** `flake.nix` names sluice's tag `v0.68.0` and the lock locks `v0.67.0`
- **THEN** the flake's type check fails, naming sluice, `v0.68.0` and `v0.67.0`

#### Scenario: A tool named by no release tag

- **WHEN** `flake.nix` and the lock name crew by `main`
- **THEN** the flake's type check fails, naming crew and `main`

#### Scenario: herdr named by no release tag

- **WHEN** `flake.nix` and the lock name herdr by `main`
- **THEN** the flake's type check fails, naming herdr and `main`

### Requirement: Each package is made from this tree, its tool's own flake at its release tag, or the flake's nixpkgs

Each package SHALL be a derivation of the source the lock names by its hash: the runner's and the launcher's of this tree; Flume's, sluice's, crew's and herdr's of the tool's release tag, each through its own flake, with no recipe of this flake's; and Ghostty's of the flake's own nixpkgs. Each tool's version but Ghostty's SHALL be its tag's without the `v`, read from its tagged tree where it carries it, and from the flake's tag where it does not.

#### Scenario: Flume through its own flake

- **WHEN** the lock names a tag of Flume's whose tree carries its version
- **THEN** the `flume` package is Flume's flake's own package for that system, unchanged, and its version is the tag's without the `v`

#### Scenario: sluice through its own flake

- **WHEN** the lock names sluice's tag `v0.68.0`, whose tree says `0.0.0`
- **THEN** the `sluice` package is sluice's flake's, its version `0.68.0`, and its build is given `0.68.0` as sluice's release build is

#### Scenario: crew through its own flake

- **WHEN** the lock names a tag of crew's, whose tree carries no version
- **THEN** the `crew` package is crew's flake's, its source, name and home page crew's own, and its version is the tag's without the `v`

#### Scenario: herdr through its own flake

- **WHEN** the lock names herdr's tag `v0.9.5`
- **THEN** the `herdr` package is herdr's flake's own `herdr` for that system, unchanged, and its version is `0.9.5`

#### Scenario: The runner from this tree

- **WHEN** the flake is evaluated
- **THEN** the `boomdrive` package's source holds this tree's `Cargo.toml`, `Cargo.lock` and `crates`, and no other path of the tree

## ADDED Requirements

### Requirement: herdr is the release Flume's terminal profile was checked against

The `herdr` package's version SHALL be the herdr that Flume's tree pins at the tag the lock names for Flume: the release Flume's terminal profile was checked against, `herdrVersion` in Flume's `devenv.nix`. No herdr plugin of the set SHALL ask, by its manifest's `min_herdr_version`, for a herdr newer than the set's. The flake's test SHALL fail where either does not hold, naming both versions.

#### Scenario: herdr is Flume's

- **WHEN** the flake is evaluated
- **THEN** the set's herdr is the version Flume's `devenv.nix` pins at the tag the lock names for Flume, and neither of the set's herdr plugins asks for a newer one

#### Scenario: herdr other than Flume's

- **WHEN** Flume's tree pins herdr `0.9.6` and the lock names herdr's tag `v0.9.5`
- **THEN** the flake's test fails, naming `0.9.6` and `0.9.5`

#### Scenario: A plugin newer than herdr

- **WHEN** a herdr plugin of the set asks for herdr `0.9.9` and the set's herdr is `0.9.5`
- **THEN** the flake's test fails, naming the plugin, `0.9.9` and `0.9.5`

### Requirement: Ghostty is the release Flume's window profile was checked against

The `ghostty` package SHALL be the flake's nixpkgs' Ghostty: `ghostty`, built from source, on each Linux system, and `ghostty-bin`, Ghostty's own release, on `aarch64-darwin`, where nothing builds its macOS app from source. Its version SHALL be `1.4.0`, the release Flume's window profile was checked against (Flume's C.8), and the flake's test SHALL fail on another, naming both versions.

#### Scenario: Ghostty on each system

- **WHEN** the flake's packages are evaluated
- **THEN** `ghostty` is nixpkgs' `ghostty-bin` on `aarch64-darwin` and nixpkgs' `ghostty` on `aarch64-linux` and `x86_64-linux`, each at `1.4.0`

#### Scenario: Ghostty moved with nixpkgs

- **WHEN** a system's Ghostty is `1.5.0`
- **THEN** the flake's test fails, naming that system, `1.5.0` and `1.4.0`
