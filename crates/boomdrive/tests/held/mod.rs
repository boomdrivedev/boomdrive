//! A runner under test held in memory, on a paused clock: its keeper, its
//! tools and its control planes the test's own, so a test of the runner's
//! loop starts no process, git or socket, and its clock moves only when
//! the test moves it (logic-is-tested-as-units-and-few-tests-run-the-
//! binary; the-runtime-flavour-is-chosen-in-main-and-tests-pause-tokios-
//! clock).

#![allow(dead_code)]

use crate::fakes::{offer, Call, FakeFlume, KEPT};
use boomdrive::config::{Address, Config};
use boomdrive::flume::{Act, Flume, LaunchArgs, Reply, Unrun};
use boomdrive::flume_host::Source;
use boomdrive::git::Git;
use boomdrive::locks::Locks;
use boomdrive::pass::{self, Between, Keeper};
use boomdrive::run::{Edge, Notices, Runner, Stopper, Toolset};
use boomdrive::sluice::{self, Sluice};
use boomdrive::terminal::Relays;
use boomdrive::{flume, hello, millpond, report};
use boomdrive_control::fake::Fake;
use boomdrive_control::{
    Answer, At, ControlPlane, Hello, Id, Lease, Outcome, Part, Pulled, Report, Reported,
    ToolVersion, Unread, Unsent, Wake, Want,
};
use std::collections::BTreeMap;
use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::sync::atomic::AtomicBool;
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Condvar, Mutex};
use std::thread::{self, JoinHandle};
use std::time::Duration;
use tempfile::TempDir;
use tokio::runtime::Runtime;
use tokio::time::Instant;

use boomdrive::clone::Found;

/// How long a test waits, in real time, for the runner's threads to do
/// what its clock has made due: a guard against a hang, never a bound a
/// test asserts.
pub const PATIENCE: Duration = Duration::from_secs(60);

/// How long a test waits, in real time, to see that the runner does
/// nothing that is not due.
pub const QUIET: Duration = Duration::from_millis(200);

/// A second of the edge's clock.
pub const SECOND: Duration = Duration::from_secs(1);

/// An hour of the edge's clock.
pub const HOUR: Duration = Duration::from_secs(3600);

/// A current-thread runtime whose clock is paused, and its edge, which a
/// runner under test and its control planes read.
pub struct Clock {
    runtime: Runtime,
    edge: Edge,
}

impl Clock {
    pub fn paused() -> Clock {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_time()
            .start_paused(true)
            .build()
            .expect("a runtime");
        let edge = Edge::of(runtime.handle());
        Clock { runtime, edge }
    }
}

/// The keeper of a runner under test: it keeps nothing, notes when each
/// keeping pass began, holds a pass under the clone's lock while its gate
/// holds it, and finds the one clone of [`KEPT`], recording the drive
/// `home`.
pub struct Keeps {
    container: PathBuf,
    edge: Edge,
    notes: Arc<Notes>,
    gate: Arc<Gate>,
}

/// What a runner under test's keeper noted, and what it is told to say.
#[derive(Default)]
pub struct Notes {
    /// When each keeping pass began, and the repositories its
    /// configuration keeps.
    passes: Mutex<Vec<(Instant, Vec<String>)>>,
    /// Each repository kept alone at a landing.
    landings: Mutex<Vec<String>>,
    /// Words each keeping pass says, failing, while it is set.
    failing: Mutex<Option<String>>,
}

/// Whether a keeping pass is to be held, and whether one is.
#[derive(Default)]
pub struct Gate {
    state: Mutex<(bool, bool)>,
    changed: Condvar,
}

impl Keeper for Keeps {
    fn ready(&self) -> Result<(), String> {
        Ok(())
    }

    fn found(&self) -> std::io::Result<Vec<Found>> {
        Ok(vec![Found {
            address: Address::parse(KEPT).expect("an address"),
            container: self.container.clone(),
            drive: Some("home".to_string()),
        }])
    }

