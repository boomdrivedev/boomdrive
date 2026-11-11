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
//! binary runs under the scratch host with stub `flume`, `sluice`,
//! `herdr` and `systemd-creds` first on its `PATH`, crew's implementation
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
    Unsent, Wake, Want,
};
use common::{keeping, Origin, Scratch};
use fakes::{offer, CredsStub, HeldFetch, HerdrStub, SluiceStub, Stub};
use std::collections::BTreeMap;
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
    format!(
        "fetch_interval = {fetch_interval:?}\n{}\n\
         [control_plane]\n\
         type = \"crew\"\n\
         poll_interval = {poll_interval:?}\n\n\
         [control_plane.crew]\n\
         host = \"host-a\"\n\n\
         [[control_plane.crew.partitions]]\n\
         label = \"home\"\n\
         state_repository = \"example/home-state\"\n",
        keeping(&[KEPT])
    )
}

/// Stub `flume`, `sluice`, `herdr` and `systemd-creds`, each in a
/// directory of its own.
struct Tools {
    flume: Stub,
    sluice: SluiceStub,
    herdr: HerdrStub,
    creds: CredsStub,
}

impl Tools {
    fn new() -> Tools {
        Tools {
            flume: Stub::new(),
            sluice: SluiceStub::new(),
            herdr: HerdrStub::new(),
            creds: CredsStub::new(),
        }
    }

    fn programs(&self) -> Programs {
        Programs {
            flume: self.flume.program(),
            sluice: self.sluice.program(),
            herdr: self.herdr.program(),
        }
    }

