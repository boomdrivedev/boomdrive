## ADDED Requirements

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
- **THEN** its system profile holds the set's `flume`, `sluice`, `crew`, `timber`, `herdr`, `ghostty`, `boomdrive-launcher`, `portless`, `worktrunk`, `pendingyou`, `plannotator` and `plannotator-tui`, each by the store path the set gives, and no `boomdrive`, on nix-darwin and on NixOS

### Requirement: herdr's plugins by default are those of the tools the set installs

`services.boomdrive.herdr.plugins` SHALL by default hold, in this order, the set's Flume, sluice and timber plugins, Pending You for Herdr and Annotate, each only where the set installs its tool: Flume's, sluice's or timber's module on, or `services.boomdrive.pendingyou.enable` or `services.boomdrive.plannotator-tui.enable` on, with that tool's package the set's own. A host's own list SHALL be linked as given.

#### Scenario: The runner's plugins

- **WHEN** a system turns on `services.boomdrive`, leaves crew off and names no package and no plugin
- **THEN** `services.boomdrive.herdr.plugins` is the set's `flume-herdr-plugin`, `sluice-herdr-plugin` and `timber-herdr-plugin`, on nix-darwin and on NixOS

#### Scenario: A host's own Flume

- **WHEN** a system turns on `services.boomdrive`, gives `services.flume.package` a package of the host's own and names no plugin
- **THEN** `services.boomdrive.herdr.plugins` is the set's `sluice-herdr-plugin` and `timber-herdr-plugin`, on nix-darwin and on NixOS

#### Scenario: Flume's module off

- **WHEN** a system turns on `services.boomdrive`, sets `services.flume.enable = false` and names no plugin
- **THEN** `services.boomdrive.herdr.plugins` is the set's `sluice-herdr-plugin` and `timber-herdr-plugin`, on nix-darwin and on NixOS

#### Scenario: The plugins of the tools turned on without the runner

- **WHEN** a system leaves `services.boomdrive` off and turns on sluice's module and `services.boomdrive.plannotator-tui.enable`, naming no plugin
- **THEN** `services.boomdrive.herdr.plugins` is the set's `sluice-herdr-plugin` and `plannotator-herdr-plugin`, on nix-darwin and on NixOS

#### Scenario: A host's own list

- **WHEN** a system turns on `services.boomdrive` and sets `services.boomdrive.herdr.plugins` to the set's `timber-herdr-plugin` alone, with `services.timber.package` a package of the host's own
- **THEN** its activation links the set's `timber-herdr-plugin` alone, on nix-darwin and on NixOS

### Requirement: The set's user links the plugins and writes Pending You's folder, with the runner or without

The steps that link herdr's plugins and write Pending You's folder SHALL run as the set's user: on nix-darwin `system.primaryUser`, under their home, and on NixOS `services.boomdrive.user`, in that user's activation alone. Without the runner, no other step of the set's SHALL run as that user, and no agent or unit SHALL be declared and no user linger. Where such a step runs and no user is named, evaluation SHALL fail, naming the option to set.

#### Scenario: The plugins linked without the runner on nix-darwin

- **WHEN** a nix-darwin system whose primary user is `alice`, at `/Users/alice`, leaves `services.boomdrive` off and turns on Flume's module and `services.boomdrive.herdr.enable`
- **THEN** its activation runs, as `alice` under `/Users/alice`, the step that links the set's `flume-herdr-plugin` with the set's herdr, runs no link of the runner's file, and declares no agent `boomdrive`

#### Scenario: The plugins linked without the runner on NixOS

- **WHEN** a NixOS system leaves `services.boomdrive` off, sets `services.boomdrive.user = "alice"`, and turns on Flume's module, with its user `alice`, and `services.boomdrive.herdr.enable`
- **THEN** `alice`'s user activation runs the step that links the set's `flume-herdr-plugin` with the set's herdr and no `keep`, another user's runs none, no user unit `boomdrive` is declared, and `alice` does not linger

#### Scenario: No primary user on nix-darwin

- **WHEN** a nix-darwin system with no primary user leaves `services.boomdrive` off and turns on Flume's module and `services.boomdrive.herdr.enable`
- **THEN** evaluation fails, naming `services.boomdrive.herdr.enable`

