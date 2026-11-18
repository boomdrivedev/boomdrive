## REMOVED Requirements

### Requirement: The set's marketplace lists Pending You's, Plannotator's and timber's plugins

**Reason**: The marketplace lists every Claude Code plugin a tool of the set ships, Flume's, sluice's, crew's and worktrunk's beside the companions' and timber's (record 0044 as amended on 2026-11-18).

**Migration**: "The set's marketplace lists every Claude Code plugin a tool of the set ships" holds the eight plugins and the folder's no link.

## ADDED Requirements

### Requirement: The set's marketplace lists every Claude Code plugin a tool of the set ships

`boomdrive-claude-plugins` SHALL be one folder, a Claude Code marketplace named `boomdrive` whose `.claude-plugin/marketplace.json` lists eight plugins, each at `./plugins/<name>` and at the version of the set's tool it belongs to: `pendingyou`, `plannotator`, `timber`, `flume`, `sluice` and `sluice-board`, both sluice's, `crew` and `worktrunk`. No file of the folder SHALL be a link.

#### Scenario: The marketplace's plugins

- **WHEN** the flake's packages are evaluated
- **THEN** `boomdrive-claude-plugins` gives the marketplace `boomdrive` with `pendingyou` at `0.41.0`, `plannotator` at `0.28.61`, `timber` at `0.7.0`, `flume` at `0.29.0`, `sluice` and `sluice-board` at `0.94.8`, `crew` at `0.64.2` and `worktrunk` at `0.80.0`

#### Scenario: No link into the store

- **WHEN** the marketplace's build is read
- **THEN** it copies every file with links followed, and fails where any file of its output is a link

### Requirement: Each tool's Claude Code plugins are of its tool's pin

The marketplace's `flume` plugin SHALL be the `plugins/claude` of the tree the set's Flume is built from, its `sluice` and `sluice-board` the `plugins/sluice` and `plugins/sluice-board` of the tree the set's sluice is built from, its `worktrunk` the `plugins/worktrunk` of the tree at the tag `flake.nix` names for worktrunk, and its `crew` the `plugin` folder of the set's crew package, which crew's flake builds, each copied whole.

#### Scenario: Each tool's plugins from its tool's pin

- **WHEN** the flake's packages are evaluated
- **THEN** the `flume` plugin is copied whole from `plugins/claude` of the flake's `flume` input, `sluice` and `sluice-board` from `plugins/sluice` and `plugins/sluice-board` of its `sluice` input, `worktrunk` from `plugins/worktrunk` of its `worktrunk` input, and `crew` from the set's `crew` package's `plugin`

### Requirement: Each tool's Claude Code plugins say its tool's version

The manifest of each of the marketplace's `flume`, `sluice`, `sluice-board`, `crew` and `worktrunk` plugins SHALL say the version of the set's tool it belongs to, or, for `worktrunk`, whose manifest names none, no version, so Claude Code reads the marketplace's. The flake's test and the marketplace's build SHALL each fail where one says another, naming the plugin and both versions.

#### Scenario: Each tool's plugins say its tool's version

- **WHEN** the flake is evaluated
- **THEN** each system's `flume`, `sluice`, `sluice-board` and `crew` plugin manifests say the version of that system's tool, and `worktrunk`'s names none

#### Scenario: A plugin other than its tool

- **WHEN** sluice's `plugins/sluice-board` says `0.94.7` and the set's sluice is `0.94.8`
- **THEN** the flake's test fails, naming `sluice-board`, `0.94.7` and `0.94.8`, and the marketplace's build fails naming its plugin and both versions

#### Scenario: worktrunk's plugin, which names no version

- **WHEN** worktrunk's manifest names no version and the set's worktrunk is `0.80.0`
- **THEN** the marketplace lists `worktrunk` at `0.80.0` and neither the flake's test nor the build fails; and where the manifest says `0.79.0`, the flake's test fails, naming `worktrunk`, `0.79.0` and `0.80.0`
