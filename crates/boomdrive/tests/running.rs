//! `boomdrive` as a host runs it: its version and usage, `run --once`,
//! `run` until it is stopped, the configuration file it reads, the
//! start's checks, the lock on the data directory among them, and what a
//! pass says on standard error (the running and configuration specs;
//! design 9 and 10).

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

#[test]
fn a_clone_failed() {
    let scratch = Scratch::new();
    let sluice = scratch.origin("agentplot/sluice");
    scratch.configure(&keeping(&["agentplot/flume", "agentplot/sluice"]));

    let output = scratch
        .boomdrive()
        .args(["run", "--once"])
        .output()
        .expect("boomdrive runs");

    assert_eq!(output.status.code(), Some(1), "{}", stderr(&output));
    let said = stderr(&output);
    let lines: Vec<&str> = said.lines().collect();
    assert_eq!(lines.len(), 4, "{said}");
    assert!(
        lines[1].starts_with("agentplot/flume: not cloned: "),
        "{said}"
    );
    assert_eq!(
        lines[2],
        "agentplot/sluice: cloned from https://github.com/agentplot/sluice.git"
    );
    assert!(
        lines[3].starts_with("agentplot/sluice: view made at "),
        "{said}"
    );
    assert!(!scratch.container("agentplot/flume").exists());
    assert_eq!(
        scratch.git(
            &scratch.clone_of("agentplot/sluice"),
            &["rev-parse", "main"]
        ),
        sluice.tip("main")
    );
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

#[test]
fn a_quiet_pass() {
    let scratch = Scratch::new();
    scratch.origin("agentplot/flume");
    scratch.origin("agentplot/sluice");
    let file = scratch.configure(&keeping(&["agentplot/flume", "agentplot/sluice"]));
    let first = scratch
        .boomdrive()
        .args(["run", "--once"])
        .output()
        .expect("boomdrive runs");
    assert_eq!(first.status.code(), Some(0), "{}", stderr(&first));

    let output = scratch
        .boomdrive()
        .args(["run", "--once"])
        .output()
        .expect("boomdrive runs");

    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    assert_eq!(
        stderr(&output),
        format!("{}\n", started(&scratch, "2 repositories", &file))
    );
}

#[test]
fn a_view_moved() {
    let scratch = Scratch::new();
    let flume = scratch.origin("agentplot/flume");
    let file = scratch.configure(&keeping(&["agentplot/flume"]));
    let first = scratch
        .boomdrive()
        .args(["run", "--once"])
        .output()
        .expect("boomdrive runs");
    assert_eq!(first.status.code(), Some(0), "{}", stderr(&first));
    let later = flume.commit("main", "later", "a later commit\n");

    let output = scratch
        .boomdrive()
        .args(["run", "--once"])
        .output()
        .expect("boomdrive runs");

    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    assert_eq!(
        stderr(&output),
        format!(
            "{}\n\
             agentplot/flume: view moved to origin/main at {}\n",
            started(&scratch, "1 repository", &file),
            &later[..7],
        )
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
fn a_bad_configuration_at_start() {
    let scratch = Scratch::new();
    scratch.origin("agentplot/flume");
    let file = scratch.configure("repositories = [\"agentplot/flume\", \"flume\"]\n");

    let output = scratch
        .boomdrive()
        .args(["run", "--once"])
        .output()
        .expect("boomdrive runs");

    assert_eq!(output.status.code(), Some(2), "{}", stderr(&output));
    assert_eq!(
        stderr(&output),
        format!(
            "boomdrive: {}:1: `repositories` entry \"flume\": an address is `owner/name`\n",
            file.display()
        )
    );
    assert!(!scratch.data().exists(), "a bad configuration was kept");
}

#[test]
fn no_home() {
    let scratch = Scratch::new();
    scratch.configure(&keeping(&[]));

    let output = scratch
        .boomdrive()
        .args(["run", "--once"])
        .env_remove("HOME")
        .output()
        .expect("boomdrive runs");

    assert_eq!(output.status.code(), Some(2), "{}", stderr(&output));
    assert_eq!(stderr(&output), "boomdrive: HOME is not set\n");
}

/// The configuration spec's "The default file": with no `--config` and
/// `XDG_CONFIG_HOME` unset, the managed layer is `config.toml` and the
/// local layer `config.local.toml`, both under `~/.config/boomdrive`.
#[test]
fn the_default_file() {
    let scratch = Scratch::new();
    scratch.origin("agentplot/flume");
    scratch.origin("agentplot/sluice");
    let file = scratch.configure(&keeping(&["agentplot/flume"]));
    assert_eq!(file, scratch.home.join(".config/boomdrive/config.toml"));
    let local = scratch.configure_local(&keeping(&["agentplot/sluice"]));
    assert_eq!(
        local,
        scratch.home.join(".config/boomdrive/config.local.toml")
    );

    let output = scratch
        .boomdrive()
        .args(["run", "--once"])
        .env_remove("XDG_CONFIG_HOME")
        .output()
        .expect("boomdrive runs");

    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    assert_eq!(
        stderr(&output).lines().next(),
        Some(started(&scratch, "2 repositories", &file).as_str())
    );
    assert!(scratch.clone_of("agentplot/flume").exists());
    assert!(scratch.clone_of("agentplot/sluice").exists());
}

/// The configuration spec's "The file named": the managed layer is the
/// file `--config` names, and the local layer is still the one in the
/// runner's own configuration directory, the default file there unread.
#[test]
fn the_file_named() {
    let scratch = Scratch::new();
    scratch.origin("agentplot/flume");
    scratch.origin("agentplot/sluice");
    scratch.configure("repositories = [\"flume\"]\n");
    scratch.configure_local(&keeping(&["agentplot/sluice"]));
    let named = scratch.root.join("boomdrive.toml");
    std::fs::write(&named, keeping(&["agentplot/flume"])).expect("the file named");

    let output = scratch
        .boomdrive()
        .args(["run", "--once", "--config"])
        .arg(&named)
        .output()
        .expect("boomdrive runs");

    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    assert_eq!(
        stderr(&output).lines().next(),
        Some(started(&scratch, "2 repositories", &named).as_str())
    );
    assert!(scratch.clone_of("agentplot/flume").exists());
    assert!(scratch.clone_of("agentplot/sluice").exists());
}

/// The configuration spec's "No local file": the managed layer's
/// repositories alone are kept, and nothing is said of a local layer.
#[test]
fn no_local_file() {
    let scratch = Scratch::new();
    let flume = scratch.origin("agentplot/flume");
    let file = scratch.configure(&keeping(&["agentplot/flume"]));
    assert!(!scratch.local_file().exists());

    let output = scratch
        .boomdrive()
        .args(["run", "--once"])
        .output()
        .expect("boomdrive runs");

    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    assert_eq!(
        stderr(&output),
        format!(
            "{}\n\
             agentplot/flume: cloned from https://github.com/agentplot/flume.git\n\
             agentplot/flume: view made at {}, at origin/main {}\n",
            started(&scratch, "1 repository", &file),
            scratch.home.join("Code/agentplot/flume").display(),
            &flume.tip("main")[..7],
        )
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
