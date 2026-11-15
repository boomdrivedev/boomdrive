//! The terminal relay (record 0004 as amended on 2026-11-15; the
//! terminal-relay spec): a terminal the control plane opens on a launch,
//! relayed between its link and Flume's relay of the launch's place,
//! `flume --json relay <place>`, the one way the runner reaches an
//! agent's terminal (core.29; Flume's 221 and 222). A terminal opens only
//! on a launch the report holds under the terminal's lease number, at a
//! place Flume named; whether the place has a live agent, and whether its
//! terminal offers a relay, are Flume's to answer (core.1). Each open
//! terminal runs on a thread of its own, its one process Flume's relay,
//! held while the terminal is open and ended by closing its input,
//! Flume's caller's word to end, never by a signal (core.13). Each screen
//! Flume gives goes on the link as Flume gave it, and each input from the
//! link goes to Flume as one line. Nothing of either is kept: the thread
//! holds no screen, and its lines name the terminal, its launch and its
//! place, with Flume's words of an unread read and of a refusal, never
//! what passed (core.11, core.35). It reaches its far end over no
//! connection but the control plane's own (core.3).

use crate::flume;
use boomdrive_control::{Id, Input, Link, Report, Terminal};
use serde::de::IgnoredAny;
use serde::Deserialize;
use serde_json::json;
use std::cell::RefCell;
use std::io::{self, BufRead, BufReader, Read, Write};
use std::process::{ChildStdin, ExitStatus};
use std::sync::mpsc::{self, Receiver, RecvTimeoutError, Sender, TryRecvError};
use std::sync::Arc;
use std::thread::{self, JoinHandle};
use std::time::Duration;

/// How long an open terminal waits on its link's input before it looks
/// again for Flume's lines and the runner's word to close.
const WAIT: Duration = Duration::from_millis(20);

/// Where a line of the relay's goes: standard error on a host.
type Say = Arc<dyn Fn(String) + Send + Sync>;

/// The terminals the runner relays, each on a thread of its own. It is
/// kept across control passes, so a pass can close the terminals on a
/// launch it lets go. Dropped, it closes every terminal still open, and
/// waits for each thread to end.
pub struct Relays {
    flume: flume::Cli,
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
    /// Terminals relayed through `flume`'s relay, each of their lines
    /// said to `say`.
    pub fn new(flume: flume::Cli, say: impl Fn(String) + Send + Sync + 'static) -> Relays {
        Relays {
            flume,
            say: Arc::new(say),
            open: RefCell::new(Vec::new()),
        }
    }

