# setup Specification

## Purpose
`boomdrive setup`, which sets up one drive of one account on a Mac Nix does not manage, by one command an agent can run, each question a flag: the account's crew state repository made private by gh where none exists, the drive named in crew's file by `crew drive set`, the runner's managed layer written by the keys Nix's module writes, the set's herdr plugins linked from the person's profile, and the runner loaded as a launchd agent of the login. Run again with the same answers it changes nothing, and a drive's repositories are added and removed by it later (record 0009 as amended 2026-11-09 and 2026-11-11; core.2, core.16, core.39).

## Requirements

### Requirement: Setup takes each answer as a flag

`boomdrive setup` SHALL set up one drive of one account on the host it runs on. Each answer SHALL be a flag: `--account`, `--drive`, `--state-repository`, `--repository` for each of the drive's repositories, `--add-repository`, `--remove-repository`, `--blueprints`, `--runs <role> <how>` and `--review <after_changes> <after_hours>`. `--repository` given with `--add-repository` or `--remove-repository` SHALL be a usage error (record 0009 as amended 2026-11-09; core.16).

#### Scenario: Every answer given

- **WHEN** `boomdrive setup --account own --drive home --state-repository example/state --repository agentplot/flume --repository agentplot/sluice --json` runs with no terminal on a scratch home
- **THEN** it sets up the drive `home` of the account `own`, keeping `agentplot/flume` and `agentplot/sluice`, and asks nothing

#### Scenario: The whole list beside a change to it

- **WHEN** setup runs with `--repository agentplot/flume --add-repository agentplot/sluice`
- **THEN** it exits 2, naming the two flags, and writes nothing

### Requirement: A terminal is asked what nothing else gives

Where standard input is a terminal, setup SHALL ask on standard error for each of the account, the drive, the state repository and the repositories that neither a flag nor the host gives, in that order, and read each answer from standard input, the repositories one a line until an empty one. Where it is not a terminal, a missing answer SHALL be a usage error naming each missing flag, and setup SHALL run nothing and write nothing.

#### Scenario: An answer missing with no terminal

- **WHEN** `boomdrive setup --account own --drive home --json` runs with no terminal on a home setup has not set up
- **THEN** it exits 2, naming `--state-repository` and `--repository` as missing, and runs no program and writes no file

#### Scenario: Asked on a terminal

- **WHEN** setup's answers are gathered with a terminal, the flags giving the account alone, and the answers read are `home`, `example/state`, then `agentplot/flume` and an empty line
- **THEN** it asks for the drive, the state repository and the repositories, in that order, and sets up the drive `home` keeping `agentplot/flume`

### Requirement: An answer not given is taken from the host

The account, the drive, its repositories and the state repository SHALL be taken from the runner's file, where setup wrote it and it names one drive. The blueprints, each role's runs and the review SHALL be taken from crew's file, where it names the drive. A `--runs` SHALL change its role alone. Where setup's file names more than one drive and nothing gives the drive, setup SHALL refuse as a usage error, naming the drives (record 0009 as amended 2026-11-11).

#### Scenario: A later setup keeps what crew's file holds

- **WHEN** setup runs with `--add-repository agentplot/crew` alone, where setup's runner file names the drive `home` and crew's file gives it the blueprints `agentplot/sluice`, the planner `on-demand` and the review `3 2`
- **THEN** crew is given the account, the drive and the state repository the runner's file names, its repositories with `agentplot/crew` after them, the blueprints `agentplot/sluice`, the planner `on-demand`, every other role as crew's file has it, and the review `3 2`

#### Scenario: One role changed

- **WHEN** setup runs with `--runs ops on-demand` alone on a host it set up with every role standing
- **THEN** crew is given `on-demand` for `ops` and `standing` for each other role

#### Scenario: Two drives and no drive named

- **WHEN** setup runs with no `--drive` and no terminal, where setup's runner file names the drives `home` and `work`
- **THEN** it exits 2, naming `--drive` and the drives `home` and `work`, and writes nothing

### Requirement: Setup's own answers stand where the host holds none

Where neither a flag nor the host gives them, the blueprints SHALL be the drive's first repository, each of the five roles `design`, `planner`, `reviewer`, `ops` and `operator` SHALL run `standing`, and the review SHALL be `5` changes or `4` hours (record 0009 as amended 2026-11-11).

#### Scenario: The first setup's own answers

- **WHEN** setup runs with the account, the drive, the state repository and the repositories `agentplot/flume` and `agentplot/sluice`, and no other answer, on a home setup has not set up
- **THEN** crew is given `agentplot/flume` as the blueprints, `standing` for each of the five roles and the review `5 4`

