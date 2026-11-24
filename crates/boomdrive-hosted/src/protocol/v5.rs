//! Version 5 of the runner's protocol (record 0054): its JSON Schema
//! files, draft 2020-12, under `protocol/v5/`, and the runner's types for
//! its messages. Version 5 is version 4 whole but that it holds no
//! observing hello and no observed report, and that a report names
//! `unleased`, each place in the partition's repositories it did not
//! lease, each file's `$id` under `protocol/v5/` and every message naming
//! `protocol_version` 5; version 4 is version 3 whole but for a forge
//! token's ask and its two answers, which go by one request at a path of
//! their own, `runner/forge-token` (record 0053); version 3 is version 2
//! whole but for a control plane's hold among the acts (record 0052);
//! version 2 is version 1 but for a place's ends, each with its delivery,
//! a control plane's confirm and land among the acts, a preview among the
//! wants, and a report's previews (record 0037 as amended on 2026-11-09).
//! A check of a message is built from [`FILES`] alone, each file under its
//! `$id`, and fetches nothing (core.42).
//!
//! The types reuse the interface's wherever version 5 carries a shape as
//! the interface writes it, a hold among the acts, and are version 5's own
//! where the wire differs: a place's status, sluice's entry and sluice's
//! answer and listing of a preview go up as their copies, the host's
//! capacity as values, and a terminal's link as sealed frames (design
//! Decision 6 of the-runner-dials-the-hosted-control-plane). Each struct
//! of version 5's own reads no field the version does not name. Nothing
//! here converts the interface's values into version 5's or back.

use boomdrive_control::{
    Acted, Answer, At, Commanded, Id, InboxCarried, Lease, Outcome, ToolVersion, Want,
};
use serde::{Deserialize, Serialize};
use serde_json::value::RawValue;
use std::collections::BTreeMap;
use std::fmt;

/// This version's number, which every message of it names as its
/// `protocol_version`.
pub const VERSION: u32 = 5;

/// This version's name: the subprotocol a socket's upgrade asks for, and
/// the start of every context a key signs (the protocol's README, *The
/// paths* and *The signatures*).
pub const SUBPROTOCOL: &str = "boomdrive.v5";

/// The address each file's `$id` is under: its own among the repository's
/// raw files at `main`. No check fetches it.
pub const ADDRESS: &str = "https://raw.githubusercontent.com/boomdrivedev/boomdrive/main/crates/boomdrive-hosted/protocol/v5/";

/// Each schema file of version 5 by its name, with its text.
pub const FILES: &[(&str, &str)] = &[
    (
        "from-runner.schema.json",
        include_str!("../../protocol/v5/from-runner.schema.json"),
    ),
    (
        "to-runner.schema.json",
        include_str!("../../protocol/v5/to-runner.schema.json"),
    ),
    (
        "shared.schema.json",
        include_str!("../../protocol/v5/shared.schema.json"),
    ),
    (
        "enrollment.schema.json",
        include_str!("../../protocol/v5/enrollment.schema.json"),
    ),
    (
        "hello.schema.json",
        include_str!("../../protocol/v5/hello.schema.json"),
    ),
    (
        "wants.schema.json",
        include_str!("../../protocol/v5/wants.schema.json"),
    ),
    (
        "report.schema.json",
        include_str!("../../protocol/v5/report.schema.json"),
    ),
    (
        "place-status.schema.json",
        include_str!("../../protocol/v5/place-status.schema.json"),
    ),
    (
        "submission-entry.schema.json",
        include_str!("../../protocol/v5/submission-entry.schema.json"),
    ),
    (
        "preview.schema.json",
        include_str!("../../protocol/v5/preview.schema.json"),
    ),
    (
        "terminal.schema.json",
        include_str!("../../protocol/v5/terminal.schema.json"),
    ),
    (
        "forge-token.schema.json",
        include_str!("../../protocol/v5/forge-token.schema.json"),
    ),
];

/// A message of version 5, either direction's: its version, its own id
/// and one body (design Decision 3).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Message<Body> {
    /// [`VERSION`], in every message of version 5. The type reads any
    /// number: a message naming another is refused by its check against
    /// the schema, which comes before it is read (core.42).
    pub protocol_version: u32,
    pub id: MessageId,
    pub body: Body,
}

impl<Body> Message<Body> {
    /// A message of this version, under its sender's id for it.
    pub fn new(id: MessageId, body: Body) -> Message<Body> {
        Message {
            protocol_version: VERSION,
            id,
            body,
        }
    }
}

/// A message's own id, given by its sender, unique among what that sender
/// sends on its enrollment and kept on a resend (core.21). It is not a
/// want's [`Id`].
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct MessageId(String);

