//! flume host, Flume's resident, as the runner asks it: by its socket and
//! never by a command (residents-ask-each-other-by-socket; core.13). While
//! the runner carries a control plane's parts it holds one subscription,
//! `{"ask":"subscribe","repositories":[…]}`, naming the folder of each
//! clone the passes read, and keeps the latest document, which each pass
//! reads as Flume's status; a document that differs from the one held
//! wakes every part (one-subscription-per-plugin). A read that names other
//! folders than the subscription asked makes it ask again, naming them.
//! `boomdrive run --once` asks `{"ask":"status"}` once for each read,
//! naming the folders it reads, and holds no subscription. An ask that
//! names no folder names no `repositories`.
//!
//! The socket is where flume host keeps it: `host-<key>.sock` in the
//! user's own directory, `/tmp` resolved and `flume-<uid>`, or
//! `$FLUME_USER_SCRATCH` where set, the key the first 10 hex digits of the
//! SHA-1 of flume's state folder, `$XDG_STATE_HOME/flume` or
//! `~/.local/state/flume`, resolved. Each line is one JSON object: the
//! runner says `{"hello":"boomdrive/<version>"}`, takes flume host's answer
//! only where it names `protocol` 1, then asks. Each `{"document":…}` holds
//! what `flume --json status` prints, and the runner keeps its bytes as
//! flume host sent them (core.11, core.27). A refusal in the document is
//! Flume's refusal, as the command's exit 1 was.
//!
//! Where no flume host answers, Flume's status is unread, and the runner
//! tries again on a bounded backoff, saying one line when flume host stops
//! answering and one when it answers again (core.22, core.23). It starts
//! no flume host, which is the host's service manager's
//! (the-service-manager-starts-each-resident).

use crate::flume::{Act, Cli, Flume, LaunchArgs, Reply, Unrun};
use serde::Deserialize;
use serde_json::value::RawValue;
use std::ffi::OsString;
use std::io::{BufRead, BufReader, ErrorKind, Write};
use std::os::unix::net::UnixStream;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Condvar, Mutex, MutexGuard};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

/// The variable that names the user's own directory, which flume host's
/// socket is in, in place of `/tmp`'s.
pub const USER_SCRATCH: &str = "FLUME_USER_SCRATCH";

/// The protocol the runner speaks with flume host.
pub const PROTOCOL: u64 = 1;

/// How long the runner waits for flume host to answer its hello.
pub const HELLO_WAIT: Duration = Duration::from_secs(10);

/// How long the runner waits for flume host's answer to a status ask.
const ANSWER_WAIT: Duration = Duration::from_secs(30);

/// How long a read waits for the subscription's first document.
const FIRST_WAIT: Duration = Duration::from_secs(10);

/// How often the subscription's thread looks whether the runner stopped.
const TICK: Duration = Duration::from_millis(200);

/// The subscription's first wait before it tries again, and its longest
/// (core.22).
const BACKOFF_FIRST: Duration = Duration::from_secs(1);
const BACKOFF_MOST: Duration = Duration::from_secs(60);

/// flume host's socket for the process whose environment `var` reads, or
/// the words saying why there is none.
pub fn socket_from(var: impl Fn(&str) -> Option<OsString>) -> Result<PathBuf, String> {
    let state = match var("XDG_STATE_HOME").filter(|dir| !dir.is_empty()) {
        Some(dir) => PathBuf::from(dir),
        None => PathBuf::from(var("HOME").ok_or("HOME is not set")?)
            .join(".local")
            .join("state"),
    }
    .join("flume");
    let state = state.canonicalize().map_err(|error| {
        format!(
            "flume's state folder {} is not there ({error}): flume has never run as this user",
            state.display()
        )
    })?;
    let user = match var(USER_SCRATCH).filter(|dir| !dir.is_empty()) {
        Some(dir) => PathBuf::from(dir),
        None => {
            let tmp = Path::new("/tmp")
                .canonicalize()
                .map_err(|error| format!("resolving /tmp: {error}"))?;
            // SAFETY: geteuid reads the process's effective user and
            // cannot fail.
            tmp.join(format!("flume-{}", unsafe { libc::geteuid() }))
        }
    };
    Ok(user.join(format!("host-{}.sock", key(&state))))
}

