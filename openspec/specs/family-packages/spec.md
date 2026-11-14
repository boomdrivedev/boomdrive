# family-packages Specification

## Purpose
The flake's package set: the runner at this tree, and Flume, sluice and their herdr plugins at release tags, each a derivation of the source the flake's lock names by its hash, for `aarch64-darwin`, `aarch64-linux` and `x86_64-linux`. It covers what the set holds, how each package is made and versioned, the runner's git crates' hashes, how a set is proven by the runner's tests with the set's own binaries, and how a host overrides a tool (core.44, core.45; record 0020).

## Requirements

### Requirement: The flake gives the family's host tools as packages

The flake SHALL give, for each of its three systems, the packages `boomdrive`, `boomdrive-claude-plugins`, `boomdrive-launcher`, `crew`, `flume`, `flume-herdr-plugin`, `ghostty`, `herdr`, `herdr-plugins`, `pendingyou`, `pendingyou-herdr-plugin`, `plannotator`, `plannotator-herdr-plugin`, `plannotator-tui`, `portless`, `sluice`, `sluice-herdr-plugin`, `timber`, `timber-herdr-plugin` and `worktrunk`, `default` the runner's. It SHALL give no git, the host's own, and no tool a repository pins.

#### Scenario: Every system's packages

- **WHEN** the flake's outputs are evaluated for each of the three systems
- **THEN** each gives the twenty packages and `default`, each a derivation, and `default` is `boomdrive`, and the flake gives no other system

#### Scenario: No tool of the host's own

- **WHEN** the flake's packages are listed for any system
- **THEN** none is git

### Requirement: The flake and its lock name one set

Each tool of the set that comes through its own flake or its own source, Flume, sluice, crew, herdr, portless, worktrunk and timber, SHALL be named in `flake.nix` by a release tag, and the lock SHALL lock that tag. A tag in `flake.nix` that the lock does not lock SHALL fail the flake's type check, naming the tool and both tags. A tool of the set named by anything but a release tag SHALL fail it, naming the tool and what names it.

#### Scenario: A tag moved without the lock

- **WHEN** `flake.nix` names sluice's tag `v0.68.0` and the lock locks `v0.67.0`
- **THEN** the flake's type check fails, naming sluice, `v0.68.0` and `v0.67.0`

#### Scenario: A tool named by no release tag

- **WHEN** `flake.nix` and the lock name crew by `main`
- **THEN** the flake's type check fails, naming crew and `main`

#### Scenario: herdr named by no release tag

- **WHEN** `flake.nix` and the lock name herdr by `main`
- **THEN** the flake's type check fails, naming herdr and `main`

#### Scenario: portless named by no release tag

- **WHEN** `flake.nix` and the lock name portless by `main`
- **THEN** the flake's type check fails, naming portless and `main`

#### Scenario: worktrunk named by no release tag

- **WHEN** `flake.nix` and the lock name worktrunk by `main`
- **THEN** the flake's type check fails, naming worktrunk and `main`

#### Scenario: timber named by no release tag

- **WHEN** `flake.nix` and the lock name timber by `main`
- **THEN** the flake's type check fails, naming timber and `main`

### Requirement: The runner's git crates are hashed at the revision the lock names

Each git package of `Cargo.lock` SHALL have one entry among the runner's package's fixed hashes, holding the revision its hash was taken at. The flake's test SHALL fail where a git package has no entry, an entry names no git package, or an entry's revision is not the one `Cargo.lock` names.

#### Scenario: weir moved without its hash

- **WHEN** `Cargo.lock` names weir's git crates at a revision other than the one their entry holds
- **THEN** the flake's test fails, naming the crate and both revisions

### Requirement: A tool's herdr plugin is the tool's, at the tool's tag

Flume's, sluice's and timber's herdr plugin packages SHALL each be its tool's `plugins/herdr` folder, taken from the source of the tool's own package, as the tool ships it, with the tool's version.

#### Scenario: Flume's plugin moves with Flume

- **WHEN** the lock names Flume's tag `v0.10.0`
- **THEN** `flume-herdr-plugin` is that tag's `plugins/herdr`, its version `0.10.0`

#### Scenario: timber's plugin moves with timber

- **WHEN** the lock names timber's tag `v0.6.0`
- **THEN** `timber-herdr-plugin` is that tag's `plugins/herdr`, taken from the source of the set's `timber`, its version `0.6.0`, and nothing in it rewritten

### Requirement: The set is the one the runner's tests pass with