impl MessageId {
    pub fn new(text: impl Into<String>) -> MessageId {
        MessageId(text.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for MessageId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// The body of a message a runner sends, one kind by its name.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FromRunner {
    Enroll(Enroll),
    Hello(Hello),
    Report(Report),
    TerminalOpened(TerminalOpened),
    TerminalOutput(TerminalOutput),
    TerminalClosed(TerminalClosed),
    MessageRefused(MessageRefused),
    ForgeToken(ForgeToken),
}

/// The body of a message a control plane sends, one kind by its name.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ToRunner {
    Enrolled(Enrolled),
    EnrollRefused(EnrollRefused),
    Pulled(Pulled),
    Reported(Reported),
    TerminalOpen(TerminalOpen),
    TerminalResume(TerminalResume),
    TerminalInput(TerminalInput),
    TerminalClose(TerminalClose),
    MessageRefused(MessageRefused),
    ForgeTokenGiven(ForgeTokenGiven),
    ForgeTokenRefused(ForgeTokenRefused),
}

/// A partition's enrollment, the body of the runner's one request to
/// `runner/enroll`: the partition's public key, made on the host, signed
/// over the enrollment's context. The token is the request's bearer, and
/// no message holds it (core.4, core.31; record 0018).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Enroll {
    /// The partition's Ed25519 public key, 32 bytes in base64url without
    /// padding.
    pub key: String,
    /// The key's signature over the enrollment's context, 64 bytes in
    /// base64url without padding.
    pub signature: String,
    /// The host's name, as its operating system gives it.
    pub machine: String,
}

/// The control plane took the partition's key.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Enrolled {
    /// The control plane's id for the host, which the runner shows and
    /// keeps nowhere: the control plane finds the enrollment by its key.
    pub host: String,
}

/// The control plane took no key, with words saying why.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EnrollRefused {
    /// The `enroll` message it answers, by that message's id.
    pub enroll: MessageId,
    pub words: String,
}

/// The interface's hello as version 5 carries it, its capacity's printed
/// part the copy of Flume's host part (design Decision 8).
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Hello {
    /// The interface's version, apart from the protocol's.
    pub interface_version: u32,
    /// `boomdrive --version`'s text.
    pub runner_version: String,
    /// Each tool's version, by the tool's name.
    pub tool_versions: BTreeMap<String, ToolVersion>,
    /// The addresses, `owner/name`, the runner keeps for this partition
    /// with their clones made (core.26).
    pub repositories: Vec<String>,
    pub capacity: Capacity,
    /// Each launch's, each command's and each preview's id with the lease
    /// number held.
    pub leases: BTreeMap<Id, u64>,
}

/// The host's capacity, as Flume's status gives it.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Capacity {
    Printed(HostPart),
    /// Flume's status could not be read, with the words saying why.
    Missing(String),
}

/// The copy of Flume's `status.host`: each field as Flume printed it, its
/// streams without their queues (core.26).
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HostPart {
    pub running: Box<RawValue>,
    pub bound: Box<RawValue>,
    pub room: Box<RawValue>,
    pub streams: Box<RawValue>,
}

/// The runner's whole report as version 5 carries it, each part by the id
/// of the want it answers.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Report {
    pub launches: BTreeMap<Id, Launched>,
    pub acts: BTreeMap<Id, Acted>,
    pub commands: BTreeMap<Id, Commanded>,
    pub inboxes: BTreeMap<Id, InboxCarried>,
    pub previews: BTreeMap<Id, Previewed>,
    /// Each place in the partition's repositories it did not lease, by its
    /// path as Flume printed it, with no lease (record 0054).
    pub unleased: BTreeMap<String, Unleased>,
}

/// A launch the runner answered, with its place's status and its
/// submission's entry as their copies.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Launched {
    pub lease: Lease,
    pub answer: Answer,
    /// The copy of Flume's status of its place (design Decision 7).
    pub status: Option<Carried>,
    /// The copy of sluice's entry of its submission (design Decision 9).
    pub entry: Option<Carried>,
}

/// A place the partition did not lease: its repository, and its status
/// as its copy, carried as a launch's is (record 0054; core.10).
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Unleased {
    pub repository: String,
    /// The copy of Flume's status of the place.
    pub status: Option<Carried>,
}

/// A preview the runner answered, with sluice's answer and its listing of
/// the preview as their copies (design Decision 3 of
/// the-hosted-protocol-carries-land-and-preview; core.27, core.55).
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Previewed {
    pub lease: Lease,
    /// The repository and the revision, as the preview named them.
    pub repository: String,
    pub revision: String,
    /// Whether the preview asked that it be put away.
    pub done: bool,
    /// `Held` while sluice waits at the host's bound to bring it up; else
    /// `Done` or `Refused`. A check refuses any other.
    pub outcome: Outcome,
    /// sluice's answer as its copy: a preview's, or a release's for a
    /// done (design Decision 4).
    pub copy: Option<Box<RawValue>>,
    pub words: Option<String>,
    /// sluice's listing of the preview as its copy.
    pub status: Option<Carried>,
    /// When the outcome, copy or words last moved.
    pub at: At,
}

