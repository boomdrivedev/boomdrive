//! The end-to-end set of `boomdrive` as a host runs it: its version and
//! usage, `run --once`'s main path, `run` until it is stopped, a stop with
//! a fetch in flight under each, and the start refused by the lock on the
//! data directory and by a file that cannot be read (the running spec;
//! design 9 and 10; logic-is-tested-as-units-and-few-tests-run-the-binary).
//! What the start reads and what a pass says are proven as units: the
//! configuration's files and the environment in `config`'s tests, and a
//! pass's lines in `clones` and `views`, in-process.

mod common;

use common::{keeping, Scratch};
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::path::Path;
use std::process::{Child, Output, Stdio};
use std::time::{Duration, Instant};

fn stdout(output: &Output) -> String {
    String::from_utf8_lossy(&output.stdout).into_owned()
}

fn stderr(output: &Output) -> String {
    String::from_utf8_lossy(&output.stderr).into_owned()
}

/// The line `boomdrive run` starts with, keeping `kept` from `file`.
fn started(scratch: &Scratch, kept: &str, file: &Path) -> String {
    format!(
        "boomdrive {}: keeping {kept} from {}; data {}; repository root {}",
        env!("CARGO_PKG_VERSION"),
        file.display(),
        scratch.data().display(),
        scratch.home.join("Code").display()
    )
}

#[test]
fn the_version() {
    let scratch = Scratch::new();
    for flag in ["--version", "-V"] {
        let output = scratch
            .boomdrive()
            .arg(flag)
            .output()
            .expect("boomdrive runs");
        assert_eq!(output.status.code(), Some(0), "{flag}");
        assert_eq!(
            stdout(&output),
            format!("boomdrive {}\n", env!("CARGO_PKG_VERSION")),
            "{flag}"
        );
    }
}

#[test]
fn no_command() {
    let scratch = Scratch::new();
    for args in [&[][..], &["keep"], &["run", "--every"]] {
        let output = scratch
            .boomdrive()
            .args(args)
            .output()
            .expect("boomdrive runs");
        assert_eq!(output.status.code(), Some(2), "{args:?}");
        assert_eq!(stdout(&output), "", "{args:?}");
        assert!(
            stderr(&output).contains("Usage: boomdrive"),
            "{args:?}: {}",
            stderr(&output)
        );
    }
}

#[test]
fn every_repository_kept() {
    let scratch = Scratch::new();
    let flume = scratch.origin("agentplot/flume");
    let sluice = scratch.origin("agentplot/sluice");
    let file = scratch.configure(&keeping(&["agentplot/flume", "agentplot/sluice"]));

    let output = scratch
        .boomdrive()
        .args(["run", "--once"])
        .output()
        .expect("boomdrive runs");

    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    let views = scratch.home.join("Code");
    assert_eq!(
        stderr(&output),
        format!(
            "{}\n\
             agentplot/flume: cloned from https://github.com/agentplot/flume.git\n\
             agentplot/flume: view made at {}, at origin/main {}\n\
             agentplot/sluice: cloned from https://github.com/agentplot/sluice.git\n\
             agentplot/sluice: view made at {}, at origin/main {}\n",
            started(&scratch, "2 repositories", &file),
            views.join("agentplot/flume").display(),
            &flume.tip("main")[..7],
            views.join("agentplot/sluice").display(),
            &sluice.tip("main")[..7],
        )
    );
    for origin in [flume, sluice] {
        assert_eq!(
            scratch.git(&scratch.clone_of(&origin.address), &["rev-parse", "main"]),
            origin.tip("main")
        );
        assert_eq!(
            scratch.git(&views.join(&origin.address), &["rev-parse", "HEAD"]),
            origin.tip("main")
        );
    }
}

/// `boomdrive run` running, killed where a test ends before it stops.
struct Running(Child);

impl Drop for Running {
    fn drop(&mut self) {
        let _ = self.0.kill();
        let _ = self.0.wait();
    }
}

