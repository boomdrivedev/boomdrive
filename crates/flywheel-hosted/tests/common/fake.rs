//! The fake hosted control plane (design Decision 14; Task notes 3.4):
//! test code alone, listening on the loopback address. It answers
//! `runner/enrol` with HTTP written by hand, and upgrades `runner/connect`
//! by tokio-tungstenite's handshake. It checks every message it is sent
//! against version 1, verifies the enrolment's and each connection's
//! signature over its context, built here from the protocol's README and
//! not from the crate's code, and records what it took, in order. A test
//! sets each partition's wants and the report it holds, and can make it
//! refuse a token, a key or the subprotocol, drop a connection, withhold
//! or delay a `reported`, and send any message.

use super::from_runner;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine;
use ed25519_dalek::{Signature, VerifyingKey};
use futures_util::{SinkExt, StreamExt};
use jsonschema::Validator;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::sync::{Arc, Condvar, Mutex, MutexGuard};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;
use tokio::sync::mpsc;
use tokio_tungstenite::tungstenite::handshake::server::{ErrorResponse, Request, Response};
use tokio_tungstenite::tungstenite::{http, Message as Frame};

/// The subprotocol of version 1, as the README names it.
pub const SUBPROTOCOL: &str = "flywheel-runner.v1";

/// The path the fake's address holds, under which its two paths go.
const BASE_PATH: &str = "/flywheel/";

/// How far a statement's time may be from the fake's clock.
const WINDOW: u64 = 300;

/// How long a test waits for what it waits on.
const PATIENCE: Duration = Duration::from_secs(10);

/// What the fake took, in the order it took it.
#[derive(Clone, Debug)]
pub enum Taken {
    /// A request to `runner/enrol`.
    Enrol {
        /// The token it bore, as `Authorization: Bearer` carried it.
        bearer: Option<String>,
        message: Value,
        /// Why the message failed version 1, where it did.
        refused: Option<String>,
        /// Whether its signature checked over the enrolment's context.
        signed: bool,
        /// The status the fake answered.
        status: u16,
    },
    /// An upgrade of `runner/connect`.
    Upgrade {
        /// The partition whose key the statement proved, where it did.
        partition: Option<String>,
        /// Each subprotocol it asked for.
        protocols: Vec<String>,
        /// Its `Authorization` header, whole.
        authorization: Option<String>,
        /// The statement's key and nonce, where it could be read.
        key: Option<[u8; 32]>,
        nonce: Option<Vec<u8>>,
        /// The status the fake answered, 101 where it took the upgrade.
        status: u16,
    },
    /// A message a partition's connection sent.
    Message {
        partition: String,
        message: Value,
        /// Why it failed version 1, where it did.
        refused: Option<String>,
    },
    /// A WebSocket ping a partition's connection sent.
    Ping { partition: String },
    /// A partition's connection closed by the runner, or lost.
    Closed { partition: String },
}

impl Taken {
    /// The kind of a message taken, or of an enrolment's message.
    pub fn kind(&self) -> Option<String> {
        let message = match self {
            Taken::Enrol { message, .. } | Taken::Message { message, .. } => message,
            _ => return None,
        };
        kind(message)
    }
}

/// A message's kind, the one key of its body.
pub fn kind(message: &Value) -> Option<String> {
    message.get("body")?.as_object()?.keys().next().cloned()
}

/// A one-time token the fake issued, for one partition.
struct Token {
    partition: String,
    /// The fake's id for the host it enrols.
    host: String,
    /// The words it is refused with, where a test refuses it.
    refused: Option<String>,
    /// The `enrol` that spent it, by its message's id and its key.
    spent: Option<(String, [u8; 32])>,
}

/// What a test set for a partition.
#[derive(Clone)]
struct Partition {
    wants: Value,
    report: Value,
    let_go: Vec<String>,
    refused: BTreeMap<String, String>,
    withhold: bool,
    delay: Duration,
}

impl Default for Partition {
    fn default() -> Partition {
        Partition {
            wants: json!([]),
            report: Value::Null,
            let_go: Vec::new(),
            refused: BTreeMap::new(),
            withhold: false,
            delay: Duration::ZERO,
        }
    }
}

