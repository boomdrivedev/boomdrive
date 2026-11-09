# Design

## Context

Record 0023 (accepted 2026-11-06) has the Boomdrive flake configure the whole host, each tool by its own module. Flume, sluice and crew each publish a flake with their own module, and the Boomdrive flake takes each as a locked input and re-exports its module. Each module writes its own tool's file, from options named as that file names them. The Boomdrive module sets the other tools' module options and writes no file of theirs (core.18, core.46). The set is each tool at a release tag, taken from the tool's own flake where it publishes one (core.44).

The flake today (record 0020, its change's design Decisions 1 to 14) does this for the runner alone:

- `nix/modules/options.nix` declares `services.boomdrive`, its `settings` the runner's file, and `tools`, the set's Flume and sluice installed beside the runner.
- `darwin.nix` and `nixos.nix` keep the runner's launchd agent and systemd user unit, and link its file.
- The set builds Flume by `nix/packages/flume.nix`, since Flume published no flake, and sluice through sluice's flake. It holds no crew.

Each tool's own module is being built in its own repository. Each design follows Boomdrive's shape of record 0020, and each has the Boomdrive flake re-export it:

| tool | its decision | its change | options |
|---|---|---|---|
| Flume | flume record 0055 | `flume-publishes-its-own-flake-and-module` | `enable`, `package` (or null), `settings`, `user` on NixOS |
| sluice | sluice record 0098 | `sluice-publishes-a-module-for-its-host-settings` | `enable`, `package` (or null), `settings`, `user` on NixOS |
| crew | crew record 0058 | `crew-publishes-a-flake-whose-module-writes-its-host-file` | `enable`, `package`, `host`, `settings`, `user` on NixOS |

Each gives `darwinModules.<tool>` and `nixosModules.<tool>`, each also `default`, and `packages.<system>.<tool>`, also `default`, for aarch64-darwin, aarch64-linux and x86_64-linux. Each module installs its package in the system profile, renders its file at evaluation, and links it at the tool's own path on activation. On nix-darwin it does so as `system.primaryUser`, and on NixOS for its `user`. None runs a service. Flume's module carries `key = "github:agentplot/flume#<platform>Modules.flume"`, so a system that imports it twice declares it once (its Decision 4). sluice's and crew's modules, as their designs stand, carry none.

This change is the first unit of the bolt `the-boomdrive-flake-configures-the-whole-host`. Three later units of the bolt set further options of these modules:

- `each-repository-is-named-once-by-its-owner` sets each repository's stream in Flume's `settings`.
- `herdr-and-ghostty-come-with-the-flake-and-an-app-opens-the-agents` sets Flume's terminal.
- `crews-partitions-give-the-runner-its-crew-table` reads crew's partitions.

They come after this unit.

## Goals / Non-Goals

**Goals:**

- `darwinModules.default` and `nixosModules.default` bring the runner's module and Flume's, sluice's and crew's own, each taken from its tool's flake at a release tag.
- The Boomdrive module sets the tools' module options and writes no file of theirs.
- Each tool's package is the set's unless the host gives its own through that tool's module.
- The set gains crew and takes Flume from Flume's flake, so no recipe of this flake's builds a tool whose own flake does.

**Non-Goals:**

- Any key of another tool's `settings`. The bolt's later units set what they need.
- herdr, Ghostty, the Boomdrive app and the shell.
- The runner's crew table.
- The runner's accounts and drives, and its local layer.
- Any change to the runner's code, its file, its agent or unit, or what the runner's tests prove of the set.
- A copy of any tool's module in this tree.

## Decisions

### 1. Each tool's module comes from its own flake, at its release tag, and none is carried here

The flake takes Flume's, sluice's and crew's flakes as inputs, each at its newest release tag whose flake gives `darwinModules.<tool>` and `nixosModules.<tool>`. It imports and re-exports each tool's module as that flake gives it. No file of this tree declares an option of `services.flume`, `services.sluice` or `services.crew`.

This change is built once each of the three has such a tag (Dependencies, below).

Alternatives:

- **A copy of a tool's module carried here until its flake gives one.** Record 0023 allows it, and the flake carries Flume's package that way today. Not taken:
  - core.18 has a tool's file written only by that tool's own module, and a copy here would be a second writer of Flume's, sluice's or crew's file.
  - sluice's design rejects it in so many words.
  - Each tool's module is being built in its own repository now, so a copy would be thrown away within days, after drifting from the module that replaces it.
- **A tool's flake taken at a commit of its `main`, before its release.** Not taken: core.44 names each tool of the set by a release tag, and the tags check refuses anything else.

### 2. The inputs, and what each follows

- `flume`: `github:agentplot/flume/<tag>`, now a flake. Its `nixpkgs`, `rust-overlay` and `nix-darwin` follow the flake's.
- `sluice`: `github:agentplot/sluice/<tag>`, the tag moved. Its `nixpkgs` and `rust-overlay` follow the flake's as now, and its `nix-darwin`, which sluice's flake gains for its tests, follows the flake's too.
- `crew`: `github:agentplot/crew/<tag>`, new. Its `nixpkgs` and `nix-darwin` follow the flake's.

With every input following, each tool's package is made with the flake's own nixpkgs, so it is a path the family's cache holds (core.47). Each tool's module evaluates against the host's own nix-darwin or nixpkgs, as Boomdrive's does. A tool whose flake takes an input not named here gets it as its own flake locks it. The builder names a `follows` for every input a tool's flake shares with this one.

`nix/packages/tag.nix` reads each tool's tag from its URL. `nix/tests/tags.nix` holds Flume, sluice and crew each to a release tag, and every input's tag to the lock's (core.45).

### 3. The set

| package | made by |
|---|---|
| `boomdrive`, `default` | this tree, as now |
| `flume` | Flume's flake's `packages.<system>.flume`, unchanged |
| `sluice` | sluice's flake's package, given its tag's version where its tree says `0.0.0` (`nix/packages/sluice.nix`, as now) |
| `crew` | crew's flake's `packages.<system>.crew`, given its tag's version (`nix/packages/crew.nix`) |
| `flume-herdr-plugin`, `sluice-herdr-plugin` | each tool's `plugins/herdr`, from its package's source, as now |

- **Flume.** Flume's tagged tree carries its version in `Cargo.toml` (Flume's release writes it), and Flume's flake builds that tree. So the set's Flume is Flume's flake's package, with nothing given to it. `nix/packages/flume.nix` and `nix/packages/flume-git-crates.nix` go, and with them the case "Flume's git crates": core.44 rules out a recipe kept here for a tool whose own flake builds it, and Flume's flake holds its own git crates' hashes.
- **crew.** crew keeps its version in no file, and its flake versions its package `0-unstable-<date>`. So `nix/packages/crew.nix` gives crew's flake's package its tag's version without the `v`, as `sluice.nix` gives sluice's. core.44's details allow a version given to a tool's package where its tagged tree does not carry one. crew prints no version of its own, and the runner never runs crew: crew's implementation reads and writes crew's state repository through the host's git (core.2, core.34). So core.38's floors and core.45's set proof name no crew, and `crates/boomdrive/tests/family.rs` is unchanged.
- **The cache.** The workflow `cache` builds every package `nix eval` lists for a system, so it builds and pushes crew with the rest, with no change to the workflow (core.47).

