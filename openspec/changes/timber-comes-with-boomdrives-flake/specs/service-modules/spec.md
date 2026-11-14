# Service modules

## MODIFIED Requirements

### Requirement: The modules hold no option that installs a tool its own module installs

The modules SHALL hold no option of their own that installs a tool whose own module installs it: setting `services.boomdrive.tools` SHALL fail evaluation, naming `services.flume.package` and `services.sluice.package`. Their own options SHALL install herdr, Ghostty, portless, worktrunk, timber, the launcher, Pending You, Plannotator and plannotator-tui. The README's install with nix SHALL name each option that installs a tool of the set, and the shell, and no `services.boomdrive.tools`.

#### Scenario: The tools option is gone

- **WHEN** a system sets `services.boomdrive.tools = [ ]`
- **THEN** evaluation fails, naming `services.flume.package` and `services.sluice.package`

#### Scenario: The README's words

- **WHEN** the README's install with nix is read
- **THEN** it names `services.flume.package`, `services.sluice.package`, `services.crew.package`, `services.boomdrive.herdr.package`, `services.boomdrive.herdr.plugins`, `services.boomdrive.ghostty.package`, `services.boomdrive.portless.package`, `services.boomdrive.worktrunk.package`, `services.boomdrive.timber.package`, `services.boomdrive.launcher.package`, `services.boomdrive.pendingyou.package`, `services.boomdrive.plannotator.package`, `services.boomdrive.plannotator-tui.package` and `nix develop github:boomdrivedev/boomdrive`, and no `services.boomdrive.tools`

### Requirement: The modules link the set's herdr plugins with herdr's own command

With `services.boomdrive.enable`, each activation SHALL run, as the runner's user, the installed herdr's own `herdr plugin link <plugin>` for each package of `services.boomdrive.herdr.plugins`, in order, with no `HERDR_*` variable of the caller's. The list's default SHALL be the set's Flume, sluice and timber plugins, then, with crew on, its companions'. The modules SHALL write no file of herdr's. With no herdr package or no plugin, none SHALL be linked.

#### Scenario: The plugins linked on nix-darwin

- **WHEN** a nix-darwin system whose primary user is `alice` turns on `services.boomdrive`
- **THEN** its activation runs, as `alice`, the step that links the set's `flume-herdr-plugin`, `sluice-herdr-plugin` and `timber-herdr-plugin` with the set's herdr

#### Scenario: The plugins linked on NixOS

- **WHEN** a NixOS system turns on `services.boomdrive` for `alice`
- **THEN** `alice`'s user activation runs the step that links the set's `flume-herdr-plugin`, `sluice-herdr-plugin` and `timber-herdr-plugin` with the set's herdr, and another user's runs none

#### Scenario: The companions' plugins with crew

- **WHEN** a system turns on `services.boomdrive` and `services.crew` and names no plugin
- **THEN** its activation links the set's `flume-herdr-plugin`, `sluice-herdr-plugin`, `timber-herdr-plugin`, `pendingyou-herdr-plugin` and `plannotator-herdr-plugin`, in that order, on nix-darwin and on NixOS

#### Scenario: Each plugin linked

- **WHEN** the activation's plugin step runs with herdr and two plugins, in a shell whose `HERDR_SOCKET_PATH` and `HERDR_ENV` are set
- **THEN** herdr is run as `herdr plugin link <plugin>` for each plugin, in order, with neither variable set, and the step exits 0

#### Scenario: No herdr of the set's

- **WHEN** `services.boomdrive.herdr.package` is `null`, or `services.boomdrive.herdr.plugins` is `[ ]`
- **THEN** the activation links no plugin, on nix-darwin and on NixOS

### Requirement: The modules install crew's companions with the runner and crew, each replaceable or dropped

With `services.boomdrive.enable` and `services.crew.enable`, the modules SHALL put `services.boomdrive.pendingyou.package`, `services.boomdrive.plannotator.package` and `services.boomdrive.plannotator-tui.package` in the system profile, and `pendingyou-signin` beside a Pending You. Each SHALL be the set's by default, a host's own replacing it and `null` installing none. Without the runner or crew, none SHALL be installed and no folder of Pending You's written.

#### Scenario: The companions with the runner and crew

- **WHEN** a system turns on `services.boomdrive` and `services.crew` and names none of the three
- **THEN** the system profile holds the set's `pendingyou`, `plannotator` and `plannotator-tui`, and `pendingyou-signin`, on nix-darwin and on NixOS

#### Scenario: A host's own Plannotator

- **WHEN** `services.boomdrive.plannotator.package` is a package of the host's own
- **THEN** the system profile holds it and not the set's Plannotator, on nix-darwin and on NixOS

#### Scenario: No Pending You of the set's

- **WHEN** `services.boomdrive.pendingyou.package` is `null`
- **THEN** the system evaluates, its profile holds no Pending You and no `pendingyou-signin`, and its activation writes no folder of Pending You's, on nix-darwin and on NixOS

#### Scenario: No companions without crew

- **WHEN** a system turns on `services.boomdrive` and leaves `services.crew` off
- **THEN** its profile holds none of the set's `pendingyou`, `plannotator` and `plannotator-tui`, its activation writes no folder of Pending You's, and its plugins' default is Flume's, sluice's and timber's alone, on nix-darwin and on NixOS

#### Scenario: No companions without the runner

- **WHEN** a system imports the platform's module, turns on `services.crew` and leaves `services.boomdrive` off
- **THEN** its profile holds none of the set's `pendingyou`, `plannotator` and `plannotator-tui`, on nix-darwin and on NixOS

## ADDED Requirements

### Requirement: The modules install timber, replaceable or dropped

With `services.boomdrive.enable`, the modules SHALL put `services.boomdrive.timber.package` in the system profile, whether or not crew is on. It SHALL be the set's `timber` for the host's system by default, a host's own package SHALL replace it, and `null` SHALL install none. With the runner not enabled, the modules SHALL install no timber. The modules SHALL write no file of timber's.

#### Scenario: timber with the runner

- **WHEN** a system turns on `services.boomdrive` and names no timber
- **THEN** the system profile holds the set's `timber`, on nix-darwin and on NixOS

#### Scenario: A host's own timber

- **WHEN** `services.boomdrive.timber.package` is a package of the host's own
- **THEN** the system profile holds it and not the set's timber, on nix-darwin and on NixOS

#### Scenario: No timber of the set's

- **WHEN** `services.boomdrive.timber.package` is `null`
- **THEN** the system evaluates and its profile holds no timber of the set's, on nix-darwin and on NixOS

#### Scenario: No timber without the runner

- **WHEN** a system imports the platform's module and leaves `services.boomdrive` off
- **THEN** its system profile holds no timber of the set's, on nix-darwin and on NixOS

### Requirement: timber's serving process is timber's own

The modules SHALL start no `timber serve`. No launchd daemon or agent, systemd unit or activation step of theirs SHALL run the set's timber. The README's install with nix SHALL name the set's timber, its herdr plugin and `timber serve`, and say that timber's pane or command starts it where none answers, or a host runs it as a service of its own.

#### Scenario: No serving process of the modules'

- **WHEN** a system turns on `services.boomdrive` and names no timber
- **THEN** no launchd daemon or agent, no systemd unit and no activation step of the system names the set's timber, on nix-darwin and on NixOS

#### Scenario: The README's timber

- **WHEN** the README's install with nix is read
- **THEN** it names `github:boomdrivedev/boomdrive#timber`, `timber-herdr-plugin` and `timber serve`
