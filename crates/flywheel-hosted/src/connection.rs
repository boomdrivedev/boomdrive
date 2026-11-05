//! One partition's connection (design Decisions 5, 6 and 10; core.3,
//! core.21 to core.23, core.31, core.42): one outgoing WebSocket to
//! `runner/connect`, upgraded on version 1's subprotocol with a statement
//! of the partition's key signed for that attempt alone. Its first message
//! at every connection is the partition's whole hello, and its hello goes
//! again whenever it changes. It is kept alive by the WebSocket protocol's
//! own pings, never a message of the protocol's, and counted lost when a
//! pong has not come by the next ping. A connection lost, refused or not
//! made is tried again on a backoff with jitter whose cap stops at
//! `poll_interval`, and its reach is said once each way. Every message it
//! is sent is checked against version 1 before it is read.
//!
//! Its task runs on the hosted implementation's runtime, and the runner's
//! loop reaches it through [`Connection`] alone. It keeps nothing of
//! enrolment but the key, read again from the host's store at every
//! attempt (core.35).

use crate::check;
use crate::key::{self, KeyName, Store};
use crate::protocol::v1::{
    Hello, Message, MessageId, Pulled, Report, ToRunner, SUBPROTOCOL, VERSION,
};
use crate::table::Table;
use flywheel_control::Want;
use futures_util::stream::{SplitSink, SplitStream};
use futures_util::{SinkExt, StreamExt};
use serde_json::{json, Value};
use std::collections::VecDeque;
use std::sync::{Arc, Condvar, Mutex, MutexGuard};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::net::TcpStream;
use tokio::runtime::Handle;
use tokio::sync::mpsc;
use tokio::task::JoinHandle;
use tokio::time::{Instant, MissedTickBehavior};
use tokio_tungstenite::tungstenite::client::IntoClientRequest;
use tokio_tungstenite::tungstenite::error::{Error, ProtocolError};
use tokio_tungstenite::tungstenite::http::{HeaderValue, StatusCode};
use tokio_tungstenite::tungstenite::protocol::frame::coding::CloseCode;
use tokio_tungstenite::tungstenite::protocol::CloseFrame;
use tokio_tungstenite::tungstenite::Message as Frame;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};
use url::Url;
use uuid::Uuid;

/// How often a live connection is pinged, and so how long a pong may
/// take before the connection is counted lost.
pub const PING_INTERVAL: Duration = Duration::from_secs(30);

/// Where the backoff's cap starts, and goes back to once a connection
/// takes its first `pulled`.
pub const LEAST_CAP: Duration = Duration::from_secs(1);

/// How long a close waits for the control plane's own close.
const CLOSE_WAIT: Duration = Duration::from_secs(1);

/// The most characters of the control plane's words a line carries.
const WORDS_LENGTH: usize = 300;

/// How many of the messages a connection sent it remembers the kinds of,
/// to name one the control plane refuses.
const KINDS_KEPT: usize = 32;

/// Where a connection says its lines, one each, from its runtime's
/// threads.
pub type Say = Arc<dyn Fn(String) + Send + Sync>;

/// What a connection calls whenever it takes a `pulled`, to wake the
/// runner for a control pass at once (design Decision 7).
pub type Wake = Arc<dyn Fn() + Send + Sync>;

type Socket = WebSocketStream<MaybeTlsStream<TcpStream>>;

/// What a partition's connection is opened with.
pub struct Dial {
    /// The partition's label, which starts each of its lines.
    pub label: String,
    /// Where its socket is opened: `runner/connect` under the hosted
    /// table's `url`.
    pub address: Url,
    /// The control plane's origin, which each statement names.
    pub origin: String,
    /// What its key is kept under in `store`.
    pub key: KeyName,
    /// The host's credential store, read again at every attempt.
    pub store: Arc<dyn Store>,
    /// Where the backoff's cap stops (core.22).
    pub poll_interval: Duration,
    pub say: Say,
    pub wake: Wake,
}

impl Dial {
    /// The connection of `table`'s partition `label`, its key read from
    /// `store`.
    pub fn new(
        table: &Table,
        label: &str,
        store: Arc<dyn Store>,
        poll_interval: Duration,
        say: Say,
        wake: Wake,
    ) -> Dial {
        Dial {
            label: label.to_string(),
            address: table.connect_address(),
            origin: table.origin(),
            key: table.key_name(label),
            store,
            poll_interval,
            say,
            wake,
        }
    }
}

