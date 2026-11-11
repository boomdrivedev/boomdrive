# Design

## Context

Record 0023 (accepted 2026-11-06) has the flake install herdr and Ghostty and give the person one way in:

- **herdr** comes at the version Flume's terminal profile was checked against, with Flume's and sluice's herdr plugins.
- **Ghostty** is named Flume's terminal app where the host names none.
- **Each is overridable.** A host replaces either through the modules' options, or drops it for its own.
- **The launcher.** A Boomdrive app, in `/Applications/Nix Apps` on macOS and as a desktop entry on Linux, runs `flume open`.
- **The shell.** A shell of the flake's, its `PATH` the set's tools, starts herdr on entering.
- **The runner's part.** Each runs at the person's asking. The runner still starts no multiplexer (core.13) and names none (core.29).

The user's words are in the research talk (`design/research/the-flakes-configuration-holds-streams-and-repositories.md`, "herdr and a way in"): "In the set, overridable", "An app to click", "A shell" and "Install Ghostty too".

The flake today, after the unit that brings each tool's own module:

- **The set** is `boomdrive`, `flume`, `sluice`, `crew` and the two herdr plugins. `typecheck.nix` fails a set holding herdr or git, the host's own (record 0012, as core.44 stood).
- **The plugins.** `nix/packages/herdr-plugin.nix` copies a tool's `plugins/herdr`. Its header leaves linking it into herdr to the host.
- **Flume's module** at the set's tag, `v0.18.0`, declares `host.multiplexer`, the terminal profile, and `host.terminal_app`, the window profile. Flume's statement C.7 reads `[host] terminal`, the word a host wrote before, as `multiplexer`, and refuses a host that gives both.
- **`flume open [<session>]`** (Flume's statement 186) shows a session in the view the host's configuration names. Asked for no name, it shows the host's default stream's own session, started first where it does not run. Its view is a window of the stream's or the host's `terminal_app`, else the line to run.
- **No module installs herdr or Ghostty.** nixpkgs' NixOS modules and nix-darwin's hold no module of either.

What each tool gives, as seen on 2026-11-08:

- **herdr** publishes a flake at its release tag `v0.9.5`, with `packages.<system>.herdr` for each of the flake's systems. It takes `nixpkgs` and `rust-overlay`, and its package's version is its tagged `Cargo.toml`'s, `0.9.5`.
- **Flume's pin.** Flume's `devenv.nix` at `v0.18.0` pins `herdrVersion = "0.9.5"`, "the release its commands are checked against". Flume's plugin asks `min_herdr_version = "0.9.5"`, and sluice's, at `v0.76.0`, `"0.9.3"`.
- **Ghostty's flake** at `v1.4.0` builds Ghostty for Linux alone (`buildablePlatforms` leaves out darwin). It brings five inputs of its own: zig-overlay, zon2nix, home-manager, flake-compat and systems.
- **The flake's locked nixpkgs** carries `ghostty` 1.4.0, built from source for Linux, and `ghostty-bin` 1.4.0 for darwin, Ghostty's own notarized `Ghostty.dmg` pinned by its hash. It also carries `herdr` 0.9.5.
- **Flume's ghostty window profile** was checked against Ghostty 1.4.0 (Flume's statement, C.8). On macOS it runs `open -na Ghostty.app`.
- **`herdr plugin link <path>`**, herdr 0.9.5's own command, records the plugin in herdr's own `~/.config/herdr/plugins.json`. Seen on a scratch config with no server:
  - it links with no server running, and starts none;
  - linking a path whose plugin id is already linked replaces that link;
  - a relink enables a plugin the person had disabled.
- **nix-darwin** places the `Applications` of `environment.systemPackages` in `/Applications/Nix Apps`. NixOS links `share/applications` of the system profile while `xdg.menus.enable`, its default, holds.

This change is the unit `herdr-and-ghostty-come-with-the-flake-and-an-app-opens-the-agents` of the bolt `the-boomdrive-flake-configures-the-whole-host`. It comes after the unit that brings each tool's own module.

## Goals / Non-Goals

**Goals:**

- The set gains herdr, at the herdr Flume's tree pins, and Ghostty, at the release Flume's window profile was checked against. Each comes from where core.44 and this design say, and is built and pushed by the cache's workflow.
- A host that turns on `services.boomdrive` gets herdr with the set's plugins linked, Ghostty and the Boomdrive launcher, with Flume told to use herdr and Ghostty. It can replace or drop each.
- One click on the launcher shows the host's agents. `nix develop github:agentplot/boomdrive` drops a person into herdr with the set on `PATH`.
- The runner's code does not change, and starts and names no multiplexer.

**Non-Goals:**

