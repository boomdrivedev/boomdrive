//! `boomdrive`, the runner on each host: it keeps the clones and views of
//! the repositories its configuration names (design 1), carries its
//! control plane's work into Flume (design Decision 5), enrolls each
//! partition of its control plane once (`boomdrive enroll`), and sets up
//! a person's drive on a Mac Nix does not manage (`boomdrive setup`).

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
/// `boomdrive setup`, a person's drive set up by one command (record 0009
/// as amended 2026-11-09 and 2026-11-11).
pub mod setup;
pub mod sluice;
pub mod terminal;
pub mod view;