/// What a partition's live connection is sent.
enum Out {
    Send(Value),
    Drop,
}

/// A partition's live connection.
struct Live {
    /// Its number, so a connection ended removes itself and not one after.
    number: u64,
    sender: mpsc::UnboundedSender<Out>,
    /// Whether it has sent its hello.
    hello: bool,
}

#[derive(Default)]
struct State {
    tokens: BTreeMap<String, Token>,
    /// Each key enrolled, to its partition.
    keys: BTreeMap<[u8; 32], String>,
    /// Each statement's key and nonce seen.
    nonces: BTreeSet<([u8; 32], Vec<u8>)>,
    /// The words each partition's key is refused with, where a test
    /// refuses it.
    refused_keys: BTreeMap<String, String>,
    refuse_subprotocol: bool,
    /// Whether every connection is closed as it comes.
    down: bool,
    partitions: BTreeMap<String, Partition>,
    connections: BTreeMap<String, Live>,
    /// The next enrolment's answer, where a test sets one.
    answer: Option<(u16, Value)>,
    taken: Vec<Taken>,
    next: u64,
}

impl State {
    /// A new id of the fake's.
    fn id(&mut self) -> String {
        self.next += 1;
        format!("plane:{}", self.next)
    }

    /// A message of version 1 under a new id of the fake's.
    fn message(&mut self, kind: &str, body: Value) -> Value {
        json!({ "protocol_version": 1, "id": self.id(), "body": { kind: body } })
    }

    /// `partition`'s `pulled`, its wants and the report it holds.
    fn pulled(&mut self, partition: &str) -> Value {
        let set = self.partitions.get(partition).cloned().unwrap_or_default();
        self.message(
            "pulled",
            json!({ "wants": set.wants, "report": set.report }),
        )
    }
}

struct Shared {
    state: Mutex<State>,
    changed: Condvar,
    validator: Validator,
    origin: String,
}

impl Shared {
    fn lock(&self) -> MutexGuard<'_, State> {
        self.state.lock().unwrap_or_else(|e| e.into_inner())
    }

    /// Records `taken`, and wakes whoever waits.
    fn take(&self, taken: Taken) {
        self.lock().taken.push(taken);
        self.changed.notify_all();
    }

    /// Why `message` fails version 1, where it does: each error's path
    /// in the message.
    fn refusal(&self, message: &Value) -> Option<String> {
        let errors: Vec<String> = self
            .validator
            .iter_errors(message)
            .map(|e| {
                format!(
                    "{} fails the schema's {}",
                    e.instance_path(),
                    e.schema_path()
                )
            })
            .collect();
        (!errors.is_empty()).then(|| errors.join("; "))
    }
}

/// A fake hosted control plane, running on a runtime of its own until it
/// is dropped.
pub struct Fake {
    port: u16,
    shared: Arc<Shared>,
    runtime: Option<tokio::runtime::Runtime>,
}

impl Fake {
    /// A fake listening on a port of the loopback address's own.
    pub fn start() -> Fake {
        let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("a loopback port");
        listener
            .set_nonblocking(true)
            .expect("a listener that waits on tokio");
        let port = listener.local_addr().expect("its address").port();
        let shared = Arc::new(Shared {
            state: Mutex::new(State::default()),
            changed: Condvar::new(),
            validator: from_runner(),
            origin: format!("http://127.0.0.1:{port}"),
        });
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .worker_threads(2)
            .enable_all()
            .build()
            .expect("the fake's runtime");
        let accepting = shared.clone();
        runtime.spawn(async move {
            let listener =
                tokio::net::TcpListener::from_std(listener).expect("the listener on tokio");
            while let Ok((stream, _)) = listener.accept().await {
                tokio::spawn(serve(stream, accepting.clone()));
            }
        });
        Fake {
            port,
            shared,
            runtime: Some(runtime),
        }
    }

    /// The fake's address, as a hosted table's `url` names it.
    pub fn url(&self) -> String {
        format!("http://127.0.0.1:{}/flywheel", self.port)
    }

    /// The fake's origin, which each signature names.
    pub fn origin(&self) -> String {
        self.shared.origin.clone()
    }

