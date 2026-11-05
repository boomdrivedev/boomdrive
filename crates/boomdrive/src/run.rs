//! `boomdrive run` (design 10): the start's checks, the lock that keeps
//! one runner to a data directory, the control plane the configuration
//! names opened, and its loop: a keeping pass on its timer, a control
//! pass on its own and when the control plane wakes it, each landing it
//! is told of kept at once, and its stop (design Decision 5). The loop is
//! the runner's one thread of work, so no two of its git calls touch one
//! clone at once. The runner opens no listening socket: every connection
//! it makes is outgoing (core.3).

use crate::carry::{self, Carrier, Configured, Passed};
use crate::config::{self, Address, Config, ControlPlaneTable, Env};
use crate::git::Git;
use crate::pass::{self, Outcome};
use crate::planes::{self, Plane};
use crate::terminal::{self, Relays};
use crate::{clone, flume, hello, report, sluice};
use boomdrive_control::{At, Wake};
use signal_hook::consts::{SIGINT, SIGTERM};
use signal_hook::iterator::Signals;
use std::ffi::OsString;
use std::fs::{self, File, TryLockError};
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{self, Receiver, RecvTimeoutError, Sender};
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

/// What the loop is told between passes.
pub enum Event {
    /// A landing on a repository, which is kept at once.
    Landed(Address),
    /// The control plane wants a control pass at once.
    Wake,
    /// The runner is to stop.
    Stop,
}

/// Tells the runner of landings, and wakes it for its control plane,
/// from whatever learns of them.
#[derive(Clone)]
pub struct Notices {
    events: Sender<Event>,
}

impl Notices {
    /// Notices sent on `events`, to the loop that holds their receiver.
    pub fn new(events: Sender<Event>) -> Notices {
        Notices { events }
    }

    /// Tells the runner of a landing on `address`, which it keeps at once
    /// between passes, leaving its timer where it was. A runner that has
    /// stopped is told nothing.
    pub fn landed(&self, address: Address) {
        let _ = self.events.send(Event::Landed(address));
    }

    /// Wakes the runner for a control pass at once, its control timer
    /// then timed from that pass's start. A runner carrying no control
    /// plane's work, or stopped, is woken to nothing.
    pub fn wake(&self) {
        let _ = self.events.send(Event::Wake);
    }

    /// What an implementation is handed at its opening, to wake the
    /// runner when its control plane pushes work.
    fn waker(&self) -> Wake {
        let notices = self.clone();
        Box::new(move || notices.wake())
    }
}

/// Stops the runner: it finishes the git call in hand, keeps no further
/// repository and says it stopped.
#[derive(Clone)]
pub struct Stopper {
    events: Sender<Event>,
    stop: Arc<AtomicBool>,
}

impl Stopper {
    /// Stops the runner, its pass read between repositories and its wait
    /// woken.
    pub fn stop(&self) {
        self.stop.store(true, Ordering::SeqCst);
        let _ = self.events.send(Event::Stop);
    }
}

/// When the pass after one that began at `last_start` begins:
/// `fetch_interval` after it, or `now` where that is past (design 10).
pub fn next_pass(last_start: Instant, interval: Duration, now: Instant) -> Instant {
    (last_start + interval).max(now)
}

/// The programs a control pass runs for Flume, sluice and herdr.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Programs {
    pub flume: PathBuf,
    pub sluice: PathBuf,
    pub herdr: PathBuf,
}

impl Programs {
    /// Each by its name, on the host's `PATH`.
    pub fn on_path() -> Programs {
        Programs {
            flume: PathBuf::from("flume"),
            sluice: PathBuf::from("sluice"),
            herdr: PathBuf::from("herdr"),
        }
    }

    /// The tools whose versions a hello names, these programs among them.
    fn tools(&self) -> hello::Cli {
        hello::Cli::on_path()
            .with("flume", &self.flume)
            .with("sluice", &self.sluice)
            .with("herdr", &self.herdr)
    }

    /// The terminals relayed through these programs' herdr, each of
    /// their lines said on standard error.
    fn relays(&self) -> Relays {
        Relays::new(terminal::Cli::new(&self.herdr), |line| eprintln!("{line}"))
    }
}

/// The control plane whose work the runner carries, as its start opened
/// it (design Decision 4).
struct Control {
    /// How often a control pass is made, as the configuration said at
    /// start.
    poll_interval: Duration,
    /// The implementation, or why it could not be opened.
    plane: Result<Plane, String>,
    programs: Programs,
    /// The terminals relaying, kept across control passes.
    relays: Relays,
}

