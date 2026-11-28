//! `boomdrive run` carrying a control plane's work (design Decisions 4, 5
//! and 13, and Decision 2 of the change
//! each-partition-the-runner-serves-runs-on-its-own): `--once` makes one
//! keeping pass and one control pass of each part side by side, and each
//! part is carried on a thread of its own, its control passes on a timer
//! of their own, hastened by no lapsed lease and after no pass that did
//! not report, at once when its implementation wakes it, and none that
//! acts while the configuration does not read, saying its control plane
//! unread once until it is read again, waiting on no other part and on no
//! keeping pass (the running spec's "Each part of the control plane is
//! carried on its own", "boomdrive run --once makes one pass", "boomdrive
//! run makes control passes on a timer of their own", "A control pass
//! runs at once when its control plane wakes the runner", "A control pass
//! after a broken configuration acts on nothing" and "A control plane
//! that cannot be read is said once until it is read again"; the
//! configuration spec's "The configuration may name the control plane"
//! and "The control-plane table is read when the runner starts"). The
//! binary runs under the scratch host with stub `flume`, `sluice` and
//! `systemd-creds` first on its `PATH`, crew's implementation
//! reaches the scratch forge's state repository through the scratch git
//! configuration, and the hosted implementation a loopback port with
//! nothing on it. The runner runs in-process over the fake control plane
//! and control planes of the test's own, its parts `home` and `work`.

mod common;
mod fakes;
#[path = "../../boomdrive-crew/tests/common/mod.rs"]
mod records;

use boomdrive::report;
use boomdrive::run::{Programs, Runner, Stopper};
use boomdrive_control::fake::Fake;
use boomdrive_control::{
    Answer, At, ControlPlane, Hello, Id, Lease, Outcome, Part, Pulled, Report, Reported, Unread,
    Unsent, Want,
};
use common::{keeping, Origin, Scratch};
use fakes::{offer, CredsStub, HeldFetch, SluiceStub, Stub, StubHost};
use std::ffi::{OsStr, OsString};
use std::process::Output;
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

/// How long a test waits for what it expects.
const PATIENCE: Duration = Duration::from_secs(20);

/// The repository every configuration here keeps.
const KEPT: &str = "agentplot/flume";

/// A configuration keeping [`KEPT`], each `fetch_interval`, carrying
/// crew's work for the host `host-a` in the partition `home`, each
/// `poll_interval`.
fn carrying(fetch_interval: &str, poll_interval: &str) -> String {
    carrying_drives(fetch_interval, poll_interval, &keeping(&[KEPT]))
}

/// A configuration as [`carrying`] writes it, but keeping the drives
/// `drives` writes, as `common::drives` writes them.
fn carrying_drives(fetch_interval: &str, poll_interval: &str, drives: &str) -> String {
    format!(
        "fetch_interval = {fetch_interval:?}\n{}\n\
         accounts.own.control_plane = \"crew\"\n\
         [control_plane]\n\
         poll_interval = {poll_interval:?}\n\n\
         [control_plane.crew]\n\
         host = \"host-a\"\n\n\
         [[control_plane.crew.partitions]]\n\
         label = \"home\"\n\
         state_repository = \"example/home-state\"\n",
        drives
    )
}

/// Stub `flume`, `sluice` and `systemd-creds`, each in a directory of its
/// own.
struct Tools {
    flume: Stub,
    sluice: SluiceStub,
    creds: CredsStub,
    millpond: Stub,
}

impl Tools {
    fn new() -> Tools {
        Tools {
            flume: Stub::new(),
            sluice: SluiceStub::new(),
            creds: CredsStub::new(),
            millpond: Stub::new(),
        }
    }

    fn programs(&self) -> Programs {
        Programs {
            flume: self.flume.program(),
            sluice: self.sluice.program(),
            millpond: self.millpond.program(),
        }
    }

    /// The test's `PATH` with the stubs first, so no real Flume, sluice
    /// or `systemd-creds` runs.
    fn path(&self) -> OsString {
        let stubs = [&self.flume.dir, &self.sluice.dir, &self.creds.dir]
            .map(|dir| dir.path().to_path_buf());
        let rest = std::env::var_os("PATH").unwrap_or_default();
        std::env::join_paths(stubs.iter().cloned().chain(std::env::split_paths(&rest)))
            .expect("a PATH")
    }

    /// Each call the stub `flume` was asked, by its command after
    /// `--json`, or `--version`.
    fn flume_commands(&self) -> Vec<String> {
        self.flume
            .calls()
            .into_iter()
            .map(|call| match call.first().map(String::as_str) {
                Some("--json") => call.get(1).cloned().unwrap_or_default(),
                _ => call.join(" "),
            })
            .collect()
    }
}

