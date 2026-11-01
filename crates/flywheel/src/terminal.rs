//! The terminal relay (design Decision 16; ledger item N1.b): a terminal
//! the control plane opens on a launch's agent, relayed between its link
//! and the herdr pane Flume's status names, through herdr's command line,
//! the one way the runner reaches herdr (record 0001). A terminal opens
//! only on a launch the report holds under the terminal's lease number,
//! and runs on a thread of its own, since it runs herdr and never git. It
//! sends the pane's screen when it changes and passes in what the far end
//! sends. Nothing of either is kept: the thread holds only the last
//! screen sent, to compare against, and its lines name the terminal, its
//! launch and the pane, never what passed (core.11; cloud core.18). It
//! reaches its far end over no connection but the control plane's own
//! (core.3).

use crate::facts::{self, Pane};
use flywheel_control::{Id, Input, Link, Report, Terminal};
use serde::Deserialize;
use serde_json::Value;
use std::cell::RefCell;
use std::fmt;
use std::path::PathBuf;
use std::process::{Command, Output, Stdio};
use std::sync::mpsc::{self, Receiver, Sender, TryRecvError};
use std::sync::Arc;
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

/// How often an open terminal reads its pane's screen.
pub const READ_INTERVAL: Duration = Duration::from_millis(250);

/// Why herdr did not do what it was asked: the `message` of herdr's
/// `error` answer, as given, its standard error where it printed none, or
/// the runner's words where herdr gave no answer.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Refused(pub String);

impl fmt::Display for Refused {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// herdr as the runner reaches it: `Cli` on a host, a stub in a test.
/// Each call names the pane and points herdr at the pane's session.
pub trait Herdr: Send + Sync {
    /// `herdr pane read <pane> --source visible --format ansi`: the pane's
    /// visible screen, as herdr printed it.
    fn read(&self, pane: &Pane) -> Result<String, Refused>;
    /// `herdr pane send-text <pane> <text>`.
    fn text(&self, pane: &Pane, text: &str) -> Result<(), Refused>;
    /// `herdr pane send-keys <pane> <keys>…`.
    fn keys(&self, pane: &Pane, keys: &[String]) -> Result<(), Refused>;
}

/// herdr's command line, by the program at `program`: `herdr` on the
/// host's `PATH` in the binary, a stub in a test. Every call runs with
/// `HERDR_SOCKET_PATH` set to the pane's session's socket, as Flume
/// points herdr at it.
#[derive(Clone, Debug)]
pub struct Cli {
    program: PathBuf,
}

impl Cli {
    pub fn new(program: impl Into<PathBuf>) -> Cli {
        Cli {
            program: program.into(),
        }
    }

    /// herdr's output from `pane <args>…` on `pane`, where it exited 0.
    fn run(&self, pane: &Pane, args: &[&str]) -> Result<Output, Refused> {
        let output = Command::new(&self.program)
            .arg("pane")
            .args(args)
            .env("HERDR_SOCKET_PATH", &pane.socket_file)
            .stdin(Stdio::null())
            .output()
            .map_err(|error| Refused(format!("herdr could not run: {error}")))?;
        match output.status.code() {
            Some(0) => Ok(output),
            Some(_) => Err(Refused(refusal(&output))),
            None => Err(Refused(format!("herdr was ended: {}", output.status))),
        }
    }
}

impl Herdr for Cli {
    fn read(&self, pane: &Pane) -> Result<String, Refused> {
        let args = ["read", &pane.id, "--source", "visible", "--format", "ansi"];
        let output = self.run(pane, &args)?;
        Ok(String::from_utf8_lossy(&output.stdout).into_owned())
    }

    fn text(&self, pane: &Pane, text: &str) -> Result<(), Refused> {
        self.run(pane, &["send-text", &pane.id, text]).map(drop)
    }

    fn keys(&self, pane: &Pane, keys: &[String]) -> Result<(), Refused> {
        let mut args = vec!["send-keys", pane.id.as_str()];
        args.extend(keys.iter().map(String::as_str));
        self.run(pane, &args).map(drop)
    }
}

/// herdr's `error` answer, as it prints one on standard error.
#[derive(Deserialize)]
struct Answered {
    error: Error,
}

#[derive(Deserialize)]
struct Error {
    message: String,
}

/// The words of herdr's refusal: the `message` of the `error` answer it
/// printed on standard error, as given, or, where it printed none, its
/// standard error, with its one final newline left off.
fn refusal(output: &Output) -> String {
    let printed = String::from_utf8_lossy(&output.stderr);
    match serde_json::from_str::<Answered>(&printed) {
        Ok(answered) => answered.error.message,
        Err(_) => printed.strip_suffix('\n').unwrap_or(&printed).to_string(),
    }
}

/// Where a line of the relay's goes: standard error on a host.
type Say = Arc<dyn Fn(String) + Send + Sync>;

/// The terminals the runner relays, each on a thread of its own. It is
/// kept across control passes, so a pass can close the terminals on a
/// launch it lets go. Dropped, it closes every terminal still open, and
/// waits for each thread to end.
pub struct Relays {
    herdr: Arc<dyn Herdr>,
    say: Say,
    open: RefCell<Vec<Relay>>,
}

/// An open terminal, as the runner holds it.
struct Relay {
    id: Id,
    launch: Id,
    /// Tells the terminal's thread to close, with the words why.
    closing: Sender<String>,
    thread: JoinHandle<()>,
}

impl Relays {
    /// Terminals relayed through `herdr`, each of their lines said to
    /// `say`.
    pub fn new(
        herdr: impl Herdr + 'static,
        say: impl Fn(String) + Send + Sync + 'static,
    ) -> Relays {
        Relays {
            herdr: Arc::new(herdr),
            say: Arc::new(say),
            open: RefCell::new(Vec::new()),
        }
    }