    /// Where a socket is opened.
    pub fn connect_address(&self) -> String {
        format!("ws://127.0.0.1:{}{BASE_PATH}runner/connect", self.port)
    }

    /// Issues `token` for `partition`, which enrols it as host `host`.
    pub fn issue(&self, partition: &str, token: &str, host: &str) {
        self.shared.lock().tokens.insert(
            token.to_string(),
            Token {
                partition: partition.to_string(),
                host: host.to_string(),
                refused: None,
                spent: None,
            },
        );
    }

    /// Refuses `token`, issued, with `words`.
    pub fn refuse_token(&self, token: &str, words: &str) {
        if let Some(token) = self.shared.lock().tokens.get_mut(token) {
            token.refused = Some(words.to_string());
        }
    }

    /// Answers the next enrolment with `status` and `answer`, whatever it
    /// holds.
    pub fn answer_enrolment(&self, status: u16, answer: Value) {
        self.shared.lock().answer = Some((status, answer));
    }

    /// Takes `key` as enrolled to `partition`, as an enrolment would.
    pub fn enrolled(&self, partition: &str, key: &VerifyingKey) {
        self.shared
            .lock()
            .keys
            .insert(key.to_bytes(), partition.to_string());
    }

    /// Each key enrolled to `partition`.
    pub fn keys(&self, partition: &str) -> Vec<[u8; 32]> {
        let state = self.shared.lock();
        state
            .keys
            .iter()
            .filter(|(_, enrolled)| enrolled.as_str() == partition)
            .map(|(key, _)| *key)
            .collect()
    }

    /// Refuses `partition`'s key at each upgrade with `words`, or takes it
    /// again where `words` is none.
    pub fn refuse_key(&self, partition: &str, words: Option<&str>) {
        let mut state = self.shared.lock();
        match words {
            Some(words) => state
                .refused_keys
                .insert(partition.to_string(), words.to_string()),
            None => state.refused_keys.remove(partition),
        };
    }

    /// Answers each upgrade without the subprotocol, or with it again.
    pub fn refuse_subprotocol(&self, refuse: bool) {
        self.shared.lock().refuse_subprotocol = refuse;
    }

    /// Closes every connection as it comes, before any byte, or takes
    /// them again.
    pub fn down(&self, down: bool) {
        self.shared.lock().down = down;
    }

    /// Sets `partition`'s wants, and sends its connection a `pulled`
    /// holding them, with no report, where it has sent its hello.
    pub fn set_wants(&self, partition: &str, wants: Value) {
        let mut state = self.shared.lock();
        state
            .partitions
            .entry(partition.to_string())
            .or_default()
            .wants = wants.clone();
        if state
            .connections
            .get(partition)
            .is_some_and(|live| live.hello)
        {
            let pulled = state.message("pulled", json!({ "wants": wants, "report": null }));
            if let Some(live) = state.connections.get(partition) {
                let _ = live.sender.send(Out::Send(pulled));
            }
        }
    }

    /// Sets the report `partition`'s control plane holds, which each
    /// `pulled` answering a hello gives.
    pub fn set_report(&self, partition: &str, report: Value) {
        self.shared
            .lock()
            .partitions
            .entry(partition.to_string())
            .or_default()
            .report = report;
    }

    /// Answers each of `partition`'s reports letting go of `let_go` and
    /// refusing each of `refused` with its words.
    pub fn answer_reports(&self, partition: &str, let_go: &[&str], refused: &[(&str, &str)]) {
        let mut state = self.shared.lock();
        let set = state.partitions.entry(partition.to_string()).or_default();
        set.let_go = let_go.iter().map(|id| id.to_string()).collect();
        set.refused = refused
            .iter()
            .map(|(id, words)| (id.to_string(), words.to_string()))
            .collect();
    }

    /// Answers none of `partition`'s reports, or each again.
    pub fn withhold_reported(&self, partition: &str, withhold: bool) {
        self.shared
            .lock()
            .partitions
            .entry(partition.to_string())
            .or_default()
            .withhold = withhold;
    }

    /// Answers each of `partition`'s reports `delay` after it comes.
    pub fn delay_reported(&self, partition: &str, delay: Duration) {
        self.shared
            .lock()
            .partitions
            .entry(partition.to_string())
            .or_default()
            .delay = delay;
    }