impl Control {
    /// The control plane `table` names, of the configuration file `file`,
    /// keeping what it keeps under `data`, and waking the runner through
    /// `notices`. An implementation says its lines on standard error.
    fn open(table: &ControlPlaneTable, file: &Path, data: &Path, notices: &Notices) -> Control {
        let programs = Programs::on_path();
        Control {
            poll_interval: table.poll_interval,
            plane: planes::open(
                &table.implementation,
                &table.implementation_table,
                file,
                data,
                Box::new(|line| eprintln!("{line}")),
                notices.waker(),
            ),
            relays: programs.relays(),
            programs,
        }
    }
}

/// A runner whose start's checks passed, holding the lock on its data
/// directory until it is dropped.
pub struct Runner {
    git: Git,
    home: PathBuf,
    file: PathBuf,
    data: PathBuf,
    config: Config,
    /// The control plane the configuration names, none where it names
    /// none.
    control: Option<Control>,
    events: Sender<Event>,
    waiting: Receiver<Event>,
    stop: Arc<AtomicBool>,
    _lock: File,
}

impl Runner {
    /// Starts a runner on the configuration `file` names, else the
    /// default one, in the environment `var` gives. The start's checks
    /// run in design 10's order, and the first that fails refuses with
    /// why, having touched nothing: `HOME` is set, the configuration
    /// reads, git is 2.48 or later, the data directory is there or can be
    /// made, and no other runner holds it. Then the control plane the
    /// configuration names is opened; one that cannot be is named by
    /// [`Runner::unopened`], and the runner keeps its repositories all
    /// the same.
    pub fn start(
        file: Option<&Path>,
        var: impl Fn(&str) -> Option<OsString>,
        git: Git,
    ) -> Result<Runner, String> {
        let env = Env::read(var)?;
        let file = config::config_path(file, &env);
        let config = config::read(&file, &env.home)?;
        git.check_version()?;
        let data = config::data_dir(&env);
        fs::create_dir_all(&data).map_err(|error| {
            format!(
                "the data directory {} cannot be made: {error}",
                data.display()
            )
        })?;
        let lock = lock(&data)?;
        let (events, waiting) = mpsc::channel();
        let notices = Notices::new(events.clone());
        let control = config
            .control_plane
            .as_ref()
            .map(|table| Control::open(table, &file, &data, &notices));
        Ok(Runner {
            git,
            home: env.home,
            file,
            data,
            config,
            control,
            events,
            waiting,
            stop: Arc::new(AtomicBool::new(false)),
            _lock: lock,
        })
    }

    /// Why the control plane the configuration names could not be
    /// opened, where it could not: no control pass is then made.
    pub fn unopened(&self) -> Option<&str> {
        match &self.control {
            Some(Control {
                plane: Err(why), ..
            }) => Some(why),
            _ => None,
        }
    }

    /// Carries the work of the control plane `open` makes, handed the
    /// runner's wake as an implementation's opener is, in place of the
    /// implementation the configuration names, at the poll interval it
    /// names, running `programs`: how a test runs the loop over a control
    /// plane and tools of its own. Refused where the configuration names
    /// no control plane.
    pub fn carry_for(
        &mut self,
        open: impl FnOnce(Wake) -> Plane,
        programs: Programs,
    ) -> Result<(), String> {
        let wake = self.notices().waker();
        let Some(control) = &mut self.control else {
            return Err("the configuration names no control plane".to_string());
        };
        control.plane = Ok(open(wake));
        control.relays = programs.relays();
        control.programs = programs;
        Ok(())
    }

    /// What tells the runner of landings, and wakes it.
    pub fn notices(&self) -> Notices {
        Notices::new(self.events.clone())
    }

    /// What stops the runner.
    pub fn stopper(&self) -> Stopper {
        Stopper {
            events: self.events.clone(),
            stop: Arc::clone(&self.stop),
        }
    }

    /// Stops the runner on SIGTERM or SIGINT, from a thread that waits
    /// for either.
    pub fn stop_on_signals(&self) -> Result<(), String> {
        let mut signals = Signals::new([SIGTERM, SIGINT])
            .map_err(|error| format!("SIGTERM and SIGINT cannot be caught: {error}"))?;
        let stopper = self.stopper();
        thread::spawn(move || {
            for _ in signals.forever() {
                stopper.stop();
            }
        });
        Ok(())
    }

    /// The line the runner starts with: its version, configuration file,
    /// data directory and repository root.
    pub fn started(&self) -> String {
        let kept = match self.config.repositories.len() {
            1 => "1 repository".to_string(),
            n => format!("{n} repositories"),
        };
        format!(
            "boomdrive {}: keeping {kept} from {}; data {}; repository root {}",
            env!("CARGO_PKG_VERSION"),
            self.file.display(),
            self.data.display(),
            self.config.repository_root.display()
        )
    }

