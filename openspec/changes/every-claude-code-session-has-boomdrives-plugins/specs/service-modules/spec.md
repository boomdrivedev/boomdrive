## RENAMED Requirements

- FROM: `### Requirement: The README names the companions and that no Claude Code home is installed into yet`
- TO: `### Requirement: The README names the companions and the Claude Code homes their plugins are installed into`

## ADDED Requirements

### Requirement: With the runner and Flume, each activation installs the set's plugins into every Claude Code home of Flume's accounts

Where `services.boomdrive.enable` is on and `services.flume.enable` is on with a package, each activation SHALL run `activate.sh claude` once, as the runner's user, with `HOME` that user's home and `PATH` the service's profiles' folders, then `$HOME/.local/bin`, after Flume's own step, Pending You's folder and herdr's links. It SHALL be given the host's `flume`, the flake's `jq`, the set's marketplace, Pending You's server's file or `-`, and the plugins to install, then `--` and those to remove.

#### Scenario: The step on nix-darwin

- **WHEN** a nix-darwin system whose primary user is `alice`, at `/Users/alice`, turns on `services.boomdrive` and `services.crew`
- **THEN** its activation runs, as `alice` with `HOME` `/Users/alice`, `activate.sh claude` with the set's Flume, `jq`, the set's marketplace and Pending You's server's file, after Flume's link of its file, Pending You's folder and herdr's links, each once

#### Scenario: The step on NixOS

- **WHEN** a NixOS system turns on `services.boomdrive` and `services.crew` for `alice`
- **THEN** `alice`'s user activation runs `activate.sh claude` after Pending You's folder and herdr's links and before `keep`, the runner's user activation depends on Flume's, and another user's runs no such step

#### Scenario: No step without the runner

- **WHEN** a system imports the platform's module, turns on `services.flume` and leaves `services.boomdrive` off
- **THEN** its activation runs no `activate.sh claude`, on nix-darwin and on NixOS

#### Scenario: No step without Flume's package

- **WHEN** a system turns on `services.boomdrive` and sets `services.flume.package` to `null`
- **THEN** its activation runs no `activate.sh claude`, on nix-darwin and on NixOS

### Requirement: The plugins installed are those of the tools the host installs

The step SHALL be given to install `flume` where Flume's module is on with a package, `sluice` and `sluice-board` where sluice's is, `timber` where timber's is, and `pendingyou` and `plannotator` where crew's companions are installed with Pending You's and Plannotator's packages. It SHALL be given to remove every other plugin the set's marketplace lists.

#### Scenario: Every plugin with the runner and crew

- **WHEN** a system turns on `services.boomdrive` and `services.crew` and sets no package `null`
- **THEN** the step installs `flume`, `sluice`, `sluice-board`, `timber`, `pendingyou` and `plannotator`, and removes none, on nix-darwin and on NixOS

#### Scenario: No companions' plugins without crew

- **WHEN** a system turns on `services.boomdrive` and leaves `services.crew` off
- **THEN** the step installs `flume`, `sluice`, `sluice-board` and `timber`, removes `pendingyou` and `plannotator`, and is given no server, on nix-darwin and on NixOS

#### Scenario: A tool turned off

- **WHEN** a system with the runner and crew sets `services.timber.enable` to `false`
- **THEN** the step removes `timber` and installs the other five

### Requirement: The homes are each Claude Code account's home Flume's status gives, and the default home

The step SHALL read the homes from `<flume> --json status`: the `home` of each account of `.status.host.pools[].accounts[]` whose `program` is `claude` and which gives one, and Claude Code's default home, with `CLAUDE_CONFIG_DIR` unset. It SHALL install into each home once, the default home first, then the others in the order status gives them. It SHALL skip an account of another program. Where Flume's status cannot be read, it SHALL say so in one line and install into the default home alone.

#### Scenario: Two accounts and the default home

- **WHEN** status gives two `claude` accounts with homes `/scratch/a` and `/scratch/b`
- **THEN** the step runs Claude Code's commands with `CLAUDE_CONFIG_DIR` unset, then `/scratch/a`, then `/scratch/b`

#### Scenario: An account with no home

- **WHEN** status gives one `claude` account with no home and one with `/scratch/a`
- **THEN** the step installs into the default home once and into `/scratch/a`

#### Scenario: Two accounts sharing one home

- **WHEN** two `claude` accounts give the same home `/scratch/a`
- **THEN** the step installs into `/scratch/a` once

#### Scenario: An account of another program

- **WHEN** status gives an account whose `program` is not `claude`, with a home
- **THEN** the step runs no command in that home

#### Scenario: Flume's status cannot be read

- **WHEN** Flume exits other than 0
- **THEN** one line says Flume's status could not be read, and the step installs into the default home alone

