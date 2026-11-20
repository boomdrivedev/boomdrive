# Design

## Context

The brief asks that a machine that runs no runner can still take Boomdrive's tools and their plugins from the Boomdrive flake, already built, turning on only the tools it wants. It cites blueprints principles 4 and 17:

- **Principle 4.** "Each tool is a plain command line a person can use alone. Installing the binary is all it needs."
- **Principle 17.** "Wherever a flake installs a tool's CLI, that flake gives the tool's plugins, its herdr plugin and its Claude Code plugin, built from the same revision. So a plugin's version is its CLI's by construction, and the person matches nothing by hand." It carries the user's words of 2026-11-12: "This is only acceptable if it means the flake is pinning the plugin to the same version of the CLI that's being installed."

As read in this tree at `2ea1a481` on 2026-11-18:

- **`nix/modules/tools.nix`** sets `services.flume.package`, `services.sluice.package`, `services.crew.package` and `services.timber.package` to the set's, whether or not the runner is on. With the runner it turns on Flume's, sluice's and timber's modules by default. So a host without the runner already takes those four by their own modules' `enable`, as the set's packages.
- **`nix/modules/apps.nix`** declares `services.boomdrive.herdr.package`, `herdr.plugins`, `ghostty.package`, `worktrunk.package`, `launcher.package` and `portless.package`. Its whole `config` sits under `lib.mkIf cfg.enable`: the five packages in the system profile, and Flume's `host.multiplexer` and `host.terminal_app`. `herdr.plugins` defaults to the set's Flume, sluice and timber plugins, then Pending You for Herdr and Annotate where crew's module is on, whatever build of each tool the host installs.
- **`nix/modules/companions.nix`** declares `services.boomdrive.pendingyou.package`, `plannotator.package` and `plannotator-tui.package`. It installs them, with `pendingyou-signin`, only where `services.boomdrive.enable` and `services.crew.enable` are both on.
- **`nix/modules/darwin.nix`** runs, under `lib.mkIf cfg.enable`, as `system.primaryUser` under their home, three steps in `postActivation`: the runner's file's link, Pending You's folder where crew is on and Pending You is not `null`, then herdr's links of the plugins. `system.requiresPrimaryUser` names `services.boomdrive.enable`. The managed documents' steps run as root, outside that `mkIf`.
- **`nix/modules/nixos.nix`** runs, under `lib.mkIf cfg.enable`, one user activation script, `boomdrive`, for `services.boomdrive.user` alone: the runner's file's link, Pending You's folder, herdr's links, then `keep`, which starts or restarts the runner's unit. It sets Flume's, sluice's, crew's and timber's `user` to the runner's user by default. `services.boomdrive.user` has no default.
- **`nix/modules/drives.nix`** gives Flume each drive's stream only with the runner.
- **The launcher's package** takes `drives ? [ ]`, so the set's `boomdrive-launcher` is the launcher made for no drive.
- **Flume's module**, at the tag the lock names, declares `services.flume.settings.host.multiplexer` and `host.terminal_app`, each `null` where unset.
- **nix-darwin's** `system.requiresPrimaryUser` is a list of option names. Evaluation fails, naming them, where it is not empty and `system.primaryUser` is `null`.

Rules relied on:

- **core.13:** the modules start no tool's process. herdr's `plugin link` starts no server.
- **core.18:** a tool's file is written only by its own module. The Boomdrive module writes Pending You's folder as Pending You's module would.
- **core.29:** the runner's code names no multiplexer. The modules' activation links herdr's plugins by herdr's own command.
- **core.44:** each tool comes from its own flake or source at its release tag. A plugin comes from the same pin as its tool.
- **core.46:** each tool's module writes its own file. The Boomdrive module's own options install the tools that have no module.
- **core.47:** a module's package is the flake's own, so its path is the one the family's cache holds.
- **core.63**, written with this change: the set installs a tool only where the host turns it on, by a switch of its own. The herdr plugins it links by default are those of the tools it installs, each beside the set's own build.
- **Blueprints principle 16:** no check reads Markdown.

## Goals / Non-Goals

**Goals:**

- A host that imports the platform's module without the runner installs each tool of the set it turns on, as the set's package, and nothing else.
- Each tool the set installs brings its herdr plugin from the same pin, linked wherever herdr is on, with the runner or without.
- A host with the runner gets the system it got before, unless it gives a tool a build of its own and names no plugins (Decision 3).

**Non-Goals:**