    fn pass(
        &self,
        config: &Config,
        locks: &Locks,
        _: &AtomicBool,
        _: Between<'_>,
        say: &mut dyn FnMut(String),
    ) -> pass::Outcome {
        let kept = config
            .repositories()
            .map(|address| address.to_string())
            .collect();
        self.notes
            .passes
            .lock()
            .expect("the passes")
            .push((self.edge.now(), kept));
        if let Some(words) = self.notes.failing.lock().expect("the failing").clone() {
            say(words);
            return pass::Outcome::Failed;
        }
        let mut state = self.gate.state.lock().expect("the gate");
        if state.0 {
            let address = Address::parse(KEPT).expect("an address");
            let _kept = locks.keep(&address);
            state.1 = true;
            self.gate.changed.notify_all();
            while state.0 {
                state = self.gate.changed.wait(state).expect("the gate");
            }
            state.1 = false;
        }
        pass::Outcome::Kept
    }

    fn keep_one(
        &self,
        _: &Config,
        _: &Locks,
        address: &Address,
        say: &mut dyn FnMut(String),
    ) -> pass::Outcome {
        self.notes
            .landings
            .lock()
            .expect("the landings")
            .push(address.to_string());
        say(format!("{address}: kept at its landing"));
        pass::Outcome::Kept
    }
}

/// The tools of a runner under test, held in memory: Flume, a sluice no
/// pass here asks, the tools' versions, and millpond, which notes each
/// inbox it ticks and answers with nothing.
pub struct Held {
    flume: Arc<FakeFlume>,
    ticks: Arc<Mutex<Vec<String>>>,
    refused: Arc<Mutex<Vec<String>>>,
}

/// Flume as a part's pass reads it: the fake, whatever flume host says.
pub struct Reading<'a>(&'a FakeFlume);

impl Flume for Reading<'_> {
    fn launch(&self, args: &LaunchArgs, brief_dir: &Path) -> Result<Reply, Unrun> {
        self.0.launch(args, brief_dir)
    }

    fn act(&self, place: &str, act: &Act) -> Result<Reply, Unrun> {
        self.0.act(place, act)
    }

    fn status(&self, repositories: &[String]) -> Result<Reply, Unrun> {
        self.0.status(repositories)
    }

    fn status_of(&self, container_dir: Option<&Path>, place: &str) -> Result<Reply, Unrun> {
        self.0.status_of(container_dir, place)
    }

    fn confirm(&self, recipient: &str, keys: &[String]) -> Result<Reply, Unrun> {
        self.0.confirm(recipient, keys)
    }
}

/// A sluice no pass here asks.
pub struct NoSluice;

/// What [`NoSluice`] answers.
pub fn no_sluice() -> sluice::Unrun {
    sluice::Unrun("no sluice runs in this test".to_string())
}

impl Sluice for NoSluice {
    fn entry(&self, _: &Path, _: &str, _: &str) -> Result<sluice::Reply, sluice::Unrun> {
        Err(no_sluice())
    }

    fn preview(&self, _: &Path, _: &str, _: &str) -> Result<sluice::Asked, sluice::Unrun> {
        Err(no_sluice())
    }

    fn previews(&self, _: &Path) -> Result<sluice::Reply, sluice::Unrun> {
        Err(no_sluice())
    }

    fn release(&self, _: &Path, _: &str) -> Result<sluice::Reply, sluice::Unrun> {
        Err(no_sluice())
    }
}

/// Every tool's version, `<tool> 1.0.0`.
pub struct Versions;

impl hello::Tools for Versions {
    fn version(&self, tool: &str) -> ToolVersion {
        ToolVersion::Printed(format!("{tool} 1.0.0"))
    }
}

impl Toolset for Held {
    fn flume(&self) -> &dyn Flume {
        &*self.flume
    }

    fn flume_reading<'a>(&'a self, _: Source<'a>) -> Box<dyn Flume + 'a> {
        Box::new(Reading(&self.flume))
    }

    fn sluice(&self) -> &dyn Sluice {
        &NoSluice
    }

    fn versions(&self) -> &dyn hello::Tools {
        &Versions
    }