#[test]
fn stopped_while_waiting() {
    let scratch = Scratch::new();
    let flume = scratch.origin("agentplot/flume");
    let file = scratch.configure(&keeping(&["agentplot/flume"]));
    let mut running = Running(
        scratch
            .boomdrive()
            .arg("run")
            .stderr(Stdio::piped())
            .spawn()
            .expect("boomdrive runs"),
    );
    let mut said = BufReader::new(running.0.stderr.take().expect("its standard error"));
    let first: Vec<String> = said
        .by_ref()
        .lines()
        .take(3)
        .map(|line| line.expect("a line"))
        .collect();
    assert_eq!(
        first,
        [
            started(&scratch, "1 repository", &file),
            "agentplot/flume: cloned from https://github.com/agentplot/flume.git".to_string(),
            format!(
                "agentplot/flume: view made at {}, at origin/main {}",
                scratch.home.join("Code/agentplot/flume").display(),
                &flume.tip("main")[..7]
            ),
        ]
    );

    let pid = libc::pid_t::try_from(running.0.id()).expect("a process id");
    // SAFETY: kill(2) takes plain integers and touches no memory of ours.
    assert_eq!(unsafe { libc::kill(pid, libc::SIGTERM) }, 0);

    let deadline = Instant::now() + Duration::from_secs(1);
    let status = loop {
        if let Some(status) = running.0.try_wait().expect("its status") {
            break status;
        }
        assert!(Instant::now() < deadline, "not stopped within a second");
        std::thread::sleep(Duration::from_millis(10));
    };
    assert_eq!(status.code(), Some(0));
    let mut rest = String::new();
    said.read_to_string(&mut rest).expect("its standard error");
    assert_eq!(rest, "boomdrive: stopped\n");
}

/// Waits up to `within` for `running` to exit, and gives its exit, or
/// fails naming `within`.
fn exits_within(running: &mut Running, within: Duration) -> std::process::ExitStatus {
    let deadline = Instant::now() + within;
    loop {
        if let Some(status) = running.0.try_wait().expect("its status") {
            return status;
        }
        assert!(Instant::now() < deadline, "not stopped within {within:?}");
        std::thread::sleep(Duration::from_millis(10));
    }
}

/// Sends `signal` to `running`.
fn signal(running: &Running, signal: libc::c_int) {
    let pid = libc::pid_t::try_from(running.0.id()).expect("a process id");
    // SAFETY: kill(2) takes plain integers and touches no memory of ours.
    assert_eq!(unsafe { libc::kill(pid, signal) }, 0);
}

/// The running spec's "Stopped with a fetch in flight", for `boomdrive
/// run`, and the clones spec's "A new clone halted in flight": the first
/// fetch of a repository whose origin never answers is ended at the stop,
/// well before its limit, the repository named not cloned with the halted
/// words, nothing of it left, and nothing the fetch started outliving the
/// runner (record 0042 as amended on 2026-11-12; core.58).
#[test]
fn stopped_with_a_fetch_in_flight() {
    let scratch = Scratch::new();
    let origin = scratch.silent("agentplot/flume");
    let file = scratch.configure(&keeping(&["agentplot/flume"]));
    let mut running = Running(
        scratch
            .boomdrive()
            .arg("run")
            .stderr(Stdio::piped())
            .spawn()
            .expect("boomdrive runs"),
    );
    let mut said = BufReader::new(running.0.stderr.take().expect("its standard error"));
    let mut first = String::new();
    said.read_line(&mut first).expect("its start line");
    assert_eq!(
        first,
        format!("{}\n", started(&scratch, "1 repository", &file))
    );
    assert!(
        origin.taken(Duration::from_secs(10)),
        "the fetch never reached its origin"
    );

    signal(&running, libc::SIGTERM);
    let status = exits_within(&mut running, Duration::from_secs(5));
    assert_eq!(status.code(), Some(0));
    let mut rest = String::new();
    said.read_to_string(&mut rest).expect("its standard error");
    assert_eq!(
        rest,
        "agentplot/flume: not cloned: git fetch with origin was stopped as its caller asked\n\
         boomdrive: stopped\n"
    );
    assert!(!scratch.container("agentplot/flume").exists());
    assert!(
        origin.closed(Duration::from_secs(1)),
        "the fetch's connection outlived the runner"
    );
}