    /// One keeping pass over the configuration the runner started with,
    /// then, where it names a control plane, one control pass (design
    /// Decision 5): failed where a git call of the keeping pass failed,
    /// or the control pass did not pull and report, its control plane
    /// unopened among it.
    pub fn once(&mut self, say: &mut dyn FnMut(String)) -> Outcome {
        let kept = pass::pass(&self.git, &self.data, &self.config, &self.stop, say);
        let carried = match &self.control {
            None => Outcome::Kept,
            Some(Control { plane: Err(_), .. }) => Outcome::Failed,
            Some(_) => match self.carry(Ok(self.config.clone()), say) {
                Some(Passed { reported: true, .. }) => Outcome::Kept,
                _ => Outcome::Failed,
            },
        };
        kept.and(carried)
    }

    /// Passes until stopped (design 10 and Decision 5). A keeping pass is
    /// made at once, on the configuration the runner started with, then
    /// one each `fetch_interval` after the start of the one before, the
    /// configuration read again for each. A pass that cannot read it
    /// names why and keeps nothing. Where the runner carries a control
    /// plane's work, a control pass follows the first keeping pass, then
    /// one each `poll_interval` after the start of the one before, sooner
    /// before a lease lapses, and at once when the control plane wakes the
    /// runner. Between passes, each landing it is told of is kept at
    /// once. Stopped, it keeps no further repository and says so.
    pub fn keep(&mut self, say: &mut dyn FnMut(String)) {
        let mut started = Some(self.config.clone());
        let mut fetch_interval = self.config.fetch_interval;
        let mut keep_due = Instant::now();
        let mut carry_due = self.carries().then(Instant::now);
        'passes: loop {
            if Instant::now() >= keep_due {
                let began = Instant::now();
                let read = match started.take() {
                    Some(config) => Ok(config),
                    None => config::read(&self.file, &self.home),
                };
                match read {
                    Ok(config) => {
                        fetch_interval = config.fetch_interval;
                        pass::pass(&self.git, &self.data, &config, &self.stop, say);
                    }
                    Err(why) => say(format!("boomdrive: {why}: nothing kept this pass")),
                }
                keep_due = next_pass(began, fetch_interval, Instant::now());
            }
            if self.stop.load(Ordering::SeqCst) {
                break 'passes;
            }
            if carry_due.is_some_and(|due| Instant::now() >= due) {
                let began = Instant::now();
                let read = config::read(&self.file, &self.home);
                carry_due = self.carry(read, say).map(|passed| {
                    let left = passed
                        .expires
                        .map(|expires| carry::left(expires, At::now()));
                    carry::next_due(began, Instant::now(), self.poll_interval(), left)
                });
            }
            loop {
                if self.stop.load(Ordering::SeqCst) {
                    break 'passes;
                }
                let due = carry_due.map_or(keep_due, |carry| carry.min(keep_due));
                let wait = due.saturating_duration_since(Instant::now());
                match self.waiting.recv_timeout(wait) {
                    Ok(Event::Landed(address)) => self.landed(&address, say),
                    Ok(Event::Wake) => {
                        if carry_due.is_some() {
                            carry_due = Some(Instant::now());
                            break;
                        }
                    }
                    Ok(Event::Stop) => break 'passes,
                    Err(RecvTimeoutError::Timeout) => break,
                    Err(RecvTimeoutError::Disconnected) => {
                        unreachable!("the runner holds a sender of its own")
                    }
                }
            }
        }
        say("boomdrive: stopped".to_string());
    }

    /// Whether the runner carries a control plane's work: the
    /// configuration names one, and it was opened.
    fn carries(&self) -> bool {
        matches!(self.control, Some(Control { plane: Ok(_), .. }))
    }

    /// The interval between control passes, as the configuration said at
    /// start.
    fn poll_interval(&self) -> Duration {
        self.control
            .as_ref()
            .map_or(Duration::ZERO, |control| control.poll_interval)
    }

    /// One control pass against the configuration as `read` gives it
    /// (design Decision 5), none where the runner carries no control
    /// plane's work. A configuration that did not read is named, and
    /// nothing is carried until it reads again.
    fn carry(
        &mut self,
        read: Result<Config, String>,
        say: &mut dyn FnMut(String),
    ) -> Option<Passed> {
        let Some(Control {
            plane: Ok(plane),
            programs,
            relays,
            ..
        }) = &mut self.control
        else {
            return None;
        };
        let unread = |say: &mut dyn FnMut(String), why: String| {
            say(format!("boomdrive: {why}: nothing carried this pass"));
            Some(Passed {
                expires: None,
                reported: false,
            })
        };
        let config = match read {
            Ok(config) => config,
            Err(why) => return unread(say, why),
        };
        let clones = match clone::find(&self.git, &self.data) {
            Ok(clones) => Configured::new(&config, clones),
            Err(error) => {
                let repositories = clone::repositories(&self.data);
                return unread(
                    say,
                    format!("{} cannot be read: {error}", repositories.display()),
                );
            }
        };
        let flume = flume::Cli::new(&programs.flume);
        let sluice = sluice::Cli::new(&programs.sluice);
        let tools = programs.tools();
        let notices = Notices::new(self.events.clone());
        let control_dir = report::control_dir(&self.data);
        let carrier = Carrier {
            flume: &flume,
            sluice: &sluice,
            git: &self.git,
            clones: &clones,
            tools: &tools,
            notices: &notices,
            control_dir: &control_dir,
            relays,
        };
        Some(carrier.pass(plane.as_mut(), say))
    }

    /// Keeps `address` at once, told of a landing on it, against the
    /// configuration as it stands: read again, so a repository dropped
    /// since the last pass is never fetched.
    fn landed(&self, address: &Address, say: &mut dyn FnMut(String)) {
        match config::read(&self.file, &self.home) {
            Ok(config) => {
                pass::keep_one(&self.git, &self.data, &config, address, say);
            }
            Err(why) => say(format!(
                "boomdrive: {why}: nothing kept at the landing on {address}"
            )),
        }
    }
}