### Requirement: Each answer is checked before any program runs

Before it runs any program, setup SHALL refuse as a usage error, naming the flag and the value:

- an account or a drive not 1 to 32 lower-case letters, digits and `-`, starting with a letter;
- an address not `owner/name`;
- a blueprints repository not among the drive's repositories;
- a role other than the five, or a way of running other than `standing` or `on-demand`;
- a review whose changes are not a whole number of at least 1, or whose hours are not above 0.

#### Scenario: A blueprints repository the drive does not keep

- **WHEN** setup runs with `--repository agentplot/flume --blueprints agentplot/sluice`
- **THEN** it exits 2, naming `--blueprints` and `agentplot/sluice` as no repository of the drive, and runs nothing

#### Scenario: A role crew does not have

- **WHEN** setup runs with `--runs main-ops standing`
- **THEN** it exits 2, naming `main-ops` and the roles `design`, `planner`, `reviewer`, `ops` and `operator`

#### Scenario: A review of nothing

- **WHEN** setup runs with `--review 0 4`
- **THEN** it exits 2, naming `--review` and its changes as a whole number of at least 1

### Requirement: A drive's repositories are added and removed by setup

`--add-repository` SHALL add each after the drive's own, and `--remove-repository` SHALL remove each, from the repositories the host holds. One added that the drive holds, or removed that it does not, SHALL change nothing. A repository the runner's file keeps in another drive, or a drive left with none, SHALL be a usage error. Every step SHALL then run as for the whole list, and the runner SHALL let a removed one go by its own rules (core.5; record 0028).

#### Scenario: A repository added

- **WHEN** setup runs with `--add-repository agentplot/crew` on the drive `home` keeping `agentplot/flume` and `agentplot/sluice`
- **THEN** crew is given the three repositories, with `--repositories` for each, and the runner's file keeps the three in the drive `home`

#### Scenario: A repository removed

- **WHEN** setup runs with `--remove-repository agentplot/sluice` on the drive `home` keeping `agentplot/flume` and `agentplot/sluice`
- **THEN** crew is given `agentplot/flume` alone, and the runner's file keeps `agentplot/flume` alone in the drive `home`

#### Scenario: A repository added twice

- **WHEN** setup runs with `--add-repository agentplot/sluice` on a drive that keeps `agentplot/sluice`
- **THEN** it refuses nothing, and crew and the runner's file are given the drive's repositories as they were

#### Scenario: The last repository removed

- **WHEN** setup runs with `--remove-repository agentplot/flume` on a drive that keeps `agentplot/flume` alone
- **THEN** it exits 2, saying a drive keeps at least one repository, and writes nothing

### Requirement: Setup runs on macOS alone

Setup SHALL refuse on any system but macOS, as a host to fix, naming the system and saying a NixOS host is set up by the flake's module, `services.boomdrive`.

#### Scenario: On Linux

- **WHEN** setup's host is Linux
- **THEN** it is refused, naming `linux` and `services.boomdrive`, and runs no program

### Requirement: On a host Nix manages, setup writes nothing and says so

Where the runner's managed file is a link, as the modules' activation makes it, setup SHALL write nothing and run no program, and SHALL refuse, saying the file is Nix's and naming `services.boomdrive.accounts` as where a drive of this host is set (record 0009 as amended 2026-11-09).

#### Scenario: The runner's file is Nix's

- **WHEN** setup runs with every answer, where `~/.config/boomdrive/config.toml` is a link
- **THEN** it exits 1, naming the file and `services.boomdrive.accounts`, and the link, crew's file and `~/Library/LaunchAgents` are as they were, and no program is run

### Requirement: Each program setup runs is found before it runs one

Before it writes anything or asks anything of a forge, setup SHALL refuse as a host to fix each of `crew`, `gh`, `herdr`, `hostname` and `launchctl` that is not on its `PATH`, naming where it comes from. It SHALL also refuse a host whose name, as `hostname -s` gives it, is not one crew takes, lower-case letters, digits and `-`, naming the command that sets one.

#### Scenario: crew missing

- **WHEN** setup runs with every answer, and no `crew` is on its `PATH`
- **THEN** it exits 2, naming `crew` and `github:boomdrivedev/boomdrive#crew`, and runs no `gh`, writes no file and loads no agent

#### Scenario: A host name crew refuses

- **WHEN** setup runs with every answer, and `hostname -s` says `Pats-MacBook-Air`
- **THEN** it exits 2, naming `Pats-MacBook-Air` and `sudo scutil --set HostName`, and writes nothing

