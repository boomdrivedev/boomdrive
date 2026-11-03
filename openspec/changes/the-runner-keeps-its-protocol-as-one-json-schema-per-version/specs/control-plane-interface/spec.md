# Control-plane interface

## ADDED Requirements

### Requirement: The protocol is held in the hosted implementation's crate alone

The runner protocol's files and types SHALL be held in the hosted implementation's crate, `crates/flywheel-hosted`, alone. It SHALL depend on no crate of the tree's but `flywheel-control`, and no crate SHALL depend on it (core.2, core.42).

#### Scenario: Only the hosted implementation's crate holds the protocol

- **WHEN** the manifests and files of every crate but `crates/flywheel-hosted` are read
- **THEN** no file names `flywheel_hosted` or `flywheel-hosted` but `crates/flywheel/src/planes.rs` and the two that hold the crate to this rule, `crates/flywheel/tests/rules.rs` and `crates/flywheel/moon.yml`, which names it only as its test task's input, no file's name ends `.schema.json`, no crate depends on `flywheel-hosted`, and `flywheel-hosted` depends on `flywheel-control` alone of the tree's crates
