## MODIFIED Requirements

### Requirement: The flake gives the family's host tools as packages

The flake SHALL give, for each of its three systems, `boomdrive`, `boomdrive-claude-plugins`, `boomdrive-launcher`, `crew`, `flume`, `flume-herdr-plugin`, `ghostty`, `herdr`, `herdr-plugins`, `pendingyou`, `pendingyou-herdr-plugin`, `plannotator`, `plannotator-herdr-plugin`, `plannotator-tui`, `portless`, `rex`, `sluice`, `sluice-herdr-plugin`, `timber`, `timber-herdr-plugin` and `worktrunk`, `default` the runner's. It SHALL give no git, the host's own, and no tool a repository pins.

#### Scenario: Every system's packages

- **WHEN** the flake's outputs are evaluated for each of the three systems
- **THEN** each gives the twenty-one packages and `default`, each a derivation, and `default` is `boomdrive`, and the flake gives no other system

#### Scenario: No tool of the host's own

- **WHEN** the flake's packages are listed for any system
- **THEN** none is git

### Requirement: Each package is made from this tree, its tool's own flake or source at its release tag, or the flake's nixpkgs

Each package but crew's companions' and Rex's SHALL be a derivation of the source the lock names by its hash: the runner's and the launcher's of this tree; Flume's, sluice's, crew's, herdr's, worktrunk's and timber's of their release tags, each through its own flake; portless's of its release tag, by this flake's recipe; and Ghostty's of the flake's nixpkgs. Each tool's version but Ghostty's SHALL be its tag's without the `v`, read from its tagged tree.

#### Scenario: Flume through its own flake

- **WHEN** the lock names a tag of Flume's whose tree carries its version
- **THEN** the `flume` package is Flume's flake's own package for that system, unchanged, and its version is the tag's without the `v`

#### Scenario: sluice through its own flake

- **WHEN** the lock names a tag of sluice's whose tree carries its version
- **THEN** the `sluice` package is sluice's flake's own `default` for that system, unchanged, and its version is the tag's without the `v`

#### Scenario: crew through its own flake

- **WHEN** the lock names a tag of crew's whose tree carries its version
- **THEN** the `crew` package is crew's flake's own `crew` for that system, unchanged, and its version is the tag's without the `v`

#### Scenario: herdr through its own flake

- **WHEN** the lock names herdr's tag `v0.9.5`
- **THEN** the `herdr` package is herdr's flake's own `herdr` for that system, unchanged, and its version is `0.9.5`

#### Scenario: worktrunk through its own flake

- **WHEN** the lock names worktrunk's tag `v0.80.0`
- **THEN** the `worktrunk` package is worktrunk's flake's own `worktrunk` for that system, its source and dependencies unchanged, its program `wt` and its version `0.80.0`, and its build is given `v0.80.0` as its `git describe`, as worktrunk's release build gets it

#### Scenario: timber through its own flake

- **WHEN** the lock names timber's tag `v0.6.0`, whose tree carries its version
- **THEN** the `timber` package is timber's flake's own `default` for that system, unchanged, its program `timber` and its version `0.6.0`

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

### Requirement: Rex is Rex's flake's own, at the release Flume's tree locks

The `rex` package SHALL be Rex's flake's own `rex` for each system, unchanged, its version the release the URL `flake.nix` names for Rex ends in. That URL SHALL be the one Flume's `devenv.lock`, at the tag the lock names for Flume, holds for Rex as its `original`, and the lock SHALL lock it at the URL and hash that `devenv.lock` locks. The flake's test SHALL fail where one does not hold, naming both, and where Flume's `devenv.lock` locks no Rex.

#### Scenario: Rex through its own flake

- **WHEN** `flake.nix` names Rex by `https://pkgs.superlogical.com/rex/nix/preview/0.1.0-preview.14.tar.gz`
- **THEN** the `rex` package of each system is Rex's flake's own `rex` for that system, unchanged, its program `rex` and its version `0.1.0-preview.14`

#### Scenario: Rex other than its URL's release

- **WHEN** a system's Rex says `0.1.0-preview.15` and `flake.nix` names Rex by the URL of `0.1.0-preview.14`
- **THEN** the flake's test fails, naming `0.1.0-preview.15` and `0.1.0-preview.14`

#### Scenario: Rex is Flume's

- **WHEN** the flake's test reads Flume's `devenv.lock` at the tag the lock names for Flume
- **THEN** `flake.nix` names Rex by the URL it holds for Rex as its `original`, and the lock locks Rex at the URL and hash it locks

#### Scenario: Rex other than Flume's

- **WHEN** Flume's `devenv.lock` locks Rex's `0.1.0-preview.19`, and `flake.nix` and the lock name `0.1.0-preview.14`
- **THEN** the flake's test fails, each failure naming both releases' URLs

#### Scenario: Rex moved without the lock

- **WHEN** Flume's `devenv.lock` and `flake.nix` name Rex's `0.1.0-preview.19`, and the lock locks `0.1.0-preview.14`
- **THEN** the flake's test fails once, naming both releases' URLs
