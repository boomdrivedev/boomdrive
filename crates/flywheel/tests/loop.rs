//! The library's loop, run in a thread on the scratch host: a landing it
//! is told of between passes, a failure that does not stop it, and the
//! configuration read again at every pass (the running spec, the clones
//! spec's landings and the configuration spec's reading while running;
//! design 10).

mod common;

use common::{keeping, Scratch};
use flywheel::config::Address;
use flywheel::run::{Notices, Runner, Stopper};
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
}

impl Running {
    /// Starts the runner on the scratch host's default configuration
    /// file, and its loop.
    fn start(scratch: &Scratch) -> Running {
        let overlay = scratch.overlay();
        let var = |name: &str| {
            overlay
                .iter()
                .find(|(key, _)| key.as_os_str() == OsStr::new(name))
                .map(|(_, value)| value.clone())
        };
        let mut runner = Runner::start(None, var, scratch.runner_git()).expect("the runner starts");
        let notices = runner.notices();
        let stopper = runner.stopper();
        let (said, lines) = mpsc::channel();
        let thread = thread::spawn(move || {
            runner.keep(&mut |line| {
                let _ = said.send(line);
            })
        });
        Running {
            notices,
            stopper,
            lines,
            thread,
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

/// Writes `text` as the configuration by one rename, so the running
/// runner never reads it half written.
fn rewrite(scratch: &Scratch, text: &str) {
    let file = scratch.config_file();
    let next = file.with_extension("next");
    std::fs::write(&next, text).expect("the configuration written");
    std::fs::rename(&next, &file).expect("the configuration in place");
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

/// The lines said when origin's `main` of `address` moved to `commit`,
/// the clone's own and the view following.
fn moved(address: &str, commit: &str) -> [String; 2] {
    [
        format!("{address}: main moved to origin/main at {}", &commit[..7]),
        format!("{address}: view moved to origin/main at {}", &commit[..7]),
    ]
}

/// The commit origin's `main` stood at when `address`'s clone was last
/// fetched.
fn fetched(scratch: &Scratch, address: &str) -> String {
    scratch.git(
        &scratch.clone_of(address),
        &["rev-parse", "refs/remotes/origin/main"],
    )
}

#[test]
fn told_between_passes() {
    let scratch = Scratch::new();
    let flume = scratch.origin("agentplot/flume");
    let sluice = scratch.origin("agentplot/sluice");
    scratch.configure(&every("1h", &["agentplot/flume", "agentplot/sluice"]));
    let running = Running::start(&scratch);
    let [.., made] = cloned(&scratch, "agentplot/sluice", &sluice.tip("main"));
    let first = running.until(&made);
    assert_eq!(first.len(), 4, "{first:#?}");
    let sluice_was = sluice.tip("main");

    let landed = flume.commit("main", "landed", "a landing\n");
    sluice.commit("main", "unlanded", "not told of\n");
    running.notices.landed(address("agentplot/flume"));

    let lines = moved("agentplot/flume", &landed);
    assert_eq!(running.until(&lines[1]), lines);
    assert_eq!(running.stop(), ["flywheel: stopped"]);
    assert_eq!(
        scratch.git(
            &scratch.home.join("Code/agentplot/flume"),
            &["rev-parse", "HEAD"]
        ),
        landed
    );
    assert_eq!(fetched(&scratch, "agentplot/sluice"), sluice_was);
}

#[test]
fn told_of_a_repository_it_does_not_keep() {
    let scratch = Scratch::new();
    let flume = scratch.origin("agentplot/flume");
    scratch.origin("someone/else");
    scratch.configure(&every("1h", &["agentplot/flume"]));
    let running = Running::start(&scratch);
    running.until(&cloned(&scratch, "agentplot/flume", &flume.tip("main"))[1]);
    let flume_was = flume.tip("main");
    flume.commit("main", "later", "not told of\n");

    running.notices.landed(address("someone/else"));

    let named = "someone/else: not kept: the configuration does not name it";
    assert_eq!(running.until(named), [named]);
    assert_eq!(running.stop(), ["flywheel: stopped"]);
    assert!(!scratch.container("someone/else").exists());
    assert!(!scratch.home.join("Code/someone").exists());
    assert_eq!(fetched(&scratch, "agentplot/flume"), flume_was);
}

#[test]
fn a_failure_does_not_stop_it() {
    let scratch = Scratch::new();
    let flume = scratch.origin("agentplot/flume");
    let sluice = scratch.origin("agentplot/sluice");
    scratch.configure(&every("1s", &["agentplot/flume", "agentplot/sluice"]));
    let running = Running::start(&scratch);
    running.until(&cloned(&scratch, "agentplot/sluice", &sluice.tip("main"))[1]);

    let away = flume.bare.with_extension("away");
    std::fs::rename(&flume.bare, &away).expect("the forge's flume taken away");
    let kept = sluice.commit("main", "later", "kept while flume fails\n");
    let lines = moved("agentplot/sluice", &kept);
    let said = running.until(&lines[1]);
    let (failed, rest) = said.split_at(said.len() - 2);
    assert_eq!(rest, lines);
    assert!(!failed.is_empty(), "{said:#?}");
    for line in failed {
        assert!(
            line.starts_with("agentplot/flume: not fetched: "),
            "{said:#?}"
        );
    }

    std::fs::rename(&away, &flume.bare).expect("the forge's flume back");
    let again = flume.commit("main", "again", "fetched again\n");
    let lines = moved("agentplot/flume", &again);
    let said = running.until(&lines[1]);
    let (failed, rest) = said.split_at(said.len() - 2);
    assert_eq!(rest, lines);
    for line in failed {
        assert!(
            line.starts_with("agentplot/flume: not fetched: "),
            "{said:#?}"
        );
    }
    assert_eq!(
        running.stop().last().map(String::as_str),
        Some("flywheel: stopped")
    );
}

#[test]
fn a_repository_added_while_running() {
    let scratch = Scratch::new();
    let flume = scratch.origin("agentplot/flume");
    let sluice = scratch.origin("agentplot/sluice");
    scratch.configure(&every("1s", &["agentplot/flume"]));
    let running = Running::start(&scratch);
    running.until(&cloned(&scratch, "agentplot/flume", &flume.tip("main"))[1]);

    rewrite(
        &scratch,
        &every("1s", &["agentplot/flume", "agentplot/sluice"]),
    );

    let lines = cloned(&scratch, "agentplot/sluice", &sluice.tip("main"));
    assert_eq!(running.until(&lines[1]), lines);
    assert_eq!(
        running.stop().last().map(String::as_str),
        Some("flywheel: stopped")
    );
    assert!(scratch.home.join("Code/agentplot/sluice").exists());
}

#[test]
fn a_file_broken_while_running() {
    let scratch = Scratch::new();
    let flume = scratch.origin("agentplot/flume");
    scratch.configure(&every("1s", &["agentplot/flume"]));
    let running = Running::start(&scratch);
    running.until(&cloned(&scratch, "agentplot/flume", &flume.tip("main"))[1]);
    let flume_was = flume.tip("main");

    rewrite(
        &scratch,
        "fetch_interval = \"1s\"\nrepositories = [\"flume\"]\n",
    );
    let broken = format!(
        "flywheel: {}:2: `repositories` entry \"flume\": an address is `owner/name`: nothing kept this pass",
        scratch.config_file().display()
    );
    assert_eq!(running.until(&broken), [broken.as_str()]);
    let later = flume.commit("main", "later", "while the file is broken\n");
    assert_eq!(running.until(&broken), [broken.as_str()]);
    let view = scratch.home.join("Code/agentplot/flume");
    assert_eq!(fetched(&scratch, "agentplot/flume"), flume_was);
    assert_eq!(scratch.git(&view, &["rev-parse", "HEAD"]), flume_was);

    rewrite(&scratch, &every("1s", &["agentplot/flume"]));
    let lines = moved("agentplot/flume", &later);
    let said = running.until(&lines[1]);
    let (passed, rest) = said.split_at(said.len() - 2);
    assert_eq!(rest, lines);
    assert!(passed.iter().all(|line| *line == broken), "{said:#?}");
    assert_eq!(
        running.stop().last().map(String::as_str),
        Some("flywheel: stopped")
    );
}