    fn tick(&self, inbox: &str) -> Result<millpond::Reply, millpond::Unrun> {
        self.ticks
            .lock()
            .expect("the ticks")
            .push(inbox.to_string());
        if self
            .refused
            .lock()
            .expect("the refused")
            .iter()
            .any(|refused| refused == inbox)
        {
            return Ok(millpond::Reply::Refused("millpond exited 1".to_string()));
        }
        Ok(millpond::Reply::Given("{}".to_string()))
    }

    fn relays(&self) -> Relays {
        Relays::new(flume::Cli::new("flume"), |_| {})
    }
}

/// What a test hears of a part's control plane: each pull and each
/// report it took, at the edge's time.
pub struct Heard {
    pub pulls: Receiver<Instant>,
    pub reports: Receiver<Instant>,
}

impl Heard {
    /// The time of the next pull.
    pub fn pull(&self) -> Instant {
        self.pulls.recv_timeout(PATIENCE).expect("a pull made")
    }

    /// The time of the next report taken.
    pub fn report(&self) -> Instant {
        self.reports.recv_timeout(PATIENCE).expect("a report taken")
    }

    /// Whether no pull comes while the clock stands.
    pub fn quiet(&self) -> bool {
        self.pulls.recv_timeout(QUIET).is_err()
    }
}

/// `fake`, telling the test of each pull and each report it takes, its
/// first `unread` pulls refused with `no network`.
pub struct Told {
    fake: Fake,
    unread: usize,
    edge: Edge,
    pulls: Sender<Instant>,
    reports: Sender<Instant>,
}

impl ControlPlane for Told {
    fn pull(&mut self, hello: &Hello, report: &Report) -> Result<Pulled, Unread> {
        let _ = self.pulls.send(self.edge.now());
        if self.unread > 0 {
            self.unread -= 1;
            return Err(Unread("no network".to_string()));
        }
        self.fake.pull(hello, report)
    }

    fn report(&mut self, report: &Report) -> Result<Reported, Unsent> {
        let reported = self.fake.report(report)?;
        let _ = self.reports.send(self.edge.now());
        Ok(reported)
    }

    fn owns(&self, id: &Id) -> bool {
        self.fake.owns(id)
    }

    fn read_through_the_runner(&self) -> bool {
        self.fake.read_through_the_runner()
    }
}

/// `fake`, its pulls refused with `no network` the first `unread` times,
/// and what the test hears of it.
pub fn told(clock: &Clock, fake: Fake, unread: usize) -> (Box<dyn ControlPlane + Send>, Heard) {
    let (pulls, pulled) = mpsc::channel();
    let (reports, reported) = mpsc::channel();
    let plane = Told {
        fake,
        unread,
        edge: clock.edge.clone(),
        pulls,
        reports,
    };
    let heard = Heard {
        pulls: pulled,
        reports: reported,
    };
    (Box::new(plane), heard)
}

/// `fake`, each pull of which waits until the test lets it go, telling
/// the test when it begins.
pub struct Slow {
    fake: Fake,
    edge: Edge,
    pulling: Sender<Instant>,
    go: Receiver<()>,
    reports: Sender<Instant>,
}

impl ControlPlane for Slow {
    fn pull(&mut self, hello: &Hello, report: &Report) -> Result<Pulled, Unread> {
        let _ = self.pulling.send(self.edge.now());
        let _ = self.go.recv();
        self.fake.pull(hello, report)
    }

    fn report(&mut self, report: &Report) -> Result<Reported, Unsent> {
        let reported = self.fake.report(report)?;
        let _ = self.reports.send(self.edge.now());
        Ok(reported)
    }

    fn owns(&self, id: &Id) -> bool {
        self.fake.owns(id)
    }

    fn read_through_the_runner(&self) -> bool {
        self.fake.read_through_the_runner()
    }
}

/// `fake` as a slow control plane, what the test hears of it, and what
/// lets its pull go.
pub fn slow(clock: &Clock, fake: Fake) -> (Box<dyn ControlPlane + Send>, Heard, Sender<()>) {
    let (pulling, pulled) = mpsc::channel();
    let (reports, reported) = mpsc::channel();
    let (go, gone) = mpsc::channel();
    let plane = Slow {
        fake,
        edge: clock.edge.clone(),
        pulling,
        go: gone,
        reports,
    };
    let heard = Heard {
        pulls: pulled,
        reports: reported,
    };
    (Box::new(plane), heard, go)
}

