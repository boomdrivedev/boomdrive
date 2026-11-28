//! Each part of the control plane carried on its own by the runner's loop,
//! on its edge's paused clock: its control passes on a timer of their own,
//! one at once when its implementation wakes it and the timer kept from
//! the woken pass, a quiet pass saying nothing, a part's reach said once
//! each way, a slow or a failing part delaying no other, and a stop with a
//! part's pass under way (the running spec's "On the control timer",
//! "Woken by the control plane", "One part woken", "Three passes unread,
//! then read", "Each part's reach its own", "A slow part", "A failing
//! part" and "Stopped with a part's pass under way"; core.22, core.23,
//! core.49, core.50). The runner starts over a keeper and tools held in
//! memory, so no test starts a process, git or a socket, and its clock
//! moves only when a test moves it
//! (logic-is-tested-as-units-and-few-tests-run-the-binary;
//! the-runtime-flavour-is-chosen-in-main-and-tests-pause-tokios-clock).

mod fakes;

use boomdrive::clone::Found;
use boomdrive::config::{Address, Config};
use boomdrive::flume::{Act, Flume, LaunchArgs, Reply, Unrun};
use boomdrive::flume_host::Source;
use boomdrive::git::Git;
use boomdrive::locks::Locks;
use boomdrive::pass::{self, Between, Keeper};
use boomdrive::run::{Edge, Runner, Stopper, Toolset};
use boomdrive::sluice::{self, Sluice};
use boomdrive::terminal::Relays;
use boomdrive::{flume, hello, millpond, report};
use boomdrive_control::fake::Fake;
use boomdrive_control::{
    ControlPlane, Hello, Id, Outcome, Part, Pulled, Report, Reported, ToolVersion, Unread, Unsent,
    Wake, Want,
};
use fakes::{offer, FakeFlume, KEPT};
use std::collections::BTreeMap;
use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::sync::atomic::AtomicBool;
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::Duration;
use tempfile::TempDir;
use tokio::runtime::Runtime;
use tokio::time::Instant;

/// How long a test waits, in real time, for the runner's threads to do
/// what its clock has made due: a guard against a hang, never a bound a
/// test asserts.
const PATIENCE: Duration = Duration::from_secs(60);

/// How long a test waits, in real time, to see that the runner does
/// nothing that is not due.
const QUIET: Duration = Duration::from_millis(200);

/// A second of the edge's clock.
const SECOND: Duration = Duration::from_secs(1);

/// An hour of the edge's clock.
const HOUR: Duration = Duration::from_secs(3600);

/// A current-thread runtime whose clock is paused, and its edge, which a
/// runner under test and its control planes read.
struct Clock {
    runtime: Runtime,
    edge: Edge,
}

impl Clock {
    fn paused() -> Clock {
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
/// keeping pass began, and finds the one clone of [`KEPT`], recording the
/// drive `home`.
struct Keeps {
    container: PathBuf,
    edge: Edge,
    passes: Arc<Mutex<Vec<Instant>>>,
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
        _: &Config,
        _: &Locks,
        _: &AtomicBool,
        _: Between<'_>,
        _: &mut dyn FnMut(String),
    ) -> pass::Outcome {
        self.passes
            .lock()
            .expect("the passes")
            .push(self.edge.now());
        pass::Outcome::Kept
    }

    fn keep_one(
        &self,
        _: &Config,
        _: &Locks,
        _: &Address,
        _: &mut dyn FnMut(String),
    ) -> pass::Outcome {
        pass::Outcome::Kept
    }
}

/// The tools of a runner under test, held in memory: Flume, a sluice no
/// pass here asks, the tools' versions, and no millpond, since no
/// configuration here names an inbox.
struct Held {
    flume: Arc<FakeFlume>,
}

/// Flume as a part's pass reads it: the fake, whatever flume host says.
struct Reading<'a>(&'a FakeFlume);

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
struct NoSluice;

/// What [`NoSluice`] answers.
fn no_sluice() -> sluice::Unrun {
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
struct Versions;

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
        Err(millpond::Unrun(format!("no millpond ticks {inbox} here")))
    }

    fn relays(&self) -> Relays {
        Relays::new(flume::Cli::new("flume"), |_| {})
    }
}