/// Where the runner finds flume host's socket: by the rule of
/// [`socket_from`] over the environment it started in, read again at each
/// try, since flume's state folder may be made after the runner starts;
/// or at a path a test gives.
#[derive(Clone, Debug)]
pub enum Where {
    Env(Vec<(String, OsString)>),
    At(PathBuf),
}

impl Where {
    /// The variables of `var` that [`socket_from`] reads.
    pub fn from_env(var: impl Fn(&str) -> Option<OsString>) -> Where {
        Where::Env(
            ["XDG_STATE_HOME", "HOME", USER_SCRATCH]
                .into_iter()
                .filter_map(|name| var(name).map(|value| (name.to_string(), value)))
                .collect(),
        )
    }

    /// The socket, or why there is none.
    pub fn socket(&self) -> Result<PathBuf, String> {
        match self {
            Where::At(socket) => Ok(socket.clone()),
            Where::Env(vars) => socket_from(|name| {
                vars.iter()
                    .find(|(key, _)| key == name)
                    .map(|(_, value)| value.clone())
            }),
        }
    }
}

/// flume's key of a folder: the first 10 hex digits of the SHA-1 of its
/// path's bytes.
pub fn key(folder: &Path) -> String {
    use sha1::{Digest, Sha1};
    use std::os::unix::ffi::OsStrExt as _;
    Sha1::digest(folder.as_os_str().as_bytes())
        .iter()
        .take(5)
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

/// flume host's answer to a hello.
#[derive(Deserialize)]
struct Hello {
    protocol: u64,
}

/// One line flume host sends after the hello.
#[derive(Deserialize)]
struct Line<'a> {
    #[serde(borrow)]
    document: Option<&'a RawValue>,
    error: Option<String>,
}

/// A document that is a refusal, as `flume --json status` prints one.
#[derive(Deserialize)]
struct Refused {
    refused: Refusal,
}

#[derive(Deserialize)]
struct Refusal {
    refusal: Reason,
}

#[derive(Deserialize)]
struct Reason {
    reason: String,
}

/// The document `raw` as the runner reads Flume's status: Flume's
/// refusal where it is one, else its text as flume host sent it.
pub fn reply(raw: &str) -> Reply {
    match serde_json::from_str::<Refused>(raw) {
        Ok(refused) => Reply::Refused(refused.refused.refusal.reason),
        Err(_) => Reply::Given(raw.to_string()),
    }
}

/// A connection to flume host whose hello was answered, read line by line.
struct Connection {
    socket: PathBuf,
    stream: UnixStream,
    reader: BufReader<UnixStream>,
    /// The bytes of a line read so far, kept across read timeouts.
    partial: Vec<u8>,
}

/// What reading a line gave.
enum Read {
    Line(String),
    /// No whole line within the read's timeout.
    Waiting,
    Ended(String),
}

impl Connection {
    /// Connects to `socket`, says the runner's hello and takes flume
    /// host's answer, within [`HELLO_WAIT`].
    fn open(socket: &Path) -> Result<Connection, String> {
        let stream = UnixStream::connect(socket)
            .map_err(|error| format!("no flume host answers on {}: {error}", socket.display()))?;
        let reader = BufReader::new(
            stream
                .try_clone()
                .map_err(|error| format!("{}: {error}", socket.display()))?,
        );
        let mut connection = Connection {
            socket: socket.to_path_buf(),
            stream,
            reader,
            partial: Vec::new(),
        };
        connection.send(&serde_json::json!({
            "hello": format!("boomdrive/{}", env!("CARGO_PKG_VERSION"))
        }))?;
        let line = connection.line_within(HELLO_WAIT, "its hello")?;
        let hello: Hello = serde_json::from_str(&line).map_err(|error| {
            format!(
                "flume host on {} answered the hello with a line the runner cannot read ({error}): {line}",
                socket.display()
            )
        })?;
        if hello.protocol != PROTOCOL {
            return Err(format!(
                "flume host on {} speaks protocol {}, and the runner speaks protocol {PROTOCOL}",
                socket.display(),
                hello.protocol
            ));
        }
        Ok(connection)
    }

    fn send(&mut self, line: &serde_json::Value) -> Result<(), String> {
        let mut text = line.to_string();
        text.push('\n');
        self.stream.write_all(text.as_bytes()).map_err(|error| {
            format!(
                "writing to flume host on {}: {error}",
                self.socket.display()
            )
        })
    }

