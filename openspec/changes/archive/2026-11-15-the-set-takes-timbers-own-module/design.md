# Design

## Context

Record 0023 has the Boomdrive flake take each tool's own module from the tool's own flake, re-export it, and set its options, and carry a tool's module itself only until that tool's flake gives one. Its amendment of 2026-11-13 put timber in the set with an option of the Boomdrive module's own, `services.boomdrive.timber.package`, "timber gives no nix-darwin or NixOS module; once it gives one, the set re-exports it as it does Flume's, and the option goes."

timber's flake at `v0.7.0`, the tag the lock names since the chore `the-set-pins-flume-v0-24-0-and-timber-v0-7-0` (lock revision `629361ba`), gives that module (timber's record 0035, accepted on 2026-11-14). As read in that tree on 2026-11-15:

- **The outputs.** `darwinModules.timber` and `nixosModules.timber`, each also `default`, each `{ _file; key = "github:boomdrivedev/timber#<platform>Modules.timber"; imports = [ (import file { inherit packages; }) ]; }`, where `packages` is timber's flake's own.
- **`services.timber`'s options**, `nix/modules/options.nix`:
  - `enable`, off by default;
  - `package`, of type `nullOr package`, `packages.${system}.default` by default, a system timber's flake gives none for failing evaluation;
  - `settings`, free-form TOML naming every table and key of `~/.config/timber/config.toml`, `{ }` by default;
  - `text`, internal and read-only, the file's text, which `settings = { }` renders as its header line alone;
  - `user`, on NixOS alone, with no default.
- **With `enable`, on nix-darwin** (`nix/modules/darwin.nix`):
  - `system.requiresPrimaryUser` names `services.timber.enable`, and a primary user with no `users.users.<user>.home` fails evaluation;
  - `environment.systemPackages` holds the package where it is not `null`;
  - one line of `system.activationScripts.postActivation.text` runs its `activate.sh link <file>` as the primary user under their home, `|| true`.
- **With `enable`, on NixOS** (`nix/modules/nixos.nix`):
  - a `user` that `users.users` declares as neither a normal nor a system user fails evaluation;
  - the package is in the system profile where it is not `null`;
  - `system.userActivationScripts.timber` runs the same link for that user alone.