/// `boomdrive run --once` on the scratch host's default configuration
/// file, with `tools` first on its `PATH`.
fn once(scratch: &Scratch, tools: &Tools) -> Output {
    let host = StubHost::under(&scratch.home, tools.flume.dir.path());
    scratch
        .boomdrive()
        .args(["run", "--once"])
        .env("PATH", tools.path())
        .env(boomdrive::flume_host::USER_SCRATCH, host.dir.path())
        .output()
        .expect("boomdrive runs")
}

fn stderr(output: &Output) -> String {
    String::from_utf8_lossy(&output.stderr).into_owned()
}

/// `words` as a call's arguments.
fn words(words: &[&str]) -> Vec<String> {
    words.iter().map(|word| word.to_string()).collect()
}

/// How many settles the stub `flume` was asked, which the runner never
/// asks (core.13).
fn settles(tools: &Tools) -> usize {
    tools
        .flume_commands()
        .iter()
        .filter(|command| *command == "settle")
        .count()
}

/// The state repository `example/home-state` on the scratch
/// forge, its `home/main` holding a launch request for `host-a` of
/// [`KEPT`]'s unit `a`, and the plan holding `a`, as crew writes them.
fn state_repository(scratch: &Scratch) -> Origin<'_> {
    let state = scratch.empty_origin("example/home-state");
    let request = records::launch(
        1,
        Some("host-a"),
        "unit/a",
        KEPT,
        "a",
        &[("Base", "main"), ("Brief", "Parse the input.")],
    );
    state.commit(
        "home/main",
        "requests.rec",
        &records::file(records::REQUESTS, &[request]),
    );
    state.commit("home/main", "plan.rec", &records::plan(&["a"], &[]));
    state
}

/// The running spec's "One control pass": crew's implementation over a
/// scratch state repository holding a launch for this host, and a stub
/// `flume` that launches it. The runner keeps its repositories, launches
/// once, writes its branch, asks Flume for no settle, and exits 0.
#[test]
fn one_control_pass() {
    let scratch = Scratch::new();
    scratch.origin(KEPT);
    let state = state_repository(&scratch);
    scratch.configure(&carrying("2m", "1m"));
    let tools = Tools::new();
    tools.flume.answers("launch", "launch.json", 0);

    let output = once(&scratch, &tools);

    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    assert!(scratch.home.join("Code").join(KEPT).exists());
    let launches = tools
        .flume_commands()
        .into_iter()
        .filter(|command| command == "launch")
        .count();
    assert_eq!(launches, 1, "{:?}", tools.flume_commands());
    assert_eq!(settles(&tools), 0, "{:?}", tools.flume_commands());
    assert!(
        stderr(&output)
            .lines()
            .any(|line| line.starts_with("home/1: launched at ")),
        "{}",
        stderr(&output)
    );
    let answers = scratch.git(&state.bare, &["show", "home/runner/host-a:answers.rec"]);
    assert!(answers.contains("Outcome: launched"), "{answers}");
    let runner = scratch.git(&state.bare, &["show", "home/runner/host-a:runner.rec"]);
    assert!(runner.contains("Host: host-a"), "{runner}");
}

/// With crew's implementation, `boomdrive run --once` writes
/// `home/runner/host-a`'s `repositories.rec`, naming the drive `home`'s
/// repository at its container in the runner's data directory, and
/// nothing of the drive `work`'s (the crew-control-plane spec's "A drive's
/// repositories and their folders"; design Decisions 1 to 3 of
/// the-runner-names-each-repository-it-keeps).
#[test]
fn the_runners_branch_names_each_repository_of_its_drive() {
    let scratch = Scratch::new();
    scratch.origin(KEPT);
    scratch.origin("example/app");
    let state = state_repository(&scratch);
    scratch.configure(&carrying_drives(
        "2m",
        "1m",
        &common::drives(&[("home", &[KEPT]), ("work", &["example/app"])]),
    ));
    let tools = Tools::new();
    tools.flume.answers("launch", "launch.json", 0);

    let output = once(&scratch, &tools);

    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    let repositories = scratch.git(
        &state.bare,
        &["show", "home/runner/host-a:repositories.rec"],
    );
    let kept = format!(
        "\nRepository: {KEPT}\nFolder: {}",
        scratch.container(KEPT).display()
    );
    assert!(repositories.contains(&kept), "{repositories}");
    assert!(!repositories.contains("example/app"), "{repositories}");
}

/// A configuration keeping [`KEPT`], carrying crew's work for the host
/// `host-a` in the partitions `home` and `work`, both of the state
/// repository `example/state`.
fn carrying_two() -> String {
    format!(
        "{}\naccounts.own.control_plane = \"crew\"\n[control_plane]\n\n\
         [control_plane.crew]\nhost = \"host-a\"\n\n\
         [[control_plane.crew.partitions]]\nlabel = \"home\"\n\
         state_repository = \"example/state\"\n\n\
         [[control_plane.crew.partitions]]\nlabel = \"work\"\n\
         state_repository = \"example/state\"\n",
        keeping(&[KEPT])
    )
}

