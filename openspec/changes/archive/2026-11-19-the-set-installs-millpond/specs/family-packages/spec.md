## ADDED Requirements

### Requirement: millpond is millpond's own flake's, its git crates fetched by their hashes

The set's millpond SHALL be millpond's own flake's package at the release tag `flake.nix` names, its version the tag's, but for its git crates, which SHALL be fetched by the hashes `nix/packages/millpond-git-crates.nix` holds, each at the revision millpond's `Cargo.lock` names at that tag, so the set evaluates against no store (record 0033; core.44, core.45).

#### Scenario: millpond at its tag

- **WHEN** the flake's packages are evaluated for each system
- **THEN** `millpond` is built from millpond's flake's package, says the tag's version, and each git crate it fetches is held at its `Cargo.lock`'s revision

## MODIFIED Requirements

### Requirement: The flake gives the family's host tools as packages

The flake SHALL give, for each of its three systems, `boomdrive`, `boomdrive-claude-plugins`, `boomdrive-launcher`, `crew`, `flume`, `flume-herdr-plugin`, `ghostty`, `herdr`, `herdr-plugins`, `millpond`, `pendingyou`, `pendingyou-herdr-plugin`, `plannotator`, `plannotator-herdr-plugin`, `plannotator-tui`, `portless`, `rex`, `sluice`, `sluice-herdr-plugin`, `timber`, `timber-herdr-plugin` and `worktrunk`, `default` the runner's. It SHALL give no git, the host's own, and no tool a repository pins.

#### Scenario: Every system's packages

- **WHEN** the flake's outputs are evaluated for each of the three systems
- **THEN** each gives the twenty-two packages and `default`, each a derivation, and `default` is `boomdrive`, and the flake gives no other system

#### Scenario: No tool of the host's own

- **WHEN** the flake's packages are listed for any system
- **THEN** none is git
