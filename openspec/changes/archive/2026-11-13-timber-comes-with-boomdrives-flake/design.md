# Design

## Context

The brief, queued by the planner from the user's ask of 2026-11-12, the first step of Timber's MVP, with no host having timber `v0.6.0` yet:

> Boomdrive's flake carries timber, its CLI and its herdr and Claude Code plugins from one pin of timber's flake, as it carries sluice and flume, so every host the set configures has timber on its path.

Its source is record 0020, which made this flake the way the family's host tools reach a host. Record 0020 asked timber to carry each release's version in its tagged tree. timber's record 0031 did, so a build of a tag reports that release, "Boomdrive's set among them".

What timber gives at `v0.6.0`, its newest tag, commit `d017f68c`, as read on 2026-11-13:

- **Its flake.**
  - **Inputs.** `nixpkgs` and `rust-overlay`, and nothing else.
  - **Systems.** It gives `packages.<system>.default` for `aarch64-darwin`, `x86_64-darwin`, `aarch64-linux` and `x86_64-linux`.
  - **No more.** It gives no module, app, shell or other package.
- **The package.**
  - **Its build.** It is a `buildRustPackage` of the binary crate `timber`, with tests off. Its toolchain is the Rust `.prototools` names, `1.98.1`, from `rust-overlay`.
  - **Its version.** Its `pname` is `timber` and its `mainProgram` `timber`. Its version is `Cargo.toml`'s `[workspace.package]`, `0.6.0` at the tag.
  - **Its revision.** `TIMBER_REVISION` is the flake's `self.rev`, so the binary names the locked commit beside its version.
  - **Its source.** Its `src` is the flake's own tree, whole.
- **Its herdr plugin**, `plugins/herdr`.
  - **The manifest** is id `agentplot.timber`, version `0.6.0`, `min_herdr_version` `0.9.3`, on Linux and macOS.
  - **One action and one pane.** The pane shows every repository the host's serving process knows, and one repository whole.
  - **What it runs.** Its scripts run under `sh` and find `timber`, `curl`, `jq` and herdr on `PATH`.
  - **The serving process.** Where nothing answers at `127.0.0.1:7420`, the pane starts `timber serve` once, detached, as any asker does under timber's statement 114. Where `timber` is not on `PATH`, the pane says so.
  - **No build steps.**
- **Its Claude Code plugin**, `plugins/claude-code`.
  - **The manifest** is name `timber`, version `0.6.0`.
  - **One hook.** A `PreToolUse` hook on Bash runs `scripts/guard.sh`, mode 0755. In a repository whose `HEAD` holds `.config/timber.toml`, the guard refuses an ecosystem's own publish command and names `timber publish`.
  - **What it runs.** It reads `jq` and `git` from `PATH`, lets everything through where `jq` is missing, and never runs `timber`.
  - **Its marketplace.** timber's root has its own marketplace, `.claude-plugin/marketplace.json`, named `timber`, listing it.
- **Under the checks.**
  - **It evaluates.** With the flake's `nixpkgs` and `rust-overlay` followed, the package evaluates on each of this flake's three systems. Each system's derivation is `timber-0.6.0` with `TIMBER_REVISION` `d017f68cccc798a24b8f4543319ad959c3d7ca59`.
  - **Offline.** Its `Cargo.lock` holds no git source, so its vendored crates need no network at evaluation.
  - **The toolchain.** The flake's locked rust-overlay carries `1.98.1`.

  Seen on 2026-11-13 with this flake's locked nixpkgs and rust-overlay.

The flake today, on `main` at `0ab26322`:

- **The set.** It holds the runner, Flume, sluice, crew, herdr, worktrunk, Ghostty, portless, the launcher and crew's companions.
- **The herdr plugins.**
  - `flume-herdr-plugin` and `sluice-herdr-plugin` are each its tool's `plugins/herdr`, made by `nix/packages/herdr-plugin.nix` from the tool's own package.
  - `herdr-plugins` holds those two for a profile, from which `boomdrive setup` links every plugin it holds.
