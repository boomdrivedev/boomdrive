//! `boomdrive-control`, the one interface through which the runner reaches
//! its control plane (design Decisions 1 and 2; core.2): a pull, which
//! gives what the control plane wants of the host, a report, which takes
//! the runner's report, and `owns`, which says whose an id is. An
//! implementation serves its control plane as parts, each a [`Part`]
//! the runner carries on its own (record 0026; core.49). Ids are the
//! control plane's, and nothing here assumes git, files or a socket. It
//! depends on no implementation of it, and holds a fake control plane and
//! the cases every implementation passes. An implementation whose control
//! plane holds the forge's App also gives that App's tokens, as
//! [`ForgeTokens`] (record 0053; core.33).

pub mod fake;
pub mod report;
pub mod shared;
pub mod want;

pub use report::{
    Acted, Answer, At, Carried, Carrying, Commanded, EntryRefused, InboxCarried, InboxFile,
    Launched, Outcome, Previewed, Report, Unleased,
};
pub use want::{
    Act, ActSort, AgentSetting, Command, CommandSort, Hold, Id, Inbox, Input, Launch, Lease, Link,
    Offer, Preview, Terminal, Want,
};

use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use time::format_description::well_known::Rfc3339;
use time::OffsetDateTime;

/// This interface's version, which every hello names.
pub const INTERFACE_VERSION: u32 = 1;

/// The runner's control plane, through one of its implementations.
pub trait ControlPlane {
    /// What the control plane wants of the host now, handed the runner's
    /// hello and its report, or why it could not be read.
    fn pull(&mut self, hello: &Hello, report: &Report) -> Result<Pulled, Unread>;
    /// Takes the runner's report, which renews every lease it names. A
    /// second report of one report changes nothing.
    fn report(&mut self, report: &Report) -> Result<Reported, Unsent>;
    /// Whether `id` is one this control plane gives: every want it gives
    /// carries an id it owns. The runner reads no id, and keeps each
    /// part's share of its report by asking the part (design Decision 1).
    fn owns(&self, id: &Id) -> bool;
    /// Whether the readers of this control plane's places reach the host
    /// only through the runner, so no session and no answerer on the host
    /// stands for it: a hosted control plane's members do, and crew's
    /// roles, which run on the host, do not. A launch of its that names
    /// no recipient then names the runner alone, so Flume takes the
    /// launch and holds each end of the place for the runner (core.54;
    /// record 0030 as amended on 2026-11-26).
    fn read_through_the_runner(&self) -> bool;
}

/// A part of the control plane, which the runner carries on its own: its
/// own pulls, answers and reports, on its own timer (record 0026;
/// core.49).
pub struct Part {
    /// The implementation's name for it, given once among its parts.
    pub label: String,
    pub plane: Box<dyn ControlPlane + Send>,
}

/// What gives a served repository's forge token where the control plane
/// holds the App, as the hosted control plane does: no part and nothing it
/// opens, but what an implementation gives for that App's tokens, which
/// `boomdrive forge-token` and the running runner each ask (design
/// Decision 6 of a-cloud-host-gets-its-github-tokens-from-the-cloud;
/// record 0053; core.2, core.33). It names no path, kind or version of any
/// protocol.
pub trait ForgeTokens: Send + Sync {
    /// The token of `repository`, by its address, `owner/name`, asked
    /// afresh: nothing here keeps one.
    fn token(&self, repository: &str) -> TokenAsked;
}

/// A forge token for one repository, and its end. Its `Debug` names its
/// end and never its secret (core.4).
#[derive(Clone, PartialEq, Eq)]
pub struct ForgeToken {
    secret: String,
    ends: OffsetDateTime,
}

impl ForgeToken {
    /// The token `secret`, ending at `ends`.
    pub fn new(secret: impl Into<String>, ends: OffsetDateTime) -> ForgeToken {
        ForgeToken {
            secret: secret.into(),
            ends,
        }
    }

    /// The token's secret, which only a credential's answer is given.
    pub fn secret(&self) -> &str {
        &self.secret
    }

    /// When the token ends.
    pub fn ends(&self) -> OffsetDateTime {
        self.ends
    }
}

impl fmt::Debug for ForgeToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ends = self
            .ends
            .format(&Rfc3339)
            .unwrap_or_else(|_| self.ends.unix_timestamp().to_string());
        f.debug_struct("ForgeToken")
            .field("ends", &ends)
            .finish_non_exhaustive()
    }
}