#### Scenario: No user on NixOS

- **WHEN** a NixOS system leaves `services.boomdrive` off, names no `services.boomdrive.user`, and turns on `services.boomdrive.herdr.enable` and Flume's module, with its user `alice`
- **THEN** evaluation fails, naming `services.boomdrive.user`

## MODIFIED Requirements

### Requirement: The Boomdrive module turns on Flume's, sluice's and timber's modules and leaves crew's to the host

With `services.boomdrive.enable`, the modules SHALL set `services.flume.enable`, `services.sluice.enable` and `services.timber.enable` true by default, a host's own value winning. They SHALL leave `services.crew.enable` to the host. On NixOS, wherever the host names `services.boomdrive.user`, with the runner or without, they SHALL set `services.flume.user`, `services.sluice.user`, `services.crew.user` and `services.timber.user` to it by default, a host's own value winning.

#### Scenario: A host with the runner alone

- **WHEN** a system turns on `services.boomdrive` and sets no option of Flume's, sluice's, crew's or timber's
- **THEN** `services.flume.enable`, `services.sluice.enable` and `services.timber.enable` are true, `services.crew.enable` is false, and the system fails no assertion, on nix-darwin and on NixOS

#### Scenario: A host that keeps its own Flume file

- **WHEN** a system turns on `services.boomdrive` and sets `services.flume.enable = false`
- **THEN** its activation links no file of Flume's, and its system profile holds no Flume of the set's

#### Scenario: The tools' user on NixOS

- **WHEN** the NixOS module is enabled for `alice`, with crew turned on with its host, its drives and its hosts, and again with `services.sluice.user = "bob"`
- **THEN** Flume's, sluice's, crew's and timber's `user` are `alice` in the first, and sluice's is `bob` in the second

#### Scenario: The tools' user without the runner

- **WHEN** a NixOS system leaves `services.boomdrive` off, sets `services.boomdrive.user = "alice"`, and turns on Flume's and sluice's modules, naming neither's user
- **THEN** Flume's and sluice's `user` are `alice`, and the system fails no assertion

### Requirement: The modules install herdr, Ghostty and the launcher, each replaceable or dropped

`services.boomdrive.herdr.enable`, `services.boomdrive.ghostty.enable` and `services.boomdrive.launcher.enable` SHALL each be `services.boomdrive.enable` by default, a host's own value winning. Where one is on, with the runner or without, the modules SHALL put its `package` in the system profile: the set's for the host's system by default, a host's own installed as given, and none for `null`. Where it is off, they SHALL install none of it.

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

- **WHEN** a system imports the platform's module, leaves `services.boomdrive` off and turns on none of the three
- **THEN** its system profile holds none of the set's `herdr`, `ghostty` and `boomdrive-launcher`, on nix-darwin and on NixOS

#### Scenario: The way in without the runner

- **WHEN** a system leaves `services.boomdrive` off and turns on `services.boomdrive.herdr.enable`, `services.boomdrive.ghostty.enable` and `services.boomdrive.launcher.enable`
- **THEN** its system profile holds the set's `herdr`, `ghostty` and `boomdrive-launcher`, on nix-darwin and on NixOS

#### Scenario: Ghostty turned off with the runner

- **WHEN** a system turns on `services.boomdrive` and sets `services.boomdrive.ghostty.enable = false`
- **THEN** its system profile holds no Ghostty of the set's and holds the set's `herdr` and `boomdrive-launcher`, on nix-darwin and on NixOS

### Requirement: The modules link the set's herdr plugins with herdr's own command

Wherever `services.boomdrive.herdr.enable` is on, with the runner or without, each activation SHALL run, as the set's user, the installed herdr's own `herdr plugin link <plugin>` for each package of `services.boomdrive.herdr.plugins`, in order, with no `HERDR_*` variable of the caller's. The modules SHALL write no file of herdr's. With herdr off, no herdr package or no plugin, none SHALL be linked.

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

#### Scenario: herdr turned off

- **WHEN** a system turns on `services.boomdrive` and sets `services.boomdrive.herdr.enable = false`
- **THEN** its activation links no plugin, and its system profile holds no herdr of the set's, on nix-darwin and on NixOS