/// Holds `<data>/lock`, refused where another runner holds it.
fn lock(data: &Path) -> Result<File, String> {
    let path = data.join("lock");
    let file = File::options()
        .create(true)
        .truncate(false)
        .write(true)
        .open(&path)
        .map_err(|error| format!("{}: {error}", path.display()))?;
    match file.try_lock() {
        Ok(()) => Ok(file),
        Err(TryLockError::WouldBlock) => Err(format!(
            "the data directory {} is held by a running runner",
            data.display()
        )),
        Err(TryLockError::Error(error)) => {
            Err(format!("{} cannot be locked: {error}", path.display()))
        }
    }
}

/// `boomdrive run`, on the configuration `file` names, else the default
/// one: starts the runner and says so on standard error, with why its
/// control plane could not be opened where it could not, then makes one
/// keeping pass and one control pass where `once`, else passes until
/// SIGTERM or SIGINT stops it. Exits 2 where the runner does not start, 1
/// where `once`'s passes failed, and 0 otherwise.
pub fn main(file: Option<&Path>, once: bool) -> ExitCode {
    let started =
        Runner::start(file, |name| std::env::var_os(name), Git::default()).and_then(|runner| {
            if !once {
                runner.stop_on_signals()?;
            }
            Ok(runner)
        });
    let mut runner = match started {
        Ok(runner) => runner,
        Err(why) => {
            eprintln!("boomdrive: {why}");
            return ExitCode::from(2);
        }
    };
    eprintln!("{}", runner.started());
    if let Some(why) = runner.unopened() {
        eprintln!("boomdrive: {why}: no control pass runs");
    }
    let mut say = |line: String| eprintln!("{line}");
    if !once {
        runner.keep(&mut say);
        return ExitCode::SUCCESS;
    }
    match runner.once(&mut say) {
        Outcome::Kept => ExitCode::SUCCESS,
        Outcome::Failed => ExitCode::from(1),
    }
}

#[cfg(test)]
mod tests {
    use super::next_pass;
    use crate::carry::next_due;
    use std::time::{Duration, Instant};

    /// The running spec's "Passes on the timer": with `fetch_interval`
    /// two minutes and a pass begun at 10:00:00, the next begins at
    /// 10:02:00, or at once when the pass before ends later than that.
    #[test]
    fn passes_on_the_timer() {
        let ten = Instant::now();
        let interval = Duration::from_secs(120);
        let due = ten + interval;
        assert_eq!(next_pass(ten, interval, ten + Duration::from_secs(5)), due);
        assert_eq!(next_pass(ten, interval, due), due);
        let late = ten + Duration::from_secs(150);
        assert_eq!(next_pass(ten, interval, late), late);
    }

    /// The running spec's "On the control timer": with `poll_interval`
    /// one minute, `fetch_interval` two minutes, and a control pass and a
    /// keeping pass begun at 10:00:00, the next control pass begins at
    /// 10:01:00, and the next keeping pass at 10:02:00, each on its own
    /// timer.
    #[test]
    fn on_the_control_timer() {
        let ten = Instant::now();
        let ended = ten + Duration::from_secs(5);
        let minute = Duration::from_secs(60);
        let control = next_due(ten, ended, minute, None);
        let keeping = next_pass(ten, 2 * minute, ended);
        assert_eq!(control, ten + minute);
        assert_eq!(keeping, ten + 2 * minute);
        let next_control = next_due(control, control + Duration::from_secs(5), minute, None);
        assert_eq!(
            next_control, keeping,
            "the second control pass is due with the keeping pass"
        );
    }
}
