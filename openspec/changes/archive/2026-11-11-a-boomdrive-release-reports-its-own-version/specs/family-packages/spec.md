## MODIFIED Requirements

### Requirement: Each package is made from its release tag's source

Each package SHALL be a derivation of the source the lock names by its hash: the runner's of this tree's crates, `Cargo.toml` and `Cargo.lock`, and Flume's and sluice's of the tool's release tag, sluice's through sluice's own flake. Each tool's version SHALL be its tag's without the `v`, read from its tagged tree where the tree carries it, and from the flake's tag where it does not. The runner's SHALL be its tree's, `Cargo.toml`'s workspace version.

#### Scenario: Flume at its tag

- **WHEN** the lock names Flume's tag `v0.10.0`
- **THEN** the `flume` package's source is that tag's tree and its version is `0.10.0`

#### Scenario: sluice through its own flake

- **WHEN** the lock names sluice's tag `v0.68.0`, whose tree says `0.0.0`
- **THEN** the `sluice` package is sluice's flake's, its version `0.68.0`, and its build is given `0.68.0` as sluice's release build is

#### Scenario: The runner from this tree

- **WHEN** the flake is evaluated
- **THEN** the `boomdrive` package's source holds this tree's `Cargo.toml`, `Cargo.lock` and `crates`, and no other path of the tree

#### Scenario: The runner at its tree's version

- **WHEN** the flake is evaluated for each system it gives
- **THEN** the `boomdrive` package's version is the one `Cargo.toml` gives under `[workspace.package]`: `0.0.0` at a commit of `main`, and the release's at a release tag

#### Scenario: An overridden input

- **WHEN** the flake's outputs are evaluated with Flume's input replaced by a tree whose `Cargo.toml` says `9.9.9`
- **THEN** the `flume` package is made from that tree, its version `9.9.9`