/// The running spec's "A control plane that cannot be read": no
/// partition's state repository can be read. The runner keeps its
/// repositories, names each part as unread, launches and acts on nothing,
/// and exits 1. Each part's pass reads Flume's status, of the stub flume
/// host, and each tool's version for the hello it hands the pull, before
/// the pull is read (design Decision 5, step 2): no Flume command but
/// `--version` runs.
#[test]
fn a_control_plane_that_cannot_be_read() {
    let scratch = Scratch::new();
    scratch.origin(KEPT);
    scratch.configure(&carrying_two());
    let tools = Tools::new();

    let output = once(&scratch, &tools);

    assert_eq!(output.status.code(), Some(1), "{}", stderr(&output));
    assert!(scratch.home.join("Code").join(KEPT).exists());
    let said = stderr(&output);
    for label in ["home", "work"] {
        let unread: Vec<&str> = said
            .lines()
            .filter(|line| {
                line.starts_with(&format!(
                    "boomdrive: {label}: the control plane could not be read: "
                ))
            })
            .collect();
        assert_eq!(unread.len(), 1, "{said}");
        assert!(unread[0].contains(&format!("{label}/main")), "{said}");
    }
    for command in tools.flume_commands() {
        assert!(command == "--version", "{:?}", tools.flume_commands());
    }
}

/// The running spec's "Stopped with a part's read in flight": `boomdrive
/// run` carries crew's partition `home`, whose state repository takes the
/// connection and never answers, and is sent SIGTERM once `home`'s read of
/// `home/main` has taken it. The read is ended at the stop, well before
/// its limit, `home` is named unread with the halted words, no settle
/// runs, and nothing the read started outlives the runner (record 0042 as
/// amended on 2026-11-12; core.58).
#[test]
fn stopped_with_a_read_in_flight() {
    /// The runner's binary, killed where the test ends before it exits.
    struct Child(std::process::Child);
    impl Drop for Child {
        fn drop(&mut self) {
            let _ = self.0.kill();
            let _ = self.0.wait();
        }
    }

    let scratch = Scratch::new();
    scratch.origin(KEPT);
    scratch.configure(&carrying("2m", "1m"));
    let state = scratch.silent("example/home-state");
    let tools = Tools::new();
    let mut running = Child(
        scratch
            .boomdrive()
            .arg("run")
            .env("PATH", tools.path())
            .stderr(std::process::Stdio::piped())
            .spawn()
            .expect("boomdrive runs"),
    );
    let said = running.0.stderr.take().expect("its standard error");
    let reading = thread::spawn(move || {
        let mut said = said;
        let mut text = String::new();
        std::io::Read::read_to_string(&mut said, &mut text).expect("its standard error");
        text
    });
    assert!(
        state.taken(Duration::from_secs(10)),
        "home's read never reached its state repository"
    );

    let pid = libc::pid_t::try_from(running.0.id()).expect("a process id");
    // SAFETY: kill(2) takes plain integers and touches no memory of ours.
    assert_eq!(unsafe { libc::kill(pid, libc::SIGTERM) }, 0);
    let deadline = Instant::now() + Duration::from_secs(5);
    let status = loop {
        if let Some(status) = running.0.try_wait().expect("its status") {
            break status;
        }
        assert!(Instant::now() < deadline, "not stopped within five seconds");
        thread::sleep(Duration::from_millis(10));
    };
    assert_eq!(status.code(), Some(0));
    let said = reading.join().expect("its standard error read");
    let unread: Vec<&str> = said
        .lines()
        .filter(|line| line.starts_with("boomdrive: home: the control plane could not be read: "))
        .collect();
    assert_eq!(unread.len(), 1, "{said}");
    assert!(
        unread[0].ends_with(
            "git fetch with https://github.com/example/home-state.git was stopped as its \
             caller asked"
        ),
        "{said}"
    );
    assert_eq!(said.lines().last(), Some("boomdrive: stopped"), "{said}");
    assert!(
        state.closed(Duration::from_secs(1)),
        "the read's connection outlived the runner"
    );
}