    /// Drops `partition`'s live connection, with no close.
    pub fn drop_connection(&self, partition: &str) {
        if let Some(live) = self.shared.lock().connections.remove(partition) {
            let _ = live.sender.send(Out::Drop);
        }
        self.shared.changed.notify_all();
    }

    /// Sends `message` on `partition`'s live connection, whatever it
    /// holds; or says there is none.
    pub fn send(&self, partition: &str, message: Value) -> bool {
        let state = self.shared.lock();
        state
            .connections
            .get(partition)
            .is_some_and(|live| live.sender.send(Out::Send(message)).is_ok())
    }

    /// Sends a message of `kind` holding `body` on `partition`'s live
    /// connection, under a new id of the fake's, which it gives.
    pub fn send_body(&self, partition: &str, kind: &str, body: Value) -> Option<String> {
        let message = self.shared.lock().message(kind, body);
        let id = message["id"].as_str().map(str::to_string);
        self.send(partition, message).then_some(id).flatten()
    }

    /// Whether `partition` has a live connection.
    pub fn connected(&self, partition: &str) -> bool {
        self.shared.lock().connections.contains_key(partition)
    }

    /// What the fake took, in order.
    pub fn taken(&self) -> Vec<Taken> {
        self.shared.lock().taken.clone()
    }

    /// What the fake took, once `until` holds of it; or the test fails,
    /// naming `what`, after ten seconds.
    pub fn wait_until(&self, what: &str, until: impl Fn(&[Taken]) -> bool) -> Vec<Taken> {
        let deadline = Instant::now() + PATIENCE;
        let mut state = self.shared.lock();
        loop {
            if until(&state.taken) {
                return state.taken.clone();
            }
            let left = deadline.saturating_duration_since(Instant::now());
            assert!(
                !left.is_zero(),
                "the fake did not see {what}: {:#?}",
                state.taken
            );
            state = self
                .shared
                .changed
                .wait_timeout(state, left)
                .unwrap_or_else(|e| e.into_inner())
                .0;
        }
    }
}

impl Drop for Fake {
    fn drop(&mut self) {
        if let Some(runtime) = self.runtime.take() {
            runtime.shutdown_background();
        }
    }
}

/// The README's context `name` over `fields`.
fn context(name: &str, fields: &[&[u8]]) -> Vec<u8> {
    let mut bytes = format!("{SUBPROTOCOL} {name}").into_bytes();
    for field in fields {
        bytes.push(0);
        bytes.extend_from_slice(field);
    }
    bytes
}

/// Whether `signature`, in base64url, is `key`'s over `context`.
fn checks(key: &[u8], signature: &[u8], context: &[u8]) -> bool {
    let Ok(key) = <[u8; 32]>::try_from(key) else {
        return false;
    };
    let Ok(signature) = <[u8; 64]>::try_from(signature) else {
        return false;
    };
    let Ok(key) = VerifyingKey::from_bytes(&key) else {
        return false;
    };
    key.verify_strict(context, &Signature::from_bytes(&signature))
        .is_ok()
}

fn decoded(text: &str) -> Option<Vec<u8>> {
    URL_SAFE_NO_PAD.decode(text).ok()
}

fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|since| since.as_secs())
        .unwrap_or_default()
}

/// Serves one connection: an upgrade of `runner/connect`, or a request
/// to `runner/enrol`.
async fn serve(stream: TcpStream, shared: Arc<Shared>) {
    if shared.lock().down {
        return;
    }
    match request_line(&stream).await {
        Some(line) if line.starts_with("GET ") => connect(stream, shared).await,
        Some(_) => enrol(stream, shared).await,
        None => {}
    }
}

/// The request's first line, read without taking it from the stream.
async fn request_line(stream: &TcpStream) -> Option<String> {
    let mut buffer = vec![0u8; 2048];
    for _ in 0..1000 {
        let n = stream.peek(&mut buffer).await.ok()?;
        if n == 0 {
            return None;
        }
        if let Some(end) = buffer[..n].iter().position(|byte| *byte == b'\n') {
            return Some(String::from_utf8_lossy(&buffer[..end]).into_owned());
        }
        if n == buffer.len() {
            return Some(String::from_utf8_lossy(&buffer[..n]).into_owned());
        }
        tokio::time::sleep(Duration::from_millis(2)).await;
    }
    None
}