Alternatives:

- **crew's package as crew's flake gives it.** Not taken: its version would not be its tag's, as core.44 and the family-packages spec ask.
- **No crew in the set, every host giving its own.** Not taken: record 0023 has the flake package crew, and a host that gives its own still can (Decision 5).

### 4. The modules

`flake.nix` gives, for each platform:

- `<platform>Modules.boomdrive`: an attribute set with `_file`, `key = "github:agentplot/boomdrive#<platform>Modules.boomdrive"`, and `imports` holding the runner's module (`nix/modules/darwin.nix` or `nixos.nix`, given `packages`), then Flume's, sluice's and crew's, each `inputs.<tool>.<platform>Modules.<tool>`.
- `<platform>Modules.default`: the same value.
- `<platform>Modules.flume`, `.sluice` and `.crew`: each tool's own module, as its flake gives it.

The key is Flume's design's (its Decision 4). With it, a host that imports both `default` and `boomdrive` takes the whole set once.

The runner's module sets options only the tools' modules declare (Decision 5), so it is not given on its own.

A host that imports a tool's module directly beside Boomdrive's takes it once only where the tool's module carries its key, as Flume's does. sluice and crew are asked to carry one (What this change asks of others). Until then a host imports Boomdrive's module alone, as record 0023's one flake has it.

