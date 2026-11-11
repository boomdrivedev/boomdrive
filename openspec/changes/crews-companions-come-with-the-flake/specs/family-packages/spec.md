## MODIFIED Requirements

### Requirement: The flake gives the family's host tools as packages

The flake SHALL give, for each of `aarch64-darwin`, `aarch64-linux` and `x86_64-linux`, the packages `boomdrive`, `boomdrive-claude-plugins`, `boomdrive-launcher`, `crew`, `flume`, `flume-herdr-plugin`, `ghostty`, `herdr`, `pendingyou`, `pendingyou-herdr-plugin`, `plannotator`, `plannotator-herdr-plugin`, `plannotator-tui`, `portless`, `sluice` and `sluice-herdr-plugin`, with `default` the runner's. It SHALL give no package of git, which is the host's own, and none of a tool a repository pins.

#### Scenario: Every system's packages

- **WHEN** the flake's outputs are evaluated for each of the three systems
- **THEN** each gives the sixteen packages and `default`, each a derivation, and `default` is `boomdrive`, and the flake gives no other system

#### Scenario: No tool of the host's own

- **WHEN** the flake's packages are listed for any system
- **THEN** none is git

### Requirement: A tool's herdr plugin is the tool's, at the tool's tag

Flume's and sluice's herdr plugin packages SHALL each be its tool's `plugins/herdr` folder, taken from the source of the tool's own package, as the tool ships it, with the tool's version.

#### Scenario: Flume's plugin moves with Flume

- **WHEN** the lock names Flume's tag `v0.10.0`
- **THEN** `flume-herdr-plugin` is that tag's `plugins/herdr`, its version `0.10.0`

### Requirement: Each package is made from this tree, its tool's own flake or source at its release tag, or the flake's nixpkgs

Each package SHALL be a derivation of the source the lock names by its hash:

- the runner's, the launcher's and the marketplace's, of this tree;
- Flume's, sluice's, crew's and herdr's, of their release tags, each through its own flake;
- portless's, of its release tag, by this flake's recipe, as it publishes no flake;
- Pending You's, of the package Pending You publishes on npm, as its source tags no release;
- Plannotator's and plannotator-tui's, through numtide/llm-agents.nix's flake, which builds each from its source at its release tag;
- Pending You for Herdr's and Annotate's, of the commit the lock names;
- Ghostty's, of the flake's nixpkgs.

Each tool's version but Ghostty's SHALL be its release's: its tag's without the `v`, read from its tagged tree where it holds one, else from the flake's tag; Pending You's from its package's `package.json`.

#### Scenario: Flume through its own flake

- **WHEN** the lock names a tag of Flume's whose tree carries its version
- **THEN** the `flume` package is Flume's flake's own package for that system, unchanged, and its version is the tag's without the `v`

#### Scenario: sluice through its own flake

- **WHEN** the lock names sluice's tag `v0.68.0`, whose tree says `0.0.0`
- **THEN** the `sluice` package is sluice's flake's, its version `0.68.0`, and its build is given `0.68.0` as sluice's release build is

#### Scenario: crew through its own flake

- **WHEN** the lock names a tag of crew's, whose tree carries no version
- **THEN** the `crew` package is crew's flake's, its source, name and home page crew's own, and its version is the tag's without the `v`

#### Scenario: herdr through its own flake

- **WHEN** the lock names herdr's tag `v0.9.5`
- **THEN** the `herdr` package is herdr's flake's own `herdr` for that system, unchanged, and its version is `0.9.5`

#### Scenario: portless from its source

- **WHEN** the lock names portless's tag `v0.17.0`
- **THEN** the `portless` package of each system is this flake's recipe's build of that tag's source, its program `portless`, and its version `0.17.0`, read from the tagged tree's `packages/portless/package.json`

#### Scenario: portless other than its tag

- **WHEN** portless's tagged tree says `0.17.1` and the lock names portless's tag `v0.17.0`
- **THEN** the flake's test fails, naming `0.17.1` and `0.17.0`

#### Scenario: The runner from this tree

- **WHEN** the flake is evaluated
- **THEN** the `boomdrive` package's source holds this tree's `Cargo.toml`, `Cargo.lock` and `crates`, and no other path of the tree

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

## ADDED Requirements

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

### Requirement: The set's marketplace lists Pending You's and Plannotator's plugins

`boomdrive-claude-plugins` SHALL be one folder, a Claude Code marketplace named `boomdrive` whose `.claude-plugin/marketplace.json` lists two plugins, `pendingyou` at the set's Pending You's version and `plannotator` at the set's Plannotator's, each at `./plugins/<name>`. No file of the folder SHALL be a link.

#### Scenario: The marketplace's two plugins

- **WHEN** the flake's packages are evaluated
- **THEN** `boomdrive-claude-plugins` gives the marketplace `boomdrive` with `pendingyou` at `0.41.0` and `plannotator` at `0.28.61`

#### Scenario: No link into the store

- **WHEN** the marketplace's build is read
- **THEN** it copies every file with links followed, and fails where any file of its output is a link

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
