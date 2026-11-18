## REMOVED Requirements

### Requirement: The set's marketplace lists Pending You's, Plannotator's and timber's plugins

**Reason**: The marketplace lists every Claude Code plugin a tool of the set ships, Flume's and sluice's beside the companions' and timber's (record 0044 as amended on 2026-11-18).

**Migration**: "The set's marketplace lists every Claude Code plugin a tool of the set ships" holds the six plugins and the folder's no link.

## ADDED Requirements

### Requirement: The set's marketplace lists every Claude Code plugin a tool of the set ships

`boomdrive-claude-plugins` SHALL be one folder, a Claude Code marketplace named `boomdrive` whose `.claude-plugin/marketplace.json` lists six plugins, each at `./plugins/<name>` and at the version of the set's tool it belongs to: `flume` at the set's Flume's, `sluice` and `sluice-board` at the set's sluice's, `timber` at the set's timber's, `pendingyou` at the set's Pending You's and `plannotator` at the set's Plannotator's. No file of the folder SHALL be a link.

#### Scenario: The marketplace's six plugins

- **WHEN** the flake's packages are evaluated
- **THEN** `boomdrive-claude-plugins` gives the marketplace `boomdrive` with `flume`, `sluice`, `sluice-board`, `timber`, `pendingyou` and `plannotator`, each at the version of the set's tool it belongs to

#### Scenario: No link into the store

- **WHEN** the marketplace's build is read
- **THEN** it copies every file with links followed, and fails where any file of its output is a link

### Requirement: Flume's and sluice's Claude Code plugins are their tools' sources', at the set's versions

The marketplace's `flume` plugin SHALL be the `plugins/claude` folder of the tree the set's Flume is built from, and its `sluice` and `sluice-board` plugins the `plugins/sluice` and `plugins/sluice-board` folders of the tree the set's sluice is built from, each copied whole. The build SHALL fail where a plugin's `.claude-plugin/plugin.json` says another version than its tool's, naming the plugin and both versions.

#### Scenario: Flume's and sluice's plugins from their tools' sources

- **WHEN** the flake's packages are evaluated
- **THEN** the `flume` plugin is copied whole from `plugins/claude` of the flake's `flume` input, and `sluice` and `sluice-board` from `plugins/sluice` and `plugins/sluice-board` of its `sluice` input

#### Scenario: Flume's and sluice's plugins say their tools' versions

- **WHEN** the flake is evaluated
- **THEN** each system's marketplace's `flume` plugin manifest says the version of that system's `flume`, and its `sluice` and `sluice-board` manifests the version of its `sluice`

#### Scenario: A plugin other than its tool

- **WHEN** sluice's `plugins/sluice-board` says `0.94.7` and the set's sluice is `0.94.8`
- **THEN** the marketplace's build fails, naming `sluice-board`, `0.94.7` and `0.94.8`
