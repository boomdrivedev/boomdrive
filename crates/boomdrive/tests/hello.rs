//! The tools' versions as the hello names them (design Decision 6), read
//! from stub programs written into the test's own temporary directory:
//! the first line each printed, a tool not there named missing (the
//! work-into-flume spec's "A tool missing"), and no multiplexer's version
//! run (the work-into-flume spec's "No multiplexer's version"; core.29).

use boomdrive::hello::{self, Cli, Tools};
use boomdrive_control::{Capacity, Report, ToolVersion};
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};

/// A program at `dir/name` that runs `body`.
fn program(dir: &Path, name: &str, body: &str) -> PathBuf {
    let file = dir.join(name);
    fs::write(&file, format!("#!/bin/sh\n{body}\n")).unwrap();
    fs::set_permissions(&file, fs::Permissions::from_mode(0o755)).unwrap();
    file
}

#[test]
fn each_tools_first_line_and_one_not_there() {
    let dir = tempfile::tempdir().unwrap();
    let tools = Cli::on_path()
        .with(
            "flume",
            program(
                dir.path(),
                "flume",
                "printf 'flume 0.4.1\\nbuilt 78cf178\\n'",
            ),
        )
        .with(
            "sluice",
            program(dir.path(), "sluice", "echo 'sluice 0.61.1'"),
        )
        .with("git", dir.path().join("git"));
    assert_eq!(
        tools.version("flume"),
        ToolVersion::Printed("flume 0.4.1".into())
    );
    assert_eq!(
        tools.version("sluice"),
        ToolVersion::Printed("sluice 0.61.1".into())
    );
    let ToolVersion::Missing(words) = tools.version("git") else {
        panic!("git is not there");
    };
    assert!(words.starts_with("git could not run: "), "{words}");
    let hello = hello::hello(
        &tools,
        vec!["agentplot/flume".into()],
        Capacity::Missing("no status".into()),
        &Report::default(),
    );
    assert!(matches!(
        hello.tool_versions["git"],
        ToolVersion::Missing(_)
    ));
    assert_eq!(
        hello.tool_versions["flume"],
        ToolVersion::Printed("flume 0.4.1".into())
    );
    assert_eq!(
        hello.tool_versions["sluice"],
        ToolVersion::Printed("sluice 0.61.1".into())
    );
    assert_eq!(
        hello.runner_version,
        format!("boomdrive {}", env!("CARGO_PKG_VERSION"))
    );
    assert_eq!(
        hello.interface_version,
        boomdrive_control::INTERFACE_VERSION
    );
}

#[test]
fn a_tool_that_fails_or_prints_nothing_is_missing() {
    let dir = tempfile::tempdir().unwrap();
    let tools = Cli::on_path()
        .with(
            "flume",
            program(dir.path(), "flume", "echo 'no such flag' >&2; exit 2"),
        )
        .with("sluice", program(dir.path(), "sluice", "true"));
    let ToolVersion::Missing(words) = tools.version("flume") else {
        panic!("flume failed");
    };
    assert!(words.ends_with(": no such flag"), "{words}");
    assert_eq!(
        tools.version("sluice"),
        ToolVersion::Missing("sluice --version printed nothing".into())
    );
}

/// The work-into-flume spec's "No multiplexer's version": a hello made
/// with programs given for `flume`, `sluice`, `git` and `herdr` names the
/// first three alone, and the program given for `herdr` never runs.
#[test]
fn the_hello_runs_no_multiplexer() {
    let dir = tempfile::tempdir().unwrap();
    let ran = dir.path().join("herdr-ran");
    let tools = Cli::on_path()
        .with("flume", program(dir.path(), "flume", "echo 'flume 0.24.0'"))
        .with(
            "sluice",
            program(dir.path(), "sluice", "echo 'sluice 0.61.1'"),
        )
        .with(
            "git",
            program(dir.path(), "git", "echo 'git version 2.48.1'"),
        )
        .with(
            "herdr",
            program(
                dir.path(),
                "herdr",
                &format!("touch '{}'; echo 'herdr 0.9.5'", ran.display()),
            ),
        );
    let hello = hello::hello(
        &tools,
        vec!["agentplot/flume".into()],
        Capacity::Missing("no status".into()),
        &Report::default(),
    );
    assert_eq!(
        hello.tool_versions.keys().collect::<Vec<_>>(),
        ["flume", "git", "sluice"]
    );
    assert!(!ran.exists(), "the hello ran herdr");
}
