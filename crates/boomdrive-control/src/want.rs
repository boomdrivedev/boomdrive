//! What the control plane wants of the host (design Decision 2): launches
//! offered under leases, acts on a launch, commands and inboxes to carry,
//! and previews to bring up or put away, each under an id the control
//! plane gave, and the terminals it opens.
//! Every value is plain data but a terminal's link, which is a channel.

use crate::report::At;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::time::Duration;

/// An id the control plane gave, unique among what it wants of the host.
/// The runner never reads into it.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Id(String);

impl Id {
    pub fn new(text: impl Into<String>) -> Id {
        Id(text.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// A lease the control plane gives with a launch or a command (cloud
/// record 0009).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Lease {
    pub number: u64,
    /// When the lease runs out unless a report renews it, or none where
    /// the control plane never moves the work.
    pub expires: Option<At>,
}

/// One thing the control plane wants of the host.
// A pull gives a handful of wants, each read once, so an offer's size
// costs nothing that boxing it would save.
#[allow(clippy::large_enum_variant)]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Want {
    Offer(Offer),
    Act(Act),
    Command(Command),
    Inbox(Inbox),
    Preview(Preview),
}

impl Want {
    /// The id the control plane gave this want.
    pub fn id(&self) -> &Id {
        match self {
            Want::Offer(offer) => &offer.id,
            Want::Act(act) => &act.id,
            Want::Command(command) => &command.id,
            Want::Inbox(inbox) => &inbox.id,
            Want::Preview(preview) => &preview.id,
        }
    }
}

/// A launch offered under a lease.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Offer {
    /// The launch's id.
    pub id: Id,
    pub lease: Lease,
    pub launch: Launch,
}

/// What Flume is to be given for one place. Its kind is the only word of
/// kinds it holds: no kinds file and no kind's source (core.5).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Launch {
    /// The repository's address, `owner/name`.
    pub repository: String,
    /// Flume's kind, a name or an identity `<source>/<name>@<version>`, as
    /// the control plane gave it (design Decision 10).
    pub kind: String,
    pub name: String,
    pub base: Option<String>,
    /// The brief's whole text.
    pub brief: Option<String>,
    /// Flume's stream the place runs in, where the launch names one.
    pub stream: Option<String>,
    pub priority: Option<i64>,
    pub intent: Option<String>,
    pub recipients: Vec<String>,
    pub holds: Vec<Hold>,
    pub hold_submission: bool,
    /// The launches this one comes after, by id.
    pub after: Vec<Id>,
    pub agent_settings: Vec<AgentSetting>,
}

/// A stage the place holds at, with the words it holds there.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Hold {
    pub stage: String,
    pub words: String,
}

/// One of the agent's settings: a field and its value.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AgentSetting {
    pub field: String,
    pub value: String,
}

/// An act on a launch's place.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Act {
    pub id: Id,
    /// The launch acted on, by id.
    pub launch: Id,
    /// The launch's lease number.
    pub lease: u64,
    pub what: ActSort,
}

/// What an act asks of its launch's place.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActSort {
    Retry {
        stage: Option<String>,
        words: Option<String>,
        submission: bool,
    },
    /// The landing of its launch's work, written as the word `land`. It
    /// holds nothing but its launch, named by the act around it, and the
    /// runner carries it out as Flume's retry of the submission of that
    /// launch's place, naming no stage and no words (core.53). Version 1
    /// of the hosted protocol names no land, a later version being the
    /// one to carry it (core.42).
    Land,
    /// A reader's confirmation of ends of its launch's place, written as
    /// `{"confirm": {"recipient": …, "deliveries": […]}}`. It asks that
    /// each end of the place it names by its delivery's key be taken as
    /// delivered to the one `recipient` it names; `deliveries` holds each
    /// key as Flume gave it, in the control plane's order. The runner
    /// carries it out as Flume's confirm of exactly those keys as that
    /// recipient, each held to the launch's place (core.56). Version 1 of
    /// the hosted protocol names no confirm, a later version being the one
    /// to carry it (core.42).
    Confirm {
        recipient: String,
        deliveries: Vec<String>,
    },
    Drop,
    Priority(i64),
    Done,
}

/// A command, under a lease of its own.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Command {
    pub id: Id,
    pub lease: Lease,
    pub what: CommandSort,
}

/// What a command asks of the host.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CommandSort {
    /// A branch made from `from` (cloud record 0017). A control plane
    /// may still send it, as version 1 of the hosted protocol names it,
    /// which a release has spoken and which is never changed (core.42).
    /// This binary refuses it, naming sluice: a branch other places land
    /// on is a Flume place's, launched as any launch and landed on a land
    /// (core.52, core.53).
    Branch {
        repository: String,
        branch: String,
        from: String,
    },
    /// A sort this binary does not carry, naming its tool, with the
    /// control plane's words.
    Other { tool: String, words: String },
}

/// An inbox to carry: the one kept on `branch` of `repository`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Inbox {
    pub id: Id,
    /// The repository's address, `owner/name`.
    pub repository: String,
    pub branch: String,
}

/// A preview of a branch at a revision, to bring up or, done, to put
/// away, under a lease of its own (core.55). It names no word of the tool
/// that keeps previews. Version 1 of the hosted protocol names no preview,
/// and never will, a release having spoken it (core.42).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Preview {
    pub id: Id,
    pub lease: Lease,
    /// The repository's address, `owner/name`.
    pub repository: String,
    pub branch: String,
    /// The revision to bring up, a commit, as the control plane gave it.
    pub revision: String,
    /// Whether the preview of `revision` is to be put away.
    pub done: bool,
}

/// A terminal the control plane opens on a launch's agent (cloud record
/// 0016).
pub struct Terminal {
    pub id: Id,
    /// The launch whose agent it is, by id.
    pub launch: Id,
    /// The launch's lease number.
    pub lease: u64,
    pub link: Box<dyn Link>,
}

impl fmt::Debug for Terminal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Terminal")
            .field("id", &self.id)
            .field("launch", &self.launch)
            .field("lease", &self.lease)
            .finish_non_exhaustive()
    }
}

/// A terminal's two-way channel to the far end, the implementation's own.
pub trait Link: Send {
    /// The far end's next input, waiting up to `wait`, or none where
    /// nothing came.
    fn input(&mut self, wait: Duration) -> Option<Input>;
    /// Sends the pane's screen, and says whether the far end still
    /// listens.
    fn screen(&mut self, text: &str) -> bool;
    /// Closes the link, with words saying why.
    fn close(self: Box<Self>, words: &str);
}

/// What the far end of a terminal sends.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Input {
    Text(String),
    Keys(Vec<String>),
    Close,
}