- Any key of Flume's file but `host.multiplexer` and `host.terminal_app`. Each repository's stream is the unit `each-repository-is-named-once-by-its-owner`'s.
- Unlinking a plugin a host drops from the list, and sending a running herdr server the plugins' `ensure` actions.
- An icon for the launcher, a signed app bundle, and a launcher that asks for a session by name.
- Ghostty's or herdr's own configuration, `~/.config/ghostty/config` and herdr's `config.toml`.
- A `nix run` of the flake, which the user did not choose.
- Any change to the runner's code, its file, its agent or unit, or what the runner's tests prove of the set.

## Decisions

### 1. herdr comes through its own flake, at the herdr Flume's tree pins

The flake takes `herdr = { url = "github:herdrdev/herdr/v0.9.5"; inputs = { nixpkgs.follows = "nixpkgs"; rust-overlay.follows = "rust-overlay"; }; }`. `packages.<system>.herdr` is `inputs.herdr.packages.${system}.herdr`, unchanged. Its version is its tagged `Cargo.toml`'s, which is the tag's.

- **Pinned to Flume's.** "The version Flume's terminal profile was checked against" is Flume's `devenv.nix` `herdrVersion` at the tag the lock names for Flume. The test reads that line from Flume's input and holds the set's herdr to it. A Flume tag that pins a newer herdr fails the gate until herdr's tag moves with it.
- **No plugin asks for more.** The test holds every herdr plugin of the set to ask, by its manifest's `min_herdr_version`, for no newer herdr than the set's.
- **Named by a release tag.** `nix/tests/tags.nix` holds herdr to one, as it holds Flume, sluice and crew (core.45).

Alternatives:

- **nixpkgs' herdr 0.9.5.** Not taken: core.44 takes a tool from its own flake where it publishes one. nixpkgs' herdr moves with nixpkgs, not with a tag the lock names.
- **Flume's own herdr**, its `devenv.nix`'s release archive. Not taken: core.44 rules out a prebuilt binary in place of a derivation the flake can make.

### 2. Ghostty comes from the flake's nixpkgs

`packages.<system>.ghostty` is the flake's own nixpkgs' Ghostty, made with the same `pkgs` as the rest of the set:

- `pkgs.ghostty`, built from source, on `aarch64-linux` and `x86_64-linux`.
- `pkgs.ghostty-bin` on `aarch64-darwin`. That is Ghostty's own notarized release of the same version, `Ghostty.dmg`, pinned by its hash in the locked nixpkgs. Ghostty's macOS app builds only with Xcode, so neither nixpkgs nor Ghostty's own flake builds it from source.

