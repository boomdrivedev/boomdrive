//! `boomdrive run` (design 10): the start's checks, the lock that keeps
//! one runner to a data directory, the control plane the configuration
//! names opened as its parts, and the runner's threads (record 0026;
//! core.49). The loop keeps the host's own work: a keeping pass on its
//! timer, each landing it is told of kept at once, between passes and
//! between two repositories of a pass, and its stop. Each part of the
//! control plane is carried on a thread of its own from the end of the
//! first keeping pass: its own control passes on its own timer, at once
//! when its implementation wakes it, its own reach and its own terminals,
//! so no part's pull, act, command or report waits on another's or on a
//! keeping pass. What they share is a clone's lock, for one keep or one
//! launch, the report file's own lock, for one save of a part's share, and
//! Flume's room (design Decisions 2 to 6). Each pass, keeping or control,
//! ends with millpond's tick of each inbox the configuration names, which
//! no two passes run at once (record 0033). The runner runs no settle of
//! Flume's: `flume host`, Flume's resident, keeps watch over the host
//! (core.13; one-resident-per-tool). The runner opens no listening
//! socket: every connection it makes is outgoing (core.3).

use crate::carry::{Carrier, Configured, Elsewhere, Least, Passed, Reach};
use crate::config::{self, Address, Config, ControlPlaneTable, Env, LocalLayer};
use crate::flume::Flume;
use crate::flume_host::{self, Source, Subscription, Where};
use crate::forge::Forge;
use crate::git::Git;
use crate::locks::Locks;
use crate::millpond::{self, Ticks};
use crate::pass::{Keeper, Keeping, Outcome};
use crate::planes;
use crate::report::{self, ReportFile};
use crate::sluice::Sluice;
use crate::terminal::Relays;
use crate::{clone, flume, hands, hello, sluice};
use boomdrive_control::{ControlPlane, Part, Wake};
use signal_hook::consts::{SIGINT, SIGTERM};
use signal_hook::iterator::Signals;
use std::collections::BTreeMap;
use std::ffi::OsString;
use std::fs::{self, File, TryLockError};
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{self, Receiver, RecvTimeoutError, Sender};
use std::sync::{Arc, Condvar, Mutex, MutexGuard};
use std::thread;
use std::time::{Duration, Instant};
use weir_bounded::Halt;

/// Where the runner says its lines, from the loop and every part's
/// thread, each line whole.
pub type Say<'a> = &'a (dyn Fn(String) + Sync);

/// What the loop is told between passes.
pub enum Event {
    /// A landing on a repository, which is kept at once.
    Landed(Address),
    /// The runner is to stop.
    Stop,
}

/// Tells the loop of landings, from whatever learns of them: a part's
/// pass that reads one among them.
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
    /// between passes or between two repositories of a pass, leaving its
    /// timer where it was. A runner that has stopped is told nothing.
    pub fn landed(&self, address: Address) {
        let _ = self.events.send(Event::Landed(address));
    }
}

/// A part's own wake, the answers of its commands coming back, and the
/// runner's stop as that part's thread waits for them: what the part's
/// thread waits on between its passes (core.50).
#[derive(Debug, Default)]
pub struct Bell {
    rung: Mutex<Rung>,
    changed: Condvar,
}

#[derive(Debug, Default)]
struct Rung {
    /// Woken since the part's pass under way began.
    woken: bool,
    /// An answer of a command of the part's came back since the part last
    /// took them (record 0002 as amended on 2026-11-19).
    answered: bool,
    stopped: bool,
}

/// What a part's thread is told once it has waited.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rang {
    /// Its next pass is due by its timer, or it was woken.
    Due,
    /// An answer of a command of the part's came back, which it takes
    /// and sends before its next pass is due.
    Answered,
    /// The runner stopped: it begins no pass.
    Stopped,
}

impl Bell {
    fn rung(&self) -> MutexGuard<'_, Rung> {
        self.rung.lock().unwrap_or_else(|e| e.into_inner())
    }

    /// Wakes the part for a pass at once, its timer then timed from that
    /// pass's start.
    pub fn wake(&self) {
        self.rung().woken = true;
        self.changed.notify_all();
    }

    /// Tells the part an answer of a command of its came back.
    pub fn answered(&self) {
        self.rung().answered = true;
        self.changed.notify_all();
    }

    /// Stops the part: it begins no further pass.
    pub fn stop(&self) {
        self.rung().stopped = true;
        self.changed.notify_all();
    }

    /// Whether the part was woken since its pass under way began, so its
    /// next is due at once.
    pub fn woken(&self) -> bool {
        self.rung().woken
    }

    /// Whether the runner stopped, so the part begins nothing further.
    pub fn stopped(&self) -> bool {
        self.rung().stopped
    }

    /// Takes the wake, and the answers, that a pass beginning now takes.
    pub fn begin_pass(&self) {
        let mut rung = self.rung();
        rung.woken = false;
        rung.answered = false;
    }

    /// Waits until `due`, or until the part is woken, an answer of its
    /// comes back, or it is stopped. A pass due is told before an answer,
    /// which the pass takes; an answer told is taken.
    pub fn wait(&self, due: Instant) -> Rang {
        let mut rung = self.rung();
        loop {
            if rung.stopped {
                return Rang::Stopped;
            }
            let left = due.saturating_duration_since(Instant::now());
            if rung.woken || left.is_zero() {
                return Rang::Due;
            }
            if rung.answered {
                rung.answered = false;
                return Rang::Answered;
            }
            rung = self
                .changed
                .wait_timeout(rung, left)
                .unwrap_or_else(|e| e.into_inner())
                .0;
        }
    }
}

