# Service modules

## MODIFIED Requirements

### Requirement: The file is at the runner's own path for commands run by hand

Each module's activation SHALL link the service's user's `~/.config/boomdrive/config.toml` to the module's file, so `boomdrive enroll` and `boomdrive run --once` run by hand read what the service reads. A link there SHALL be replaced. A file or folder there that is no link SHALL be moved aside once, to `config.toml.before-nix`, and named in the activation's output. The activation SHALL leave `~/.config/boomdrive/config.local.toml`, the runner's local layer, as it is, since the operator or a control plane writes it (record 0009 as amended 2026-11-06; core.18).

#### Scenario: An empty home

- **WHEN** the activation runs for a home with no `.config/boomdrive`
- **THEN** `~/.config/boomdrive/config.toml` is a link to the module's file

#### Scenario: A person's own file

- **WHEN** the activation finds a file at `~/.config/boomdrive/config.toml` that is no link
- **THEN** it moves it to `config.toml.before-nix`, links the module's file, and names both paths

#### Scenario: A file moved aside before

- **WHEN** the activation finds a file at `~/.config/boomdrive/config.toml` that is no link, and one at `config.toml.before-nix`
- **THEN** it changes neither, names both paths, and fails the link alone

#### Scenario: Already linked

- **WHEN** the activation finds the link to the module's file
- **THEN** it changes nothing and says nothing

#### Scenario: A local layer beside it

- **WHEN** the activation runs for a home whose `.config/boomdrive` holds a `config.local.toml`
- **THEN** it links `config.toml` to the module's file, and `config.local.toml` is there as it was, its text unchanged
