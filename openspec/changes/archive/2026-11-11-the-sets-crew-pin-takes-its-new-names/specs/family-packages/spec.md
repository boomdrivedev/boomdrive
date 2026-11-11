## MODIFIED Requirements

### Requirement: Each package is made from this tree, its tool's own flake at its release tag, or the flake's nixpkgs

Each package SHALL be a derivation of the source the lock names by its hash: the runner's and the launcher's of this tree; Flume's, sluice's, crew's and herdr's of the tool's release tag, each through its own flake, with no recipe of this flake's; and Ghostty's of the flake's own nixpkgs. Each tool's version but Ghostty's SHALL be its tag's without the `v`, read from its tagged tree where it carries it, and from the flake's tag where it does not.

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

#### Scenario: The runner from this tree

- **WHEN** the flake is evaluated
- **THEN** the `boomdrive` package's source holds this tree's `Cargo.toml`, `Cargo.lock` and `crates`, and no other path of the tree
