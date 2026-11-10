//! Version 2 of the runner's protocol (record 0037): its JSON Schema
//! files, draft 2020-12, under `protocol/v2/`, and the runner's types for
//! its messages. Version 2 is version 1 but for a place's ends, each with
//! its delivery, and a control plane's confirm. A check of a message is
//! built from [`FILES`] alone, each file under its `$id`, and fetches
//! nothing (core.42).
//!
//! The types reuse the interface's wherever version 2 carries a shape as
//! the interface writes it, and are version 2's own where the wire
//! differs: a place's status and sluice's entry go up as their copies,
//! the host's capacity as values, and a terminal's link as sealed frames
//! (design Decision 6 of the-runner-dials-the-hosted-control-plane). Each
//! struct of version 2's own reads no field the version does not name.
//! Nothing here converts the interface's values into version 2's or
//! back.

use boomdrive_control::{Acted, Answer, At, Commanded, Id, InboxCarried, Lease, ToolVersion, Want};
use serde::{Deserialize, Serialize};
use serde_json::value::RawValue;
use std::collections::BTreeMap;
use std::fmt;

/// This version's number, which every message of it names as its
/// `protocol_version`.
pub const VERSION: u32 = 2;

/// This version's name: the subprotocol a socket's upgrade asks for, and
/// the start of every context a key signs (the protocol's README, *The
/// paths* and *The signatures*).
pub const SUBPROTOCOL: &str = "boomdrive.v2";

/// The address each file's `$id` is under: its own among the repository's
/// raw files at `main`. No check fetches it.
pub const ADDRESS: &str = "https://raw.githubusercontent.com/agentplot/boomdrive/main/crates/boomdrive-hosted/protocol/v2/";

/// Each schema file of version 2 by its name, with its text.
pub const FILES: &[(&str, &str)] = &[
    (
        "from-runner.schema.json",
        include_str!("../../protocol/v2/from-runner.schema.json"),
    ),
    (
        "to-runner.schema.json",
        include_str!("../../protocol/v2/to-runner.schema.json"),
    ),
    (
        "shared.schema.json",
        include_str!("../../protocol/v2/shared.schema.json"),
    ),
    (
        "enrollment.schema.json",
        include_str!("../../protocol/v2/enrollment.schema.json"),
    ),
    (
        "hello.schema.json",
        include_str!("../../protocol/v2/hello.schema.json"),
    ),
    (
        "wants.schema.json",
        include_str!("../../protocol/v2/wants.schema.json"),
    ),
    (
        "report.schema.json",
        include_str!("../../protocol/v2/report.schema.json"),
    ),
    (
        "place-status.schema.json",
        include_str!("../../protocol/v2/place-status.schema.json"),
    ),
    (
        "submission-entry.schema.json",
        include_str!("../../protocol/v2/submission-entry.schema.json"),
    ),
    (
        "terminal.schema.json",
        include_str!("../../protocol/v2/terminal.schema.json"),
    ),
];

/// A message of version 2, either direction's: its version, its own id
/// and one body (design Decision 3).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Message<Body> {
    /// [`VERSION`], in every message of version 2. The type reads any
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

/// The interface's hello as version 2 carries it, its capacity's printed
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

/// The runner's whole report as version 2 carries it, each part by the id
/// of the want it answers.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Report {
    pub launches: BTreeMap<Id, Launched>,
    pub acts: BTreeMap<Id, Acted>,
    pub commands: BTreeMap<Id, Commanded>,
    pub inboxes: BTreeMap<Id, InboxCarried>,
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