/// The control-plane-interface spec's "The hosted control plane named",
/// and design Decision 14's `run --once` under a hosted control plane
/// that cannot be reached: the runner opens the hosted implementation
/// with its table, names no word as one this binary does not carry, keeps
/// its repositories and says the control plane unread once. Its control
/// pass did not pull, so `--once` exits 1. The partition's key is read
/// from no store: the scratch host's `HOME` holds no keychain, and the
/// stub `systemd-creds` keeps no credential, so nothing is dialed.
#[test]
fn the_hosted_control_plane_named() {
    let scratch = Scratch::new();
    scratch.origin(KEPT);
    scratch.configure(&format!(
        "{}accounts.own.control_plane = \"hosted\"\n[control_plane]\n\n\
         [control_plane.hosted]\nurl = {:?}\n\n\
         [[control_plane.hosted.partitions]]\nlabel = \"home\"\nrepositories = [{KEPT:?}]\n",
        keeping(&[KEPT]),
        loopback_with_nothing_on_it()
    ));
    let tools = Tools::new();

    let output = once(&scratch, &tools);

    assert_eq!(output.status.code(), Some(1), "{}", stderr(&output));
    let said = stderr(&output);
    assert!(!said.contains("does not carry"), "{said}");
    assert!(!said.contains("no control pass runs"), "{said}");
    let unread: Vec<&str> = said
        .lines()
        .filter(|line| line.starts_with("boomdrive: home: the control plane could not be read: "))
        .collect();
    assert_eq!(unread.len(), 1, "{said}");
    assert!(scratch.home.join("Code").join(KEPT).exists());
    assert!(!tools.creds.kept_or_read(), "{:?}", tools.creds.calls());
}

/// The running spec's "crew and a cloud side by side": the account `own`
/// names crew, whose partition `home` reads and writes its state
/// repository, and the account `cloud` names the hosted control plane at a
/// loopback port with nothing on it. One runner carries both parts: crew's
/// `home` reports, its runner branch written, and the hosted `cloud` is
/// said unread once, neither waiting on the other (record 0054; core.49).
#[test]
fn crew_and_a_cloud_side_by_side() {
    let scratch = Scratch::new();
    scratch.origin(KEPT);
    let state = state_repository(&scratch);
    scratch.configure(&format!(
        "{}accounts.own.control_plane = \"crew\"\n\
         accounts.cloud.control_plane = \"hosted\"\n\
         accounts.cloud.drives.garden.repositories = []\n\n\
         [control_plane.crew]\nhost = \"host-a\"\n\n\
         [[control_plane.crew.partitions]]\nlabel = \"home\"\n\
         state_repository = \"example/home-state\"\n\n\
         [control_plane.hosted]\nurl = {:?}\n\n\
         [[control_plane.hosted.partitions]]\nlabel = \"cloud\"\nrepositories = []\n",
        keeping(&[KEPT]),
        loopback_with_nothing_on_it()
    ));
    let tools = Tools::new();

    let output = once(&scratch, &tools);

    let said = stderr(&output);
    let runner = scratch.git(&state.bare, &["show", "home/runner/host-a:runner.rec"]);
    assert!(runner.contains("Host: host-a"), "{runner}");
    let unread: Vec<&str> = said
        .lines()
        .filter(|line| line.starts_with("boomdrive: cloud: the control plane could not be read: "))
        .collect();
    assert_eq!(unread.len(), 1, "{said}");
    assert!(
        !said.contains("boomdrive: home: the control plane could not be read"),
        "{said}"
    );
}

/// The running spec's "A control plane that could not be opened": the
/// account `own` names the hosted control plane, whose table names no
/// `url`; the runner says its implementation's refusal, runs no control
/// pass and keeps its repositories. Its control pass did not pull, so
/// `--once` exits 1. It runs no `flume` command.
#[test]
fn a_control_plane_that_could_not_be_opened() {
    let scratch = Scratch::new();
    scratch.origin(KEPT);
    scratch.configure(&format!(
        "{}accounts.own.control_plane = \"hosted\"\n[control_plane.hosted]\npartitions = []\n",
        keeping(&[KEPT])
    ));
    let tools = Tools::new();

    let output = once(&scratch, &tools);

    assert_eq!(output.status.code(), Some(1), "{}", stderr(&output));
    let said = stderr(&output);
    assert!(
        said.lines().any(
            |line| line.contains("[control_plane.hosted]: names no `url`")
                && line.ends_with(": no control pass runs")
        ),
        "{said}"
    );
    assert!(scratch.home.join("Code").join(KEPT).exists());
    assert_eq!(tools.flume.calls(), Vec::<Vec<String>>::new());
    assert_eq!(tools.sluice.calls(), Vec::<Vec<String>>::new());
}

/// A loopback address with nothing listening on it.
fn loopback_with_nothing_on_it() -> String {
    let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("a loopback port");
    let port = listener.local_addr().expect("its address").port();
    drop(listener);
    format!("http://127.0.0.1:{port}/boomdrive")
}

/// The fake control plane, telling the test when each pull is made and
/// each report taken.
struct Told {
    fake: Fake,
    /// How many pulls are still refused, with `no network`, before the
    /// fake is asked.
    unread: usize,
    pulls: Sender<Instant>,
    reports: Sender<Instant>,
}

