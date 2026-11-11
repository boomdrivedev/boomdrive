# Design

## Context

The brief: "A person opens any drive's agents from the Boomdrive launcher, one entry per drive of the host, each showing that drive's stream with flume open: a menu-bar menu on macOS and one action per drive in the desktop entry on Linux." Its sources are records 0023 and 0028, both accepted, and Flume's statement 186.

The launcher today, from the change `herdr-and-ghostty-come-with-the-flake-and-an-app-opens-the-agents`:

- **The script.** `nix/packages/launcher.sh` puts the user's profiles ahead of its `PATH` and runs `flume open` once, with no session. Where Flume refuses, it shows Flume's words: in an alert titled `Boomdrive` on macOS, by `osascript`, and on standard error elsewhere. `nix/tests/way-in.sh` holds its cases, with stubs.
- **The package.** `nix/packages/launcher.nix`, called by `flake.nix` with `pkgs.callPackage`, so it has `override`.
  - On `aarch64-darwin` it is `Applications/Boomdrive.app`: an `Info.plist` with `LSUIElement` true, and the script as its executable, `Contents/MacOS/boomdrive-launcher`.
  - On Linux it is the script at `bin/boomdrive-launcher` and `share/applications/boomdrive.desktop`, made by `makeDesktopItem`, which runs it.
  - Its parts are given as `passthru.parts`, which `nix/tests/way-in.nix` reads.
- **The module.** `nix/modules/apps.nix` declares `services.boomdrive.launcher.package`, the set's `boomdrive-launcher` by default, and installs it with the runner.

What the module knows of the drives:

- `nix/modules/accounts.nix`'s `drivesOf` lists each drive as `{ account, name, machine, stream, addresses }`.
- `nix/modules/drives.nix` keeps the drives of this host: those whose `machine` is unset or is `services.crew.host`. It gives each Flume's `settings.streams.<stream.name>`. A drive of another machine gives Flume no stream (record 0028's amendment of 2026-11-08).
- A drive's name is 1 to 32 lower-case letters, digits and `-`, starting with a letter, and is given once on the host across its accounts (record 0028's amendment of 2026-11-07).

What Flume gives, at the set's tag `v0.21.2`:

- **`flume open [--] [<session>]`** shows a session by its name, never a stream's (186). A stream's own session is the one its `session` names, else the stream's name (107). Asked for no name, Flume shows the default stream's own. A name no stream gives is refused, naming each stream's sessions, before anything starts. Seen on a host with Flume 0.20.1: `flume open --json -- <name>` refuses a name no stream gives, naming each stream's sessions.
- **Flume's module** declares `streams.<stream>.session`, null where unset, "the stream's name where unset".

What nixpkgs gives, at the flake's lock:

- `makeDesktopItem` takes `actions`, an attribute set of an id to `{ name, exec }`. It writes `Actions=` from the set's names, in name order, and each value as given, escaping nothing.
- `lib.generators.toPlist { escape = true; }` renders a plist from a value, escaping its strings for XML.
- The darwin `stdenv` compiles Objective-C against the default Apple SDK, with no other input. On `aarch64-darwin` its linker signs each binary ad hoc.