The gate SHALL run the set's own Flume and sluice, the flake's packages of its lock, given to the runner's tests by their paths. Those tests SHALL check each program's version against its tag, and read the answers a scratch host gives with the runner's own readers. No test of the set SHALL run a Flume or a sluice found on the host's `PATH`.

#### Scenario: Each tool's version

- **WHEN** the runner's tests run in the gate
- **THEN** the set's Flume says `flume <Flume's tag without the v>` and its sluice says `sluice <sluice's tag without the v>`

#### Scenario: Flume's status of a scratch host

- **WHEN** the set's Flume is asked `--json status` on a scratch host that holds no place
- **THEN** the runner reads the answer as Flume's status, with no place and the host part as Flume printed it

#### Scenario: The set's Flume keeps the runner's watch

- **WHEN** the set's Flume is asked `--json settle --every 60s` on a scratch host that holds no place, as the runner asks it
- **THEN** the runner reads the answer as given, and the set's Flume's `--json status` then gives the host's watch with a bound of 60 seconds

#### Scenario: The set's Flume takes the runner as a recipient

- **WHEN** the set's Flume, on a scratch host, is asked `--json launch` with `--recipient scratch/planner --recipient boomdrive` in a folder that holds no repository, and `--json confirm --recipient=boomdrive -- <a key no end is under>`, each as the runner asks it
- **THEN** each exits 1 with Flume's refusal, which the runner reads as Flume's reason, and neither is a usage error; and the set's Flume is no older than the least the runner names

#### Scenario: A submission sluice has no record of

- **WHEN** the set's sluice is asked, in a scratch repository, for the entry of a submission it has no record of
- **THEN** the runner reads its answer as a missing entry

#### Scenario: Not the host's tools

- **WHEN** the runner's tests of the set run in a shell whose `PATH` holds another `flume` and `sluice`
- **THEN** they run the set's, by their paths

### Requirement: Each package is made from this tree, its tool's own flake or source at its release tag, or the flake's nixpkgs

Each package but crew's companions' SHALL be a derivation of the source the lock names by its hash: the runner's and the launcher's of this tree; Flume's, sluice's, crew's, herdr's, worktrunk's and timber's of their release tags, each through its own flake; portless's of its release tag, by this flake's recipe; and Ghostty's of the flake's nixpkgs. Each tool's version but Ghostty's SHALL be its tag's without the `v`, read from its tagged tree where it holds one, else from the flake's tag.

#### Scenario: Flume through its own flake

- **WHEN** the lock names a tag of Flume's whose tree carries its version
- **THEN** the `flume` package is Flume's flake's own package for that system, unchanged, and its version is the tag's without the `v`

#### Scenario: sluice through its own flake

- **WHEN** the lock names sluice's tag `v0.68.0`, whose tree says `0.0.0`
- **THEN** the `sluice` package is sluice's flake's, its version `0.68.0`, and its build is given `0.68.0` as sluice's release build is

#### Scenario: crew through its own flake

- **WHEN** the lock names a tag of crew's whose tree carries its version
- **THEN** the `crew` package is crew's flake's own `crew` for that system, unchanged, and its version is the tag's without the `v`

#### Scenario: herdr through its own flake

- **WHEN** the lock names herdr's tag `v0.9.5`
- **THEN** the `herdr` package is herdr's flake's own `herdr` for that system, unchanged, and its version is `0.9.5`

#### Scenario: worktrunk through its own flake

- **WHEN** the lock names worktrunk's tag `v0.80.0`
- **THEN** the `worktrunk` package is worktrunk's flake's own `worktrunk` for that system, its source and dependencies unchanged, its program `wt` and its version `0.80.0`, and its build is given `v0.80.0` as its `git describe`, as worktrunk's release build gets it

#### Scenario: timber through its own flake

- **WHEN** the lock names timber's tag `v0.6.0`, whose tree carries its version
- **THEN** the `timber` package is timber's flake's own `default` for that system, unchanged, its program `timber` and its version `0.6.0`

#### Scenario: portless from its source

- **WHEN** the lock names portless's tag `v0.17.0`
- **THEN** the `portless` package of each system is this flake's recipe's build of that tag's source, its program `portless`, and its version `0.17.0`, read from the tagged tree's `packages/portless/package.json`

#### Scenario: portless other than its tag

- **WHEN** portless's tagged tree says `0.17.1` and the lock names portless's tag `v0.17.0`
- **THEN** the flake's test fails, naming `0.17.1` and `0.17.0`