/// What a partition's connection has taken of its control plane since
/// its hello: the wants of its last `pulled`, in the control plane's
/// order, and the report the control plane held at the connection's
/// first.
#[derive(Clone, Debug)]
pub struct Read {
    pub wants: Vec<Want>,
    pub report: Option<Report>,
}

/// Where a partition's connection stands.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
enum Phase {
    /// No hello has been handed it, so it has not been attempted.
    #[default]
    Unhanded,
    /// An attempt is being made, or a connection made waits for its first
    /// `pulled`.
    Attempting,
    /// Its connection has taken a `pulled` since its hello.
    Live,
    /// It waits on its backoff to be tried again.
    Waiting,
}

/// A failure that is the host's or the control plane's to fix rather than
/// the network's: it waits at the backoff's cap, and is said once while
/// the control plane is unreachable (design Decisions 3 and 5).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Refusal {
    NoKey,
    KeyUnread,
    KeyRefused,
    Subprotocol,
}

/// Why an attempt failed, or a connection was lost.
struct Failure {
    words: String,
    refusal: Option<Refusal>,
}

impl Failure {
    /// A failure of the network's, or the control plane's answer, tried
    /// again on the backoff.
    fn again(words: impl Into<String>) -> Failure {
        Failure {
            words: words.into(),
            refusal: None,
        }
    }

    /// A refusal, tried again at the backoff's cap.
    fn refused(refusal: Refusal, words: impl Into<String>) -> Failure {
        Failure {
            words: words.into(),
            refusal: Some(refusal),
        }
    }
}

/// How an attempt ended.
enum Ended {
    /// The implementation closed the connection.
    Closed,
    Failed(Failure),
}

/// What the connection's task holds that the runner's loop reads.
#[derive(Default)]
struct State {
    /// The hello last handed, as version 1 writes it.
    hello: Option<Value>,
    phase: Phase,
    /// The last `pulled` its connection took since its hello.
    pulled: Option<Pulled>,
    /// The report the control plane held at the connection's first
    /// `pulled`.
    held: Option<Report>,
    /// Why the partition is not reached, where it is not.
    why: Option<String>,
    /// Whether the partition was reached when last said: none before its
    /// first attempt ends.
    reached: Option<bool>,
    /// The refusals said since the control plane was last reached.
    said: Vec<Refusal>,
}

#[derive(Default)]
struct Shared {
    state: Mutex<State>,
    changed: Condvar,
}

impl Shared {
    fn lock(&self) -> MutexGuard<'_, State> {
        self.state.lock().unwrap_or_else(|e| e.into_inner())
    }

    /// Changes the state by `change`, and wakes whoever waits on it.
    fn change<T>(&self, change: impl FnOnce(&mut State) -> T) -> T {
        let changed = change(&mut self.lock());
        self.changed.notify_all();
        changed
    }
}

/// What the runner's loop asks of a connection's task.
enum Command {
    /// A new hello was handed.
    Hello,
    /// Close the connection, with a normal close.
    Close,
}

/// A partition's connection, as the runner's loop holds it. Dropping it
/// closes the connection.
pub struct Connection {
    label: String,
    shared: Arc<Shared>,
    commands: mpsc::UnboundedSender<Command>,
    task: Option<JoinHandle<()>>,
}

impl Connection {
    /// Opens the connection `dial` gives, its task on `runtime`. It is
    /// first attempted once a hello is handed it.
    pub fn open(runtime: &Handle, dial: Dial) -> Connection {
        let shared = Arc::new(Shared::default());
        let (commands, received) = mpsc::unbounded_channel();
        let label = dial.label.clone();
        let task = runtime.spawn(run(dial, shared.clone(), received));
        Connection {
            label,
            shared,
            commands,
            task: Some(task),
        }
    }

    /// The partition's label.
    pub fn label(&self) -> &str {
        &self.label
    }