impl ControlPlane for Told {
    fn pull(&mut self, hello: &Hello, report: &Report) -> Result<Pulled, Unread> {
        let _ = self.pulls.send(Instant::now());
        if self.unread > 0 {
            self.unread -= 1;
            return Err(Unread("no network".to_string()));
        }
        self.fake.pull(hello, report)
    }

    fn report(&mut self, report: &Report) -> Result<Reported, Unsent> {
        let reported = self.fake.report(report)?;
        let _ = self.reports.send(Instant::now());
        Ok(reported)
    }

    fn owns(&self, id: &Id) -> bool {
        self.fake.owns(id)
    }

    fn read_through_the_runner(&self) -> bool {
        self.fake.read_through_the_runner()
    }
}

/// What a test hears of one part's control plane.
struct Heard {
    pulls: Receiver<Instant>,
    reports: Receiver<Instant>,
}

impl Heard {
    /// When the next pull is made; panics where none is made in time.
    fn pull(&self) -> Instant {
        self.pulls
            .recv_timeout(PATIENCE)
            .expect("a pull made in time")
    }

    /// Whether a pull is made within `wait`.
    fn pulled_within(&self, wait: Duration) -> bool {
        self.pulls.recv_timeout(wait).is_ok()
    }

    /// When the next report is taken; panics where none is in time.
    fn report(&self) -> Instant {
        self.reports
            .recv_timeout(PATIENCE)
            .expect("a report taken in time")
    }
}

/// `fake` as a part's control plane, its first `unread` pulls refused
/// with `no network`, and what the test hears of it.
fn told(fake: Fake, unread: usize) -> (Box<dyn ControlPlane + Send>, Heard) {
    let (pulls, pulled) = mpsc::channel();
    let (reports, reported) = mpsc::channel();
    let plane = Told {
        fake,
        unread,
        pulls,
        reports,
    };
    let heard = Heard {
        pulls: pulled,
        reports: reported,
    };
    (Box::new(plane), heard)
}

/// The fake control plane of the part `label`, owning the ids under
/// `<label>/` alone, holding `wants`.
fn part_holding(label: &str, wants: Vec<Want>) -> Fake {
    let mut fake = Fake::owning(&format!("{label}/"));
    fake.set_wants(wants);
    fake
}

/// An offer of a launch into [`KEPT`] with the id `id`, naming no stream.
fn launch(id: &str) -> Want {
    Want::Offer(offer(id, id.rsplit('/').next().unwrap_or(id)))
}

/// The runner, running in a thread of the test's, carrying parts of the
/// test's own with the stub tools.
struct Running {
    /// What the test hears of `home`'s control plane, where the runner
    /// carries one the harness made.
    home: Option<Heard>,
    stopper: Stopper,
    lines: Receiver<(Instant, String)>,
    thread: JoinHandle<()>,
    tools: Tools,
    /// The stub flume host the runner subscribes to, answering from the
    /// stub `flume`'s status answer.
    host: StubHost,
    /// The edge's runtime, dropped after the runner's thread.
    _runtime: tokio::runtime::Runtime,
}

impl Running {
    /// Starts the runner on the scratch host's default configuration
    /// file, which names crew's implementation, and carries one part,
    /// `home`, the fake control plane, in its place.
    fn start(scratch: &Scratch) -> Running {
        Running::over(scratch, Fake::new(), 0)
    }

    /// Starts the runner as [`Running::start`] does, carrying `fake`'s
    /// work as `home`, its first `unread` pulls refused with `no network`.
    fn over(scratch: &Scratch, fake: Fake, unread: usize) -> Running {
        let (plane, heard) = told(fake, unread);
        let mut running = Running::carrying(scratch, vec![("home", plane)], Tools::new());
        running.home = Some(heard);
        running
    }

    /// Starts the runner on the scratch host's default configuration
    /// file, which names crew's implementation, and carries `parts`, each
    /// by its label, in its place, running `tools`.
    fn carrying(
        scratch: &Scratch,
        parts: Vec<(&str, Box<dyn ControlPlane + Send>)>,
        tools: Tools,
    ) -> Running {
        let overlay = scratch.overlay();
        let var = |name: &str| {
            overlay
                .iter()
                .find(|(key, _)| key.as_os_str() == OsStr::new(name))
                .map(|(_, value)| value.clone())
        };
        let runtime = tokio::runtime::Runtime::new().expect("the edge's runtime");
        let edge = boomdrive::run::Edge::of(runtime.handle());
        let mut runner =
            Runner::start(None, var, scratch.runner_git(), edge).expect("the runner starts");
        let host = StubHost::new(tools.flume.dir.path());
        runner.flume_socket(host.socket.clone());
        runner.tools(tools.programs());
        runner
            .carry_for(|_| {
                parts
                    .into_iter()
                    .map(|(label, plane)| Part {
                        label: label.to_string(),
                        plane,
                    })
                    .collect()
            })
            .expect("the configuration names a control plane");
        let stopper = runner.stopper();
        let (said, lines) = mpsc::channel();
        let thread = thread::spawn(move || {
            runner.keep(&|line| {
                let _ = said.send((Instant::now(), line));
            })
        });
        Running {
            home: None,
            stopper,
            lines,
            thread,
            tools,
            host,
            _runtime: runtime,
        }
    }