### Requirement: In each home the step runs only Claude Code's own commands, for the set's marketplace

In each home, with `CLAUDE_CONFIG_DIR` set to it or unset for the default, the step SHALL run `claude plugin marketplace add <marketplace> --json` and `claude plugin list --json`. Then, each naming `<plugin>@boomdrive` with `--json`, it SHALL `install` each plugin to install the list lacks, `update --scope user` each it holds at another version than the marketplace's, and `uninstall` each to remove it holds. It runs no other `claude` command but Pending You's server's.

#### Scenario: A fresh home

- **WHEN** the step runs in a home whose list holds no plugin of `boomdrive`, with `flume` and `timber` to install
- **THEN** it runs the marketplace's add, the list, then the install of `flume@boomdrive` and of `timber@boomdrive`, and no update or uninstall

#### Scenario: Nothing differs

- **WHEN** the home's list holds `flume@boomdrive` and `timber@boomdrive`, each at the marketplace's version
- **THEN** it runs the marketplace's add and the list, and nothing else

#### Scenario: A new build of a plugin

- **WHEN** the home's list holds `timber@boomdrive` at `0.6.0` and the marketplace lists `timber` at `0.7.0`
- **THEN** it runs `claude plugin update timber@boomdrive --scope user --json`, and no install of it

#### Scenario: A plugin to remove

- **WHEN** `pendingyou` is to be removed and the home's list holds `pendingyou@boomdrive`
- **THEN** it runs `claude plugin uninstall pendingyou@boomdrive --json`, and no uninstall where the list does not hold it

#### Scenario: The home's own plugins

- **WHEN** the home's list holds `plannotator@plannotator` and `foo@other`
- **THEN** the step names neither in any command

### Requirement: Pending You's server is added where a home names none

Where the step is given Pending You's server's file, it SHALL run `claude mcp add-json --scope user pendingyou <json>` in each home whose own `.claude.json`, `$CLAUDE_CONFIG_DIR/.claude.json` or else `$HOME/.claude.json`, names no `mcpServers.pendingyou`. The JSON SHALL be the server Pending You's folder's `signin/.claude.json` names. The step SHALL never remove or rewrite a server.

#### Scenario: A home with no server

- **WHEN** the step is given the server and the home's `.claude.json` names no `pendingyou`
- **THEN** it runs `claude mcp add-json --scope user pendingyou` with the folder's server, after the plugins' commands

#### Scenario: A home that names one

- **WHEN** the home's `.claude.json` names `mcpServers.pendingyou`
- **THEN** it runs no `claude mcp` command there

#### Scenario: No server given

- **WHEN** the step is given `-` and the home's `.claude.json` names `pendingyou`
- **THEN** it runs no `claude mcp` command, and the server stays

#### Scenario: The server's file

- **WHEN** a system turns on `services.boomdrive` and `services.crew` for `alice`, at `/Users/alice`
- **THEN** the step's server file names `https://www.pendingyou.com/mcp` and the helper `/Users/alice/.config/pendingyou/bin/pendingyou-mcp-headers`, on nix-darwin and on NixOS

### Requirement: The step stops nothing

The step SHALL exit 0. Where it finds no `claude` on its `PATH`, it SHALL say so in one line and run nothing. Each command Claude Code refuses or fails SHALL be one line naming the home, the command and Claude Code's words, and the step SHALL go on with the next command. It SHALL say nothing where every command held.

#### Scenario: No Claude Code

- **WHEN** no `claude` is on the step's `PATH`
- **THEN** one line says so, no Flume or Claude Code command runs, and it exits 0

#### Scenario: A command refused

- **WHEN** Claude Code refuses `claude plugin install timber@boomdrive --json` in `/scratch/a` with the words `no such plugin`
- **THEN** one line names `/scratch/a`, the command and `no such plugin`, the step runs the commands after it, and exits 0

#### Scenario: Every command held

- **WHEN** every command answers ok
- **THEN** the step says nothing

### Requirement: Where Pending You's folder is written, the login environment puts its wake mod on CLAUDE_CODE_PLUGIN_DIRS

Where the modules write Pending You's folder, `environment.extraInit` SHALL hold the lines of `nix/modules/plugin-dirs.sh`, on nix-darwin and on NixOS. For a user whose `$HOME/.config/pendingyou/mod` is a folder, they SHALL put that folder on `CLAUDE_CODE_PLUGIN_DIRS`, ahead of the paths already there and once, and export it. For a user with no such folder, they SHALL leave the environment as it is. Where the folder is not written, `extraInit` SHALL hold none of them.

#### Scenario: A fresh environment

- **WHEN** the lines run with the folder there and `CLAUDE_CODE_PLUGIN_DIRS` unset
- **THEN** `CLAUDE_CODE_PLUGIN_DIRS` is `$HOME/.config/pendingyou/mod` and is exported