### Requirement: A person's own file is moved aside once and named

A file at the runner's managed path whose first line is not setup's SHALL be moved to `config.toml.before-setup` before setup writes its own, both paths named in its answer, as the modules move a person's file aside. Where that path is taken, setup SHALL refuse as a host to fix before anything is written, naming both and changing neither (record 0009 as amended 2026-11-09; core.46).

#### Scenario: A person's own file

- **WHEN** setup runs with every answer, where `~/.config/boomdrive/config.toml` is a file a person wrote
- **THEN** that file is at `config.toml.before-setup`, its text unchanged, setup's file is at `config.toml`, and the answer names both

#### Scenario: Both files there

- **WHEN** setup runs with every answer, where `~/.config/boomdrive/config.toml` is a file setup did not write and `config.toml.before-setup` is there
- **THEN** it exits 2, naming both paths, and changes neither and runs no `gh`

### Requirement: GitHub's sign-in is needed, and the others are named

Setup SHALL refuse as a host to fix, before it makes or writes anything, where `gh auth status --hostname github.com` exits other than 0, naming `gh auth login`. It SHALL read Claude Code's sign-in from `loggedIn` in `claude auth status --json`, and Pending You's from the `Sign-in:` line of `pendingyou status` saying `ok`. It SHALL name each missing one, with `claude auth login` or `npx -y pendingyou@latest init`, and go on.

#### Scenario: GitHub not signed in

- **WHEN** setup runs with every answer, and `gh auth status --hostname github.com` exits 1
- **THEN** it exits 2, naming GitHub's sign-in and `gh auth login`, and makes no repository, runs no `crew` and writes nothing

#### Scenario: Claude Code not signed in

- **WHEN** setup runs with every answer, and `claude auth status --json` gives `{"loggedIn":false}`
- **THEN** it sets up the drive, exits 0, and its answer names Claude Code's sign-in as missing with `claude auth login`

#### Scenario: Pending You not installed

- **WHEN** setup runs with every answer, and no `pendingyou` is on its `PATH`
- **THEN** it sets up the drive, exits 0, and its answer names Pending You's sign-in as missing with `npx -y pendingyou@latest init`

### Requirement: The state repository is made where none exists

Setup SHALL read the state repository with `gh repo view <owner/name> --json name`. Where gh says it cannot resolve the repository, setup SHALL make it with `gh repo create <owner/name> --private`. Any other failure of either SHALL refuse setup with gh's words, before crew is run or the runner's file written (record 0009 as amended 2026-11-09; crew record 0065).

#### Scenario: No state repository

- **WHEN** setup runs with every answer, and gh says it cannot resolve `example/state`
- **THEN** it runs `gh repo create example/state --private`, and its answer says the state repository was made

#### Scenario: The state repository there

- **WHEN** setup runs with every answer, and `gh repo view example/state --json name` exits 0
- **THEN** it runs no `gh repo create`, and its answer says the state repository was there

#### Scenario: The forge refuses

- **WHEN** setup runs with every answer, and `gh repo create` exits 1 saying `HTTP 403`
- **THEN** it exits 1 with gh's words, runs no `crew`, writes no runner file and loads no agent

### Requirement: crew names the drive by its own command

Setup SHALL run `crew drive set <drive> --json` once, with, in this order: `--state`, each `--blueprints`, `--machine <host>`, `--hosts <host> <host> darwin`, `--session <drive>`, `--folder <repository root>`, each `--repos`, `--accounts <role> <account>` and `--runs <role> <how>` for each of the five roles, `--review`, and `--repositories <repository> <clone's folder> <view>` for each repository. The host SHALL be as `hostname -s` gives it (crew record 0065; core.57).

#### Scenario: crew's flags

- **WHEN** setup runs with the account `own`, the drive `home`, the state repository `example/state` and the repositories `agentplot/flume` and `agentplot/sluice`, on the host `host-a` whose home is `/h`
- **THEN** crew is run once as `crew drive set home --json` with `--state example/state`, `--blueprints agentplot/flume`, `--machine host-a`, `--hosts host-a host-a darwin`, `--session home`, `--folder /h/Code`, both `--repos`, `--accounts <role> own` and `--runs <role> standing` for each role, `--review 5 4`, and `--repositories agentplot/flume /h/.local/share/boomdrive/repositories/agentplot/flume /h/Code/agentplot/flume` and its like for `agentplot/sluice`

### Requirement: crew is never given --flume, and its answer is carried