/// The running spec's "run --once stopped with a fetch in flight":
/// SIGINT ends the first fetch in flight of a repository
/// whose origin never answers, the repository named not cloned with the
/// halted words, nothing of it left, no `flume` run, and nothing the fetch
/// started outliving the runner, which exits 1 (record 0042 as amended on
/// 2026-11-12; core.39, core.58).
#[test]
fn once_stopped_with_a_fetch_in_flight() {
    let scratch = Scratch::new();
    let calls = scratch.root.join("flume-calls");
    let flume = scratch.bin.join("flume");
    std::fs::write(
        &flume,
        format!("#!/bin/sh\necho \"$*\" >> '{}'\nexit 1\n", calls.display()),
    )
    .expect("the stub flume");
    let origin = scratch.silent("agentplot/flume");
    let file = scratch.configure(&keeping(&["agentplot/flume"]));
    let mut running = Running(
        scratch
            .boomdrive()
            .args(["run", "--once"])
            .stderr(Stdio::piped())
            .spawn()
            .expect("boomdrive runs"),
    );
    let mut said = BufReader::new(running.0.stderr.take().expect("its standard error"));
    let mut first = String::new();
    said.read_line(&mut first).expect("its start line");
    assert_eq!(
        first,
        format!("{}\n", started(&scratch, "1 repository", &file))
    );
    assert!(
        origin.taken(Duration::from_secs(10)),
        "the fetch never reached its origin"
    );

    signal(&running, libc::SIGINT);
    let status = exits_within(&mut running, Duration::from_secs(5));
    assert_eq!(status.code(), Some(1));
    let mut rest = String::new();
    said.read_to_string(&mut rest).expect("its standard error");
    assert_eq!(
        rest,
        "agentplot/flume: not cloned: git fetch with origin was stopped as its caller asked\n\
         boomdrive: stopped\n"
    );
    assert!(!scratch.container("agentplot/flume").exists());
    assert!(!calls.exists(), "flume was run");
    assert!(
        origin.closed(Duration::from_secs(1)),
        "the fetch's connection outlived the runner"
    );
}

#[test]
fn a_second_runner() {
    let scratch = Scratch::new();
    scratch.origin("agentplot/flume");
    scratch.configure(&keeping(&["agentplot/flume"]));
    std::fs::create_dir_all(scratch.data()).expect("the data directory");
    let running = File::create(scratch.data().join("lock")).expect("the lock");
    running.lock().expect("the lock held");

    let output = scratch
        .boomdrive()
        .args(["run", "--once"])
        .output()
        .expect("boomdrive runs");

    assert_eq!(output.status.code(), Some(2), "{}", stderr(&output));
    assert_eq!(
        stderr(&output),
        format!(
            "boomdrive: the data directory {} is held by a running runner\n",
            scratch.data().display()
        )
    );
    assert!(
        !scratch.data().join("repositories").exists(),
        "a second runner touched the data directory"
    );
}

#[test]
fn no_file() {
    let scratch = Scratch::new();
    scratch.origin("agentplot/flume");

    let output = scratch
        .boomdrive()
        .args(["run"])
        .output()
        .expect("boomdrive runs");

    assert_eq!(output.status.code(), Some(2), "{}", stderr(&output));
    let said = stderr(&output);
    assert!(
        said.starts_with(&format!("boomdrive: {}: ", scratch.config_file().display())),
        "{said}"
    );
    assert_eq!(said.lines().count(), 1, "{said}");
    assert!(!scratch.data().exists(), "a missing file was kept");
}
