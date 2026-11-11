# Family packages

## ADDED Requirements

### Requirement: Each package is made from the runner's tree or its tool's own flake at its release tag

Each package SHALL be a derivation of the source the lock names by its hash: the runner's of this tree's crates, `Cargo.toml` and `Cargo.lock`, and Flume's, sluice's and crew's of the tool's release tag, each through the tool's own flake, with no recipe of this flake's for any of them. Each package's version SHALL be its tag's without the `v`, read from its tagged tree where the tree carries it, and from the flake's tag where it does not.

#### Scenario: Flume through its own flake

- **WHEN** the lock names a tag of Flume's whose tree carries its version
- **THEN** the `flume` package is Flume's flake's own package for that system, unchanged, and its version is the tag's without the `v`

#### Scenario: sluice through its own flake

- **WHEN** the lock names sluice's tag `v0.68.0`, whose tree says `0.0.0`
- **THEN** the `sluice` package is sluice's flake's, its version `0.68.0`, and its build is given `0.68.0` as sluice's release build is

#### Scenario: crew through its own flake

- **WHEN** the lock names a tag of crew's, whose tree carries no version
- **THEN** the `crew` package is crew's flake's, its source, name and home page crew's own, and its version is the tag's without the `v`

#### Scenario: The runner from this tree

- **WHEN** the flake is evaluated
- **THEN** the `boomdrive` package's source holds this tree's `Cargo.toml`, `Cargo.lock` and `crates`, and no other path of the tree

## REMOVED Requirements

### Requirement: Each package is made from its release tag's source

**Reason**: Flume's package is Flume's flake's own, so no recipe of this flake's builds Flume from a tree it is given (core.44), and crew joins the set through its own flake. "Each package is made from the runner's tree or its tool's own flake at its release tag" holds what stands of it.

**Migration**: A host that replaced Flume's input by a tree replaces it by a flake of Flume's, whose package the set then takes, or gives its own Flume through `services.flume.package`.

## MODIFIED Requirements

### Requirement: The flake gives the family's host tools as packages

The flake SHALL give, for each of `aarch64-darwin`, `aarch64-linux` and `x86_64-linux`, the packages `boomdrive`, `flume`, `sluice`, `crew`, `flume-herdr-plugin` and `sluice-herdr-plugin`, with `default` the runner's. It SHALL give no package of herdr or of git, which are the host's own, and none of a tool a repository pins.

#### Scenario: Every system's packages

- **WHEN** the flake's outputs are evaluated for each of the three systems
- **THEN** each gives the six packages and `default`, each a derivation, and `default` is `boomdrive`, and the flake gives no other system

#### Scenario: No tool of the host's own

- **WHEN** the flake's packages are listed for any system
- **THEN** none is herdr or git

### Requirement: The flake and its lock name one set

Each tool of the set but the runner, Flume, sluice and crew, SHALL be named in `flake.nix` by a release tag, and the lock SHALL lock that tag. A tag in `flake.nix` that the lock does not lock SHALL fail the flake's type check, naming the tool and both tags. A tool of the set named by anything but a release tag SHALL fail it, naming the tool and what names it.

#### Scenario: A tag moved without the lock

- **WHEN** `flake.nix` names sluice's tag `v0.68.0` and the lock locks `v0.67.0`
- **THEN** the flake's type check fails, naming sluice, `v0.68.0` and `v0.67.0`

#### Scenario: A tool named by no release tag

- **WHEN** `flake.nix` and the lock name crew by `main`
- **THEN** the flake's type check fails, naming crew and `main`
