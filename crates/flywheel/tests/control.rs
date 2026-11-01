//! `flywheel run` carrying a control plane's work (design Decisions 4 and
//! 5): `--once` makes one keeping pass and one control pass, and the loop
//! makes control passes on a timer of their own, at once when the control
//! plane wakes it, and none that acts while the configuration does not
//! read (the running spec's "flywheel run --once makes one pass",
//! "flywheel run makes control passes on a timer of their own", "A
//! control pass runs at once when its control plane wakes the runner" and
//! "A control pass after a broken configuration acts on nothing"; the
//! configuration spec's "The configuration may name the control plane"
//! and "The control-plane table is read when the runner starts"). The
//! binary runs under the scratch host with stub `flume`, `sluice` and
//! `herdr` first on its `PATH`, and crew's implementation reaches the
//! scratch forge's state repository through the scratch git
//! configuration. The loop runs in-process over the fake control plane.

mod common;
mod fakes;
#[path = "../../flywheel-crew/tests/common/mod.rs"]
mod records;

use common::{keeping, Origin, Scratch};
use fakes::{HerdrStub, SluiceStub, Stub};
use flywheel::run::{Programs, Runner, Stopper};
use flywheel_control::fake::Fake;
use flywheel_control::{ControlPlane, Hello, Pulled, Report, Reported, Unread, Unsent, Wake};
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

/// Stub `flume`, `sluice` and `herdr`, each in a directory of its own.
struct Tools {
    flume: Stub,
    sluice: SluiceStub,
    herdr: HerdrStub,
}

impl Tools {
    fn new() -> Tools {
        Tools {
            flume: Stub::new(),
            sluice: SluiceStub::new(),
            herdr: HerdrStub::new(),
        }
    }

    fn programs(&self) -> Programs {
        Programs {
            flume: self.flume.program(),
            sluice: self.sluice.program(),
            herdr: self.herdr.program(),
        }
    }

    /// The test's `PATH` with the stubs first, so no real Flume, sluice or
    /// herdr runs.
    fn path(&self) -> OsString {
        let stubs = [&self.flume.dir, &self.sluice.dir, &self.herdr.dir]
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

/// `flywheel run --once` on the scratch host's default configuration
/// file, with `tools` first on its `PATH`.
fn once(scratch: &Scratch, tools: &Tools) -> Output {
    scratch
        .flywheel()
        .args(["run", "--once"])
        .env("PATH", tools.path())
        .output()
        .expect("flywheel runs")
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
            .any(|line| line.starts_with("flywheel: Flume's status could not be read: ")),
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
        .filter(|line| line.starts_with("flywheel: the control plane could not be read: "))
        .collect();
    assert_eq!(unread.len(), 1, "{said}");
    assert!(
        unread[0].ends_with(": nothing carried this pass"),
        "{}",
        unread[0]
    );
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

/// The control-plane-interface spec's "The hosted control plane named":
/// the runner names `hosted` as a control plane this binary does not
/// carry and names `crew`, runs no control pass, and keeps its
/// repositories. Its control pass did not pull, so `--once` exits 1.
#[test]
fn the_hosted_control_plane_named() {
    let scratch = Scratch::new();
    scratch.origin(KEPT);
    let file = scratch.configure(&format!(
        "{}[control_plane]\ntype = \"hosted\"\n",
        keeping(&[KEPT])
    ));
    let tools = Tools::new();

    let output = once(&scratch, &tools);

    assert_eq!(output.status.code(), Some(1), "{}", stderr(&output));
    let named = format!(
        "flywheel: {}: `type` is \"hosted\", a control plane this binary does not carry: \
         it carries `crew`: no control pass runs",
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

/// The fake control plane, telling the test when each pull is made.
struct Told {
    fake: Fake,
    pulls: Sender<Instant>,
}

impl ControlPlane for Told {
    fn pull(&mut self, hello: &Hello, report: &Report) -> Result<Pulled, Unread> {
        let _ = self.pulls.send(Instant::now());
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
            fake: Fake::new(),
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

/// The running spec's "flywheel run makes control passes on a timer of
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
        "flywheel: {}:2: `repositories` entry \"flume\": an address is `owner/name`: \
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
    let (_, said) = running.until("flywheel: stopped");
    running.thread.join().expect("the loop ends");
    assert_eq!(said, Vec::<String>::new());
}
