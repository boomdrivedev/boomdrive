//! The tools' versions as the hello names them (design Decision 6), read
//! from stub programs written into the test's own temporary directory:
//! the first line each printed, and a tool not there named missing (the
//! work-into-flume spec's "A tool missing").

use flywheel::hello::{self, Cli, Tools};
use flywheel_control::{Capacity, Report, ToolVersion};
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
        .with(
            "git",
            program(dir.path(), "git", "echo 'git version 2.48.1'"),
        )
        .with("herdr", dir.path().join("herdr"));
    assert_eq!(
        tools.version("flume"),
        ToolVersion::Printed("flume 0.4.1".into())
    );
    assert_eq!(
        tools.version("sluice"),
        ToolVersion::Printed("sluice 0.61.1".into())
    );
    let ToolVersion::Missing(words) = tools.version("herdr") else {
        panic!("herdr is not there");
    };
    assert!(words.starts_with("herdr could not run: "), "{words}");
    let hello = hello::hello(
        &tools,
        vec!["agentplot/flume".into()],
        Capacity::Missing("no status".into()),
        &Report::default(),
    );
    assert_eq!(
        hello.tool_versions["git"],
        ToolVersion::Printed("git version 2.48.1".into())
    );
    assert!(matches!(
        hello.tool_versions["herdr"],
        ToolVersion::Missing(_)
    ));
    assert_eq!(
        hello.runner_version,
        format!("flywheel {}", env!("CARGO_PKG_VERSION"))
    );
    assert_eq!(hello.interface_version, flywheel_control::INTERFACE_VERSION);
}

#[test]
fn a_tool_that_fails_or_prints_nothing_is_missing() {
    let dir = tempfile::tempdir().unwrap();
    let tools = Cli::on_path()
        .with(
            "herdr",
            program(dir.path(), "herdr", "echo 'no such flag' >&2; exit 2"),
        )
        .with("sluice", program(dir.path(), "sluice", "true"));
    let ToolVersion::Missing(words) = tools.version("herdr") else {
        panic!("herdr failed");
    };
    assert!(words.ends_with(": no such flag"), "{words}");
    assert_eq!(
        tools.version("sluice"),
        ToolVersion::Missing("sluice --version printed nothing".into())
    );
}
