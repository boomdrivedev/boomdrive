## MODIFIED Requirements

### Requirement: The modules hold no option that installs a tool its own module installs

The modules SHALL hold no option of their own that installs a tool whose own module installs it: setting `services.boomdrive.tools` SHALL fail evaluation, naming `services.flume.package` and `services.sluice.package`. Their own options SHALL install herdr, Ghostty, portless and the launcher. The README's install with nix SHALL name each option that installs a tool of the set, and the shell, and no `services.boomdrive.tools`.

#### Scenario: The tools option is gone

- **WHEN** a system sets `services.boomdrive.tools = [ ]`
- **THEN** evaluation fails, naming `services.flume.package` and `services.sluice.package`

#### Scenario: The README's words

- **WHEN** the README's install with nix is read
- **THEN** it names `services.flume.package`, `services.sluice.package`, `services.crew.package`, `services.boomdrive.herdr.package`, `services.boomdrive.herdr.plugins`, `services.boomdrive.ghostty.package`, `services.boomdrive.portless.package`, `services.boomdrive.launcher.package` and `nix develop github:agentplot/boomdrive`, and no `services.boomdrive.tools`

## ADDED Requirements

### Requirement: The modules install portless, replaceable or dropped

With `services.boomdrive.enable`, the modules SHALL put `services.boomdrive.portless.package` in the system profile. It SHALL be the set's portless for the host's system by default. A host's own package SHALL replace it, and `null` SHALL install none. With the runner not enabled, the modules SHALL install no portless.

#### Scenario: portless with the runner

- **WHEN** a system turns on `services.boomdrive` and names no portless
- **THEN** the system profile holds the set's `portless`, on nix-darwin and on NixOS

#### Scenario: A host's own portless

- **WHEN** `services.boomdrive.portless.package` is a package of the host's own
- **THEN** the system profile holds it and not the set's portless, on nix-darwin and on NixOS

#### Scenario: No portless of the set's

- **WHEN** `services.boomdrive.portless.package` is `null`
- **THEN** the system evaluates, and its profile holds no portless of the set's, on nix-darwin and on NixOS

#### Scenario: No portless without the runner

- **WHEN** a system imports the platform's module and leaves `services.boomdrive` off
- **THEN** its system profile holds no portless of the set's, on nix-darwin and on NixOS

### Requirement: portless's proxy and its trust are the person's

The modules SHALL start no proxy of portless's. No launchd daemon or agent, systemd unit or activation step of theirs SHALL run portless. They SHALL write no file of portless's and trust no certificate authority. The README's install with nix SHALL say that portless's proxy is started by its first `portless run` at a terminal or by `portless service install`, and that `portless trust` is the person's.

#### Scenario: No proxy of the modules'

- **WHEN** a system turns on `services.boomdrive` and names no portless
- **THEN** no launchd daemon or agent, no systemd unit and no activation step of the system names the set's portless, on nix-darwin and on NixOS

#### Scenario: The README's proxy

- **WHEN** the README's install with nix is read
- **THEN** it names `portless service install` and `portless trust`
