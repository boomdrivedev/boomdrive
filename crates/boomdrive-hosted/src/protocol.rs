//! The runner's protocol with a hosted control plane, one module per
//! version (core.42): each holds its version's JSON Schema files, kept
//! under `protocol/` in this crate, which every check of a message is
//! built from. A [`Version`] is one the runner's code can speak, and
//! [`spoken`] the one `protocol/spoken.json` marks spoken, which every
//! message the runner sends or reads is of (Boom:
//! `the-cloud-leads-the-runners-protocol`).

pub mod v1;
pub mod v2;
pub mod v3;
pub mod v4;
pub mod v5;
pub mod v6;
pub mod v7;

use crate::check::Checks;
use std::sync::OnceLock;

/// One version the runner's code can speak: its number, its subprotocol,
/// where its files' `$id`s are, its files, and its checks, built from its
/// files alone at their first use.
pub struct Version {
    /// Every message's `protocol_version`.
    pub number: u32,
    /// The subprotocol a socket's upgrade asks for, and the start of every
    /// context a key signs (the protocol's README, *The paths* and *The
    /// signatures*).
    pub subprotocol: &'static str,
    /// The address each file's `$id` is under. No check fetches it.
    pub address: &'static str,
    /// Each schema file by its name, with its text.
    pub files: &'static [(&'static str, &'static str)],
    pub(crate) checks: Checks,
}

impl std::fmt::Debug for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.subprotocol)
    }
}

/// Version 5, read and written with version 7's types.
pub static V5: Version = Version {
    number: v5::VERSION,
    subprotocol: v5::SUBPROTOCOL,
    address: v5::ADDRESS,
    files: v5::FILES,
    checks: Checks::new(),
};

/// Version 6, version 5 whole but for the control plane's
/// `newest_release`, read and written with version 7's types.
pub static V6: Version = Version {
    number: v6::VERSION,
    subprotocol: v6::SUBPROTOCOL,
    address: v6::ADDRESS,
    files: v6::FILES,
    checks: Checks::new(),
};

/// Version 7, version 6 whole but for the hello's
/// `repository_statuses`.
pub static V7: Version = Version {
    number: v7::VERSION,
    subprotocol: v7::SUBPROTOCOL,
    address: v7::ADDRESS,
    files: v7::FILES,
    checks: Checks::new(),
};

/// Each version the runner's code can speak, oldest first.
pub fn speakable() -> [&'static Version; 3] {
    [&V5, &V6, &V7]
}

/// `protocol/spoken.json`, as the crate holds it.
pub const SPOKEN_FILE: &str = include_str!("../protocol/spoken.json");

/// The version `protocol/spoken.json` marks spoken, the newest it marks
/// where it marks several: the one every upgrade offers, every message is
/// checked against and every message names (core.42). The crate's test
/// `spoken` holds the file to one version the code can speak.
pub fn spoken() -> &'static Version {
    static SPOKEN: OnceLock<&'static Version> = OnceLock::new();
    SPOKEN.get_or_init(|| {
        let file: serde_json::Value = serde_json::from_str(SPOKEN_FILE)
            .expect("protocol/spoken.json is JSON, as the crate's test `spoken` checks");
        let marked = file["versions"]
            .as_array()
            .into_iter()
            .flatten()
            .filter(|entry| entry["spoken"] == true)
            .filter_map(|entry| entry["version"].as_u64());
        let newest = marked
            .max()
            .expect("protocol/spoken.json marks a version spoken, as its test checks");
        speakable()
            .into_iter()
            .find(|version| u64::from(version.number) == newest)
            .expect(
                "the version spoken.json marks spoken is one the code speaks, as its test checks",
            )
    })
}