/// Answers a request to `runner/enrol`, written and read by hand.
async fn enrol(stream: TcpStream, shared: Arc<Shared>) {
    let mut reader = BufReader::new(stream);
    let mut line = String::new();
    if reader.read_line(&mut line).await.is_err() {
        return;
    }
    let path = line
        .split_whitespace()
        .nth(1)
        .unwrap_or_default()
        .to_string();
    let mut length = 0usize;
    let mut bearer = None;
    loop {
        let mut header = String::new();
        match reader.read_line(&mut header).await {
            Ok(0) | Err(_) => return,
            Ok(_) => {}
        }
        let header = header.trim_end();
        if header.is_empty() {
            break;
        }
        let Some((name, value)) = header.split_once(':') else {
            continue;
        };
        match name.trim().to_ascii_lowercase().as_str() {
            "content-length" => length = value.trim().parse().unwrap_or_default(),
            "authorization" => bearer = value.trim().strip_prefix("Bearer ").map(str::to_string),
            _ => {}
        }
    }
    let mut body = vec![0u8; length];
    if reader.read_exact(&mut body).await.is_err() {
        return;
    }
    let mut stream = reader.into_inner();
    if !line.starts_with("POST ") || path != format!("{BASE_PATH}runner/enrol") {
        let _ = answer(&mut stream, 404, "no such path".to_string()).await;
        return;
    }

    let message: Value = serde_json::from_slice(&body).unwrap_or(Value::Null);
    let refused = shared.refusal(&message);
    let (status, signed, reply) = enrolment(&shared, &message, &refused, bearer.as_deref());
    shared.take(Taken::Enrol {
        bearer,
        message,
        refused,
        signed,
        status,
    });
    let _ = answer(&mut stream, status, reply.to_string()).await;
}

/// The enrolment's status, whether its signature checked, and the fake's
/// answer.
fn enrolment(
    shared: &Shared,
    message: &Value,
    refused: &Option<String>,
    bearer: Option<&str>,
) -> (u16, bool, Value) {
    let mut state = shared.lock();
    let id = message["id"].as_str().unwrap_or("unread").to_string();
    if let Some(words) = refused {
        let reply = state.message("message_refused", json!({ "message": id, "words": words }));
        return (400, false, reply);
    }
    if kind(message).as_deref() != Some("enrol") {
        let words = "runner/enrol takes an enrol alone";
        let reply = state.message("message_refused", json!({ "message": id, "words": words }));
        return (400, false, reply);
    }
    let enrol = &message["body"]["enrol"];
    let key = decoded(enrol["key"].as_str().unwrap_or_default()).unwrap_or_default();
    let signature = decoded(enrol["signature"].as_str().unwrap_or_default()).unwrap_or_default();
    let token = bearer.unwrap_or_default();
    let digest: String = Sha256::digest(token.as_bytes())
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect();
    let signed = checks(
        &key,
        &signature,
        &context(
            "enrol",
            &[
                shared.origin.as_bytes(),
                format!("sha256:{digest}").as_bytes(),
                &key,
            ],
        ),
    );
    if let Some((status, reply)) = state.answer.take() {
        return (status, signed, reply);
    }
    let refuse = |state: &mut State, words: &str| {
        let reply = state.message("enrol_refused", json!({ "enrol": id, "words": words }));
        (401, signed, reply)
    };
    if bearer.is_none() {
        return refuse(&mut state, "no enrolment token was borne");
    }
    if !signed {
        return refuse(&mut state, "the enrolment's signature does not check");
    }
    let key: [u8; 32] = key.try_into().unwrap_or([0; 32]);
    let Some(issued) = state.tokens.get(token) else {
        return refuse(&mut state, "the token is none this control plane issued");
    };
    if let Some(words) = issued.refused.clone() {
        return refuse(&mut state, &words);
    }
    match issued.spent.clone() {
        Some(spent) if spent != (id.clone(), key) => {
            return refuse(&mut state, "the token was spent");
        }
        _ => {}
    }
    let (partition, host) = (issued.partition.clone(), issued.host.clone());
    if let Some(issued) = state.tokens.get_mut(token) {
        issued.spent = Some((id, key));
    }
    state.keys.insert(key, partition);
    let reply = state.message("enrolled", json!({ "host": host }));
    (200, signed, reply)
}