/// The part `label`'s control plane, wanting `wants`.
pub fn part_holding(label: &str, wants: Vec<Want>) -> Fake {
    let mut fake = Fake::owning(&format!("{label}/"));
    fake.set_wants(wants);
    fake
}

/// An offer of the launch `id`, named after its last segment.
pub fn launch(id: &str) -> Want {
    Want::Offer(offer(id, id.rsplit('/').next().unwrap_or(id)))
}

/// A configuration keeping [`KEPT`] each hour and carrying crew's work for
/// the host `host-a` in the partition `home`, each `poll_interval`; a test
/// gives the runner its parts in place of crew's.
pub fn carrying(poll_interval: &str) -> String {
    format!(
        "fetch_interval = \"1h\"\n\
         accounts.own.drives.home.repositories = [{KEPT:?}]\n\
         accounts.own.control_plane = \"crew\"\n\
         [control_plane]\n\
         poll_interval = {poll_interval:?}\n\n\
         [control_plane.crew]\n\
         host = \"host-a\"\n\n\
         [[control_plane.crew.partitions]]\n\
         label = \"home\"\n\
         state_repository = \"example/home-state\"\n"
    )
}

/// A runner under test, carrying the parts it was given over its keeper
/// and tools in memory, on its clock.
pub struct Running {
    notices: Notices,
    flume: Arc<FakeFlume>,
    refused: Arc<Mutex<Vec<String>>>,
    wakes: BTreeMap<String, Wake>,
    pub stopper: Stopper,
    lines: Receiver<(Instant, String)>,
    thread: JoinHandle<()>,
    notes: Arc<Notes>,
    gate: Arc<Gate>,
    ticks: Arc<Mutex<Vec<String>>>,
    pub data: PathBuf,
    clock: Clock,
    home: TempDir,
}

impl Running {
    /// A runner over [`carrying`]'s configuration at `poll_interval`,
    /// carrying `parts` over `flume`, its loop on a thread of its own.
    pub fn carrying(
        clock: Clock,
        poll_interval: &str,
        parts: Vec<(&str, Box<dyn ControlPlane + Send>)>,
        flume: FakeFlume,
    ) -> Running {
        Running::over(clock, &carrying(poll_interval), parts, flume, None)
    }

    /// A runner over the configuration `text`, which names no control
    /// plane, keeping its repositories alone.
    pub fn keeping(clock: Clock, text: &str) -> Running {
        Running::over(clock, text, Vec::new(), FakeFlume::new(), None)
    }

    /// A runner keeping its repositories alone over the managed layer
    /// `managed` and the local layer `local`.
    pub fn layered(clock: Clock, managed: &str, local: &str) -> Running {
        Running::start(
            clock,
            managed,
            Some(local),
            Vec::new(),
            FakeFlume::new(),
            None,
        )
    }

    /// A runner over the configuration `text`, carrying `parts` over
    /// `flume`, its report file holding `report` where one is given when
    /// it starts, its loop on a thread of its own.
    pub fn over(
        clock: Clock,
        text: &str,
        parts: Vec<(&str, Box<dyn ControlPlane + Send>)>,
        flume: FakeFlume,
        report: Option<Report>,
    ) -> Running {
        Running::start(clock, text, None, parts, flume, report)
    }

