//! The local layer, `config.local.toml`, read over the managed one at
//! every keeping pass, on the scratch host (the configuration spec's "How
//! the local layer was read is said once each way" and "While the local
//! layer is not taken, nothing is let go"; design Decisions 5 and 7): its
//! line said once each way, by `run --once` and by a running runner's
//! loop, and nothing let go while a local file stands that is not taken.

mod common;

use boomdrive::config;
use boomdrive::run::{Runner, Stopper};
use common::{keeping, Scratch};
use std::ffi::OsStr;
use std::path::Path;
use std::sync::mpsc::{self, Receiver};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

/// How long a test waits for a line it expects.
const PATIENCE: Duration = Duration::from_secs(20);

/// The runner's loop, running in a thread of the test's.
struct Running {
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
        let stopper = runner.stopper();
        let (said, lines) = mpsc::channel();
        let thread = thread::spawn(move || {
            runner.keep(&mut |line| {
                let _ = said.send(line);
            })
        });
        Running {
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

/// A configuration keeping `addresses`, fetched each second, with `top`
/// above it.
fn every_second(top: &str, addresses: &[&str]) -> String {
    format!("{top}fetch_interval = \"1s\"\n{}", keeping(addresses))
}

/// Writes `text` at `file` by one rename, so the running runner never
/// reads it half written.
fn rewrite(file: &Path, text: &str) {
    let next = file.with_extension("next");
    std::fs::write(&next, text).expect("the file written");
    std::fs::rename(&next, file).expect("the file in place");
}

/// The line a pass says of the local layer `scratch` keeps, taken.
fn taken(scratch: &Scratch) -> String {
    format!("boomdrive: {}: taken", scratch.local_file().display())
}

/// The line a pass says of the local layer `scratch` keeps, not taken for
/// `why`.
fn not_taken(scratch: &Scratch, why: &str) -> String {
    format!(
        "boomdrive: {}: not taken, the managed layer read alone and nothing let go: {why}",
        scratch.local_file().display()
    )
}

/// The fault the runner names in the local layer `local` over the managed
/// layer `managed`, as the library reads them.
fn fault_of(scratch: &Scratch, managed: &str, local: &str) -> String {
    let config = config::parse_layers(
        managed,
        &scratch.config_file(),
        Some(local),
        &scratch.local_file(),
        &scratch.home,
    )
    .expect("a managed layer");
    match config.local {
        config::LocalLayer::NotTaken {
            fault: Some(fault), ..
        } => fault,
        other => panic!("not taken for a fault: {other:?}"),
    }
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

/// The line a pass says when origin's `main` of `address` moved to
/// `commit`.
fn moved(address: &str, commit: &str) -> String {
    format!("{address}: view moved to origin/main at {}", &commit[..7])
}

fn stderr(output: &std::process::Output) -> String {
    String::from_utf8_lossy(&output.stderr).into_owned()
}

/// The configuration spec's "Taken at the start": `run --once` with a
/// local layer keeping `agentplot/sluice` says one line naming
/// `config.local.toml` as taken, and keeps both layers' repositories.
#[test]
fn taken_at_the_start() {
    let scratch = Scratch::new();
    scratch.origin("agentplot/flume");
    scratch.origin("agentplot/sluice");
    scratch.configure(&keeping(&["agentplot/flume"]));
    scratch.configure_local(&keeping(&["agentplot/sluice"]));

    let output = scratch
        .boomdrive()
        .args(["run", "--once"])
        .output()
        .expect("boomdrive runs");

    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    let said = stderr(&output);
    let local: Vec<&str> = said
        .lines()
        .filter(|line| line.contains("config.local.toml"))
        .collect();
    assert_eq!(local, [taken(&scratch)], "{said}");
    assert_eq!(
        said.lines().nth(1),
        Some(taken(&scratch).as_str()),
        "{said}"
    );
    assert!(scratch.clone_of("agentplot/flume").exists());
    assert!(scratch.clone_of("agentplot/sluice").exists());
}

/// The configuration spec's "A fault said once": a local layer that is
/// not TOML for three passes is named once, and once mended is said taken
/// once.
#[test]
fn a_fault_said_once() {
    let scratch = Scratch::new();
    let flume = scratch.origin("agentplot/flume");
    let sluice = scratch.origin("agentplot/sluice");
    let managed = every_second("", &["agentplot/flume"]);
    scratch.configure(&managed);
    let broken = "repositories = [\n";
    scratch.configure_local(broken);
    let named = not_taken(&scratch, &fault_of(&scratch, &managed, broken));

    let running = Running::start(&scratch);
    let first = running.until(&cloned(&scratch, "agentplot/flume", &flume.tip("main"))[1]);
    assert_eq!(first[0], named, "{first:#?}");
    for pass in 0..3 {
        let later = flume.commit("main", &format!("later-{pass}"), "a later commit\n");
        assert_eq!(
            running.until(&moved("agentplot/flume", &later)),
            [moved("agentplot/flume", &later)]
        );
    }

    rewrite(&scratch.local_file(), &keeping(&["agentplot/sluice"]));
    assert_eq!(running.until(&taken(&scratch)), [taken(&scratch)]);
    let lines = cloned(&scratch, "agentplot/sluice", &sluice.tip("main"));
    assert_eq!(running.until(&lines[1]), lines);
    let later = flume.commit("main", "mended", "after the local layer is mended\n");
    assert_eq!(
        running.until(&moved("agentplot/flume", &later)),
        [moved("agentplot/flume", &later)]
    );
    assert_eq!(running.stop(), ["boomdrive: stopped"]);
}

/// A running runner whose pinned `repositories` the local layer sets says
/// them passed over once, at its first pass, and not again.
#[test]
fn pinned_repositories_passed_over_said_once() {
    let scratch = Scratch::new();
    let flume = scratch.origin("agentplot/flume");
    scratch.origin("agentplot/sluice");
    scratch.configure(&every_second(
        "pin = [\"repositories\"]\n",
        &["agentplot/flume"],
    ));
    scratch.configure_local(&keeping(&["agentplot/sluice"]));
    let passed_over = format!(
        "{}; `repositories` passed over, pinned by `repositories`",
        taken(&scratch)
    );

    let running = Running::start(&scratch);
    let first = running.until(&cloned(&scratch, "agentplot/flume", &flume.tip("main"))[1]);
    assert_eq!(
        first,
        [passed_over.clone(), first[1].clone(), first[2].clone()]
    );
    for pass in 0..2 {
        let later = flume.commit("main", &format!("later-{pass}"), "a later commit\n");
        assert_eq!(
            running.until(&moved("agentplot/flume", &later)),
            [moved("agentplot/flume", &later)]
        );
    }
    assert_eq!(running.stop(), ["boomdrive: stopped"]);
    assert!(!scratch.container("agentplot/sluice").exists());
}