    /// Hands the connection the partition's hello, which it sends first at
    /// every connection, and at once on a live connection where it differs
    /// from the hello it sent; or why the hello fails version 1, which is
    /// then not handed.
    pub fn hand(&self, hello: &Hello) -> Result<(), String> {
        let hello = serde_json::to_value(hello)
            .map_err(|e| format!("`{}`'s hello could not be written: {e}", self.label))?;
        if let Some(words) = check::failure(check::from_runner(), &envelope("hello", &hello).1) {
            return Err(format!(
                "`{}`'s hello fails version 1's check, so is not sent: {words}",
                self.label
            ));
        }
        let changed = self.shared.change(|state| {
            if state.hello.as_ref() == Some(&hello) {
                return false;
            }
            state.hello = Some(hello);
            if state.phase == Phase::Unhanded {
                state.phase = Phase::Attempting;
            }
            true
        });
        if changed {
            let _ = self.commands.send(Command::Hello);
        }
        Ok(())
    }

    /// What the connection has taken since its hello, waiting up to
    /// `patience` while an attempt is being made; or why it has taken
    /// nothing. A connection waiting on its backoff is not waited for.
    pub fn read(&self, patience: Duration) -> Result<Read, String> {
        let deadline = std::time::Instant::now() + patience;
        let mut state = self.shared.lock();
        loop {
            match state.phase {
                Phase::Live => {
                    return Ok(Read {
                        wants: state
                            .pulled
                            .as_ref()
                            .map(|pulled| pulled.wants.clone())
                            .unwrap_or_default(),
                        report: state.held.clone(),
                    })
                }
                Phase::Unhanded => {
                    return Err("no hello has been handed it, so it is not dialed".to_string())
                }
                Phase::Waiting => {
                    return Err(state
                        .why
                        .clone()
                        .unwrap_or_else(|| "it waits to be tried again".to_string()))
                }
                Phase::Attempting => {
                    let left = deadline.saturating_duration_since(std::time::Instant::now());
                    if left.is_zero() {
                        return Err(format!(
                            "the control plane gave no `pulled` within {patience:?}"
                        ));
                    }
                    state = self
                        .shared
                        .changed
                        .wait_timeout(state, left)
                        .unwrap_or_else(|e| e.into_inner())
                        .0;
                }
            }
        }
    }

    /// Closes the connection with a normal close, and gives its task, to
    /// wait on while it closes; none where it was given before.
    pub fn close(&mut self) -> Option<JoinHandle<()>> {
        let _ = self.commands.send(Command::Close);
        self.task.take()
    }
}

impl Drop for Connection {
    fn drop(&mut self) {
        let _ = self.commands.send(Command::Close);
    }
}

/// The backoff a partition's attempts wait on (core.22): a wait drawn at
/// random from above zero up to a cap, which starts at [`LEAST_CAP`],
/// doubles at each failed attempt and stops at `poll_interval`.
struct Backoff {
    cap: Duration,
    most: Duration,
}

impl Backoff {
    fn new(poll_interval: Duration) -> Backoff {
        Backoff {
            cap: LEAST_CAP.min(poll_interval),
            most: poll_interval,
        }
    }

    /// The wait after a failed attempt, after which the cap doubles.
    fn wait(&mut self) -> Duration {
        let wait = drawn(self.cap);
        self.cap = self.cap.saturating_mul(2).min(self.most);
        wait
    }

    /// The wait after a refusal: the whole cap's stop, which the cap then
    /// stays at.
    fn at_cap(&mut self) -> Duration {
        self.cap = self.most;
        self.most
    }

    /// The cap back at its start, once a connection has taken its first
    /// `pulled`.
    fn reset(&mut self) {
        self.cap = LEAST_CAP.min(self.most);
    }
}

/// A wait drawn at random, evenly, from one millisecond up to `cap`, from
/// the operating system's randomness; `cap` itself where none can be
/// read. It is whole milliseconds, as tokio's timer counts, so no wait is
/// rounded up past its cap.
fn drawn(cap: Duration) -> Duration {
    let millis = cap.as_millis().max(1);
    let random = getrandom::u64().map(u128::from).unwrap_or(millis - 1);
    Duration::from_millis(u64::try_from(random % millis + 1).unwrap_or(u64::MAX))
}

/// A message of version 1 of `kind` holding `body`, under a new id of
/// its own, a UUID of version 7, time-ordered and random, so that no id
/// is given twice (design Decision 7).
fn envelope(kind: &str, body: &Value) -> (String, Value) {
    let id = Uuid::now_v7().to_string();
    let message = json!({ "protocol_version": VERSION, "id": id, "body": { kind: body } });
    (id, message)
}