/// Writes an HTTP answer of `status` holding `body`, and closes.
async fn answer(stream: &mut TcpStream, status: u16, body: String) -> std::io::Result<()> {
    let reason = match status {
        200 => "OK",
        400 => "Bad Request",
        401 => "Unauthorized",
        403 => "Forbidden",
        404 => "Not Found",
        _ => "Answered",
    };
    let head = format!(
        "HTTP/1.1 {status} {reason}\r\nContent-Type: application/json\r\n\
         Content-Length: {}\r\nConnection: close\r\n\r\n",
        body.len()
    );
    stream.write_all(head.as_bytes()).await?;
    stream.write_all(body.as_bytes()).await?;
    stream.shutdown().await
}

/// What an upgrade proved: the partition, or the status and words it is
/// refused with.
fn upgrade(shared: &Shared, request: &Request) -> (Taken, Result<String, (u16, String)>) {
    let protocols: Vec<String> = request
        .headers()
        .get_all("sec-websocket-protocol")
        .iter()
        .filter_map(|value| value.to_str().ok())
        .flat_map(|value| value.split(','))
        .map(|protocol| protocol.trim().to_string())
        .collect();
    let authorization = request
        .headers()
        .get("authorization")
        .and_then(|value| value.to_str().ok())
        .map(str::to_string);
    let parts: Vec<&str> = authorization
        .as_deref()
        .and_then(|value| value.strip_prefix("FlywheelHost "))
        .map(|statement| statement.split('.').collect())
        .unwrap_or_default();
    let (key, time, nonce, signature) = match parts.as_slice() {
        [key, time, nonce, signature] => (
            decoded(key).and_then(|key| <[u8; 32]>::try_from(key).ok()),
            Some(*time),
            decoded(nonce),
            decoded(signature),
        ),
        _ => (None, None, None, None),
    };
    let mut taken = Taken::Upgrade {
        partition: None,
        protocols: protocols.clone(),
        authorization: authorization.clone(),
        key,
        nonce: nonce.clone(),
        status: 101,
    };

    let mut state = shared.lock();
    let decision = (|| {
        if request.uri().path() != format!("{BASE_PATH}runner/connect") {
            return Err((404, "no such path".to_string()));
        }
        if !protocols.iter().any(|protocol| protocol == SUBPROTOCOL) {
            return Err((
                400,
                format!("{SUBPROTOCOL} is the version this control plane speaks"),
            ));
        }
        let (Some(key), Some(time), Some(nonce), Some(signature)) =
            (key, time, nonce.clone(), signature)
        else {
            return Err((401, "the upgrade holds no statement".to_string()));
        };
        let context = context(
            "connect",
            &[shared.origin.as_bytes(), &key, time.as_bytes(), &nonce],
        );
        if nonce.len() != 16 || !checks(&key, &signature, &context) {
            return Err((401, "the statement's signature does not check".to_string()));
        }
        let at: u64 = time.parse().unwrap_or_default();
        if now().abs_diff(at) > WINDOW {
            return Err((
                401,
                "the statement's time is not within 300 seconds of the control plane's clock"
                    .to_string(),
            ));
        }
        if !state.nonces.insert((key, nonce)) {
            return Err((401, "the statement's nonce was seen".to_string()));
        }
        let Some(partition) = state.keys.get(&key).cloned() else {
            return Err((401, "this key is enrolled to no host".to_string()));
        };
        if let Some(words) = state.refused_keys.get(&partition) {
            return Err((401, words.clone()));
        }
        Ok(partition)
    })();
    if let Taken::Upgrade {
        partition, status, ..
    } = &mut taken
    {
        match &decision {
            Ok(proved) => *partition = Some(proved.clone()),
            Err((refused, _)) => *status = *refused,
        }
    }
    drop(state);
    (taken, decision)
}