    /// The test's `PATH` with the stubs first, so no real Flume, sluice,
    /// herdr or `systemd-creds` runs.
    fn path(&self) -> OsString {
        let stubs = [
            &self.flume.dir,
            &self.sluice.dir,
            &self.herdr.dir,
            &self.creds.dir,
        ]
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
    scratch
        .boomdrive()
        .args(["run", "--once"])
        .env("PATH", tools.path())
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

/// How many settles the stub `flume` was asked.
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
/// once, writes its branch, settles every place after every other `flume`
/// command, on the shorter interval (the running spec's "The shorter
/// interval"), and exits 0.
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
    let calls = tools.flume.calls();
    assert_eq!(
        calls.last(),
        Some(&words(&["--json", "settle", "--every", "60s"])),
        "{calls:?}"
    );
    assert_eq!(settles(&tools), 1, "{calls:?}");
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

/// The running spec's "Flume's status refused": the control pass pulled
/// and reported, so the tool's refusal is named and no failure. Nothing
/// is launched, the runner's branch is written, and the run exits 0.
#[test]
fn flumes_status_refused() {
    let scratch = Scratch::new();
    scratch.origin(KEPT);
    let state = state_repository(&scratch);
    scratch.configure(&carrying("2m", "1m"));
    let tools = Tools::new();
    tools.flume.answers_text("status", "", 1);

    let output = once(&scratch, &tools);

    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    assert!(
        stderr(&output)
            .lines()
            .any(|line| line.starts_with("boomdrive: home: Flume's status could not be read: ")),
        "{}",
        stderr(&output)
    );
    assert!(
        !tools
            .flume_commands()
            .iter()
            .any(|command| command == "launch"),
        "{:?}",
        tools.flume_commands()
    );
    let runner = scratch.git(&state.bare, &["show", "home/runner/host-a:runner.rec"]);
    assert!(runner.contains("Host: host-a"), "{runner}");
}

/// A configuration keeping [`KEPT`], carrying crew's work for the host
/// `host-a` in the partitions `home` and `work`, both of the state
/// repository `example/state`.
fn carrying_two() -> String {
    format!(
        "{}\n[control_plane]\ntype = \"crew\"\n\n\
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
/// and exits 1. Each part's pass reads Flume's status and each tool's
/// version for the hello it hands the pull, before the pull is read
/// (design Decision 5, step 2), and the settle ends the pass: no other
/// Flume command runs.
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
        assert!(
            ["status", "--version", "settle"].contains(&command.as_str()),
            "{:?}",
            tools.flume_commands()
        );
    }
    assert_eq!(settles(&tools), 1, "{:?}", tools.flume_commands());
}

/// The running spec's "One part unread": crew's control plane serves
/// `home` and `work`, the state repository holds no `home/main`, and
/// `work/main` holds a launch for this host. The runner names `home` as
/// unread, launches `work`'s launch once, writes `work`'s runner branch,
/// and exits 0.
#[test]
fn one_part_unread() {
    let scratch = Scratch::new();
    scratch.origin(KEPT);
    let state = scratch.empty_origin("example/state");
    let request = records::launch(
        1,
        Some("host-a"),
        "unit/a",
        KEPT,
        "a",
        &[("Base", "main"), ("Brief", "Parse the input.")],
    );
    state.commit(
        "work/main",
        "requests.rec",
        &records::file(records::REQUESTS, &[request]),
    );
    state.commit("work/main", "plan.rec", &records::plan(&["a"], &[]));
    scratch.configure(&carrying_two());
    let tools = Tools::new();
    tools.flume.answers("launch", "launch.json", 0);

    let output = once(&scratch, &tools);

    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    let said = stderr(&output);
    assert!(
        said.lines()
            .any(|line| line.starts_with("boomdrive: home: the control plane could not be read: ")),
        "{said}"
    );
    let launches = tools
        .flume_commands()
        .into_iter()
        .filter(|command| command == "launch")
        .count();
    assert_eq!(launches, 1, "{:?}", tools.flume_commands());
    assert!(
        said.lines()
            .any(|line| line.starts_with("work/1: launched at ")),
        "{said}"
    );
    let answers = scratch.git(&state.bare, &["show", "work/runner/host-a:answers.rec"]);
    assert!(answers.contains("Outcome: launched"), "{answers}");
    assert!(!scratch.holds(
        &state.bare,
        &[
            "rev-parse",
            "--verify",
            "--quiet",
            "refs/heads/home/runner/host-a"
        ]
    ));
}

/// The configuration spec's "No control plane": `--once` keeps the
/// repositories and runs no `flume` command but the settle that ends its
/// pass, on the default `fetch_interval`.
#[test]
fn no_control_plane() {
    let scratch = Scratch::new();
    scratch.origin(KEPT);
    scratch.configure(&keeping(&[KEPT]));
    let tools = Tools::new();

    let output = once(&scratch, &tools);

    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    assert!(scratch.home.join("Code").join(KEPT).exists());
    assert_eq!(
        tools.flume.calls(),
        [words(&["--json", "settle", "--every", "120s"])]
    );
    assert_eq!(tools.sluice.calls(), Vec::<Vec<String>>::new());
    assert_eq!(tools.herdr.calls(), Vec::<Vec<String>>::new());
}

/// The running spec's "A settle Flume refuses": with no control plane, a
/// stub `flume` that refuses the settle. `--once` says Flume's refusal in
/// its words, and exits 0: a refused settle is no failure.
#[test]
fn a_settle_flume_refuses() {
    let scratch = Scratch::new();
    scratch.origin(KEPT);
    scratch.configure(&keeping(&[KEPT]));
    let tools = Tools::new();
    tools.flume.answers_text(
        "settle",
        "{\"refused\":{\"refusal\":{\"reason\":\"no store\"}}}\n",
        1,
    );

    let output = once(&scratch, &tools);

    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    let refused = "boomdrive: Flume refused the settle of every place: no store";
    assert_eq!(
        stderr(&output)
            .lines()
            .filter(|line| *line == refused)
            .count(),
        1,
        "{}",
        stderr(&output)
    );
    assert!(scratch.home.join("Code").join(KEPT).exists());
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
        "{}[control_plane]\ntype = \"hosted\"\n\n\
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

/// The control-plane-interface spec's "A word not carried": the runner
/// names `elsewhere` as a control plane this binary does not carry and
/// names `crew` and `hosted`, runs no control pass, and keeps its
/// repositories. Its control pass did not pull, so `--once` exits 1. It
/// runs no `flume` command but the settle, on `fetch_interval` alone (the
/// running spec's "A control plane that could not be opened").
#[test]
fn a_word_not_carried() {
    let scratch = Scratch::new();
    scratch.origin(KEPT);
    let file = scratch.configure(&format!(
        "{}[control_plane]\ntype = \"elsewhere\"\n",
        keeping(&[KEPT])
    ));
    let tools = Tools::new();

    let output = once(&scratch, &tools);

    assert_eq!(output.status.code(), Some(1), "{}", stderr(&output));
    let named = format!(
        "boomdrive: {}: `type` is \"elsewhere\", a control plane this binary does not carry: \
         it carries `crew`, `hosted`: no control pass runs",
        file.display()
    );
    assert!(
        stderr(&output).lines().any(|line| line == named),
        "{}",
        stderr(&output)
    );
    assert!(scratch.home.join("Code").join(KEPT).exists());
    assert_eq!(
        tools.flume.calls(),
        [words(&["--json", "settle", "--every", "120s"])]
    );
    assert_eq!(tools.sluice.calls(), Vec::<Vec<String>>::new());
    assert_eq!(tools.herdr.calls(), Vec::<Vec<String>>::new());
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
}

/// A control plane of the test's own whose pull waits on the test: it
/// tells the test when a pull begins, and answers it, as the fake does,
/// only once the test lets it go.
struct Slow {
    fake: Fake,
    pulling: Sender<Instant>,
    go: Receiver<()>,
    reports: Sender<Instant>,
}

impl ControlPlane for Slow {
    fn pull(&mut self, hello: &Hello, report: &Report) -> Result<Pulled, Unread> {
        let _ = self.pulling.send(Instant::now());
        let _ = self.go.recv();
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

/// `fake` as a part's control plane whose pulls wait on the test, what
/// the test hears of it, and what lets each pull go.
fn slow(fake: Fake) -> (Box<dyn ControlPlane + Send>, Heard, Sender<()>) {
    let (pulling, pulled) = mpsc::channel();
    let (reports, reported) = mpsc::channel();
    let (go, gone) = mpsc::channel();
    let plane = Slow {
        fake,
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
    /// The wake each part's implementation was handed at its opening, by
    /// the part's label, as a pushing implementation's own thread would
    /// hold it.
    wakes: BTreeMap<String, Wake>,
    /// What the test hears of `home`'s control plane, where the runner
    /// carries one the harness made.
    home: Option<Heard>,
    stopper: Stopper,
    lines: Receiver<(Instant, String)>,
    thread: JoinHandle<()>,
    tools: Tools,
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
        let mut runner = Runner::start(None, var, scratch.runner_git()).expect("the runner starts");
        let mut wakes = BTreeMap::new();
        runner
            .carry_for(
                |wake_for| {
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
                },
                tools.programs(),
            )
            .expect("the configuration names a control plane");
        let stopper = runner.stopper();
        let (said, lines) = mpsc::channel();
        let thread = thread::spawn(move || {
            runner.keep(&|line| {
                let _ = said.send((Instant::now(), line));
            })
        });
        Running {
            wakes,
            home: None,
            stopper,
            lines,
            thread,
            tools,
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

    /// Wakes the part `label` as its implementation would.
    fn wake(&self, label: &str) {
        (self.wakes[label])();
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

    /// Waits until the stub `flume` has been asked `settles` settles;
    /// panics where it is not in time.
    fn settled(&self, settles: usize) {
        let deadline = Instant::now() + PATIENCE;
        while self::settles(&self.tools) < settles {
            assert!(
                Instant::now() < deadline,
                "{settles} settles not run in time: {:?}",
                self.tools.flume_commands()
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

/// The commit `KEPT`'s clone last fetched origin's `main` at.
fn fetched(scratch: &Scratch) -> String {
    scratch.git(
        &scratch.clone_of(KEPT),
        &["rev-parse", "refs/remotes/origin/main"],
    )
}

/// The running spec's "On the control timer", over a part: `home`'s
/// first control pass follows the first keeping pass, and the next come
/// each `poll_interval`, while the keeping pass keeps its own
/// `fetch_interval`.
#[test]
fn control_passes_on_their_own_timer() {
    let scratch = Scratch::new();
    let origin = scratch.origin(KEPT);
    scratch.configure(&carrying("1h", "1s"));
    let running = Running::start(&scratch);
    let (kept, _) = running.until(&view_made(&scratch, &origin.tip("main")));
    let first = running.pull();
    assert!(
        first >= kept,
        "the first control pass follows the keeping pass"
    );
    let was = fetched(&scratch);
    origin.commit("main", "later", "not fetched within the hour\n");

    running.pull();
    let third = running.pull();

    // A pull comes partway into its pass, after Flume's status and the
    // tools' versions are read, so two pulls may stand closer than the
    // passes' starts; the timer's own times are its unit tests'. Three
    // passes a second apart span two seconds, and never less than one.
    assert!(
        third - first >= Duration::from_secs(1),
        "{:?} from the first control pass to the third",
        third - first
    );
    running.stop();
    assert_eq!(fetched(&scratch), was, "no keeping pass but the first");
}

/// The running spec's "Passes back to back share one settle": the first
/// keeping pass and the first control pass run back to back, and one
/// settle follows them, on the shorter interval, after every other
/// `flume` command of the control pass; no other runs before the next
/// turn, an hour on.
#[test]
fn passes_back_to_back_share_one_settle() {
    let scratch = Scratch::new();
    scratch.origin(KEPT);
    scratch.configure(&carrying("1h", "1h"));
    let running = Running::start(&scratch);
    running.pull();
    running.settled(1);
    thread::sleep(Duration::from_millis(500));

    let calls = running.tools.flume.calls();
    assert_eq!(settles(&running.tools), 1, "{calls:?}");
    assert_eq!(
        calls.last(),
        Some(&words(&["--json", "settle", "--every", "3600s"])),
        "{calls:?}"
    );
    running.stop();
}

/// The running spec's "Each control pass ends with a settle": with
/// `poll_interval` a second, each control pass's `flume` commands, from
/// its read of Flume's status, end with one settle on the shorter
/// interval, before the next control pass reads Flume's status.
#[test]
fn each_control_pass_ends_with_a_settle() {
    let scratch = Scratch::new();
    scratch.origin(KEPT);
    scratch.configure(&carrying("1h", "1s"));
    let running = Running::start(&scratch);
    for _ in 0..3 {
        running.pull();
    }
    let calls = running.tools.flume.calls();
    running.stop();

    let starts: Vec<usize> = calls
        .iter()
        .enumerate()
        .filter(|(_, call)| *call == &words(&["--json", "status"]))
        .map(|(n, _)| n)
        .collect();
    assert!(starts.len() >= 3, "three control passes: {calls:?}");
    assert_eq!(
        starts[0], 0,
        "a flume command before the first pass: {calls:?}"
    );
    let settle = words(&["--json", "settle", "--every", "1s"]);
    for pass in starts.windows(2) {
        let group = &calls[pass[0]..pass[1]];
        assert_eq!(group.last(), Some(&settle), "{group:?} in {calls:?}");
        assert_eq!(
            group
                .iter()
                .filter(|call| call.get(1).map(String::as_str) == Some("settle"))
                .count(),
            1,
            "{group:?} in {calls:?}"
        );
    }
}

/// The running spec's "Woken by the control plane": woken through the
/// wake its implementation was handed for `home` at its opening, a
/// control pass of `home` runs at once, and none follows it within the
/// hour's interval.
#[test]
fn woken_by_the_control_plane() {
    let scratch = Scratch::new();
    scratch.origin(KEPT);
    scratch.configure(&carrying("1h", "1h"));
    let running = Running::start(&scratch);
    running.pull();

    let woken = Instant::now();
    running.wake("home");
    let pulled = running.pull();

    assert!(
        pulled - woken < Duration::from_secs(5),
        "{:?}",
        pulled - woken
    );
    assert!(
        !running.pulled_within(Duration::from_secs(2)),
        "the next control pass is an hour after the woken one"
    );
    running.stop();
}

/// The running spec's "One part woken": the wake handed for `home` makes
/// `home`'s control pass at once, and `work`'s next pass begins when its
/// own timer says, an hour on.
#[test]
fn one_part_woken() {
    let scratch = Scratch::new();
    scratch.origin(KEPT);
    scratch.configure(&carrying("1h", "1h"));
    let (home, home_heard) = told(Fake::owning("home/"), 0);
    let (work, work_heard) = told(Fake::owning("work/"), 0);
    let running = Running::carrying(&scratch, vec![("home", home), ("work", work)], Tools::new());
    home_heard.pull();
    work_heard.pull();

    let woken = Instant::now();
    running.wake("home");
    let pulled = home_heard.pull();

    assert!(
        pulled - woken < Duration::from_secs(5),
        "{:?}",
        pulled - woken
    );
    assert!(
        !work_heard.pulled_within(Duration::from_secs(2)),
        "work's next control pass is its own timer's, an hour on"
    );
    running.stop();
}

/// The running spec's "Woken by the control plane", its timer: the pass
/// after the woken one comes `poll_interval` after the woken pass's start,
/// not after the start of the pass before it.
#[test]
fn the_timer_kept_from_the_woken_pass() {
    let scratch = Scratch::new();
    scratch.origin(KEPT);
    scratch.configure(&carrying("1h", "4s"));
    let running = Running::start(&scratch);
    running.pull();
    thread::sleep(Duration::from_secs(1));

    let woken = Instant::now();
    running.wake("home");
    // A pass the timer began before the wake is not the woken one.
    let mut pulled = running.pull();
    while pulled < woken {
        pulled = running.pull();
    }
    let next = running.pull();

    // The woken pass began after the wake, and the next no sooner than
    // four seconds after that; timed from the pass before, it would come
    // a second sooner.
    assert!(
        next - woken >= Duration::from_secs(4),
        "{:?} from the wake to the pass after the woken one",
        next - woken
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

/// A control pass that changes nothing says nothing (design Decision 5):
/// over a control plane wanting nothing, the runner says nothing after its
/// first keeping pass until it stops.
#[test]
fn a_quiet_control_pass_says_nothing() {
    let scratch = Scratch::new();
    let origin = scratch.origin(KEPT);
    scratch.configure(&carrying("1h", "1s"));
    let running = Running::start(&scratch);
    running.until(&view_made(&scratch, &origin.tip("main")));
    running.pull();
    running.pull();
    running.stopper.stop();
    let (_, said) = running.until("boomdrive: stopped");
    running.thread.join().expect("the runner ends");
    assert_eq!(said, Vec::<String>::new());
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

/// The running spec's "Three passes unread, then read": `home` refuses
/// three pulls in a row with `no network`, then gives the fourth. One
/// line names `home` unread, and one, at the fourth pass, says it is read
/// again; the passes after say nothing of it.
#[test]
fn three_passes_unread_then_read() {
    let scratch = Scratch::new();
    scratch.origin(KEPT);
    scratch.configure(&carrying("1h", "1s"));
    let running = Running::over(&scratch, Fake::new(), 3);

    let (read_again, said) = running.until("boomdrive: home: the control plane is read again");
    let pulls: Vec<Instant> = (0..4).map(|_| running.pull()).collect();
    let reach: Vec<&String> = said
        .iter()
        .filter(|line| line.starts_with("boomdrive: home: the control plane"))
        .collect();
    assert_eq!(
        reach,
        ["boomdrive: home: the control plane could not be read: no network"]
    );
    assert!(read_again >= pulls[3], "said at the fourth pass");
    assert!(
        running
            .home()
            .pulls
            .try_iter()
            .all(|fifth| fifth > read_again),
        "said before the fifth pass"
    );

    running.pull();
    running.pull();
    running.stopper.stop();
    let (_, after) = running.until("boomdrive: stopped");
    running.thread.join().expect("the runner ends");
    assert_eq!(after, Vec::<String>::new());
}

/// The running spec's "Each part's reach its own": `home` refuses every
/// pull, and `work` gives every pull. One line names `home` as unread, and
/// no line names `work`'s reach.
#[test]
fn each_parts_reach_its_own() {
    let scratch = Scratch::new();
    scratch.origin(KEPT);
    scratch.configure(&carrying("1h", "1s"));
    let mut refusing = Fake::owning("home/");
    refusing.refuse_pulls("no network");
    let (home, home_heard) = told(refusing, 0);
    let (work, work_heard) = told(Fake::owning("work/"), 0);
    let running = Running::carrying(&scratch, vec![("home", home), ("work", work)], Tools::new());
    for _ in 0..3 {
        home_heard.pull();
        work_heard.pull();
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

/// The running spec's "A slow part": `home`'s pull answers only once the
/// test lets it go, and `work` offers a launch whose stream has room.
/// `work`'s launch is made and reported within seconds, and `work`'s next
/// control pass begins `poll_interval` after the start of its last.
#[test]
fn a_slow_part() {
    let scratch = Scratch::new();
    scratch.origin(KEPT);
    scratch.configure(&carrying("1h", "2s"));
    let tools = Tools::new();
    tools.flume.answers("launch", "launch.json", 0);
    let (home, home_heard, home_go) = slow(Fake::owning("home/"));
    let (work, work_heard) = told(part_holding("work", vec![launch("work/1")]), 0);
    let running = Running::carrying(&scratch, vec![("home", home), ("work", work)], tools);
    let began = home_heard.pull();

    let (launched, _, _) = running.until_starting("work/1: launched at ");
    let reported = work_heard.report();
    let first = work_heard.pull();
    let second = work_heard.pull();

    assert!(
        launched - began < Duration::from_secs(10),
        "{:?}",
        launched - began
    );
    assert!(reported >= launched);
    let apart = second - first;
    assert!(
        apart >= Duration::from_secs(1) && apart < Duration::from_secs(4),
        "{apart:?} between work's passes"
    );
    assert!(
        home_heard.reports.try_recv().is_err(),
        "home's pass still waits"
    );
    home_go.send(()).expect("home's pull let go");
    drop(home_go);
    running.stop();
}

/// The running spec's "A failing part": `home` refuses every pull and
/// report with `no network`, and `work` offers a launch whose stream has
/// room. `work`'s launch is made and reported, and one line says `home`
/// cannot be read.
#[test]
fn a_failing_part() {
    let scratch = Scratch::new();
    scratch.origin(KEPT);
    scratch.configure(&carrying("1h", "1s"));
    let tools = Tools::new();
    tools.flume.answers("launch", "launch.json", 0);
    let mut failing = Fake::owning("home/");
    failing.refuse_pulls("no network");
    failing.refuse_reports("no network");
    let (home, home_heard) = told(failing, 0);
    let (work, work_heard) = told(part_holding("work", vec![launch("work/1")]), 0);
    let running = Running::carrying(&scratch, vec![("home", home), ("work", work)], tools);

    let (_, launched, mut said) = running.until_starting("work/1: launched at ");
    work_heard.report();
    home_heard.pull();
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

/// The running spec's "Stopped with a part's pass under way": stopped
/// while `home`'s control pass waits on its pull and `work` waits for its
/// next pass, `work` begins no pass, `home`'s pass ends with its report
/// saved, the runner says it stopped, and no pass begins after that line.
#[test]
fn stopped_with_a_parts_pass_under_way() {
    let scratch = Scratch::new();
    scratch.origin(KEPT);
    scratch.configure(&carrying("1h", "1h"));
    let tools = Tools::new();
    tools.flume.answers("launch", "launch.json", 0);
    let (home, home_heard, home_go) = slow(part_holding("home", vec![launch("home/1")]));
    let (work, work_heard) = told(Fake::owning("work/"), 0);
    let running = Running::carrying(&scratch, vec![("home", home), ("work", work)], tools);
    home_heard.pull();
    work_heard.pull();
    work_heard.report();

    running.stopper.stop();
    thread::sleep(Duration::from_millis(300));
    home_go.send(()).expect("home's pull let go");
    let reported = home_heard.report();
    let said = running.stop();

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
    let file = report::file(&report::control_dir(&scratch.data()));
    let saved = report::read(&file).unwrap().expect("the report file");
    assert_eq!(
        saved.launches[&Id::new("home/1")].answer.outcome,
        Outcome::Launched
    );
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