#### Scenario: A person's own value

- **WHEN** they run with `CLAUDE_CODE_PLUGIN_DIRS` `/scratch/own`
- **THEN** it is `$HOME/.config/pendingyou/mod:/scratch/own`

#### Scenario: Run twice

- **WHEN** they run again in the environment they made
- **THEN** `CLAUDE_CODE_PLUGIN_DIRS` is unchanged

#### Scenario: No folder

- **WHEN** they run in a home with no `.config/pendingyou/mod`
- **THEN** `CLAUDE_CODE_PLUGIN_DIRS` is as it was

#### Scenario: The lines with the folder

- **WHEN** a system turns on `services.boomdrive` and `services.crew`
- **THEN** its `environment.extraInit` holds the lines, on nix-darwin and on NixOS

#### Scenario: No lines without the folder

- **WHEN** a system turns on `services.boomdrive` and leaves `services.crew` off, or sets `services.boomdrive.pendingyou.package` to `null`
- **THEN** its `environment.extraInit` holds none of the lines

## MODIFIED Requirements

### Requirement: The modules write no file of Claude Code's but its managed telemetry and transcripts documents

No activation step, launchd job or systemd unit of the modules SHALL write a file of Claude Code's, whatever the host turns on, but the managed telemetry document in Claude Code's managed-settings folder where the host names a collector, and the managed transcripts document there wherever Flume is on. None SHALL run `claude` but `activate.sh claude`, which runs only the `claude plugin` and `claude mcp add-json` commands its requirements name.

#### Scenario: No Claude Code of the modules'

- **WHEN** a system turns on `services.boomdrive` and `services.crew`, with a collector named and with none
- **THEN** no activation step, launchd job or systemd unit of the system runs `claude` or names `.claude` or `CLAUDE_CONFIG_DIR` but `activate.sh claude`'s step, which is run once, on nix-darwin and on NixOS

#### Scenario: The step's commands

- **WHEN** `activate.sh claude` is read
- **THEN** the only `claude` commands it runs are `plugin marketplace add`, `plugin list`, `plugin install`, `plugin update`, `plugin uninstall` and `mcp add-json --scope user pendingyou`

### Requirement: The Boomdrive module writes no file of the other tools'

The modules' own activation SHALL link the runner's file, write Pending You's folder where crew's companions are installed, link `services.boomdrive.herdr.plugins` by herdr's own `plugin link`, run `activate.sh claude` where the runner and Flume are on, write or remove Claude Code's two managed documents on nix-darwin, and do nothing else of a tool's. Each other tool's files, crew's source in Flume's folder and Flume's and sluice's `[telemetry]` among them, SHALL be written by its own module.

#### Scenario: The tools' activation is their own

- **WHEN** a system with `services.boomdrive` and crew turned on is set beside one that imports Flume's, sluice's, crew's and timber's own modules alone, each turned on with the same settings, packages and user, and Flume's with crew's source declared as the modules declare it
- **THEN** on nix-darwin their activations are the same but for the runner's own link, Pending You's folder, herdr's plugins' links, `activate.sh claude` and the managed telemetry and transcripts documents' steps, each run once, and on NixOS their user activations are the same but for the runner's own

#### Scenario: The tools' telemetry is their own

- **WHEN** a system with `services.boomdrive` and crew turned on names the collector `http://localhost:4318`, and is set beside one that imports Flume's, sluice's, crew's and timber's own modules alone, as above, with Flume's and sluice's `settings.telemetry` each `{ send = true; collector = "http://localhost:4318"; }`
- **THEN** on nix-darwin their activations are the same but for what the scenario above leaves aside, and on NixOS their user activations' Flume and sluice snippets are each the same

### Requirement: The README names the companions and the Claude Code homes their plugins are installed into

The README's install with nix SHALL name Pending You, Plannotator, plannotator-tui and their herdr plugins, which come with the runner where crew's module is on, and `pendingyou-signin`, the person's one sign-in. It SHALL say that each switch installs the set's marketplace's plugins, with Pending You's server, into every Claude Code home of Flume's accounts and the default home by Claude Code's own commands, and that the login environment loads the wake mod.

#### Scenario: The README's companions

- **WHEN** the README's install with nix is read
- **THEN** it names `pendingyou-signin`, `boomdrive-claude-plugins`, `~/.config/pendingyou`, `CLAUDE_CODE_PLUGIN_DIRS` and the Claude Code commands each switch runs in each home

#### Scenario: What the README says stays

- **WHEN** the README's install with nix is read
- **THEN** it says the modules write `~/.config/pendingyou` and leave its sign-ins, key and state the command line's, that each switch changes nothing else in a Claude Code home, and that a home's own install of one of these plugins from another marketplace stays until the person removes it