The gate evaluates the flake and never builds it (record 0020's Decision 14).

## Goals / Non-Goals

**Goals:**

- Each drive of this host has one entry in the launcher, and picking it shows that drive's stream's own session through `flume open`.
- A menu-bar menu on macOS and one desktop action per drive on Linux.
- The set's own launcher stays one path for each system, of no host's settings, which the cache holds (core.47).
- The runner's code does not change.

**Non-Goals:**

- An entry for a session a stream names for a kind or a stage (Flume's 199). A person opens one with `flume open <session>`.
- An icon, a signed bundle, or a tray icon on Linux.
- Starting the menu at login. A person who wants it there adds `Boomdrive` to their own login items.
- A menu that changes while it runs. After a switch that changes the drives, the person quits Boomdrive and opens it again.
- Showing whether a session runs, or how many agents it holds.

## Decisions

### 1. Each drive of this host is one entry, opening its stream's own session

The entries are the drives of this host, as `drives.nix` keeps them: each drive whose `machine` is unset or is `services.crew.host`. Each entry is `{ drive, session }`:

- **`drive`** is the drive's name. It is the entry's title.
- **`session`** is the stream's own session, as Flume names it (107, 186). It is `services.flume.settings.streams.<stream.name>.session` where that is set, and the drive's `stream.name` otherwise.

The entries are in the order of the drives' names, on both platforms. Drive names are unique on the host, and `makeDesktopItem` writes `Actions` in name order, so one order holds on both.

A drive of another machine has no entry, since it has no stream on this host (record 0028).

The launcher keeps one entry beyond the drives, `Default stream`. It runs `flume open` with no name, as the launcher does today, so Flume shows its default stream's own session. On Linux it is the desktop entry's own click.

Alternatives:

- **Pass each drive's stream name to `flume open`.** Not taken: Flume opens a session, never a stream (186). A stream whose `session` is set would be refused.
- **Ask Flume at each click for the stream's own session, from its listing.** Not taken: it is a second Flume call and a JSON reader in the script for every click. The module reads the same `settings` that Flume's managed file is written from.
- **An entry for each stream Flume declares.** Not taken: the brief asks for the drives, and a stream of the host's own, outside every drive, would appear as a drive.
- **An entry for a drive of another machine.** Not taken: Flume would refuse it, since this host gives that drive no stream.
- **Drives only, with no default entry.** Not taken: the desktop entry's own click runs something, and a host with no drive here would have an empty menu. The default stream is what the launcher opened before.
- **The entries in the accounts' order, then each account's drives'.** Not taken: `makeDesktopItem` orders actions by name, so the two platforms would disagree.

### 2. The script opens the session it is given

`launcher.sh` takes at most one argument:

- **Given a session**, it runs `flume open -- <session>`. The `--` keeps a session that starts with `-` from being read as a flag.
- **Given none**, it runs `flume open`, as today.
- **Given more than one**, it runs nothing, says `boomdrive-launcher: one session at most` on standard error, and exits 2.

Everything else stays as it is: the profiles ahead of `PATH`, Flume's words shown in an alert on macOS and on standard error elsewhere, and exit 1 where Flume refuses.

The menu and the desktop entry each run the script and nothing else. So the `PATH`, the alert and the refusal stay in one file, which `way-in.sh` tests with stubs.

Alternatives:

- **The menu running `flume` itself.** Not taken: the menu would repeat the script's `PATH` and its alert in Objective-C, which no test of the gate runs.

### 3. On macOS the launcher is a menu-bar app

`Boomdrive.app` holds:

- `Contents/MacOS/boomdrive-menu`, the app's executable, compiled from `nix/packages/launcher.m`;
- `Contents/MacOS/boomdrive-launcher`, the script, as today;
- `Contents/Info.plist`.

The `Info.plist` is `lib.generators.toPlist { escape = true; }` of the package's `passthru.info`:

| key | value |
|---|---|
| `CFBundleName`, `CFBundleDisplayName` | `Boomdrive` |
| `CFBundleExecutable` | `boomdrive-menu` |
| `CFBundleIdentifier` | `io.github.agentplot.boomdrive` |
| `CFBundlePackageType` | `APPL` |
| `LSUIElement` | true |
| `BoomdriveDrives` | an array of dictionaries, each `drive` and `session`, in the order the package is given |

What the menu does:

- **At start**, it sets the app's activation policy to an accessory's, so it shows in the menu bar and not in the Dock, as `LSUIElement` also says.
- **The status item** is titled `Boomdrive`. Its menu holds:
  - one item for each dictionary of `BoomdriveDrives`, titled its `drive`;
  - `Default stream`;
  - a separator;
  - `Quit Boomdrive`, with the key equivalent `q`, which ends the app.
- **A dictionary with no string `drive` or `session`** is left out of the menu.
- **Picking a drive's item** runs the script, `boomdrive-launcher` in the bundle's `Contents/MacOS`, with the drive's session as its one argument. **Picking `Default stream`** runs it with none.
  - The script is started with `NSTask` and not waited on, so the menu answers at once and each picking opens a window of its own, as Flume's 186 has it.
  - The script shows Flume's refusal itself (Decision 2).
  - Where the script cannot be started, the menu shows an alert titled `Boomdrive` with the error's own words.
- **Opened again** while it runs, macOS brings the running app forward, and nothing else happens.

The app runs from the person's opening it until they quit it or log out. No module or service starts it (core.13, core.46).

`launcher.m` is one file of Objective-C with ARC, using AppKit alone. The package compiles it with the darwin `stdenv`'s compiler: `$CC -fobjc-arc -Wall -Wextra -Werror -framework AppKit -o boomdrive-menu launcher.m`. The compiled menu is a derivation of its own, `passthru.parts.menu`, which takes no drive. So the launcher made for any host's drives holds the set's one menu path.

Alternatives:

- **Swift.** Not taken: the nixpkgs Swift toolchain is far heavier than the stdenv's compiler, and swift-format would join the gate for one file.
- **JavaScript for Automation**, run by `osascript -l JavaScript` from the bundle's executable. Not taken: the process would be `osascript`, not the bundle's own executable. So the app's identity, its `LSUIElement` and its quitting at logout rest on the scripting bridge's behaviour, which nothing here could check.
- **A Rust crate**, through the `objc2` crates. Not taken: it is a crate and a moon project of its own that builds on macOS alone, for one menu.
- **A third-party menu-bar app**, such as SwiftBar, given a plugin. Not taken: it brings another app into the set, and its plugin folder is that app's own setting (core.18).
- **A dialog that asks for the drive** at each click, with `osascript`'s `choose from list`. Not taken: the brief asks for a menu-bar menu.

### 4. On Linux the desktop entry has one action for each drive

`boomdrive.desktop` stays `makeDesktopItem` with `name = "boomdrive"`, `desktopName = "Boomdrive"` and `terminal = false`. Its `exec` is the script with no argument, which opens the default stream.

It gains `actions`, one for each entry: its id and its `name` are the drive's name, and its `exec` is the script with the entry's session as one argument. A drive's name is a valid action id.

The session is quoted as the Desktop Entry Specification's `Exec` key asks, since `makeDesktopItem` writes each value as given:

1. Each `\`, `"`, `` ` `` and `$` is preceded by `\`, and the session is put in double quotes.
2. Each `%` is written `%%`, so no field code is read.
3. Each `\` of the result is written `\\`, the string type's escape, which a reader undoes first.

So the session `my $work` is the argument `"my \\$work"`.

Alternatives:

- **The entry's own click opening the first drive.** Not taken: which drive is first is the drives' names' order, not a choice of the person's, and the default stream is what the click opens today.
- **A tray icon.** Not taken: the brief asks for desktop actions. A tray needs a running process and a desktop that shows trays.

### 5. The drives reach the launcher through its override

`launcher.nix` takes `drives ? [ ]`, a list of `{ drive, session }` (Decision 1). The flake's package `boomdrive-launcher` is it with none.

The module's `services.boomdrive.launcher.package` defaults to `set.boomdrive-launcher.override { drives = <this host's entries>; }`:

- The override re-calls `launcher.nix` with the flake's own nixpkgs, which `flake.nix`'s `callPackage` closed over. So nothing of it is made with the host's `pkgs`.
- What depends on the drives is text and copies: the `Info.plist`, the desktop entry and the bundle's or package's tree. The compiled menu and the script are the set's own paths, which the cache holds.
- With no drive of this host, the entries are `[ ]`, the argument's own default, so the override gives the set's package's path, and the system profile holds it, as today.
- A host's own package is installed as given, since the default is all the module makes.

The option's `defaultText` names the set's launcher made for this host's drives, and its description says each drive of this host is an entry.

The entries are computed in `apps.nix` from one list of the drives of this host. `accounts.nix` gains `hereOf host drives`, the filter `drives.nix` holds today, so both files read one list. `apps.nix` reads `services.crew.host` as `drives.nix` does, and reads `config.services.flume.settings.streams` for each stream's `session`, a null `streams` holding none.

Alternatives:

- **A file the module writes, which the launcher reads at each start.** Not taken: a desktop entry's actions are lines of the desktop file, not read from another. So Linux would still need a desktop entry of the host's own, and the two platforms would take their drives by two routes.
- **nix-darwin's `system.defaults.CustomUserPreferences`**, read through `NSUserDefaults`. Not taken: it is a third route, macOS alone, and the preferences daemon may give a running app the old value.
- **The module making the bundle with the host's `pkgs`.** Not taken: core.47 rules out a module's package made with the host's `pkgs`.

### 6. clang-format formats the menu's source

`launcher.m` is the tree's first Objective-C file. core.7 has every tracked file owned by a formatter the gate runs, or excluded by name with its reason, and core.8 has each tool at the version the tree pins.

- `treefmt.nix` turns on `programs.clang-format`, with `includes = [ "nix/packages/*.m" ]`. Its package is devenv's nixpkgs' `clang-tools`, the treefmt module's default.
- `devenv.nix` pins `clangFormatVersion` to that `clang-tools`' version, asserts it as it asserts Biome's, and puts the package in the shell.
- `check-toolchain.sh` holds `clang-format --version` to the pin, as `release biome …` does.

The file is laid out in clang-format's default style for Objective-C, with no `.clang-format` file.

Alternatives:

- **The file excluded by name.** Not taken: core.7 allows it, but code written by hand would then have no layout the gate holds.

### 7. The tests

Every evaluated case is under `nix/tests`, run by `nix:test` against no store, with core.41's neutral names. The sh cases run stubs in scratch folders.

- **`way-in.sh`** gains "A drive's session opened" and "More than one session", beside "Opened", which stays.
- **`nix/tests/way-in.nix`**:
  - "The app on macOS" reads `passthru.info` and the `Info.plist`'s text. The executable is `boomdrive-menu`, the menu part is compiled from `launcher.m`, and the script is in `Contents/MacOS`.
  - "Each drive in the menu on macOS" and "Each drive an action on Linux" call `outputs.packages.<system>.boomdrive-launcher.override` with two drives.
  - "A session quoted in its action" uses the session `my $work`.
  - "The desktop entry on Linux" keeps its lines, and the set's package has no `Actions`.
- **`nix/tests/apps.nix`** holds the service-modules delta's cases, each on nix-darwin and on NixOS. Their accounts are those of `drives.nix`'s cases: `own` and `work`, the drives `home` and `garden`, and `services.crew.host` `host-a` where a machine is named.
- **`nix/tests/cache.nix`'s** "A host's own nixpkgs" holds as it is, with no drive. It gains "A host's drives in the launcher".

The gate never builds, so the menu's compile is a task's own check, `nix build` of the `aarch64-darwin` launcher on a Mac. "The menu's source" holds `launcher.m` to read `BoomdriveDrives`, title its item `Boomdrive`, name `Default stream` and `Quit Boomdrive`, and run `boomdrive-launcher`. What a click does is seen by a person on a Mac, since no task of the gate runs a menu bar.

### Rules relied on

- **core.13, core.29.** The runner's code does not change. The menu runs at the person's opening of it, no module or service starts it, and it runs Flume's command alone (Decision 3).
- **core.18, core.46.** The module writes no file of Flume's. It reads Flume's `settings` for a stream's `session`, and gives the launcher what it read (Decisions 1 and 5).
- **core.44, core.47.** The launcher is of this tree. Its compiled menu is made by the flake's own nixpkgs, and the launcher made for a host's drives is the set's by its override (Decision 5).
- **core.7, core.8.** The `.m` file is formatted by clang-format, pinned and checked (Decision 6).
- **core.12.** The package's argument and the plist's key name what they hold, `drives` and `BoomdriveDrives`.
- **core.4.** No option or file holds a credential.
- **core.41.** Neutral names in every case.

### Rules amended

In the commit that writes this change:

- **core.13's details** do not rule out the launcher's menu on macOS, which a person opens and quits, and which no module or service starts. A multiplexer's server is started by `flume open` when a person picks an entry of the launcher.
- **core.44's details** say the launcher's menu on macOS is compiled from this tree by the flake's own nixpkgs.
- **core.46's details** do not rule out the launcher made for this host's drives, each drive of this host an entry opening its stream's own session.
- **core.47's details** do not rule out the launcher a module installs made for a host's drives by the set's own launcher's override. It is made with the flake's own nixpkgs, its compiled menu is the set's path the cache holds, and what is the host's is text and copies.

Record 0023 gains the amendment "each drive opens from the launcher", which carries Decisions 1 to 6.

### Rules departed from

None.

## Risks / Trade-offs

- **The menu is unchecked by the gate.** A compile error in `launcher.m` shows at the task's `nix build` on a Mac, and on `main` at the cache workflow's `aarch64-darwin` job. What a click does is seen by a person on a Mac.
- **A running menu keeps its drives.** The menu reads its drives at start. After a switch that changes them, it shows the old drives until the person quits it and opens the new one. A drive left out by then is refused by Flume, naming each stream's sessions.
- **A session set in Flume's local layer.** The module reads Flume's `settings`, the managed layer. A stream's `session` set only in Flume's local layer is unknown to it, and that drive's entry is refused by Flume, naming the session to open.
- **No login item.** The menu is in the menu bar only while the person keeps it open.
- **The bundle is unsigned**, as before. Its menu is signed ad hoc by the linker, and nothing quarantines a store path.

## Migration Plan

A host takes the change by moving its Boomdrive input. With `services.boomdrive.enable`:

- **macOS.** `Boomdrive` in `/Applications/Nix Apps` becomes a menu-bar app. A click puts `Boomdrive` in the menu bar, and its menu opens each drive of the host, or the default stream.
- **Linux.** `Boomdrive` keeps its click, the default stream, and gains an action for each drive.
- **A host's own launcher**, given as `services.boomdrive.launcher.package`, is installed as given and gets no entries.

## Open Questions

None.

## Task notes

**1.1** In `launcher.sh`:

- `case $# in 0) set -- open ;; 1) set -- open -- "$1" ;; *) echo 'boomdrive-launcher: one session at most' >&2; exit 2 ;; esac`, before `PATH` is set.
- `words=$(flume "$@" 2>&1)`.
- The header comment says it opens the session it is given, or Flume's default with none, and that the menu and the desktop entry run it.

In `way-in.sh`, the `run` of a case passes its arguments to the launcher. The cases:

- "A drive's session opened": run with `home`. The stub's one call is `flume open -- home`, and neither `osascript` nor standard error saw anything.
- "More than one session": run with `home garden`. The status is 2, standard error holds `one session at most`, and `flume` is not called.

**2.1** In `devenv.nix`, beside `biomeVersion`, pin `clangFormatVersion` to devenv's nixpkgs' `pkgs.clang-tools.version`, read in the shell with `nix eval`. Add `(assert pkgs.clang-tools.version == clangFormatVersion; pkgs.clang-tools)` to `packages`, with a comment naming treefmt's clang-format and `nix/packages/launcher.m`. In `check-toolchain.sh`, add `release clang-format "$(pinned clangFormatVersion)" clang-format --version` after Biome's, and name clang-format in its header. In `treefmt.nix`, add after Biome's block:

```nix
# clang-format lays out the launcher's menu, the tree's one Objective-C
# file, in its default style (design Decision 6 of the change
# each-drive-opens-from-the-launcher).
programs.clang-format.enable = true;
programs.clang-format.includes = [ "nix/packages/*.m" ];
```

**2.2** `nix/packages/launcher.m` does what Decision 3 says, in one file:

- `@interface BoomdriveMenu : NSObject <NSApplicationDelegate>`, holding the status item.
- `applicationDidFinishLaunching:` reads `[[NSBundle mainBundle] objectForInfoDictionaryKey:@"BoomdriveDrives"]`, builds the menu, and makes the status item with `NSVariableStatusItemLength`, its button's `title` `Boomdrive`.
- Each drive's item has the action `open:`, the target the delegate and the `representedObject` the session. `Default stream`'s `representedObject` is nil. `Quit Boomdrive` has `terminate:` and the key `q`.
- `open:` takes the script's URL from `[[NSBundle mainBundle] URLForAuxiliaryExecutable:@"boomdrive-launcher"]`. It runs `[NSTask launchedTaskWithExecutableURL:arguments:error:terminationHandler:]`, with `@[ session ]` or `@[]` and a nil handler. On an error, it runs an `NSAlert` with `messageText` `Boomdrive` and `informativeText` the error's `localizedDescription`.
- `main` makes the shared application, sets `NSApplicationActivationPolicyAccessory`, sets the delegate and runs it.
- A header comment cites record 0023 as amended on 2026-11-11 and this design's Decision 3.

**2.3** `launcher.nix` takes `{ lib, runCommand, writeTextFile, makeDesktopItem, stdenv, drives ? [ ] }`, with `stdenv` in place of `stdenvNoCC`:

- **The menu** on darwin, `stdenv.mkDerivation { pname = "boomdrive-menu"; inherit version; src = ./launcher.m; dontUnpack = true; buildPhase = "$CC -fobjc-arc -Wall -Wextra -Werror -framework AppKit -o boomdrive-menu $src"; installPhase = "install -Dm755 boomdrive-menu $out${app}/MacOS/boomdrive-menu"; }`, with `meta.platforms` darwin.
- **`info`**, Decision 3's table, with `BoomdriveDrives = drives`. The plist part is a `writeTextFile` whose `text` is `lib.generators.toPlist { escape = true; } info`, at `${app}/Info.plist`.
- `parts` is `{ script, plist, menu }` on darwin and `{ script, desktop }` on Linux. `passthru` gains `info` on darwin and `drives` on both.
- The `runCommand` stays: it copies each part's tree into `$out`, and restores the script's and the menu's modes.

`nix/tests/way-in.nix`:

- "The app on macOS" holds `info`'s `CFBundleName`, `CFBundleExecutable` `boomdrive-menu` and `LSUIElement` true. It holds the plist's `text` to be `lib.generators.toPlist { escape = true; } info`, the menu's `src` to be a file named `launcher.m`, by `baseNameOf (toString parts.menu.src)`, and the script's `destination` and `text` as today.
- "Each drive in the menu on macOS": `override { drives = [ { drive = "garden"; session = "garden"; } { drive = "home"; session = "house"; } ]; }`. Its `info.BoomdriveDrives` is that list, and its `parts.menu.drvPath` is the set's package's.
- "No drive": the set's package's `info.BoomdriveDrives` is `[ ]`, and its desktop entry's text has no `Actions=` line.
- "The menu's source": `builtins.readFile ../packages/launcher.m` holds `@"BoomdriveDrives"`, `@"Boomdrive"`, `@"Default stream"`, `@"Quit Boomdrive"` and `@"boomdrive-launcher"`.

**2.4** True when `nix build --no-link .#packages.aarch64-darwin.boomdrive-launcher` succeeds on an `aarch64-darwin` host, outside the gate's sandbox, from the place's root.

**2.5** In `launcher.nix`, a `quoted` function does Decision 4's three steps with `builtins.replaceStrings`. The desktop item gains `actions = lib.listToAttrs (map (entry: { name = entry.drive; value = { name = entry.drive; exec = "${exec} ${quoted entry.session}"; }; }) drives)`, where `exec` is the script's path the entry's own `exec` names.

`nix/tests/way-in.nix`:

- "Each drive an action on Linux": the `x86_64-linux` package overridden with the two drives of 2.3. Its desktop entry's lines hold `Actions=garden;home`, `[Desktop Action garden]` then `Name=garden` and `Exec=<script> "garden"`, and `[Desktop Action home]` then `Name=home` and `Exec=<script> "house"`.
- "A session quoted in its action": one drive `home` with the session `my $work`. Its line is `Exec=<script> "my \\$work"`.

**3.1** In `accounts.nix`, `hereOf = host: lib.filter (drive: drive.machine == null || drive.machine == host);`, with a comment. `drives.nix` uses it for `here`. True when the drive cases of `nix/tests/drives.nix` still hold.

**3.2** In `apps.nix`, add `options` to the module's arguments. Then:

- `host`, as `drives.nix` reads it.
- `streams = config.services.flume.settings.streams or null`.
- `sessionOf = drive: let named = if streams == null then null else streams.${drive.stream.name}.session or null; in if named == null then drive.stream.name else named;`.
- `entries = lib.sort (a: b: a.drive < b.drive) (map (drive: { drive = drive.name; session = sessionOf drive; }) (hereOf host (drivesOf cfg.accounts)));`.

`launcher.package` becomes its own `mkOption`. Its type is `nullOr package` and its default `set.boomdrive-launcher.override { drives = entries; }`. Its `defaultText` is `boomdrive.packages.''${pkgs.stdenv.hostPlatform.system}.boomdrive-launcher.override { drives = <each drive of this host>; }`. Its description says each drive of this host is an entry opening its stream's own session, a host's own package replaces it, and `null` installs none. The header comment cites this design's Decisions 1 and 5.

`nix/tests/apps.nix` gains, on each platform, with `installed = evaluated.config.services.boomdrive.launcher.package`:

- "Each drive of this host in the launcher": the accounts `own` with the drive `home`, and `work` with the drive `garden`, each keeping one repository. `installed.drives` is `garden` then `home`, each its own session, and the profile holds `installed`.
- "A drive of another machine has no entry": `services.crew.host = "host-a"`, and `garden` sets `machine = "host-b"`. `installed.drives` holds `home` alone.
- "A stream's own session named": `services.flume.settings.streams.home.session = "house"`. `home`'s session is `house`.
- "No drive, the set's launcher": with no accounts, `installed.outPath` is the set's `boomdrive-launcher`'s.
- "A host's own launcher": a `pkgs.runCommand "own-launcher" { } "mkdir $out"`, with the drives of the first case, is installed as given.

**3.3** `cache.nix` gains "A host's drives in the launcher", on each platform. It uses `ownNixpkgs`'s system with the drive `home`. The installed launcher's `drvPath` is the set's `boomdrive-launcher.override { drives = [ { drive = "home"; session = "home"; } ]; }`'s, and its `parts.menu.outPath` on nix-darwin is the set's package's.

**4.1** The README's **Installing with nix**:

- **The set's launcher.** On macOS, a menu-bar app, `Boomdrive`. On Linux, a desktop entry.
- **What the modules install.** The launcher made for each drive of the host: its menu, or its desktop entry's actions, holds an entry for each drive, which opens that drive's stream's own session, and `Default stream`, which opens Flume's default.
  - A drive of another machine has no entry.
  - The menu stays in the menu bar until it is quit. After a switch that changes the drives, quit it and open it again.
  - A host's own package gets no entries.

`cache.nix` gains "The README's launcher": the README's install with nix names `menu bar`, `each drive` and `Default stream`.

**4.2** CLAUDE.md's paragraph on the flake reads "the Boomdrive launcher, a menu-bar app on macOS and a desktop entry on Linux, with an entry for each drive, that runs `flume open`". `openspec/config.yaml`'s stack line says the same.
