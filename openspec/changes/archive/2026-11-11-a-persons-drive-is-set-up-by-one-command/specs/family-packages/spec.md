## MODIFIED Requirements

### Requirement: The flake gives the family's host tools as packages

The flake SHALL give, for each of `aarch64-darwin`, `aarch64-linux` and `x86_64-linux`, the packages `boomdrive`, `boomdrive-launcher`, `crew`, `flume`, `flume-herdr-plugin`, `ghostty`, `herdr`, `herdr-plugins`, `sluice` and `sluice-herdr-plugin`, with `default` the runner's. It SHALL give no package of git, which is the host's own, and none of a tool a repository pins.

#### Scenario: Every system's packages

- **WHEN** the flake's outputs are evaluated for each of the three systems
- **THEN** each gives the ten packages and `default`, each a derivation, and `default` is `boomdrive`, and the flake gives no other system

#### Scenario: No tool of the host's own

- **WHEN** the flake's packages are listed for any system
- **THEN** none is git

## ADDED Requirements

### Requirement: The set's herdr plugins are one package a profile holds

The `herdr-plugins` package SHALL hold each of the set's herdr plugins at `share/herdr/plugins/<tool>`: `flume`, the `flume-herdr-plugin` package, and `sluice`, the `sluice-herdr-plugin` package, and nothing else, so one Nix profile holds both beside the set's other packages. Setup links each from the profile with herdr's own command (the setup spec; record 0009 as amended 2026-11-09).

#### Scenario: The plugins in one package

- **WHEN** the flake's `herdr-plugins` is evaluated for each of the three systems
- **THEN** it holds `share/herdr/plugins/flume`, the system's `flume-herdr-plugin`, and `share/herdr/plugins/sluice`, the system's `sluice-herdr-plugin`, and no `bin`
