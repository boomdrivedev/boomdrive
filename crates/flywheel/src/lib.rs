//! `flywheel`, the runner on each host: it keeps the clones and views of
//! the repositories its configuration names (design 1), carries its
//! control plane's work into Flume (design Decision 5), and enrols each
//! partition of its control plane once (`flywheel enroll`).

pub mod carry;
pub mod clone;
pub mod config;
pub mod enroll;
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
