## MODIFIED Requirements

### Requirement: The modules hold no option that installs a tool its own module installs

The modules SHALL hold no option of their own that installs a tool whose own module installs it: setting `services.boomdrive.tools` SHALL fail evaluation, naming `services.flume.package` and `services.sluice.package`. Their own options SHALL install herdr, Rex, Ghostty, portless, worktrunk, the launcher, Pending You, Plannotator and plannotator-tui. The README's install with nix SHALL name each option that installs a tool of the set, and the shell, and no `services.boomdrive.tools`.

#### Scenario: The tools option is gone

- **WHEN** a system sets `services.boomdrive.tools = [ ]`
- **THEN** evaluation fails, naming `services.flume.package` and `services.sluice.package`

#### Scenario: The README's words

- **WHEN** the README's install with nix is read
- **THEN** it names `services.flume.package`, `services.sluice.package`, `services.crew.package`, `services.boomdrive.herdr.package`, `services.boomdrive.herdr.plugins`, `services.boomdrive.rex.package`, `services.boomdrive.ghostty.package`, `services.boomdrive.portless.package`, `services.boomdrive.worktrunk.package`, `services.boomdrive.launcher.package`, `services.boomdrive.pendingyou.package`, `services.boomdrive.plannotator.package`, `services.boomdrive.plannotator-tui.package` and `nix develop github:boomdrivedev/boomdrive`, and no `services.boomdrive.tools`

## ADDED Requirements

### Requirement: The modules install Rex, replaceable or dropped

With `services.boomdrive.enable`, the modules SHALL put `services.boomdrive.rex.package` in the system profile. It SHALL be the set's `rex` for the host's system by default, a host's own package SHALL replace it, and `null` SHALL install none. With the runner not enabled, the modules SHALL install no Rex. They SHALL name Rex in no setting of Flume's, and no launchd daemon or agent, systemd unit or activation step of theirs SHALL run Rex.

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

- **WHEN** a system imports the platform's module and leaves `services.boomdrive` off
- **THEN** its system profile holds no Rex of the set's, on nix-darwin and on NixOS

#### Scenario: No Rex server of the modules'

- **WHEN** a system turns on `services.boomdrive` and names no Rex
- **THEN** no launchd daemon or agent, no systemd unit and no activation step of the system names the set's Rex, on nix-darwin and on NixOS
