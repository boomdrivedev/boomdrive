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
//! ends with Flume's settle of every place, which keeps Flume's watch
//! over the host while the runner runs, and which no two passes run at
//! once (record 0029). The runner opens no listening socket: every
//! connection it makes is outgoing (core.3).

use crate::carry::{Carrier, Configured, Passed, Reach};
use crate::config::{self, Address, Config, ControlPlaneTable, Env};
use crate::git::Git;
use crate::locks::Locks;
use crate::pass::{self, Outcome};
use crate::planes;
use crate::report::{self, ReportFile};
use crate::settle::{self, Settles};
use crate::terminal::{self, Relays};
use crate::{clone, flume, hello, sluice};
use boomdrive_control::{Part, Wake};
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

/// A part's own wake, and the runner's stop as that part's thread waits
/// for it: what the part's thread waits on between its passes, and what
/// the pass under way reads to know the next is due (core.50).
#[derive(Debug, Default)]
pub struct Bell {
    rung: Mutex<Rung>,
    changed: Condvar,
}

#[derive(Debug, Default)]
struct Rung {
    /// Woken since the part's pass under way began.
    woken: bool,
    stopped: bool,
}

/// What a part's thread is told once it has waited.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rang {
    /// Its next pass is due by its timer, or it was woken.
    Due,
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

    /// Takes the wake that a pass beginning now answers.
    pub fn begin_pass(&self) {
        self.rung().woken = false;
    }

    /// Waits until `due`, or until the part is woken or stopped.
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

    /// Stops every part.
    fn stop(&self) {
        for bell in self.bells().values() {
            bell.stop();
        }
    }
}

/// Stops the runner: it finishes the git call in hand, keeps no further
/// repository, begins no part's pass, lets each part end the pass in
/// hand, and says it stopped.
#[derive(Clone)]
pub struct Stopper {
    events: Sender<Event>,
    stop: Arc<AtomicBool>,
    bells: Bells,
}

impl Stopper {
    /// Stops the runner, its keeping pass read between repositories, its
    /// wait woken, and every part's wait woken.
    pub fn stop(&self) {
        self.stop.store(true, Ordering::SeqCst);
        let _ = self.events.send(Event::Stop);
        self.bells.stop();
    }
}

/// When the pass after one that began at `last_start` begins:
/// `fetch_interval` after it, or `now` where that is past (design 10).
pub fn next_pass(last_start: Instant, interval: Duration, now: Instant) -> Instant {
    (last_start + interval).max(now)
}

/// The programs the runner runs for Flume, sluice and herdr: its settles
/// and its parts' control passes'.
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
    /// How often each part's control pass is made, as the configuration
    /// said at start.
    poll_interval: Duration,
    /// The implementation's parts, or why it could not be opened.
    parts: Result<Vec<Part>, String>,
}

