# Way in

## MODIFIED Requirements

### Requirement: The flake gives a shell that starts herdr

The flake SHALL give `devShells.<system>.default` for each of `aarch64-darwin`, `aarch64-linux` and `x86_64-linux`. Its `PATH` SHALL hold the set's `boomdrive`, `flume`, `sluice`, `crew`, `herdr` and `worktrunk`. Entering it interactively SHALL start herdr once, as herdr's own start with no argument, and leave the person in the shell when herdr ends. A command run through the shell SHALL start no herdr, and neither SHALL a shell entered from inside herdr. The shell SHALL link no plugin and write no file.

#### Scenario: Every system's shell

- **WHEN** the flake's `devShells` are evaluated for each of the three systems
- **THEN** each gives `default`, a derivation whose packages are the set's `boomdrive`, `flume`, `sluice`, `crew`, `herdr` and `worktrunk`

#### Scenario: Entering the shell

- **WHEN** the shell's hook runs in an interactive shell, outside herdr
- **THEN** herdr is run once, with no argument, and the shell goes on once herdr exits

#### Scenario: A command through the shell

- **WHEN** the shell's hook runs in a shell that is not interactive, as `nix develop --command` runs one
- **THEN** no herdr is run

#### Scenario: Inside herdr already

- **WHEN** the shell's hook runs in an interactive shell whose `HERDR_ENV` is `1`
- **THEN** no herdr is run