    /// Opens `terminal` where `report` holds its launch under its lease
    /// number and the launch's carried status names its agent's pane, and
    /// relays it on a thread of its own. Otherwise closes its link at
    /// once, with words saying why, having run no herdr.
    pub fn open(&self, terminal: Terminal, report: &Report) {
        let Terminal {
            id,
            launch,
            lease,
            link,
        } = terminal;
        let pane = match agent_pane(report, &launch, lease) {
            Ok(pane) => pane,
            Err(words) => {
                (self.say)(format!(
                    "{id}: the terminal on {launch} is not opened: {words}"
                ));
                link.close(&words);
                return;
            }
        };
        (self.say)(format!(
            "{id}: the terminal on {launch} relays pane {}",
            pane.id
        ));
        let (closing, told) = mpsc::channel();
        let relaying = Relaying {
            id: id.clone(),
            launch: launch.clone(),
            pane,
            herdr: Arc::clone(&self.herdr),
            say: Arc::clone(&self.say),
        };
        let thread = thread::spawn(move || relaying.run(link, &told));
        let mut open = self.open.borrow_mut();
        open.retain(|relay| !relay.thread.is_finished());
        open.push(Relay {
            id,
            launch,
            closing,
            thread,
        });
    }

    /// Closes every terminal open on launch `launch`, which the runner
    /// lets go.
    pub fn let_go(&self, launch: &Id) {
        for relay in self.open.borrow().iter() {
            if relay.launch == *launch {
                // A thread already ended has nothing to close.
                let _ = relay.closing.send(format!("its launch {launch} is let go"));
            }
        }
    }

    /// The ids of the terminals still relaying.
    pub fn relaying(&self) -> Vec<Id> {
        let open = self.open.borrow();
        open.iter()
            .filter(|relay| !relay.thread.is_finished())
            .map(|relay| relay.id.clone())
            .collect()
    }
}

impl Drop for Relays {
    fn drop(&mut self) {
        for relay in std::mem::take(self.open.get_mut()) {
            let Relay {
                closing, thread, ..
            } = relay;
            let _ = closing.send("the runner stopped".to_string());
            let _ = thread.join();
        }
    }
}

/// The pane launch `launch`'s agent shows in, where `report` holds the
/// launch under lease number `lease` and the status carried of its place
/// names one, as `clues.shown_at.agent` gives it; otherwise the words
/// saying why there is none.
fn agent_pane(report: &Report, launch: &Id, lease: u64) -> Result<Pane, String> {
    let held = report
        .launches
        .get(launch)
        .filter(|launched| launched.lease.number == lease)
        .ok_or_else(|| format!("this runner holds no launch {launch} under lease {lease}"))?;
    let place = held.answer.place_dir.as_deref();
    let status = held.status.as_ref();
    let answer = status.and_then(|status| serde_json::from_str::<Value>(&status.text).ok());
    answer
        .as_ref()
        .zip(place)
        .and_then(|(answer, place)| facts::pane(facts::entry(answer, place)?))
        .ok_or_else(|| "no agent is running in its place".to_string())
}

/// One open terminal's relay, on its own thread.
struct Relaying {
    id: Id,
    launch: Id,
    pane: Pane,
    herdr: Arc<dyn Herdr>,
    say: Say,
}

impl Relaying {
    /// Relays between `link` and the pane until the link closes or stops
    /// listening, herdr refuses a read, or `closing` says why, then closes
    /// the link with the words why and says so.
    fn run(self, mut link: Box<dyn Link>, closing: &Receiver<String>) {
        let why = self.relay(link.as_mut(), closing);
        link.close(&why);
        (self.say)(format!(
            "{}: the terminal on {} at pane {} is closed: {why}",
            self.id, self.launch, self.pane.id
        ));
    }

    /// The pane's screen out every [`READ_INTERVAL`], sent only when it
    /// differs from the last sent, and the far end's input in as it comes,
    /// until the relay closes; the words why it closes.
    fn relay(&self, link: &mut dyn Link, closing: &Receiver<String>) -> String {
        let mut last: Option<String> = None;
        let mut due = Instant::now();
        loop {
            match closing.try_recv() {
                Ok(why) => return why,
                Err(TryRecvError::Disconnected) => return "the runner stopped".to_string(),
                Err(TryRecvError::Empty) => {}
            }
            let began = Instant::now();
            if began >= due {
                due = began + READ_INTERVAL;
                match self.herdr.read(&self.pane) {
                    Ok(screen) if last.as_ref() == Some(&screen) => {}
                    Ok(screen) => {
                        if !link.screen(&screen) {
                            return "its far end stopped listening".to_string();
                        }
                        last = Some(screen);
                    }
                    Err(Refused(words)) => {
                        return format!("herdr refused a read of the pane: {words}");
                    }
                }
            }
            let passed = match link.input(due.saturating_duration_since(Instant::now())) {
                None => Ok(()),
                Some(Input::Text(text)) => self.herdr.text(&self.pane, &text),
                Some(Input::Keys(keys)) => self.herdr.keys(&self.pane, &keys),
                Some(Input::Close) => return "its link closed".to_string(),
            };
            // herdr's words of a refused input may repeat the input, so
            // the line names only the pane.
            if passed.is_err() {
                (self.say)(format!(
                    "{}: herdr refused input to pane {} of {}",
                    self.id, self.pane.id, self.launch
                ));
            }
        }
    }
}
