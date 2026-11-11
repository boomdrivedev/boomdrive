## MODIFIED Requirements

### Requirement: The Boomdrive module writes no file of the other tools'

The modules' own activation SHALL link the runner's file, write Pending You's folder where crew's companions are installed, run herdr's own `plugin link` for the plugins `services.boomdrive.herdr.plugins` lists, and do nothing else of a tool's. Flume's, sluice's and crew's files SHALL each be written and linked by that tool's own module, from its own `settings`, as it writes them when imported without Boomdrive's.

#### Scenario: The tools' activation is their own

- **WHEN** a system with `services.boomdrive` and crew turned on is set beside one that imports Flume's, sluice's and crew's own modules alone, each turned on with the same settings, packages and user
- **THEN** on nix-darwin their activations are the same but for the runner's own link, Pending You's folder and herdr's plugins' links, and on NixOS their user activations are the same but for the runner's own

### Requirement: The modules hold no option that installs a tool its own module installs

The modules SHALL hold no option of their own that installs a tool whose own module installs it: setting `services.boomdrive.tools` SHALL fail evaluation, naming `services.flume.package` and `services.sluice.package`. Their own options SHALL install herdr, Ghostty, portless, the launcher, Pending You, Plannotator and plannotator-tui. The README's install with nix SHALL name each option that installs a tool of the set, and the shell, and no `services.boomdrive.tools`.

#### Scenario: The tools option is gone

- **WHEN** a system sets `services.boomdrive.tools = [ ]`
- **THEN** evaluation fails, naming `services.flume.package` and `services.sluice.package`

#### Scenario: The README's words

- **WHEN** the README's install with nix is read
- **THEN** it names `services.flume.package`, `services.sluice.package`, `services.crew.package`, `services.boomdrive.herdr.package`, `services.boomdrive.herdr.plugins`, `services.boomdrive.ghostty.package`, `services.boomdrive.portless.package`, `services.boomdrive.launcher.package`, `services.boomdrive.pendingyou.package`, `services.boomdrive.plannotator.package`, `services.boomdrive.plannotator-tui.package` and `nix develop github:agentplot/boomdrive`, and no `services.boomdrive.tools`

### Requirement: The modules link the set's herdr plugins with herdr's own command

With `services.boomdrive.enable`, each activation SHALL run, as the runner's user, the installed herdr's own `herdr plugin link <plugin>` for each package of `services.boomdrive.herdr.plugins`, in order, with no `HERDR_*` variable of the caller's. The list's default SHALL be the set's `flume-herdr-plugin` and `sluice-herdr-plugin`, followed, where `services.crew.enable` is true, by `pendingyou-herdr-plugin` and `plannotator-herdr-plugin`. The modules SHALL write no file of herdr's. With no herdr package, or no plugin, none SHALL be linked.

#### Scenario: The plugins linked on nix-darwin

- **WHEN** a nix-darwin system whose primary user is `alice` turns on `services.boomdrive`
- **THEN** its activation runs, as `alice`, the step that links the set's `flume-herdr-plugin` and `sluice-herdr-plugin` with the set's herdr

#### Scenario: The plugins linked on NixOS

- **WHEN** a NixOS system turns on `services.boomdrive` for `alice`
- **THEN** `alice`'s user activation runs the step that links the set's `flume-herdr-plugin` and `sluice-herdr-plugin` with the set's herdr, and another user's runs none

#### Scenario: The companions' plugins with crew

- **WHEN** a system turns on `services.boomdrive` and `services.crew` and names no plugin
- **THEN** its activation links the set's `flume-herdr-plugin`, `sluice-herdr-plugin`, `pendingyou-herdr-plugin` and `plannotator-herdr-plugin`, in that order, on nix-darwin and on NixOS

#### Scenario: Each plugin linked

- **WHEN** the activation's plugin step runs with herdr and two plugins, in a shell whose `HERDR_SOCKET_PATH` and `HERDR_ENV` are set
- **THEN** herdr is run as `herdr plugin link <plugin>` for each plugin, in order, with neither variable set, and the step exits 0

#### Scenario: No herdr of the set's

- **WHEN** `services.boomdrive.herdr.package` is `null`, or `services.boomdrive.herdr.plugins` is `[ ]`
- **THEN** the activation links no plugin, on nix-darwin and on NixOS

## ADDED Requirements

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
- **THEN** its profile holds none of the set's `pendingyou`, `plannotator` and `plannotator-tui`, its activation writes no folder of Pending You's, and its plugins' default is Flume's and sluice's alone, on nix-darwin and on NixOS

#### Scenario: No companions without the runner

- **WHEN** a system imports the platform's module, turns on `services.crew` and leaves `services.boomdrive` off
- **THEN** its profile holds none of the set's `pendingyou`, `plannotator` and `plannotator-tui`, on nix-darwin and on NixOS

### Requirement: Each activation writes Pending You's folder before it links the plugins

Where the companions are installed and Pending You's package is not `null`, each activation SHALL run, as the runner's user, the step that writes Pending You's folder, `<home>/.config/pendingyou`, before the step that links the plugins.

#### Scenario: The folder written on nix-darwin

- **WHEN** a nix-darwin system whose primary user is `alice`, at `/Users/alice`, turns on `services.boomdrive` and `services.crew`
- **THEN** its activation runs, as `alice`, the step that writes the folder's four files and its wake mod into `/Users/alice/.config/pendingyou`, before the step that links the plugins

#### Scenario: The folder written on NixOS