/// Upgrades `runner/connect`, and serves the partition's connection.
async fn connect(stream: TcpStream, shared: Arc<Shared>) {
    let mut proved = None;
    // tungstenite's handshake gives the callback's refusal its type.
    #[allow(clippy::result_large_err)]
    let callback = |request: &Request, mut response: Response| {
        let (taken, decision) = upgrade(&shared, request);
        shared.take(taken);
        match decision {
            Ok(partition) => {
                if !shared.lock().refuse_subprotocol {
                    response.headers_mut().insert(
                        "sec-websocket-protocol",
                        http::HeaderValue::from_static(SUBPROTOCOL),
                    );
                }
                proved = Some(partition);
                Ok(response)
            }
            Err((status, words)) => {
                let refusal: ErrorResponse = http::Response::builder()
                    .status(status)
                    .body(Some(words))
                    .unwrap_or_default();
                Err(refusal)
            }
        }
    };
    let Ok(socket) = tokio_tungstenite::accept_hdr_async(stream, callback).await else {
        return;
    };
    let Some(partition) = proved else {
        return;
    };

    let (sender, mut outgoing) = mpsc::unbounded_channel();
    let number = {
        let mut state = shared.lock();
        state.next += 1;
        let number = state.next;
        if let Some(earlier) = state.connections.insert(
            partition.clone(),
            Live {
                number,
                sender: sender.clone(),
                hello: false,
            },
        ) {
            let _ = earlier.sender.send(Out::Drop);
        }
        number
    };
    shared.changed.notify_all();

    let (mut sink, mut frames) = socket.split();
    loop {
        tokio::select! {
            frame = frames.next() => match frame {
                Some(Ok(Frame::Text(text))) => {
                    answer_message(&shared, &partition, text.as_str(), &sender);
                }
                Some(Ok(Frame::Ping(_))) => shared.take(Taken::Ping { partition: partition.clone() }),
                Some(Ok(Frame::Close(_))) | Some(Err(_)) | None => {
                    shared.take(Taken::Closed { partition: partition.clone() });
                    break;
                }
                Some(Ok(_)) => {}
            },
            out = outgoing.recv() => match out {
                Some(Out::Send(message)) => {
                    if sink.send(Frame::text(message.to_string())).await.is_err() {
                        break;
                    }
                }
                Some(Out::Drop) | None => break,
            },
        }
    }
    let mut state = shared.lock();
    if state
        .connections
        .get(&partition)
        .is_some_and(|live| live.number == number)
    {
        state.connections.remove(&partition);
    }
    drop(state);
    shared.changed.notify_all();
}

/// Takes `text`, a message `partition`'s connection sent, and answers it
/// as a control plane does.
fn answer_message(
    shared: &Arc<Shared>,
    partition: &str,
    text: &str,
    sender: &mpsc::UnboundedSender<Out>,
) {
    let message: Value = serde_json::from_str(text).unwrap_or(Value::Null);
    let refused = shared.refusal(&message);
    shared.take(Taken::Message {
        partition: partition.to_string(),
        message: message.clone(),
        refused: refused.clone(),
    });
    let mut state = shared.lock();
    let id = message["id"].clone();
    if let Some(words) = refused {
        if id.is_string() {
            let reply = state.message("message_refused", json!({ "message": id, "words": words }));
            let _ = sender.send(Out::Send(reply));
        }
        return;
    }
    match kind(&message).as_deref() {
        Some("hello") => {
            if let Some(live) = state.connections.get_mut(partition) {
                live.hello = true;
            }
            let pulled = state.pulled(partition);
            let _ = sender.send(Out::Send(pulled));
        }
        Some("report") => {
            let set = state.partitions.get(partition).cloned().unwrap_or_default();
            if set.withhold {
                return;
            }
            let reported = state.message(
                "reported",
                json!({ "report": id, "let_go": set.let_go, "refused": set.refused }),
            );
            let sender = sender.clone();
            tokio::spawn(async move {
                tokio::time::sleep(set.delay).await;
                let _ = sender.send(Out::Send(reported));
            });
        }
        _ => {}
    }
}
