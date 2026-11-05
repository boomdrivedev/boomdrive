//! `boomdrive run` carrying a control plane's work (design Decisions 4, 5
//! and 13): `--once` makes one keeping pass and one control pass, and the
//! loop makes control passes on a timer of their own, hastened by no
//! lapsed lease and after no pass that did not report, at once when the
//! control plane wakes it, and none that acts while the configuration
//! does not read, saying the control plane unread once until it is read
//! again (the running spec's "boomdrive run --once makes one pass",
//! "boomdrive run makes control passes on a timer of their own", "A
//! control pass runs at once when its control plane wakes the runner", "A
//! control pass after a broken configuration acts on nothing" and "A
//! control plane that cannot be read is said once until it is read
//! again"; the
//! configuration spec's "The configuration may name the control plane"
//! and "The control-plane table is read when the runner starts"). The
//! binary runs under the scratch host with stub `flume`, `sluice`,
//! `herdr` and `systemd-creds` first on its `PATH`, crew's implementation
//! reaches the scratch forge's state repository through the scratch git
//! configuration, and the hosted implementation a loopback port with
//! nothing on it. The loop runs in-process over the fake control plane.

mod common;
mod fakes;
#[path = "../../boomdrive-crew/tests/common/mod.rs"]
mod records;

use boomdrive::report;
use boomdrive::run::{Programs, Runner, Stopper};
use boomdrive_control::fake::Fake;
use boomdrive_control::{
    Answer, At, ControlPlane, Hello, Id, Lease, Outcome, Pulled, Report, Reported, Unread, Unsent,
    Wake,
};
use common::{keeping, Origin, Scratch};
use fakes::{CredsStub, HerdrStub, SluiceStub, Stub};
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
/// crew's work for the host `deneb` in the partition `madswan`, each
/// `poll_interval`.
fn carrying(fetch_interval: &str, poll_interval: &str) -> String {
    format!(
        "fetch_interval = {fetch_interval:?}\n{}\n\
         [control_plane]\n\
         type = \"crew\"\n\
         poll_interval = {poll_interval:?}\n\n\
         [control_plane.crew]\n\
         host = \"deneb\"\n\n\
         [[control_plane.crew.partitions]]\n\
         label = \"madswan\"\n\
         state_repository = \"afterthought/madswan-state\"\n",
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

/// The state repository `afterthought/madswan-state` on the scratch
/// forge, its `madswan/main` holding a launch request for `deneb` of
/// [`KEPT`]'s unit `a`, and the plan holding `a`, as crew writes them.
fn state_repository(scratch: &Scratch) -> Origin<'_> {
    let state = scratch.empty_origin("afterthought/madswan-state");
    let request = records::launch(
        1,
        Some("deneb"),
        "unit/a",
        KEPT,
        "a",
        &[("Base", "main"), ("Brief", "Parse the input.")],
    );
    state.commit(
        "madswan/main",
        "requests.rec",
        &records::file(records::REQUESTS, &[request]),
    );
    state.commit("madswan/main", "plan.rec", &records::plan(&["a"], &[]));
    state
}

/// The running spec's "One control pass": crew's implementation over a
/// scratch state repository holding a launch for this host, and a stub
/// `flume` that launches it. The runner keeps its repositories, launches
/// once, writes its branch, and exits 0.
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
    assert!(
        stderr(&output)
            .lines()
            .any(|line| line.starts_with("madswan/1: launched at ")),
        "{}",
        stderr(&output)
    );
    let answers = scratch.git(&state.bare, &["show", "madswan/runner/deneb:answers.rec"]);
    assert!(answers.contains("Outcome: launched"), "{answers}");
    let runner = scratch.git(&state.bare, &["show", "madswan/runner/deneb:runner.rec"]);
    assert!(runner.contains("Host: deneb"), "{runner}");
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
            .any(|line| line.starts_with("boomdrive: Flume's status could not be read: ")),
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
    let runner = scratch.git(&state.bare, &["show", "madswan/runner/deneb:runner.rec"]);
    assert!(runner.contains("Host: deneb"), "{runner}");
}

