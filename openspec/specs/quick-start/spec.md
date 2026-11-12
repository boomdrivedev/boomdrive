# quick-start Specification

## Purpose
Boomdrive's quick start, the site's page `/start/`, which takes a stranger with nothing of the family installed, on an Apple silicon Mac or a NixOS machine, to a drive whose agents build and land a change, each step's text a file the page shows as it stands and a test of the gate runs as it stands (record 0043; site.7, site.8).

## Requirements

### Requirement: The quick start is one page of a Mac's steps and NixOS's

The site SHALL serve the quick start at `/boomdrive/start/`, titled `Quick start`, linked from the masthead's sections as `Start`. It SHALL open on the words of `src/content/pages/start.md`, then show the section `On a Mac` and the section `On NixOS`, each holding its steps in their order, numbered from 1. Every step SHALL be on the page with no script, and the page SHALL hold no script of its own.

#### Scenario: The page and its two ways

- **WHEN** `site:test` reads `site/dist/start/index.html`
- **THEN** it holds the lede, then the section `On a Mac` with each Mac step numbered in order, then the section `On NixOS` with each NixOS step numbered in order, and no `script` element

#### Scenario: Reached from the masthead

- **WHEN** `site:test` reads any built page
- **THEN** its masthead links `/boomdrive/start/` as `Start`

### Requirement: A step is a record of its words, its files and its test

Each step SHALL be one Markdown record of the collection `start`, holding its title, its place on a Mac, on NixOS or both, each file it shows by its path under `site/src/start/` with the path the reader writes it to where they write one, and its test as a project, `boomdrive` or `nix`, and a name. The build SHALL refuse a step with no place, no file or no test.

#### Scenario: Every step is whole

- **WHEN** `site:check` and `site:build` run
- **THEN** each record under `site/src/content/start/` gives a place on at least one platform, at least one file and a test, and neither reports an error

#### Scenario: A platform's places are its order

- **WHEN** `site:test` reads the steps
- **THEN** the places each platform's steps give are 1 up to their count, each once

### Requirement: The page shows each step's files as they stand

The page SHALL show each file a step names in a block of its own, its text exactly as the file holds it, and SHALL name above the block the path the reader writes it to where the step gives one. No step's file SHALL be shown in any other form.

#### Scenario: Each file as it stands

- **WHEN** `site:test` reads the built page and each file a step names
- **THEN** each file's text, escaped as HTML, is the text of one block under that step, with the step's path to write it to before it where the step gives one

### Requirement: Every step names a test its project holds, and every file is a step's

Each step's test SHALL be a test its project holds: for `boomdrive`, a test function of `crates/boomdrive/tests/quick_start.rs` by its name; for `nix`, a case of `nix/tests/quick-start.nix` by its name. Every file under `site/src/start/` but the folder's shellcheck settings SHALL be a file some step shows (site.8).

#### Scenario: Each step's test is there

- **WHEN** `site:test` reads each step's test
- **THEN** `crates/boomdrive/tests/quick_start.rs` holds `fn <name>(` for each `boomdrive` test, and `nix/tests/quick-start.nix` holds `"<name>" =` for each `nix` test

#### Scenario: No file is left unshown

- **WHEN** `site:test` lists `site/src/start/`
- **THEN** every file there but `.shellcheckrc` is named by a step

### Requirement: The quick start names none of ours

No step's words or files, and nothing on the page, SHALL name a drive or a machine of the fleet's contexts. The reader's own names SHALL be the placeholders `your-github-name`, `your-repository`, `your-host` and `you`, the account `me` and the drive `home` (site.7).

#### Scenario: None of our drives or machines

- **WHEN** `site:test` reads every drive's and machine's name in `site/src/content/contexts/contexts.yaml`
- **THEN** none appears on the built quick start, in a step's record or in a file under `site/src/start/`

### Requirement: The page's blocks follow the reader's theme and width

A step's block SHALL take its colours from the page's own tokens, carrying no colour of its own, so it follows light and dark as the page does, and SHALL scroll within itself where its lines pass the page's width, so the page never scrolls sideways at phone width.

#### Scenario: No colour of its own

- **WHEN** `site:test` reads the built quick start
- **THEN** no block of a step's file carries a `style` attribute or a colour of its own

