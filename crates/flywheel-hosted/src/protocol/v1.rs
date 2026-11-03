//! Version 1 of the runner's protocol (design Decisions 2 to 12): its
//! JSON Schema files, draft 2020-12, under `protocol/v1/`. A check of a
//! message is built from [`FILES`] alone, each file under its `$id`, and
//! fetches nothing (core.42).

/// This version's number, which every message of it names as its
/// `protocol_version`.
pub const VERSION: u32 = 1;

/// The address each file's `$id` is under: its own among the repository's
/// raw files at `main`. No check fetches it.
pub const ADDRESS: &str = "https://raw.githubusercontent.com/agentplot/flywheel-runner/main/crates/flywheel-hosted/protocol/v1/";

/// Each schema file of version 1 by its name, with its text.
pub const FILES: &[(&str, &str)] = &[
    (
        "from-runner.schema.json",
        include_str!("../../protocol/v1/from-runner.schema.json"),
    ),
    (
        "to-runner.schema.json",
        include_str!("../../protocol/v1/to-runner.schema.json"),
    ),
    (
        "shared.schema.json",
        include_str!("../../protocol/v1/shared.schema.json"),
    ),
    (
        "hello.schema.json",
        include_str!("../../protocol/v1/hello.schema.json"),
    ),
    (
        "wants.schema.json",
        include_str!("../../protocol/v1/wants.schema.json"),
    ),
    (
        "report.schema.json",
        include_str!("../../protocol/v1/report.schema.json"),
    ),
    (
        "place-status.schema.json",
        include_str!("../../protocol/v1/place-status.schema.json"),
    ),
    (
        "submission-entry.schema.json",
        include_str!("../../protocol/v1/submission-entry.schema.json"),
    ),
    (
        "terminal.schema.json",
        include_str!("../../protocol/v1/terminal.schema.json"),
    ),
];
