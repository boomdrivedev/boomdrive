//! `boomdrive-hosted`, the hosted control plane's implementation of the
//! control-plane interface (design Decision 1; core.2), and the one crate
//! that holds the runner's protocol with it: one JSON Schema per version,
//! under `protocol/`, and the runner's types for its messages (core.42).
//! It holds the hosted table, the host's key and its stores, a
//! partition's enrollment, each partition's connection, the cut of each
//! tool's answer to its copy, the checks of version 4's messages, and
//! `Hosted`, the `ControlPlane` over a partition's connection, which
//! [`open`] opens as a part for each partition; and the control plane's
//! forge tokens, which [`forge_tokens`] opens (record 0053).

pub mod check;
pub mod connection;
pub mod copy;
pub mod enrollment;
pub mod forge_token;
pub mod key;
pub mod plane;
pub mod protocol;
pub mod table;

pub use enrollment::{Enrolled, Refusal};
pub use plane::{open, Hosted};

use std::path::Path;
use table::Table;

/// The hosted control plane's forge tokens, from the hosted table,
/// `table`, of the configuration file `file`, each partition's key read
/// from the host's own credential store, opened at each token asked; a
/// store that cannot be opened is said by each ask, which then asks
/// nothing (design Decision 7 of
/// a-cloud-host-gets-its-github-tokens-from-the-cloud; core.32, core.33).
/// It reads the table alone, and opens no store, socket or part.
pub fn forge_tokens(
    table: &toml::Table,
    file: &Path,
) -> Result<Box<dyn boomdrive_control::ForgeTokens>, String> {
    let table = Table::read(table, file)?;
    Ok(Box::new(forge_token::Tokens::new(
        table,
        Box::new(key::store_for_host),
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

/// The host's name, as its operating system gives it, which an
/// enrollment carries (the cloud's first run, FR3).
fn machine() -> String {
    gethostname::gethostname().to_string_lossy().into_owned()
}
