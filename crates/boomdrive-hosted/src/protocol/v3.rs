//! Version 3 of the runner's protocol (record 0052): its JSON Schema
//! files, draft 2020-12, under `protocol/v3/`. Version 3 is version 2
//! whole but for a control plane's hold among the acts, each file's `$id`
//! under the repository's own address, and every message naming
//! `protocol_version` 3. A check of a message is built from [`FILES`]
//! alone, each file under its `$id`, and fetches nothing (core.42).

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
