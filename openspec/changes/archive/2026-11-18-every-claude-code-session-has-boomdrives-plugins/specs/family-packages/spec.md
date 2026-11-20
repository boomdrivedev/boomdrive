## MODIFIED Requirements

### Requirement: The set's marketplace lists every Claude Code plugin a tool of the set ships

`boomdrive-claude-plugins` SHALL be one folder, a Claude Code marketplace named `boomdrive` whose `.claude-plugin/marketplace.json` lists eight plugins, each at `./plugins/<name>` and at the version of the set's tool it belongs to: `pendingyou`, `plannotator`, `timber`, `flume`, `sluice` and `sluice-board`, both sluice's, `crew` and `worktrunk`. No file of the folder SHALL be a link.

#### Scenario: The marketplace's plugins

- **WHEN** the flake's packages are evaluated
- **THEN** `boomdrive-claude-plugins` gives the marketplace `boomdrive` with `pendingyou`, `plannotator`, `timber`, `flume`, `sluice`, `sluice-board`, `crew` and `worktrunk`, each at the version of the set's tool it belongs to, `flume` at the set's Flume's `0.31.0`

#### Scenario: No link into the store

- **WHEN** the marketplace's build is read
- **THEN** it copies every file with links followed, and fails where any file of its output is a link
