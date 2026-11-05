//! `flywheel-hosted`, the hosted control plane's implementation of the
//! control-plane interface (design Decision 1; core.2), and the one crate
//! that holds the runner's protocol with it: one JSON Schema per version,
//! under `protocol/`, and the runner's types for its messages (core.42).
//! It holds the hosted table, the host's key and its stores, a
//! partition's enrolment, each partition's connection, the cut of each
//! tool's answer to its copy, the checks of version 1's messages, and
//! `Hosted`, the `ControlPlane` over every partition's connection, which
//! [`open`] opens.

pub mod check;
pub mod connection;
pub mod copy;
pub mod enrolment;
pub mod key;
pub mod plane;
pub mod protocol;
pub mod table;

pub use enrolment::{Enrolled, Refusal};
pub use plane::{open, Hosted};

use std::path::Path;
use table::Table;

/// Enrols the partition `label` of the hosted table, `table`, of the
/// configuration file `file`, with its one-time `token`, keeping its key
/// in the host's own credential store (design Decision 4; core.32).
pub fn enrol(
    table: &toml::Table,
    file: &Path,
    label: &str,
    token: &str,
) -> Result<Enrolled, Refusal> {
    let table = Table::read(table, file).map_err(Refusal::host)?;
    enrolment::enrol(&table, label, token, key::store_for_host, &machine())
}

/// The host's name, as its operating system gives it, which an
/// enrolment carries (the cloud's first run, FR3).
fn machine() -> String {
    gethostname::gethostname().to_string_lossy().into_owned()
}
