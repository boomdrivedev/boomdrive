## MODIFIED Requirements

### Requirement: The modules install herdr, Ghostty and the launcher, each replaceable or dropped

With `services.boomdrive.enable`, the modules SHALL put `services.boomdrive.herdr.package`, `services.boomdrive.ghostty.package` and `services.boomdrive.launcher.package` in the system profile. herdr and Ghostty SHALL each be the set's for the host's system by default. The launcher SHALL be the set's for the host's system made for each drive of this host by default, which is the set's own `boomdrive-launcher` where this host has no drive. A host's own package SHALL replace each, installed as given, and `null` SHALL install none. With the runner not enabled, the modules SHALL install none of them.

#### Scenario: The way in with the runner

- **WHEN** a system turns on `services.boomdrive`, names none of the three and has no drive
- **THEN** the system profile holds the set's `herdr`, `ghostty` and `boomdrive-launcher`, on nix-darwin and on NixOS

#### Scenario: A host's own herdr

- **WHEN** `services.boomdrive.herdr.package` is a package of the host's own
- **THEN** the system profile holds it and not the set's herdr, and the activation links the plugins with it, on nix-darwin and on NixOS

#### Scenario: No Ghostty of the set's

- **WHEN** `services.boomdrive.ghostty.package` is `null`
- **THEN** the system profile holds no Ghostty of the set's, and `services.flume.settings.host.terminal_app` is still `ghostty`, on nix-darwin and on NixOS

#### Scenario: A host's own launcher

- **WHEN** `services.boomdrive.launcher.package` is a package of the host's own, and the host has the drives `garden` and `home`
- **THEN** the system profile holds that package as given, on nix-darwin and on NixOS

#### Scenario: Not without the runner

- **WHEN** a system imports the platform's module and leaves `services.boomdrive` off
- **THEN** its system profile holds none of the set's `herdr`, `ghostty` and `boomdrive-launcher`, on nix-darwin and on NixOS

## ADDED Requirements

### Requirement: The launcher has an entry for each drive of this host

With `services.boomdrive.enable`, the launcher the modules make SHALL be the set's `boomdrive-launcher` made, by its override, for one entry for each drive of this host, in the order of the drives' names, each entry's `drive` the drive's name. A drive of this host is one whose `machine` is unset or is `services.crew.host`. A drive of another machine SHALL have no entry.

#### Scenario: Each drive of this host in the launcher

- **WHEN** the account `own` has the drive `home` and the account `work` the drive `garden`, each keeping one repository and naming no machine
- **THEN** the installed launcher is made for `garden` with the session `garden`, then `home` with the session `home`, on nix-darwin and on NixOS

#### Scenario: A drive of another machine has no entry

- **WHEN** `services.crew.host` is `host-a`, the drive `garden` sets `machine = "host-b"`, and the drive `home` names no machine
- **THEN** the installed launcher is made for `home` alone, on nix-darwin and on NixOS

#### Scenario: No drive, the set's launcher

- **WHEN** a system turns on `services.boomdrive` with no account
- **THEN** the installed launcher is the set's `boomdrive-launcher`'s store path, on nix-darwin and on NixOS

### Requirement: A drive's entry opens its stream's own session

An entry's `session` SHALL be the drive's stream's own session as Flume names it: `services.flume.settings.streams.<stream>.session` where that is set, and the drive's stream's name otherwise (Flume's statements 107 and 186).

#### Scenario: A stream's own session named

- **WHEN** the drive `home` keeps one repository and `services.flume.settings.streams.home.session` is `house`
- **THEN** the installed launcher's entry `home` has the session `house`, on nix-darwin and on NixOS

### Requirement: The README says the launcher opens each drive

The README's install with nix SHALL say that the launcher the modules install is a menu-bar app on macOS and a desktop entry on Linux, holding an entry for each drive of the host and `Default stream`.

#### Scenario: The README's launcher

- **WHEN** the README's install with nix is read
- **THEN** it names `menu bar`, `each drive` and `Default stream`