- **Installing Claude Code's plugins.** Installing the set's marketplace into Claude Code's homes stays record 0044 point 3's unit (Decision 8).
- **A profile with no module.** `nix profile add` of the set's packages is unchanged. The README shows it.
- **The runner's own parts.** Its file, its service, its signed copy, the drives' streams in Flume, crew's table and kept repositories, and the telemetry the set gives sessions and tools stay with the runner.
- **New packages.** The set's packages, the lock and the cache's workflow do not change.

## Decisions

### 1. A switch for each tool

Flume, sluice, crew and timber are turned on by their own modules' `enable`, as now. Each of the other tools gains a boolean option of the Boomdrive module, beside its `package`:

| tool | switch | on by default where |
|---|---|---|
| herdr | `services.boomdrive.herdr.enable` | `services.boomdrive.enable` |
| Ghostty | `services.boomdrive.ghostty.enable` | `services.boomdrive.enable` |
| the launcher | `services.boomdrive.launcher.enable` | `services.boomdrive.enable` |
| portless | `services.boomdrive.portless.enable` | `services.boomdrive.enable` |
| worktrunk | `services.boomdrive.worktrunk.enable` | `services.boomdrive.enable` |
| Pending You | `services.boomdrive.pendingyou.enable` | `services.boomdrive.enable` and `services.crew.enable` |
| Plannotator | `services.boomdrive.plannotator.enable` | `services.boomdrive.enable` and `services.crew.enable` |
| plannotator-tui | `services.boomdrive.plannotator-tui.enable` | `services.boomdrive.enable` and `services.crew.enable` |

Each default is the option's plain default, so a host's own value wins, with the runner or without. Its `defaultText` names the options it reads. Where a switch is on, the modules put its `package` in the system profile unless it is `null`, and `pendingyou-signin` beside a Pending You that is not `null`. Where it is off, they install nothing of that tool.

So a host with the runner, whose new switches all default as before, gets the same profile. A host without the runner gets nothing until it turns a switch on.

**Not taken:**

- **Every package not `null`, wherever the module is imported.** That puts the whole set on every host that imports the module and turns nothing on, such as afterthought/swancloud's Macs not flagged `runner`.
- **One switch for every tool without the runner.** That is not "only the tools it wants".
- **`programs.<tool>.enable`.** It parts each switch from its tool's `package`, which is under `services.boomdrive`, and a host would write one tool in two places.
- **`package = null` as the switch.** `null` already means "the host installs its own". Flume's terminal words, and herdr's links with a host's own herdr, read a tool as on while its package is the host's.

### 2. With the runner, as before

Each switch's default reads `services.boomdrive.enable`, and the companions' also `services.crew.enable`. That is the condition each tool is installed under today. tools.nix keeps turning on Flume's, sluice's and timber's modules with the runner, and leaves crew's to the host.

Without the runner, the modules turn on none of the four tools' modules. A host turns on each it wants by that tool's own `enable`, and its package is the set's, as tools.nix already sets it.

### 3. herdr's plugins come only beside the set's own build of their tool

`services.boomdrive.herdr.plugins`' default holds, in this order:

1. the set's `flume-herdr-plugin`, where `services.flume.enable` is on and `services.flume.package` is the set's `flume`;
2. the set's `sluice-herdr-plugin`, where sluice's module is on and its package is the set's;
3. the set's `timber-herdr-plugin`, where timber's module is on and its package is the set's;
4. the set's `pendingyou-herdr-plugin`, where `services.boomdrive.pendingyou.enable` is on and its package is the set's `pendingyou`;
5. the set's `plannotator-herdr-plugin`, where `services.boomdrive.plannotator-tui.enable` is on and its package is the set's `plannotator-tui`.

With the runner, no crew and every package the set's, that is Flume's, sluice's and timber's, as now. With crew too, it is all five, as now.

A host's own list is linked as given. A host that runs its own build of a tool and wants its pane lists its own build's plugin there.

This is principle 17: the plugin linked is the one built from the same pin as the command line installed. It replaces record 0023's amendment of 2026-11-13, which linked timber's plugin beside a host's own timber because "a host's own timber wants the pane too". That pairs the set's plugin with a command line of another version, which principle 17 rules out.

**Not taken:**

- **Each tool's plugin wherever herdr is on.** It links plugins whose tool the host does not have, or has at another version.
- **The plugin wherever the tool's module is on, whatever its package.** It keeps the mismatch above for a host's own build.

### 4. The set's user, with the runner or without

The steps that run as a user are herdr's links and Pending You's folder. They run wherever their tool is on, with the runner or without:

