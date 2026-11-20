//! Version 2 of the runner's protocol (record 0037 as amended on
//! 2026-11-09): its JSON Schema files, draft 2020-12, under
//! `protocol/v2/`, as every release from `v0.8.0` to the last before
//! version 3 spoke it. Version 2 is version 3 but for its subprotocol, its
//! contexts, its files' `$id`s and a control plane's hold, which it does
//! not have. The runner speaks version 4 alone, so this module holds
//! version 2's files and no type, and its files never change (core.42;
//! record 0052).

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
        "preview.schema.json",
        include_str!("../../protocol/v2/preview.schema.json"),
    ),
    (
        "terminal.schema.json",
        include_str!("../../protocol/v2/terminal.schema.json"),
    ),
];
