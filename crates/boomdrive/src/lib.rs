//! `boomdrive`, the runner on each host: it keeps the clones and views of
//! the repositories its configuration names (design 1), carries its
//! control plane's work into Flume (design Decision 5), and enrolls each
//! partition of its control plane once (`boomdrive enroll`).

pub mod carry;
pub mod clone;
pub mod code_workspace;
pub mod config;
pub mod enroll;
pub mod facts;
pub mod flume;
pub mod forge;
pub mod forge_token;
pub mod git;
pub mod hello;
pub mod inbox;
pub mod lacks;
pub mod locks;
pub mod pass;
pub mod planes;
pub mod report;
pub mod run;
pub mod settle;
pub mod sluice;
pub mod terminal;
pub mod view;