- **herdr's links** run where `services.boomdrive.herdr.enable` is on, `herdr.package` is not `null` and the plugin list is not empty.
- **Pending You's folder** is written where `services.boomdrive.pendingyou.enable` is on and its package is not `null`, before the links, with crew or without.

Each runs as the set's user. Without the runner, the runner's own steps are not run:

- **On nix-darwin**, the set's user is `system.primaryUser`, under `users.users.<user>.home`, by `sudo --user` as now. The runner's file's link, the signed copy and the agent stay under `services.boomdrive.enable`. `system.requiresPrimaryUser` names `services.boomdrive.herdr.enable` where the links run and `services.boomdrive.pendingyou.enable` where the folder is written, beside `services.boomdrive.enable` with the runner. The assertion on the primary user's home holds wherever any of these runs.
- **On NixOS**, the set's user is `services.boomdrive.user`. The user activation script `boomdrive` runs for that user alone, its steps in order: the runner's file's link with the runner, Pending You's folder, herdr's links, then `keep` with the runner. Without the runner, the user does not linger and no unit is declared. Where the folder or the links would run and `services.boomdrive.user` is not defined, evaluation fails, naming `services.boomdrive.user` and the switch that needs it. The assertion that `users.users` declares the user as a normal or system user holds wherever a step runs.
- **On NixOS**, Flume's, sluice's, crew's and timber's `user` are `services.boomdrive.user` by default wherever the host defines it, with the runner or without, a host's own winning. Where it is not defined, none is set, and each tool's module asks for its own as it does without the set.

`services.boomdrive.user`'s description says it is the user the set is set up for: the one whose manager runs the runner, and as whom the plugins are linked and Pending You's folder written, with the runner or without.

**Not taken:**

- **A second user option for a host without the runner.** The person the set is set up for is the same either way, and a host that turns the runner on later would move the value.
- **Each tool's own `user`.** herdr's plugins and Pending You's folder belong to no tool's module.
- **The steps only with the runner.** That leaves a host without the runner with herdr and no plugin, against principle 17.

### 5. Flume's terminal words follow herdr and Ghostty

Where `services.boomdrive.herdr.enable` is on, `services.flume.settings.host.multiplexer` is `herdr` by default. Where `services.boomdrive.ghostty.enable` is on, `host.terminal_app` is `ghostty` by default. This holds with the runner or without, and whether or not the host keeps the set's package of either. A host's own value wins, and where the host gives Flume's older `host.terminal`, no `multiplexer` is set, as now. With the runner and both switches at their defaults, Flume gets the words it gets now.

**Not taken:** the words only with the runner. A host without the runner that turns on Flume and herdr would then name no multiplexer, and `flume open` would print its command.

### 6. The launcher holds no drive without the runner

A drive gives Flume its stream only with the runner (drives.nix; record 0028). So `services.boomdrive.launcher.package`'s default is the set's launcher made for each drive of this host with the runner, and the set's own `boomdrive-launcher`, holding `Default stream` alone, without it. Its `defaultText` says so.

**Not taken:** an entry for each drive without the runner. Each would open a session Flume has no stream for, and show Flume's refusal.

### 7. Already built

Every package a switch installs is the set's for the host's system, the flake's own derivation made with its own nixpkgs (set.nix), so its path is one the family's cache holds (core.47). That includes the launcher without the runner, which is the set's own path. `pendingyou-signin` and Pending You's folder are text and copies the host makes, with the runner or without. No package is added. Trusting the cache stays the host's own nix setting (core.47).

### 8. What stays as it is

- **The runner's own parts.** The runner's package, file, agent or unit, signed copy, drives' streams, crew's table and kept repositories stay under `services.boomdrive.enable`.
- **Telemetry and transcripts.** Claude Code's telemetry document, and Flume's and sluice's telemetry, stay with the runner and a collector. The transcripts document stays wherever Flume is on (record 0046).
- **Claude Code's plugins.** The set's marketplace is one package in the flake, and its install into Claude Code's homes stays record 0044 point 3's unit. Wherever the set installs a tool, with the runner or without, principle 17 has that unit give the tool's Claude Code plugin there too (record 0023 as amended on 2026-11-18).
- **A profile with no module.** `nix profile add` of the set's packages, `herdr-plugins` among them, is unchanged.

### 9. Tests