Alternatives:

- **`boomdrive` the runner's module alone and `default` the whole set.** Not taken: the runner's module cannot evaluate without the options it sets.
- **No re-export of the tools' modules.** Not taken: record 0023 has the flake re-export each, so a host that takes only the Boomdrive flake can still take one tool's module alone.

### 5. What the Boomdrive module sets of the tools' modules

`nix/modules/tools.nix`, given `packages`, is imported by `darwin.nix` and `nixos.nix` beside `options.nix`. It sets:

- **Each tool's package, always:** `services.flume.package`, `services.sluice.package` and `services.crew.package`, each `lib.mkDefault` the set's for the host's system.
  - A tool module's own default sits below `mkDefault`, so the set's wins over it.
  - A host's plain definition sits above `mkDefault`, so the host's own wins over the set's.
  - Flume's and sluice's modules take `null`, which installs none. crew's module takes no `null`, since a later unit reads its package's plugin path.
  - This holds whether or not the runner is enabled, so a host that turns on one tool of the set alone gets the set's package, the path the cache holds, as the binary-cache spec has every module's default.
- **Flume and sluice, where `services.boomdrive.enable`:** `services.flume.enable` and `services.sluice.enable`, each `lib.mkDefault true`.
  - The runner runs both, and record 0023's laptop writes its repositories and nothing else.
  - A host that keeps its own Flume or sluice file sets that tool's `enable = false`, and installs the tool itself.
- **crew: not turned on.** crew's module asks for `host` and a non-empty `partitions`, with no default (crew's `code.5`). A host with no crew partitions, a laptop or one under a hosted control plane, would fail evaluation. crew's module is the host's to turn on, with its facts.
- **On NixOS, where `services.boomdrive.enable`:** `services.flume.user`, `services.sluice.user` and `services.crew.user`, each `lib.mkDefault` `services.boomdrive.user`. `nixos.nix` sets these beside its own options, since nix-darwin's tools take `system.primaryUser` as the runner does.

It sets nothing else:

- No key of another tool's `settings`.
- No file, link or activation line of another tool's.
- No option of a tool's module that this design does not name.

Each tool's module writes its own file, from its own `settings`, and links it by its own activation (core.18, core.46). The runner's module links the runner's file alone, as now.

Alternatives:

- **Flume and sluice turned on only where the host asks.** Not taken: a laptop would write more than its repositories, against record 0023's one line, and the bolt's later units set Flume's `settings`, which a module turned off never writes.
- **crew turned on where `control_plane.type` is `crew`.** Not taken: the runner reads crew's state repository, not crew's file, so the runner needs no crew on its host. A host under crew that runs crew's commands names its partitions, and with them turns crew on.
- **The packages set only where the runner is enabled.** Not taken: a host that imports Boomdrive's module for one tool would get that tool's own default, which is no path the family's cache holds for sluice or crew.

### 6. `services.boomdrive.tools` goes

Each tool's module installs its package, so `tools` would install the same tools a second time. `options.nix` replaces it with `lib.mkRemovedOptionModule [ "services" "boomdrive" "tools" ]`. A host that sets it fails evaluation with words naming what replaces it:

- `services.flume.package` and `services.sluice.package`, each `null` for none;
- or that tool's `enable = false`.

No host is known to set it: afterthought/swancloud does not yet import these modules.

Alternatives:

- **`tools` kept beside the tools' modules.** Not taken: two options would install one tool, and a host could hold two of one tool in its profile.
- **`tools` dropped with no word.** Not taken: a host that set it would get nix's bare "does not exist", naming nothing to do.

### 7. The runner's own module is otherwise unchanged

The agent, the unit, the runner's file and its link do not change, and neither does the service's `PATH`. Each tool's module installs its package in the system profile, which the `PATH` names. So the runner, what Flume launches and a person's shell still run one copy of each tool. The runner's tests prove the set's Flume and sluice by the same outputs `devenv.nix` reads (core.45), at their new tags.

### 8. The tests

Every case is an evaluated case under `nix/tests`, run by `nix:typecheck` or `nix:test` against no store (record 0020's Decision 14). Each case's names are core.41's: users `alice` and `bob`, the host `host-a`, and repositories under `example/`.

- **`nix/tests/tools.nix`**, added to `test`, holds the service-modules delta's added scenarios, each on nix-darwin and on NixOS where the scenario names no platform:
  - "Each tool's options are its own flake's" and "The tools' modules re-exported";
  - "Imported twice";
  - "A host with the runner alone", "A host that keeps its own Flume file" and "The tools' user on NixOS";
  - "The set's packages", "A host's own crew" and "No sluice of the set's";
  - "The tools' activation is their own".
- **`nix/tests/settings.nix`** gains `crewModule`, crew's own settings for one partition with neutral names: `host-a`, the partition `home`, and `example/state` and `example/blueprints`. It holds every key crew's module asks for at its tag. `lib.nix` gains `toolsDarwinSystem` and `toolsNixosSystem`, each importing the three tools' own modules alone, for "The tools' activation is their own".
- **`packages.nix`** replaces "Flume at its tag" and "An overridden input" with "Flume through its own flake", gains "crew through its own flake" and "A tool named by no release tag", and drops "Flume's git crates".
- **`typecheck.nix`**'s set gains `crew`.
- **`darwin.nix` and `nixos.nix`**: their `tools` cases become the modified scenarios, and they gain "The tools option is gone".
- **`cache.nix`**'s "A host's own nixpkgs" enables crew and holds the profile to the flake's own crew too.
- **`cache.nix`** gains "The README's words".

### Rules relied on

- core.18: each tool's file is written by its own module, and the Boomdrive module sets the other tools' module options (Decisions 1 and 5).
- core.44: each tool at a release tag, taken from its own flake, with no recipe kept for a tool whose own flake builds it. Its details allow crew's version given from its tag (Decision 3).
- core.45: each tag moves with the lock in this change, and the runner's tests prove the set's Flume and sluice at their new tags (Decision 7).
- core.46: each tool's module writes its tool's file from one option per key, and the Boomdrive module sets the others' options and writes no file of theirs (Decision 5).
- core.47: every package is the flake's own derivation, made with its own nixpkgs, and built and pushed by the workflow (Decisions 2 and 3).
- core.4: no option this change sets holds a credential. Each tool's module refuses one in its own `settings`, by its own rule.
- core.13: no module declares a service but the runner's.
- core.7 and core.8: no crate, tool or tracked file type is added.
- core.41: neutral names in every case.

### Rules amended

- core.46's sentence, as record 0023's splice left it, reads "the flake's modules and run the runner". It becomes "the flake's modules run the runner", which changes no meaning.
- core.46's details say "each tool installed by its own module, its package the set's unless the host gives its own (record 0023)" where they said "the set's tools installed beside the runner where the host asks".

### Rules departed from

None.

## Dependencies

This change is built once each of these has a release tag whose flake gives `darwinModules.<tool>` and `nixosModules.<tool>`:

- Flume, from `flume-publishes-its-own-flake-and-module`;
- sluice, from `sluice-publishes-a-module-for-its-host-settings`;
- crew, from `crew-publishes-a-flake-whose-module-writes-its-host-file`.

Tasks 1.1 to 1.3 each take one tool's newest such tag. Where a tool has none yet, its task cannot be done and the build waits for that release. The task does not carry a module in its place (Decision 1).

The chore `the-sets-flume-and-sluice-are-their-latest-releases` of the same bolt moves Flume to `v0.17.1` and sluice to `v0.74.3`, neither giving a module. This change's tags supersede them, and where both touch `flake.nix` and `flake.lock`, the later tags are kept.

## What this change asks of others

- **sluice and crew: a key on each module.** sluice's and crew's modules should each carry `_file` and `key = "github:agentplot/<tool>#<platform>Modules.<tool>"`, as Flume's Decision 4 does. Then a host that imports a tool's module beside Boomdrive's, or at Boomdrive's revision through Boomdrive's re-export, declares its options once. Without it, nix refuses the second declaration. It is the family's one rule for a module imported twice, Flume's, and is told to the orchestrator for sluice and crew.
- **afterthought/swancloud**, once it takes this flake, passes its hosts' facts as `services.flume.settings`, `services.sluice.settings` and `services.crew.settings` and stops writing those files (record 0023), in the same switch, since each module moves a hand-written file aside on its first link.

## Risks / Trade-offs

- **A hand-written Flume or sluice file is moved aside at a host's first switch.** Turning on Flume's and sluice's modules links each tool's file, and a file there that is no link moves once to `config.toml.before-nix`, by each tool's own activation. A host moves its values into each tool's `settings` in the switch that takes this flake, or sets that tool's `enable = false`. The README says so.
- **Three releases before the build.** The bolt's later units wait on this one, and it waits on three repositories' releases.
- **Two revisions of one tool's module.** A host whose own flake takes a tool's flake at another tag, and imports both, gets the first module imported where the tool's module carries a key (Flume's), and nix's refusal of a second declaration where it does not. A host takes one revision by making its input follow Boomdrive's, `inputs.sluice.follows = "boomdrive/sluice"`.
- **crew's package version is the set's word.** The set's crew says its tag's version where crew's own flake says `0-unstable-<date>`. Both are crew's tree at one tag, and crew prints neither.