    /// One line, waiting `wait` at most; one that does not come in time
    /// names `what` it answers.
    fn line_within(&mut self, wait: Duration, what: &str) -> Result<String, String> {
        let deadline = Instant::now() + wait;
        loop {
            let left = deadline.saturating_duration_since(Instant::now());
            if left.is_zero() {
                return Err(format!(
                    "flume host on {} did not answer {what} within {}s",
                    self.socket.display(),
                    wait.as_secs()
                ));
            }
            match self.read(left.min(TICK)) {
                Read::Line(line) => return Ok(line),
                Read::Waiting => {}
                Read::Ended(words) => return Err(words),
            }
        }
    }

    /// One line, waiting `wait` at most for it.
    fn read(&mut self, wait: Duration) -> Read {
        if let Err(error) = self
            .stream
            .set_read_timeout(Some(wait.max(Duration::from_millis(1))))
        {
            return Read::Ended(format!("{}: {error}", self.socket.display()));
        }
        match self.reader.read_until(b'\n', &mut self.partial) {
            Ok(0) => Read::Ended(format!(
                "flume host on {} closed the connection",
                self.socket.display()
            )),
            Ok(_) if self.partial.ends_with(b"\n") => {
                let line = String::from_utf8_lossy(&self.partial)
                    .trim_end()
                    .to_string();
                self.partial.clear();
                Read::Line(line)
            }
            Ok(_) => Read::Waiting,
            Err(error) if matches!(error.kind(), ErrorKind::WouldBlock | ErrorKind::TimedOut) => {
                Read::Waiting
            }
            Err(error) => Read::Ended(format!(
                "reading from flume host on {}: {error}",
                self.socket.display()
            )),
        }
    }

    /// The document of a line after the ask, or why the line ends what
    /// was asked.
    fn document(&self, line: &str) -> Result<Reply, String> {
        let read: Line<'_> = serde_json::from_str(line).map_err(|error| {
            format!(
                "flume host on {} sent a line the runner cannot read ({error}): {line}",
                self.socket.display()
            )
        })?;
        match (read.document, read.error) {
            (Some(document), _) => Ok(reply(document.get())),
            (None, Some(words)) => Err(format!("flume host refused the ask: {words}")),
            (None, None) => Err(format!(
                "flume host on {} sent a line with no document: {line}",
                self.socket.display()
            )),
        }
    }
}

/// The ask `ask`, `status` or `subscribe`, naming each of `repositories`
/// by its clone's folder, and no `repositories` where it names none.
pub fn ask(ask: &str, repositories: &[String]) -> serde_json::Value {
    match repositories {
        [] => serde_json::json!({ "ask": ask }),
        named => serde_json::json!({ "ask": ask, "repositories": named }),
    }
}

/// Flume's status, asked once of flume host on `socket`:
/// `{"ask":"status"}`, naming each of `repositories` by its clone's
/// folder, as `boomdrive run --once` reads it.
pub fn status(socket: &Path, repositories: &[String]) -> Result<Reply, Unrun> {
    let mut connection = Connection::open(socket).map_err(Unrun)?;
    connection
        .send(&ask("status", repositories))
        .map_err(Unrun)?;
    let line = connection
        .line_within(ANSWER_WAIT, "the status ask")
        .map_err(Unrun)?;
    connection.document(&line).map_err(Unrun)
}

/// What the subscription holds.
enum Held {
    /// No document yet since the runner started.
    Starting,
    /// The latest document, and the folders its subscription named.
    Document(Reply, Vec<String>),
    /// The subscription is down, and why.
    Down(String),
}

/// What the subscription holds, and the folders the last read named, none
/// before the first read.
struct State {
    held: Held,
    named: Option<Vec<String>>,
}

/// The runner's one subscription to flume host: the latest document, held
/// for every part's reads.
pub struct Subscription {
    socket: Where,
    state: Mutex<State>,
    changed: Condvar,
}

/// How a subscription ended.
enum Ended {
    /// The runner stopped.
    Stopped,
    /// A read named other folders than it asked, so it asks again at once.
    Named,
    /// flume host stopped answering, and why.
    Down(String),
}

impl Subscription {
    /// A subscription to flume host on `socket`, holding nothing yet.
    pub fn new(socket: Where) -> Subscription {
        Subscription {
            socket,
            state: Mutex::new(State {
                held: Held::Starting,
                named: None,
            }),
            changed: Condvar::new(),
        }
    }

