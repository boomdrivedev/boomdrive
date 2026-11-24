## ADDED Requirements

### Requirement: Flume's resident runs with the runner

Where the modules install Flume's package with the runner, Flume's own module SHALL run `flume host`, Flume's resident, as the user's service: the launchd agent `flume-host` on nix-darwin and the user unit `flume-host` on NixOS. The Boomdrive module SHALL declare no service of Flume's of its own, and SHALL run no settle (core.13, core.46; `one-resident-per-tool`).

#### Scenario: Flume's resident on nix-darwin

- **WHEN** a nix-darwin system turns on `services.boomdrive` and sets no option of Flume's
- **THEN** its launchd user agent `flume-host` runs the set's Flume's `flume host`, and no job of the system runs `flume settle`

#### Scenario: Flume's resident on NixOS

- **WHEN** a NixOS system turns on `services.boomdrive` and sets no option of Flume's
- **THEN** its systemd user service `flume-host` runs the set's Flume's `flume host`, and no unit of the system runs `flume settle`