/// Each part's bell, by its label, made as its implementation is opened
/// and handed the part's wake.
#[derive(Clone, Debug, Default)]
struct Bells(Arc<Mutex<BTreeMap<String, Arc<Bell>>>>);

impl Bells {
    fn bells(&self) -> MutexGuard<'_, BTreeMap<String, Arc<Bell>>> {
        self.0.lock().unwrap_or_else(|e| e.into_inner())
    }

    /// The bell of the part `label`.
    fn of(&self, label: &str) -> Arc<Bell> {
        Arc::clone(self.bells().entry(label.to_string()).or_default())
    }

    /// What the implementation is handed for the part `label`, to wake it
    /// when its control plane pushes the part's work.
    fn waker(&self, label: &str) -> Wake {
        let bell = self.of(label);
        Box::new(move || bell.wake())
    }

    /// Wakes every part, as a change of Flume's status heard does (the
    /// change the-runner-hears-flume-by-subscription, Decision 4).
    fn wake_all(&self) {
        for bell in self.bells().values() {
            bell.wake();
        }
    }

    /// Stops every part.
    fn stop(&self) {
        for bell in self.bells().values() {
            bell.stop();
        }
    }
}

/// Stops the runner: it ends each try of a call to a remote in flight, its
/// own git's or crew's implementation's, as the try's limit would, through
/// the runner's halt, begins no such try, keeps no further repository,
/// begins no part's pass, lets each part end the pass in hand, and says it
/// stopped (record 0042 as amended on 2026-11-12; core.58).
#[derive(Clone)]
pub struct Stopper {
    events: Sender<Event>,
    stop: Arc<AtomicBool>,
    bells: Bells,
    /// The runner's halt, `weir-bounded`'s, which every try made for the
    /// runner reads.
    halt: Halt,
}

impl Stopper {
    /// Stops the runner: its stop flag set first, so a keeping pass whose
    /// fetch the halt ends reads the stop before it begins the next
    /// repository; then its halt, ending every try in flight and any begun
    /// after; then its wait woken, and every part's wait woken.
    pub fn stop(&self) {
        self.stop.store(true, Ordering::SeqCst);
        self.halt.halt();
        let _ = self.events.send(Event::Stop);
        self.bells.stop();
    }
}

/// When the pass after one that began at `last_start` begins:
/// `fetch_interval` after it, or `now` where that is past (design 10).
pub fn next_pass(last_start: Instant, interval: Duration, now: Instant) -> Instant {
    (last_start + interval).max(now)
}

/// The programs the runner runs for Flume, sluice and millpond: its
/// ticks, its parts' control passes' and its terminals' relays.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Programs {
    pub flume: PathBuf,
    pub sluice: PathBuf,
    pub millpond: PathBuf,
}

impl Programs {
    /// Each by its name, on the host's `PATH`.
    pub fn on_path() -> Programs {
        Programs {
            flume: PathBuf::from("flume"),
            sluice: PathBuf::from("sluice"),
            millpond: PathBuf::from("millpond"),
        }
    }

    /// The tools whose versions a hello names, these programs among them.
    fn tools(&self) -> hello::Cli {
        hello::Cli::on_path()
            .with("flume", &self.flume)
            .with("sluice", &self.sluice)
    }
}

/// The runner's seam to the tools it runs for its parts and its ticks:
/// Flume, sluice, the tools whose versions a hello names, millpond's tick
/// and the terminals' relays. [`Clis`] runs each tool's own command line
/// at the programs [`Programs`] names; the runner's loop and `run --once`
/// reach the tools through it alone (core.16, core.29).
pub trait Toolset: Send + Sync {
    /// Flume as a part's hands run it: its launches, acts and confirms.
    fn flume(&self) -> &dyn Flume;
    /// Flume as a part's pass reads it, its status read from flume host
    /// by `source`, every other command run as [`Toolset::flume`] runs it.
    fn flume_reading<'a>(&'a self, source: Source<'a>) -> Box<dyn Flume + 'a>;
    /// sluice, as a part's pass and its hands run it.
    fn sluice(&self) -> &dyn Sluice;
    /// The tools whose versions a hello names.
    fn versions(&self) -> &dyn hello::Tools;
    /// millpond's tick of the inbox `inbox`.
    fn tick(&self, inbox: &str) -> Result<millpond::Reply, millpond::Unrun>;
    /// The terminals relayed through Flume, each of their lines said on
    /// standard error.
    fn relays(&self) -> Relays;
}

/// Each tool's own command line, at the programs a [`Programs`] names.
pub struct Clis {
    flume: flume::Cli,
    sluice: sluice::Cli,
    millpond: millpond::Cli,
    versions: hello::Cli,
}

impl From<Programs> for Clis {
    fn from(programs: Programs) -> Clis {
        Clis {
            flume: flume::Cli::new(&programs.flume),
            sluice: sluice::Cli::new(&programs.sluice),
            millpond: millpond::Cli::new(&programs.millpond),
            versions: programs.tools(),
        }
    }
}

impl Toolset for Clis {
    fn flume(&self) -> &dyn Flume {
        &self.flume
    }

    fn flume_reading<'a>(&'a self, source: Source<'a>) -> Box<dyn Flume + 'a> {
        Box::new(flume_host::Resident {
            cli: &self.flume,
            source,
        })
    }

    fn sluice(&self) -> &dyn Sluice {
        &self.sluice
    }

