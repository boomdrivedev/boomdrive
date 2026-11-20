//! Version 3 of the runner's protocol (record 0052): its JSON Schema
//! files, draft 2020-12, under `protocol/v3/`, as every release from
//! `v0.18.0` to the last before version 4 spoke it. Version 3 is version 4
//! but for its subprotocol, its contexts, its files' `$id`s and a forge
//! token's ask and answers, which it does not have. The runner speaks
//! version 4 alone, so this module holds version 3's files and no type,
//! and its files never change (core.42; record 0053).

/// This version's number, which every message of it names as its
/// `protocol_version`.
pub const VERSION: u32 = 3;

/// This version's name: the subprotocol a socket's upgrade asks for, and
/// the start of every context a key signs (the protocol's README, *The
/// paths* and *The signatures*).
pub const SUBPROTOCOL: &str = "boomdrive.v3";

/// The address each file's `$id` is under: its own among the repository's
/// raw files at `main`. No check fetches it.
pub const ADDRESS: &str = "https://raw.githubusercontent.com/boomdrivedev/boomdrive/main/crates/boomdrive-hosted/protocol/v3/";

/// Each schema file of version 3 by its name, with its text.
pub const FILES: &[(&str, &str)] = &[
    (
        "from-runner.schema.json",
        include_str!("../../protocol/v3/from-runner.schema.json"),
    ),
    (
        "to-runner.schema.json",
        include_str!("../../protocol/v3/to-runner.schema.json"),
    ),
    (
        "shared.schema.json",
        include_str!("../../protocol/v3/shared.schema.json"),
    ),
    (
        "enrollment.schema.json",
        include_str!("../../protocol/v3/enrollment.schema.json"),
    ),
    (
        "hello.schema.json",
        include_str!("../../protocol/v3/hello.schema.json"),
    ),
    (
        "wants.schema.json",
        include_str!("../../protocol/v3/wants.schema.json"),
    ),
    (
        "report.schema.json",
        include_str!("../../protocol/v3/report.schema.json"),
    ),
    (
        "place-status.schema.json",
        include_str!("../../protocol/v3/place-status.schema.json"),
    ),
    (
        "submission-entry.schema.json",
        include_str!("../../protocol/v3/submission-entry.schema.json"),
    ),
    (
        "preview.schema.json",
        include_str!("../../protocol/v3/preview.schema.json"),
    ),
    (
        "terminal.schema.json",
        include_str!("../../protocol/v3/terminal.schema.json"),
    ),
];