### Requirement: The modules name herdr and Ghostty Flume's terminal and window

Where `services.boomdrive.herdr.enable` is on, the modules SHALL set `services.flume.settings.host.multiplexer` to `herdr` by default, and where `services.boomdrive.ghostty.enable` is on, `services.flume.settings.host.terminal_app` to `ghostty` by default, a host's own value winning each time, with the runner or without, and whether or not the host keeps the set's herdr and Ghostty. Where the host's `services.flume.settings.host` gives `terminal`, they SHALL set no `multiplexer`.

#### Scenario: Flume's terminal and window

- **WHEN** a system turns on `services.boomdrive` and names no key of Flume's `host`
- **THEN** `services.flume.settings.host` gives `multiplexer` `herdr` and `terminal_app` `ghostty`, on nix-darwin and on NixOS

#### Scenario: A host's own window

- **WHEN** a system turns on `services.boomdrive` and sets `services.flume.settings.host.terminal_app = "terminal"`
- **THEN** `services.flume.settings.host` gives `terminal_app` `terminal` and `multiplexer` `herdr`

#### Scenario: The word of before

- **WHEN** a system turns on `services.boomdrive` and sets `services.flume.settings.host.terminal = "herdr"`
- **THEN** `services.flume.settings.host` gives `terminal` `herdr` and no `multiplexer`

#### Scenario: Flume's terminal without the runner

- **WHEN** a system leaves `services.boomdrive` off and turns on Flume's module and `services.boomdrive.herdr.enable`, and not Ghostty's
- **THEN** `services.flume.settings.host` gives `multiplexer` `herdr` and leaves `terminal_app` unset, on nix-darwin and on NixOS

### Requirement: The modules install portless, replaceable or dropped

`services.boomdrive.portless.enable` SHALL be `services.boomdrive.enable` by default, a host's own value winning. Where it is on, with the runner or without, the modules SHALL put `services.boomdrive.portless.package` in the system profile: the set's portless for the host's system by default, a host's own replacing it, and none for `null`. Where it is off, the modules SHALL install no portless.

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

- **WHEN** a system imports the platform's module, leaves `services.boomdrive` off and does not turn portless on
- **THEN** its system profile holds no portless of the set's, on nix-darwin and on NixOS

#### Scenario: portless without the runner

- **WHEN** a system leaves `services.boomdrive` off and turns on `services.boomdrive.portless.enable`
- **THEN** its system profile holds the set's `portless`, on nix-darwin and on NixOS

### Requirement: The modules install worktrunk, replaceable or dropped

`services.boomdrive.worktrunk.enable` SHALL be `services.boomdrive.enable` by default, a host's own value winning. Where it is on, with the runner or without, the modules SHALL put `services.boomdrive.worktrunk.package` in the system profile: the set's `worktrunk` for the host's system by default, a host's own replacing it, and none for `null`. Where it is off, the modules SHALL install no worktrunk. The modules SHALL write no file of worktrunk's.

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

- **WHEN** a system imports the platform's module, leaves `services.boomdrive` off and does not turn worktrunk on
- **THEN** its system profile holds no worktrunk of the set's, on nix-darwin and on NixOS

#### Scenario: worktrunk without the runner

- **WHEN** a system leaves `services.boomdrive` off and turns on `services.boomdrive.worktrunk.enable`
- **THEN** its system profile holds the set's `worktrunk`, on nix-darwin and on NixOS

### Requirement: The launcher has an entry for each drive of this host

With `services.boomdrive.enable`, the launcher the modules make SHALL be the set's `boomdrive-launcher` made, by its override, for one entry for each drive of this host, in the order of the drives' names, each entry's `drive` the drive's name. A drive of this host is one whose `machine` is unset or is `services.crew.host`. A drive of another machine SHALL have no entry. With no drive of this host, or without the runner, it SHALL be the set's own `boomdrive-launcher`.

#### Scenario: Each drive of this host in the launcher

- **WHEN** the account `own` has the drive `home` and the account `work` the drive `garden`, each keeping one repository and naming no machine
- **THEN** the installed launcher is made for `garden` with the session `garden`, then `home` with the session `home`, on nix-darwin and on NixOS

#### Scenario: A drive of another machine has no entry