    fn state(&self) -> MutexGuard<'_, State> {
        self.state.lock().unwrap_or_else(|e| e.into_inner())
    }

    /// Flume's status, the latest document, with Flume's status of each of
    /// `repositories`, by its clone's folder: waiting up to ten seconds for
    /// the first document, or for one that names `repositories` where the
    /// subscription named others, and unread while the subscription is
    /// down. A document that names other folders is read as it stands.
    pub fn latest(&self, repositories: &[String]) -> Result<Reply, Unrun> {
        let mut state = self.state();
        if state.named.as_deref() != Some(repositories) {
            state.named = Some(repositories.to_vec());
        }
        let (state, _) = self
            .changed
            .wait_timeout_while(state, FIRST_WAIT, |state| match &state.held {
                Held::Starting => true,
                Held::Document(_, asked) => state.named.as_ref() != Some(asked),
                Held::Down(_) => false,
            })
            .unwrap_or_else(|e| e.into_inner());
        match &state.held {
            Held::Document(reply, _) => Ok(reply.clone()),
            Held::Down(words) => Err(Unrun(words.clone())),
            Held::Starting => Err(Unrun(
                "flume host has sent no status since the runner started".to_string(),
            )),
        }
    }

    /// The folders the last read named, none before the first read.
    fn named(&self) -> Option<Vec<String>> {
        self.state().named.clone()
    }

    /// Takes `reply`, the latest document of the subscription that named
    /// `asked`, and whether it wakes the parts: where it differs from the
    /// document held of the same ask, or comes after flume host stopped
    /// answering. The first document, and the first of an ask that names
    /// other folders, wake none, since the read that named them waits for
    /// it.
    fn heard(&self, reply: Reply, asked: &[String]) -> bool {
        let mut state = self.state();
        let changed = match &state.held {
            Held::Starting => false,
            Held::Document(was, was_asked) => was_asked == asked && *was != reply,
            Held::Down(_) => true,
        };
        state.held = Held::Document(reply, asked.to_vec());
        self.changed.notify_all();
        changed
    }

    fn down(&self, words: &str) {
        self.state().held = Held::Down(words.to_string());
        self.changed.notify_all();
    }

    /// Holds the subscription until `stop` is set, from the first read,
    /// which names the folders it asks: each document that differs from
    /// the one held is kept and calls `wake`; a read that names other
    /// folders makes it ask again at once, naming them; a subscription
    /// that ends, or cannot be made, is tried again on a bounded backoff.
    /// It says one line when flume host stops answering and one when it
    /// answers again (core.22, core.23).
    pub fn hold(&self, stop: &AtomicBool, wake: &dyn Fn(), say: &dyn Fn(String)) {
        let mut backoff = BACKOFF_FIRST;
        let mut said_down = false;
        while !stop.load(Ordering::SeqCst) {
            let Some(named) = self.named() else {
                std::thread::sleep(TICK);
                continue;
            };
            let ended = self.follow(&named, stop, wake, &mut |answered| {
                backoff = BACKOFF_FIRST;
                if answered && std::mem::replace(&mut said_down, false) {
                    say("boomdrive: flume host answers again".to_string());
                }
            });
            let why = match ended {
                Ended::Stopped => break,
                Ended::Named => continue,
                Ended::Down(why) => why,
            };
            self.down(&why);
            if !std::mem::replace(&mut said_down, true) {
                say(format!("boomdrive: flume host does not answer: {why}"));
            }
            sleep_unless(stop, jittered(backoff));
            backoff = (backoff * 2).min(BACKOFF_MOST);
        }
    }

    /// One subscription naming `named`, followed until it ends, giving
    /// why, a read names other folders, or `stop` is set. `answered` is
    /// called at each document.
    fn follow(
        &self,
        named: &[String],
        stop: &AtomicBool,
        wake: &dyn Fn(),
        answered: &mut dyn FnMut(bool),
    ) -> Ended {
        let socket = match self.socket.socket() {
            Ok(socket) => socket,
            Err(words) => return Ended::Down(words),
        };
        let mut connection = match Connection::open(&socket) {
            Ok(connection) => connection,
            Err(words) => return Ended::Down(words),
        };
        if let Err(words) = connection.send(&ask("subscribe", named)) {
            return Ended::Down(words);
        }
        while !stop.load(Ordering::SeqCst) {
            if self.named().as_deref() != Some(named) {
                return Ended::Named;
            }
            match connection.read(TICK) {
                Read::Waiting => {}
                Read::Ended(words) => return Ended::Down(words),
                Read::Line(line) => match connection.document(&line) {
                    Ok(reply) => {
                        answered(true);
                        if self.heard(reply, named) {
                            wake();
                        }
                    }
                    Err(words) => return Ended::Down(words),
                },
            }
        }
        Ended::Stopped
    }
}