    /// What the test hears of `home`'s control plane.
    fn home(&self) -> &Heard {
        self.home
            .as_ref()
            .expect("home's control plane is the harness's")
    }

    /// When `home`'s next pull is made; panics where none is in time.
    fn pull(&self) -> Instant {
        self.home().pull()
    }

    /// Whether `home` pulls within `wait`.
    fn pulled_within(&self, wait: Duration) -> bool {
        self.home().pulled_within(wait)
    }

    /// When `line` is said, the lines said before it since the last read;
    /// panics where it is not said in time.
    fn until(&self, line: &str) -> (Instant, Vec<String>) {
        self.until_one(|said| said == line, line)
    }

    /// When a line starting `start` is said, that line and the lines said
    /// before it since the last read; panics where none is said in time.
    fn until_starting(&self, start: &str) -> (Instant, String, Vec<String>) {
        let deadline = Instant::now() + PATIENCE;
        let mut said = Vec::new();
        while let Ok((at, next)) = self
            .lines
            .recv_timeout(deadline.saturating_duration_since(Instant::now()))
        {
            if next.starts_with(start) {
                return (at, next, said);
            }
            said.push(next);
        }
        panic!("no line starting {start:?} said in time; said: {said:#?}");
    }

    fn until_one(&self, is: impl Fn(&str) -> bool, what: &str) -> (Instant, Vec<String>) {
        let deadline = Instant::now() + PATIENCE;
        let mut said = Vec::new();
        while let Ok((at, next)) = self
            .lines
            .recv_timeout(deadline.saturating_duration_since(Instant::now()))
        {
            if is(&next) {
                return (at, said);
            }
            said.push(next);
        }
        panic!("{what:?} not said in time; said: {said:#?}");
    }

    /// Waits until the stub `millpond` has been asked `ticks` ticks;
    /// panics where it is not in time.
    fn ticked(&self, ticks: usize) {
        let deadline = Instant::now() + PATIENCE;
        while self.tools.millpond.calls().len() < ticks {
            assert!(
                Instant::now() < deadline,
                "{ticks} ticks not run in time: {:?}",
                self.tools.millpond.calls()
            );
            thread::sleep(Duration::from_millis(20));
        }
    }

    /// Stops the runner, and gives the lines said from the last read, its
    /// stop the last, each with when it was said.
    fn stop(self) -> Vec<(Instant, String)> {
        self.stopper.stop();
        self.thread.join().expect("the runner ends");
        self.lines.try_iter().collect()
    }
}

/// The line said when a pass makes [`KEPT`]'s view at `commit`.
fn view_made(scratch: &Scratch, commit: &str) -> String {
    format!(
        "{KEPT}: view made at {}, at origin/main {}",
        scratch.home.join("Code").join(KEPT).display(),
        &commit[..7]
    )
}

/// Writes `text` as the configuration by one rename, so the running
/// runner never reads it half written.
fn rewrite(scratch: &Scratch, text: &str) {
    let file = scratch.config_file();
    let next = file.with_extension("next");
    std::fs::write(&next, text).expect("the configuration written");
    std::fs::rename(&next, &file).expect("the configuration in place");
}

/// The running spec's "Passes back to back share their ticks": the first
/// keeping pass and the first control pass run back to back, and one tick
/// of the inbox `home` follows them; no other runs before the next turn,
/// an hour on, and Flume is asked for no settle.
#[test]
fn passes_back_to_back_share_their_ticks() {
    let scratch = Scratch::new();
    scratch.origin(KEPT);
    scratch.configure(&format!("inboxes = [\"home\"]\n{}", carrying("1h", "1h")));
    let running = Running::start(&scratch);
    running.pull();
    running.ticked(1);
    thread::sleep(Duration::from_millis(500));

    assert_eq!(
        running.tools.millpond.calls(),
        [words(&["tick", "--inbox", "home", "--json"])]
    );
    assert_eq!(
        settles(&running.tools),
        0,
        "{:?}",
        running.tools.flume_commands()
    );
    running.stop();
}

