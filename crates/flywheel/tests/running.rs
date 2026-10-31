//! `flywheel` as a host runs it: its version and usage, `run --once`,
//! the configuration file it reads, and the start's checks, the lock on
//! the data directory among them (the running and configuration specs;
//! design 10).

mod common;

use common::{keeping, Scratch};
use std::fs::File;
use std::path::Path;
use std::process::Output;

fn stdout(output: &Output) -> String {
    String::from_utf8_lossy(&output.stdout).into_owned()
}

fn stderr(output: &Output) -> String {
    String::from_utf8_lossy(&output.stderr).into_owned()
}

/// The line `flywheel run` starts with, keeping `kept` from `file`.
fn started(scratch: &Scratch, kept: &str, file: &Path) -> String {
    format!(
        "flywheel {}: keeping {kept} from {}; data {}; views {}",
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
            .flywheel()
            .arg(flag)
            .output()
            .expect("flywheel runs");
        assert_eq!(output.status.code(), Some(0), "{flag}");
        assert_eq!(
            stdout(&output),
            format!("flywheel {}\n", env!("CARGO_PKG_VERSION")),
            "{flag}"
        );
    }
}

#[test]
fn no_command() {
    let scratch = Scratch::new();
    for args in [&[][..], &["keep"], &["run", "--every"]] {
        let output = scratch
            .flywheel()
            .args(args)
            .output()
            .expect("flywheel runs");
        assert_eq!(output.status.code(), Some(2), "{args:?}");
        assert_eq!(stdout(&output), "", "{args:?}");
        assert!(
            stderr(&output).contains("Usage: flywheel"),
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
        .flywheel()
        .args(["run", "--once"])
        .output()
        .expect("flywheel runs");

    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    assert_eq!(
        stderr(&output),
        format!(
            "{}\n\
             agentplot/flume: cloned from https://github.com/agentplot/flume.git\n\
             agentplot/sluice: cloned from https://github.com/agentplot/sluice.git\n",
            started(&scratch, "2 repositories", &file)
        )
    );
    for origin in [flume, sluice] {
        assert_eq!(
            scratch.git(&scratch.clone_of(&origin.address), &["rev-parse", "main"]),
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
        .flywheel()
        .args(["run", "--once"])
        .output()
        .expect("flywheel runs");

    assert_eq!(output.status.code(), Some(1), "{}", stderr(&output));
    let said = stderr(&output);
    let lines: Vec<&str> = said.lines().collect();
    assert_eq!(lines.len(), 3, "{said}");
    assert!(
        lines[1].starts_with("agentplot/flume: not cloned: "),
        "{said}"
    );
    assert_eq!(
        lines[2],
        "agentplot/sluice: cloned from https://github.com/agentplot/sluice.git"
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

#[test]
fn a_second_runner() {
    let scratch = Scratch::new();
    scratch.origin("agentplot/flume");
    scratch.configure(&keeping(&["agentplot/flume"]));
    std::fs::create_dir_all(scratch.data()).expect("the data directory");
    let running = File::create(scratch.data().join("lock")).expect("the lock");
    running.lock().expect("the lock held");

    let output = scratch
        .flywheel()
        .args(["run", "--once"])
        .output()
        .expect("flywheel runs");

    assert_eq!(output.status.code(), Some(2), "{}", stderr(&output));
    assert_eq!(
        stderr(&output),
        format!(
            "flywheel: the data directory {} is held by a running runner\n",
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
        .flywheel()
        .args(["run", "--once"])
        .output()
        .expect("flywheel runs");

    assert_eq!(output.status.code(), Some(2), "{}", stderr(&output));
    assert_eq!(
        stderr(&output),
        format!(
            "flywheel: {}:1: `repositories` entry \"flume\": an address is `owner/name`\n",
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
        .flywheel()
        .args(["run", "--once"])
        .env_remove("HOME")
        .output()
        .expect("flywheel runs");

    assert_eq!(output.status.code(), Some(2), "{}", stderr(&output));
    assert_eq!(stderr(&output), "flywheel: HOME is not set\n");
}

#[test]
fn the_default_file() {
    let scratch = Scratch::new();
    scratch.origin("agentplot/flume");
    let file = scratch.configure(&keeping(&["agentplot/flume"]));
    assert_eq!(file, scratch.home.join(".config/flywheel/config.toml"));

    let output = scratch
        .flywheel()
        .args(["run", "--once"])
        .env_remove("XDG_CONFIG_HOME")
        .output()
        .expect("flywheel runs");

    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    assert_eq!(
        stderr(&output).lines().next(),
        Some(started(&scratch, "1 repository", &file).as_str())
    );
    assert!(scratch.clone_of("agentplot/flume").exists());
}

#[test]
fn the_file_named() {
    let scratch = Scratch::new();
    scratch.origin("agentplot/flume");
    scratch.configure("repositories = [\"flume\"]\n");
    let named = scratch.root.join("flywheel.toml");
    std::fs::write(&named, keeping(&["agentplot/flume"])).expect("the file named");

    let output = scratch
        .flywheel()
        .args(["run", "--once", "--config"])
        .arg(&named)
        .output()
        .expect("flywheel runs");

    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    assert_eq!(
        stderr(&output).lines().next(),
        Some(started(&scratch, "1 repository", &named).as_str())
    );
    assert!(scratch.clone_of("agentplot/flume").exists());
}

#[test]
fn no_file() {
    let scratch = Scratch::new();
    scratch.origin("agentplot/flume");

    let output = scratch
        .flywheel()
        .args(["run"])
        .output()
        .expect("flywheel runs");

    assert_eq!(output.status.code(), Some(2), "{}", stderr(&output));
    let said = stderr(&output);
    assert!(
        said.starts_with(&format!("flywheel: {}: ", scratch.config_file().display())),
        "{said}"
    );
    assert_eq!(said.lines().count(), 1, "{said}");
    assert!(!scratch.data().exists(), "a missing file was kept");
}
