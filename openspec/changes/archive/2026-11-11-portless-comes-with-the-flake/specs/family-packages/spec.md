## RENAMED Requirements

- FROM: `### Requirement: Each package is made from this tree, its tool's own flake at its release tag, or the flake's nixpkgs`
- TO: `### Requirement: Each package is made from this tree, its tool's own flake or source at its release tag, or the flake's nixpkgs`

## MODIFIED Requirements

### Requirement: The flake gives the family's host tools as packages

The flake SHALL give, for each of `aarch64-darwin`, `aarch64-linux` and `x86_64-linux`, the packages `boomdrive`, `boomdrive-launcher`, `crew`, `flume`, `flume-herdr-plugin`, `ghostty`, `herdr`, `portless`, `sluice` and `sluice-herdr-plugin`, with `default` the runner's. It SHALL give no package of git, which is the host's own, and none of a tool a repository pins.

#### Scenario: Every system's packages

- **WHEN** the flake's outputs are evaluated for each of the three systems
- **THEN** each gives the ten packages and `default`, each a derivation, and `default` is `boomdrive`, and the flake gives no other system

#### Scenario: No tool of the host's own

- **WHEN** the flake's packages are listed for any system
- **THEN** none is git

### Requirement: The flake and its lock name one set

Each tool of the set that comes through its own flake or its own source, Flume, sluice, crew, herdr and portless, SHALL be named in `flake.nix` by a release tag, and the lock SHALL lock that tag. A tag in `flake.nix` that the lock does not lock SHALL fail the flake's type check, naming the tool and both tags. A tool of the set named by anything but a release tag SHALL fail it, naming the tool and what names it.

#### Scenario: A tag moved without the lock

- **WHEN** `flake.nix` names sluice's tag `v0.68.0` and the lock locks `v0.67.0`
- **THEN** the flake's type check fails, naming sluice, `v0.68.0` and `v0.67.0`

#### Scenario: A tool named by no release tag

- **WHEN** `flake.nix` and the lock name crew by `main`
- **THEN** the flake's type check fails, naming crew and `main`

#### Scenario: herdr named by no release tag

- **WHEN** `flake.nix` and the lock name herdr by `main`
- **THEN** the flake's type check fails, naming herdr and `main`

#### Scenario: portless named by no release tag

- **WHEN** `flake.nix` and the lock name portless by `main`
- **THEN** the flake's type check fails, naming portless and `main`

### Requirement: Each package is made from this tree, its tool's own flake or source at its release tag, or the flake's nixpkgs

Each package SHALL be a derivation of the source the lock names by its hash: the runner's and the launcher's of this tree; Flume's, sluice's, crew's and herdr's of their release tags, each through its own flake; portless's of its release tag, by this flake's recipe, as it publishes no flake; and Ghostty's of the flake's nixpkgs. Each tool's version but Ghostty's SHALL be its tag's without the `v`, read from its tagged tree where it holds one, else from the flake's tag.

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

#### Scenario: portless from its source

- **WHEN** the lock names portless's tag `v0.17.0`
- **THEN** the `portless` package of each system is this flake's recipe's build of that tag's source, its program `portless`, and its version `0.17.0`, read from the tagged tree's `packages/portless/package.json`

#### Scenario: portless other than its tag

- **WHEN** portless's tagged tree says `0.17.1` and the lock names portless's tag `v0.17.0`
- **THEN** the flake's test fails, naming `0.17.1` and `0.17.0`

#### Scenario: The runner from this tree

- **WHEN** the flake is evaluated
- **THEN** the `boomdrive` package's source holds this tree's `Cargo.toml`, `Cargo.lock` and `crates`, and no other path of the tree

## ADDED Requirements

### Requirement: portless is built from its package alone and runs on the flake's Node.js

The `portless` package SHALL be built from its workspace's package `packages/portless` alone, with the flake's nixpkgs' pnpm 11. Its dependencies SHALL be one fetch of that package's alone, whose hash is the same on every system. Its program `portless` SHALL run that build on the flake's nixpkgs' Node.js 24. It SHALL find the flake's nixpkgs' openssl, procps, which and hostname after the host's own on its `PATH`. It SHALL bring no git, which is the host's own.

#### Scenario: One fetch for every system

- **WHEN** the flake's packages are evaluated
- **THEN** each system's `portless` takes its dependencies from a fetch of the workspace's `portless` package alone, under pnpm 11, and the fetch's hash is the same on every system

#### Scenario: The flake's Node.js

- **WHEN** the flake's packages are evaluated
- **THEN** each system's `portless` runs on the flake's nixpkgs' `nodejs_24` for that system

#### Scenario: portless runs the host's git

- **WHEN** the programs portless's `PATH` is given are listed, for any system
- **THEN** they are the flake's nixpkgs' `openssl`, `procps`, `which` and `hostname` for that system, and none is git
