# way-in Specification

## Purpose
The person's ways to see the host's agents that the flake gives: the Boomdrive launcher, an app on macOS and a desktop entry on Linux that runs `flume open`, and the flake's shell, whose `PATH` is the set's tools and which starts herdr on entering. Each runs at the person's asking, and the runner starts and names no multiplexer (record 0023; core.13, core.29).

## Requirements

### Requirement: The launcher opens the host's agents at a person's asking

The `boomdrive-launcher` package's script SHALL, each time it is run, run Flume once: `flume open -- <session>` where it is given one argument, the session, and `flume open` where it is given none, so Flume shows that session, or the default stream's own, in its configured view (Flume's statement 186). It SHALL put the running user's profiles and the system's before the `PATH` it is given, so it runs the host's one Flume. It SHALL start, keep and name no multiplexer itself (core.13, core.29).

#### Scenario: Opened

- **WHEN** the launcher runs with no argument and Flume opens a window, exiting 0
- **THEN** Flume is run once, as `flume open`, and nothing is shown

#### Scenario: A drive's session opened

- **WHEN** the launcher runs with the argument `home` and Flume opens a window, exiting 0
- **THEN** Flume is run once, as `flume open -- home`, and nothing is shown

#### Scenario: The profiles first

- **WHEN** the launcher runs as `alice`, whose home is `/home/alice`, with the `PATH` `/usr/bin:/bin`
- **THEN** Flume runs with a `PATH` that names `/etc/profiles/per-user/alice/bin` and `/home/alice/.nix-profile/bin` before `/usr/bin`

### Requirement: The launcher shows Flume's refusal

Where Flume exits other than 0, the launcher SHALL show Flume's words as Flume gave them: in an alert titled `Boomdrive` on macOS, and on its standard error elsewhere.

#### Scenario: Flume refuses on macOS

- **WHEN** the launcher runs on macOS and Flume exits 1 saying `no window program could be run`
- **THEN** an alert titled `Boomdrive` shows `no window program could be run`

#### Scenario: Flume refuses elsewhere

- **WHEN** the launcher runs on Linux and Flume exits 1 saying `no window program could be run`
- **THEN** the launcher's standard error holds `no window program could be run`, and no alert is asked for

### Requirement: The launcher is an app on macOS and a desktop entry on Linux

On `aarch64-darwin`, the `boomdrive-launcher` package SHALL hold `Applications/Boomdrive.app`, a menu-bar app nix-darwin places in `/Applications/Nix Apps`, holding the script `Contents/MacOS/boomdrive-launcher` and an `Info.plist` naming the bundle `Boomdrive`, its executable `boomdrive-menu`, and `LSUIElement` true. On `aarch64-linux` and `x86_64-linux`, it SHALL hold `share/applications/boomdrive.desktop`, named `Boomdrive`, which runs the script with no argument, in no terminal.

#### Scenario: The app on macOS

- **WHEN** the `boomdrive-launcher` package of `aarch64-darwin` is evaluated
- **THEN** its `Info.plist` gives `CFBundleName` `Boomdrive`, `CFBundleExecutable` `boomdrive-menu` and `LSUIElement` true, its menu is compiled from `nix/packages/launcher.m` into `Contents/MacOS/boomdrive-menu`, and its script is `launcher.sh` at `Contents/MacOS/boomdrive-launcher`

#### Scenario: The desktop entry on Linux

- **WHEN** the `boomdrive-launcher` package of `x86_64-linux` is evaluated
- **THEN** it holds `share/applications/boomdrive.desktop`, whose `Name` is `Boomdrive`, whose `Exec` runs the script with no argument, and whose `Terminal` is false

### Requirement: The flake gives a shell that starts herdr

The flake SHALL give `devShells.<system>.default` for each of its three systems. Its `PATH` SHALL hold the set's `boomdrive`, `flume`, `sluice`, `crew`, `herdr`, `portless` and `worktrunk`. Entering it interactively SHALL start herdr once, as herdr's own start with no argument, and leave the person in the shell when herdr ends. A command run through the shell, or a shell entered from inside herdr, SHALL start no herdr. The shell SHALL link no plugin and write no file.

#### Scenario: Every system's shell

- **WHEN** the flake's `devShells` are evaluated for each of the three systems
- **THEN** each gives `default`, a derivation whose packages are the set's `boomdrive`, `flume`, `sluice`, `crew`, `herdr`, `portless` and `worktrunk`

#### Scenario: Entering the shell

- **WHEN** the shell's hook runs in an interactive shell, outside herdr
- **THEN** herdr is run once, with no argument, and the shell goes on once herdr exits

#### Scenario: A command through the shell

- **WHEN** the shell's hook runs in a shell that is not interactive, as `nix develop --command` runs one
- **THEN** no herdr is run

#### Scenario: Inside herdr already

- **WHEN** the shell's hook runs in an interactive shell whose `HERDR_ENV` is `1`
- **THEN** no herdr is run

### Requirement: The launcher opens one session at most

Given more than one argument, the `boomdrive-launcher` package's script SHALL run nothing, say `boomdrive-launcher: one session at most` on its standard error and exit 2.

#### Scenario: More than one session

- **WHEN** the launcher runs with the arguments `home` and `garden`
- **THEN** it exits 2, its standard error holds `one session at most`, and Flume is not run

### Requirement: The menu runs the script for the entry a person picks

The menu, the app's executable `Contents/MacOS/boomdrive-menu`, compiled from `nix/packages/launcher.m`, SHALL title its status item `Boomdrive` and hold an entry for each drive of its `Info.plist`'s `BoomdriveDrives`, titled the drive's name, then `Default stream`, then `Quit Boomdrive`. Picking a drive's entry SHALL run the bundle's script with the drive's session, and `Default stream` SHALL run it with none, each without waiting on it. No module or service SHALL start the app.

#### Scenario: The menu's source

- **WHEN** `nix/packages/launcher.m` is read
- **THEN** it reads `BoomdriveDrives`, titles its status item `Boomdrive`, names `Default stream` and `Quit Boomdrive`, and runs the auxiliary executable `boomdrive-launcher`

### Requirement: The launcher is made for the drives it is given

`nix/packages/launcher.nix` SHALL take `drives`, a list of entries, each a drive's name, `drive`, and the session it opens, `session`. The flake's own `boomdrive-launcher` SHALL be made with none. The compiled menu SHALL be the same path whatever drives the launcher is made for.

#### Scenario: No drive

- **WHEN** the flake's own `boomdrive-launcher` is evaluated
- **THEN** its `Info.plist` gives `BoomdriveDrives` as an empty array on `aarch64-darwin`, and its desktop entry gives no `Actions` on `x86_64-linux`

### Requirement: On macOS each drive is in the menu's Info.plist

On `aarch64-darwin`, the app's `Info.plist` SHALL give `BoomdriveDrives`, an array of each entry's `drive` and `session`, in the order the launcher is given them, its strings escaped for XML.

#### Scenario: Each drive in the menu on macOS

- **WHEN** the `aarch64-darwin` package is made for the drive `garden`, with the session `garden`, and the drive `home`, with the session `house`
- **THEN** its `Info.plist` gives `BoomdriveDrives` as `garden` with `garden`, then `home` with `house`, and its compiled menu is the flake's own package's

### Requirement: On Linux each drive is an action of the desktop entry

On `aarch64-linux` and `x86_64-linux`, the desktop entry SHALL give one action for each entry, whose id and name are the entry's `drive`, and which runs the script with the entry's `session` as its one argument, quoted as the Desktop Entry Specification's `Exec` key asks.

#### Scenario: Each drive an action on Linux

- **WHEN** the `x86_64-linux` package is made for the drive `garden`, with the session `garden`, and the drive `home`, with the session `house`
- **THEN** its desktop entry gives `Actions=garden;home`, the action `garden` named `garden` running the script with `"garden"`, and the action `home` named `home` running it with `"house"`

#### Scenario: A session quoted in its action

- **WHEN** the `x86_64-linux` package is made for the drive `home`, with the session `my $work`
- **THEN** the action `home`'s `Exec` runs the script with `"my \\$work"`