Each case runs on nix-darwin and on NixOS unless it names one, through the existing helpers of `nix/tests/apps.nix` and `nix/tests/tools.nix`. A system without the runner is `t.darwinOf [ outputs.darwinModules.default <given> ]` or `t.nixosOf [ outputs.nixosModules.default <given> ]`, as `without` is now. A NixOS case that runs a user step gives `services.boomdrive.user = "alice"`.

The cases each task adds are named in its task. Existing cases that keep their meaning keep their names: *Not without the runner*, *No portless without the runner*, *No worktrunk without the runner*, *No timber without the runner* and *No companions without the runner* now read a system that turns nothing on. *The companions' plugins with crew*, one case reading both platforms, stays as it is.

### 10. The words, and the rules written now

The README's **Installing with nix** says:

- each tool has a switch of its own, and the runner's turns them on;
- how a host without the runner takes the tools it wants, with an example;
- that herdr's plugins follow the tools the set installs, and that a host with its own build of a tool lists its own build's plugin;
- that the set's user links the plugins and writes Pending You's folder, with the runner or without.

`CLAUDE.md`'s and `openspec/config.yaml`'s summaries of the flake say each tool is turned on by a switch of its own, which the runner's turns on, so a host without the runner takes the ones it wants. No check reads these words (principle 16).

Record 0023's amendment of 2026-11-18, record 0044's of that day, core.63, and core.18's and core.46's details are written in this change's own commit. No task changes them.

## Risks / Trade-offs

- **A runner host with its own build of a tool loses that tool's pane.** Where it gives Flume, sluice or timber a package of its own, or turns that tool's module off, and names no plugins, the set no longer links the set's plugin of it. herdr keeps a plugin linked until `herdr plugin unlink`, so a pane already linked stays until the person unlinks it. The README says to list the plugin of its own build. afterthought/swancloud's runner host names its plugins, so it is not one.
- **NixOS without the runner needs a user for herdr's plugins.** A host that turns herdr on, with a tool whose plugin is in the default, and names no `services.boomdrive.user` fails evaluation, naming it. That is wanted: the links would otherwise run for nobody.
- **The option's name.** `services.boomdrive.user` names the set's user on a host with no runner. Its description says so.

## Migration Plan

- **A host with the runner** takes the change with its next move of the input, and changes nothing unless the first risk is its.
- **A host without the runner** turns on, in its configuration, each tool it wants:
  - `services.flume.enable`, `services.sluice.enable`, `services.crew.enable` and `services.timber.enable` for those four;
  - `services.boomdrive.<tool>.enable` for the others;
  - on NixOS, `services.boomdrive.user`.

  Each tool's file that its own module writes is linked as with the runner, a file of the host's own moved once aside. A host that keeps a tool's file its own leaves that module off and installs the tool itself.

## Task notes

- **1.1.** The five switches are `lib.mkOption { type = types.bool; default = cfg.enable; defaultText = lib.literalExpression "config.services.boomdrive.enable"; }`, each described as turning its tool on, by default with the runner. `config` becomes a `lib.mkMerge` of one `lib.mkIf` for each tool, so each tool's install reads its own switch. The launcher's default override reads `entries` only where `cfg.enable` is on, and passes `[ ]` otherwise.
- **1.2.** `multiplexer` keeps its guard on `terminal`, inside `lib.mkIf cfg.herdr.enable`. `terminal_app` is set inside `lib.mkIf cfg.ghostty.enable`.
- **2.1.** The companions' switches default to `cfg.enable && config.services.crew.enable`. `installing` goes, and each package is installed under its own switch.
- **3.1.** "The set's" is the set's derivation, compared by `==`, which Nix decides for two derivations by their `outPath`. A tool's module is on by `config.services.<tool>.enable`. The `defaultText` gives the five conditions.
- **4.1.** On nix-darwin, the runner's `lib.mkIf cfg.enable` keeps the agent, the signing, `requiresPrimaryUser`'s runner entry and the runner's link. A second `lib.mkIf` holds the folder's and the links' steps, appended after the runner's link in `postActivation.text`, so their order stays the runner's link, the folder, the links. The primary user's home assertion is moved to a `lib.mkIf` on any of the three.
- **4.2.** On NixOS, `steps` gains the runner's link and `keep` only under `cfg.enable`. The script `boomdrive` is declared wherever `steps` is not empty, and `linger`, the unit and the runner's assertions stay under `cfg.enable`. The user's assertion reads `options.services.boomdrive.user.isDefined` before `config.users.users.${user}`, so evaluation fails with its message and not with the option's own "used but not defined".
- **4.3.** The tools' users move to a `lib.mkIf options.services.boomdrive.user.isDefined` block, outside the runner's.
