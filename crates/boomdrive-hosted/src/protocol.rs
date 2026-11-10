//! The runner's protocol with a hosted control plane, one module per
//! version (core.42): each holds its version's JSON Schema files, kept
//! under `protocol/` in this crate, which every check of a message is
//! built from.

pub mod v1;
pub mod v2;
