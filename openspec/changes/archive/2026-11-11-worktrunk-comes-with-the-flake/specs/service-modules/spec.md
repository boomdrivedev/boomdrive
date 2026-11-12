# Service modules

## MODIFIED Requirements

### Requirement: The modules hold no option that installs a tool its own module installs

The modules SHALL hold no option of their own that installs a tool whose own module installs it: setting `services.boomdrive.tools` SHALL fail evaluation, naming `services.flume.package` and `services.sluice.package`. Their own options SHALL install herdr, Ghostty, portless, worktrunk and the launcher. The README's install with nix SHALL name each option that installs a tool of the set, and the shell, and no `services.boomdrive.tools`.

#### Scenario: The tools option is gone

- **WHEN** a system sets `services.boomdrive.tools = [ ]`
- **THEN** evaluation fails, naming `services.flume.package` and `services.sluice.package`

#### Scenario: The README's words

- **WHEN** the README's install with nix is read
- **THEN** it names `services.flume.package`, `services.sluice.package`, `services.crew.package`, `services.boomdrive.herdr.package`, `services.boomdrive.herdr.plugins`, `services.boomdrive.ghostty.package`, `services.boomdrive.portless.package`, `services.boomdrive.worktrunk.package`, `services.boomdrive.launcher.package` and `nix develop github:agentplot/boomdrive`, and no `services.boomdrive.tools`

## ADDED Requirements

### Requirement: The modules install worktrunk, replaceable or dropped

With `services.boomdrive.enable`, the modules SHALL put `services.boomdrive.worktrunk.package` in the system profile. It SHALL be the set's `worktrunk` for the host's system by default, a host's own package SHALL replace it, and `null` SHALL install none. With the runner not enabled, the modules SHALL install no worktrunk. The modules SHALL write no file of worktrunk's.

#### Scenario: worktrunk with the runner

- **WHEN** a system turns on `services.boomdrive` and names no worktrunk
- **THEN** the system profile holds the set's `worktrunk`, on nix-darwin and on NixOS

#### Scenario: A host's own worktrunk

- **WHEN** `services.boomdrive.worktrunk.package` is a package of the host's own
- **THEN** the system profile holds it and not the set's worktrunk, on nix-darwin and on NixOS

#### Scenario: No worktrunk of the set's

- **WHEN** `services.boomdrive.worktrunk.package` is `null`
- **THEN** the system evaluates and its profile holds no worktrunk of the set's, on nix-darwin and on NixOS

#### Scenario: No worktrunk without the runner

- **WHEN** a system imports the platform's module and leaves `services.boomdrive` off
- **THEN** its system profile holds no worktrunk of the set's, on nix-darwin and on NixOS