/// The control plane's `words`, on one line, cut to [`WORDS_LENGTH`]
/// characters.
fn one_line(words: &str) -> String {
    let line: Vec<&str> = words.split_whitespace().collect();
    let line = line.join(" ");
    match line.char_indices().nth(WORDS_LENGTH) {
        Some((end, _)) => format!("{}…", &line[..end]),
        None => line,
    }
}

/// The connection's task: it waits for a hello, then attempts, serves and
/// waits on its backoff until it is closed.
async fn run(dial: Dial, shared: Arc<Shared>, mut commands: mpsc::UnboundedReceiver<Command>) {
    loop {
        let handed = shared.lock().hello.is_some();
        if handed {
            break;
        }
        match commands.recv().await {
            Some(Command::Hello) => {}
            Some(Command::Close) | None => return,
        }
    }
    let mut backoff = Backoff::new(dial.poll_interval);
    loop {
        shared.change(|state| state.phase = Phase::Attempting);
        let failure = match attempt(&dial, &shared, &mut commands, &mut backoff).await {
            Ended::Closed => return,
            Ended::Failed(failure) => failure,
        };
        let wait = match failure.refusal {
            Some(_) => backoff.at_cap(),
            None => backoff.wait(),
        };
        unreached(&dial, &shared, failure);
        let until = Instant::now() + wait;
        loop {
            tokio::select! {
                () = tokio::time::sleep_until(until) => break,
                command = commands.recv() => match command {
                    Some(Command::Hello) => {}
                    Some(Command::Close) | None => return,
                },
            }
        }
    }
}

/// One attempt: the key read, the upgrade made with a fresh statement,
/// and the connection served until it is lost or closed.
async fn attempt(
    dial: &Dial,
    shared: &Shared,
    commands: &mut mpsc::UnboundedReceiver<Command>,
    backoff: &mut Backoff,
) -> Ended {
    let label = &dial.label;
    let (store, name) = (dial.store.clone(), dial.key.clone());
    let key = match tokio::task::spawn_blocking(move || store.read(&name)).await {
        Ok(Ok(Some(key))) => key,
        Ok(Ok(None)) => {
            return Ended::Failed(Failure::refused(
                Refusal::NoKey,
                format!(
                    "no key of `{label}`'s is kept in the host's credential store: \
                     `flywheel enroll {label}` enrols the partition"
                ),
            ))
        }
        Ok(Err(words)) => return Ended::Failed(Failure::refused(Refusal::KeyUnread, words)),
        Err(e) => {
            return Ended::Failed(Failure::refused(
                Refusal::KeyUnread,
                format!("`{label}`'s key could not be read: {e}"),
            ))
        }
    };
    let time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|since| since.as_secs())
        .unwrap_or_default();
    let nonce = match key::nonce() {
        Ok(nonce) => nonce,
        Err(words) => return Ended::Failed(Failure::again(words)),
    };
    let statement = key::statement(&key, &dial.origin, time, &nonce);
    drop(key);

    let mut request = match dial.address.as_str().into_client_request() {
        Ok(request) => request,
        Err(e) => return Ended::Failed(Failure::again(format!("no upgrade could be made: {e}"))),
    };
    let Ok(mut authorization) = HeaderValue::from_str(&statement) else {
        return Ended::Failed(Failure::again("the statement is no header's value"));
    };
    authorization.set_sensitive(true);
    let headers = request.headers_mut();
    headers.insert(
        "sec-websocket-protocol",
        HeaderValue::from_static(SUBPROTOCOL),
    );
    headers.insert("authorization", authorization);

    let connecting = tokio_tungstenite::connect_async(request);
    tokio::pin!(connecting);
    let connected = loop {
        tokio::select! {
            connected = &mut connecting => break connected,
            command = commands.recv() => match command {
                Some(Command::Hello) => {}
                Some(Command::Close) | None => return Ended::Closed,
            },
        }
    };
    match connected {
        Ok((socket, _)) => serve(dial, shared, commands, backoff, socket).await,
        Err(error) => Ended::Failed(refused(label, error)),
    }
}

