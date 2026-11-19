## MODIFIED Requirements

### Requirement: Without the runner, the modules install only the tools the host turns on

A system that imports the platform's module and leaves `services.boomdrive` off SHALL get from the set only the tools it turns on, each by its own module's `enable` or by the Boomdrive module's switch for it, each as its `package`, and their herdr plugins. With none turned on, its system profile SHALL hold no package of the set's, and its activation SHALL run no step of the set's as a user.

#### Scenario: Nothing turned on

- **WHEN** a system imports the platform's module, leaves `services.boomdrive` off and turns on no tool, on nix-darwin with no primary user and on NixOS with no `services.boomdrive.user`
- **THEN** it evaluates, failing no assertion, its system profile holds no package of the set's, and its activation runs no step of the set's as a user, on nix-darwin and on NixOS

#### Scenario: Only the tools turned on

- **WHEN** a system leaves `services.boomdrive` off and turns on Flume's module and `services.boomdrive.herdr.enable`, with `services.boomdrive.user = "alice"` on NixOS
- **THEN** its system profile holds the set's `flume` and `herdr` and no other package of the set's, and its activation links the set's `flume-herdr-plugin` alone, on nix-darwin and on NixOS

#### Scenario: Every tool turned on without the runner

- **WHEN** a system leaves `services.boomdrive` off and turns on Flume's, sluice's, crew's and timber's modules and every switch of the Boomdrive module's, naming no package, with `services.boomdrive.user = "alice"` on NixOS
- **THEN** its system profile holds the set's `flume`, `sluice`, `crew`, `timber`, `herdr`, `rex`, `ghostty`, `boomdrive-launcher`, `portless`, `worktrunk`, `pendingyou`, `plannotator` and `plannotator-tui`, each by the store path the set gives, and no `boomdrive`, on nix-darwin and on NixOS

### Requirement: The modules install Rex, replaceable or dropped

`services.boomdrive.rex.enable` SHALL be `services.boomdrive.enable` by default, a host's own value winning. Where it is on, with the runner or without, the modules SHALL put `services.boomdrive.rex.package` in the system profile: the set's `rex` for the host's system by default, a host's own replacing it, and none for `null`. Where it is off, they SHALL install no Rex. They SHALL name Rex in no setting of Flume's, and no step, unit, daemon or agent of theirs SHALL run Rex.

#### Scenario: Rex with the runner

- **WHEN** a system turns on `services.boomdrive` and names no Rex
- **THEN** the system profile holds the set's `rex`, and Flume's `host.multiplexer` is still `herdr`, on nix-darwin and on NixOS

#### Scenario: A host's own Rex

- **WHEN** `services.boomdrive.rex.package` is a package of the host's own
- **THEN** the system profile holds it and not the set's Rex, on nix-darwin and on NixOS

#### Scenario: No Rex of the set's

- **WHEN** `services.boomdrive.rex.package` is `null`
- **THEN** the system evaluates and its profile holds no Rex of the set's, on nix-darwin and on NixOS

#### Scenario: No Rex without the runner

- **WHEN** a system imports the platform's module, leaves `services.boomdrive` off and does not turn Rex on
- **THEN** its system profile holds no Rex of the set's, on nix-darwin and on NixOS

#### Scenario: Rex without the runner

- **WHEN** a system leaves `services.boomdrive` off and turns on `services.boomdrive.rex.enable`
- **THEN** its system profile holds the set's `rex`, on nix-darwin and on NixOS

#### Scenario: No Rex server of the modules'

- **WHEN** a system turns on `services.boomdrive` and names no Rex
- **THEN** no launchd daemon or agent, no systemd unit and no activation step of the system names the set's Rex, on nix-darwin and on NixOS
