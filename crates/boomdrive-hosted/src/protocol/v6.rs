//! Version 6 of the runner's protocol: its JSON Schema files, draft
//! 2020-12, under `protocol/v6/`, as every release from `v0.33.0` speaks
//! it. Version 6 is version 7 but for its
//! subprotocol, its contexts, its files' `$id`s and the hello's
//! `repository_statuses`, which it does not have. Its messages are read
//! with version 7's types, since each message of version 6 is one of
//! version 7's but for its `protocol_version` and a hello with no
//! `repository_statuses`, so this module holds version 6's files and no
//! type, and its files never change (core.42).

/// This version's number, which every message of it names as its
/// `protocol_version`.
pub const VERSION: u32 = 6;

/// This version's name: the subprotocol a socket's upgrade asks for, and
/// the start of every context a key signs (the protocol's README, *The
/// paths* and *The signatures*).
pub const SUBPROTOCOL: &str = "boomdrive.v6";

/// The address each file's `$id` is under: its own among the repository's
/// raw files at `main`. No check fetches it.
pub const ADDRESS: &str = "https://raw.githubusercontent.com/boomdrivedev/boomdrive/main/crates/boomdrive-hosted/protocol/v6/";

/// Each schema file of version 6 by its name, with its text.
pub const FILES: &[(&str, &str)] = &[
    (
        "from-runner.schema.json",
        include_str!("../../protocol/v6/from-runner.schema.json"),
    ),
    (
        "to-runner.schema.json",
        include_str!("../../protocol/v6/to-runner.schema.json"),
    ),
    (
        "shared.schema.json",
        include_str!("../../protocol/v6/shared.schema.json"),
    ),
    (
        "enrollment.schema.json",
        include_str!("../../protocol/v6/enrollment.schema.json"),
    ),
    (
        "hello.schema.json",
        include_str!("../../protocol/v6/hello.schema.json"),
    ),
    (
        "wants.schema.json",
        include_str!("../../protocol/v6/wants.schema.json"),
    ),
    (
        "report.schema.json",
        include_str!("../../protocol/v6/report.schema.json"),
    ),
    (
        "place-status.schema.json",
        include_str!("../../protocol/v6/place-status.schema.json"),
    ),
    (
        "submission-entry.schema.json",
        include_str!("../../protocol/v6/submission-entry.schema.json"),
    ),
    (
        "preview.schema.json",
        include_str!("../../protocol/v6/preview.schema.json"),
    ),
    (
        "terminal.schema.json",
        include_str!("../../protocol/v6/terminal.schema.json"),
    ),
    (
        "forge-token.schema.json",
        include_str!("../../protocol/v6/forge-token.schema.json"),
    ),
];
