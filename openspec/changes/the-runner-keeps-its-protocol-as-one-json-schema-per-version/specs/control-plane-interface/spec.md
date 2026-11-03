## MODIFIED Requirements

### Requirement: The control plane is reached through one interface

The runner SHALL reach its control plane only through `flywheel-control`'s `ControlPlane` interface: a pull, which takes the runner's hello and report, and a report, which takes the runner's report. No code outside an implementation SHALL name crew, its records or a hosted control plane's protocol. The one list of implementations, `crates/flywheel/src/planes.rs`, SHALL name each by its word and its opener alone. The runner protocol's files and types SHALL be held in the hosted implementation's crate, `crates/flywheel-hosted`, alone, which SHALL depend on no crate of the tree's but `flywheel-control` (core.2, core.42).

#### Scenario: Only crew's crate names crew

- **WHEN** the sources of `crates/flywheel` and `crates/flywheel-control` are read
- **THEN** no file names `flywheel_crew` but `crates/flywheel/src/planes.rs`, and `flywheel-control` depends on no implementation

#### Scenario: Only the hosted implementation's crate holds the protocol

- **WHEN** the manifests and files of every crate but `crates/flywheel-hosted` are read
- **THEN** no file but `crates/flywheel/src/planes.rs` names `flywheel_hosted` or `flywheel-hosted`, no file's name ends `.schema.json`, no crate depends on `flywheel-hosted`, and `flywheel-hosted` depends on `flywheel-control` alone of the tree's crates
