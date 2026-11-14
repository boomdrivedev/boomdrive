# Family packages

## MODIFIED Requirements

### Requirement: The flake gives the family's host tools as packages

The flake SHALL give, for each of its three systems, the packages `boomdrive`, `boomdrive-claude-plugins`, `boomdrive-launcher`, `crew`, `flume`, `flume-herdr-plugin`, `ghostty`, `herdr`, `herdr-plugins`, `pendingyou`, `pendingyou-herdr-plugin`, `plannotator`, `plannotator-herdr-plugin`, `plannotator-tui`, `portless`, `sluice`, `sluice-herdr-plugin`, `timber`, `timber-herdr-plugin` and `worktrunk`, `default` the runner's. It SHALL give no git, the host's own, and no tool a repository pins.

#### Scenario: Every system's packages

- **WHEN** the flake's outputs are evaluated for each of the three systems
- **THEN** each gives the twenty packages and `default`, each a derivation, and `default` is `boomdrive`, and the flake gives no other system

#### Scenario: No tool of the host's own

- **WHEN** the flake's packages are listed for any system
- **THEN** none is git

### Requirement: The flake and its lock name one set

Each tool of the set that comes through its own flake or its own source, Flume, sluice, crew, herdr, portless, worktrunk and timber, SHALL be named in `flake.nix` by a release tag, and the lock SHALL lock that tag. A tag in `flake.nix` that the lock does not lock SHALL fail the flake's type check, naming the tool and both tags. A tool of the set named by anything but a release tag SHALL fail it, naming the tool and what names it.

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

#### Scenario: worktrunk named by no release tag

- **WHEN** `flake.nix` and the lock name worktrunk by `main`
- **THEN** the flake's type check fails, naming worktrunk and `main`

#### Scenario: timber named by no release tag

- **WHEN** `flake.nix` and the lock name timber by `main`
- **THEN** the flake's type check fails, naming timber and `main`

### Requirement: A tool's herdr plugin is the tool's, at the tool's tag

Flume's, sluice's and timber's herdr plugin packages SHALL each be its tool's `plugins/herdr` folder, taken from the source of the tool's own package, as the tool ships it, with the tool's version.

#### Scenario: Flume's plugin moves with Flume

- **WHEN** the lock names Flume's tag `v0.10.0`
- **THEN** `flume-herdr-plugin` is that tag's `plugins/herdr`, its version `0.10.0`

#### Scenario: timber's plugin moves with timber

- **WHEN** the lock names timber's tag `v0.6.0`
- **THEN** `timber-herdr-plugin` is that tag's `plugins/herdr`, taken from the source of the set's `timber`, its version `0.6.0`, and nothing in it rewritten

### Requirement: Each package is made from this tree, its tool's own flake or source at its release tag, or the flake's nixpkgs

Each package but crew's companions' SHALL be a derivation of the source the lock names by its hash: the runner's and the launcher's of this tree; Flume's, sluice's, crew's, herdr's, worktrunk's and timber's of their release tags, each through its own flake; portless's of its release tag, by this flake's recipe; and Ghostty's of the flake's nixpkgs. Each tool's version but Ghostty's SHALL be its tag's without the `v`, read from its tagged tree where it holds one, else from the flake's tag.

#### Scenario: Flume through its own flake

- **WHEN** the lock names a tag of Flume's whose tree carries its version
- **THEN** the `flume` package is Flume's flake's own package for that system, unchanged, and its version is the tag's without the `v`

#### Scenario: sluice through its own flake

- **WHEN** the lock names sluice's tag `v0.68.0`, whose tree says `0.0.0`
- **THEN** the `sluice` package is sluice's flake's, its version `0.68.0`, and its build is given `0.68.0` as sluice's release build is

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

### Requirement: herdr is the release Flume's terminal profile was checked against

The `herdr` package's version SHALL be the herdr that Flume's tree pins at the tag the lock names for Flume: the release Flume's terminal profile was checked against, `herdrVersion` in Flume's `devenv.nix`. No herdr plugin of the set SHALL ask, by its manifest's `min_herdr_version`, for a herdr newer than the set's. The flake's test SHALL fail where either does not hold, naming both versions.

#### Scenario: herdr is Flume's

- **WHEN** the flake is evaluated
- **THEN** the set's herdr is the version Flume's `devenv.nix` pins at the tag the lock names for Flume, and none of Flume's, sluice's and timber's herdr plugins asks for a newer one

#### Scenario: herdr other than Flume's

- **WHEN** Flume's tree pins herdr `0.9.6` and the lock names herdr's tag `v0.9.5`
- **THEN** the flake's test fails, naming `0.9.6` and `0.9.5`

#### Scenario: A plugin newer than herdr

- **WHEN** a herdr plugin of the set asks for herdr `0.9.9` and the set's herdr is `0.9.5`
- **THEN** the flake's test fails, naming the plugin, `0.9.9` and `0.9.5`

## REMOVED Requirements

### Requirement: The set's marketplace lists Pending You's and Plannotator's plugins

**Reason**: The set's marketplace lists timber's Claude Code plugin beside crew's companions' (record 0044 as amended on 2026-11-13). "The set's marketplace lists Pending You's, Plannotator's and timber's plugins" holds what stands of it, its scenario "No link into the store" among it.

**Migration**: None: no module installs the marketplace into a Claude Code home yet, and a person who added it by hand finds a third plugin there at its next `claude plugin marketplace update`.

## ADDED Requirements

### Requirement: The set's marketplace lists Pending You's, Plannotator's and timber's plugins

`boomdrive-claude-plugins` SHALL be one folder, a Claude Code marketplace named `boomdrive` whose `.claude-plugin/marketplace.json` lists three plugins, `pendingyou` at the set's Pending You's version, `plannotator` at the set's Plannotator's and `timber` at the set's timber's, each at `./plugins/<name>`. No file of the folder SHALL be a link.

#### Scenario: The marketplace's three plugins

- **WHEN** the flake's packages are evaluated
- **THEN** `boomdrive-claude-plugins` gives the marketplace `boomdrive` with `pendingyou` at `0.41.0`, `plannotator` at `0.28.61` and `timber` at `0.6.0`

#### Scenario: No link into the store

- **WHEN** the marketplace's build is read
- **THEN** it copies every file with links followed, and fails where any file of its output is a link

### Requirement: timber's plugins say the set's timber's version

The version of `timber-herdr-plugin`'s `herdr-plugin.toml` and of the marketplace's `timber` plugin's `.claude-plugin/plugin.json` SHALL each be the set's `timber`'s. The flake's test SHALL fail where one is not, naming the plugin and both versions.

#### Scenario: timber's plugins say the set's timber

- **WHEN** the flake is evaluated
- **THEN** each system's `timber-herdr-plugin` manifest and the marketplace's `timber` plugin manifest say the version of that system's `timber`

#### Scenario: A timber plugin other than the set's timber

- **WHEN** timber's herdr plugin says `0.5.3` and the set's timber is `0.6.0`
- **THEN** the flake's test fails, naming the plugin, `0.5.3` and `0.6.0`

### Requirement: timber's Claude Code plugin is its command's source's

The marketplace's `timber` plugin SHALL be the `plugins/claude-code` folder of the source the set's `timber` is built from, copied whole.

#### Scenario: timber's Claude Code plugin from its command's source

- **WHEN** the flake's packages are evaluated
- **THEN** the `timber` plugin is copied whole from the `plugins/claude-code` of the source the set's `timber` is built from
