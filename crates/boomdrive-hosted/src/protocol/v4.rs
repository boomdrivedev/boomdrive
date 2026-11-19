//! Version 4 of the runner's protocol (record 0053): its JSON Schema
//! files, draft 2020-12, under `protocol/v4/`. Version 4 is version 3
//! whole but for a forge token's ask and its two answers, which go by one
//! request at a path of their own, `runner/forge-token`, each file's `$id`
//! under `protocol/v4/`, and every message naming `protocol_version` 4. A
//! check of a message is built from [`FILES`] alone, each file under its
//! `$id`, and fetches nothing (core.42).

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