/// The running spec's "A control pass runs no settle" and "Passes read
/// the subscription": with `poll_interval` a second, three control passes
/// read Flume's status from the runner's one subscription to the stub
/// flume host, which names the kept clone's folder, and the stub `flume`
/// is asked no status and no settle.
#[test]
fn passes_read_the_subscription() {
    let scratch = Scratch::new();
    scratch.origin(KEPT);
    scratch.configure(&carrying("1h", "1s"));
    let running = Running::start(&scratch);
    for _ in 0..3 {
        running.pull();
    }
    let calls = running.tools.flume.calls();
    let asks = running.host.asks();
    running.stop();

    assert_eq!(asks.len(), 1, "one subscription: {asks:?}");
    let asked: serde_json::Value = serde_json::from_str(&asks[0]).expect("an ask");
    assert_eq!(asked["ask"], "subscribe", "{asked}");
    let named = asked["repositories"].as_array().expect("the folders named");
    assert_eq!(named.len(), 1, "{asked}");
    assert!(
        named[0]
            .as_str()
            .is_some_and(|folder| folder.ends_with(&format!("/repositories/{KEPT}"))),
        "the kept clone's folder: {asked}"
    );
    assert!(
        !calls
            .iter()
            .any(|call| matches!(call.get(1).map(String::as_str), Some("status" | "settle"))),
        "{calls:?}"
    );
}

/// The running spec's "A change heard wakes the parts": with
/// `poll_interval` an hour, once `home`'s first control pass has ended, a
/// change of the stub flume host's document makes a control pass within
/// seconds.
#[test]
fn a_change_heard_wakes_the_parts() {
    let scratch = Scratch::new();
    scratch.origin(KEPT);
    scratch.configure(&carrying("1h", "1h"));
    let running = Running::start(&scratch);
    running.pull();
    // The first document wakes the part too; the passes it makes come
    // at once, and then none until the document changes.
    while running.pulled_within(Duration::from_secs(1)) {}

    let changed = Instant::now();
    running
        .tools
        .flume
        .answers("status", "status-live-agent.json", 0);
    let woke = running.pull();
    assert!(
        woke - changed < Duration::from_secs(5),
        "{:?} from the change to the pass",
        woke - changed
    );
    running.stop();
}

/// The running spec's "flume host stops and comes back": with the stub
/// flume host stopped, the runner says once that flume host does not
/// answer, and `home`'s pass says its status could not be read; started
/// again, the runner says once that it answers again.
#[test]
fn flume_host_stops_and_comes_back() {
    let scratch = Scratch::new();
    scratch.origin(KEPT);
    scratch.configure(&carrying("1h", "1s"));
    let mut running = Running::start(&scratch);
    running.pull();
    running.host.stop();
    let (_, line, _) = running.until_starting("boomdrive: flume host does not answer: ");
    assert!(line.contains("closed the connection"), "{line}");
    running.until_starting("boomdrive: home: Flume's status could not be read: ");
    running.host.start();
    let (_, _, between) = running.until_starting("boomdrive: flume host answers again");
    assert!(
        !between
            .iter()
            .any(|line| line.starts_with("boomdrive: flume host does not answer")),
        "said twice: {between:?}"
    );
    running.stop();
}

/// The running spec's "The configuration broken while running": `home`'s
/// next control pass names the file and acts on nothing, and once the file
/// is valid again, the pass after acts as before.
#[test]
fn the_configuration_broken_while_running() {
    let scratch = Scratch::new();
    scratch.origin(KEPT);
    let valid = carrying("1h", "1s");
    scratch.configure(&valid);
    let running = Running::start(&scratch);
    running.pull();

    rewrite(&scratch, &valid.replace(&format!("{KEPT:?}"), "\"flume\""));
    let broken = format!(
        "boomdrive: home: {}:2: `accounts.own.drives.home.repositories` entry \"flume\": an \
         address is `owner/name`: nothing carried this pass",
        scratch.config_file().display()
    );
    running.until(&broken);
    // A pull begun before the file broke may be told of; none after.
    running.home().pulls.try_iter().for_each(drop);
    running.until(&broken);
    assert_eq!(
        running.home().pulls.try_iter().count(),
        0,
        "no pull while the file does not read"
    );

    rewrite(&scratch, &valid);
    running.pull();
    running.stop();
}

/// The configuration spec's "A poll interval changed while running": the
/// control passes keep the interval the runner started with.
#[test]
fn a_poll_interval_changed_while_running() {
    let scratch = Scratch::new();
    scratch.origin(KEPT);
    scratch.configure(&carrying("1h", "1s"));
    let running = Running::start(&scratch);
    running.pull();

    rewrite(&scratch, &carrying("1h", "5m"));
    running.pull();
    running.pull();
    running.stop();
}