Setup SHALL pass crew no `--flume`, `--roles` or `--console`. It SHALL carry crew's answer as crew printed it. Where crew exits other than 0, setup SHALL refuse with crew's words, and SHALL write no runner file and load no agent (crew record 0064 as amended 2026-11-09; core.1, core.11).

#### Scenario: No --flume for any role

- **WHEN** setup runs with `--runs design on-demand` and every other role standing
- **THEN** crew's arguments hold no `--flume`, `--roles` or `--console`

#### Scenario: crew's answer carried

- **WHEN** crew answers `{"label":"home","teams":{"outcome":"written"},"state":{"outcome":"made"}}`
- **THEN** setup's answer holds that object as crew printed it

#### Scenario: crew refuses

- **WHEN** crew exits 1 saying `the teams file crew drive set would write is refused`
- **THEN** setup exits 1 with crew's words, and writes no runner file and loads no agent

### Requirement: The runner's file holds the keys Nix writes

Setup SHALL write the runner's managed layer, `config.toml`, holding `accounts`, with each account, drive and repository setup has set up, and `[control_plane]`, with `type` the implementation's word and its own table: crew's `host`, and a partition for each drive with its `label` and `state_repository`. Past its first line, it SHALL be byte for byte the file Nix's module renders for the same accounts and control plane (record 0009 as amended 2026-11-09).

#### Scenario: One drive under crew

- **WHEN** setup runs with the account `own`, the drive `home`, the state repository `example/state` and the repositories `agentplot/flume` and `agentplot/sluice`, on the host `host-a`
- **THEN** the runner's file, past its first line, is `nix/tests/files/setup.toml` past its first line, byte for byte

#### Scenario: A second drive

- **WHEN** setup, having set up the drive `home` of `own`, runs for the drive `work` of `own` keeping `example/app`
- **THEN** the runner's file keeps the drives `home` and `work`, each with its repositories, and crew's table names both as partitions

### Requirement: Setup knows its own file and writes it only where it changed

The runner's file's first line SHALL be `# Written by boomdrive setup; a change goes through boomdrive setup.`, by which setup knows its own file. Setup SHALL write it whole, by a file renamed into place, only where its bytes differ, and read it back as the runner reads it before it loads the agent. It SHALL leave `config.local.toml` as it is (core.17, core.18).

#### Scenario: Nothing changed

- **WHEN** setup runs again with the answers it was given before
- **THEN** the runner's file is not written, its modified time as it was, and its answer says it is unchanged

#### Scenario: A local layer beside it

- **WHEN** setup runs where `config.local.toml` holds a repository
- **THEN** `config.local.toml` is as it was, its text unchanged

### Requirement: herdr's plugins are linked from the person's profile

Setup SHALL link, by `herdr plugin link <folder>`, each folder under `share/herdr/plugins` of `~/.nix-profile`, else `~/.local/state/nix/profile`, that no plugin of `herdr plugin list --json` has as its root. It SHALL run herdr with no `HERDR_` variable and standard input closed, as the modules' activation does, and SHALL write no file of herdr's (record 0034, point 1; core.29).

#### Scenario: Both plugins linked

- **WHEN** setup runs with every answer, the profile holding `share/herdr/plugins/flume` and `share/herdr/plugins/sluice`, and herdr listing neither
- **THEN** herdr is run as `herdr plugin link` once for each folder, and the answer says each was linked

#### Scenario: A plugin already linked

- **WHEN** herdr lists a plugin whose root is the profile's `share/herdr/plugins/flume`
- **THEN** only `share/herdr/plugins/sluice` is linked, and the answer says `flume` was there

### Requirement: A plugin missing or refused is named, and setup goes on

herdr's refusal of a link SHALL be named with herdr's words. Where no profile holds `share/herdr/plugins`, setup SHALL name the plugins as missing, with `github:boomdrivedev/boomdrive#herdr-plugins`. In either case setup SHALL go on.

#### Scenario: No plugin in the profile

- **WHEN** setup runs with every answer, and no profile holds `share/herdr/plugins`
- **THEN** no `herdr plugin link` is run, it exits 0, and its answer names the plugins as missing with `github:boomdrivedev/boomdrive#herdr-plugins`

### Requirement: The runner runs as a launchd agent of the login

Setup SHALL write `~/Library/LaunchAgents/io.github.agentplot.boomdrive.plist`. Its program SHALL wait on `/bin/wait4path /nix/store`, then `exec` `boomdrive run --config <the runner's file>`. It SHALL start at load, be kept alive at most once in 30 seconds, run as interactive, leave its process group when stopped, have `HOME` and a fixed `PATH`, and log to `~/Library/Logs/boomdrive.log`, as nix-darwin's agent does (record 0012).