- **WHEN** a NixOS system turns on `services.boomdrive` and `services.crew` for `alice`
- **THEN** `alice`'s user activation runs the step that writes the folder into her home's `.config/pendingyou`, before the plugins' step, and another user's runs none

### Requirement: Pending You's folder holds its launchers, its manifest and its wake mod

The folder SHALL hold four files and the wake mod: `bin/pendingyou-hook` and `bin/pendingyou-mcp-headers`, each exporting `PENDINGYOU_CONFIG_DIR` as the folder and running the given Pending You; `claude-code.json`, Pending You's manifest, naming the origin `https://www.pendingyou.com` and the folder's helper; `signin/.claude.json`, naming the MCP server `pendingyou` with that helper; and `mod`, the given Pending You's wake mod, copied whole, as Pending You's `init` copies it into the folder.

#### Scenario: The launchers run the given Pending You

- **WHEN** `services.boomdrive.pendingyou.package` is a package of the host's own
- **THEN** both launchers run that package's `pendingyou`, and both export the folder as `PENDINGYOU_CONFIG_DIR`

#### Scenario: The wake mod in the folder

- **WHEN** the folder is read
- **THEN** its `mod` is the given Pending You's `mod`, copied whole

#### Scenario: A crew session's stop check

- **WHEN** the folder's `pendingyou-hook` is read
- **THEN** for `stopcheck` with `CREW_AGENT` set it runs `pendingyou` with its arguments, sends its standard output to `/dev/null` and exits 0, and for every other call it `exec`s `pendingyou` with its arguments

### Requirement: The folder's step writes only what differs

The step SHALL write each file of the folder's source whole, with the source's mode made writable by its owner, 0755 where the source runs and 0644 where it does not, by a temporary file renamed over its path, only where no regular file there has the same bytes and mode, and SHALL replace a link standing at its path. It SHALL make the folder with mode 0700 where it is missing, and say one line for each file it wrote and nothing where none differed.

#### Scenario: A fresh home

- **WHEN** the step runs in a home with no `.config/pendingyou`
- **THEN** the folder is made with mode 0700, holds the four files and the wake mod's as regular files, the launcher and the helper 0755 and the rest 0644, and one line names each

#### Scenario: Nothing differs

- **WHEN** the step runs again with the same source
- **THEN** no file is written and nothing is said

### Requirement: The folder's step writes the wake mod's modules before its hooks

The step SHALL write the wake mod's `hooks/wake.js`, `hooks/register.js` and `hooks/hooks.json` before the folder's other files, in that order, as Pending You's `init` does, so a session that reloads the mod finds the modules its hooks name.

#### Scenario: The wake mod's order

- **WHEN** the step writes the wake mod into a fresh home
- **THEN** it writes `mod/hooks/wake.js`, `mod/hooks/register.js` and `mod/hooks/hooks.json` first, in that order

### Requirement: The folder's step touches nothing else of the folder and stops nothing

The step SHALL read, write and remove nothing under the folder but the source's files. A file it cannot write SHALL be said in one line naming it, and the step SHALL exit 0.

#### Scenario: The command line's own files

- **WHEN** the folder holds a sign-in, a key and a `permission-cards/` folder of the command line's
- **THEN** the step leaves each as it was

#### Scenario: A link where a launcher goes

- **WHEN** `bin/pendingyou-hook` is a link into a store
- **THEN** the step replaces it with a regular file holding the source's bytes

#### Scenario: A file it cannot write

- **WHEN** the folder's `bin` cannot be written
- **THEN** one line names the file not written, and the step exits 0

### Requirement: The modules write none of Claude Code's files

No activation step, launchd job or systemd unit of the modules SHALL run `claude`, name a Claude Code home or write a file of Claude Code's, whatever the host turns on.

#### Scenario: No Claude Code of the modules'

- **WHEN** a system turns on `services.boomdrive` and `services.crew`
- **THEN** no activation step, launchd job or systemd unit of the system runs `claude` or names `.claude`, `CLAUDE_CONFIG_DIR` or the set's marketplace, on nix-darwin and on NixOS

### Requirement: One sign-in command signs Claude Code's connection and herdr's in

`pendingyou-signin` SHALL export `PENDINGYOU_CONFIG_DIR` as `$HOME/.config/pendingyou`, run the given Pending You's `pendingyou login --app claude-code` with its arguments and `CLAUDE_CONFIG_DIR` the folder's `signin`, then run `pendingyou app login herdr` with its arguments but `--force`. No step of the modules SHALL run it.

#### Scenario: The sign-in's two commands

- **WHEN** `pendingyou-signin`'s script is read
- **THEN** given `--force` it runs `pendingyou login --app claude-code --force` with `CLAUDE_CONFIG_DIR` the folder's `signin`, then `pendingyou app login herdr` with no `--force`

### Requirement: The README names the companions and that no Claude Code home is installed into yet

The README's install with nix SHALL name Pending You, Plannotator and plannotator-tui in the set, and their herdr plugins. It SHALL say that they come with the runner where crew's module is on, and that the modules write Pending You's folder and leave its sign-ins, key and state the command line's. It SHALL name `pendingyou-signin` as the person's one sign-in, and say that the companions' Claude Code plugins are installed into no Claude Code home yet.

#### Scenario: The README's companions

- **WHEN** the README's install with nix is read
- **THEN** it names `pendingyou-signin`, `boomdrive-claude-plugins` and `~/.config/pendingyou`, and says that no Claude Code home is installed into yet
