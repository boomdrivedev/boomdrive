//! `boomdrive`, the runner on each host: it keeps the clones and views of
//! the repositories its configuration names (design 1), and carries its
//! control plane's work into Flume (design Decision 5).

/// The runner's name before it was renamed `boomdrive` (record 0016): the
/// name of the data directory, of the clone's records and of the mark a
/// runner of that name kept on a host, which this runner carries over or
/// still reads (core.36).
pub const BEFORE_RENAME: &str = "flywheel";

pub mod carry;
pub mod clone;
pub mod code_workspace;
pub mod config;
pub mod facts;
pub mod flume;
pub mod git;
pub mod hello;
pub mod inbox;
pub mod lacks;
pub mod pass;
pub mod planes;
pub mod report;
pub mod run;
pub mod sluice;
pub mod terminal;
pub mod view;
