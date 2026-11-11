# Service modules

## RENAMED Requirements

- FROM: `### Requirement: The modules hold no option that installs another tool`
- TO: `### Requirement: The modules hold no option that installs a tool its own module installs`

## MODIFIED Requirements

### Requirement: The runner finds its tools on the profiles' PATH

Each module SHALL put `package` in the system profile, and each tool's own module SHALL put that tool's package there, the set's unless the host gives its own. The service's `PATH` SHALL be the user's profiles, then the system's, then the operating system's folders, so the runner, what it launches and a person's shell run one copy of each tool. git SHALL be left to the host.

#### Scenario: The set's tools

- **WHEN** the module is enabled with no tool's package named
- **THEN** the system profile holds the package, the set's Flume and the set's sluice

#### Scenario: A host's own tools

- **WHEN** the module is enabled with `services.flume.package` and `services.sluice.package` each `null`
- **THEN** the system profile holds the package and no Flume or sluice of the set's

#### Scenario: The user's profile first

- **WHEN** the module is enabled for `alice`
- **THEN** the service's `PATH` names `/etc/profiles/per-user/alice/bin` and `alice`'s `~/.nix-profile/bin` before the system profile's `bin`, and holds no package's own folder

### Requirement: The Boomdrive module writes no file of the other tools'

The modules' own activation SHALL link the runner's file, run herdr's own `plugin link` for the plugins `services.boomdrive.herdr.plugins` lists, and do nothing else of a tool's. Flume's, sluice's and crew's files SHALL each be written and linked by that tool's own module, from its own `settings`, as it writes them when imported without Boomdrive's.

#### Scenario: The tools' activation is their own

- **WHEN** a system with `services.boomdrive` and crew turned on is set beside one that imports Flume's, sluice's and crew's own modules alone, each turned on with the same settings, packages and user
- **THEN** on nix-darwin their activations are the same but for the runner's own link and herdr's plugins' links, and on NixOS their user activations are the same but for the runner's own

### Requirement: The modules hold no option that installs a tool its own module installs

The modules SHALL hold no option of their own that installs a tool whose own module installs it: setting `services.boomdrive.tools` SHALL fail evaluation, naming `services.flume.package` and `services.sluice.package`. Their own options SHALL install herdr, Ghostty and the launcher. The README's install with nix SHALL name each option that installs a tool of the set, and the shell, and no `services.boomdrive.tools`.

#### Scenario: The tools option is gone

- **WHEN** a system sets `services.boomdrive.tools = [ ]`
- **THEN** evaluation fails, naming `services.flume.package` and `services.sluice.package`

#### Scenario: The README's words

- **WHEN** the README's install with nix is read
- **THEN** it names `services.flume.package`, `services.sluice.package`, `services.crew.package`, `services.boomdrive.herdr.package`, `services.boomdrive.herdr.plugins`, `services.boomdrive.ghostty.package`, `services.boomdrive.launcher.package` and `nix develop github:agentplot/boomdrive`, and no `services.boomdrive.tools`

## ADDED Requirements

### Requirement: The modules install herdr, Ghostty and the launcher, each replaceable or dropped

With `services.boomdrive.enable`, the modules SHALL put `services.boomdrive.herdr.package`, `services.boomdrive.ghostty.package` and `services.boomdrive.launcher.package` in the system profile. Each SHALL be the set's for the host's system by default, a host's own package SHALL replace it, and `null` SHALL install none. With the runner not enabled, the modules SHALL install none of them.

#### Scenario: The way in with the runner

- **WHEN** a system turns on `services.boomdrive` and names none of the three
- **THEN** the system profile holds the set's `herdr`, `ghostty` and `boomdrive-launcher`, on nix-darwin and on NixOS

#### Scenario: A host's own herdr

- **WHEN** `services.boomdrive.herdr.package` is a package of the host's own
- **THEN** the system profile holds it and not the set's herdr, and the activation links the plugins with it, on nix-darwin and on NixOS