/// The running spec's "A lapsed lease", over the runner: the report file
/// holds a launch under a lease that lapsed, and the control plane cannot
/// be reached, so the next control pass comes `poll_interval` after the
/// one before, never back to back.
#[test]
fn a_lapsed_lease_sets_no_pass_sooner() {
    let scratch = Scratch::new();
    scratch.origin(KEPT);
    scratch.configure(&carrying("1h", "2s"));
    holding(
        &scratch,
        "held",
        At::from(At::now().time() - Duration::from_secs(60)),
    );
    let mut fake = Fake::new();
    fake.refuse_pulls("no network");
    let running = Running::over(&scratch, fake, 0);

    let first = running.pull();
    let second = running.pull();

    // A pull comes partway into its pass; passes two seconds apart hold
    // their pulls more than a second apart, where back to back they come
    // within the pass's own time.
    assert!(
        second - first >= Duration::from_secs(1),
        "{:?} between control passes",
        second - first
    );
    running.stop();
}

/// The running spec's "A pass that did not report", over the runner: the
/// report file holds a launch under a lease that lapses in seconds, and
/// the control plane takes no report, so no pass is made sooner than
/// `poll_interval`, an hour.
#[test]
fn a_pass_that_did_not_report_sets_no_pass_sooner() {
    let scratch = Scratch::new();
    scratch.origin(KEPT);
    scratch.configure(&carrying("1h", "1h"));
    holding(
        &scratch,
        "held",
        At::from(At::now().time() + Duration::from_secs(5)),
    );
    let mut fake = Fake::new();
    fake.refuse_reports("no network");
    let running = Running::over(&scratch, fake, 0);

    running.pull();

    // Hastened by the lease, the next pass would come within half the
    // time it has left, two and a half seconds.
    assert!(
        !running.pulled_within(Duration::from_secs(4)),
        "a pass that did not report hastens none"
    );
    running.stop();
}

/// The running spec's "Each part on its own timer": `work` holds a lease
/// expiring seconds after a pass of `work` that reported, so `work`'s next
/// pass comes by the midpoint of the time left, and `home`'s keeps its
/// own `poll_interval`.
#[test]
fn each_part_on_its_own_timer() {
    let scratch = Scratch::new();
    scratch.origin(KEPT);
    scratch.configure(&carrying("1h", "8s"));
    holding(
        &scratch,
        "work/held",
        At::from(At::now().time() + Duration::from_secs(8)),
    );
    let (home, home_heard) = told(Fake::owning("home/"), 0);
    let (work, work_heard) = told(Fake::owning("work/"), 0);
    let running = Running::carrying(&scratch, vec![("home", home), ("work", work)], Tools::new());

    let home_first = home_heard.pull();
    let work_first = work_heard.pull();
    let work_second = work_heard.pull();
    let home_second = home_heard.pull();

    let work_apart = work_second - work_first;
    assert!(
        work_apart < Duration::from_secs(6),
        "{work_apart:?} between work's passes, hastened by its lease"
    );
    let home_apart = home_second - home_first;
    assert!(
        home_apart >= Duration::from_secs(7),
        "{home_apart:?} between home's passes, its own poll_interval"
    );
    running.stop();
}

/// The running spec's "A keeping pass under way": a keeping pass's fetch
/// of [`KEPT`] is held, and `work`'s control passes fall due meanwhile:
/// they pull, answer and report while the fetch is held.
#[test]
fn a_keeping_pass_under_way() {
    let scratch = Scratch::new();
    let origin = scratch.origin(KEPT);
    scratch.configure(&carrying("3s", "1s"));
    let held = HeldFetch::new(&scratch.gitconfig, KEPT);
    let tools = Tools::new();
    tools.flume.answers("launch", "launch.json", 0);
    let (work, work_heard) = told(part_holding("work", vec![launch("work/1")]), 0);
    let running = Running::carrying(&scratch, vec![("work", work)], tools);
    running.until(&view_made(&scratch, &origin.tip("main")));
    // `work/1` is launched before the fetch is held: a launch into the
    // clone waits on its keep (core.49; record 0026), so one made after
    // the hold would keep `work`'s pass waiting until the fetch ends.
    running.until_starting("work/1: launched at ");
    held.hold();
    held.wait_held(PATIENCE);
    // Every pull and report the test has heard of may have come before
    // the fetch was held; those that come now come while it is held.
    work_heard.pulls.try_iter().for_each(drop);
    work_heard.reports.try_iter().for_each(drop);

    work_heard.pull();
    work_heard.report();
    work_heard.pull();
    work_heard.report();

    held.let_go();
    running.stop();
}

/// Writes the runner's report file on the scratch host, holding launch
/// `id` at a place under lease 1, expiring at `expires`.
fn holding(scratch: &Scratch, id: &str, expires: At) {
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
    let control_dir = report::control_dir(&scratch.data());
    std::fs::create_dir_all(&control_dir).expect("the control directory");
    report::save(&report::file(&control_dir), &held).expect("the report saved");
}