- **WHEN** `services.crew.host` is `host-a`, the drive `garden` sets `machine = "host-b"`, and the drive `home` names no machine
- **THEN** the installed launcher is made for `home` alone, on nix-darwin and on NixOS

#### Scenario: No drive, the set's launcher

- **WHEN** a system turns on `services.boomdrive` with no account
- **THEN** the installed launcher is the set's `boomdrive-launcher`'s store path, on nix-darwin and on NixOS

#### Scenario: No entries without the runner

- **WHEN** a system leaves `services.boomdrive` off, turns on `services.boomdrive.launcher.enable`, and has the account `own` with the drive `home` keeping one repository and naming no machine
- **THEN** the installed launcher is the set's `boomdrive-launcher`'s store path, on nix-darwin and on NixOS

### Requirement: The modules install crew's companions, each turned on by itself, replaceable or dropped

`services.boomdrive.pendingyou.enable`, `services.boomdrive.plannotator.enable` and `services.boomdrive.plannotator-tui.enable` SHALL each be on by default where `services.boomdrive.enable` and `services.crew.enable` both are, a host's own value winning. Where one is on, runner or not, the modules SHALL put its `package` in the system profile, and `pendingyou-signin` beside a Pending You: the set's by default, a host's own as given, none for `null`. Off, they SHALL install none of it.

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

- **WHEN** a system imports the platform's module, turns on `services.crew`, leaves `services.boomdrive` off and turns on no companion
- **THEN** its profile holds none of the set's `pendingyou`, `plannotator` and `plannotator-tui`, on nix-darwin and on NixOS

#### Scenario: A companion turned on without the runner

- **WHEN** a system leaves `services.boomdrive` and `services.crew` off and turns on `services.boomdrive.pendingyou.enable`
- **THEN** its profile holds the set's `pendingyou` and `pendingyou-signin`, and neither the set's `plannotator` nor its `plannotator-tui`, on nix-darwin and on NixOS

### Requirement: Each activation writes Pending You's folder before it links the plugins

Wherever `services.boomdrive.pendingyou.enable` is on and its package is not `null`, with the runner or without, each activation SHALL run, as the set's user, the step that writes Pending You's folder, `<home>/.config/pendingyou`, before the step that links the plugins.

#### Scenario: The folder written on nix-darwin

- **WHEN** a nix-darwin system whose primary user is `alice`, at `/Users/alice`, turns on `services.boomdrive` and `services.crew`
- **THEN** its activation runs, as `alice`, the step that writes the folder's four files and its wake mod into `/Users/alice/.config/pendingyou`, before the step that links the plugins

#### Scenario: The folder written on NixOS

- **WHEN** a NixOS system turns on `services.boomdrive` and `services.crew` for `alice`
- **THEN** `alice`'s user activation runs the step that writes the folder into her home's `.config/pendingyou`, before the plugins' step, and another user's runs none

#### Scenario: The folder written without the runner

- **WHEN** a system leaves `services.boomdrive` and `services.crew` off and turns on `services.boomdrive.pendingyou.enable`, with the primary user `alice` on nix-darwin and `services.boomdrive.user = "alice"` on NixOS
- **THEN** its activation runs, as `alice`, the step that writes the folder into `alice`'s home's `.config/pendingyou`, and on NixOS another user's runs none

### Requirement: The modules install timber, replaceable or dropped

With `services.boomdrive.enable`, timber's own module, which the Boomdrive module turns on, SHALL put `services.timber.package` in the system profile, whether or not crew is on: the set's `timber` by default, a host's own where given, none for `null`. A host's `services.timber.enable = false` SHALL install no timber of the set's and link no file of timber's. Without the runner, timber's module SHALL be off until the host turns it on. The Boomdrive module SHALL write no file of timber's.

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

#### Scenario: timber turned on without the runner

- **WHEN** a system leaves `services.boomdrive` off and turns on timber's module, naming no package, with its user `alice` on NixOS
- **THEN** its system profile holds the set's `timber`, on nix-darwin and on NixOS

## RENAMED Requirements

- FROM: `### Requirement: The modules install crew's companions with the runner and crew, each replaceable or dropped`
- TO: `### Requirement: The modules install crew's companions, each turned on by itself, replaceable or dropped`