- **The link.** `activate.sh link` links `~/.config/timber/config.toml` to the module's file. A file or folder there that is no link is moved once to `config.toml.before-nix`. Where that path is taken already, it changes neither and fails, and the `|| true` stops nothing else.
- **Nothing else.** No launchd agent or daemon, no systemd unit, no `timber serve`, no file under `/etc` and no other tool's file.
- **What an empty file binds.** timber reads a file holding no table as it reads no file: each table it leaves out takes the given binding (timber's 118).

The set today:

- **`flake.nix`'s `tools`** is `flume`, `sluice` and `crew`. `modulesOf` imports each one's `<platform>Modules.<tool>` into the platform's `boomdrive` module and gives each again beside `boomdrive` and `default`.
- **`nix/modules/tools.nix`** sets each tool's `package` to the set's by default, whether or not the runner is enabled, and turns on Flume's and sluice's modules with the runner, a host's `false` winning.
- **`nix/modules/nixos.nix`** gives Flume, sluice and crew the runner's user by default.
- **`nix/modules/apps.nix`** declares `services.boomdrive.timber.package`, the set's `timber` by default, and with the runner puts it in the system profile where it is not `null`.
- **`nix/modules/options.nix`** removes `services.boomdrive.tools` by `lib.mkRemovedOptionModule`, whose failure names `services.flume.package` and `services.sluice.package`.
- **The set's `timber`** is `inputs.timber.packages.<system>.default`, its flake following the set's `nixpkgs`, `rust-overlay` and `nix-darwin`. So timber's module's default package is the same derivation as the set's, which the case "timber through its own flake" holds by `drvPath`.

Rules relied on:

- **core.18:** a tool's file is written only by its own module. The Boomdrive module sets the others' module options.
- **core.44:** each tool is taken from its own flake where it publishes one.
- **core.46:** each tool's module writes its file, and no module starts timber's serving process.
- **core.47:** a module's package is the flake's own, so its path is the one the cache holds.
- **Blueprints principle 16:** no check reads Markdown.

## Goals / Non-Goals

**Goals:**

- `services.timber` on every system that takes the set's modules is timber's own module, at the tag the lock names, declared once however the host imports it.
- Every host that has the runner keeps timber installed, as it did with the set's own option, and gets timber's file from timber's module.
- No option of the Boomdrive module's own installs timber.

**Non-Goals:**

- **Any key of timber's `settings`.** The set knows no fact of timber's bindings (core.57): they are the host's, written in `services.timber.settings`.
- **Moving timber's pin.** `v0.7.0` carries the module.
- **timber's serving process.** It stays timber's (timber's 114).
- **timber's herdr plugin and Claude Code plugin.** They stay where they are.

## Decisions

### 1. Re-exported and imported, as Flume's, sluice's and crew's are

`flake.nix`'s `tools` gains `timber`, after `crew`. So:

- the platform's `boomdrive` module, and `default`, import timber's `<platform>Modules.timber` after crew's;
- `darwinModules` and `nixosModules` each give `boomdrive`, `crew`, `default`, `flume`, `sluice` and `timber`;
- every option of `services.timber` is declared in a file of timber's input, and none in this tree.

timber's key lets a host import timber's module from timber's own flake as well, and take it once.

The comments of `flake.nix` that name the tools' modules name timber's.

Alternatives:

- **A copy of timber's module here.** Not taken: core.18 has each tool's file written by its own module, from its own flake.
- **Re-exported, not imported.** Not taken: a host would import timber's module by hand to keep timber, which "as it does Flume's" rules out.

### 2. Turned on with the runner

`tools.nix` sets `services.timber.enable` to true by default where `services.boomdrive.enable` is on, beside Flume's and sluice's. A host's `false` wins. Without the runner, timber's module stays off.

This keeps what the amendment of 2026-11-13 gave: timber installed with the runner, whether or not crew is on, and none without it.

Alternatives:

- **Left to the host, as crew's is.** Not taken: every host would drop timber at its next switch, and have to turn it on. crew is left to the host because it needs a host and drives to be named; timber needs nothing named.
- **On wherever the set's modules are imported.** Not taken: without the runner the set installs none of its own apps.

### 3. Its package the set's

`tools.nix` sets `services.timber.package` to the set's `timber` by default, whether or not the runner is enabled, as it does Flume's, sluice's and crew's. A host's own package, or `null`, wins.

Today that is the same derivation as timber's module's default. Set here, the set's path is the one the module installs should the two ever part, and one rule covers every tool's package (core.47).

Alternative:

- **timber's module's own default alone.** Not taken. It holds only while timber's input follows the set's `nixpkgs` and `rust-overlay`, which nothing in timber's module says.

### 4. Its user the runner's

- **NixOS.** `nixos.nix` sets `services.timber.user` to `services.boomdrive.user` by default, beside Flume's, sluice's and crew's, a host's own winning.
- **nix-darwin.** timber's module links for `system.primaryUser`, whose login runs the runner too. The set gives it nothing.

Alternative:

- **No user given.** Not taken: a NixOS host with the runner would fail evaluation, timber's `user` having no default.

### 5. Its file is its module's

With timber's module on, each switch links `~/.config/timber/config.toml` for the runner's user. Where the host sets no `services.timber.settings`, the file is the header line alone, which binds as no file does (timber's 118): a host's timber works as it did with no file.

The Boomdrive module writes no key of timber's `settings` and none of timber's files (core.18):

- **On nix-darwin**, the activation gains timber's own link line.
- **On NixOS**, the user activation gains timber's own snippet, `timber`.

*The tools' activation is their own* compares the runner's system with the tools' own modules alone. timber's module joins those, turned on with the set's package and, on NixOS, the user `alice`.

A host with a file of its own at that path finds it moved once to `config.toml.before-nix`, as Flume's and sluice's modules do. A host that keeps its own file sets `services.timber.enable = false`, and installs its timber itself (Risks).

Alternative:

- **timber's module turned on only where the host names a setting.** Not taken: a condition on `settings` would also decide whether timber is installed, and a host with nothing to bind would lose it.

### 6. The option fails, naming what replaces it

`apps.nix` no longer declares `timber.package`, and its install list drops it. `options.nix` removes `services.boomdrive.timber.package` by `lib.mkRemovedOptionModule`, beside `services.boomdrive.tools`. Its words: timber's own module installs timber; set `services.timber.package` to the host's own package, or to `null` for none, or set `services.timber.enable` to `false` to keep timber's file the host's own.

`apps.nix`'s header says timber comes by its own module, which the Boomdrive module turns on, and keeps the plugin's words.

Alternatives:

- **`lib.mkRenamedOptionModule` to `services.timber.package`.** Not taken. A host's `null` would carry over and install no timber, while timber's module linked a file the host never asked for. Failing makes the host choose between the two.
- **The option kept, setting `services.timber.package`.** Not taken: two options for one package, which the brief rules out.

### 7. What stays

- **timber's herdr plugin.** `services.boomdrive.herdr.plugins`' default keeps `timber-herdr-plugin` after sluice's. timber's module links no herdr plugin, and herdr's links are the Boomdrive module's own (record 0023).
- **timber's Claude Code plugin.** The set's marketplace keeps it (record 0044 as amended on 2026-11-13).
- **timber's serving process.** No step, job or unit of the modules runs the set's timber, timber's module's included: its link line names its `activate.sh` and its file, never the package. *No serving process of the modules'* holds as before.
- **The set's packages, the lock, the shell, the cache's workflow and the boot.** The boot's NixOS system takes timber's module with the runner, and the boot's check asks `timber --version`, as before. The type check's *The boot* still evaluates it.

### 8. Tests

The service-modules delta's scenarios are cases of `nix/tests/`, evaluated against no store by `nix/check.sh`, with core.41's neutral names.

**`nix/tests/lib.nix`.** `toolsOf` gains timber's own module, turned on with the set's `timber`, after crew's. `t.toolsNixosSystem` gives it the user `alice`.

**`nix/tests/tools.nix`.** Its `tools` gains `timber`, and its `fileNames` timber's file, `timber-config.toml`. The cases:

- *Each tool's options are its own flake's*, *The tools' modules re-exported* (the six names), *A host with the runner alone* (timber's `enable` true), *The set's packages* (`timber` among them) and *The tools' user on NixOS* (timber's `alice`) read timber with the rest.
- *The tools' activation is their own*, on each platform, reads timber's link with the rest. The NixOS case holds `timber`'s snippet to the tools' own.
- *timber's own module beside the set's* is new. A system imports the platform's `default` and timber's flake's own `<platform>Modules.timber`, and evaluates.

**`nix/tests/apps.nix`.**

- *timber with the runner* holds that timber's `enable` is true, beside the set's timber in the profile.
- *A host's own timber* sets `services.timber.package`. It holds that timber's link is the one the set's timber gets.
- *No timber of the set's* sets `services.timber.package = null`. The system evaluates, with no set's timber, and timber's file still linked.
- *A host that keeps its own timber file* is new. With `services.timber.enable = false`, no timber of the set's is in the profile and no link of timber's runs.
- *No timber without the runner* holds that timber's module stays off.
- *No serving process of the modules'* is unchanged.
- *The timber option is gone* is new, on each platform. `services.boomdrive.timber.package` fails evaluation, naming `services.timber.package` and `services.timber.enable`.

**`nix/tests/cache.nix`.** *The README's words* no longer looks for `services.boomdrive.timber.package`, and looks for nothing new. No case reads the README's new words (principle 16).

### 9. The words, and the rules written now

Written in this change's own commit, by this stage:

- **Record 0023's amendment of 2026-11-15.** It holds the brief's ask and Decisions 1 to 7, each with the path not taken.
- **core.18's details.** They allow the Boomdrive module to set timber's module options, and timber's file linked by timber's own module.
- **core.46's details.** The Boomdrive module's own options no longer name timber. They allow timber's own module, re-exported and imported, turned on with the runner, its NixOS user the runner's, and the removed option failing.

The build writes the README's **Installing with nix**:

- **The tools' modules.** timber's module joins Flume's, sluice's and crew's: `services.timber`, from timber's flake at the tag this flake names, given again as `darwinModules.timber` and `nixosModules.timber`. Its file is `~/.config/timber/config.toml`, written from `services.timber.settings`, the header alone where none is set, and moved aside once where the host has its own.
- **The runner's part.** With the runner, timber's module is turned on, and its herdr plugin linked with Flume's and sluice's. The list of the runner's module's own apps drops timber: five, not six.
- **The `null` lines.** `services.timber.package = null;` sits with Flume's and sluice's. The paragraph after them says a host keeps timber's file its own with `services.timber.enable = false`.
- **The serving process.** Its paragraph names `services.timber.package` where it named the removed option.

No check reads these words (principle 16). CLAUDE.md and `openspec/config.yaml` already say each tool's own module is re-exported and timber is installed with the runner, and do not change.

## Risks / Trade-offs

- **A host's own timber file is moved aside once.** A host that wrote `~/.config/timber/config.toml` by hand finds it at `config.toml.before-nix` after its next switch, and timber reads the module's file. Where `config.toml.before-nix` is taken already, timber's link fails, says why, and stops nothing else. The README says to write the bindings in `services.timber.settings`, or turn timber's module off. Each tool of the set behaves the same.
- **A host that set the removed option fails evaluation.** Its failure names both ways forward. A host that set it to its own package moves it to `services.timber.package`. One that set `null` chooses `services.timber.package = null`, with timber's file linked, or `services.timber.enable = false`, without it.
- **timber's module rides the set's pin.** A timber release that changes its module reaches the hosts with the lock's move, in a change the gate passes (core.45), as Flume's and sluice's do.
- **Two writers of timber's file.** A host that keeps another writer of `~/.config/timber/config.toml`, such as a dotfile manager, has two, the last activation winning, until it drops one (timber's record 0035).

## Migration Plan

- **A host with the runner and no timber option** gets timber's module at its next switch: the same timber in its profile and the header-only file linked.
- **A host that set `services.boomdrive.timber.package`** moves it, as Risks say.
- **A host that imported timber's module itself** evaluates as before, its settings kept, the module taken once by its key. It may drop its import.
- **afterthought/swancloud** takes the change with its next move of the Boomdrive input.
- **Rolling back** is a pin before this change. Its set installs timber by its own option and links no file of timber's. The link timber's module made at `~/.config/timber/config.toml` stays, naming a file of an older generation, until the host removes it by hand, as with Flume's and sluice's.

## Task notes

- **1.1.** `t.darwinSystem` and `t.nixosSystem` import the platform's `default`. For *timber's own module beside the set's*, the case adds `t.inputs.timber.darwinModules.timber` or `nixosModules.timber` to `t.darwinOf` or `t.nixosOf`'s modules, after the platform's `default`. It turns the runner on as `t.darwinSystem` and `t.nixosSystem` do, so timber's module is on.
- **1.3.** The timber link's text on nix-darwin is the activation's line naming `timber-config.toml`, and on NixOS the user activation's `timber` snippet, as `tools.nix`'s `activationOf` reads a tool's. The cases in `apps.nix` compare that text, with the string's context discarded.
- **1.4.** `lib.mkRemovedOptionModule`'s failure is an assertion, so `t.failures` reads its words, as the cases of `services.boomdrive.tools` read theirs.