/// What a test hears of a part's control plane: each pull and each
/// report it took, at the edge's time.
struct Heard {
    pulls: Receiver<Instant>,
    reports: Receiver<Instant>,
}

impl Heard {
    /// The time of the next pull.
    fn pull(&self) -> Instant {
        self.pulls.recv_timeout(PATIENCE).expect("a pull made")
    }

    /// The time of the next report taken.
    fn report(&self) -> Instant {
        self.reports.recv_timeout(PATIENCE).expect("a report taken")
    }

    /// Whether no pull comes while the clock stands.
    fn quiet(&self) -> bool {
        self.pulls.recv_timeout(QUIET).is_err()
    }
}

/// `fake`, telling the test of each pull and each report it takes, its
/// first `unread` pulls refused with `no network`.
struct Told {
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
fn told(clock: &Clock, fake: Fake, unread: usize) -> (Box<dyn ControlPlane + Send>, Heard) {
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
struct Slow {
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
fn slow(clock: &Clock, fake: Fake) -> (Box<dyn ControlPlane + Send>, Heard, Sender<()>) {
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
fn part_holding(label: &str, wants: Vec<Want>) -> Fake {
    let mut fake = Fake::owning(&format!("{label}/"));
    fake.set_wants(wants);
    fake
}

/// An offer of the launch `id`, named after its last segment.
fn launch(id: &str) -> Want {
    Want::Offer(offer(id, id.rsplit('/').next().unwrap_or(id)))
}

/// A configuration keeping [`KEPT`] each hour and carrying crew's work for
/// the host `host-a` in the partition `home`, each `poll_interval`; a test
/// gives the runner its parts in place of crew's.
fn carrying(poll_interval: &str) -> String {
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
struct Running {
    wakes: BTreeMap<String, Wake>,
    stopper: Stopper,
    lines: Receiver<(Instant, String)>,
    thread: JoinHandle<()>,
    keeps: Arc<Mutex<Vec<Instant>>>,
    data: PathBuf,
    clock: Clock,
    home: TempDir,
}

impl Running {
    /// A runner over [`carrying`]'s configuration at `poll_interval`,
    /// carrying `parts` over `flume`, its loop on a thread of its own.
    fn carrying(
        clock: Clock,
        poll_interval: &str,
        parts: Vec<(&str, Box<dyn ControlPlane + Send>)>,
        flume: FakeFlume,
    ) -> Running {
        let home = tempfile::tempdir().expect("a home");
        let file = home.path().join(".config/boomdrive/config.toml");
        std::fs::create_dir_all(file.parent().expect("its folder")).expect("its folder");
        std::fs::write(&file, carrying(poll_interval)).expect("the configuration");
        let data = home.path().join(".local/share/boomdrive");
        let container = data.join("repositories").join(KEPT);
        std::fs::create_dir_all(&container).expect("the clone's container");
        let root = home.path().to_path_buf();
        let var = move |name: &str| (name == "HOME").then(|| OsString::from(&root));
        let keeps = Arc::new(Mutex::new(Vec::new()));
        let (noted, edge) = (Arc::clone(&keeps), clock.edge.clone());
        let mut runner = Runner::start_with(
            None,
            var,
            Git::default(),
            clock.edge.clone(),
            move |_, _, _, _| {
                Box::new(Keeps {
                    container,
                    edge,
                    passes: noted,
                })
            },
        )
        .expect("the runner starts");
        runner.toolset(Box::new(Held {
            flume: Arc::new(flume),
        }));
        let mut wakes = BTreeMap::new();
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
        let stopper = runner.stopper();
        let (said, lines) = mpsc::channel();
        let at = clock.edge.clone();
        let thread = thread::spawn(move || {
            runner.keep(&|line| {
                let _ = said.send((at.now(), line));
            })
        });
        Running {
            wakes,
            stopper,
            lines,
            thread,
            keeps,
            data,
            clock,
            home,
        }
    }

    /// Moves the edge's clock on by `by`, ending each wait it makes due.
    fn advance(&self, by: Duration) {
        self.clock.runtime.block_on(tokio::time::advance(by));
    }

    /// The part `label`'s wake, as its implementation would ring it.
    fn wake(&self, label: &str) {
        (self.wakes[label])();
    }

    /// The time `line` is said, and the lines said before it since the
    /// last read.
    fn until(&self, line: &str) -> (Instant, Vec<String>) {
        let (at, _, said) = self.until_starting(line);
        (at, said)
    }

    /// The time and the words of the next line starting `start`, and the
    /// lines said before it since the last read.
    fn until_starting(&self, start: &str) -> (Instant, String, Vec<String>) {
        let mut said = Vec::new();
        while let Ok((at, next)) = self.lines.recv_timeout(PATIENCE) {
            if next.starts_with(start) {
                return (at, next, said);
            }
            said.push(next);
        }
        panic!("no line starting {start:?} said; said: {said:#?}");
    }

    /// The time each keeping pass began.
    fn kept(&self) -> Vec<Instant> {
        self.keeps.lock().expect("the passes").clone()
    }

    /// Stops the runner, and gives each line it said since the last read.
    fn stop(self) -> Vec<(Instant, String)> {
        self.stopped().0
    }

    /// Stops the runner, and gives each line it said since the last read
    /// and its home, the data directory in it, to read after the stop.
    fn stopped(self) -> (Vec<(Instant, String)>, TempDir) {
        self.stopper.stop();
        self.thread.join().expect("the runner ends");
        (self.lines.try_iter().collect(), self.home)
    }
}

/// The running spec's "On the control timer", over a part: `home`'s first
/// control pass follows the first keeping pass, and the next come each
/// `poll_interval`, a second, while the keeping pass keeps its own
/// `fetch_interval`, an hour.
#[test]
fn control_passes_on_their_own_timer() {
    let clock = Clock::paused();
    let (home, heard) = told(&clock, Fake::owning("home/"), 0);
    let running = Running::carrying(clock, "1s", vec![("home", home)], FakeFlume::new());

    let first = heard.pull();
    running.advance(SECOND);
    let second = heard.pull();
    running.advance(SECOND);
    let third = heard.pull();

    let kept = running.kept();
    assert_eq!(kept.len(), 1, "no keeping pass but the first");
    assert!(first >= kept[0], "the first control pass follows it");
    assert_eq!(second - first, SECOND);
    assert_eq!(third - first, 2 * SECOND);
    running.stop();
}

/// The running spec's "Woken by the control plane": woken through the
/// wake its implementation was handed for `home` at its opening, a
/// control pass of `home` runs at once, and none follows it within the
/// hour's interval.
#[test]
fn woken_by_the_control_plane() {
    let clock = Clock::paused();
    let (home, heard) = told(&clock, Fake::owning("home/"), 0);
    let running = Running::carrying(clock, "1h", vec![("home", home)], FakeFlume::new());
    let first = heard.pull();
    heard.report();

    running.wake("home");
    let woken = heard.pull();
    heard.report();
    running.advance(HOUR - SECOND);
    let quiet = heard.quiet();
    running.advance(SECOND);
    let next = heard.pull();

    assert_eq!(woken, first, "the woken pass runs at once");
    assert!(quiet, "no control pass within the hour");
    assert_eq!(next - woken, HOUR);
    running.stop();
}

/// The running spec's "One part woken": the wake handed for `home` makes
/// `home`'s control pass at once, and `work`'s next pass begins when its
/// own timer says, an hour on.
#[test]
fn one_part_woken() {
    let clock = Clock::paused();
    let (home, home_heard) = told(&clock, Fake::owning("home/"), 0);
    let (work, work_heard) = told(&clock, Fake::owning("work/"), 0);
    let running = Running::carrying(
        clock,
        "1h",
        vec![("home", home), ("work", work)],
        FakeFlume::new(),
    );
    let first = home_heard.pull();
    let work_first = work_heard.pull();
    home_heard.report();

    running.wake("home");
    let woken = home_heard.pull();
    let quiet = work_heard.quiet();
    running.advance(HOUR);
    let work_next = work_heard.pull();

    assert_eq!(woken, first, "home's woken pass runs at once");
    assert!(quiet, "work is not woken");
    assert_eq!(work_next - work_first, HOUR, "work's own timer");
    running.stop();
}

/// The running spec's "Woken by the control plane", its timer: the pass
/// after the woken one comes `poll_interval`, four seconds, after the
/// woken pass's start, not after the start of the pass before it.
#[test]
fn the_timer_kept_from_the_woken_pass() {
    let clock = Clock::paused();
    let (home, heard) = told(&clock, Fake::owning("home/"), 0);
    let running = Running::carrying(clock, "4s", vec![("home", home)], FakeFlume::new());
    let first = heard.pull();
    heard.report();
    running.advance(SECOND);

    running.wake("home");
    let woken = heard.pull();
    heard.report();
    running.advance(3 * SECOND);
    let quiet = heard.quiet();
    running.advance(SECOND);
    let next = heard.pull();

    assert_eq!(woken - first, SECOND);
    assert!(quiet, "no pass four seconds after the one before the wake");
    assert_eq!(next - woken, 4 * SECOND);
    running.stop();
}

/// A control pass that changes nothing says nothing (design Decision 5):
/// over a control plane wanting nothing, the runner says nothing until it
/// stops.
#[test]
fn a_quiet_control_pass_says_nothing() {
    let clock = Clock::paused();
    let (home, heard) = told(&clock, Fake::owning("home/"), 0);
    let running = Running::carrying(clock, "1s", vec![("home", home)], FakeFlume::new());
    heard.pull();
    running.advance(SECOND);
    heard.pull();

    let said: Vec<String> = running.stop().into_iter().map(|(_, line)| line).collect();

    assert_eq!(said, ["boomdrive: stopped"]);
}

/// The running spec's "Three passes unread, then read": `home` refuses
/// three pulls in a row with `no network`, then gives the fourth. One
/// line names `home` unread, and one, at the fourth pass, says it is read
/// again; the passes after say nothing of it.
#[test]
fn three_passes_unread_then_read() {
    let clock = Clock::paused();
    let (home, heard) = told(&clock, Fake::new(), 3);
    let running = Running::carrying(clock, "1s", vec![("home", home)], FakeFlume::new());
    let first = heard.pull();
    let mut pulls = vec![first];
    for _ in 0..3 {
        running.advance(SECOND);
        pulls.push(heard.pull());
    }

    let (read_again, said) = running.until("boomdrive: home: the control plane is read again");
    running.advance(SECOND);
    heard.pull();
    running.advance(SECOND);
    heard.pull();
    let after: Vec<String> = running.stop().into_iter().map(|(_, line)| line).collect();

    let reach: Vec<&String> = said
        .iter()
        .filter(|line| line.starts_with("boomdrive: home: the control plane"))
        .collect();
    assert_eq!(
        reach,
        ["boomdrive: home: the control plane could not be read: no network"]
    );
    assert_eq!(read_again, pulls[3], "said at the fourth pass");
    assert_eq!(after, ["boomdrive: stopped"]);
}

/// The running spec's "Each part's reach its own": `home` refuses every
/// pull, and `work` gives every pull. One line names `home` as unread,
/// and no line names `work`'s reach.
#[test]
fn each_parts_reach_its_own() {
    let clock = Clock::paused();
    let mut refusing = Fake::owning("home/");
    refusing.refuse_pulls("no network");
    let (home, home_heard) = told(&clock, refusing, 0);
    let (work, work_heard) = told(&clock, Fake::owning("work/"), 0);
    let running = Running::carrying(
        clock,
        "1s",
        vec![("home", home), ("work", work)],
        FakeFlume::new(),
    );
    for _ in 0..3 {
        home_heard.pull();
        work_heard.pull();
        running.advance(SECOND);
    }

    let said: Vec<String> = running.stop().into_iter().map(|(_, line)| line).collect();

    let reach: Vec<&String> = said
        .iter()
        .filter(|line| line.contains(": the control plane "))
        .collect();
    assert_eq!(
        reach,
        ["boomdrive: home: the control plane could not be read: no network"],
        "{said:#?}"
    );
}

/// The running spec's "A failing part": `home` refuses every pull and
/// report with `no network`, and `work` offers a launch whose stream has
/// room. `work`'s launch is made and reported, and one line says `home`
/// cannot be read.
#[test]
fn a_failing_part() {
    let clock = Clock::paused();
    let mut failing = Fake::owning("home/");
    failing.refuse_pulls("no network");
    failing.refuse_reports("no network");
    let (home, home_heard) = told(&clock, failing, 0);
    let (work, work_heard) = told(&clock, part_holding("work", vec![launch("work/1")]), 0);
    let running = Running::carrying(
        clock,
        "1s",
        vec![("home", home), ("work", work)],
        FakeFlume::new(),
    );

    let (_, launched, mut said) = running.until_starting("work/1: launched at ");
    work_heard.report();
    home_heard.pull();
    running.advance(SECOND);
    home_heard.pull();

    said.push(launched);
    said.extend(running.stop().into_iter().map(|(_, line)| line));
    let unread = "boomdrive: home: the control plane could not be read: no network";
    let named: Vec<&String> = said
        .iter()
        .filter(|line| line.starts_with("boomdrive: home") || line.starts_with("home/"))
        .collect();
    assert_eq!(named, [unread], "{said:#?}");
}

/// The running spec's "A slow part": `home`'s pull answers only once the
/// test lets it go, and `work` offers a launch whose stream has room.
/// `work`'s launch is made and reported while `home`'s pull waits, and
/// `work`'s next control pass begins `poll_interval` after the start of
/// its last.
#[test]
fn a_slow_part() {
    let clock = Clock::paused();
    let (home, home_heard, home_go) = slow(&clock, Fake::owning("home/"));
    let (work, work_heard) = told(&clock, part_holding("work", vec![launch("work/1")]), 0);
    let running = Running::carrying(
        clock,
        "2s",
        vec![("home", home), ("work", work)],
        FakeFlume::new(),
    );
    let began = home_heard.pull();

    let (launched, _, _) = running.until_starting("work/1: launched at ");
    let reported = work_heard.report();
    let first = work_heard.pull();
    running.advance(2 * SECOND);
    let second = work_heard.pull();

    assert_eq!(launched, began, "launched while home's pull waits");
    assert_eq!(reported, launched);
    assert_eq!(second - first, 2 * SECOND, "work's own timer");
    assert!(
        home_heard.reports.try_recv().is_err(),
        "home's pass still waits"
    );
    home_go.send(()).expect("home's pull let go");
    drop(home_go);
    running.stop();
}

/// The running spec's "Stopped with a part's pass under way": stopped
/// while `home`'s control pass waits on its pull and `work` waits for its
/// next pass, `work` begins no pass, `home`'s pass ends with its report
/// saved, the runner says it stopped, and no pass begins after that line.
#[test]
fn stopped_with_a_parts_pass_under_way() {
    let clock = Clock::paused();
    let (home, home_heard, home_go) = slow(&clock, part_holding("home", vec![launch("home/1")]));
    let (work, work_heard) = told(&clock, Fake::owning("work/"), 0);
    let running = Running::carrying(
        clock,
        "1h",
        vec![("home", home), ("work", work)],
        FakeFlume::new(),
    );
    home_heard.pull();
    work_heard.pull();
    work_heard.report();

    running.stopper.stop();
    home_go.send(()).expect("home's pull let go");
    let reported = home_heard.report();
    let data = running.data.clone();
    let (said, _home) = running.stopped();

    let (stopped, last) = said.last().cloned().expect("a line");
    assert_eq!(last, "boomdrive: stopped", "{said:#?}");
    assert!(reported <= stopped, "home's pass ended before the stop");
    assert!(
        work_heard.pulls.try_recv().is_err(),
        "work began no pass once stopped"
    );
    assert!(
        home_heard.pulls.try_recv().is_err(),
        "home began no pass once stopped"
    );
    let file = report::file(&report::control_dir(&data));
    let saved = report::read(&file).unwrap().expect("the report file");
    assert_eq!(
        saved.launches[&Id::new("home/1")].answer.outcome,
        Outcome::Launched
    );
}