#### Scenario: The runner from this tree

- **WHEN** the flake is evaluated
- **THEN** the `boomdrive` package's source holds this tree's `Cargo.toml`, `Cargo.lock` and `crates`, and no other path of the tree

### Requirement: The runner's version is its tree's

The `boomdrive` package's version SHALL be its tree's, `Cargo.toml`'s workspace version, which a release commit alone writes (core.48).

#### Scenario: The runner at its tree's version

- **WHEN** the flake is evaluated for each system it gives
- **THEN** the `boomdrive` package's version is the one `Cargo.toml` gives under `[workspace.package]`: `0.0.0` at a commit of `main`, and the release's at a release tag

### Requirement: herdr is the release Flume's terminal profile was checked against

The `herdr` package's version SHALL be the herdr that Flume's tree pins at the tag the lock names for Flume: the release Flume's terminal profile was checked against, `herdrVersion` in Flume's `devenv.nix`. No herdr plugin of the set SHALL ask, by its manifest's `min_herdr_version`, for a herdr newer than the set's. The flake's test SHALL fail where either does not hold, naming both versions.

#### Scenario: herdr is Flume's

- **WHEN** the flake is evaluated
- **THEN** the set's herdr is the version Flume's `devenv.nix` pins at the tag the lock names for Flume, and none of Flume's, sluice's and timber's herdr plugins asks for a newer one

#### Scenario: herdr other than Flume's

- **WHEN** Flume's tree pins herdr `0.9.6` and the lock names herdr's tag `v0.9.5`
- **THEN** the flake's test fails, naming `0.9.6` and `0.9.5`

#### Scenario: A plugin newer than herdr

- **WHEN** a herdr plugin of the set asks for herdr `0.9.9` and the set's herdr is `0.9.5`
- **THEN** the flake's test fails, naming the plugin, `0.9.9` and `0.9.5`

### Requirement: Ghostty is the release Flume's window profile was checked against

