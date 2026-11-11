## ADDED Requirements

### Requirement: The App's key is named by a path outside the store

`services.boomdrive.settings.forge` SHALL be declared, unset by default: its `app_id` a positive integer and its `key_file` a string, so a Nix path value fails the option's type and a key is never copied into the store. Evaluation SHALL fail, naming `services.boomdrive.settings.forge.key_file` and the Nix store, where `key_file` begins with the store directory. Set, it SHALL be rendered as the file's `[forge]`, which the runner reads (record 0007 as amended on 2026-11-11; core.46).

#### Scenario: The App named

- **WHEN** `settings.forge` is `{ app_id = 1; key_file = "~/.config/boomdrive/app.pem"; }`
- **THEN** the file holds `[forge]` with `app_id = 1` and that `key_file`, and the runner reads the App `1` from it

#### Scenario: A key file in the store

- **WHEN** `settings.forge.key_file` is `"/nix/store/abc-app.pem"`
- **THEN** evaluation fails, naming `services.boomdrive.settings.forge.key_file` and the Nix store

#### Scenario: A key given as a Nix path

- **WHEN** `settings.forge.key_file` is a Nix path value
- **THEN** evaluation fails on the option's type
