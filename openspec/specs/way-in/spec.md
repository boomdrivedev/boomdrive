# way-in Specification

## Purpose
The person's ways to see the host's agents that the flake gives: the Boomdrive launcher, an app on macOS and a desktop entry on Linux that runs `flume open`, and the flake's shell, whose `PATH` is the set's tools and which starts herdr on entering. Each runs at the person's asking, and the runner starts and names no multiplexer (record 0023; core.13, core.29).

## Requirements

### Requirement: The launcher opens the host's agents at a person's asking

The `boomdrive-launcher` package SHALL, each time it is opened, run `flume open` once, with no session, so Flume shows the default stream's own session in its configured view (Flume's statement 186). It SHALL put the running user's profiles and the system's before the `PATH` it is given, so it runs the host's one Flume. It SHALL start, keep and name no multiplexer itself (core.13, core.29).

#### Scenario: Opened

- **WHEN** the launcher runs and Flume opens a window, exiting 0
- **THEN** Flume is run once, as `flume open`, and nothing is shown

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

On `aarch64-darwin`, the `boomdrive-launcher` package SHALL hold `Applications/Boomdrive.app`, whose `Info.plist` names the bundle `Boomdrive` and its executable the launcher, so nix-darwin places it in `/Applications/Nix Apps`. On `aarch64-linux` and `x86_64-linux`, it SHALL hold `share/applications/boomdrive.desktop`, named `Boomdrive`, which runs the launcher in no terminal.

#### Scenario: The app on macOS

- **WHEN** the `boomdrive-launcher` package of `aarch64-darwin` is evaluated
- **THEN** it holds `Applications/Boomdrive.app/Contents/Info.plist`, whose `CFBundleName` is `Boomdrive` and whose `CFBundleExecutable` is the launcher in `Contents/MacOS`

#### Scenario: The desktop entry on Linux

- **WHEN** the `boomdrive-launcher` package of `x86_64-linux` is evaluated
- **THEN** it holds `share/applications/boomdrive.desktop`, whose `Name` is `Boomdrive`, whose `Exec` runs the launcher, and whose `Terminal` is false

### Requirement: The flake gives a shell that starts herdr

The flake SHALL give `devShells.<system>.default` for each of its three systems. Its `PATH` SHALL hold the set's `boomdrive`, `flume`, `sluice`, `crew`, `herdr` and `portless`. Entering it interactively SHALL start herdr once, as herdr's own start with no argument, and leave the person in the shell when herdr ends. A command run through the shell SHALL start no herdr, and neither SHALL a shell entered from inside herdr. The shell SHALL link no plugin and write no file.

#### Scenario: Every system's shell

- **WHEN** the flake's `devShells` are evaluated for each of the three systems
- **THEN** each gives `default`, a derivation whose packages are the set's `boomdrive`, `flume`, `sluice`, `crew`, `herdr` and `portless`

#### Scenario: Entering the shell

- **WHEN** the shell's hook runs in an interactive shell, outside herdr
- **THEN** herdr is run once, with no argument, and the shell goes on once herdr exits

#### Scenario: A command through the shell

- **WHEN** the shell's hook runs in a shell that is not interactive, as `nix develop --command` runs one
- **THEN** no herdr is run

#### Scenario: Inside herdr already

- **WHEN** the shell's hook runs in an interactive shell whose `HERDR_ENV` is `1`
- **THEN** no herdr is run