- **The marketplace.** `boomdrive-claude-plugins` is the set's one Claude Code marketplace, `boomdrive`, holding Pending You's and Plannotator's plugins (record 0044, point 2). No module installs it into a Claude Code home yet: that is the unit `the-companions-load-in-claude-codes-sessions` (record 0044, point 3).
- **`nix/modules/apps.nix`.** It declares the Boomdrive module's own options for what it installs with the runner. Its `herdr.plugins` default is Flume's and sluice's plugins, then crew's companions' where crew's module is on.
- **The shell.** `nix/shell.nix` holds the set's commands.
- **The cache's workflow.** It builds every package `nix eval` lists for each system.

## Goals / Non-Goals

**Goals:**

- The set gains timber, through its own flake at its release tag, and its herdr plugin from the same source, each reporting `0.6.0`. The cache's workflow builds and pushes both.
- The set's marketplace lists timber's Claude Code plugin, from the same source.
- A host that turns on `services.boomdrive` gets `timber` in its system profile, which it can replace or drop, and timber's herdr plugin linked.
- The flake's shell has `timber`.
- The runner's code does not change.

**Non-Goals:**

- **Installing timber's Claude Code plugin into a Claude Code home.** That is the unit `the-companions-load-in-claude-codes-sessions`, which installs the set's marketplace (Decision 4).
- **timber's serving process as a service.** The modules start no `timber serve` and declare no service of it. timber's statement 114 lets a host run one of its own.
- **timber's own files.** That is a repository's `.config/timber.toml`, its policy, its store and the credentials of the accounts it publishes to. Each is the repository's or the person's.
- **The quick start's Mac install and `herdr-plugins`** (Decision 9).
- **`x86_64-darwin`.** timber's flake gives it, and this flake gives none (record 0020).
- **Any change to the runner's code, its file, its service, or what the runner's tests prove of the set.**

## Decisions

### 1. timber comes through its own flake, at its release tag

The flake takes:

```nix
timber = {
  url = "github:boomdrivedev/timber/v0.6.0";
  inputs = {
    nixpkgs.follows = "nixpkgs";
    rust-overlay.follows = "rust-overlay";
  };
};
```