impl Control {
    /// The control plane `table` names, of the configuration file `file`,
    /// keeping what it keeps under `data`, each part woken through its
    /// bell among `bells`. An implementation says its lines on standard
    /// error.
    fn open(table: &ControlPlaneTable, file: &Path, data: &Path, bells: &Bells) -> Control {
        Control {
            poll_interval: table.poll_interval,
            parts: planes::open(
                &table.implementation,
                &table.implementation_table,
                file,
                data,
                table.poll_interval,
                &|_| Box::new(|line| eprintln!("{line}")),
                &|label| bells.waker(label),
            ),
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
    /// The programs the runner runs for Flume, sluice and herdr, each on
    /// the host's `PATH` unless a test puts its own in place.
    programs: Programs,
    /// The control plane the configuration names, none where it names
    /// none.
    control: Option<Control>,
    /// The runner's settles of every place, shared by the loop and every
    /// part's thread.
    watch: Watch,
    events: Sender<Event>,
    notices: Notices,
    waiting: Receiver<Event>,
    stop: Arc<AtomicBool>,
    /// Each part's bell, by its label.
    bells: Bells,
    /// Each clone's lock, which the keeper takes alone and a launch into
    /// the clone shared (design Decision 4).
    locks: Locks,
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
    data: &'a Path,
    poll_interval: Duration,
    programs: &'a Programs,
    notices: &'a Notices,
    locks: &'a Locks,
    report: &'a ReportFile,
    watch: &'a Watch,
}

/// The runner's watch over the host: Flume's settle of every place,
/// `flume settle --every <bound>`, run once and awaited at the end of
/// each pass, keeping or control (core.13; record 0029). The loop and
/// every part's thread share it, so no two settles run at once: a pass
/// that ends while another's settle is under way shares that settle, as a
/// keeping pass shares the settle of the control pass that follows it at
/// once, and waits for nothing (core.49).
struct Watch {
    /// The `fetch_interval` the latest reading of the configuration gave.
    fetch_interval: Mutex<Duration>,
    /// Whether the last settle was answered, held while a settle runs.
    settles: Mutex<Settles>,
}

impl Watch {
    /// A watch whose bound starts from `fetch_interval`, as the
    /// configuration the runner started with gave it.
    fn new(fetch_interval: Duration) -> Watch {
        Watch {
            fetch_interval: Mutex::new(fetch_interval),
            settles: Mutex::default(),
        }
    }

    /// Takes the `fetch_interval` a reading of the configuration gave.
    fn read(&self, fetch_interval: Duration) {
        *self
            .fetch_interval
            .lock()
            .unwrap_or_else(|e| e.into_inner()) = fetch_interval;
    }

    /// Runs `flume`'s settle of every place, its bound the latest
    /// `fetch_interval`, or the shorter of it and `poll_interval` while
    /// the runner makes control passes, and says its answer only where it
    /// changes. Where another pass's settle is under way, runs none.
    fn settle(&self, flume: &Path, poll_interval: Option<Duration>, say: Say<'_>) {
        let mut settles = match self.settles.try_lock() {
            Ok(settles) => settles,
            Err(std::sync::TryLockError::Poisoned(poisoned)) => poisoned.into_inner(),
            Err(std::sync::TryLockError::WouldBlock) => return,
        };
        let fetch_interval = *self
            .fetch_interval
            .lock()
            .unwrap_or_else(|e| e.into_inner());
        let bound = settle::bound(fetch_interval, poll_interval);
        let answer = flume::Cli::new(flume).settle_every(bound);
        settles.heard(answer, &mut |line| say(line));
    }
}

impl Runner {
    /// Starts a runner on the configuration `file` names, else the
    /// default one, in the environment `var` gives. The start's checks
    /// run in design 10's order, and the first that fails refuses with
    /// why, having touched nothing: `HOME` is set, the configuration
    /// reads, git is 2.48 or later, the data directory is there or can be
    /// made, and no other runner holds it. Then the control plane the
    /// configuration names is opened as its parts; one that cannot be is
    /// named by [`Runner::unopened`], and the runner keeps its
    /// repositories all the same.
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
        let bells = Bells::default();
        let control = config
            .control_plane
            .as_ref()
            .map(|table| Control::open(table, &file, &data, &bells));
        let report = ReportFile::new(&report::control_dir(&data));
        let watch = Watch::new(config.fetch_interval);
        Ok(Runner {
            git,
            home: env.home,
            file,
            data,
            config,
            programs: Programs::on_path(),
            control,
            watch,
            events,
            notices,
            waiting,
            stop: Arc::new(AtomicBool::new(false)),
            bells,
            locks: Locks::new(),
            report,
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
        self.programs = programs;
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
            data: &self.data,
            poll_interval: control.poll_interval,
            programs: &self.programs,
            notices: &self.notices,
            locks: &self.locks,
            report: &self.report,
            watch: &self.watch,
        })
    }

    /// The interval between control passes where the runner makes them,
    /// its control plane opened as parts; none where it makes none.
    fn polls(&self) -> Option<Duration> {
        match &self.control {
            Some(Control {
                poll_interval,
                parts: Ok(parts),
            }) if !parts.is_empty() => Some(*poll_interval),
            _ => None,
        }
    }

    /// One keeping pass over the configuration the runner started with,
    /// then, where it names a control plane, one control pass of each
    /// part, side by side (design Decision 2): failed where a git call of
    /// the keeping pass failed, or no part both pulled and reported, its
    /// control plane unopened among it. Each part's pass carries out every
    /// act, since no pass follows it (core.50). Then the settle that ends
    /// them (record 0029): a settle refused or unanswered is said, and no
    /// failure.
    pub fn once(&mut self, say: Say<'_>) -> Outcome {
        let polls = self.polls();
        let kept = pass::pass(
            &self.git,
            &self.data,
            &self.config,
            &self.locks,
            &self.stop,
            &mut || false,
            &mut |line| say(line),
        );
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
                                let relays = host.programs.relays();
                                let reach = Reach::default();
                                host.carry(part, Ok(config), &relays, &reach, &|| false, say)
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
        self.watch.settle(&self.programs.flume, polls, say);
        kept.and(carried)
    }

    /// Passes until stopped (design 10 and Decision 2). A keeping pass is
    /// made at once, on the configuration the runner started with, then
    /// one each `fetch_interval` after the start of the one before, the
    /// configuration read again for each. A pass that cannot read it
    /// names why and keeps nothing. Between passes, and between two
    /// repositories of a keeping pass, each landing it is told of is kept
    /// at once. Where the runner carries a control plane's work, each part
    /// is carried on a thread of its own from the end of the first keeping
    /// pass: a control pass at once, then one each `poll_interval` after
    /// the start of the part's one before, sooner, after a pass that
    /// reported, before a lease of the part's not yet lapsed lapses, and
    /// at once when its implementation wakes it. Each pass, keeping or
    /// control, ends with one settle of every place, before its thread
    /// waits (record 0029). Stopped, it keeps no further repository,
    /// begins no part's pass and no settle, waits for each part to end the
    /// pass in hand, and says so.
    pub fn keep(&mut self, say: Say<'_>) {
        let polls = self.polls();
        let mut parts = self.take_parts();
        let bells: Vec<Arc<Bell>> = parts
            .iter()
            .map(|part| self.bells.of(&part.label))
            .collect();
        let this = &*self;
        let host = this.host();
        let mut unstarted = Some(parts.iter_mut().zip(bells.iter()));
        thread::scope(|scope| {
            let mut started = Some(this.config.clone());
            let mut fetch_interval = this.config.fetch_interval;
            let mut keep_due = Instant::now();
            'passes: loop {
                let mut kept = false;
                if Instant::now() >= keep_due {
                    kept = true;
                    let began = Instant::now();
                    let read = match started.take() {
                        Some(config) => Ok(config),
                        None => config::read(&this.file, &this.home),
                    };
                    match read {
                        Ok(config) => {
                            fetch_interval = config.fetch_interval;
                            this.watch.read(fetch_interval);
                            pass::pass(
                                &this.git,
                                &this.data,
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
                // follows at once shares their settle, so no launch waits
                // behind a settle of every place; a keeping pass on a
                // configuration that did not read settles all the same
                // (record 0029).
                if kept && !followed {
                    this.watch.settle(&this.programs.flume, polls, say);
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
        match config::read(&self.file, &self.home) {
            Ok(config) => {
                pass::keep_one(
                    &self.git,
                    &self.data,
                    &config,
                    &self.locks,
                    address,
                    &mut |line| say(line),
                );
            }
            Err(why) => say(format!(
                "boomdrive: {why}: nothing kept at the landing on {address}"
            )),
        }
    }
}

impl Host<'_> {
    /// Carries `part` on the thread this runs on until the runner stops
    /// (design Decision 2): a pass at once, then each when its timer or
    /// its wake, through `bell`, says, its own reach and terminals kept
    /// across its passes, each pass followed by the settle of every place
    /// (record 0029). Stopped, it begins no further pass and no settle.
    fn carry_on(&self, part: &mut Part, bell: &Bell, say: Say<'_>) {
        let relays = self.programs.relays();
        let reach = Reach::default();
        let mut due = Instant::now();
        while bell.wait(due) == Rang::Due {
            let began = Instant::now();
            bell.begin_pass();
            let read = config::read(self.file, self.home);
            let poll_interval = self.poll_interval;
            let is_due = || began.elapsed() >= poll_interval || bell.woken();
            let passed = self.carry(part, read, &relays, &reach, &is_due, say);
            due = passed.next_due(began, Instant::now(), poll_interval);
            // The control pass ends with the settle of every place, no
            // part of it, before the part waits (record 0029).
            if !bell.stopped() {
                self.watch
                    .settle(&self.programs.flume, Some(poll_interval), say);
            }
        }
    }

    /// One control pass of `part` against the configuration as `read`
    /// gives it (design Decision 2), `due` saying whether the part's next
    /// is due, its lines naming the part's label. A configuration that did
    /// not read is named, and nothing is carried until it reads again.
    fn carry(
        &self,
        part: &mut Part,
        read: Result<Config, String>,
        relays: &Relays,
        reach: &Reach,
        due: &dyn Fn() -> bool,
        say: Say<'_>,
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
        let flume = flume::Cli::new(&self.programs.flume);
        let sluice = sluice::Cli::new(&self.programs.sluice);
        let tools = self.programs.tools();
        let control_dir = report::control_dir(self.data);
        let carrier = Carrier {
            label,
            flume: &flume,
            sluice: &sluice,
            git: self.git,
            clones: &clones,
            tools: &tools,
            notices: self.notices,
            control_dir: &control_dir,
            report: self.report,
            locks: self.locks,
            relays,
            reach,
            due,
        };
        carrier.pass(part.plane.as_mut(), &mut |line| say(line))
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
/// passes until SIGTERM or SIGINT stops it. Exits 2 where the runner does
/// not start, 1 where `once`'s passes failed, and 0 otherwise.
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
    use super::{next_pass, Bell, Bells, Rang};
    use crate::carry::next_due;
    use std::time::{Duration, Instant};

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