    fn versions(&self) -> &dyn hello::Tools {
        &self.versions
    }

    fn tick(&self, inbox: &str) -> Result<millpond::Reply, millpond::Unrun> {
        self.millpond.tick(inbox)
    }

    fn relays(&self) -> Relays {
        Relays::new(self.flume.clone(), |line| eprintln!("{line}"))
    }
}

/// The control plane whose work the runner carries, as its start opened
/// it (design Decision 4).
struct Control {
    /// How often each part's control pass is made, as the configuration
    /// said at start.
    poll_interval: Duration,
    /// The implementation's parts, or why it could not be opened.
    parts: Result<Vec<Part>, String>,
}

impl Control {
    /// The control plane `table` names, of the configuration file `file`,
    /// keeping what it keeps under `data`, each part woken through its
    /// bell among `bells`, and given `halt`, the runner's, at which every
    /// call to a remote it makes for the runner ends (record 0042 as
    /// amended on 2026-11-12; core.58). An implementation says its lines
    /// on standard error.
    fn open(
        table: &ControlPlaneTable,
        file: &Path,
        data: &Path,
        bells: &Bells,
        halt: &Halt,
    ) -> Control {
        let parts = planes::open(
            table,
            file,
            data,
            &|_| Box::new(|line| eprintln!("{line}")),
            &|label| bells.waker(label),
            halt,
        );
        Control {
            poll_interval: table.poll_interval,
            parts,
        }
    }
}

/// A runner whose start's checks passed, holding the lock on its data
/// directory until it is dropped.
pub struct Runner {
    git: Git,
    home: PathBuf,
    /// The managed layer's file.
    file: PathBuf,
    /// The local layer's file, read over it (design Decision 1).
    local: PathBuf,
    data: PathBuf,
    config: Config,
    /// The tools the runner runs, each tool's own command line on the
    /// host's `PATH` unless a test puts its own in place.
    toolset: Box<dyn Toolset>,
    /// The control plane the configuration names, none where it names
    /// none.
    control: Option<Control>,
    /// Where flume host's socket is, by the environment the runner started
    /// in unless a test puts its own in place.
    flume_socket: Where,
    /// The runner's ticks of each inbox, shared by the loop and every
    /// part's thread.
    ticker: Ticker,
    events: Sender<Event>,
    notices: Notices,
    waiting: Receiver<Event>,
    stop: Arc<AtomicBool>,
    /// The runner's halt, `weir-bounded`'s, given to its git and set by its
    /// stop, so every try of a call to a remote made for the runner ends
    /// at it (record 0042 as amended on 2026-11-12; core.58).
    halt: Halt,
    /// Each part's bell, by its label.
    bells: Bells,
    /// Each clone's lock, which the keeper takes alone and a launch into
    /// the clone shared (design Decision 4).
    locks: Locks,
    /// What keeps the clones, the loop's passes and the keeping of a
    /// landing: the host's, by git, each fetch given its credential by a
    /// forge made at start, which keeps the App's token for each
    /// repository across passes, minted from `[forge]`'s key or asked of a
    /// control plane that holds the App (record 0007 as amended on
    /// 2026-11-11 and 2026-11-18; record 0053; design Decisions 3 and 5).
    keeper: Box<dyn Keeper>,
    /// The one report file, each part keeping its share of it (design
    /// Decision 5).
    report: ReportFile,
    _lock: File,
}

/// What every part's thread carries its passes over: the host's own,
/// shared by every part.
struct Host<'a> {
    git: &'a Git,
    home: &'a Path,
    file: &'a Path,
    local: &'a Path,
    data: &'a Path,
    poll_interval: Duration,
    toolset: &'a dyn Toolset,
    notices: &'a Notices,
    locks: &'a Locks,
    report: &'a ReportFile,
    ticker: &'a Ticker,
    /// Where flume host's socket is.
    flume_socket: &'a Where,
    /// The runner's subscription to flume host, which each pass reads
    /// Flume's status from; none for `run --once`, whose passes ask flume
    /// host once for each read.
    subscription: Option<&'a Subscription>,
}

/// The runner's ticks: millpond's tick of each inbox the configuration
/// names, `millpond tick --inbox <name> --json`, each run once and awaited
/// at the end of each pass, keeping or control (core.13; record 0033).
/// The loop and every part's thread share it, so no two ticks run at once:
/// a pass that ends while another's ticks are under way shares them, as a
/// keeping pass shares the ticks of the control pass that follows it at
/// once, and waits for nothing (core.49).
struct Ticker {
    /// The `inboxes` the latest reading of the configuration gave.
    read: Mutex<Vec<String>>,
    /// Whether each inbox's last tick was answered, held while the ticks
    /// run.
    ticks: Mutex<Ticks>,
}

impl Ticker {
    /// A ticker whose ticks start from `inboxes`, as the configuration the
    /// runner started with gave them.
    fn new(inboxes: Vec<String>) -> Ticker {
        Ticker {
            read: Mutex::new(inboxes),
            ticks: Mutex::default(),
        }
    }

    /// Takes the `inboxes` a reading of the configuration gave.
    fn read(&self, inboxes: &[String]) {
        *self.read.lock().unwrap_or_else(|e| e.into_inner()) = inboxes.to_vec();
    }