/// Why an upgrade was not made: a refusal of the key or of the
/// subprotocol, tried again at the cap, or any other failure.
fn refused(label: &str, error: Error) -> Failure {
    match error {
        Error::Http(response) => {
            let status = response.status();
            let words = response
                .body()
                .as_deref()
                .map(|body| one_line(&String::from_utf8_lossy(body)))
                .filter(|words| !words.is_empty());
            let words = match words {
                Some(words) => format!(": {words} ({status})"),
                None => format!(" ({status})"),
            };
            match status {
                StatusCode::UNAUTHORIZED | StatusCode::FORBIDDEN => Failure::refused(
                    Refusal::KeyRefused,
                    format!(
                        "it does not take `{label}`'s key{words}; `flywheel enroll {label}` \
                         enrols the partition again"
                    ),
                ),
                _ => Failure::again(format!("it answered the upgrade{words}")),
            }
        }
        Error::Protocol(ProtocolError::SecWebSocketSubProtocolError(_)) => Failure::refused(
            Refusal::Subprotocol,
            format!("it does not speak {SUBPROTOCOL}, the version this runner speaks"),
        ),
        error => Failure::again(error.to_string()),
    }
}

/// The kinds of the messages a connection sent, by their ids, the last
/// [`KINDS_KEPT`] of them.
#[derive(Default)]
struct Kinds(VecDeque<(String, &'static str)>);

impl Kinds {
    fn note(&mut self, id: String, kind: &'static str) {
        if self.0.len() == KINDS_KEPT {
            self.0.pop_front();
        }
        self.0.push_back((id, kind));
    }

    /// The kind of the message `id`, or `message` where it is not kept.
    fn of(&self, id: &MessageId) -> &'static str {
        self.0
            .iter()
            .find(|(sent, _)| sent == id.as_str())
            .map(|(_, kind)| *kind)
            .unwrap_or("message")
    }
}

/// A live connection's sending half, and the kinds of what it sent.
struct Sender {
    sink: SplitSink<Socket, Frame>,
    kinds: Kinds,
}

impl Sender {
    /// Sends a message of `kind` holding `body`, under a new id.
    async fn send(&mut self, kind: &'static str, body: &Value) -> Result<(), Failure> {
        let (id, message) = envelope(kind, body);
        self.sink
            .send(Frame::text(message.to_string()))
            .await
            .map_err(|e| Failure::again(format!("its `{kind}` could not be sent: {e}")))?;
        self.kinds.note(id, kind);
        Ok(())
    }
}

/// What a frame of the control plane's text held.
enum Received {
    /// A message that passed version 1, read.
    Message(Message<ToRunner>),
    /// A message that failed version 1, by its id, with words naming the
    /// paths that failed and never a value.
    Failing { id: MessageId, words: String },
    /// A message whose id cannot be read, which cannot be named.
    Unnamed(&'static str),
}

/// `text`, checked against `to-runner.schema.json` before it is read
/// (design Decision 10; core.42).
fn received(text: &str) -> Received {
    let Ok(value) = serde_json::from_str::<Value>(text) else {
        return Received::Unnamed("a message that is no JSON");
    };
    let id = match value.get("id") {
        Some(Value::String(id)) if check::failure(check::message_id(), &value["id"]).is_none() => {
            MessageId::new(id.clone())
        }
        _ => return Received::Unnamed("a message with no id that can be read"),
    };
    if let Some(words) = check::failure(check::to_runner(), &value) {
        return Received::Failing {
            id,
            words: format!("it fails version 1's check: {words}"),
        };
    }
    match serde_json::from_value(value) {
        Ok(message) => Received::Message(message),
        Err(_) => Received::Failing {
            id,
            words: "it passes version 1's check, but could not be read".to_string(),
        },
    }
}

/// Serves a connection made: the whole hello first, then each frame as it
/// comes, a ping at each tick and a hello whenever one is handed that
/// differs from the one sent, until the connection is lost or closed.
async fn serve(
    dial: &Dial,
    shared: &Shared,
    commands: &mut mpsc::UnboundedReceiver<Command>,
    backoff: &mut Backoff,
    socket: Socket,
) -> Ended {
    let label = &dial.label;
    let (sink, mut frames) = socket.split();
    let mut sender = Sender {
        sink,
        kinds: Kinds::default(),
    };
    let mut sent = shared.lock().hello.clone().unwrap_or_default();
    if let Err(failure) = sender.send("hello", &sent).await {
        return Ended::Failed(failure);
    }
    let mut pings = tokio::time::interval_at(Instant::now() + PING_INTERVAL, PING_INTERVAL);
    pings.set_missed_tick_behavior(MissedTickBehavior::Delay);
    let mut ponged = true;
    let mut first = true;
    loop {
        tokio::select! {
            frame = frames.next() => {
                let text = match frame {
                    Some(Ok(Frame::Text(text))) => text,
                    Some(Ok(Frame::Binary(_))) => {
                        return unnamed(label, sender, frames, "a frame that is no text").await
                    }
                    Some(Ok(Frame::Pong(_))) => {
                        ponged = true;
                        continue;
                    }
                    Some(Ok(Frame::Ping(_) | Frame::Frame(_))) => continue,
                    Some(Ok(Frame::Close(_))) | None => {
                        return Ended::Failed(Failure::again("it closed the connection"))
                    }
                    Some(Err(e)) => {
                        return Ended::Failed(Failure::again(format!("the connection was lost: {e}")))
                    }
                };
                let message = match received(text.as_str()) {
                    Received::Message(message) => message,
                    Received::Failing { id, words } => {
                        if let Err(failure) = refuse(dial, &mut sender, &id, &words).await {
                            return Ended::Failed(failure);
                        }
                        continue;
                    }
                    Received::Unnamed(what) => return unnamed(label, sender, frames, what).await,
                };
                match message.body {
                    ToRunner::Pulled(pulled) => {
                        if first {
                            backoff.reset();
                        }
                        reached(dial, shared, pulled, first);
                        first = false;
                    }
                    ToRunner::MessageRefused(refusal) => (dial.say)(format!(
                        "{label}: the control plane refused the runner's {} {}: {}",
                        sender.kinds.of(&refusal.message),
                        refusal.message,
                        one_line(&refusal.words)
                    )),
                    ToRunner::Enrolled(_) | ToRunner::EnrolRefused(_) => {
                        let words = "an enrolment's answer is taken on `runner/enrol` alone, \
                                     never on `runner/connect`";
                        if let Err(failure) = refuse(dial, &mut sender, &message.id, words).await {
                            return Ended::Failed(failure);
                        }
                    }
                    _ => {}
                }
            }
            _ = pings.tick() => {
                if !ponged {
                    return Ended::Failed(Failure::again(format!(
                        "no pong came within {}s of a ping",
                        PING_INTERVAL.as_secs()
                    )));
                }
                if let Err(e) = sender.sink.send(Frame::Ping(Vec::new().into())).await {
                    return Ended::Failed(Failure::again(format!("a ping could not be sent: {e}")));
                }
                ponged = false;
            }
            command = commands.recv() => match command {
                Some(Command::Hello) => {
                    let hello = shared.lock().hello.clone().unwrap_or_default();
                    if hello != sent {
                        if let Err(failure) = sender.send("hello", &hello).await {
                            return Ended::Failed(failure);
                        }
                        sent = hello;
                    }
                }
                Some(Command::Close) | None => {
                    let close = CloseFrame {
                        code: CloseCode::Normal,
                        reason: "".into(),
                    };
                    let _ = sender.sink.send(Frame::Close(Some(close))).await;
                    let _ = tokio::time::timeout(CLOSE_WAIT, async {
                        while let Some(Ok(_)) = frames.next().await {}
                    })
                    .await;
                    return Ended::Closed;
                }
            }
        }
    }
}

/// Refuses the control plane's message `id` with `words`: one line, and
/// `message_refused` naming it, and nothing else done with it.
async fn refuse(
    dial: &Dial,
    sender: &mut Sender,
    id: &MessageId,
    words: &str,
) -> Result<(), Failure> {
    (dial.say)(format!(
        "{}: the control plane's message {id} is refused: {words}",
        dial.label
    ));
    let body = json!({ "message": id, "words": words });
    sender.send("message_refused", &body).await
}

/// Closes a connection that was sent `what`, a message that cannot be
/// named, with code 1007, its payload not version 1's (design Decision
/// 10).
async fn unnamed(
    label: &str,
    mut sender: Sender,
    frames: SplitStream<Socket>,
    what: &str,
) -> Ended {
    let close = CloseFrame {
        // 1007: the frame's data is not what the connection carries.
        code: CloseCode::Invalid,
        reason: "a message of version 1 names its id".into(),
    };
    let _ = sender.sink.send(Frame::Close(Some(close))).await;
    drop(frames);
    Ended::Failed(Failure::again(format!(
        "it sent {what}, so `{label}`'s connection was closed"
    )))
}

/// The partition's connection took `pulled`, its first since its hello
/// where `first`: the partition is reached, which is said where it was
/// not, and the runner is woken.
fn reached(dial: &Dial, shared: &Shared, pulled: Pulled, first: bool) {
    let say = shared.change(|state| {
        if first {
            state.held = pulled.report.clone();
        }
        state.pulled = Some(pulled);
        state.phase = Phase::Live;
        state.why = None;
        state.said.clear();
        state.reached.replace(true) != Some(true)
    });
    if say {
        (dial.say)(format!("{}: the control plane is reached", dial.label));
    }
    (dial.wake)();
}

/// The partition's attempt failed, or its connection was lost, for
/// `failure`: it is unreachable, which is said where it was reached or
/// not yet tried, and a refusal is said once while it lasts.
fn unreached(dial: &Dial, shared: &Shared, failure: Failure) {
    let say = shared.change(|state| {
        state.phase = Phase::Waiting;
        state.pulled = None;
        state.held = None;
        state.why = Some(failure.words.clone());
        let lost = state.reached.replace(false) != Some(false);
        if lost {
            state.said.clear();
        }
        let refusal = match failure.refusal {
            Some(refusal) if !state.said.contains(&refusal) => {
                state.said.push(refusal);
                true
            }
            _ => false,
        };
        lost || refusal
    });
    if say {
        (dial.say)(format!(
            "{}: the control plane is unreachable: {}",
            dial.label, failure.words
        ));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_cap_doubles_from_a_second_and_stops_at_the_poll_interval() {
        let mut backoff = Backoff::new(Duration::from_secs(60));
        let caps: Vec<u64> = (0..9)
            .map(|_| {
                let cap = backoff.cap;
                let wait = backoff.wait();
                assert!(
                    !wait.is_zero() && wait <= cap,
                    "{wait:?} is not within {cap:?}"
                );
                cap.as_secs()
            })
            .collect();
        assert_eq!(caps, [1, 2, 4, 8, 16, 32, 60, 60, 60]);
        backoff.reset();
        assert_eq!(backoff.cap, LEAST_CAP);
    }

    #[test]
    fn a_refusal_waits_the_whole_cap() {
        let mut backoff = Backoff::new(Duration::from_secs(60));
        assert_eq!(backoff.at_cap(), Duration::from_secs(60));
        assert!(backoff.wait() <= Duration::from_secs(60));
        assert_eq!(backoff.cap, Duration::from_secs(60));
    }

    #[test]
    fn a_poll_interval_under_a_second_is_the_cap_from_the_start() {
        let mut backoff = Backoff::new(Duration::from_millis(200));
        assert!(backoff.wait() <= Duration::from_millis(200));
    }

    #[test]
    fn a_message_with_no_id_is_unnamed() {
        for text in [
            "[1, 2]",
            "{\"body\": {}}",
            "{\"id\": 5}",
            "{\"id\": \"a b\"}",
            "no",
        ] {
            assert!(
                matches!(received(text), Received::Unnamed(_)),
                "{text} is named"
            );
        }
    }

    #[test]
    fn a_failing_message_is_named_and_its_words_hold_no_value() {
        let text = json!({
            "protocol_version": 1,
            "id": "plane:9",
            "body": { "pulled": { "wants": [], "report": null, "secret": "n0t-sh0wn" } }
        })
        .to_string();
        let Received::Failing { id, words } = received(&text) else {
            panic!("{text} is not refused");
        };
        assert_eq!(id.as_str(), "plane:9");
        assert!(!words.contains("n0t-sh0wn"), "{words}");
    }

    #[test]
    fn the_control_planes_words_are_one_line_and_cut() {
        assert_eq!(one_line("this key\nis revoked "), "this key is revoked");
        let long = "a".repeat(WORDS_LENGTH + 10);
        assert_eq!(one_line(&long).chars().count(), WORDS_LENGTH + 1);
    }
}