/// `wait`, give or take a quarter, so runners started together do not try
/// again together (core.22).
fn jittered(wait: Duration) -> Duration {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|now| now.subsec_nanos())
        .unwrap_or_default();
    let quarter = wait / 4;
    let span = quarter.as_nanos().max(1) * 2;
    wait - quarter + Duration::from_nanos((u128::from(nanos) % span) as u64)
}

/// Sleeps `wait`, ending at once where `stop` is set.
fn sleep_unless(stop: &AtomicBool, wait: Duration) {
    let deadline = Instant::now() + wait;
    while !stop.load(Ordering::SeqCst) {
        let left = deadline.saturating_duration_since(Instant::now());
        if left.is_zero() {
            return;
        }
        std::thread::sleep(left.min(TICK));
    }
}

/// Where a pass reads Flume's status: the runner's subscription, or one
/// status ask of flume host's socket.
pub enum Source<'a> {
    Subscription(&'a Subscription),
    Ask(&'a Where),
}

/// Flume as the running runner reaches it: its status from flume host's
/// socket, and every command of Flume's by its command line (the change
/// the-runner-hears-flume-by-subscription, Decision 6).
pub struct Resident<'a> {
    pub cli: &'a Cli,
    pub source: Source<'a>,
}

impl Flume for Resident<'_> {
    fn launch(&self, args: &LaunchArgs, brief_dir: &Path) -> Result<Reply, Unrun> {
        self.cli.launch(args, brief_dir)
    }

    fn act(&self, place: &str, act: &Act) -> Result<Reply, Unrun> {
        self.cli.act(place, act)
    }

    fn status(&self, repositories: &[String]) -> Result<Reply, Unrun> {
        match &self.source {
            Source::Subscription(subscription) => subscription.latest(repositories),
            Source::Ask(socket) => status(&socket.socket().map_err(Unrun)?, repositories),
        }
    }

    fn status_of(&self, container_dir: Option<&Path>, place: &str) -> Result<Reply, Unrun> {
        self.cli.status_of(container_dir, place)
    }

    fn confirm(&self, recipient: &str, keys: &[String]) -> Result<Reply, Unrun> {
        self.cli.confirm(recipient, keys)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    fn env(pairs: &[(&str, &Path)]) -> impl Fn(&str) -> Option<OsString> {
        let map: BTreeMap<String, OsString> = pairs
            .iter()
            .map(|(name, value)| (name.to_string(), value.as_os_str().to_os_string()))
            .collect();
        move |name| map.get(name).cloned()
    }

    /// The running spec's "The socket of a scratch home".
    #[test]
    fn the_socket_of_a_scratch_home() {
        let dir = tempfile::tempdir().unwrap();
        let (state, scratch) = (dir.path().join("state"), dir.path().join("scratch"));
        std::fs::create_dir_all(state.join("flume")).unwrap();
        let socket =
            socket_from(env(&[("XDG_STATE_HOME", &state), (USER_SCRATCH, &scratch)])).unwrap();
        let resolved = state.join("flume").canonicalize().unwrap();
        assert_eq!(
            socket,
            scratch.join(format!("host-{}.sock", key(&resolved)))
        );
        assert_eq!(key(&resolved).len(), 10);
    }

    #[test]
    fn a_state_folder_not_there() {
        let dir = tempfile::tempdir().unwrap();
        let words = socket_from(env(&[("XDG_STATE_HOME", dir.path())])).unwrap_err();
        assert!(
            words.contains("flume has never run as this user"),
            "{words}"
        );
    }

    #[test]
    fn the_key_is_sha1s() {
        // The first five bytes of the SHA-1 of "/a".
        assert_eq!(key(Path::new("/a")), "2256c6ac80");
    }

    /// The running spec's "A refusal in the document".
    #[test]
    fn a_refusal_in_the_document() {
        assert_eq!(
            reply(r#"{"refused":{"refusal":{"reason":"no store"}}}"#),
            Reply::Refused("no store".to_string())
        );
        let given = r#"{"status":{"places":[],"host":{"room":null}}}"#;
        assert_eq!(reply(given), Reply::Given(given.to_string()));
    }
}