    /// Opens `terminal` where `report` holds its launch under its lease
    /// number at a place Flume named, and relays it through Flume's relay
    /// of that place on a thread of its own. Otherwise closes its link at
    /// once, with words saying why, having run nothing.
    pub fn open(&self, terminal: Terminal, report: &Report) {
        let Terminal {
            id,
            launch,
            lease,
            link,
        } = terminal;
        let place = match held_place(report, &launch, lease) {
            Ok(place) => place,
            Err(words) => {
                (self.say)(format!(
                    "{id}: the terminal on {launch} is not opened: {words}"
                ));
                link.close(&words);
                return;
            }
        };
        (self.say)(format!(
            "{id}: the terminal on {launch} at {place} is opened"
        ));
        let (closing, told) = mpsc::channel();
        let relaying = Relaying {
            id: id.clone(),
            launch: launch.clone(),
            place,
            flume: self.flume.clone(),
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
    /// Closes each open relay's input, and waits for its thread, which
    /// awaits Flume's end.
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

/// The place launch `launch` stands at, as Flume named it at the launch,
/// where `report` holds the launch under lease number `lease`; otherwise
/// the words saying why there is none.
fn held_place(report: &Report, launch: &Id, lease: u64) -> Result<String, String> {
    let held = report
        .launches
        .get(launch)
        .filter(|launched| launched.lease.number == lease)
        .ok_or_else(|| format!("this runner holds no launch {launch} under lease {lease}"))?;
    held.answer
        .place_dir
        .clone()
        .ok_or_else(|| format!("{launch} has no place Flume named"))
}

/// One line Flume's relay writes on its standard output (Flume's 221).
#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
enum Out {
    /// What the terminal shows, given each time it differs.
    Screen(String),
    /// Flume's words of a read the terminal did not answer.
    Unread(String),
    /// An input Flume did not type. Its words may name what the person
    /// typed, so they are never read (core.11).
    Untyped(IgnoredAny),
    /// Flume's answer: the relay ended.
    Relayed(Relayed),
    /// Flume's answer: the relay refused.
    Refused(Refused),
}

#[derive(Deserialize)]
struct Relayed {
    /// Who ended it, `caller` or `gone`, as Flume gave it.
    ended: String,
}

#[derive(Deserialize)]
struct Refused {
    refusal: Reason,
}

#[derive(Deserialize)]
struct Reason {
    reason: String,
}

/// Flume's answer to the relay, as its last line gave it.
enum Answer {
    Relayed(String),
    Refused(String),
}

/// One open terminal's relay, on its own thread.
struct Relaying {
    id: Id,
    launch: Id,
    place: String,
    flume: flume::Cli,
    say: Say,
}

/// Flume's relay as one terminal runs it: its input, while open, and the
/// runner's words of why it closed it, once it has.
struct Running {
    input: Option<ChildStdin>,
    why: Option<String>,
    answer: Option<Answer>,
}

impl Running {
    /// The runner closes the relay, with `words` why: Flume's input is
    /// closed, its caller's word to end, and no further screen is sent.
    fn close(&mut self, words: impl Into<String>) {
        self.input = None;
        self.why.get_or_insert_with(|| words.into());
    }
}

impl Relaying {
    /// Relays between `link` and Flume's relay of the place until either
    /// ends, then closes the link with the words why, once Flume has
    /// exited, and says so.
    fn run(self, mut link: Box<dyn Link>, closing: &Receiver<String>) {
        let why = self.relay(link.as_mut(), closing);
        link.close(&why);
        (self.say)(format!(
            "{}: the terminal on {} at {} is closed: {why}",
            self.id, self.launch, self.place
        ));
    }

    /// Runs Flume's relay of the place: each of its lines read as it
    /// comes, the far end's input written to it, until the runner closes
    /// its input or Flume ends; then reads its output to its end and
    /// awaits its exit, signalling nothing. The words why it closed.
    fn relay(&self, link: &mut dyn Link, closing: &Receiver<String>) -> String {
        let mut child = match self.flume.relay(&self.place) {
            Ok(child) => child,
            Err(error) => return format!("flume could not run: {error}"),
        };
        let (Some(stdout), Some(mut stderr)) = (child.stdout.take(), child.stderr.take()) else {
            unreachable!("flume::Cli::relay pipes Flume's output and error");
        };
        let (out, lines) = mpsc::channel();
        let reader = thread::spawn(move || read_lines(stdout, &out));
        // Flume's standard error is read whole, on a thread of its own, so
        // a Flume that writes to it never waits on the runner.
        let errors = thread::spawn(move || {
            let mut printed = Vec::new();
            let _ = stderr.read_to_end(&mut printed);
            printed
        });
        let mut running = Running {
            input: child.stdin.take(),
            why: None,
            answer: None,
        };
        loop {
            if running.why.is_none() {
                match closing.try_recv() {
                    Ok(words) => running.close(words),
                    Err(TryRecvError::Disconnected) => running.close("the runner stopped"),
                    Err(TryRecvError::Empty) => {}
                }
            }
            let ended = loop {
                match lines.try_recv() {
                    Ok(line) => self.take(line, link, &mut running),
                    Err(TryRecvError::Empty) => break false,
                    Err(TryRecvError::Disconnected) => break true,
                }
            };
            if ended {
                break;
            }
            if running.input.is_some() {
                self.pass_in(link.input(WAIT), &mut running);
            } else {
                match lines.recv_timeout(WAIT) {
                    Ok(line) => self.take(line, link, &mut running),
                    Err(RecvTimeoutError::Timeout) => {}
                    Err(RecvTimeoutError::Disconnected) => break,
                }
            }
        }
        running.input = None;
        let _ = reader.join();
        let printed = errors.join().unwrap_or_default();
        let status = child.wait();
        match running.why {
            Some(why) => why,
            None => ended(status, running.answer, &printed),
        }
    }

    /// One line of Flume's: a screen sent on the link while the runner
    /// has not closed the relay, an unread read or an untyped input said,
    /// an answer kept.
    fn take(&self, line: Out, link: &mut dyn Link, running: &mut Running) {
        match line {
            Out::Screen(screen) => {
                if running.why.is_none() && !link.screen(&screen) {
                    running.close("its far end stopped listening");
                }
            }
            Out::Unread(words) => (self.say)(format!(
                "{}: flume relay of {} for {} read nothing: {words}",
                self.id, self.place, self.launch
            )),
            Out::Untyped(IgnoredAny) => (self.say)(format!(
                "{}: flume relay of {} for {} left an input untyped",
                self.id, self.place, self.launch
            )),
            Out::Relayed(relayed) => running.answer = Some(Answer::Relayed(relayed.ended)),
            Out::Refused(refused) => {
                running.answer = Some(Answer::Refused(refused.refusal.reason));
            }
        }
    }

    /// The far end's input, written to Flume's input as one line each, or
    /// the relay closed where the link closed. A write that fails, Flume
    /// having ended, ends the input, and Flume's answer closes the
    /// terminal.
    fn pass_in(&self, input: Option<Input>, running: &mut Running) {
        let line = match input {
            None => return,
            Some(Input::Close) => return running.close("its link closed"),
            Some(Input::Text(text)) => json!({ "text": text }),
            Some(Input::Keys(keys)) => json!({ "keys": keys }),
        };
        let Some(stdin) = running.input.as_mut() else {
            return;
        };
        if write_line(stdin, &line).is_err() {
            running.input = None;
        }
    }
}

/// `line` written to Flume's input whole, as one line, and flushed.
fn write_line(stdin: &mut ChildStdin, line: &serde_json::Value) -> io::Result<()> {
    let mut bytes = serde_json::to_vec(line)?;
    bytes.push(b'\n');
    stdin.write_all(&bytes)?;
    stdin.flush()
}

/// Each line Flume writes on `stdout` sent to `out` as it comes, until its
/// output closes. A line that is none of the relay's forms is passed over,
/// unsaid: Flume's lines only gain forms, and one the runner cannot read
/// may hold the screen.
fn read_lines(stdout: impl Read, out: &Sender<Out>) {
    for line in BufReader::new(stdout).split(b'\n') {
        let Ok(line) = line else {
            return;
        };
        if let Ok(line) = serde_json::from_slice::<Out>(&line) {
            if out.send(line).is_err() {
                return;
            }
        }
    }
}

/// The words of a relay Flume ended, the runner having closed nothing,
/// read by its exit as every answer of Flume's is: its relay's end, its
/// refusal's reason, or else its standard error, its one final newline
/// left off.
fn ended(status: io::Result<ExitStatus>, answer: Option<Answer>, printed: &[u8]) -> String {
    let status = match status {
        Ok(status) => status,
        Err(error) => return format!("flume could not be awaited: {error}"),
    };
    match (status.code(), answer) {
        (None, _) => format!("flume was ended: {status}"),
        (Some(0), Some(Answer::Relayed(ended))) => format!("Flume's relay ended: {ended}"),
        (Some(1), Some(Answer::Refused(reason))) => reason,
        (Some(_), _) => {
            let printed = String::from_utf8_lossy(printed);
            printed.strip_suffix('\n').unwrap_or(&printed).to_string()
        }
    }
}