The test holds each system's Ghostty to `1.4.0`, the release Flume's ghostty window profile was checked against (Flume's C.8). The constant stands in the test, with a comment naming C.8. Flume's tree pins no Ghostty to read. A nixpkgs that moves Ghostty fails the gate until the constant moves with a check of Flume's profile.

This departs from core.44's sentence, "taken from the tool's own flake where it publishes one", for Ghostty on Linux. It departs from its details' rule against a release archive "in place of the flake's own derivation" for Ghostty on macOS, where no derivation builds one. core.44's details are amended to say so (Rules amended).

Alternatives:

- **Ghostty's own flake on Linux, and `ghostty-bin` on macOS.** Not taken:
  - Ghostty's flake gives nothing for macOS, the platform record 0023's fresh Mac needs, so it would be one of two sources.
  - It brings five inputs to the lock and its follows, for one package that nixpkgs builds from the same release's source.
- **A recipe of this flake's for Ghostty's dmg.** Not taken: it repeats nixpkgs' `ghostty-bin`, a recipe kept here where one is kept already.
- **Ghostty left the host's own.** Not taken: the user chose "Install Ghostty too" over the host's own terminal app.

### 3. The launcher is a package of this tree

`packages.<system>.boomdrive-launcher` is made by `nix/packages/launcher.nix`, with the set's `pkgs`, from one script, `nix/packages/launcher.sh`.

What the script does:

- It puts the running user's profiles and the system's ahead of the `PATH` it is given: `/etc/profiles/per-user/$(id -un)/bin`, `$HOME/.nix-profile/bin`, `$HOME/.local/state/nix/profile/bin`, `/run/wrappers/bin`, `/run/current-system/sw/bin` and `/nix/var/nix/profiles/default/bin`. An app macOS opens is given `/usr/bin:/bin:/usr/sbin:/sbin`, so this is how it finds the host's one Flume, as the runner's service does (design Decision 11 of record 0020's change).
- It runs `flume open` once, with no session, so Flume shows the default stream's own session (186). Flume reads the host user's login environment for what it runs (Flume's clause 29), so the launcher passes nothing else.
- Where Flume exits 0, it shows nothing.
- Where Flume exits otherwise, it shows Flume's words, its standard output and error as Flume gave them, and exits 1. On macOS (`uname -s` is `Darwin`) they go in an alert, `osascript` running `display alert "Boomdrive" message <words>` with the words passed as an argument and never spliced into the script. Elsewhere they go on standard error, which the desktop session's log keeps.

What the package holds:

- **On `aarch64-darwin`:** `Applications/Boomdrive.app/Contents/Info.plist` and `Contents/MacOS/boomdrive-launcher`, the script.
  - The plist names `CFBundleName` and `CFBundleDisplayName` `Boomdrive`, `CFBundleExecutable` `boomdrive-launcher`, `CFBundleIdentifier` `io.github.agentplot.boomdrive`, `CFBundlePackageType` `APPL`, and `LSUIElement` true.
  - `LSUIElement` true means a click shows no Dock icon for a script that runs and ends.
  - nix-darwin places the app in `/Applications/Nix Apps`.
- **On each Linux system:** `share/applications/boomdrive.desktop`, made by `makeDesktopItem`. It is named `Boomdrive`, its `Exec` the script, its `Terminal` false.

Each part is a `writeTextFile` with its `destination`. A `runCommand` copies each whole into `$out`, since an app bundle's parts are files, not links. The parts are given as `passthru.parts`, which the evaluated cases read. The package's version is the runner's `Cargo.toml`'s, as the runner's is.

It is a package of the set, not made in the module, so it is a path the family's cache holds (core.47). It depends on no host's settings.

Alternatives:

- **The app's executable running `flume open` with the store path of the set's Flume.** Not taken: a host that gives its own Flume would open with another Flume than the one its agents run under.
- **The launcher starting herdr, or attaching to a session itself.** Not taken: the session's name and view are Flume's to choose (186), and core.29 keeps the runner's side naming no multiplexer.
- **An app made with the host's `pkgs` in the module.** Not taken: core.47 rules out a module's package made with the host's `pkgs`.
- **The option named `app`.** Not taken: Flume uses `app` for a multiplexer's own app as a view (186), and core.12 keeps a word to one thing. The launcher's option is `launcher`, and its app is `Boomdrive`.

### 4. The shell

`devShells.<system>.default`, for each of the three systems, is `nix/shell.nix`. It is a `mkShellNoCC` made with the set's `pkgs`. Its `packages` are the set's `boomdrive`, `flume`, `sluice`, `crew` and `herdr`, and its `shellHook` is `nix/shell-hook.sh`, read in whole.

The hook runs `herdr` with no argument, herdr's own start, where the shell is interactive (`$-` holds `i`) and `HERDR_ENV` is unset. When herdr ends, by its exit or a detach, the person is in the shell with the set on `PATH`. A command run through the shell starts no herdr: `nix develop --command` runs its command in a bash that is not interactive, and direnv's `use flake` sources the hook in one too. A shell entered from a herdr pane starts none.

The shell links no plugin: the modules do. Ghostty and the launcher are not in it, since a person in the shell is in a terminal already. The repository's own development shell stays devenv's (`devenv.nix`), and the flake's header says so.

Alternatives:

- **`exec herdr`.** Not taken: a detach would end the person's shell.
- **Attaching to Flume's default stream's session.** Not taken: the session's name is Flume's (186), and a session Flume's terminal profile names is a word of Flume's the shell would copy. The launcher is the way into Flume's own session.
- **Linking the set's plugins in the hook.** Not taken: a shell of another revision than the host's modules would relink the host's herdr to plugins of tools the host does not run.
- **A `nix run` app.** Not taken: the user chose "A shell" over a `nix run` with nothing installed.

### 5. The modules' own options for herdr, Ghostty and the launcher

`nix/modules/way-in.nix`, given `packages`, is imported by `darwin.nix` and `nixos.nix` beside `options.nix` and `tools.nix`. It declares:

| option | type | default |
|---|---|---|
| `services.boomdrive.herdr.package` | package or `null` | the set's `herdr` for the host's system |
| `services.boomdrive.herdr.plugins` | list of packages | the set's `flume-herdr-plugin` and `sluice-herdr-plugin` |
| `services.boomdrive.ghostty.package` | package or `null` | the set's `ghostty` |
| `services.boomdrive.launcher.package` | package or `null` | the set's `boomdrive-launcher` |

With `services.boomdrive.enable`, it puts each non-null package in `environment.systemPackages`. A host's own package replaces the set's, and `null` drops it for the host's own. Without the runner it installs none.

The options are the Boomdrive module's own because neither herdr nor Ghostty gives a nix-darwin or NixOS module, and the launcher is the runner's flake's. The service-modules requirement that the modules hold no option installing another tool is narrowed to tools whose own module installs them.

Each has its `defaultText` in `options.nix`'s manner.

Alternatives:

- **`programs.herdr` and `programs.ghostty`.** Not taken: a later nixpkgs, nix-darwin or home-manager module of either name would collide with them.
- **Installing them whether or not the runner is enabled.** Not taken: a host that imports Boomdrive's module for one tool alone would get a terminal and an app it did not ask for.

### 6. The set's plugins are linked by herdr's own command at activation

With `services.boomdrive.enable`, a non-null `herdr.package` and a non-empty `herdr.plugins`, the activation runs `activate.sh plugins <herdr> <plugin>…`. `<herdr>` is `${herdr.package}/bin/herdr` and each plugin is its store path, in the list's order.

- **On nix-darwin** it is a line of `system.activationScripts.postActivation`, run as the primary user under their own home, as the runner's link is, and `|| true`.
- **On NixOS** it is a line of the runner's own `system.userActivationScripts.boomdrive`, inside its `id -un` guard, so the runner's user alone runs it.

The step, for each plugin:

- it runs `<herdr> plugin link <plugin>` with standard input closed and every `HERDR_*` variable of its caller's unset, so it reaches the user's own herdr home;
- it discards herdr's answer where the link holds;
- where herdr refuses, it says one line on standard error, `boomdrive: herdr did not link <plugin>: <herdr's words>`, and goes on.

The step exits 0. Each failure is said and stops nothing, as the runner's own link does.

herdr's `plugin link` records the plugin in herdr's own `plugins.json` and starts no server (Context). So the module writes no file of herdr's (core.18) and starts no multiplexer (core.13, core.46). Each activation relinks, so a link follows the plugin's store path as the set moves. A host that wants one of them off drops it from `plugins`, since a relink enables a plugin the person disabled. A host whose own Flume or sluice is another version gives that tool's plugin in `plugins`.

`nix/packages/herdr-plugin.nix`'s header says the modules link it with herdr's own command, where it said the host runs that command.

Alternatives:

- **Nix writing herdr's `plugins.json`.** Not taken: it is herdr's file, and core.18 has only a tool's own module, a person or a control plane write a tool's file.
- **Flume's and sluice's modules each linking its own plugin.** Not taken: herdr is this flake's in the set, and neither tool's module installs herdr. It would ask two repositories for a link this flake can make with the herdr it installs.
- **Linking only a plugin not yet linked from its path, so a person's disable holds.** Not taken: it reads herdr's list on every switch, for a case a host meets by dropping the plugin from the list.
- **Unlinking a plugin the list drops.** Not taken here: it needs herdr's list read and its store paths matched. A dropped plugin stays linked until the person runs `herdr plugin unlink`, which the README says.

### 7. Flume is told herdr and Ghostty

With `services.boomdrive.enable`, `way-in.nix` sets two keys of `services.flume.settings.host`, each by `lib.mkDefault`, so a host's own value wins:

- `multiplexer = "herdr"`, unless the host's own `services.flume.settings.host` gives `terminal`. That is Flume's word of before, which Flume reads as `multiplexer` and refuses beside it (Flume's C.7). The condition reads `terminal` alone, a key Flume's module does not declare. So it does not depend on the definition it guards, and evaluation does not recurse.
- `terminal_app = "ghostty"`.

Record 0023 says the module names Flume's "`terminal` and `terminal_app`". Flume's file has since named its terminal profile `multiplexer` (C.7), which its module at the set's tag declares, so the module sets that word. Each default holds whether or not the host keeps the set's herdr and Ghostty. A host that drops either "for its own", as record 0023 says, still runs herdr and Ghostty. A host with neither sets its own words. `host.view` is left Flume's own, `terminal`, a window running the multiplexer's attach.

Alternatives:

- **The defaults only where the set's packages are installed.** Not taken: a host that drops herdr for its own herdr would lose Flume's terminal.
- **The defaults only where the host names neither key.** Not taken: each key is its own choice. A host that names its own window app still runs its agents in herdr.

### 8. The tests

Every evaluated case is under `nix/tests`, run by `nix:typecheck` or `nix:test` against no store (record 0020's Decision 14), with core.41's neutral names. The sh cases run stubs in scratch folders, as `nix/tests/activate.sh` does. No case runs a real herdr, Flume, Ghostty or `osascript`, so no tool is added to the gate (core.8).

- **`typecheck.nix`.**
  - The set gains `boomdrive-launcher`, `ghostty` and `herdr`.
  - `hosts` holds `git` alone.
  - Each system's `devShells.default` evaluates.
- **`packages.nix`** holds the family-packages delta's cases:
  - "herdr through its own flake", "herdr is Flume's", "herdr other than Flume's" and "A plugin newer than herdr";
  - "Ghostty on each system" and "Ghostty moved with nixpkgs";
  - "herdr named by no release tag".
  - The herdr and Ghostty checks are functions, called with the set's values for the cases that hold and with given ones for the cases that fail.
- **`tags.nix`'s** tools named by a release tag gain `herdr`.
- **`nix/tests/way-in.nix`**, added to `default.nix`'s `test`, holds:
  - the way-in spec's evaluated cases: "The app on macOS", "The desktop entry on Linux" and "Every system's shell";
  - the service-modules delta's added cases, each on nix-darwin and on NixOS unless it names a platform.
- **`nix/tests/way-in.sh`**, run by `nix/check.sh test` after `activate.sh`, holds:
  - the launcher's cases, with stubs of `flume`, `osascript`, `uname` and `id`;
  - the shell hook's cases, sourcing `nix/shell-hook.sh` in `bash -i -c` and `bash -c` with a scratch `HOME` and a stub of `herdr`.
- **`nix/tests/activate.sh`** gains "Each plugin linked" and "A link herdr refuses", with a stub of `herdr`.
- **`tools.nix`'s** "The tools' activation is their own on nix-darwin" takes the plugins' line out with the runner's link before comparing.
- **`cache.nix`'s** "A host's own nixpkgs" holds `herdr`, `ghostty` and `boomdrive-launcher` to the flake's own, and "The README's words" reads the options the delta names.

### Rules relied on

- **core.13 and core.29.** The runner's code is unchanged and runs no herdr. The launcher, the shell and the activation's `plugin link` each run at a person's asking or a person's switch, and none starts a server for the runner (Decisions 3, 4 and 6).
- **core.18 and core.46.** herdr's file is written by herdr's own command. Flume's file is written by Flume's module, from options the Boomdrive module sets by default (Decisions 6 and 7).
- **core.44 and core.45.**
  - herdr comes from its own flake at a release tag, moved only with the lock and held to Flume's pin.
  - Ghostty comes from the flake's nixpkgs (Decisions 1 and 2, as Rules amended says).
  - The launcher is of this tree.
- **core.47.** Every new package is made with the flake's own nixpkgs, and the workflow builds and pushes it with no change of its own.
- **core.12.** The options name what they install, `herdr`, `ghostty` and `launcher`, and none takes Flume's `app` (Decision 3).
- **core.4.** No option holds a credential.
- **core.7 and core.8.** No crate, tool of the gate or tracked file type is added: `.sh` and `.nix` are formatted already.
- **core.41.** Neutral names in every case.

### Rules amended

In the commit that writes this change:

- **core.13's details** do not rule out a multiplexer's server that `flume open` starts when a person opens the launcher, or that herdr starts when a person enters the shell (record 0023).
- **core.29's details** do not rule out herdr installed by the modules, the set's plugins linked by herdr's own `plugin link`, and herdr started by the shell, none of which is the runner's code.
- **core.44's details** name where herdr, Ghostty and the launcher come from (Decisions 1 to 3). Ghostty on macOS is Ghostty's own release as nixpkgs' `ghostty-bin` gives it, where nothing builds its app from source.
- **core.46's details** do not rule out:
  - the runner's module's own options for herdr, its plugins, Ghostty and the launcher;
  - Flume's `host.multiplexer` and `host.terminal_app` set by default;
  - herdr's own `plugin link` run at activation.

### Rules departed from

core.44, for Ghostty alone, as Decision 2 says and its amended details now hold.

## Sequencing

The bolt's sibling units `each-repository-is-named-once-by-its-owner` and `crews-partitions-give-the-runner-its-crew-table` touch some of the same files:

- `nix/modules/darwin.nix` and `nixos.nix`, by their imports;
- `nix/tests/default.nix`, `cache.nix` and `tools.nix`;
- `README.md` and `CLAUDE.md`;
- the service-modules spec, by other requirements.

This change keeps its own options and their Flume keys in `way-in.nix` and its own cases in `way-in.nix` and `way-in.sh`, so what it shares with them is a line of each file. Whichever lands second merges those lines, and its `nix:test` holds both.

## Risks / Trade-offs

- **A running herdr server.** A herdr server already running when a switch relinks the plugins runs their startup at its own next start. The person restarts it, or runs a plugin's `ensure` action, to take a new version at once.
- **Plugins dropped from `plugins`** stay linked in herdr until the person unlinks them.
- **A relink enables a disabled plugin.** A host that wants a plugin off drops it from `plugins`.
- **Two Ghosttys on a Mac.** Where the host keeps a Ghostty of its own in `/Applications` beside the set's in `/Applications/Nix Apps`, `open -na Ghostty.app` opens the one Launch Services chooses. A host with its own sets `services.boomdrive.ghostty.package = null`.
- **A host with no display** gets Ghostty and the launcher with the runner, and Ghostty's libraries on Linux. It sets each `null`, and Flume's `host.terminal_app` its own, which the README says.
- **Ghostty's version is nixpkgs'.** A nixpkgs that moves Ghostty fails the gate until the test's constant moves with a check of Flume's window profile. That holds back a lock update for a check a person makes.
- **herdr's build in the cache.** herdr is built by each system's job of the workflow from its own flake, a Rust and Zig build, until its path is pushed. Ghostty's Linux path is nixpkgs' own, which the workflow substitutes from nixpkgs' cache where it holds it.
- **The launcher is unsigned**, as every app nix-darwin places is. macOS runs it, since nothing quarantines a store path.

## Migration Plan

A host takes the change by moving its Boomdrive input. With `services.boomdrive.enable`:

- **herdr.** Its first switch installs the set's herdr and links the set's plugins in the user's herdr. A host with herdr of its own sets `services.boomdrive.herdr.package` to it, or to `null` to keep its own entirely. In that case it links the plugins itself, or keeps its own linker.
- **Ghostty.** The first switch installs the set's Ghostty. A host with its own sets `services.boomdrive.ghostty.package = null`.
- **Flume's file** gains `[host] multiplexer = "herdr"` and `terminal_app = "ghostty"` unless the host's `services.flume.settings.host` names its own.
- **The launcher.** `Boomdrive` appears in `/Applications/Nix Apps` or the desktop's applications.

afterthought/swancloud, once it takes the flake, either drops its hosts' own herdr and its plugin linker for the set's, or passes its own herdr through these options and keeps its linker (record 0023).

## Open Questions

None.

## Task notes

**1.1** In `flake.nix`:

- Add the `herdr` input of Decision 1. Lock it with `nix flake lock --update-input herdr` in the devenv shell, outside the sandbox.
- Give `packagesOf`'s `tools` `herdr = inputs.herdr.packages.${system}.herdr;`.
- The header comment, the inputs' comment and the set's comment say git is the host's own, where they say herdr and git.

In `typecheck.nix`:

- `set` gains `"herdr"` in name order.
- `hosts` is `[ "git" ]`, with its comment citing record 0023.

**1.2** In `packages.nix`:

- `herdrHeld = { pinned, herdr, plugins }: [ failures ]`, where `plugins` maps a name to its `min_herdr_version`, each failure naming both versions, and the plugin where one asks for more.
- "herdr is Flume's" reads `pinned` from `${inputs.flume.outPath}/devenv.nix`. Split it into lines and match the one `herdrVersion = "<v>";`. A Flume tree with no such line fails, saying Flume's pin could not be read.
- It reads each plugin's floor from `builtins.fromTOML` of `${plugin.src}/herdr-plugin.toml`.
- "herdr other than Flume's" and "A plugin newer than herdr" call `herdrHeld` with given values.
- "herdr through its own flake": each system's `herdr` has the `drvPath` of `inputs.herdr.packages.<system>.herdr`, and its version is the lock's tag without the `v`.

In `tags.nix`, the list of tools named by a release tag gains `"herdr"`. "herdr named by no release tag" is built as "A tool named by no release tag" is, with herdr's URL `github:herdrdev/herdr/main` and the lock's node's `original.ref` `main`.

**1.3** In `flake.nix`, `packagesOf` gives `ghostty = if pkgs.stdenv.hostPlatform.isDarwin then pkgs.ghostty-bin else pkgs.ghostty;`, with a comment citing Decision 2. `typecheck.nix`'s set gains `"ghostty"`.

**1.4** In `packages.nix`:

- `ghosttyHeld = system: package: [ failures ]` holds `package.version` to `"1.4.0"`, named `checked`, with a comment citing Flume's C.8, and names both on a failure.
- "Ghostty on each system" holds each system's `ghostty` to `pkgs.ghostty-bin`'s or `pkgs.ghostty`'s `drvPath` from `inputs.nixpkgs.legacyPackages.<system>` (no overlay changes either), and to `ghosttyHeld`.
- "Ghostty moved with nixpkgs" calls `ghosttyHeld` with `{ version = "1.5.0"; }`.

**2.1** `nix/packages/launcher.sh` is Decision 3's script: POSIX `sh`, a header comment, `set -u`, no `-e`, so a refusal is shown. Its words are `$(flume open 2>&1)`. On `Darwin`, `osascript -e 'on run argv' -e 'display alert "Boomdrive" message (item 1 of argv)' -e 'end run' "$words"`.

`nix/tests/way-in.sh` is in `activate.sh`'s manner: a scratch folder for each case, stubs first on `PATH` recording their calls and environment, and each failure one line `nix: <case>: <what>`. Its cases:

- "Opened": `flume` exits 0, saying a line. Its call is `open` alone, and neither `osascript` nor standard error saw anything.
- "Flume refuses on macOS": `uname -s` says `Darwin`, and `flume` exits 1 saying the words. `osascript`'s last argument is those words, and its script holds `display alert "Boomdrive"`.
- "Flume refuses elsewhere": `uname -s` says `Linux`. Standard error holds the words, and `osascript` is not called.
- "The profiles first": `id -un` says `alice`, `HOME` is a scratch `/home/alice`-shaped folder, and `PATH` is the stubs' then `/usr/bin:/bin`. The `PATH` the `flume` stub records names `/etc/profiles/per-user/alice/bin` and `$HOME/.nix-profile/bin` before `/usr/bin`.

`nix/check.sh test` runs `sh nix/tests/way-in.sh` after `activate.sh`, and its header names it.

**2.2** `nix/packages/launcher.nix` is `{ lib, runCommand, writeTextFile, makeDesktopItem, stdenvNoCC }` (Decision 3), called from `packagesOf` as `boomdrive-launcher`.

- **The script.** A `writeTextFile` of `launcher.sh`'s text, executable. On darwin its `destination` is `/Applications/Boomdrive.app/Contents/MacOS/boomdrive-launcher`, and on Linux `/bin/boomdrive-launcher`.
- **`Info.plist`** is a `writeTextFile` with `destination` `/Applications/Boomdrive.app/Contents/Info.plist` and Decision 3's keys.
- **The desktop entry** is `makeDesktopItem { name = "boomdrive"; desktopName = "Boomdrive"; exec = "<the script's path>"; terminal = false; comment = "Open the host's agents"; }`.
- **The package** is a `runCommand` that copies each part's tree into `$out` with `cp -R --no-preserve=mode` and restores the script's mode. Its `passthru.parts` is an attribute set of the parts.
- **The version** is the workspace's, `(builtins.fromTOML (builtins.readFile ../../Cargo.toml)).workspace.package.version`, as `boomdrive.nix` reads the runner's.

The cases, in `way-in.nix`:

- "The app on macOS": the `aarch64-darwin` package's `passthru.parts`' plist `text` holds `<key>CFBundleName</key>` `<string>Boomdrive</string>` and `<key>CFBundleExecutable</key>` `<string>boomdrive-launcher</string>`. Its script's `destination` is under `Contents/MacOS/`, and its `text` is `launcher.sh`'s.
- "The desktop entry on Linux": the `x86_64-linux` package's desktop item's `text` holds `Name=Boomdrive`, `Terminal=false` and an `Exec=` naming the script's path.

`typecheck.nix`'s set gains `"boomdrive-launcher"`.

**2.3** `nix/shell-hook.sh` is Decision 4's hook, sourced by bash, so it uses no `exit` and sets nothing: `case $- in *i*) [ -n "${HERDR_ENV:-}" ] || herdr ;; esac`, with a header comment. `nix/shell.nix` is `{ pkgs, set }: pkgs.mkShellNoCC { packages = [ set.boomdrive set.flume set.sluice set.crew set.herdr ]; shellHook = builtins.readFile ./shell-hook.sh; }`. `flake.nix` gives `devShells = lib.genAttrs systems (system: { default = import ./nix/shell.nix { … }; })`. It uses the same `pkgs` as `packagesOf`, through a shared `pkgsOf system`, and comments that the repository's development shell is devenv's.

The cases:

- **In `way-in.sh`**, each sourcing the hook with a scratch `HOME` holding an empty `.bashrc`, and a stub `herdr` that records its arguments:
  - "Entering the shell": `bash -i -c '. <hook>; echo after'` runs herdr once with no argument and prints `after`.
  - "A command through the shell": `bash -c '. <hook>'` runs no herdr.
  - "Inside herdr already": `HERDR_ENV=1 bash -i -c '. <hook>'` runs no herdr.
  - The cases ignore bash's words about job control on standard error.
- **In `way-in.nix`**, "Every system's shell": each system's `devShells.default` is a derivation, its `nativeBuildInputs` or `packages` hold the five packages' `outPath`s, and its `shellHook` is the hook's text.
- **In `typecheck.nix`**, each system's `devShells.default.drvPath` evaluates.

**3.1** `nix/modules/way-in.nix` (Decision 5), imported by `darwin.nix` and `nixos.nix` after `tools.nix`. It reads the set through `set.nix`. Its header comment cites record 0023 and this design.

The cases, in `way-in.nix`'s `onEach`, as in `tools.nix`:

- "The way in with the runner": the profile of `t.darwinSystem { }` holds the set's three by `outPath`.
- "A host's own herdr": with `herdr.package` a `pkgs.runCommand "own-herdr" { } "mkdir -p $out/bin"`, the profile holds it and not the set's herdr. The plugin step's line names `<own>/bin/herdr`.
- "No Ghostty of the set's": with `ghostty.package = null`, the profile holds no set's Ghostty, and Flume's `host.terminal_app` is `ghostty`.
- "Not without the runner": `t.darwinOf [ outputs.darwinModules.default ]` and `t.nixosOf [ outputs.nixosModules.default ]` hold none of the three.

**3.2** In `way-in.nix` (Decision 7). The cases, in `nix/tests/way-in.nix`:

- "Flume's terminal and window", on each platform: `config.services.flume.settings.host.multiplexer` is `herdr` and `terminal_app` is `ghostty`.
- "A host's own window" and "The word of before": the second also holds that `multiplexer` is `null`, the value Flume's module gives a key it leaves out of the file.

**3.3** `activate.sh` gains `plugins`. Its header and usage name it, and it does what Decision 6 says.

- It unsets each `HERDR_*` name `env` lists, in a subshell, before it runs herdr.
- It exits 0, with one line on standard error for each refusal.

In `darwin.nix`, `postActivation` gains, after the runner's link, the line `sudo --user=<user> -- /usr/bin/env HOME=<home> /bin/sh ${./activate.sh} plugins <herdr> <plugins…> || true`, under `lib.optionalString` of a non-null package and a non-empty list. In `nixos.nix`, the runner's user snippet gains `${pkgs.runtimeShell} ${./activate.sh} plugins …  || true` inside its guard, likewise.

`nix/packages/herdr-plugin.nix`'s header says the modules link the plugin with herdr's own command at activation (Decision 6).

The cases:

- **In `nix/tests/activate.sh`**, with a stub `herdr` recording its arguments and its `HERDR_*` environment:
  - "Each plugin linked": two plugins, the caller's `HERDR_SOCKET_PATH` and `HERDR_ENV` set. Two calls `plugin link <p1>` and `plugin link <p2>` in order, no `HERDR_*` seen, and status 0.
  - "A link herdr refuses": the stub exits 1 saying `manifest not found` for the first. One line on standard error naming the first plugin and those words, the second still called, and status 0.
- **In `nix/tests/way-in.nix`**:
  - "The plugins linked on nix-darwin": the activation's text holds one line, run under `sudo --user=alice`, naming `activate.sh`'s `plugins`, the set's herdr's `bin/herdr` and both plugins' paths in order.
  - "The plugins linked on NixOS": `system.userActivationScripts.boomdrive`'s text holds the same step after its `id -un` test of `alice`.
  - "No herdr of the set's": with `herdr.package = null`, and again with `plugins = [ ]`, no activation text names `plugins`.
- **In `tools.nix`**, "The tools' activation is their own on nix-darwin" leaves out the line naming `activate.sh`'s `plugins` with the runner's link.

**3.4** `cache.nix`'s `overlay` gains `herdr`, `ghostty` and `boomdrive-launcher`, and `ownNixpkgs` holds the three to the set's `outPath`s.

**4.1** The README's **Installing with nix**:

- **The set.** It holds herdr, from its own flake at the herdr Flume pins, Ghostty, from nixpkgs at the release Flume's window was checked against, and the Boomdrive launcher. git alone is the host's own, and the profile line may add `#herdr` and `#ghostty`.
- **The modules.** With `services.boomdrive.enable` they install herdr, Ghostty and the launcher, `Boomdrive` in `/Applications/Nix Apps` or the desktop's applications, which runs `flume open`.
  - They link `services.boomdrive.herdr.plugins`, by default the set's Flume and sluice plugins, with herdr's own `plugin link` at each switch.
  - A relink enables a disabled plugin, so a host drops it from the list. A dropped plugin stays linked until `herdr plugin unlink`.
  - They set Flume's `host.multiplexer` to `herdr` and `host.terminal_app` to `ghostty` unless the host names its own.
- **Replacing or dropping.** `services.boomdrive.herdr.package`, `services.boomdrive.ghostty.package` and `services.boomdrive.launcher.package`, each a package of the host's own or `null`. A host with no display sets Ghostty and the launcher `null` and Flume's `host.terminal_app` its own.
- **The shell.** `nix develop github:agentplot/boomdrive` gives a shell with the set on `PATH` that starts herdr, and leaves the person in it when herdr ends.

`cache.nix`'s "The README's words" expects the delta's names.

**4.2** CLAUDE.md's paragraph on the flake: "gives the runner, Flume, sluice and crew pinned together, with herdr, its plugins and Ghostty, each tool's own module re-exported beside the runner's service, `services.boomdrive`, … and a shell, `devShells.<system>.default`, that starts herdr". `openspec/config.yaml`'s stack line names the same.
