## MODIFIED Requirements

### Requirement: The modules bring each tool's own module

`darwinModules.default` and `nixosModules.default`, each the same value as the platform's `boomdrive`, SHALL import the runner's module and Flume's, sluice's, crew's and timber's, each the module that tool's flake gives at the release tag the lock names. No file of this tree SHALL declare an option of `services.flume`, `services.sluice`, `services.crew` or `services.timber`.

#### Scenario: Each tool's options are its own flake's

- **WHEN** a system imports the platform's `default` module
- **THEN** every option of `services.flume`, `services.sluice`, `services.crew` and `services.timber` is declared in a file of that tool's input, and none in this tree, on nix-darwin and on NixOS

### Requirement: The flake re-exports each tool's module

The flake SHALL give each of Flume's, sluice's, crew's and timber's modules, as that tool's flake gives it, as `darwinModules.<tool>` and `nixosModules.<tool>`, beside `boomdrive` and `default`.

#### Scenario: The tools' modules re-exported

- **WHEN** the flake's `darwinModules` and `nixosModules` are listed, and a system imports the platform's `sluice` module alone
- **THEN** each gives `boomdrive`, `crew`, `default`, `flume`, `sluice` and `timber`, and the system declares `services.sluice` from sluice's input and no `services.boomdrive`

### Requirement: The Boomdrive module is taken once however often it is imported

The platform's `boomdrive` module SHALL carry the key `github:agentplot/boomdrive#<platform>Modules.boomdrive`, so a system that imports it more than once, as `default` and as `boomdrive`, takes it and the tools' modules it imports once. A system that imports the platform's `default` module and timber's flake's own module for the platform SHALL take timber's module once.

#### Scenario: Imported twice

- **WHEN** a system imports the platform's `default` module and its `boomdrive` module
- **THEN** it evaluates, failing no assertion, on nix-darwin and on NixOS

#### Scenario: timber's own module beside the set's

- **WHEN** a system turns on `services.boomdrive`, importing the platform's `default` module and timber's flake's own `darwinModules.timber` or `nixosModules.timber`
- **THEN** it evaluates, failing no assertion, on nix-darwin and on NixOS

### Requirement: The Boomdrive module turns on Flume's, sluice's and timber's modules and leaves crew's to the host

With `services.boomdrive.enable`, the modules SHALL set `services.flume.enable`, `services.sluice.enable` and `services.timber.enable` true by default, a host's own value winning. They SHALL leave `services.crew.enable` to the host. On NixOS they SHALL set `services.flume.user`, `services.sluice.user`, `services.crew.user` and `services.timber.user` to `services.boomdrive.user` by default, a host's own value winning.

#### Scenario: A host with the runner alone

- **WHEN** a system turns on `services.boomdrive` and sets no option of Flume's, sluice's, crew's or timber's
- **THEN** `services.flume.enable`, `services.sluice.enable` and `services.timber.enable` are true, `services.crew.enable` is false, and the system fails no assertion, on nix-darwin and on NixOS

#### Scenario: A host that keeps its own Flume file

- **WHEN** a system turns on `services.boomdrive` and sets `services.flume.enable = false`
- **THEN** its activation links no file of Flume's, and its system profile holds no Flume of the set's

#### Scenario: The tools' user on NixOS

- **WHEN** the NixOS module is enabled for `alice`, with crew turned on with its host, its drives and its hosts, and again with `services.sluice.user = "bob"`
- **THEN** Flume's, sluice's, crew's and timber's `user` are `alice` in the first, and sluice's is `bob` in the second

### Requirement: Each tool's package is the set's unless the host gives its own

The modules SHALL set `services.flume.package`, `services.sluice.package`, `services.crew.package` and `services.timber.package` to the set's package of that tool for the host's system by default, whether or not the runner is enabled. A host's own value SHALL win, and SHALL change nothing of the tool's file.

#### Scenario: The set's packages

- **WHEN** a system turns on `services.boomdrive` and crew with its host, its drives and its hosts, naming no tool's package
- **THEN** the system profile holds the set's `boomdrive`, `flume`, `sluice`, `crew` and `timber`, on nix-darwin and on NixOS

#### Scenario: A host's own crew

- **WHEN** `services.crew.package` is a package of the host's own
- **THEN** the system profile holds it and not the set's crew, and crew's activation links the same file it links with the set's

#### Scenario: No sluice of the set's

- **WHEN** `services.sluice.package` is `null`
- **THEN** the system profile holds no sluice of the set's, and sluice's file is still linked

### Requirement: The Boomdrive module writes no file of the other tools'

