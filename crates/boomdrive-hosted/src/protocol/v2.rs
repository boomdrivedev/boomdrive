//! Version 2 of the runner's protocol (record 0037): its JSON Schema
//! files, draft 2020-12, under `protocol/v2/`, which are version 1's but
//! for a place's ends, each with its delivery, and a control plane's
//! confirm. A check of a message is built from [`FILES`] alone, each file
//! under its `$id`, and fetches nothing (core.42).

/// This version's number, which every message of it names as its
/// `protocol_version`.
pub const VERSION: u32 = 2;

/// This version's name: the subprotocol a socket's upgrade asks for, and
/// the start of every context a key signs (the protocol's README, *The
/// paths* and *The signatures*).
pub const SUBPROTOCOL: &str = "boomdrive.v2";

/// The address each file's `$id` is under: its own among the repository's
/// raw files at `main`. No check fetches it.
pub const ADDRESS: &str = "https://raw.githubusercontent.com/agentplot/boomdrive/main/crates/boomdrive-hosted/protocol/v2/";

/// Each schema file of version 2 by its name, with its text.
pub const FILES: &[(&str, &str)] = &[
    (
        "from-runner.schema.json",
        include_str!("../../protocol/v2/from-runner.schema.json"),
    ),
    (
        "to-runner.schema.json",
        include_str!("../../protocol/v2/to-runner.schema.json"),
    ),
    (
        "shared.schema.json",
        include_str!("../../protocol/v2/shared.schema.json"),
    ),
    (
        "enrollment.schema.json",
        include_str!("../../protocol/v2/enrollment.schema.json"),
    ),
    (
        "hello.schema.json",
        include_str!("../../protocol/v2/hello.schema.json"),
    ),
    (
        "wants.schema.json",
        include_str!("../../protocol/v2/wants.schema.json"),
    ),
    (
        "report.schema.json",
        include_str!("../../protocol/v2/report.schema.json"),
    ),
    (
        "place-status.schema.json",
        include_str!("../../protocol/v2/place-status.schema.json"),
    ),
    (
        "submission-entry.schema.json",
        include_str!("../../protocol/v2/submission-entry.schema.json"),
    ),
    (
        "terminal.schema.json",
        include_str!("../../protocol/v2/terminal.schema.json"),
    ),
];