## Migration Plan

A host that imports these modules today and sets `services.boomdrive.tools` removes it:

- To keep the set's Flume and sluice, it sets nothing.
- To install a tool of its own, it sets that tool's `package`.
- To keep a tool's file its own, it sets that tool's `enable = false`.

A host that writes Flume's or sluice's file by its own tooling moves those values into `services.flume.settings` or `services.sluice.settings`. A host that runs crew turns on `services.crew` with its `host` and `settings.partitions`. Nothing changes for a host until it moves its Boomdrive input.

## Open Questions

None.

## Task notes

**1.1** In `flake.nix`, `sluice.url` names the newest release tag of agentplot/sluice whose flake gives `darwinModules.sluice` and `nixosModules.sluice`, read with `git ls-remote --tags` and its `flake.nix` at that tag. `inputs.sluice.inputs.nix-darwin.follows = "nix-darwin"` joins its other two `follows`. Lock it with `nix flake lock --update-input sluice` in the devenv shell, outside the sandbox. If sluice's tree at that tag carries its own version, `sluice.nix` already passes the package unchanged.

**1.2** `flume` loses `flake = false` and its comment, names Flume's newest release tag whose flake gives `darwinModules.flume` and `nixosModules.flume`, and follows `nixpkgs`, `rust-overlay` and `nix-darwin`. `packagesOf`'s `tools.flume` is `inputs.flume.packages.${system}.flume`. `git rm nix/packages/flume.nix nix/packages/flume-git-crates.nix`. In `packages.nix`:

- "Flume's git crates" goes.
- "Flume at its tag" and "An overridden input" become "Flume through its own flake": each system's `flume` has the `drvPath` of `inputs.flume.packages.<system>.flume`, and its `version` is the lock's tag without the `v`.
- `pluginOf` reads the plugin from `inputs.flume.outPath` as now.

The README's paragraph on Flume's git crates is task 3.1's.

**1.3** `crew` is `github:agentplot/crew/<tag>`, the newest release tag whose flake gives `darwinModules.crew` and `nixosModules.crew`, following `nixpkgs` and `nix-darwin`. `nix/packages/crew.nix` is `{ lib, package, tag }`, with a header comment in `sluice.nix`'s manner. It gives `package.overrideAttrs` the version `lib.removePrefix "v" tag`, with `__intentionallyOverridingVersion = true`, and changes nothing else. `packagesOf` gives `crew` from it, with `inputs.crew.packages.${system}.crew` and the tag `tagOf` reads from the URL. In `packages.nix`, "crew through its own flake": each system's `crew` has the `src`, `pname` and `meta.homepage` of crew's flake's package, and the lock's tag without the `v` as its version.

**1.4** `typecheck.nix`'s `set` gains `"crew"` in name order.

**1.5** `tags.nix`'s list of tools named by a release tag gains `"crew"`. In `packages.nix`, "A tool named by no release tag" calls `tags` with `flake.nix`'s inputs, crew's URL replaced by `github:agentplot/crew/main` and the lock's crew node's `original.ref` set to `main`. It expects one failure, naming `crew` and `main`.

**1.6** The set's Flume and sluice at the new tags. `crates/boomdrive/tests/family.rs` is unchanged. A failure there is a difference between the runner and a tool's new release, and is fixed in the runner's readers, citing the tool's clause, or the tag stays where the test passes, as record 0020's Decision 6 has it.

**2.1** In `flake.nix`, `hostModule = platform: file: { _file = ./flake.nix; key = "github:agentplot/boomdrive#${platform}Modules.boomdrive"; imports = [ (import file { inherit packages; }) ] ++ map (tool: inputs.${tool}."${platform}Modules".${tool}) [ "flume" "sluice" "crew" ]; }`. Then `darwinModules = rec { boomdrive = hostModule "darwin" ./nix/modules/darwin.nix; default = boomdrive; flume = inputs.flume.darwinModules.flume; sluice = …; crew = …; }`, and `nixosModules` likewise. The comment above them says each tool's module is its own flake's.

The cases, in `nix/tests/tools.nix` added to `default.nix`'s `test`, on both platforms:

- "Each tool's options are its own flake's": every option `lib.collect lib.isOption options.services.<tool>` gives has each of its `declarations` under `inputs.<tool>.outPath`, for each of the three tools.
- "The tools' modules re-exported": the attribute names of `darwinModules` and of `nixosModules` are `boomdrive crew default flume sluice`. A system of `outputs.<platform>Modules.sluice` alone, beside the platform's minimal configuration, declares `services.sluice.enable` from sluice's input and no `services.boomdrive`.
- "Imported twice": `t.darwinSystem outputs.darwinModules.boomdrive` evaluates with no failed assertion, and likewise on NixOS.

**2.2** `nix/modules/tools.nix` (Decision 5), imported by `darwin.nix` and `nixos.nix` beside `options.nix`. Its `set` is resolved as `options.nix`'s is: share the `set` and its `throw` by moving them to a small `nix/modules/set.nix` both import, or repeat the three lines with a comment. `nixos.nix` sets the three `user` defaults under its `mkIf cfg.enable`. `settings.nix` gains `crewModule`, and `lib.nix`'s systems take a given module that turns crew on with it.

