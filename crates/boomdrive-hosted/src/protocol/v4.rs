//! Version 4 of the runner's protocol (record 0053): its JSON Schema
//! files, draft 2020-12, under `protocol/v4/`, as every release from
//! `v0.21.0` to the last before version 5 spoke it. Version 4 is version 5
//! but for its subprotocol, its contexts, its files' `$id`s, an observing
//! hello and an observed report, which version 5 does not have, and a
//! report's unleased places, which it does not have. The runner speaks
//! version 5 alone, so this module holds version 4's files and no type,
//! and its files never change (core.42; record 0054).

/// This version's number, which every message of it names as its
/// `protocol_version`.
pub const VERSION: u32 = 4;

/// This version's name: the subprotocol a socket's upgrade asks for, and
/// the start of every context a key signs (the protocol's README, *The
/// paths* and *The signatures*).
pub const SUBPROTOCOL: &str = "boomdrive.v4";

/// The address each file's `$id` is under: its own among the repository's
/// raw files at `main`. No check fetches it.
pub const ADDRESS: &str = "https://raw.githubusercontent.com/boomdrivedev/boomdrive/main/crates/boomdrive-hosted/protocol/v4/";

/// Each schema file of version 4 by its name, with its text.
pub const FILES: &[(&str, &str)] = &[
    (
        "from-runner.schema.json",
        include_str!("../../protocol/v4/from-runner.schema.json"),
    ),
    (
        "to-runner.schema.json",
        include_str!("../../protocol/v4/to-runner.schema.json"),
    ),
    (
        "shared.schema.json",
        include_str!("../../protocol/v4/shared.schema.json"),
    ),
    (
        "enrollment.schema.json",
        include_str!("../../protocol/v4/enrollment.schema.json"),
    ),
    (
        "hello.schema.json",
        include_str!("../../protocol/v4/hello.schema.json"),
    ),
    (
        "wants.schema.json",
        include_str!("../../protocol/v4/wants.schema.json"),
    ),
    (
        "report.schema.json",
        include_str!("../../protocol/v4/report.schema.json"),
    ),
    (
        "place-status.schema.json",
        include_str!("../../protocol/v4/place-status.schema.json"),
    ),
    (
        "submission-entry.schema.json",
        include_str!("../../protocol/v4/submission-entry.schema.json"),
    ),
    (
        "preview.schema.json",
        include_str!("../../protocol/v4/preview.schema.json"),
    ),
    (
        "terminal.schema.json",
        include_str!("../../protocol/v4/terminal.schema.json"),
    ),
    (
        "forge-token.schema.json",
        include_str!("../../protocol/v4/forge-token.schema.json"),
    ),
];