    /// Runs millpond's tick of each inbox the latest reading named, in its
    /// order, and says each answer only where it changes. A tick that
    /// fails stops nothing, and where the configuration names no inbox,
    /// none runs (record 0033). Where another pass's ticks are under way,
    /// runs none.
    fn tick(&self, tools: &dyn Toolset, say: Say<'_>) {
        let mut ticks = match self.ticks.try_lock() {
            Ok(ticks) => ticks,
            Err(std::sync::TryLockError::Poisoned(poisoned)) => poisoned.into_inner(),
            Err(std::sync::TryLockError::WouldBlock) => return,
        };
        let inboxes = self.read.lock().unwrap_or_else(|e| e.into_inner()).clone();
        ticks.keep(&inboxes);
        for inbox in &inboxes {
            ticks.heard(inbox, tools.tick(inbox), &mut |line| say(line));
        }
    }
}

impl Runner {
    /// Starts a runner on the configuration `file` names, else the
    /// default one, with the local layer over it, in the environment
    /// `var` gives. The start's checks run in design 10's order, and the first that fails refuses with
    /// why, having touched nothing: `HOME` is set, the configuration
    /// reads, git is 2.48 or later, the data directory is there or can be
    /// made, and no other runner holds it. Then the control plane the
    /// configuration names is opened as its parts; one that cannot be is
    /// named by [`Runner::unopened`], and the runner keeps its
    /// repositories all the same. The runner makes its halt, gives it to
    /// `git`, and keeps it for its stop, which sets it: a try of a call to
    /// a remote in flight at the stop is ended there, and none begins
    /// after it (record 0042 as amended on 2026-11-12; core.58).
    pub fn start(
        file: Option<&Path>,
        var: impl Fn(&str) -> Option<OsString>,
        git: Git,
    ) -> Result<Runner, String> {
        let flume_socket = Where::from_env(&var);
        let env = Env::read(var)?;
        let file = config::config_path(file, &env);
        let local = config::local_path(&env);
        let config = config::read_layers(&file, &local, &env.home)?;
        git.check_version()?;
        let data = config::data_dir(&env);
        fs::create_dir_all(&data).map_err(|error| {
            format!(
                "the data directory {} cannot be made: {error}",
                data.display()
            )
        })?;
        let lock = lock(&data)?;
        let halt = Halt::default();
        let git = git.halted_by(halt.clone());
        let (events, waiting) = mpsc::channel();
        let notices = Notices::new(events.clone());
        let bells = Bells::default();
        let control = config
            .control_plane
            .as_ref()
            .map(|table| Control::open(table, &file, &data, &bells, &halt));
        let report = ReportFile::new(&report::control_dir(&data));
        let ticker = Ticker::new(config.inboxes.clone());
        let keeper = Box::new(Keeping {
            git: git.clone(),
            forge: Forge::for_runner(&config, &file),
            data: data.clone(),
        });
        Ok(Runner {
            git,
            home: env.home,
            file,
            local,
            data,
            config,
            toolset: Box::new(Clis::from(Programs::on_path())),
            control,
            flume_socket,
            ticker,
            events,
            notices,
            waiting,
            stop: Arc::new(AtomicBool::new(false)),
            halt,
            bells,
            locks: Locks::new(),
            report,
            keeper,
            _lock: lock,
        })
    }

    /// Why the control plane the configuration names could not be
    /// opened, where it could not: no control pass is then made.
    pub fn unopened(&self) -> Option<&str> {
        match &self.control {
            Some(Control {
                parts: Err(why), ..
            }) => Some(why),
            _ => None,
        }
    }

    /// Carries the work of the parts `open` makes, handed the wake of
    /// each part by its label as an implementation's opener is, in place
    /// of the implementation the configuration names, at the poll
    /// interval it names, running `programs`: how a test runs the runner
    /// over control planes and tools of its own. Refused where the
    /// configuration names no control plane.
    pub fn carry_for(
        &mut self,
        open: impl FnOnce(&dyn Fn(&str) -> Wake) -> Vec<Part>,
        programs: Programs,
    ) -> Result<(), String> {
        let bells = self.bells.clone();
        let Some(control) = &mut self.control else {
            return Err("the configuration names no control plane".to_string());
        };
        control.parts = Ok(open(&|label| bells.waker(label)));
        self.tools(programs);
        Ok(())
    }

    /// Runs `programs` in place of those on the host's `PATH`: how a test
    /// runs the runner with tools of its own, a control plane named or
    /// not.
    pub fn tools(&mut self, programs: Programs) {
        self.toolset = Box::new(Clis::from(programs));
    }

    /// Puts flume host's socket at `socket`, in place of the one the
    /// runner's environment names: a test's stub flume host.
    pub fn flume_socket(&mut self, socket: PathBuf) {
        self.flume_socket = Where::At(socket);
    }

    /// What tells the runner of landings.
    pub fn notices(&self) -> Notices {
        self.notices.clone()
    }

