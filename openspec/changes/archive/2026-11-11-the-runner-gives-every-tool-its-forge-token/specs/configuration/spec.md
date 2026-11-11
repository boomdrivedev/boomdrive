## MODIFIED Requirements

### Requirement: The configuration holds nothing the runner does not read

The configuration SHALL be refused, naming the key by its path, when it holds any key this spec does not name, at its top, in an account, in a drive or in `[forge]`. In particular, no key SHALL hold a token, key or credential. `[forge]`'s `key_file` names where the App's key is kept on the host, never the key. Where no `[forge]` is named, git's own credential helper on the host answers for every fetch (core.4, core.33).

#### Scenario: A misspelt key

- **WHEN** the configuration says `repositories_root = "~/Code"`
- **THEN** it is refused, naming `repositories_root` as a key the runner does not read

#### Scenario: A token

- **WHEN** the configuration says `token = "..."`
- **THEN** it is refused, naming `token` as a key the runner does not read, and the value appears in no output

#### Scenario: A key in a drive

- **WHEN** the account `own`'s drive `home` says `machine = "elsewhere"`
- **THEN** it is refused, naming `accounts.own.drives.home.machine` as a key the runner does not read

#### Scenario: A key in an account

- **WHEN** the account `own` says `state = "example/state"`
- **THEN** it is refused, naming `accounts.own.state` as a key the runner does not read

#### Scenario: A key's text in the forge table

- **WHEN** `[forge]` says `key = "..."` beside its `app_id` and `key_file`
- **THEN** it is refused, naming `forge.key` as a key the runner does not read, and the value appears in no output

## ADDED Requirements

### Requirement: The configuration may name the Boomdrive App

The configuration MAY hold a `[forge]` table naming the Boomdrive App whose key the runner holds: `app_id`, a positive integer, and `key_file`, the path of the App's private key. Both SHALL be required where the table is present. The key file SHALL not be read with the configuration: what stands at its path refuses a mint, never the configuration (record 0007 as amended on 2026-11-09 and 2026-11-11; core.12, core.33).

#### Scenario: The App named

- **WHEN** the configuration says `[forge]` with `app_id = 1` and `key_file = "~/.config/boomdrive/app.pem"`
- **THEN** the runner reads the App `1` with its key at `.config/boomdrive/app.pem` under the home, whether or not a file stands there

#### Scenario: No App named

- **WHEN** the configuration names no `[forge]`
- **THEN** the runner reads no App

#### Scenario: A key file missing from the table

- **WHEN** `[forge]` says `app_id = 1` alone
- **THEN** it is refused, naming `forge.key_file`

### Requirement: The App's key file is a path outside the store

`[forge]`'s `key_file` SHALL be absolute or begin with `~/`, and SHALL be refused, naming it, where it is neither or where it lies under the Nix store directory, which every user reads (core.4, core.33).

#### Scenario: A key file in the store

- **WHEN** `[forge]`'s `key_file` is `/nix/store/abc-app.pem`
- **THEN** it is refused, naming `forge.key_file` and the Nix store

#### Scenario: A relative key file

- **WHEN** `[forge]`'s `key_file` is `keys/app.pem`
- **THEN** it is refused, naming `forge.key_file` and saying it is absolute or begins with `~/`

### Requirement: The App is named only in the managed layer of a host that holds it

Where the configuration's control plane holds the App, as the hosted control plane does, `[forge]` SHALL be refused, naming it. The local layer SHALL not set `[forge]` (record 0007 as amended on 2026-11-09; core.17, core.33).

#### Scenario: The App beside a hosted control plane

- **WHEN** the configuration names `[forge]` and its `[control_plane]` says `type = "hosted"`
- **THEN** it is refused, naming `forge` and saying the hosted control plane holds the App

#### Scenario: The App in the local layer

- **WHEN** the local layer says `[forge]` with `app_id = 1`
- **THEN** the local layer is not taken, and the runner names `forge`