/// A tool's answer carried as its copy, which goes up as the runner cut
/// it, never written again (core.27).
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Carried {
    pub copy: Box<RawValue>,
    /// When the runner first read it as it now stands.
    pub at: At,
    /// Whether the tool's last listing held it.
    pub listed: bool,
}

/// The whole of what the control plane wants, in its order, and the
/// runner's report as the control plane holds it: given at every
/// connection in answer to the hello, and with no report whenever the
/// wants change (design Decision 5).
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Pulled {
    pub wants: Vec<Want>,
    pub report: Option<Report>,
}

/// The control plane's answer to a report.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Reported {
    /// The `report` message it answers, by that message's id.
    pub report: MessageId,
    /// The ids the control plane carries no more.
    pub let_go: Vec<Id>,
    /// Each id whose report was refused, its lease no longer the
    /// runner's, with the control plane's words.
    pub refused: BTreeMap<Id, String>,
}

/// Either end's answer to a message that failed its check, which is
/// refused whole (core.42).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MessageRefused {
    /// The message, by its id.
    pub message: MessageId,
    pub words: String,
}

/// An end's key share for one terminal (design Decision 10; core.30).
/// Each of its bytes is base64url without padding.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Share {
    /// The scheme both ends use, which the two ends agree.
    pub suite: String,
    /// The end's public key for this terminal alone.
    pub key: String,
    /// The share's signature by the end's signing key.
    pub signature: String,
    /// What the control plane certified on that signing key.
    pub certificate: String,
}

/// The control plane opens a terminal on a launch's agent, with the
/// browser's share.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TerminalOpen {
    pub terminal: Id,
    /// The launch whose agent it is, by id.
    pub launch: Id,
    /// The launch's lease number.
    pub lease: u64,
    pub share: Share,
}

/// The runner opened a terminal, with its share.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TerminalOpened {
    pub terminal: Id,
    pub share: Share,
}

/// A frame of the pane's bytes, sealed.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TerminalOutput {
    pub terminal: Id,
    /// The place of the frame's first byte in the runner's stream,
    /// counted from 0 at the terminal's opening.
    pub offset: u64,
    /// Whether the frame is the pane's present screen, which the browser
    /// draws after clearing its terminal.
    pub redraw: bool,
    /// The suite's ciphertext of the frame's bytes, in base64url.
    pub sealed: String,
}

/// A frame of the person's keys, sealed.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TerminalInput {
    pub terminal: Id,
    /// The place of the frame's first byte in the browser's stream,
    /// counted from 0 at the terminal's opening.
    pub offset: u64,
    /// The suite's ciphertext of the frame's bytes, in base64url.
    pub sealed: String,
}

/// A browser whose socket dropped, its keys still held, says the offset
/// it has every byte up to.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TerminalResume {
    pub terminal: Id,
    pub seen: u64,
}

/// The browser's end closes a terminal.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TerminalClose {
    pub terminal: Id,
}

/// The runner closed a terminal, with words saying why.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TerminalClosed {
    pub terminal: Id,
    pub words: String,
}

/// The runner's ask for a served repository's forge token, the body of
/// its one request to `runner/forge-token`, signed under the key of a
/// partition served the repository. It never goes on a partition's socket
/// (design Decision 2 of a-cloud-host-gets-its-github-tokens-from-the-cloud;
/// record 0053).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ForgeToken {
    /// The repository's address, `owner/name`.
    pub repository: String,
}

/// The control plane minted a token for the repository from the App's
/// installation on the host's account. Its `token` is the one field of any
/// version that holds a credential, so its `Debug` names the ask, the
/// repository and the token's end, and never the token (core.4; record
/// 0053).
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ForgeTokenGiven {
    /// The ask it answers, by that message's id.
    pub forge_token: MessageId,
    /// The repository the token is for, as the ask named it.
    pub repository: String,
    /// The token as the forge gave it.
    pub token: String,
    /// When the token ends, as the forge gave it.
    pub expires_at: At,
}

impl fmt::Debug for ForgeTokenGiven {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ForgeTokenGiven")
            .field("forge_token", &self.forge_token)
            .field("repository", &self.repository)
            .field("expires_at", &format_args!("{}", self.expires_at))
            .finish_non_exhaustive()
    }
}

/// The control plane gave no token for the repository, with words saying
/// why.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ForgeTokenRefused {
    /// The ask it answers, by that message's id.
    pub forge_token: MessageId,
    /// The repository the ask named.
    pub repository: String,
    /// True alone where the App's installation on the host's account does
    /// not cover the repository, which the runner keeps as it keeps the
    /// forge's word that the App is not installed.
    pub not_installed: bool,
    pub words: String,
}
