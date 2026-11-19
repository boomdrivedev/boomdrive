//! `boomdrive-hosted`, the hosted control plane's implementation of the
//! control-plane interface (design Decision 1; core.2), and the one crate
//! that holds the runner's protocol with it: one JSON Schema per version,
//! under `protocol/`, and the runner's types for its messages (core.42).
//! It holds the hosted table, the host's key and its stores, a
//! partition's enrollment, each partition's connection, the cut of each
//! tool's answer to its copy, the checks of version 4's messages, and
//! `Hosted`, the `ControlPlane` over a partition's connection, which
//! [`open`] opens as a part for each partition; the observers of
//! `[observer]`, which [`observer::open`] opens, one for each partition
//! (record 0048); and the control plane's forge tokens, which
//! [`forge_tokens`] opens (record 0053).

pub mod check;
pub mod connection;
pub mod copy;
pub mod enrollment;
pub mod forge_token;
pub mod key;
pub mod observer;
pub mod plane;
pub mod protocol;
pub mod table;

pub use enrollment::{Enrolled, Refusal};
pub use plane::{open, Hosted};

use std::path::Path;
use table::Table;

/// The words a cloud host's `boomdrive forge-token` refuses every
/// operation with, until a version of the protocol carries a forge token:
/// under the hosted control plane the Boomdrive App is the control plane's,
/// which mints the host's tokens and sends them down the runner's own
/// connection (record 0007 as amended on 2026-11-09; core.33). They name
/// [`protocol::v3::VERSION`] as text, since a `const` is made of no
/// `format!`: a later version changes the number here with its own, and a
/// version that carries tokens changes these words with it.
pub const NO_FORGE_TOKEN: &str =
    "the hosted control plane mints this host's forge tokens, and version 3 of its protocol carries none yet";

/// The hosted control plane's forge tokens, from the hosted table,
/// `table`, of the configuration file `file`, each partition's key read
/// from the host's own credential store at each ask; a store that cannot
/// be opened is said by each ask, which then asks nothing (design
/// Decision 7 of a-cloud-host-gets-its-github-tokens-from-the-cloud;
/// core.32, core.33). It opens no socket and no part.
pub fn forge_tokens(
    table: &toml::Table,
    file: &Path,
) -> Result<Box<dyn boomdrive_control::ForgeTokens>, String> {
    let table = Table::read(table, file)?;
    Ok(Box::new(forge_token::Tokens::new(
        table,
        key::store_for_host(),
    )))
}

/// Enrolls the partition `label` of the hosted table, `table`, of the
/// configuration file `file`, with the one-time token `token` gives once
/// the partition and the store are found, keeping its key in the host's
/// own credential store (design Decision 4; core.32).
pub fn enroll(
    table: &toml::Table,
    file: &Path,
    label: &str,
    token: impl FnOnce() -> Result<String, String>,
) -> Result<Enrolled, Refusal> {
    let table = Table::read(table, file).map_err(Refusal::host)?;
    enrollment::enroll(&table, label, token, key::store_for_host, &machine())
}

/// Enrolls the partition `label` of the observer table, `[observer]`,
/// `table`, of the configuration file `file`, as [`enroll`] enrolls a
/// hosted partition, at the observer's `runner/enroll`, its key kept in
/// the host's own credential store under `observer.<label>` (design
/// Decision 4 of a-runner-reports-to-an-observer-it-takes-no-work-from;
/// record 0048; core.31, core.32).
pub fn enroll_observer(
    table: &toml::Table,
    file: &Path,
    label: &str,
    token: impl FnOnce() -> Result<String, String>,
) -> Result<Enrolled, Refusal> {
    let table = Table::read_observer(table, file).map_err(Refusal::host)?;
    enrollment::enroll(&table, label, token, key::store_for_host, &machine())
}

/// The host's name, as its operating system gives it, which an
/// enrollment carries (the cloud's first run, FR3).
fn machine() -> String {
    gethostname::gethostname().to_string_lossy().into_owned()
}