- **One pin.** The lock gains the one node `timber`: both of its inputs follow this flake's, so the set keeps one nixpkgs and one rust-overlay, and the cache holds what the flake builds (core.47). Its CLI and both its plugins come from this one pin (Decisions 2 to 4).
- **Named by a release tag.** `nix/tests/tags.nix` holds timber to one, as it holds Flume, sluice, crew, herdr, portless and worktrunk (core.45).
- **`v0.6.0`.** It is timber's newest release, the one the user asked the hosts to have.
- **A host's tool, not a repository's pin.** timber serves every checkout on a host from one process at one fixed address (timber's 114), and its herdr pane and Claude Code plugin are the host's. No kit repository pins it in its `.prototools` for sluice to equip a place with, which core.44 and core.37 keep out of the set (record 0009 as amended on 2026-11-05).

Alternatives:

- **timber's own `nixpkgs` and `rust-overlay`.** Not taken: they add a second nixpkgs to the lock and to every evaluation, for a build the flake's own makes.
- **timber's `main`.** Not taken: core.45 names each tool by a release tag. `main` says `0.0.0`.
- **timber's release archive.** Not taken: core.44 rules out a prebuilt binary where the flake can make the derivation.

### 2. Its CLI is its flake's package, unchanged

`packages.<system>.timber` is `inputs.timber.packages.${system}.default`, unchanged, as Flume's and crew's are.

- **Its version.** Its tagged tree carries `0.6.0` (timber's record 0031), so its package is `timber-0.6.0` and `timber --version` says that release. The set gives it nothing (core.44; blueprints principle 14).
- **Its name.** It is `default` in timber's flake, and the set names it `timber`, its `pname`.

Alternative: **an override giving it a version or a revision.** Not taken: its tree carries both. Its revision is the locked commit, which `self.rev` gives it.

### 3. Its herdr plugin is made as Flume's and sluice's are

`packages.<system>.timber-herdr-plugin` is `nix/packages/herdr-plugin.nix` called with the set's `timber`:

- **What it is.** It is `plugins/herdr` of the source of timber's own package, copied whole, with timber's version.
- **Nothing rewritten.** Its scripts find `timber`, `curl`, `jq` and herdr on the user's `PATH`, as Flume's and sluice's find theirs.
- **Held to herdr.** Its `min_herdr_version`, `0.9.3`, is held to the set's herdr, `0.9.5`, beside Flume's and sluice's floors.
- **Held to timber.** The test holds its `herdr-plugin.toml`'s `version` to the set's timber's (Decision 5).

It is made outside the flake's `plugins` set, which `herdr-plugins` is called with, so `herdr-plugins` stays Flume's and sluice's (Decision 9).

Alternative: **its scripts patched to the store's `curl`, `jq` and `timber`.** Not taken: the set takes each tool's plugin as the tool ships it. A host whose `PATH` lacks `jq` lacks it for Flume's plugin too.

### 4. Its Claude Code plugin is in the set's marketplace

`nix/packages/claude-plugins.nix` is given the set's `timber`. The marketplace `boomdrive` lists a third plugin:

```
plugins/timber/…    plugins/claude-code of the set's timber's source, whole
```

- **Its entry.** It is `name` `timber`, `source` `./plugins/timber`, `version` the set's timber's, with a description of its own.
- **Copied as Plannotator's is.** It is copied with links followed, `cp -RL`, so `scripts/guard.sh` keeps its mode 0755 and the folder holds no link (record 0022 as amended on 2026-11-11). Every file is then made writable by its owner, as now.
- **Held to timber.** The build fails where its `.claude-plugin/plugin.json` says a version other than the set's timber's, as Plannotator's does. The test holds the same by evaluation (Decision 5).
- **Its source in `passthru`.** `passthru.sources.timber` gives the folder it copies.
- **The description.** The marketplace's description and `meta.description` name timber's plugin beside crew's companions'.
- **No install.** No Claude Code home gets it from this change. The unit `the-companions-load-in-claude-codes-sessions` installs the set's marketplace into each home (record 0044, point 3), and finds `timber` among its plugins. That unit is told so: what it installs of the marketplace, and for which sessions, is its own design.

Alternatives:

- **timber's own marketplace, `timber`, as a package of the set.** Not taken: each home would add two marketplaces, and record 0044 gives the set one of its own. timber's root marketplace stays timber's, for a person who adds it by hand.
- **A package of the plugin alone.** Not taken: Claude Code installs a plugin from a marketplace, not from a folder.

### 5. The tests hold timber's plugins to timber

A function `timberHeld { timber, plugins }` takes timber's version and a map from each plugin's name to its manifest's version. It gives one failure for each plugin whose version differs, naming the plugin, its version and timber's:

- **The set's case** reads `timber-herdr-plugin`'s `herdr-plugin.toml` and the marketplace's source's `.claude-plugin/plugin.json`, and holds both to the set's `timber`.
- **A given case** holds `timber` `0.6.0` beside a plugin saying `0.5.3`, and expects one failure naming both.

A timber tag whose release commit left a manifest behind fails the gate before a host links it.

### 6. The modules' own option installs it, with the runner

`nix/modules/apps.nix` declares `services.boomdrive.timber.package` by `installed "timber"`, as worktrunk's is:

| option | type | default |
|---|---|---|
| `services.boomdrive.timber.package` | package or `null` | the set's `timber` for the host's system |

- **Where it goes.** With `services.boomdrive.enable`, `environment.systemPackages` holds it where it is not `null`, beside the other apps. So the runner's user, and every user, finds `timber` on `PATH`.
- **Replaced or dropped.** A host's own package replaces it, and `null` installs none.
- **Without the runner** none is installed.
- **Not with crew.** It does not turn on crew: timber is not crew's companion.
- **Its words.** Its description says what timber is and what calls it.
- **No file of timber's.** The modules write none (core.18) and start no process of timber's (core.13).

The option is the Boomdrive module's own because timber gives no nix-darwin or NixOS module, as worktrunk gives none. Once timber's flake gives one, the set re-exports it as it does Flume's, and this option goes (the service-modules requirement that no option installs a tool its own module installs).

Alternatives:

- **Installed without the runner.** Not taken: a host that imports the module for one tool alone would get a `timber` it did not ask for.
- **Installed only with crew, as the companions are.** Not taken: a host that runs the runner under a hosted control plane lands work too, and timber delivers what lands.

### 7. timber's herdr plugin is linked with Flume's and sluice's

`services.boomdrive.herdr.plugins`' default gains `set.timber-herdr-plugin`, after `set.sluice-herdr-plugin` and before crew's companions'. It is there whether or not crew is on. The option's `defaultText` and description name it.

Alternative: **linked only where `services.boomdrive.timber.package` is not `null`.** Not taken: a host whose own `timber` replaces the set's still wants the pane. A host that wants no pane leaves the plugin out of the list, as for any plugin.

### 8. The shell holds timber

`nix/shell.nix`'s `packages` gain `set.timber`, after `set.crew`, and its header names timber among the set's tools on `PATH`.

Alternative: **leaving it out, as Ghostty is.** Not taken: Ghostty is a window the person in the shell is already in, and `timber` is a command the person and the agents run in a checkout.

### 9. The quick start and `herdr-plugins` stay as they are

The quick start's Mac install puts in a profile what its later steps and `boomdrive setup` run (the quick-start spec), and `herdr-plugins` holds those tools' plugins, which setup links. timber is in neither:

- no step of the quick start runs `timber`;
- a timber pane linked on a Mac whose profile holds no `timber` says only that timber is not on `PATH`.

The README names `github:boomdrivedev/boomdrive#timber` among what the Mac's profile line may add, as it names worktrunk and portless. It names `timber-herdr-plugin` beside the other plugins a host links by hand.

Alternative: **timber in the Mac's install and in `herdr-plugins`.** Not taken now: it gives every stranger's Mac a tool none of the quick start's steps runs, and changes the site's quick start. Adding it later is one line of each.

### 10. The words

- **The README's Installing with nix:**
  - timber in the set, through its own flake at its release tag, with its herdr plugin and its Claude Code plugin from the same tag;
  - `#timber` on the profile line's list of what it may add;
  - `timber-herdr-plugin` among the plugins a host links by hand;
  - the marketplace holding timber's plugin, installed into no Claude Code home yet;
  - with the runner, the modules install `timber` and link its pane;
  - `services.boomdrive.timber.package` among the options set to `null`;
  - timber's serving process: started by timber's pane or command where none answers, or run by a host as its own service, and never by the modules;
  - timber in the shell.
- **CLAUDE.md and `openspec/config.yaml`** name timber among the tools the flake pins together, and among what the modules install with the runner.

### 11. The tests

Every evaluated case is under `nix/tests`, run by `nix:typecheck` or `nix:test` against no store (record 0020's Decision 14), with core.41's neutral names. No case builds or runs timber.

- **The lock's new node** reaches the checks through `nix/inputs.nix`, which reads every node of the lock and gives a flake its `self` with the locked `rev`. It needs no change.
- **The case names.** Each case's name is new to the tests, since `default.nix` merges every file's cases into one set.

### Rules relied on

- **core.44 and core.45.** timber comes from its own flake at a release tag, moves only with the lock, and reports its tag's version. Its plugins are of the same tag (Decisions 1 to 5).
- **core.47.** The packages are made with the flake's own nixpkgs and rust-overlay, which timber's input follows. The workflow builds and pushes them with no change of its own.
- **core.37.** The set's timber is a derivation from source, no release archive.
- **core.18 and core.46.** The modules write no file of timber's, and their option holds no credential (core.4).
- **core.1, core.13 and core.29.** The runner runs no `timber` and starts nothing of timber's. timber's serving process is started by timber's own pane or command at a person's or an agent's asking. The link of timber's herdr plugin is the modules' activation's, by herdr's own command, as for the set's other plugins.
- **core.3.** timber's serving process listens on the loopback address, and is no socket of the runner's.
- **core.12.** The option names what it installs, `timber`, timber's own name.
- **core.7 and core.8.** No crate, gate tool or tracked file type is added: `.nix` is formatted already, and the checks evaluate timber without running it.
- **core.41.** Neutral names in every case.

### Rules amended

In the commit that writes this change:

- **core.13's details** do not rule out timber's serving process, `timber serve`. timber's herdr pane or a timber command starts it where none answers, at a person's or an agent's asking (timber's 114), and it is none of the runner's.
- **core.44's details** do not rule out:
  - timber through its own flake at its release tag, its package unchanged;
  - its herdr plugin from the same tag;
  - its Claude Code plugin, from the same tag, in the set's marketplace.

  Each is replaceable or dropped through the modules' options. timber is a host's tool, which no repository pins.
- **core.46's details** rule out timber's serving process started, kept or declared as a service by a module. They do not rule out the runner's module's own option for timber, which gives no nix-darwin or NixOS module, until timber gives one.
- **Record 0023** gains an amendment of 2026-11-13. It carries the brief's ask and holds Decisions 1 to 9 as the writing agent's choices, each cheap to change, with the path not taken.
- **Record 0044** gains an amendment of 2026-11-13 saying the set's marketplace also lists timber's plugin, which its point 3's unit finds there.

### Rules departed from

None.

## Risks / Trade-offs

- **Two timbers on a host.**
  - **Shadowed.** A host that keeps a `timber` of its own in a user's profile has it ahead of the system profile's on `PATH`, and the set's goes unused. Such a host sets `services.boomdrive.timber.package` to `null` or to its own, which the README says.
  - **One serving process.** Two timbers of different versions on one host both reach the one serving process at the one address, which the first asker started.
- **The pane's `curl` and `jq`.** On a host whose `PATH` holds neither, as a NixOS host may, the pane fails in its shell's words. The set takes the plugin as timber ships it, and the host adds both, as Flume's plugin already needs `jq`.
- **timber's build in the cache.**
  - **A Rust build.** Each system's job builds timber's crates, AWS's SDK among them, until its paths are pushed.
  - **The x86_64-linux job's room.** That job ran out of disk before record 0020's amendment of 2026-11-13 freed it. timber adds to what its store holds. Should a run's log show the room short again, that amendment names the next form.
- **timber's toolchain follows the flake's rust-overlay.** A timber tag whose `.prototools` names a Rust the locked rust-overlay lacks fails `nix:typecheck` until rust-overlay moves. That holds back timber's tag, never a host.
- **The plugin waits for its install.** Until `the-companions-load-in-claude-codes-sessions` lands, no session loads timber's guard from the set. A session in a timber repository can still run an ecosystem's own publish command, unless the person added timber's own marketplace.
- **The old addresses in timber's `v0.6.0`.**
  - **Its words.** Its plugin manifests and package `meta` name `github.com/agentplot/timber`, which moved to boomdrivedev after the tag.
  - **Its herdr plugin's id** is `agentplot.timber`.

  The set carries them as given, and timber's next release carries the new address.

## Migration Plan

A host takes the change by moving its Boomdrive input. With `services.boomdrive.enable`, its first switch:

- installs the set's `timber` in the system profile;
- links timber's herdr plugin at its activation. A herdr already running takes it at its next start.

A host with a `timber` of its own sets `services.boomdrive.timber.package` to its own, or to `null`. afterthought/swancloud takes this with its next move of the input.

## Open Questions

None.

## Task notes

**0.** Every evaluated case runs against no store, as `nix/check.sh` does. No check builds a package or reaches the network. Two steps reach it, each in the devenv shell outside the sandbox:

- locking timber's input (1.1);
- building once on this host what the gate never builds:
  - `packages.aarch64-darwin.timber`, before 1.1 is ticked, and its `bin/timber --version`, which says `0.6.0`;
  - `packages.aarch64-darwin.boomdrive-claude-plugins`, before 2.1 is ticked, whose build runs the copy and the version check of Decision 4.

  The summary names each build's result. A build that fails stops the task, as a failing check does.

**1.1** In `flake.nix`:

- Add the `timber` input of Decision 1, after `worktrunk`, with a comment. The comment says timber comes through its own flake at its release tag, its CLI and both its plugins from this one pin (record 0023 as amended on 2026-11-13; design Decision 1).
- Lock it with `nix flake lock` in the devenv shell, outside the sandbox, which adds the missing input and moves no other. The lock gains the one node `timber`, its `nixpkgs` and `rust-overlay` following the root's, and no second nixpkgs or rust-overlay.
- `packagesOf`'s `tools` gains, after `crew`:

  ```nix
  # timber's own flake's package, unchanged: its tagged tree carries its
  # version (timber's record 0031; core.44).
  timber = inputs.timber.packages.${system}.default;
  ```

- The outer set, beside the companions' herdr plugins, gains `timber-herdr-plugin = pluginOf tools.timber;`, with a comment saying it is timber's `plugins/herdr` at its tag, as Flume's and sluice's are, and outside `plugins`, so `herdr-plugins` stays theirs (design Decisions 3 and 9).
- The header comment and `description` name timber among the tools at a release tag through its own flake, and its two plugins.

`nix/tests/typecheck.nix`'s `set` gains `"timber"` and `"timber-herdr-plugin"`, in name order, after `"sluice-herdr-plugin"`. `tags.nix`'s list gains `"timber"`, and its header names it.

**1.2** In `nix/tests/packages.nix`, beside their neighbours:

- **"timber through its own flake"**, for each system, with `own = inputs.timber.packages.${system}.default`:
  - `timber.drvPath == own.drvPath`;
  - `timber.version` is `t.lockedVersion "timber"`;
  - `timber.meta.mainProgram` is `timber`.

  Each failure names the system and what it found.
- **"timber's plugin moves with timber"** is `each (pluginOf "timber")`.
- **`floorsOf`** gains `"timber-herdr-plugin"`, so "herdr is Flume's" holds its floor to the set's herdr.
- **"timber named by no release tag"** is built as "worktrunk named by no release tag" is, with timber's URL `github:boomdrivedev/timber/main` and the lock's node's `original.ref` `main`.

In `nix/tests/workflow.nix`, `pushed` gains `"timber"` and `"timber-herdr-plugin"`, so "The runner is pushed with its tools" holds each system's job to build both.

**2.1** In `nix/packages/claude-plugins.nix`:

- It takes `timber`. `plugins` gains, last:

  ```nix
  {
    name = "timber";
    source = "./plugins/timber";
    inherit (timber) version;
    description = "timber: an ecosystem's own publish command refused in a timber repository, naming timber publish";
  }
  ```

- `timberPlugin = "${timber.src}/plugins/claude-code"`. `passthru.sources.timber` is it.
- The build runs `cp -RL ${timberPlugin} "$out/plugins/timber"` after Plannotator's copy and before `chmod -R u+w "$out"`. After that, it checks `jq -r .version "$out/plugins/timber/.claude-plugin/plugin.json"` against `timber.version` as Plannotator's is checked, failing with "timber's plugin says <v>, not the set's timber <version>".
- The marketplace's `description` is "The Boomdrive set's Claude Code plugins: crew's companions', Pending You and Plannotator, and timber's". `meta.description` is "The Boomdrive set's Claude Code marketplace". The header comment names timber's plugin and this design's Decision 4.

`flake.nix`'s call gains `inherit (tools) pendingyou plannotator timber;`, and its comment on the marketplace names timber's plugin.

In `nix/tests/companions.nix`, the case "The marketplace's two plugins" becomes "The marketplace's three plugins". Its `expected` gains `{ name = "timber"; inherit (packages.timber) version; source = "./plugins/timber"; }`, last.

**2.2** `nix/tests/timber.nix` is added, and `nix/tests/default.nix`'s `test` imports it after `companions.nix`. Its header names the family-packages cases it holds and this design's Decisions 4 and 5. It holds:

- **`timberHeld`**, as Decision 5 gives it. Each failure reads "<plugin> says <v>, and the set's timber is <version>".
- **"timber's plugins say the set's timber"**, for each system:
  - `timberHeld` over `timber-herdr-plugin`'s `(builtins.fromTOML (builtins.readFile "${packages.timber-herdr-plugin.src}/herdr-plugin.toml")).version`;
  - and `(builtins.fromJSON (builtins.readFile "${market.sources.timber}/.claude-plugin/plugin.json")).version`, with `market = packages.boomdrive-claude-plugins`.
- **"A timber plugin other than the set's timber"**: `timberHeld { timber = "0.6.0"; plugins.timber-herdr-plugin = "0.5.3"; }` gives exactly one failure, naming `timber-herdr-plugin`, `0.5.3` and `0.6.0`.
- **"timber's Claude Code plugin from its command's source"**, built as "Plannotator's plugin from its command's source" is:
  - `market.sources.timber` is `"${packages.timber.src}/plugins/claude-code"`, both with their string context discarded;
  - `market.buildCommand` holds `cp -RL <that source> "$out/plugins/timber"`.

**3.1** In `nix/modules/apps.nix`:

- `options.services.boomdrive.timber.package = installed "timber" "<description>"`. The description: "timber, which delivers what the merge queue lands, and whose herdr pane and Claude Code plugin call it, installed with the runner; a host's own replaces it, and `null` installs none. Its serving process, `timber serve`, is timber's own to start."
- `environment.systemPackages`' list gains `cfg.timber.package`, after `cfg.worktrunk.package`.
- The header comment names timber among the apps, which gives no nix-darwin or NixOS module, citing record 0023 as amended on 2026-11-13 and this design's Decision 6.

In `nix/tests/apps.nix`, each case `onEach`, as worktrunk's are:

- **"timber with the runner"**: `system { }` evaluates with no failure, and its profile holds `set.timber.outPath`.
- **"A host's own timber"**: `services.boomdrive.timber.package` is `pkgs.runCommand "own-timber" { } "mkdir -p $out/bin"`, given as a module function as `hostsWorktrunk`'s is. The profile holds the option's `outPath` and not `set.timber.outPath`.
- **"No timber of the set's"**: with the option `null`, the system evaluates with no failure, and its profile holds no `set.timber.outPath`.
- **"No timber without the runner"**: `without`'s profile holds no `set.timber.outPath`.
- **"No serving process of the modules'"**, built as `noProxy` is: no step `stepsOf platform (system { })` gives holds the set's timber's `outPath`, its string context discarded. The plugins' step names `timber-herdr-plugin`, another path, and holds it not.

The file's header names timber and this change.

In `nix/tests/cache.nix`, `names` gains `"timber"`, so the host's overlay gives a `timber` of its own, and "A host's own nixpkgs" holds the profile to the set's.

**3.2** In `nix/modules/apps.nix`, `herdr.plugins`' default is `[ set.flume-herdr-plugin set.sluice-herdr-plugin set.timber-herdr-plugin ]`, then crew's companions' plugins where crew is on. Its `defaultText` gains the same line. Its description reads "Flume's, sluice's and timber's, then, where crew's module is on, crew's companions', …".

In the tests, each list of the plugins the modules link names `timber-herdr-plugin` after `sluice-herdr-plugin`:

- `nix/tests/apps.nix`: `stepOf`'s step; `noCompanionsWithoutCrew`'s `expected`, whose words become "not Flume's, sluice's and timber's alone"; and `companionsPlugins`' `plugins`;
- `nix/tests/nixos.nix`: `keeps`' plugins line;
- `nix/tests/cache.nix`: `plugins`, whose order is the order linked, so the host's overlay gives a `timber-herdr-plugin` of its own and "A host's own nixpkgs" holds the plugins linked to the flake's own.

**3.3** `nix/shell.nix`'s `packages` gain `set.timber` after `set.crew`. Its header names timber among the set's tools on `PATH`, which the person and agents run in a checkout. In `nix/tests/way-in.nix`, `shellTools` gains `"timber"`.

**4.1** The README's **Installing with nix**, as Decision 10 lists:

- **The set's list** gains, after worktrunk: "timber, through its own flake at its release tag, which delivers what the merge queue lands, with its herdr plugin, `timber-herdr-plugin`, from the same tag". The marketplace's item names timber's plugin beside the companions'.
- **The profile line's list** of what it may add gains `github:boomdrivedev/boomdrive#timber`.
- **The herdr plugins' paragraph** names `timber-herdr-plugin` beside Flume's and sluice's, at its tool's tag.
- **The modules' list** gains: "It installs timber, `services.boomdrive.timber.package`, and its herdr plugin is linked with Flume's and sluice's." The count before the block of `null`s becomes six, and the block gains `services.boomdrive.timber.package = null;`.
- **timber's serving process.** One paragraph says it is timber's own: timber's pane, or a timber command, starts `timber serve` where nothing answers at `127.0.0.1:7420`, and a host may run it as a service of its own. The modules start none. A host with a `timber` of its own sets the option `null` or to its own, so one `timber` runs.
- **The marketplace's paragraph** says it lists timber's plugin too, installed into no Claude Code home yet, as the companions' are.
- **The shell's paragraph** names timber.

In `nix/tests/cache.nix`:

- "The README's words"' list gains `"services.boomdrive.timber.package"`, after worktrunk's;
- **"The README's timber"**, built as "The README's proxy" is, holds that the README's Installing with nix names `github:boomdrivedev/boomdrive#timber`, `timber-herdr-plugin` and `timber serve`.

**4.2** CLAUDE.md's paragraph on the flake reads "gives the runner, Flume, sluice, crew and timber pinned together, with herdr, its plugins, …", and its modules "install herdr, Ghostty, portless, worktrunk, timber and the Boomdrive launcher". Its sentence on the marketplace names timber's plugin. `openspec/config.yaml`'s stack paragraph names the same.
