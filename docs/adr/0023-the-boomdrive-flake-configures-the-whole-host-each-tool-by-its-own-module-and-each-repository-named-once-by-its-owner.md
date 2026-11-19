---
number: 23
title: The Boomdrive flake configures the whole host, each tool by its own module, and each repository named once by its owner
status: accepted
date: 2026-11-06
decision-makers:
- Chuck Swanberg
- the design agent
---

# The Boomdrive flake configures the whole host, each tool by its own module, and each repository named once by its owner

Status: accepted by the user on 2026-11-06, in the design agent's pane: "Other than that nitpick I approve them all" (the nitpick being flume record 0055's).

Amends record 0020 in part, and record 0012 where it keeps herdr the host's own, each marked so where it stands.

Deciders: the user, in the research talk `research--the-flakes-configuration-holds-streams-and-repositories` on 2026-11-06 (agentplot/boomdrive's `design/research/the-flakes-configuration-holds-streams-and-repositories.md`, merged with this record), who asked for it, "it just has one flat list of repos, but we should be able to configure the streams and the repos as well", and settled it: "I thought we were going to create one flake for people to use from Boom Drive so they didn't have to set up two flakes. [...] I agree that you should set up a flake for Flume as well so Flume could be used independently but I want to make sure that it's a one-stop shop with the Boom Drive flake"; "I want to make sure the flake also packages and configures crew and sluice"; a repository's stream, "One block by owner"; Flume's module, "Flume's flake, re-exported"; its accounts, "Yes, all of Flume's file"; herdr, "In the set, overridable"; the way in, "An app to click" and "A shell"; the terminal app, "Install Ghostty too"; and crew's partitions, "Yes, derive it". The design agent for writing it into this repository's records and rules.

Sources: `design/the-flakes-configuration/design.md` and `host.nix` (the talk's design and its sketch of a host), merged with this record; records 0009, 0012, 0013 and 0020; rules core.5, core.13, core.17, core.18, core.29, core.44 and core.46; flume record 0055 (a repository named by its address; Flume's own flake and module; Flume reading usage itself), proposed with this one; Flume's statement 143, 186 and C.7; a host's Flume configuration as afterthought/swancloud writes it.

## Context and problem statement

The flake's modules write the runner's file alone (record 0020): one flat list of repositories, and nothing of Flume's streams, sluice's host settings or crew's partitions. A host with several streams writes each repository in the runner's list and again, by its folder, in Flume's file, each by a different writer. herdr and the terminal app are the host's own, so a fresh Mac has no way to see its agents after the install.

## Considered options

1. **The flake configures the whole host.** One flake brings the runner, Flume, sluice and crew, each configured by its own module, which the Boomdrive flake re-exports from the tool's own flake; each repository is written once, under its owner, with the owner's stream; herdr and Ghostty come in the set, with a Boomdrive app and a shell to open the agents.
2. The runner's flat list, beside a block of Flume's own naming each repository's stream. Each repository written twice.
3. The Boomdrive flake owning every tool's module. A tool used without Boomdrive has no module.

## Decision outcome

Option 1.

- **One flake.** A host adds the Boomdrive flake alone. Its modules bring the runner, Flume, sluice and crew, each packaged and configured. Flume, sluice and crew each publish a flake with their own module, so each can be used without Boomdrive; the Boomdrive flake takes each as a locked input and re-exports its module, and carries a tool's module itself only until that tool's flake gives one, as it carries Flume's package today.
- **Each tool's module writes its own file.** `services.flume.settings` is Flume's file, `services.sluice.settings` sluice's host settings, `services.crew.settings` crew's and `services.boomdrive.settings` the runner's, each key named as its file names it. No module writes another tool's file; the Boomdrive module sets the other tools' module options.
- **Each repository named once, by its owner.** A host writes each owner once, with its repositories' names and, where its work runs on this host, its stream: `services.boomdrive.repositories.<owner> = { stream = "<stream>"; names = [ ... ]; }`. The module writes each address into the runner's `repositories` and gives each address its owner's stream in Flume's module. An owner with no stream gives Flume no rule. The person writes no folder. (Amended in part by record 0028, accepted by the user on 2026-11-07: each repository is named under its account's drive and its owner, and the drive gives the stream.)
- **herdr and Ghostty are in the set.** The flake installs herdr at the version Flume's terminal profile was checked against, with Flume's and sluice's herdr plugins, and Ghostty, which the module names Flume's `terminal` and `terminal_app` where the host names neither. Each is overridable: a host replaces either through the modules' options, or drops it for its own.
- **worktrunk and portless are in the set.** (Amended 2026-11-08, the user's choice on their card "Boomdrive flake: beyond Pending You and Plannotator, which of your tools should it also install?": "worktrunk (wt), portless (the .localhost dev addresses)".) The flake installs worktrunk's `wt`, which crew and sluice's worktree driver call, and portless, which gives each worktree's dev server a `.localhost` address and whose routes crew's board reads. Each is the set's by default and overridable as herdr is: a host replaces either through the modules' options, or drops it for its own. Claude Code stays the person's own, installed and updated as they chose: the flake installs none, and writes none of its files.
- **One way in.** The module installs a Boomdrive app, in `/Applications/Nix Apps` on macOS and as a desktop entry on Linux, which runs `flume open`, and the flake gives a shell, its `PATH` the set's tools, that starts herdr on entering. Each runs at the person's asking: the runner still starts no multiplexer (core.13) and names none (core.29).
- **crew's partitions are written once, in crew's module.** `services.crew.settings.partitions` lists every partition with its `machine`, and `services.crew.host` names this host to crew. Where the control plane is crew and no `[control_plane.crew]` is written, the Boomdrive module writes it: the host from `services.crew.host`, and each partition whose `machine` is this host. A table written out wins. The runner's file still names its partitions (core.17), and neither the runner nor crew changes.
- **A laptop** writes `services.boomdrive.repositories.<owner>.names` and nothing else: no stream, no pool, every place in Flume's default stream under the login's own program.

Not taken: the runner reading crew's `teams.json`, which core.17 rules out and which ties the runner to crew's file versions; the runner reporting each clone's folder to Flume, which Flume naming a repository by its address (flume record 0055) makes needless.

### Consequences

- Rules core.18 and core.46 say a tool's file is written by its own module, and the Boomdrive module sets the others' options; core.44's details take herdr and Ghostty into the set; core.5's details say the owner block's stream is no key of the runner's file. 
- The configuration and service-modules specs change through an OpenSpec change.
- The queued units of the bolt `clones-are-found-from-boomdrive`, which give Flume each clone's folder, are no longer needed once Flume names a repository by its address.
- Flume publishes a flake with its `services.flume` module (flume record 0055), sluice a module for its host settings, and agentplot/crew a flake with its package and a module for its host file.
- afterthought/swancloud, once the modules land, passes its hosts' facts as option values and stops writing Flume's, sluice's and crew's files.

## Amendment, 2026-11-08: how the module writes the runner's crew table

Status: decided on 2026-11-08 by the construct stage of the change `crews-partitions-give-the-runner-its-crew-table`. These are the writing agent's choices, made within this record and rules core.17, core.18 and core.19, each cheap to change. The change's design gives the path not taken for each, in its Decisions 1 to 6.

- **Only where crew's module is on.** The module writes `[control_plane.crew]` where `control_plane.type` is `crew`, `services.crew.enable` is true and the host writes no table. Not taken: reading crew's `host` and `partitions` wherever they are defined, crew on or off. A host that keeps crew off writes its table out, as the change `the-flake-configures-each-tool-by-its-own-module` has it in its Decision 5.
- **What of each partition.** The table holds each partition whose `machine` is `services.crew.host`, in crew's order. Each is its `label`, and its `state` as `state_repository`, and nothing else of it. Not taken: the fleet's every partition. The runner would then claim another machine's launches that name no host.
- **Whole.** A table the host writes is the file's table, whole. None of crew's partitions is merged into it. Partitions are added through the local layer, whose `partitions` replace the managed layer's (core.17).
- **None beside another control plane.** Under a hosted control plane, or with none, no crew table is written, whatever crew's module holds (core.19).
- **Two failures at evaluation.** With the runner enabled, evaluation fails in two cases. The first is under crew with no table written and crew's module off. The second is where the module would write the table and no partition names this host. Each failure names the options to set. Not taken: the runner's refusal at start, which it repeats under the service manager while the switch that wrote the file succeeds.
- **Where it lives.** The table is the default of `settings.control_plane.crew`, so `services.boomdrive.settings` reads what the file holds. Not taken: a fill at render time, which `settings` would not show.

## Amendment, 2026-11-11: each drive opens from the launcher

Status: decided on 2026-11-11 by the construct stage of the change `each-drive-opens-from-the-launcher`. Its brief asks: "A person opens any drive's agents from the Boomdrive launcher, one entry per drive of the host, each showing that drive's stream with flume open: a menu-bar menu on macOS and one action per drive in the desktop entry on Linux." The brief's ask is carried as given. The rest are the writing agent's choices, made within this record, record 0028 and Flume's statement 186, each cheap to change. The change's design gives the path not taken for each, in its Decisions 1 to 6.

Amends "One way in" above, where the launcher runs `flume open` with no session.

- **An entry for each drive of this host.** Each drive whose `machine` is unset or is this host is one entry, titled its name, in the order of the drives' names. It opens its stream's own session, as Flume names it: the stream's `session` in Flume's settings, else the stream's name (Flume's 107 and 186). A drive of another machine has no entry, since it has no stream here (record 0028). Not taken: the stream's name passed to `flume open`, which Flume refuses where the stream names a session.
- **And Flume's default.** One more entry, `Default stream`, runs `flume open` with no name, as the launcher did before. On Linux it is the desktop entry's own click. Not taken: the drives alone, which leave the desktop entry's click, and a host with no drive here, nothing to open.
- **One script runs Flume.** The launcher's script runs `flume open -- <session>` given a session, and `flume open` given none. It keeps the user's profiles on `PATH` and shows Flume's refusal, and the menu and the desktop entry run it and nothing else.
- **On macOS, a menu-bar app.** `Boomdrive.app` becomes a menu-bar app, compiled from one Objective-C file of this tree by the flake's own nixpkgs. Its status item `Boomdrive` holds each drive, `Default stream` and `Quit Boomdrive`, and its drives are in its `Info.plist`. It runs from the person's opening it until they quit it. No module or service starts it, at login or otherwise. Not taken: Swift, JavaScript for Automation run by `osascript`, a Rust crate, and a third-party menu-bar app.
- **On Linux, the desktop entry's actions.** `boomdrive.desktop` gains one action for each drive, which runs the script with the drive's session.
- **The drives are given to the launcher's package.** The module installs the set's launcher made for this host's drives by its override, with the flake's own nixpkgs. Its compiled menu is the set's path the cache holds, and what is the host's is text and copies. A host's own launcher is installed as given. Not taken: a file of drives that the launcher reads at its start, which a desktop entry's actions cannot read.
- **clang-format formats the menu's source**, pinned by `devenv.nix` and checked by `check-toolchain.sh` (core.7, core.8). Not taken: the file excluded from formatting by name.

core.13's, core.44's, core.46's and core.47's details are amended in this amendment's commit with what they allow.

## Amendment, 2026-11-11: how portless comes with the flake

Status: decided on 2026-11-11 by the construct stage of the change `portless-comes-with-the-flake`. These are the writing agent's choices, made within this record's amendment of 2026-11-08, which puts portless in the set, and rules core.13, core.44 and core.46. Each is cheap to change. The change's design gives the path not taken for each, in its Decisions 1 to 7.

- **From its source at its release tag.** portless publishes no flake, and nixpkgs carries none. So a recipe of this flake's builds its command line, the workspace's `packages/portless` alone, from vercel-labs/portless's source at its release tag, which the lock names by its hash. It builds with the flake's nixpkgs' pnpm 11, its dependencies one fetch the same on every system, and runs on that nixpkgs' Node.js 24. Not taken: the npm package's archive, whose `dist` is a build someone else made, which core.44 rules out where the flake can make the derivation.
- **git stays the host's own.** The recipe gives portless openssl, ps, which and hostname after the host's `PATH`, and no git: portless runs the host's git.
- **Installed with the runner, as herdr is.** `services.boomdrive.portless.package` is the set's by default. A host's own replaces it and `null` drops it, and without the runner nothing is installed.
- **The proxy and its trust stay the person's.** The modules start no proxy of portless's and declare no service of it. They write none of its files and trust no certificate authority. portless starts its proxy at its first `portless run` at a terminal, or the person runs `portless service install` once. `portless trust` is theirs. Not taken: a launchd daemon and a systemd service of the module's running the set's proxy. That puts a root service on port 443 on every host that takes the flake, which is a host's own choice. An option a host turns on can add it later.
- **In the shell.** The flake's shell's `PATH` gains portless, beside crew, which reads its routes.

## Amendment, 2026-11-11: where the set's worktrunk comes from

Status: decided on 2026-11-11 by the construct stage of the change `worktrunk-comes-with-the-flake`. These are the writing agent's choices, made within the amendment of 2026-11-08 above and rules core.18, core.44 and core.46, each cheap to change. The change's design gives the path not taken for each, in its Decisions 1 to 4.

- **Through its own flake, at the release sluice checked.** worktrunk comes from its own flake at a release tag, following the flake's `nixpkgs` and `rust-overlay`. The tag is the worktrunk sluice's `devenv.nix` pins at the tag the lock names for sluice, the release its worktree driver's tests drive, and the flake's test holds it there. Not taken: nixpkgs' worktrunk, which moves with nixpkgs; the release archive sluice's shell fetches, a prebuilt binary (core.44).
- **A host's tool, not a repository's pin.** crew and sluice's driver call the host's `wt`. sluice's pin is read to choose the release, as Flume's pin of herdr is, and makes worktrunk no tool a repository pins (record 0009 as amended on 2026-11-05).
- **Its tag as its `git describe`.** worktrunk's flake gives its build the commit's short revision, so its `wt --version` names a commit. The set gives the build its tag, as worktrunk's release build gets it from git, so `wt --version` says the tag's version (core.44). Not taken: worktrunk's flake's package unchanged.
- **The Boomdrive module's own option.** `services.boomdrive.worktrunk.package` installs it with the runner, the set's by default, a host's own replacing it and `null` dropping it, since worktrunk gives no nix-darwin or NixOS module. The modules write no file of worktrunk's, its shell integration among them. Not taken: worktrunk's home-manager module, which system modules cannot import.
- **In the flake's shell.** The shell holds worktrunk beside crew and sluice, which call it. Not taken: leaving it out, as Ghostty is.

## Amendment, 2026-11-11: how Pending You and Plannotator come with the flake

Status: agentplot-design's rulings, the user's answer and the construct stage's choices of 2026-11-11, written as record 0044, each cheap to change.

- **Record 0044 holds them.** The set brings Pending You, Plannotator, plannotator-tui and their herdr plugins, Pending You's own folder, one sign-in command and a marketplace of the set's own holding Pending You's and Plannotator's Claude Code plugins, with the runner where crew's module is on.
- **Claude Code stays the person's own.** The flake still installs no Claude Code and writes none of its files. Claude Code loads the companions itself, once the set installs them into its homes with Claude Code's own commands. That step is the unit `the-companions-load-in-claude-codes-sessions`, for every session on the accounts, the user's own included, as the user answered (record 0044, point 3).

## Amendment, 2026-11-13: timber comes with the flake

Status: decided on 2026-11-13 by the construct stage of the change `timber-comes-with-boomdrives-flake`. Its brief, which the planner queued from the user's ask of 2026-11-12, the first step of Timber's MVP, with no host having timber v0.6.0 yet, asks: "Boomdrive's flake carries timber, its CLI and its herdr and Claude Code plugins from one pin of timber's flake, as it carries sluice and flume, so every host the set configures has timber on its path." The brief's ask is carried as given. The rest are the writing agent's choices, made within this record, record 0020 and rules core.13, core.44 and core.46, each cheap to change. The change's design gives the path not taken for each, in its Decisions 1 to 9.

- **Through its own flake, at its release tag.** timber's flake is an input at `v0.6.0`, its newest release, following the flake's `nixpkgs` and `rust-overlay`, so the lock gains one node and the set keeps one nixpkgs. Its command line is that flake's package, unchanged: timber's tagged tree carries its version (timber's record 0031), so it reports its tag's. Not taken: timber's own nixpkgs and rust-overlay; its `main`; a release archive (core.44).
- **Its plugins from the same pin.** Its herdr plugin is its source's `plugins/herdr`, made as Flume's and sluice's are. Its Claude Code plugin, its source's `plugins/claude-code`, is a third plugin of the set's marketplace (record 0044 as amended on 2026-11-13). The flake's test holds both manifests to the set's timber's version. Not taken: timber's own marketplace carried as a second package.
- **A host's tool.** timber serves every checkout on a host from one process at one fixed address (timber's 114). No repository pins it, so record 0009's amendment of 2026-11-05 does not keep it out of the set.
- **The Boomdrive module's own option, with the runner.** `services.boomdrive.timber.package` installs it where the runner is enabled, whether or not crew is on. It is the set's by default, a host's own replaces it, and `null` drops it. timber gives no nix-darwin or NixOS module; once it gives one, the set re-exports it as it does Flume's, and the option goes. Not taken: installed only with crew, since a host under a hosted control plane lands work too; installed without the runner.
- **Its pane linked with Flume's and sluice's.** `services.boomdrive.herdr.plugins`' default gains timber's plugin after sluice's, whether or not crew is on. Not taken: linked only where the set's timber is installed, since a host's own timber wants the pane too.
- **Its serving process stays timber's.** The modules start no `timber serve` and declare no service of it, and write no file of timber's. timber's pane or command starts it where none answers, and a host may run it as a service of its own (timber's 114).
- **In the flake's shell,** beside the runner, Flume, sluice and crew.
- **Not in the quick start's Mac install, nor in `herdr-plugins`.** "Every host the set configures" is read as every host whose configuration takes the set's modules. A Mac set up by `boomdrive setup` adds `github:boomdrivedev/boomdrive#timber` by its profile line, as it adds worktrunk and portless. Not taken: timber in the quick start, whose steps run none of it; one line of each adds it later.

## Amendment, 2026-11-15: the set takes timber's own module

Status: decided on 2026-11-15 by the construct stage of the change `the-set-takes-timbers-own-module`, a unit of the bolt `a-hosts-set-takes-its-tools-new-releases`. Its brief asks: "Boomdrive's flake re-exports timber's own module as it does Flume's and sluice's, and services.boomdrive.timber.package goes." That is this record's amendment of 2026-11-13, "once it gives one, the set re-exports it as it does Flume's, and the option goes", now that timber's flake at `v0.7.0`, the tag the lock names, gives `darwinModules.timber` and `nixosModules.timber` (timber's record 0035; timber's clause 118 as amended at `boomdrivedev/blueprints` 573ec1a: "Boomdrive's set takes that module from timber's flake and writes no file of timber's"). The brief's ask is carried as given. The rest are the writing agent's choices, made within this record, timber's record 0035 and rules core.18 and core.46, each cheap to change. The change's design gives the path not taken for each, in its Decisions 1 to 7.

Amends the amendment of 2026-11-13 above, where the Boomdrive module's own option installs timber and the modules write no file of timber's.

- **Re-exported and imported, as Flume's, sluice's and crew's are.** The flake gives timber's module as `darwinModules.timber` and `nixosModules.timber`, and the platform's `boomdrive` module imports it after crew's. Its key, `github:boomdrivedev/timber#<platform>Modules.timber`, lets a system import it again from timber's own flake and declare `services.timber` once. No file of this tree declares an option of `services.timber`. Not taken: a copy of timber's module carried here (core.18).
- **Turned on with the runner, a host's `false` winning.** With `services.boomdrive.enable`, `services.timber.enable` is true by default, whether or not crew is on, so every host that has the runner keeps timber on its path. Not taken: left to the host, as crew's is, which would drop timber from every host at its next switch.
- **Its package the set's, whether or not the runner is enabled.** `services.timber.package` is the set's `timber` by default, a host's own or `null` winning, as Flume's, sluice's and crew's are. With the flake's `nixpkgs` and `rust-overlay` followed, it is the same derivation as timber's module's own default; set here, the cache's path holds even should the two part. Not taken: timber's module's default alone.
- **Its user the runner's.** On NixOS, `services.timber.user` is `services.boomdrive.user` by default, a host's own winning. On nix-darwin timber's module links for the primary user, the runner's own.
- **Its file is its module's.** With timber's module on, each switch links `~/.config/timber/config.toml` to the file `services.timber.settings` renders, its header alone where the host sets nothing, which binds as no file does (timber's 118). A file of the host's own there is moved once to `config.toml.before-nix`, as Flume's and sluice's are. A host that keeps its own file sets `services.timber.enable = false`, and installs its timber itself. Not taken: timber's module turned on only where a host names a setting, which would drop the package with it.
- **The option fails, naming what replaces it.** Setting `services.boomdrive.timber.package` fails evaluation, naming `services.timber.package` and `services.timber.enable`, as `services.boomdrive.tools` names Flume's and sluice's. Not taken: a renamed option that carries the value over, under which a host's `null` would install no timber while linking a file of timber's it never asked for.
- **Its pane and its Claude Code plugin stay as they are.** timber's module links no herdr plugin, so `services.boomdrive.herdr.plugins` keeps timber's, and the set's marketplace keeps timber's Claude Code plugin. Its serving process stays timber's: timber's module starts no `timber serve` and declares no service of it (timber's record 0035).

core.18's and core.46's details are amended in this amendment's commit with what they allow.

## Amendment, 2026-11-18: a host without the runner takes the tools it turns on

Status: decided on 2026-11-18 by the construct stage of the change `boomdrives-tools-install-on-a-machine-without-the-runner`, the unit of the bolt `every-machine-takes-boomdrives-tools`. A computer that does not run the runner can still install Boomdrive's tools and their herdr plugins from the Boomdrive flake, already built, turning on only the ones it wants. The brief asks: "A machine that runs no runner can still take Boomdrive's tools and their plugins from the Boomdrive flake, already built, turning on only the tools it wants", citing blueprints principles 4 and 17. The brief's ask is carried as given. The rest are the writing agent's choices, made within this record, record 0044 and those two principles, each cheap to change. The change's design gives the path not taken for each, in its Decisions 1 to 8.

Amends "herdr and Ghostty are in the set" and "worktrunk and portless are in the set" above, portless's "Installed with the runner, as herdr is" and worktrunk's "The Boomdrive module's own option" of 2026-11-11, timber's "Its pane linked with Flume's and sluice's" of 2026-11-13, and timber's "Turned on with the runner, a host's `false` winning" of 2026-11-15, where each tool comes with the runner alone; and record 0044's point 8.

- **Each tool has a switch of its own.** Flume, sluice, crew and timber are turned on by their own modules' `enable`, as before. herdr, Ghostty, the launcher, portless and worktrunk each gain `services.boomdrive.<tool>.enable`, and Pending You, Plannotator and plannotator-tui gain `services.boomdrive.pendingyou.enable`, `services.boomdrive.plannotator.enable` and `services.boomdrive.plannotator-tui.enable`. Where a switch is on, with the runner or without, the modules install that tool's `package`: the set's by default, a host's own as given, and none for `null`. Not taken: installing every package that is not `null` wherever the module is imported, which puts the whole set on every host that imports the module and turns nothing on; one switch for every tool without the runner, which is not "only the tools it wants"; options under `programs.<tool>`, apart from each tool's `package` option, which is under `services.boomdrive`.
- **The runner's switch turns them on by default.** With `services.boomdrive.enable`, Flume's, sluice's and timber's modules, herdr, Ghostty, the launcher, portless and worktrunk are on by default, and where crew's module is on too, the three companions, a host's own value winning each time. A host with the runner gets what it got before. Without the runner, every switch is off until the host turns it on.
- **A herdr plugin comes only beside the set's own build of its tool.** `services.boomdrive.herdr.plugins` holds by default the set's herdr plugin of each tool the set installs: Flume's, sluice's or timber's where that tool's module is on and its package is the set's, Pending You for Herdr where Pending You is on and the set's, and Annotate where plannotator-tui is on and the set's. That is principle 17: "a plugin's version is its CLI's by construction", on the user's words of 2026-11-12, "This is only acceptable if it means the flake is pinning the plugin to the same version of the CLI that's being installed." It replaces the amendment of 2026-11-13's choice to link timber's plugin beside a host's own timber, "since a host's own timber wants the pane too": such a host lists its own build's plugin, and a host's own list is linked as given. Not taken: each tool's plugin wherever herdr is on, whichever build of the tool the host runs.
- **The set's user, with the runner or without.** The plugins are linked wherever herdr is on, and Pending You's folder written wherever Pending You is on, each as the set's user: the primary user on nix-darwin, and `services.boomdrive.user` on NixOS. Without the runner, those are the only steps run as that user: no runner's file is linked, no agent or unit is declared, and no user lingers. Where such a step runs and no user is named, evaluation fails, naming the option to set. On NixOS, wherever the host names `services.boomdrive.user`, Flume's, sluice's, crew's and timber's `user` are it by default, a host's own winning. Not taken: a second user option for a host without the runner, since the one person the set is set up for is the same with the runner or without; each tool's own `user`, since herdr's plugins and Pending You's folder are no tool's module's.
- **Flume's terminal words follow the apps turned on.** Flume's `host.multiplexer` is `herdr` by default where herdr is on, and its `host.terminal_app` is `ghostty` where Ghostty is on, with the runner or without, and whether or not the host keeps the set's package of either.
- **The launcher holds no drive without the runner.** A drive gives Flume its stream only with the runner (record 0028), so without it the launcher is the set's own, holding `Default stream` alone.
- **Already built.** Every package a switch installs is the set's for the host's system, made with the flake's own nixpkgs, so its path is one the family's cache holds (core.47). No package is added. Whether to trust the cache stays the host's.
- **Claude Code's plugins.** Their install into Claude Code's homes stays record 0044 point 3's unit. Wherever the set installs a tool, with the runner or without, principle 17 has that unit give the tool's Claude Code plugin there too.
- **What stays with the runner.** The runner itself, its file and its service, its signed copy on macOS, the drives' streams in Flume, crew's table and kept repositories, Claude Code's telemetry document and Flume's and sluice's telemetry. Claude Code's transcripts document stays wherever Flume is on (record 0046).

core.18's and core.46's details are amended, and core.62 is written, in this amendment's commit.

## Amendment, 2026-11-18: Rex in the set

Status: decided on 2026-11-18 by the design agent, on afterthought-design's ask that the user wants to try Flume's Rex terminal and takes every tool only through this flake; its own choice, cheap to change.

- **Rex is in the set beside herdr.** Rex is the other terminal Flume carries a profile for (flume record 0068), and Flume drives it from the `PATH`. The set gives Rex from Rex's own flake (core.44), built into the family's cache, as it gives herdr and Ghostty.
- **At Flume's pin.** The set takes Rex at the revision the pinned Flume's Rex profile is tested against, read from that Flume tag, and moves it whenever the set moves Flume's pin, as it holds Ghostty at the release Flume's window is tested against. Not taken: Rex at a revision of the set's own choosing, which can drift from what Flume's Rex profile reads; waiting for Flume's flake to export Rex, which nothing else needs.
- **The host chooses its terminal.** A host that takes the set gets Rex installed; which terminal Flume opens stays the host's Flume settings.

## Amendment, 2026-11-18: how Rex comes with the flake

Status: decided on 2026-11-18 by the code stage of the fix `main--the-set-carries-rex-matched-to-flume`, which builds the amendment above through the OpenSpec change `the-set-carries-rex-matched-to-flume`. These are the writing agent's choices, made within the amendment above and rules core.13, core.18, core.44, core.45 and core.46, each cheap to change. The change's design gives the path not taken for each, in its Decisions 1 to 7.

- **By the URL Flume's tree locks.** `flake.nix` names Rex's flake by the URL its release keeps for good, the one Flume's `devenv.yaml` names at the tag the lock names for Flume, `https://pkgs.superlogical.com/rex/nix/preview/0.1.0-preview.14.tar.gz` at Flume v0.29.0, its `nixpkgs` following the flake's, and the lock locks it as Flume's `devenv.lock` does. Not taken: Rex's preview channel's URL, which says no release; Rex's own nixpkgs, a second nixpkgs in the lock.
- **Rex's flake's own package, unchanged.** The set's `rex` is Rex's flake's `rex` for each of the set's three systems, the three that flake packages: Rex's release binary, as Rex publishes no source, its version the release its URL names. Rex's flake allows Rex alone of unfree packages, in its own nixpkgs, so the set allows no other and asks no host to.
- **Held to Flume's `devenv.lock`.** The flake's test reads Flume's `devenv.lock` at the tag the lock names for Flume, and fails where `flake.nix` names another URL for Rex than its `original`, or the lock another URL or hash than it locks, naming both. A move of Flume's pin to a tag whose tree locks another Rex so fails the gate until the set's Rex moves with it. Not taken: reading `devenv.yaml`, which nix reads no YAML of; a version constant, as Ghostty's is, since Flume's tree pins its Rex where it pins no Ghostty.
- **The Boomdrive module's own option, with the runner.** `services.boomdrive.rex.package` installs Rex with the runner, the set's by default, a host's own replacing it and `null` installing none, as worktrunk's does. The modules name Rex in no setting of Flume's, so Flume's `host.multiplexer` stays `herdr` by default, write no file of Rex's and start no server of it. Not taken: Rex's own NixOS module, which runs Rex's server as a system service no tool of the set asks for, and has no nix-darwin counterpart; its home-manager module, which system modules cannot import, and which writes into a home.
- **In the flake's shell,** beside herdr, since Flume in the shell drives whichever terminal its settings name from the `PATH`. Not taken: leaving it out, as Ghostty is.
- **The boot asks `rex version`.** The booted system holds Rex, and Rex takes no `--version`, so the boot asks its version as `rex version`, as it asks crew's as `crew _release`.
- **Not in `herdr-plugins` or the quick start.** A Mac set up by `boomdrive setup` may add `github:boomdrivedev/boomdrive#rex` by its profile line, which the README names.

core.44's, core.45's and core.46's details are amended in this amendment's commit with what they allow.

## Amendment, 2026-11-18: Rex has a switch of its own

Status: decided on 2026-11-18 by the code stage of the fix `every-machine-takes-boomdrives-tools-1`, which merged `main`'s Rex into the bolt `every-machine-takes-boomdrives-tools`. The two amendments above of that day each decided alone: one that every tool of the set has a switch of its own, the other that Rex comes with the runner. core.62 decides between them, so this is the writing agent's reading of the written rules, cheap to change. The OpenSpec change `rex-has-a-switch-of-its-own` carries it.

Amends "The Boomdrive module's own option, with the runner" above.

- **Rex has a switch, as worktrunk has.** `services.boomdrive.rex.enable` is `services.boomdrive.enable` by default, a host's own value winning. Where it is on, with the runner or without, the modules install `services.boomdrive.rex.package`: the set's by default, a host's own as given, and none for `null`. Where it is off, they install no Rex. A host with the runner gets what `main` gave it. Not taken: Rex with the runner alone, which core.62 rules out as a tool only the runner's switch installs.
- **The rest stands.** The modules still name Rex in no setting of Flume's, write no file of Rex's and start no server of it, with the runner or without. Rex has no herdr plugin, so `services.boomdrive.herdr.plugins` does not change.

core.46's and core.62's details name Rex's switch in the merge commit that resolved the two amendments.
