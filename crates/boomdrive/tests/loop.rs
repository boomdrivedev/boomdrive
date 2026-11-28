//! The library's loop, run in a thread on the scratch host, told of a
//! landing during a keeping pass: the host's own keeping pass hears it
//! between two repositories over real git (the clones spec's "Told during
//! a keeping pass"; design 10). The rest of the loop's logic, its
//! landings, failures, readings of the configuration and ticks, is proven
//! on a paused clock with no process in `keeping.rs`
//! (logic-is-tested-as-units-and-few-tests-run-the-binary). The runner
//! runs a stub `flume` and a stub `millpond` of the test's, and no Flume
//! or millpond of the host's.

mod common;
mod fakes;

use boomdrive::config::Address;
use boomdrive::run::{Notices, Programs, Runner, Stopper};
use common::{keeping, Scratch};
use fakes::{HeldFetch, Stub, StubHost};
use std::ffi::OsStr;
use std::sync::mpsc::{self, Receiver};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

/// How long a test waits for a line it expects.
const PATIENCE: Duration = Duration::from_secs(20);

/// The runner's loop, running in a thread of the test's.
struct Running {
    notices: Notices,
    stopper: Stopper,
    lines: Receiver<String>,
    thread: JoinHandle<()>,
    /// The stub `flume` the runner runs, kept while it runs.
    _flume: Stub,
    /// The stub `millpond` the runner runs, kept while it runs.
    _millpond: Stub,
    /// A stub flume host, which a runner that carries no part never asks.
    _host: StubHost,
    /// The edge's runtime, dropped after the runner's thread.
    _runtime: tokio::runtime::Runtime,
}

impl Running {
    /// Starts the runner on the scratch host's default configuration
    /// file, running a stub `flume`, and its loop. Its sluice is a stub
    /// that is not there: a runner with no control plane runs none.
    fn start(scratch: &Scratch) -> Running {
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
        let flume = Stub::new();
        let millpond = Stub::new();
        let host = StubHost::new(flume.dir.path());
        runner.flume_socket(host.socket.clone());
        runner.tools(Programs {
            flume: flume.program(),
            sluice: scratch.root.join("absent/sluice"),
            millpond: millpond.program(),
        });
        let notices = runner.notices();
        let stopper = runner.stopper();
        let (said, lines) = mpsc::channel();
        let thread = thread::spawn(move || {
            runner.keep(&|line| {
                let _ = said.send(line);
            })
        });
        Running {
            notices,
            stopper,
            lines,
            thread,
            _flume: flume,
            _millpond: millpond,
            _host: host,
            _runtime: runtime,
        }
    }

    /// The lines said from the last read until `line`, it the last;
    /// panics where it is not said in time.
    fn until(&self, line: &str) -> Vec<String> {
        let deadline = Instant::now() + PATIENCE;
        let mut said = Vec::new();
        while let Ok(next) = self
            .lines
            .recv_timeout(deadline.saturating_duration_since(Instant::now()))
        {
            let done = next == line;
            said.push(next);
            if done {
                return said;
            }
        }
        panic!("{line:?} not said in time; said: {said:#?}");
    }

    /// Stops the runner, and gives the lines said from the last read, its
    /// stop the last.
    fn stop(self) -> Vec<String> {
        self.stopper.stop();
        self.thread.join().expect("the loop ends");
        self.lines.try_iter().collect()
    }
}

/// A configuration keeping `addresses`, fetched each `interval`.
fn every(interval: &str, addresses: &[&str]) -> String {
    format!("fetch_interval = {interval:?}\n{}", keeping(addresses))
}

fn address(text: &str) -> Address {
    Address::parse(text).expect("an address")
}

/// The lines a pass says when it clones `address`, at `commit`.
fn cloned(scratch: &Scratch, address: &str, commit: &str) -> [String; 2] {
    [
        format!("{address}: cloned from https://github.com/{address}.git"),
        format!(
            "{address}: view made at {}, at origin/main {}",
            scratch.home.join("Code").join(address).display(),
            &commit[..7]
        ),
    ]
}

/// The lines said when origin's `main` of `address` moved to `commit`:
/// the view following, and nothing of the clone's own `main`, which the
/// runner never moves (core.14).
fn moved(address: &str, commit: &str) -> [String; 1] {
    [format!(
        "{address}: view moved to origin/main at {}",
        &commit[..7]
    )]
}

/// The commit origin's `main` stood at when `address`'s clone was last
/// fetched.
fn fetched(scratch: &Scratch, address: &str) -> String {
    scratch.git(
        &scratch.clone_of(address),
        &["rev-parse", "refs/remotes/origin/main"],
    )
}

/// The clones spec's "Told during a keeping pass": told of a landing on
/// `agentplot/timber` while the pass keeps `agentplot/flume`, its fetch
/// held, the runner keeps the landing after `agentplot/flume` and before
/// `agentplot/sluice`.
#[test]
fn told_during_a_keeping_pass() {
    let scratch = Scratch::new();
    scratch.origin("agentplot/flume");
    let sluice = scratch.origin("agentplot/sluice");
    let timber = scratch.origin("agentplot/timber");
    let kept = ["agentplot/flume", "agentplot/sluice", "agentplot/timber"];
    scratch.configure(&every("4s", &kept));
    let held = HeldFetch::new(&scratch.gitconfig, "agentplot/flume");
    let running = Running::start(&scratch);
    let [.., made] = cloned(&scratch, "agentplot/timber", &timber.tip("main"));
    running.until(&made);

    held.hold();
    let sluice_moved = sluice.commit("main", "later", "kept by the pass\n");
    let timber_landed = timber.commit("main", "landed", "a landing\n");
    held.wait_held(PATIENCE);
    running.notices.landed(address("agentplot/timber"));
    held.let_go();

    let [timber_line] = moved("agentplot/timber", &timber_landed);
    let [sluice_line] = moved("agentplot/sluice", &sluice_moved);
    assert_eq!(
        running.until(&sluice_line),
        [timber_line, sluice_line.clone()]
    );
    running.stop();
    assert_eq!(fetched(&scratch, "agentplot/timber"), timber_landed);
}
