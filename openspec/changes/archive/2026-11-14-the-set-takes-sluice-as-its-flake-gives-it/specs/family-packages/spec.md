## MODIFIED Requirements

### Requirement: Each package is made from this tree, its tool's own flake or source at its release tag, or the flake's nixpkgs

Each package but crew's companions' SHALL be a derivation of the source the lock names by its hash: the runner's and the launcher's of this tree; Flume's, sluice's, crew's, herdr's, worktrunk's and timber's of their release tags, each through its own flake; portless's of its release tag, by this flake's recipe; and Ghostty's of the flake's nixpkgs. Each tool's version but Ghostty's SHALL be its tag's without the `v`, read from its tagged tree.

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