#### Scenario: A long line at phone width

- **WHEN** `site:test` reads the site's built stylesheet
- **THEN** a step's block scrolls along its own lines, `overflow-x: auto`, and wraps none of them

### Requirement: The Mac's install brings the set and what the steps run

The Mac's install SHALL be one `nix profile add` of installables, each a package for `aarch64-darwin`: the set's `boomdrive`, `flume`, `sluice`, `herdr`, `herdr-plugins` and `crew` from the flake README's install names, and nixpkgs' `git`, `gh` and `nodejs`, which hold every program the later steps and setup run.

#### Scenario: The Mac's install

- **WHEN** `nix:test` reads `site/src/start/mac/install.sh`
- **THEN** it is one `nix profile add` whose installables are exactly those nine, each the flake README's install names or `nixpkgs`, each a package of `aarch64-darwin`

### Requirement: The sign-ins are those setup names

The sign-in step SHALL be one line for each sign-in setup checks, each the fix setup names: `gh auth login`, `claude auth login` and `npx -y pendingyou@latest init`, and nothing else.

#### Scenario: Each sign-in as setup names it

- **WHEN** `boomdrive:test` runs setup over stand-ins where GitHub, Claude Code and Pending You are each signed out, and reads `site/src/start/both/sign-in.sh`
- **THEN** the file's lines are, in order, the command setup's refusal names for GitHub and the fixes its answer names for Claude Code and Pending You

### Requirement: A Mac's Flume files open agents in herdr, where crew's roles answer

The Mac's Flume step SHALL write `~/.config/flume/config.toml` with the `[host]` keys `multiplexer = "herdr"` and `terminal_app = "terminal"`, the drive's stream `home` and the repository's work in it, and `~/.config/flume/sources/crew.toml` naming crew's kinds folder and roles file as the set's module names them, which the set's Flume SHALL take with no refusal, until Flume has a command that sets them, which setup runs (record 0043; principle 11).

#### Scenario: The set's Flume takes the file

- **WHEN** `boomdrive:test` writes `site/src/start/mac/flume.toml` and `site/src/start/mac/crew-source.toml` where the reader writes them in a scratch home, beside crew's roles file as crew writes it, and runs the set's Flume `--json status` there
- **THEN** Flume answers with no refusal, its default stream and the stream `home` each give the multiplexer `herdr` and the terminal app `terminal`, and its source `crew` names crew's kinds folder and roles file under the home and reads the five roles of `home` from it

#### Scenario: The Mac's source is the set's

- **WHEN** `nix:test` evaluates the NixOS drive's system and reads `site/src/start/mac/crew-source.toml`
- **THEN** the file's keys are exactly those the set's module gives Flume as the source `crew`

### Requirement: The Mac's setup sets up the drive as written

The Mac's setup step SHALL be one `boomdrive setup` line, which, run as written over stand-ins for gh, crew, herdr, `launchctl` and `hostname`, SHALL set up the account `me`'s drive `home` keeping `your-github-name/your-repository` under crew, with the state repository `your-github-name/drive-state`.

#### Scenario: Setup as written

- **WHEN** `boomdrive:test` runs `site/src/start/mac/setup.sh` through `sh`, its `PATH` the runner built from the tree, then the stand-ins
- **THEN** it exits 0, and the runner's file it wrote names the account `me`'s drive `home` keeping `your-github-name/your-repository`, and crew's table the partition `home` with `your-github-name/drive-state`

### Requirement: NixOS's flake takes the set's module and the drive's file

The NixOS flake step SHALL give `/etc/nixos/flake.nix`, whose `boomdrive` input is the flake README's install names, and whose outputs give `nixosConfigurations.your-host` from the reader's `./configuration.nix`, the set's `nixosModules.default` and `./boomdrive.nix`, the drive step's file.

#### Scenario: The host's flake

- **WHEN** `nix:test` calls `site/src/start/nixos/flake.nix`'s outputs with this flake as `boomdrive` and a `nixpkgs` whose `lib.nixosSystem` gives back its arguments
- **THEN** `nixosConfigurations.your-host`'s modules are `./configuration.nix`, the set's `nixosModules.default` and `./boomdrive.nix`, and its `boomdrive` input's address is the one README's install names

