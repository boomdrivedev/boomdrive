//! `flywheel-crew`, crew's implementation of the control-plane interface
//! (design Decisions 1 and 17 to 19; core.2), and the one crate that names
//! crew, its branches or its records. It reads crew's records at one tip
//! of each partition's flywheel branch, `<label>/main`, and every runner's
//! branch beside it, and gives this host's requests and role runs as
//! wants under the lease number 1. It writes the runner's own branch,
//! `<label>/runner/<host>`, in crew's formats, and pushes no other.

pub mod config;