`options.nix` loses `tools` and gains `(lib.mkRemovedOptionModule [ "services" "boomdrive" "tools" ] "<words>")` in `imports`. The words name `services.flume.package` and `services.sluice.package`, each null for none, and each tool's `enable = false`. `darwin.nix` and `nixos.nix` put `cfg.package` alone in `environment.systemPackages`. The header comments of `options.nix`, `darwin.nix` and `nixos.nix` say each tool is installed by its own module. `file.nix` evaluates `options.nix` alone, as now, and is unchanged.

The cases, in `tools.nix` unless named:

- "A host with the runner alone": `t.darwinSystem { }`'s `services.flume.enable` and `services.sluice.enable` are true, `services.crew.enable` false, and it fails no assertion. The same holds of `t.nixosSystem { }`.
- "The tools' user on NixOS": with crew on, the three `user`s are `alice`, and with `services.sluice.user = "bob"`, sluice's is `bob`.
- "The set's packages": with crew on, the profile holds the set's `boomdrive`, `flume`, `sluice` and `crew`.
- In `darwin.nix` and `nixos.nix`:
  - "The set's tools" holds as now.
  - "A host's own tools": `own` becomes `services.flume.package = null; services.sluice.package = null;`.
  - "Nothing else changed on nix-darwin" compares two systems differing only in `services.flume.package`.
  - "The tools option is gone" sets `services.boomdrive.tools = [ ]` and expects a failure naming `services.flume.package` and `services.sluice.package`.

**2.3** The cases, in `tools.nix`:

- "A host that keeps its own Flume file": with `services.flume.enable = false`, the activation's text names no `flume-config.toml`, and the profile holds no set's Flume. On NixOS, `system.userActivationScripts` has no `flume`.
- "A host's own crew": with `services.crew.package` a `pkgs.runCommand` laid out as crew's design asks, the profile holds it and not the set's crew, and crew's activation line names the same file as with the set's.
- "No sluice of the set's": with `services.sluice.package = null`, the profile holds no set's sluice, and sluice's file is still linked.

**2.4** "The tools' activation is their own":

- On nix-darwin, `t.darwinSystem` with crew on, its `postActivation` text with the runner's own link line taken out, equals `toolsDarwinSystem`'s, whose three tools are on with the set's packages, crew's settings and the primary user `alice`.
- On NixOS, `system.userActivationScripts`' names less `boomdrive` equal `toolsNixosSystem`'s, and each of `flume`, `sluice` and `crew` has the same text in both.

**2.5** `cache.nix`'s `overlay` gains `"crew"`, `hosts` turn crew on with `crewModule`, and `ownNixpkgs` holds `crew` with the rest.

**3.1** README, **Installing with nix**:

- The set holds crew too, and the profile line may add `github:agentplot/boomdrive#crew`.
- The modules bring Flume's, sluice's and crew's own modules, `services.flume`, `services.sluice` and `services.crew`. Each writes its own tool's file from its `settings`, named as that file names them, and links it at the tool's own path. A hand-written file there moves once to `config.toml.before-nix`, and for crew `teams.json.before-nix`.
- `services.boomdrive.enable` turns on Flume's and sluice's, and crew's is turned on with its `host` and partitions. The example gains a short `services.crew`, with neutral names.
- `services.boomdrive.tools` goes from the text, replaced by each tool's `package`, null for none, and `enable = false` to keep a tool's file one's own.
- The override paragraph:
  - A tool's version moves by its input, as now.
  - Flume's version and its git crates' hashes are Flume's flake's, so `flume-git-crates.nix` leaves the text.
  - A host's own package goes in that tool's `package`, sluice's override example among them.

`cache.nix`'s "The README's words" reads that section and expects `services.flume.package`, `services.sluice.package` and `services.crew.package` in it, and no `services.boomdrive.tools`.

**3.2** CLAUDE.md's paragraph on the flake: "gives the runner, Flume, sluice and crew pinned together, each tool's own module re-exported beside the runner's service, `services.boomdrive`". `openspec/config.yaml`'s stack line names crew and each tool's module in the same words.

**3.3** core.46's sentence and details, as Rules amended says.
