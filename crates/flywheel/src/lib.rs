//! `flywheel`, the runner on each host: it keeps the clones and views of
//! the repositories its configuration names (design 1), and carries its
//! control plane's work into Flume (design Decision 5).

pub mod carry;
pub mod clone;
pub mod config;
pub mod facts;
pub mod flume;
pub mod git;
pub mod hello;
pub mod lacks;
pub mod pass;
pub mod report;
pub mod run;
pub mod view;