#### Scenario: The agent

- **WHEN** setup runs with every answer, for a home `/h`
- **THEN** the plist names the label, the program `/bin/sh -c '/bin/wait4path /nix/store && exec boomdrive run --config /h/.config/boomdrive/config.toml'`, `RunAtLoad`, `KeepAlive`, `ThrottleInterval` 30, `ProcessType` `Interactive`, `AbandonProcessGroup`, `HOME` `/h`, `PATH` `/h/.nix-profile/bin:/h/.local/state/nix/profile/bin:/nix/var/nix/profiles/default/bin:/h/.local/bin:/opt/homebrew/bin:/usr/local/bin:/usr/bin:/bin:/usr/sbin:/sbin`, and the log `/h/Library/Logs/boomdrive.log`

### Requirement: The agent is loaded, and loaded again only where it changed

Setup SHALL write the plist only where its bytes differ. It SHALL run `launchctl bootstrap gui/<uid> <plist>` where `launchctl print gui/<uid>/io.github.agentplot.boomdrive` finds no agent. Where an agent is loaded and the plist changed, it SHALL first run `launchctl bootout` on it. Where the plist is unchanged and the agent loaded, it SHALL run neither. A refusal of `launchctl`'s SHALL refuse setup with its words (core.13).

#### Scenario: First loaded

- **WHEN** setup runs with every answer, and `launchctl print` finds no agent
- **THEN** it runs `launchctl bootstrap gui/<uid>` with the plist and no `bootout`, and its answer says the agent was loaded

#### Scenario: Changed and loaded

- **WHEN** setup runs where the plist there differs from the one it renders, and `launchctl print` finds the agent
- **THEN** it runs `launchctl bootout gui/<uid>/io.github.agentplot.boomdrive`, then `launchctl bootstrap`, and its answer says the agent was reloaded

#### Scenario: Loaded and unchanged

- **WHEN** setup runs again with the answers it was given before, and `launchctl print` finds the agent
- **THEN** it runs no `bootstrap` and no `bootout`, the plist is not written, and its answer says the agent was running

### Requirement: Setup answers in one object

With `--json`, setup SHALL print one object, `{"set_up": {…}}`. It SHALL hold `account`, `drive`, `repositories`, `control_plane`, `config`, `service`, `plugins` and `missing`. `control_plane` is the implementation's answer: crew's holds `state_repository`, `drive`, crew's answer, and `start`, `crew main up <drive>`. Each of the others holds its file or folder with its outcome, and `missing` holds each with its fix. Without `--json`, setup SHALL say one line a step, ending with `start`.

#### Scenario: The answer

- **WHEN** setup runs with every answer and `--json` on a scratch home with both plugins in its profile
- **THEN** it prints one object `{"set_up": …}` with the account, the drive, the repositories, crew's answer, `start` `crew main up home`, `config`, `service`, both plugins linked and nothing missing, and exits 0

### Requirement: Setup's exit says how it ended

Setup SHALL exit 0 answered, a missing sign-in, plugin or plugin link among its answer. It SHALL exit 1 refused, on a host Nix manages or where gh, crew or `launchctl` refused, and 2 for a usage error or a host to fix. A refusal under `--json` SHALL be `{"refused":{"refusal":{"reason":…}}}` on standard output (core.39).

#### Scenario: A refusal in JSON

- **WHEN** `boomdrive setup --json --account Own --drive home --state-repository example/state --repository agentplot/flume` runs
- **THEN** it prints `{"refused":{"refusal":{"reason":…}}}`, the reason naming `Own`, and exits 2

### Requirement: Run again, setup changes nothing

Run again with the answers it was given, setup SHALL write no file, make no repository, link no plugin and load no agent, and SHALL run crew with the same flags (record 0009 as amended 2026-11-09).

#### Scenario: Run again

- **WHEN** setup runs twice with the same answers
- **THEN** the second run makes no repository, writes neither the runner's file nor the plist, links no plugin, runs no `bootstrap` or `bootout`, and runs crew with the same flags as the first

### Requirement: Only crew's implementation sets up a drive

The steps of setup that name crew, its state repository, `crew drive set`, crew's file and the command that starts its agents, SHALL be crew's implementation's, `boomdrive-crew`, reached through the binary's list of implementations as `enroll` is. The binary's other code SHALL name none of them (core.2).

#### Scenario: Only crew's crate names crew

- **WHEN** the files of `crates/boomdrive/src` but `planes.rs` are read
- **THEN** none names `boomdrive_crew`, and `planes.rs` names `boomdrive_crew`'s setup