The modules' own activation SHALL link the runner's file, write Pending You's folder where crew's companions are installed, run herdr's own `plugin link` for the plugins `services.boomdrive.herdr.plugins` lists, write or remove each of Claude Code's two managed documents on nix-darwin, and do nothing else of a tool's. Flume's, sluice's, crew's and timber's files, crew's source in Flume's folder among them, SHALL each be written by that tool's own module, as it writes them without Boomdrive's.

#### Scenario: The tools' activation is their own

- **WHEN** a system with `services.boomdrive` and crew turned on is set beside one that imports Flume's, sluice's, crew's and timber's own modules alone, each turned on with the same settings, packages and user, and Flume's with crew's source declared as the modules declare it
- **THEN** on nix-darwin their activations are the same but for the runner's own link, Pending You's folder, herdr's plugins' links and the managed telemetry and transcripts documents' steps, each run once, and on NixOS their user activations are the same but for the runner's own

### Requirement: The modules hold no option that installs a tool its own module installs

The modules SHALL hold no option of their own that installs a tool whose own module installs it: setting `services.boomdrive.tools` SHALL fail evaluation, naming `services.flume.package` and `services.sluice.package`, and setting `services.boomdrive.timber.package` SHALL fail evaluation, naming `services.timber.package` and `services.timber.enable`. Their own options SHALL install herdr, Ghostty, portless, worktrunk, the launcher, Pending You, Plannotator and plannotator-tui. The README's install with nix SHALL name Flume's, sluice's and crew's `package`, each option of the modules' own that installs a tool, and the shell, and no `services.boomdrive.tools`.

#### Scenario: The tools option is gone

- **WHEN** a system sets `services.boomdrive.tools = [ ]`
- **THEN** evaluation fails, naming `services.flume.package` and `services.sluice.package`

#### Scenario: The timber option is gone

- **WHEN** a system turns on `services.boomdrive` and sets `services.boomdrive.timber.package`
- **THEN** evaluation fails, naming `services.timber.package` and `services.timber.enable`, on nix-darwin and on NixOS

#### Scenario: The README's words

- **WHEN** the README's install with nix is read
- **THEN** it names `services.flume.package`, `services.sluice.package`, `services.crew.package`, `services.boomdrive.herdr.package`, `services.boomdrive.herdr.plugins`, `services.boomdrive.ghostty.package`, `services.boomdrive.portless.package`, `services.boomdrive.worktrunk.package`, `services.boomdrive.launcher.package`, `services.boomdrive.pendingyou.package`, `services.boomdrive.plannotator.package`, `services.boomdrive.plannotator-tui.package` and `nix develop github:boomdrivedev/boomdrive`, and no `services.boomdrive.tools`

### Requirement: The modules install timber, replaceable or dropped

With `services.boomdrive.enable`, timber's own module, which the Boomdrive module turns on, SHALL put `services.timber.package` in the system profile, whether or not crew is on. It SHALL be the set's `timber` for the host's system by default, a host's own package SHALL replace it, and `null` SHALL install none. A host's `services.timber.enable = false` SHALL install no timber of the set's and link no file of timber's. With the runner not enabled, the modules SHALL turn on no module of timber's and install no timber. The Boomdrive module SHALL write no file of timber's.

#### Scenario: timber with the runner

- **WHEN** a system turns on `services.boomdrive` and names no timber
- **THEN** `services.timber.enable` is true and the system profile holds the set's `timber`, on nix-darwin and on NixOS

#### Scenario: A host's own timber

- **WHEN** `services.timber.package` is a package of the host's own
- **THEN** the system profile holds it and not the set's timber, and timber's activation links the same file it links with the set's, on nix-darwin and on NixOS

#### Scenario: No timber of the set's

- **WHEN** `services.timber.package` is `null`
- **THEN** the system evaluates, its profile holds no timber of the set's, and timber's file is still linked, on nix-darwin and on NixOS

#### Scenario: A host that keeps its own timber file

- **WHEN** a system turns on `services.boomdrive` and sets `services.timber.enable = false`
- **THEN** it evaluates, its activation links no file of timber's, and its system profile holds no timber of the set's, on nix-darwin and on NixOS

#### Scenario: No timber without the runner

- **WHEN** a system imports the platform's module and leaves `services.boomdrive` off
- **THEN** `services.timber.enable` is false and its system profile holds no timber of the set's, on nix-darwin and on NixOS

## RENAMED Requirements

- FROM: `### Requirement: The Boomdrive module turns on Flume's and sluice's modules and leaves crew's to the host`
- TO: `### Requirement: The Boomdrive module turns on Flume's, sluice's and timber's modules and leaves crew's to the host`
