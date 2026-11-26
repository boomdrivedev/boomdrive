//! Version 5 of the runner's protocol (record 0054): its JSON Schema
//! files, draft 2020-12, under `protocol/v5/`, as every release from
//! `v0.27.0` speaks it. Version 5 is version 6 but for its subprotocol,
//! its contexts, its files' `$id`s and the control plane's
//! `newest_release`, which it does not have. Its messages are read with
//! version 6's types, since each message of version 5 is one of version
//! 6's but for its `protocol_version`, so this module holds version 5's
//! files and no type, and its files never change (core.42).

/// This version's number, which every message of it names as its
/// `protocol_version`.
pub const VERSION: u32 = 5;

/// This version's name: the subprotocol a socket's upgrade asks for, and
/// the start of every context a key signs (the protocol's README, *The
/// paths* and *The signatures*).
pub const SUBPROTOCOL: &str = "boomdrive.v5";

/// The address each file's `$id` is under: its own among the repository's
/// raw files at `main`. No check fetches it.
pub const ADDRESS: &str = "https://raw.githubusercontent.com/boomdrivedev/boomdrive/main/crates/boomdrive-hosted/protocol/v5/";

/// Each schema file of version 5 by its name, with its text.
pub const FILES: &[(&str, &str)] = &[
    (
        "from-runner.schema.json",
        include_str!("../../protocol/v5/from-runner.schema.json"),
    ),
    (
        "to-runner.schema.json",
        include_str!("../../protocol/v5/to-runner.schema.json"),
    ),
    (
        "shared.schema.json",
        include_str!("../../protocol/v5/shared.schema.json"),
    ),
    (
        "enrollment.schema.json",
        include_str!("../../protocol/v5/enrollment.schema.json"),
    ),
    (
        "hello.schema.json",
        include_str!("../../protocol/v5/hello.schema.json"),
    ),
    (
        "wants.schema.json",
        include_str!("../../protocol/v5/wants.schema.json"),
    ),
    (
        "report.schema.json",
        include_str!("../../protocol/v5/report.schema.json"),
    ),
    (
        "place-status.schema.json",
        include_str!("../../protocol/v5/place-status.schema.json"),
    ),
    (
        "submission-entry.schema.json",
        include_str!("../../protocol/v5/submission-entry.schema.json"),
    ),
    (
        "preview.schema.json",
        include_str!("../../protocol/v5/preview.schema.json"),
    ),
    (
        "terminal.schema.json",
        include_str!("../../protocol/v5/terminal.schema.json"),
    ),
    (
        "forge-token.schema.json",
        include_str!("../../protocol/v5/forge-token.schema.json"),
    ),
];