    /// A runner as [`Running::over`] starts it, its local layer `local`
    /// where one is given.
    fn start(
        clock: Clock,
        text: &str,
        local: Option<&str>,
        parts: Vec<(&str, Box<dyn ControlPlane + Send>)>,
        flume: FakeFlume,
        report: Option<Report>,
    ) -> Running {
        let home = tempfile::tempdir().expect("a home");
        let file = home.path().join(".config/boomdrive/config.toml");
        std::fs::create_dir_all(file.parent().expect("its folder")).expect("its folder");
        std::fs::write(&file, text).expect("the configuration");
        if let Some(local) = local {
            std::fs::write(file.with_file_name("config.local.toml"), local)
                .expect("the local layer");
        }
        let data = home.path().join(".local/share/boomdrive");
        let container = data.join("repositories").join(KEPT);
        std::fs::create_dir_all(&container).expect("the clone's container");
        if let Some(report) = report {
            let control_dir = report::control_dir(&data);
            std::fs::create_dir_all(&control_dir).expect("the control directory");
            report::save(&report::file(&control_dir), &report).expect("the report saved");
        }
        let root = home.path().to_path_buf();
        let var = move |name: &str| (name == "HOME").then(|| OsString::from(&root));
        let notes = Arc::new(Notes::default());
        let gate = Arc::new(Gate::default());
        let (noted, edge, held) = (Arc::clone(&notes), clock.edge.clone(), Arc::clone(&gate));
        let mut runner = Runner::start_with(
            None,
            var,
            Git::default(),
            clock.edge.clone(),
            move |_, _, _, _| {
                Box::new(Keeps {
                    container,
                    edge,
                    notes: noted,
                    gate: held,
                })
            },
        )
        .expect("the runner starts");
        let ticks = Arc::new(Mutex::new(Vec::new()));
        let refused = Arc::new(Mutex::new(Vec::new()));
        let flume = Arc::new(flume);
        runner.toolset(Box::new(Held {
            flume: Arc::clone(&flume),
            ticks: Arc::clone(&ticks),
            refused: Arc::clone(&refused),
        }));
        let mut wakes = BTreeMap::new();
        if !parts.is_empty() {
            runner
                .carry_for(|wake_for| {
                    parts
                        .into_iter()
                        .map(|(label, plane)| {
                            wakes.insert(label.to_string(), wake_for(label));
                            Part {
                                label: label.to_string(),
                                plane,
                            }
                        })
                        .collect()
                })
                .expect("the configuration names a control plane");
        }
        let stopper = runner.stopper();
        let notices = runner.notices();
        let (said, lines) = mpsc::channel();
        let at = clock.edge.clone();
        let thread = thread::spawn(move || {
            runner.keep(&|line| {
                let _ = said.send((at.now(), line));
            })
        });
        Running {
            notices,
            flume,
            refused,
            wakes,
            stopper,
            lines,
            thread,
            notes,
            gate,
            ticks,
            data,
            clock,
            home,
        }
    }

    /// Moves the edge's clock on by `by`, ending each wait it makes due.
    pub fn advance(&self, by: Duration) {
        self.clock.runtime.block_on(tokio::time::advance(by));
    }

    /// The part `label`'s wake, as its implementation would ring it.
    pub fn wake(&self, label: &str) {
        (self.wakes[label])();
    }

    /// The time `line` is said, and the lines said before it since the
    /// last read.
    pub fn until(&self, line: &str) -> (Instant, Vec<String>) {
        let (at, _, said) = self.until_starting(line);
        (at, said)
    }

    /// The time and the words of the next line starting `start`, and the
    /// lines said before it since the last read.
    pub fn until_starting(&self, start: &str) -> (Instant, String, Vec<String>) {
        let mut said = Vec::new();
        while let Ok((at, next)) = self.lines.recv_timeout(PATIENCE) {
            if next.starts_with(start) {
                return (at, next, said);
            }
            said.push(next);
        }
        panic!("no line starting {start:?} said; said: {said:#?}");
    }

    /// Writes `text` as the runner's configuration file, whole, as an
    /// editor's rename leaves it.
    pub fn rewrite(&self, text: &str) {
        let file = self.home.path().join(".config/boomdrive/config.toml");
        let next = file.with_extension("next");
        std::fs::write(&next, text).expect("the configuration written");
        std::fs::rename(&next, &file).expect("the configuration in place");
    }

    /// The runner's configuration file.
    pub fn config_file(&self) -> PathBuf {
        self.home.path().join(".config/boomdrive/config.toml")
    }

    /// The runner's home, `HOME`.
    pub fn home(&self) -> &Path {
        self.home.path()
    }

    /// The runner's local layer.
    pub fn local_file(&self) -> PathBuf {
        self.home.path().join(".config/boomdrive/config.local.toml")
    }

