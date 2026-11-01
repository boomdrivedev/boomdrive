//! `flywheel`, the runner on each host: it keeps the clones and views of
//! the repositories its configuration names (design 1).

pub mod clone;
pub mod config;
pub mod flume;
pub mod git;
pub mod lacks;
pub mod pass;
pub mod run;
pub mod view;