### Requirement: NixOS's drive file gives the drive to the runner, crew and Flume

The NixOS drive step SHALL give `/etc/nixos/boomdrive.nix`, a module that trusts the family's cache as the flake's `nixConfig` names it, installs `git`, `gh` and `nodejs`, and sets the account `me`'s drive `home` keeping `your-github-name/your-repository` under crew, with crew's host and crew's drive `home` naming `your-host` and `your-github-name/drive-state`.

#### Scenario: The drive on NixOS

- **WHEN** `nix:test` evaluates a system of `x86_64-linux` with the user `you`, the set's NixOS module and `site/src/start/nixos/boomdrive.nix`
- **THEN** the runner's file names the account `me`'s drive `home` keeping `your-github-name/your-repository` under crew with host `your-host`, crew's drive `home` names `your-github-name/drive-state`, Flume's host multiplexer is `herdr`, Flume is given the stream `home` with the repository's work in it and crew's source, and the cache and its key are the flake's `nixConfig`'s

### Requirement: NixOS's switch builds the configuration the flake gives

The NixOS switch step SHALL be one `sudo nixos-rebuild switch --flake /etc/nixos#your-host` line, naming the folder the flake step writes to and the configuration it gives, whose system SHALL evaluate.

#### Scenario: The switch's system

- **WHEN** `nix:test` reads `site/src/start/nixos/switch.sh` and evaluates the system of the drive's file
- **THEN** the line names `/etc/nixos` and `your-host`, and the system's `system.build.toplevel` evaluates to a derivation

### Requirement: The state repository is made as setup makes it

The NixOS state step SHALL be one line, the `gh` command setup runs on a Mac to make the state repository `your-github-name/drive-state` where none exists.

#### Scenario: Made as setup makes it

- **WHEN** `boomdrive:test` runs setup over stand-ins where gh finds no `your-github-name/drive-state`, and reads `site/src/start/nixos/state.sh`
- **THEN** the file's one line is the `gh repo create` call setup made, word for word

### Requirement: The landing step's files are the ones sluice and Flume land main by

The landing step SHALL give, for the drive's repository, `.config/sluice.toml`, a policy that lands `main` on `origin` after one command any repository runs, and `.config/flume.toml`, binding sluice as the repository's merge queue, and the lines that commit both and push them to `main`. They SHALL leave a `main` whose policy the set's sluice reads with no refusal, and whose binding the set's Flume reads, so a kind that lands launches there.

#### Scenario: The policy on main

- **WHEN** `boomdrive:test` writes `site/src/start/both/sluice.toml` and `site/src/start/both/flume.toml` into a scratch clone of a bare origin, runs `site/src/start/both/land.sh` there through `sh`, and, in another clone, asks the set's sluice of `main` and launches a fix with the set's Flume, its stage held
- **THEN** origin's `main` holds both files, sluice reads `main`'s policy and refuses nothing, and Flume launches the fix to land, its base from the merge queue

### Requirement: The drive's agents start as setup says

The start step SHALL be the one line setup's answer gives as `start` for the drive `home`.

#### Scenario: Started as setup says

- **WHEN** `boomdrive:test` runs setup with `--json` over stand-ins for the drive `home`, and reads `site/src/start/both/start.sh`
- **THEN** the file's one line is the answer's `start`

### Requirement: A change the drive is given lands

A change the drive's control plane gives SHALL land on the drive's repository's `main` through the runner, the set's Flume and the set's sluice, under the landing step's policy and the Mac's Flume files, and the last step's line SHALL open the drive's stream's session through the set's Flume on that host.

#### Scenario: From crew's launch to main

- **WHEN** `boomdrive:test` puts a fix of `main` for `your-github-name/your-repository` in a scratch state repository as crew writes it, then crew's send of it, runs the runner over the set's Flume and sluice under the Mac's Flume files with a stand-in agent that commits one change, the drive's stream printing its agents' commands where the gate runs no multiplexer, and runs `site/src/start/both/watch.sh`
- **THEN** the repository's origin holds the change on `main`, the runner's report carries the place's status as the set's Flume gives it once the change landed, and the set's Flume answers the watch line for the drive's stream with its own refusal under the printed command, which holds no session a window could show