The `ghostty` package SHALL be the flake's nixpkgs' Ghostty: `ghostty`, built from source, on each Linux system, and `ghostty-bin`, Ghostty's own release, on `aarch64-darwin`, where nothing builds its macOS app from source. Its version SHALL be `1.4.0`, the release Flume's window profile was checked against (Flume's C.8), and the flake's test SHALL fail on another, naming both versions.

#### Scenario: Ghostty on each system

- **WHEN** the flake's packages are evaluated
- **THEN** `ghostty` is nixpkgs' `ghostty-bin` on `aarch64-darwin` and nixpkgs' `ghostty` on `aarch64-linux` and `x86_64-linux`, each at `1.4.0`

#### Scenario: Ghostty moved with nixpkgs

- **WHEN** a system's Ghostty is `1.5.0`
- **THEN** the flake's test fails, naming that system, `1.5.0` and `1.4.0`

### Requirement: portless is built from its package alone and runs on the flake's Node.js

The `portless` package SHALL be built from its workspace's package `packages/portless` alone, with the flake's nixpkgs' pnpm 11. Its dependencies SHALL be one fetch of that package's alone, whose hash is the same on every system. Its program `portless` SHALL run that build on the flake's nixpkgs' Node.js 24. It SHALL find the flake's nixpkgs' openssl, procps, which and hostname after the host's own on its `PATH`. It SHALL bring no git, which is the host's own.

#### Scenario: One fetch for every system

- **WHEN** the flake's packages are evaluated
- **THEN** each system's `portless` takes its dependencies from a fetch of the workspace's `portless` package alone, under pnpm 11, and the fetch's hash is the same on every system

#### Scenario: The flake's Node.js

- **WHEN** the flake's packages are evaluated
- **THEN** each system's `portless` runs on the flake's nixpkgs' `nodejs_24` for that system

#### Scenario: portless runs the host's git

- **WHEN** the programs portless's `PATH` is given are listed, for any system
- **THEN** they are the flake's nixpkgs' `openssl`, `procps`, `which` and `hostname` for that system, and none is git

### Requirement: worktrunk is the release sluice's worktree driver was checked against

The `worktrunk` package's version SHALL be the worktrunk that sluice's tree pins at the tag the lock names for sluice: the release its worktree driver's tests drive, the `version` of the `worktrunk` block of sluice's `devenv.nix`. The flake's test SHALL fail where it does not hold, naming both versions, and SHALL fail where sluice's pin cannot be read, naming the file.

#### Scenario: worktrunk is sluice's

- **WHEN** the flake is evaluated
- **THEN** each system's worktrunk is the version sluice's `devenv.nix` pins at the tag the lock names for sluice

#### Scenario: worktrunk other than sluice's

- **WHEN** sluice's tree pins worktrunk `0.81.0` and the set's worktrunk is `0.80.0`
- **THEN** the flake's test fails, naming `0.81.0` and `0.80.0`

### Requirement: The packages of crew's companions are made from what each publishes

The packages of crew's companions SHALL each be a derivation of the source the lock names by its hash: Pending You's of its npm package, as its source tags no release, its version its `package.json`'s; Plannotator's and plannotator-tui's numtide/llm-agents.nix's, built from each one's source at its release tag, its version the tag's without the `v`; Pending You for Herdr's and Annotate's of the commit the lock names; and the set's marketplace of this tree, made of those packages.

#### Scenario: Pending You from its npm package

- **WHEN** the lock names Pending You's npm package `pendingyou-0.41.0.tgz`
- **THEN** the `pendingyou` package of each system holds that package's `dist`, `mod` and `package.json`, its program `pendingyou`, and its version `0.41.0`

#### Scenario: Pending You other than its package

- **WHEN** the input's address names `0.41.0` and its `package.json` says `0.41.1`
- **THEN** the flake's test fails, naming `0.41.1` and `0.41.0`

#### Scenario: Plannotator and plannotator-tui through llm-agents.nix

- **WHEN** the flake's packages are evaluated for any system
- **THEN** `plannotator` and `plannotator-tui` are numtide/llm-agents.nix's packages of those names for that system, unchanged, their sources backnotprop/plannotator at `v0.28.61` and plannotator/plannotator-tui at `v0.9.9`, and their versions `0.28.61` and `0.9.9`

#### Scenario: Plannotator other than its tag

- **WHEN** llm-agents.nix's `plannotator` is version `0.28.62` and its source's tag `v0.28.62`
- **THEN** the flake's test fails, naming `plannotator`, `0.28.62` and `0.28.61`

### Requirement: Pending You runs on the flake's Node.js

The `pendingyou` package's program SHALL run its `dist/cli.js` on the flake's nixpkgs' `nodejs_24`.

#### Scenario: Pending You's Node.js

- **WHEN** the flake's packages are evaluated
- **THEN** each system's `pendingyou` runs on the flake's nixpkgs' `nodejs_24` for that system

### Requirement: Pending You for Herdr runs the set's Pending You

`pendingyou-herdr-plugin` SHALL be recordplane/herdr-pendingyou at the commit the lock names, copied whole, its version its `herdr-plugin.toml`'s. The Pending You its `bin/run.sh` names SHALL be the set's.

#### Scenario: Pending You for Herdr names the set's Pending You

- **WHEN** the flake's packages are evaluated
- **THEN** `pendingyou-herdr-plugin`'s `bin/run.sh` names `pendingyou@0.41.0`, the set's `pendingyou`'s version

#### Scenario: Pending You for Herdr other than the set's

- **WHEN** the plugin's `bin/run.sh` names `pendingyou@0.40.0` and the set's Pending You is `0.41.0`
- **THEN** the flake's test fails, naming `0.40.0` and `0.41.0`

### Requirement: Annotate holds its runtime and the set's plannotator-tui

`plannotator-herdr-plugin` SHALL be plannotator/herdr-annotate at the commit the lock names, whose `plannotator-tui.version` SHALL be the set's plannotator-tui's. Its `bin` SHALL hold its runtime, built from that commit's `rust` source, and the set's plannotator-tui's program, each beside the version file its build step reads, so herdr's link fetches nothing.

#### Scenario: Annotate holds its runtime and the set's plannotator-tui

- **WHEN** the flake's packages are evaluated for any system
- **THEN** `plannotator-herdr-plugin`'s runtime is built from its source's `rust` folder with that folder's `Cargo.lock`, and the plannotator-tui it holds is the set's `plannotator-tui`

#### Scenario: Annotate other than the set's plannotator-tui

- **WHEN** Annotate's `plannotator-tui.version` says `0.9.8` and the set's plannotator-tui is `0.9.9`
- **THEN** the flake's test fails, naming `0.9.8` and `0.9.9`

### Requirement: Pending You's plugin holds its hooks and its skill, and no wake mod and no MCP server

The `pendingyou` plugin SHALL hold Pending You's hooks, each `"$HOME/.config/pendingyou/bin/pendingyou-hook" <sub> --app claude-code || true`, for the events and subcommands Pending You's `init` gives Claude Code, in its order, each with a 10-second timeout but SessionEnd's, which have none. It SHALL hold Pending You's skill. It SHALL hold no wake mod, which is Pending You's folder's, and no MCP server.

#### Scenario: Pending You's hooks

- **WHEN** the `pendingyou` plugin's hooks are read
- **THEN** they are SessionStart `pickup`, UserPromptSubmit `handoff`, Stop `stopcheck`, SessionEnd `presence`, PermissionRequest `permission`, PostToolUse and PostToolUseFailure `permission-done`, SessionEnd `permission-done` and Notification `notify` matching `permission_prompt`, each running the launcher by `$HOME` with `--app claude-code`

#### Scenario: Pending You's hooks against its own

- **WHEN** the set's Pending You's `dist/claude.js` gives Claude Code a hook the plugin does not hold
- **THEN** the flake's test fails, naming the hook's event and subcommand

#### Scenario: No wake mod in the plugin

- **WHEN** the `pendingyou` plugin is read
- **THEN** it holds no `register.js` and no `wake.js`, and its hooks name no module

#### Scenario: No MCP server in the plugin

- **WHEN** the `pendingyou` plugin is read
- **THEN** it holds no `.mcp.json` and its manifest names no MCP server

### Requirement: Plannotator's plugin is its command's source's

The `plannotator` plugin SHALL be the `apps/hook` folder of the source the set's `plannotator` is built from, copied whole.

#### Scenario: Plannotator's plugin from its command's source

- **WHEN** the flake's packages are evaluated
- **THEN** the `plannotator` plugin is copied from the `apps/hook` of the source the set's `plannotator` is built from

### Requirement: The set's herdr plugins are one package a profile holds

The `herdr-plugins` package SHALL hold each of the set's herdr plugins at `share/herdr/plugins/<tool>`: `flume`, the `flume-herdr-plugin` package, and `sluice`, the `sluice-herdr-plugin` package, and nothing else, so one Nix profile holds both beside the set's other packages. Setup links each from the profile with herdr's own command (the setup spec; record 0009 as amended 2026-11-09).

#### Scenario: The plugins in one package

- **WHEN** the flake's `herdr-plugins` is evaluated for each of the three systems
- **THEN** it holds `share/herdr/plugins/flume`, the system's `flume-herdr-plugin`, and `share/herdr/plugins/sluice`, the system's `sluice-herdr-plugin`, and no `bin`

### Requirement: The set's marketplace lists Pending You's, Plannotator's and timber's plugins

`boomdrive-claude-plugins` SHALL be one folder, a Claude Code marketplace named `boomdrive` whose `.claude-plugin/marketplace.json` lists three plugins, `pendingyou` at the set's Pending You's version, `plannotator` at the set's Plannotator's and `timber` at the set's timber's, each at `./plugins/<name>`. No file of the folder SHALL be a link.

#### Scenario: The marketplace's three plugins

- **WHEN** the flake's packages are evaluated
- **THEN** `boomdrive-claude-plugins` gives the marketplace `boomdrive` with `pendingyou` at `0.41.0`, `plannotator` at `0.28.61` and `timber` at `0.6.0`

#### Scenario: No link into the store

- **WHEN** the marketplace's build is read
- **THEN** it copies every file with links followed, and fails where any file of its output is a link

### Requirement: timber's plugins say the set's timber's version

The version of `timber-herdr-plugin`'s `herdr-plugin.toml` and of the marketplace's `timber` plugin's `.claude-plugin/plugin.json` SHALL each be the set's `timber`'s. The flake's test SHALL fail where one is not, naming the plugin and both versions.

#### Scenario: timber's plugins say the set's timber

- **WHEN** the flake is evaluated
- **THEN** each system's `timber-herdr-plugin` manifest and the marketplace's `timber` plugin manifest say the version of that system's `timber`

#### Scenario: A timber plugin other than the set's timber

- **WHEN** timber's herdr plugin says `0.5.3` and the set's timber is `0.6.0`
- **THEN** the flake's test fails, naming the plugin, `0.5.3` and `0.6.0`

### Requirement: timber's Claude Code plugin is its command's source's

The marketplace's `timber` plugin SHALL be the `plugins/claude-code` folder of the source the set's `timber` is built from, copied whole.

#### Scenario: timber's Claude Code plugin from its command's source

- **WHEN** the flake's packages are evaluated
- **THEN** the `timber` plugin is copied whole from the `plugins/claude-code` of the source the set's `timber` is built from