/// What an ask of a repository's token gives: the token; the control
/// plane's word that the App's installation does not cover the
/// repository, with words; or a refusal, with words, and whether the host
/// must be fixed before an ask can go (core.39's exit 2), as where no key
/// of a partition served it is in the host's store. No words hold a token,
/// a statement or a key (core.4, core.11).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TokenAsked {
    Token(ForgeToken),
    NotInstalled(String),
    Refused { words: String, host_to_fix: bool },
}

/// What an implementation is handed for each part it opens, to wake the
/// runner for that part once its control plane pushes the part's work:
/// each call makes a control pass of that part at once, the part's timer
/// then timed from that pass's start. It may be called from any thread. A
/// runner that has stopped is woken to nothing.
pub type Wake = Box<dyn Fn() + Send + Sync>;

/// What the runner tells the control plane of its host at every pull
/// (design Decision 6).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Hello {
    /// This interface's version, [`INTERFACE_VERSION`].
    pub interface_version: u32,
    /// `boomdrive --version`'s text.
    pub runner_version: String,
    /// Each tool's version, by the tool's name.
    pub tool_versions: BTreeMap<String, ToolVersion>,
    /// The addresses, `owner/name`, the runner keeps with their clones
    /// made.
    pub repositories: Vec<String>,
    /// Apart from `repositories`, the addresses whose clones the runner
    /// keeps while its configuration no longer names them, because a
    /// place it launched there has not ended; no launch is taken in one.
    /// Read as none where a hello lacks it.
    #[serde(default)]
    pub draining: Vec<String>,
    /// Each drive of the configuration, by its name, with each repository
    /// it names whose clone is made, in the configuration's order. A drive
    /// none of whose clones is made is named with none. Read as none where
    /// a hello lacks it.
    #[serde(default)]
    pub drives: BTreeMap<String, Vec<KeptRepository>>,
    /// Flume's status of each repository of `repositories`, by its
    /// address, as Flume's status gives it for the repository's clone.
    /// Read as none where a hello lacks it.
    #[serde(default)]
    pub repository_statuses: BTreeMap<String, RepositoryStatus>,
    pub capacity: Capacity,
    /// Each launch's, each command's and each preview's id with the lease
    /// number held.
    pub leases: BTreeMap<Id, u64>,
}

/// A repository a drive keeps, its clone made.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct KeptRepository {
    /// Its address, `owner/name`, as the configuration writes it.
    pub address: String,
    /// Its clone's container, the folder `flume launch --repository`
    /// names.
    pub container_dir: String,
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

/// A repository's status as Flume's status gives it: whether the merge
/// queue its binding names lands its default branch, and each kind Flume
/// would run there with whether it lands.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RepositoryStatus {
    /// The item of Flume's `status.repositories` for the repository's
    /// clone, as Flume printed it.
    Printed(String),
    /// Flume's status gave none, with the words saying why.
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
    /// Each launch whose carried status the control plane took with the
    /// report this answers, and no other: the runner confirms the ends
    /// Flume owes it of a status only once it is named here (design
    /// Decision 1; core.54; record 0030 as amended on 2026-11-09). An
    /// answer of before, naming none, reads as naming none.
    #[serde(default, skip_serializing_if = "BTreeSet::is_empty")]
    pub statuses_taken: BTreeSet<Id>,
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A forge token's `Debug` names its end, and no character of its
    /// secret: the text is the same whatever the secret is (core.4).
    #[test]
    fn a_forge_tokens_debug_names_its_end_alone() {
        let ends = OffsetDateTime::parse("2026-11-18T13:00:00Z", &Rfc3339).unwrap();
        let text = format!("{:?}", ForgeToken::new("ghs_n0t-sh0wn", ends));
        assert!(text.contains("2026-11-18T13:00:00Z"), "{text}");
        assert!(!text.contains("ghs_n0t-sh0wn"), "{text}");
        assert_eq!(
            format!("{:?}", ForgeToken::new("another-secret-entirely", ends)),
            text,
            "a forge token's Debug turns on its secret"
        );
        let given = format!(
            "{:?}",
            TokenAsked::Token(ForgeToken::new("ghs_n0t-sh0wn", ends))
        );
        assert!(!given.contains("ghs_n0t-sh0wn"), "{given}");
    }
}
