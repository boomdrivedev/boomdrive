//! Version 1 of the runner's protocol (design Decisions 2 to 13 of
//! the-runner-dials-the-hosted-control-plane): its JSON Schema files,
//! draft 2020-12, under `protocol/v1/`, as every release from `v0.1.0` to
//! `v0.7.3` spoke it. The runner speaks version 2 alone, so this module
//! holds version 1's files and no type, and its files never change
//! (core.42; record 0037).

/// This version's number, which every message of it names as its
/// `protocol_version`.
pub const VERSION: u32 = 1;

/// This version's name: the subprotocol a socket's upgrade asks for, and
/// the start of every context a key signs (the protocol's README, *The
/// paths* and *The signatures*).
pub const SUBPROTOCOL: &str = "boomdrive.v1";

/// The address each file's `$id` is under: its own among the repository's
/// raw files at `main`. No check fetches it.
pub const ADDRESS: &str = "https://raw.githubusercontent.com/agentplot/boomdrive/main/crates/boomdrive-hosted/protocol/v1/";

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
        "enrollment.schema.json",
        include_str!("../../protocol/v1/enrollment.schema.json"),
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
