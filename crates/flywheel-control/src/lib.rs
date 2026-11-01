//! `flywheel-control`, the one interface through which the runner reaches
//! its control plane (design Decisions 1 and 2; core.2): a pull, which
//! gives what the control plane wants of the host, and a report, which
//! takes the runner's whole report. Ids are the control plane's, and
//! nothing here assumes git, files or a socket. It depends on no
//! implementation of it, and holds a fake control plane and the cases
//! every implementation passes.

pub mod fake;
pub mod report;
pub mod shared;
pub mod want;

pub use report::{
    Acted, Answer, At, Carried, Carrying, Commanded, InboxCarried, InboxFile, Launched, Outcome,
    Report,
};
pub use want::{
    Act, ActSort, AgentSetting, Command, CommandSort, Hold, Id, Inbox, Input, Launch, Lease, Link,
    Offer, Terminal, Want,
};

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fmt;

/// This interface's version, which every hello names.
pub const INTERFACE_VERSION: u32 = 1;

/// The runner's control plane, through one of its implementations.
pub trait ControlPlane {
    /// What the control plane wants of the host now, handed the runner's
    /// hello and its whole report, or why it could not be read.
    fn pull(&mut self, hello: &Hello, report: &Report) -> Result<Pulled, Unread>;
    /// Takes the runner's whole report, which renews every lease it names.
    /// A second report of one report changes nothing.
    fn report(&mut self, report: &Report) -> Result<Reported, Unsent>;
}

/// What an implementation is handed when it is opened, to wake the runner
/// once its control plane pushes work: each call makes a control pass at
/// once, the runner's control timer then timed from that pass's start
/// (design Decision 5). It may be called from any thread. A runner that
/// has stopped is woken to nothing.
pub type Wake = Box<dyn Fn() + Send + Sync>;

/// What the runner tells the control plane of its host at every pull
/// (design Decision 6).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Hello {
    /// This interface's version, [`INTERFACE_VERSION`].
    pub interface_version: u32,
    /// `flywheel --version`'s text.
    pub runner_version: String,
    /// Each tool's version, by the tool's name.
    pub tool_versions: BTreeMap<String, ToolVersion>,
    /// The addresses, `owner/name`, the runner keeps with their clones
    /// made.
    pub repositories: Vec<String>,
    pub capacity: Capacity,
    /// Each launch's and each command's id with the lease number held.
    pub leases: BTreeMap<Id, u64>,
}

/// A tool's version as the runner read it.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ToolVersion {
    /// The first line of its `--version`, as it printed it.
    Printed(String),
    /// The tool could not be run, with the words saying why.
    Missing(String),
}

/// The host's capacity, as Flume's status gives it.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Capacity {
    Printed(HostPart),
    /// Flume's status could not be read, with the words saying why.
    Missing(String),
}

/// Fields of Flume's `status.host`, each as Flume printed it.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct HostPart {
    pub running: String,
    pub bound: String,
    pub room: String,
    pub streams: String,
}

/// What a pull gives.
#[derive(Debug)]
pub struct Pulled {
    /// What the control plane wants, in its order.
    pub wants: Vec<Want>,
    /// The terminals to open.
    pub terminals: Vec<Terminal>,
    /// The runner's report as the control plane last held it, or none.
    pub report: Option<Report>,
}

/// A pull that could not be read, with the words saying why.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Unread(pub String);

/// A report that could not be sent, with the words saying why.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Unsent(pub String);

impl fmt::Display for Unread {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl fmt::Display for Unsent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for Unread {}
impl std::error::Error for Unsent {}

/// The control plane's answer to a report.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Reported {
    /// The ids the control plane carries no more.
    pub let_go: Vec<Id>,
    /// Each id whose report was refused, its lease no longer the
    /// runner's, with the control plane's words.
    pub refused: BTreeMap<Id, String>,
}