/// The running spec's "A control plane that cannot be read": no
/// partition's state repository can be read. The runner keeps its
/// repositories, names the control plane as unread, launches and acts on
/// nothing, and exits 1. The pass reads Flume's status and each tool's
/// version for the hello it hands the pull, before the pull is read
/// (design Decision 5, step 2): no other Flume command runs.
#[test]
fn a_control_plane_that_cannot_be_read() {
    let scratch = Scratch::new();
    scratch.origin(KEPT);
    scratch.configure(&carrying("2m", "1m"));
    let tools = Tools::new();

    let output = once(&scratch, &tools);

    assert_eq!(output.status.code(), Some(1), "{}", stderr(&output));
    assert!(scratch.home.join("Code").join(KEPT).exists());
    let said = stderr(&output);
    let unread: Vec<&str> = said
        .lines()
        .filter(|line| line.starts_with("boomdrive: the control plane could not be read: "))
        .collect();
    assert_eq!(unread.len(), 1, "{said}");
    for command in tools.flume_commands() {
        assert!(
            ["status", "--version"].contains(&command.as_str()),
            "{:?}",
            tools.flume_commands()
        );
    }
}

/// The configuration spec's "No control plane": `--once` keeps the
/// repositories and runs no `flume` command.
#[test]
fn no_control_plane() {
    let scratch = Scratch::new();
    scratch.origin(KEPT);
    scratch.configure(&keeping(&[KEPT]));
    let tools = Tools::new();

    let output = once(&scratch, &tools);

    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    assert!(scratch.home.join("Code").join(KEPT).exists());
    assert_eq!(tools.flume.calls(), Vec::<Vec<String>>::new());
    assert_eq!(tools.sluice.calls(), Vec::<Vec<String>>::new());
    assert_eq!(tools.herdr.calls(), Vec::<Vec<String>>::new());
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
        .filter(|line| line.starts_with("boomdrive: the control plane could not be read: "))
        .collect();
    assert_eq!(unread.len(), 1, "{said}");
    assert!(unread[0].contains("home"), "{said}");
    assert!(scratch.home.join("Code").join(KEPT).exists());
    assert!(!tools.creds.kept_or_read(), "{:?}", tools.creds.calls());
}

/// The control-plane-interface spec's "A word not carried": the runner
/// names `elsewhere` as a control plane this binary does not carry and
/// names `crew` and `hosted`, runs no control pass, and keeps its
/// repositories. Its control pass did not pull, so `--once` exits 1.
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
    assert_eq!(tools.flume.calls(), Vec::<Vec<String>>::new());
}

/// A loopback address with nothing listening on it.
fn loopback_with_nothing_on_it() -> String {
    let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("a loopback port");
    let port = listener.local_addr().expect("its address").port();
    drop(listener);
    format!("http://127.0.0.1:{port}/boomdrive")
}

/// The fake control plane, telling the test when each pull is made.
struct Told {
    fake: Fake,
    /// How many pulls are still refused, with `no network`, before the
    /// fake is asked.
    unread: usize,
    pulls: Sender<Instant>,
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
        self.fake.report(report)
    }
}

/// The runner's loop over the fake control plane, running in a thread of
/// the test's, with the stub tools.
struct Running {
    /// The wake the control plane was handed at its opening, as a pushing
    /// implementation's own thread would hold it.
    wake: Wake,
    stopper: Stopper,
    lines: Receiver<(Instant, String)>,
    pulls: Receiver<Instant>,
    thread: JoinHandle<()>,
    _tools: Tools,
}

impl Running {
    /// Starts the runner on the scratch host's default configuration
    /// file, which names crew's implementation, and carries the fake
    /// control plane's work in its place.
    fn start(scratch: &Scratch) -> Running {
        Running::over(scratch, Fake::new(), 0)
    }

    /// Starts the runner as [`Running::start`] does, carrying `fake`'s
    /// work, its first `unread` pulls refused with `no network`.
    fn over(scratch: &Scratch, fake: Fake, unread: usize) -> Running {
        let overlay = scratch.overlay();
        let var = |name: &str| {
            overlay
                .iter()
                .find(|(key, _)| key.as_os_str() == OsStr::new(name))
                .map(|(_, value)| value.clone())
        };
        let tools = Tools::new();
        let mut runner = Runner::start(None, var, scratch.runner_git()).expect("the runner starts");
        let (told, pulls) = mpsc::channel();
        let plane = Told {
            fake,
            unread,
            pulls: told,
        };
        let mut handed = None;
        runner
            .carry_for(
                |wake| {
                    handed = Some(wake);
                    Box::new(plane)
                },
                tools.programs(),
            )
            .expect("the configuration names a control plane");
        let wake = handed.expect("the control plane is handed the runner's wake");
        let stopper = runner.stopper();
        let (said, lines) = mpsc::channel();
        let thread = thread::spawn(move || {
            runner.keep(&mut |line| {
                let _ = said.send((Instant::now(), line));
            })
        });
        Running {
            wake,
            stopper,
            lines,
            pulls,
            thread,
            _tools: tools,
        }
    }

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