    /// Writes `text` as the runner's local layer, whole, as an editor's
    /// rename leaves it.
    pub fn rewrite_local(&self, text: &str) {
        let file = self.local_file();
        let next = file.with_extension("next");
        std::fs::write(&next, text).expect("the local layer written");
        std::fs::rename(&next, &file).expect("the local layer in place");
    }

    /// Holds the next keeping pass, under the clone's lock, until
    /// [`Running::let_keeping_go`].
    pub fn hold_keeping(&self) {
        self.gate.state.lock().expect("the gate").0 = true;
    }

    /// Waits until a keeping pass is held.
    pub fn keeping_held(&self) {
        let state = self.gate.state.lock().expect("the gate");
        let (state, _) = self
            .gate
            .changed
            .wait_timeout_while(state, PATIENCE, |state| !state.1)
            .expect("the gate");
        assert!(state.1, "no keeping pass held");
    }

    /// Lets the keeping pass held go.
    pub fn let_keeping_go(&self) {
        self.gate.state.lock().expect("the gate").0 = false;
        self.gate.changed.notify_all();
    }

    /// Each inbox ticked, in order.
    pub fn ticks(&self) -> Vec<String> {
        self.ticks.lock().expect("the ticks").clone()
    }

    /// The time each keeping pass began.
    pub fn kept(&self) -> Vec<Instant> {
        self.notes
            .passes
            .lock()
            .expect("the passes")
            .iter()
            .map(|(at, _)| *at)
            .collect()
    }

    /// The repositories each keeping pass's configuration kept.
    pub fn kept_repositories(&self) -> Vec<Vec<String>> {
        self.notes
            .passes
            .lock()
            .expect("the passes")
            .iter()
            .map(|(_, kept)| kept.clone())
            .collect()
    }

    /// Each repository kept alone at a landing.
    pub fn landings(&self) -> Vec<String> {
        self.notes.landings.lock().expect("the landings").clone()
    }

    /// Makes each keeping pass say `words` and fail, or, given none, pass.
    pub fn failing(&self, words: Option<&str>) {
        *self.notes.failing.lock().expect("the failing") = words.map(str::to_string);
    }

    /// Tells the runner of a landing on `address`.
    pub fn landed(&self, address: &str) {
        self.notices
            .landed(Address::parse(address).expect("an address"));
    }

    /// Has millpond refuse each tick of `inbox`.
    pub fn refuse_ticks_of(&self, inbox: &str) {
        self.refused
            .lock()
            .expect("the refused")
            .push(inbox.to_string());
    }

    /// Each call the runner made of Flume.
    pub fn flume_calls(&self) -> Vec<Call> {
        self.flume.calls()
    }

    /// Stops the runner, and gives each line it said since the last read.
    pub fn stop(self) -> Vec<(Instant, String)> {
        self.stopped().0
    }

    /// Stops the runner, and gives each line it said since the last read
    /// and its home, the data directory in it, to read after the stop.
    pub fn stopped(self) -> (Vec<(Instant, String)>, TempDir) {
        self.stopper.stop();
        self.thread.join().expect("the runner ends");
        (self.lines.try_iter().collect(), self.home)
    }
}

/// Whether `holds` comes to hold within [`PATIENCE`], in real time, of
/// what the runner's threads do once its clock has made it due.
pub fn patiently(holds: impl Fn() -> bool) -> bool {
    let deadline = std::time::Instant::now() + PATIENCE;
    while !holds() {
        if std::time::Instant::now() >= deadline {
            return false;
        }
        thread::sleep(Duration::from_millis(10));
    }
    true
}

/// A report holding the launch `id`, launched at `/places/held` under a
/// lease of number 1 that expires at `expires`.
pub fn holding(id: &str, expires: At) -> Report {
    let mut held = Report::default();
    let lease = Lease {
        number: 1,
        expires: Some(expires),
    };
    let answer = Answer {
        outcome: Outcome::Launched,
        place_dir: Some("/places/held".to_string()),
        words: None,
        at: At::now(),
    };
    held.answer(&Id::new(id), lease, answer);
    held
}