    /// What stops the runner.
    pub fn stopper(&self) -> Stopper {
        Stopper {
            events: self.events.clone(),
            stop: Arc::clone(&self.stop),
            bells: self.bells.clone(),
            halt: self.halt.clone(),
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

    /// The line the runner starts with: its version, the repositories
    /// both layers keep, its configuration file, data directory and
    /// repository root.
    pub fn started(&self) -> String {
        let kept = match self.config.repositories().count() {
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

    /// The parts the runner carries, taken from it while they are
    /// carried; none where the configuration names no control plane or it
    /// was not opened.
    fn take_parts(&mut self) -> Vec<Part> {
        match &mut self.control {
            Some(Control {
                parts: Ok(parts), ..
            }) => std::mem::take(parts),
            _ => Vec::new(),
        }
    }

    /// Gives the parts taken back.
    fn give_back(&mut self, parts: Vec<Part>) {
        if let Some(Control {
            parts: Ok(held), ..
        }) = &mut self.control
        {
            *held = parts;
        }
    }

    /// What each part's thread carries its passes over, none where the
    /// configuration names no control plane.
    fn host(&self) -> Option<Host<'_>> {
        let control = self.control.as_ref()?;
        Some(Host {
            git: &self.git,
            home: &self.home,
            file: &self.file,
            local: &self.local,
            data: &self.data,
            poll_interval: control.poll_interval,
            toolset: &*self.toolset,
            notices: &self.notices,
            locks: &self.locks,
            report: &self.report,
            ticker: &self.ticker,
            flume_socket: &self.flume_socket,
            subscription: None,
        })
    }

    /// One keeping pass over the configuration the runner started with,
    /// saying how it read the local layer as a running runner's first
    /// pass does, then, where it names a control plane, one control pass of each
    /// part, side by side (design Decision 2): failed where a git call of
    /// the keeping pass failed, or no part both pulled and reported, its
    /// control plane unopened among it. Each part's pass carries out every
    /// act, since no pass follows it (core.50). Then the ticks that end
    /// them (record 0033): a tick refused or unanswered is said, and no
    /// failure. Stopped, as `boomdrive run` is, it ends the try of a call
    /// to a remote in flight and begins none, through its halt, keeps no
    /// further repository, begins no part's pass once its keeping pass has
    /// ended and no tick, lets each part end the pass in hand, says it
    /// stopped, and fails (record 0042 as amended on 2026-11-12; core.39,
    /// core.58).
    pub fn once(&mut self, say: Say<'_>) -> Outcome {
        self.say_local(&mut LocalLayer::NoFile, &self.config.local, say);
        let kept = self.keeper.pass(
            &self.config,
            &self.locks,
            &self.stop,
            &mut || false,
            &mut |line| say(line),
        );
        if self.stopped(say) {
            return Outcome::Failed;
        }
        let carried = match &self.control {
            None => Outcome::Kept,
            Some(Control { parts: Err(_), .. }) => Outcome::Failed,
            Some(_) => {
                let mut parts = self.take_parts();
                let config = self.config.clone();
                let host = self.host().expect("a control plane named");
                let reported = thread::scope(|scope| {
                    let passes: Vec<_> = parts
                        .iter_mut()
                        .map(|part| {
                            let (host, config) = (&host, config.clone());
                            scope.spawn(move || {
                                let relays = host.toolset.relays();
                                let (reach, least, elsewhere) =
                                    (Reach::default(), Least::default(), Elsewhere::default());
                                host.carry_once(
                                    part,
                                    Ok(config),
                                    &relays,
                                    (&reach, &least, &elsewhere),
                                    say,
                                )
                            })
                        })
                        .collect();
                    passes
                        .into_iter()
                        .map(|pass| pass.join().expect("a part's pass ends"))
                        .filter(|passed| passed.reported)
                        .count()
                });
                self.give_back(parts);
                match reported {
                    0 => Outcome::Failed,
                    _ => Outcome::Kept,
                }
            }
        };
        if self.stopped(say) {
            return Outcome::Failed;
        }
        self.ticker.tick(&*self.toolset, say);
        kept.and(carried)
    }

    /// Whether the runner was stopped, saying so where it was.
    fn stopped(&self, say: Say<'_>) -> bool {
        let stopped = self.stop.load(Ordering::SeqCst);
        if stopped {
            say("boomdrive: stopped".to_string());
        }
        stopped
    }

    /// Passes until stopped (design 10 and Decision 2). A keeping pass is
    /// made at once, on the configuration the runner started with, then
    /// one each `fetch_interval` after the start of the one before, the
    /// configuration read again for each, its local layer over its
    /// managed one, and how it read the local layer said where that
    /// differs from the last one said. A pass that cannot read it names
    /// why and keeps nothing. Between passes, and between two
    /// repositories of a keeping pass, each landing it is told of is kept
    /// at once. Where the runner carries a control plane's work, each part
    /// is carried on a thread of its own from the end of the first keeping
    /// pass: a control pass at once, then one each `poll_interval` after
    /// the start of the part's one before, sooner, after a pass that
    /// reported, before a lease of the part's not yet lapsed lapses, and
    /// at once when its implementation wakes it. Each pass, keeping or
    /// control, ends with the ticks of each inbox, before its thread
    /// waits (record 0033). Stopped, it ends the try of a call to a remote
    /// in flight, its own fetch's or a part's, and begins none, through
    /// its halt (record 0042 as amended on 2026-11-12; core.58), keeps no
    /// further repository, begins no part's pass and no tick, waits for
    /// each part to end the pass in hand, and says so.
    pub fn keep(&mut self, say: Say<'_>) {
        let mut parts = self.take_parts();
        let bells: Vec<Arc<Bell>> = parts
            .iter()
            .map(|part| self.bells.of(&part.label))
            .collect();
        let this = &*self;
        // One subscription to flume host for every part, held while the
        // runner carries any (the change the-runner-hears-flume-by-
        // subscription, Decision 1).
        let subscription = Subscription::new(this.flume_socket.clone());
        let carries = !parts.is_empty();
        let host = this.host().map(|host| Host {
            subscription: Some(&subscription),
            ..host
        });
        let mut unstarted = Some(parts.iter_mut().zip(bells.iter()));
        thread::scope(|scope| {
            if carries && host.is_some() {
                let (subscription, stop, bells) = (&subscription, &*this.stop, &this.bells);
                scope.spawn(move || {
                    subscription.hold(stop, &|| bells.wake_all(), &|line| say(line));
                });
            }
            let mut started = Some(this.config.clone());
            let mut fetch_interval = this.config.fetch_interval;
            let mut keep_due = Instant::now();
            let mut said_local = LocalLayer::NoFile;
            'passes: loop {
                let mut kept = false;
                if Instant::now() >= keep_due {
                    kept = true;
                    let began = Instant::now();
                    let read = match started.take() {
                        Some(config) => Ok(config),
                        None => this.read(),
                    };
                    match read {
                        Ok(config) => {
                            this.say_local(&mut said_local, &config.local, say);
                            fetch_interval = config.fetch_interval;
                            this.ticker.read(&config.inboxes);
                            this.keeper.pass(
                                &config,
                                &this.locks,
                                &this.stop,
                                &mut || this.between(say),
                                &mut |line| say(line),
                            );
                        }
                        Err(why) => say(format!("boomdrive: {why}: nothing kept this pass")),
                    }
                    keep_due = next_pass(began, fetch_interval, Instant::now());
                }
                if this.stop.load(Ordering::SeqCst) {
                    break 'passes;
                }
                let mut followed = false;
                if let (Some(host), Some(unstarted)) = (&host, unstarted.take()) {
                    for (part, bell) in unstarted {
                        followed = true;
                        scope.spawn(move || host.carry_on(part, bell, say));
                    }
                }
                // A keeping pass that each part's first control pass
                // follows at once shares their ticks, so no launch waits
                // behind a tick; a keeping pass on a configuration that did
                // not read ticks all the same (record 0033).
                if kept && !followed {
                    this.ticker.tick(&*this.toolset, say);
                }
                loop {
                    if this.stop.load(Ordering::SeqCst) {
                        break 'passes;
                    }
                    let wait = keep_due.saturating_duration_since(Instant::now());
                    match this.waiting.recv_timeout(wait) {
                        Ok(Event::Landed(address)) => this.landed(&address, say),
                        Ok(Event::Stop) => break 'passes,
                        Err(RecvTimeoutError::Timeout) => break,
                        Err(RecvTimeoutError::Disconnected) => {
                            unreachable!("the runner holds a sender of its own")
                        }
                    }
                }
            }
            // Every part begins no further pass, and the scope waits for
            // each to end the pass in hand.
            for bell in &bells {
                bell.stop();
            }
        });
        self.give_back(parts);
        say("boomdrive: stopped".to_string());
    }

    /// Says how a keeping pass read the local layer, `read`, where it
    /// differs from `said`, the last one said, once each way (design
    /// Decision 7; core.23 as the model).
    fn say_local(&self, said: &mut LocalLayer, read: &LocalLayer, say: Say<'_>) {
        if read != said {
            say(local_line(&self.local, read));
            *said = read.clone();
        }
    }

    /// The configuration as it stands, both its layers read again (design
    /// Decision 8).
    fn read(&self) -> Result<Config, String> {
        config::read_layers(&self.file, &self.local, &self.home)
    }

    /// Keeps each landing told since, between two repositories of a
    /// keeping pass, waiting for nothing: a stop told is read by the pass
    /// from the stop itself. Says whether it kept a landing (design
    /// Decision 3).
    fn between(&self, say: Say<'_>) -> bool {
        let mut kept = false;
        for event in self.waiting.try_iter() {
            match event {
                Event::Landed(address) => {
                    self.landed(&address, say);
                    kept = true;
                }
                Event::Stop => {}
            }
        }
        kept
    }

    /// Keeps `address` at once, told of a landing on it, against the
    /// configuration as it stands: read again, so a repository dropped
    /// since the last pass is never fetched. It is kept under its lock.
    fn landed(&self, address: &Address, say: Say<'_>) {
        match self.read() {
            Ok(config) => {
                self.keeper
                    .keep_one(&config, &self.locks, address, &mut |line| say(line));
            }
            Err(why) => say(format!(
                "boomdrive: {why}: nothing kept at the landing on {address}"
            )),
        }
    }
}

/// The line saying how the local layer, the file `file`, was read (design
/// Decision 7): taken, with each key passed over and its pin; not taken,
/// with each key it may not set or its fault; or gone.
fn local_line(file: &Path, read: &LocalLayer) -> String {
    let file = file.display();
    match read {
        LocalLayer::NoFile => format!("boomdrive: {file}: gone, the managed layer read alone"),
        LocalLayer::Taken { passed_over } => {
            let mut line = format!("boomdrive: {file}: taken");
            for pinned in passed_over {
                line += &format!("; `{}` passed over, pinned by `{}`", pinned.key, pinned.pin);
            }
            line
        }
        LocalLayer::NotTaken { keys, fault } => {
            let why = match fault {
                Some(fault) => fault.clone(),
                None => {
                    let named: Vec<String> = keys.iter().map(|key| format!("`{key}`")).collect();
                    match named.as_slice() {
                        [one] => format!("{one} is a key the local layer does not set"),
                        _ => format!("{} are keys the local layer does not set", named.join(", ")),
                    }
                }
            };
            format!(
                "boomdrive: {file}: not taken, the managed layer read alone and nothing let go: \
                 {why}"
            )
        }
    }
}

impl Host<'_> {
    /// Carries `part` on the thread this runs on until the runner stops
    /// (design Decision 2): a pass at once, then each when its timer or
    /// its wake, through `bell`, says, its own reach, terminals, word of
    /// Flume's version and the places it did not launch kept across its
    /// passes, each pass followed by the ticks of each inbox (record
    /// 0033). Each want's command runs
    /// on a thread of its own, kept across the part's passes, and each
    /// answer that comes back rings `bell`, so the part takes it, starts
    /// what waited on it and sends it at once, between its passes (record
    /// 0002 as amended on 2026-11-19; core.50). Stopped, it begins no
    /// further pass and no tick, and waits for every command in flight
    /// to end, taking and sending their answers.
    fn carry_on(&self, part: &mut Part, bell: &Bell, say: Say<'_>) {
        let relays = self.toolset.relays();
        let (reach, least, elsewhere) = (Reach::default(), Least::default(), Elsewhere::default());
        hands::carried(
            self.toolset.flume(),
            self.toolset.sluice(),
            self.locks,
            &|| bell.answered(),
            |underway| {
                let mut due = Instant::now();
                loop {
                    match bell.wait(due) {
                        Rang::Stopped => break,
                        Rang::Answered => {
                            let read = config::read_layers(self.file, self.local, self.home);
                            self.carry(
                                part,
                                read,
                                &relays,
                                (&reach, &least, &elsewhere),
                                say,
                                |carrier, plane| {
                                    carrier.collect(plane, underway, &mut |line| say(line));
                                    None
                                },
                            );
                        }
                        Rang::Due => {
                            let began = Instant::now();
                            bell.begin_pass();
                            let read = config::read_layers(self.file, self.local, self.home);
                            let poll_interval = self.poll_interval;
                            let passed = self.carry(
                                part,
                                read,
                                &relays,
                                (&reach, &least, &elsewhere),
                                say,
                                |carrier, plane| {
                                    Some(carrier.carry_pass(plane, underway, &mut |line| say(line)))
                                },
                            );
                            due = passed.next_due(began, Instant::now(), poll_interval);
                            // The control pass ends with the ticks of each
                            // inbox, no part of it, before the part waits
                            // (record 0033).
                            if !bell.stopped() {
                                self.ticker.tick(self.toolset, say);
                            }
                        }
                    }
                }
                if !underway.idle() {
                    let read = config::read_layers(self.file, self.local, self.home);
                    self.carry(
                        part,
                        read,
                        &relays,
                        (&reach, &least, &elsewhere),
                        say,
                        |carrier, plane| {
                            carrier.finish(plane, underway, &mut |line| say(line));
                            None
                        },
                    );
                }
            },
        );
    }

    /// One control pass of `part`, as `run --once` makes it, against the
    /// configuration as `read` gives it (design Decision 2), waiting for
    /// every command it starts, its lines naming the part's label. A
    /// configuration that did not read is named, and nothing is carried
    /// until it reads again.
    fn carry_once(
        &self,
        part: &mut Part,
        read: Result<Config, String>,
        relays: &Relays,
        (reach, least, elsewhere): (&Reach, &Least, &Elsewhere),
        say: Say<'_>,
    ) -> Passed {
        self.carry(
            part,
            read,
            relays,
            (reach, least, elsewhere),
            say,
            |carrier, plane| Some(carrier.pass(plane, &mut |line| say(line))),
        )
    }

    /// `body` with the carrier of `part` against the configuration as
    /// `read` gives it (design Decision 2), its lines naming the part's
    /// label, and the part's control plane. A configuration that did not
    /// read is named, and nothing is carried until it reads again.
    #[allow(clippy::too_many_arguments)]
    fn carry(
        &self,
        part: &mut Part,
        read: Result<Config, String>,
        relays: &Relays,
        (reach, least, elsewhere): (&Reach, &Least, &Elsewhere),
        say: Say<'_>,
        body: impl FnOnce(&Carrier, &mut dyn ControlPlane) -> Option<Passed>,
    ) -> Passed {
        let label = &part.label;
        let unread = |why: String| {
            say(format!(
                "boomdrive: {label}: {why}: nothing carried this pass"
            ));
            Passed {
                expires: None,
                reported: false,
            }
        };
        let config = match read {
            Ok(config) => config,
            Err(why) => return unread(why),
        };
        let clones = match clone::find(self.git, self.data) {
            Ok(clones) => Configured::new(&config, clones),
            Err(error) => {
                let repositories = clone::repositories(self.data);
                return unread(format!(
                    "{} cannot be read: {error}",
                    repositories.display()
                ));
            }
        };
        let flume = self.toolset.flume_reading(match self.subscription {
            Some(subscription) => Source::Subscription(subscription),
            None => Source::Ask(self.flume_socket),
        });
        let control_dir = report::control_dir(self.data);
        let carrier = Carrier {
            label,
            flume: &*flume,
            sluice: self.toolset.sluice(),
            git: self.git,
            clones: &clones,
            tools: self.toolset.versions(),
            notices: self.notices,
            control_dir: &control_dir,
            report: self.report,
            locks: self.locks,
            relays,
            reach,
            least,
            elsewhere,
        };
        body(&carrier, part.plane.as_mut()).unwrap_or(Passed {
            expires: None,
            reported: false,
        })
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
/// keeping pass and one control pass of each part where `once`, else
/// passes until SIGTERM or SIGINT stops it. Either way SIGTERM and SIGINT
/// stop it, ending each try of a call to a remote in flight (record 0042
/// as amended on 2026-11-12; core.58). Exits 2 where the runner does not
/// start, 1 where `once`'s passes failed or were stopped, and 0 otherwise.
pub fn main(file: Option<&Path>, once: bool) -> ExitCode {
    let started =
        Runner::start(file, |name| std::env::var_os(name), Git::default()).and_then(|runner| {
            runner.stop_on_signals()?;
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
    let say = |line: String| eprintln!("{line}");
    if !once {
        runner.keep(&say);
        return ExitCode::SUCCESS;
    }
    match runner.once(&say) {
        Outcome::Kept => ExitCode::SUCCESS,
        Outcome::Failed => ExitCode::from(1),
    }
}

#[cfg(test)]
mod tests {
    use super::{next_pass, Bell, Bells, Event, Rang, Stopper, Ticker};
    use crate::carry::next_due;
    use std::fs;
    use std::os::unix::fs::PermissionsExt;
    use std::time::{Duration, Instant};

    /// The running spec's "A pass ends while a tick is under way": a pass
    /// that ends while another's ticks run runs none, and returns without
    /// waiting for them, which it could not do here, the first tick running
    /// until the test lets it go; a pass after the first ticks ended runs
    /// its own.
    #[test]
    fn a_tick_under_way_is_shared() {
        let dir = tempfile::tempdir().unwrap();
        let (millpond, calls, go) = (
            dir.path().join("millpond"),
            dir.path().join("calls"),
            dir.path().join("go"),
        );
        let program = format!(
            "#!/bin/sh\necho \"$3\" >> '{}'\nwhile [ ! -e '{}' ]; do sleep 0.02; done\n\
             echo '{{}}'\n",
            calls.display(),
            go.display()
        );
        fs::write(&millpond, program).unwrap();
        fs::set_permissions(&millpond, fs::Permissions::from_mode(0o755)).unwrap();
        let ticker = Ticker::new(vec!["home".to_string()]);
        let programs = super::Clis::from(super::Programs {
            flume: dir.path().join("flume"),
            sluice: dir.path().join("sluice"),
            millpond,
        });
        let said = std::sync::Mutex::new(Vec::new());
        let say = |line: String| said.lock().unwrap().push(line);
        std::thread::scope(|scope| {
            let first = scope.spawn(|| ticker.tick(&programs, &say));
            while !calls.exists() {
                std::thread::sleep(Duration::from_millis(10));
            }
            ticker.tick(&programs, &say);
            fs::write(&go, "").unwrap();
            first.join().unwrap();
        });
        assert_eq!(fs::read_to_string(&calls).unwrap(), "home\n");
        ticker.tick(&programs, &say);
        assert_eq!(fs::read_to_string(&calls).unwrap(), "home\nhome\n");
        assert_eq!(*said.lock().unwrap(), [] as [String; 0]);
    }

    /// A part's wake is read by its pass under way until its next pass
    /// begins (core.50), and ends its wait at once; the wake of one part
    /// wakes no other.
    #[test]
    fn a_wake_is_read_by_its_parts_pass_under_way() {
        let bells = Bells::default();
        let (home, work) = (bells.of("home"), bells.of("work"));
        let wake = bells.waker("home");
        assert!(!home.woken());
        wake();
        assert!(home.woken());
        assert!(!work.woken());
        let later = Instant::now() + Duration::from_secs(3600);
        assert_eq!(home.wait(later), Rang::Due);
        home.begin_pass();
        assert!(!home.woken());
    }

    /// A stop ends every part's wait, and a stopped part begins no pass,
    /// however due.
    #[test]
    fn a_stop_ends_every_wait() {
        let bells = Bells::default();
        let bell = bells.of("home");
        let waiting =
            std::thread::spawn(move || bell.wait(Instant::now() + Duration::from_secs(3600)));
        std::thread::sleep(Duration::from_millis(50));
        bells.stop();
        assert_eq!(waiting.join().unwrap(), Rang::Stopped);
        let stopped = Bell::default();
        stopped.stop();
        stopped.wake();
        assert_eq!(stopped.wait(Instant::now()), Rang::Stopped);
    }

    /// The running spec's "Stopped with a fetch in flight" and "Stopped
    /// with a part's read in flight", at the stopper: a stop sets the stop
    /// flag and the runner's halt, tells the loop and stops every part
    /// (record 0042 as amended on 2026-11-12; core.58).
    #[test]
    fn a_stop_halts_every_try() {
        let (events, waiting) = std::sync::mpsc::channel();
        let stop = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
        let bells = Bells::default();
        let bell = bells.of("home");
        let halt = weir_bounded::Halt::default();
        let stopper = Stopper {
            events,
            stop: std::sync::Arc::clone(&stop),
            bells,
            halt: halt.clone(),
        };
        stopper.stop();
        assert!(stop.load(std::sync::atomic::Ordering::SeqCst));
        assert!(halt.halted());
        assert!(matches!(waiting.try_recv(), Ok(Event::Stop)));
        assert!(bell.stopped());
    }

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
    /// one minute, `fetch_interval` two minutes, and a control pass of
    /// `home` and a keeping pass begun at 10:00:00, `home`'s next control
    /// pass begins at 10:01:00, and the next keeping pass at 10:02:00,
    /// each on its own timer.
    #[test]
    fn on_the_control_timer() {
        let ten = Instant::now();
        let ended = ten + Duration::from_secs(5);
        let minute = Duration::from_secs(60);
        let control = next_due(ten, ended, minute, None, true);
        let keeping = next_pass(ten, 2 * minute, ended);
        assert_eq!(control, ten + minute);
        assert_eq!(keeping, ten + 2 * minute);
        let next_control = next_due(
            control,
            control + Duration::from_secs(5),
            minute,
            None,
            true,
        );
        assert_eq!(
            next_control, keeping,
            "the second control pass is due with the keeping pass"
        );
    }
}