    /// When `line` is said, the lines said before it since the last read;
    /// panics where it is not said in time.
    fn until(&self, line: &str) -> (Instant, Vec<String>) {
        let deadline = Instant::now() + PATIENCE;
        let mut said = Vec::new();
        while let Ok((at, next)) = self
            .lines
            .recv_timeout(deadline.saturating_duration_since(Instant::now()))
        {
            if next == line {
                return (at, said);
            }
            said.push(next);
        }
        panic!("{line:?} not said in time; said: {said:#?}");
    }

    fn stop(self) {
        self.stopper.stop();
        self.thread.join().expect("the loop ends");
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

/// The running spec's "boomdrive run makes control passes on a timer of
/// their own": a control pass follows the first keeping pass, and the
/// next come each `poll_interval`, while the keeping pass keeps its own
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

/// The running spec's "Woken by the control plane": woken through the
/// wake its control plane was handed at its opening, a control pass runs
/// at once, and none follows it within the hour's interval.
#[test]
fn woken_by_the_control_plane() {
    let scratch = Scratch::new();
    scratch.origin(KEPT);
    scratch.configure(&carrying("1h", "1h"));
    let running = Running::start(&scratch);
    running.pull();

    let woken = Instant::now();
    (running.wake)();
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
    (running.wake)();
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

/// The running spec's "The configuration broken while running": the next
/// control pass names the file and acts on nothing, and once the file is
/// valid again, the pass after acts as before.
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
        "boomdrive: {}:2: `repositories` entry \"flume\": an address is `owner/name`: \
         nothing carried this pass",
        scratch.config_file().display()
    );
    running.until(&broken);
    // A pull begun before the file broke may be told of; none after.
    running.pulls.try_iter().for_each(drop);
    running.until(&broken);
    assert_eq!(
        running.pulls.try_iter().count(),
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
/// over a control plane wanting nothing, the loop says nothing after its
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
    running.thread.join().expect("the loop ends");
    assert_eq!(said, Vec::<String>::new());
}

/// The running spec's "A lapsed lease", over the loop: the report file
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

/// The running spec's "A pass that did not report", over the loop: the
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

/// The running spec's "Three passes unread, then read": the control
/// plane refuses three pulls in a row with `no network`, then gives the
/// fourth. One line names it unread, and one, at the fourth pass, says it
/// is read again; the passes after say nothing of it.
#[test]
fn three_passes_unread_then_read() {
    let scratch = Scratch::new();
    scratch.origin(KEPT);
    scratch.configure(&carrying("1h", "1s"));
    let running = Running::over(&scratch, Fake::new(), 3);

    let (read_again, said) = running.until("boomdrive: the control plane is read again");
    let pulls: Vec<Instant> = (0..4).map(|_| running.pull()).collect();
    let reach: Vec<&String> = said
        .iter()
        .filter(|line| line.starts_with("boomdrive: the control plane"))
        .collect();
    assert_eq!(
        reach,
        ["boomdrive: the control plane could not be read: no network"]
    );
    assert!(read_again >= pulls[3], "said at the fourth pass");
    assert!(
        running.pulls.try_iter().all(|fifth| fifth > read_again),
        "said before the fifth pass"
    );

    running.pull();
    running.pull();
    running.stopper.stop();
    let (_, after) = running.until("boomdrive: stopped");
    running.thread.join().expect("the loop ends");
    assert_eq!(after, Vec::<String>::new());
}

/// Writes the runner's report file on the scratch host, holding launch
/// `held` at a place under lease 1, expiring at `expires`.
fn holding(scratch: &Scratch, expires: At) {
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
    held.answer(&Id::new("held"), lease, answer);
    let control_dir = report::control_dir(&scratch.data());
    std::fs::create_dir_all(&control_dir).expect("the control directory");
    report::save(&report::file(&control_dir), &held).expect("the report saved");
}