#### Scenario: No Ghostty of the set's

- **WHEN** `services.boomdrive.ghostty.package` is `null`
- **THEN** the system profile holds no Ghostty of the set's, and `services.flume.settings.host.terminal_app` is still `ghostty`, on nix-darwin and on NixOS

#### Scenario: Not without the runner

- **WHEN** a system imports the platform's module and leaves `services.boomdrive` off
- **THEN** its system profile holds none of the set's `herdr`, `ghostty` and `boomdrive-launcher`, on nix-darwin and on NixOS

### Requirement: The modules link the set's herdr plugins with herdr's own command

With `services.boomdrive.enable`, each activation SHALL run, as the runner's user, the installed herdr's own `herdr plugin link <plugin>` for each package of `services.boomdrive.herdr.plugins`, in order, with no `HERDR_*` variable of the caller's. The list's default SHALL be the set's two herdr plugins. The modules SHALL write no file of herdr's. With no herdr package, or no plugin, none SHALL be linked.

#### Scenario: The plugins linked on nix-darwin

- **WHEN** a nix-darwin system whose primary user is `alice` turns on `services.boomdrive`
- **THEN** its activation runs, as `alice`, the step that links the set's `flume-herdr-plugin` and `sluice-herdr-plugin` with the set's herdr

#### Scenario: The plugins linked on NixOS

- **WHEN** a NixOS system turns on `services.boomdrive` for `alice`
- **THEN** `alice`'s user activation runs the step that links the set's `flume-herdr-plugin` and `sluice-herdr-plugin` with the set's herdr, and another user's runs none

#### Scenario: Each plugin linked

- **WHEN** the activation's plugin step runs with herdr and two plugins, in a shell whose `HERDR_SOCKET_PATH` and `HERDR_ENV` are set
- **THEN** herdr is run as `herdr plugin link <plugin>` for each plugin, in order, with neither variable set, and the step exits 0

#### Scenario: No herdr of the set's

- **WHEN** `services.boomdrive.herdr.package` is `null`, or `services.boomdrive.herdr.plugins` is `[ ]`
- **THEN** the activation links no plugin, on nix-darwin and on NixOS

### Requirement: A plugin herdr refuses to link stops nothing else

A link herdr refuses SHALL be said in one line naming the plugin and herdr's words, and SHALL stop no other plugin's link and nothing else of the activation.

#### Scenario: A link herdr refuses

- **WHEN** herdr refuses the first plugin's link, saying `manifest not found`
- **THEN** one line names that plugin and `manifest not found`, the second plugin is still linked, and the step exits 0

### Requirement: The modules name herdr and Ghostty Flume's terminal and window

With `services.boomdrive.enable`, the modules SHALL set `services.flume.settings.host.multiplexer` to `herdr` and `services.flume.settings.host.terminal_app` to `ghostty` by default, each a host's own value winning. They SHALL do so whether or not the host keeps the set's herdr and Ghostty. Where the host's `services.flume.settings.host` gives `terminal`, the word Flume reads as `multiplexer` and refuses beside it, they SHALL set no `multiplexer`.

#### Scenario: Flume's terminal and window

- **WHEN** a system turns on `services.boomdrive` and names no key of Flume's `host`
- **THEN** `services.flume.settings.host` gives `multiplexer` `herdr` and `terminal_app` `ghostty`, on nix-darwin and on NixOS

#### Scenario: A host's own window

- **WHEN** a system turns on `services.boomdrive` and sets `services.flume.settings.host.terminal_app = "terminal"`
- **THEN** `services.flume.settings.host` gives `terminal_app` `terminal` and `multiplexer` `herdr`

#### Scenario: The word of before

- **WHEN** a system turns on `services.boomdrive` and sets `services.flume.settings.host.terminal = "herdr"`
- **THEN** `services.flume.settings.host` gives `terminal` `herdr` and no `multiplexer`
