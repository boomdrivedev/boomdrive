//! A drive's workspace goes with its drive (the views spec's "A drive's
//! workspace goes with its drive"; core.43; record 0017 as revised;
//! design Decisions 7 and 8): each kept repository's clone records the
//! drive the configuration names it under, and once a drive has left the
//! configuration and no clone the runner holds records it, the pass
//! removes the drive's workspace at the repository root, and no other.

mod common;

use boomdrive::pass::Outcome;
use common::{drives, keeping, Scratch};
use serde_json::{json, Value};
use std::fs;
use std::path::{Path, PathBuf};

/// The drive the clone of `address` records, `boomdrive.drive` in its
/// bare repository's own configuration.
fn recorded_drive(scratch: &Scratch, address: &str) -> String {
    scratch.git(
        &scratch.clone_of(address),
        &["config", "--local", "--get", "boomdrive.drive"],
    )
}

/// The clone records the drive its repository is named under, and the
/// one it is named under next where that changes.
#[test]
fn the_clone_records_its_drive() {
    let scratch = Scratch::new();
    scratch.origin("agentplot/flume");

    let (outcome, _) = scratch.pass(&keeping(&["agentplot/flume"]));

    assert_eq!(outcome, Outcome::Kept);
    assert_eq!(recorded_drive(&scratch, "agentplot/flume"), "home");

    let (outcome, lines) = scratch.pass(&drives(&[("work", &["agentplot/flume"])]));

    assert_eq!(outcome, Outcome::Kept);
    assert_eq!(lines, [] as [String; 0], "a record is said nowhere");
    assert_eq!(recorded_drive(&scratch, "agentplot/flume"), "work");
}

/// A configuration keeping each of `kept`'s drives with its addresses,
/// that asks for the workspaces.
fn asking_for(kept: &[(&str, &[&str])]) -> String {
    format!("code_workspaces = true\n{}", drives(kept))
}

/// `drive`'s workspace at the default repository root.
fn workspace(scratch: &Scratch, drive: &str) -> PathBuf {
    scratch.home.join(format!("Code/{drive}.code-workspace"))
}

/// The line a pass says when it removes the workspace of the departed
/// drive `drive`.
fn taken(scratch: &Scratch, drive: &str) -> String {
    format!(
        "boomdrive: workspace removed at {}: its drive {drive} has left the configuration",
        workspace(scratch, drive).display()
    )
}

const REMOVED: &str = "agentplot/flume: removed: clone and view";

/// Keeps `agentplot/flume` in the drive `home` for one pass that asks for
/// the workspaces, and gives its view.
fn kept_in_home(scratch: &Scratch) -> PathBuf {
    scratch.origin("agentplot/flume");
    let (outcome, _) = scratch.pass(&asking_for(&[("home", &["agentplot/flume"])]));
    assert_eq!(outcome, Outcome::Kept);
    assert!(workspace(scratch, "home").is_file(), "no workspace written");
    scratch.home.join("Code/agentplot/flume")
}

/// The views spec's "The drive's last view".
#[test]
fn the_drives_last_view() {
    let scratch = Scratch::new();
    let view = kept_in_home(&scratch);

    let (outcome, lines) = scratch.pass(&asking_for(&[]));

    assert_eq!(outcome, Outcome::Kept);
    assert_eq!(lines, [REMOVED.to_string(), taken(&scratch, "home")]);
    assert!(!view.exists(), "the view is left");
    assert!(
        !scratch.container("agentplot/flume").exists(),
        "the clone is left"
    );
    assert!(
        !workspace(&scratch, "home").exists(),
        "the workspace is left"
    );

    let (outcome, lines) = scratch.pass(&asking_for(&[]));
    assert_eq!(outcome, Outcome::Kept);
    assert_eq!(lines, [] as [String; 0]);
}

/// The views spec's "A view still holding work": the workspace waits for
/// the pass that removes the drive's last view.
#[test]
fn a_view_still_holding_work() {
    let scratch = Scratch::new();
    let view = kept_in_home(&scratch);
    let file = workspace(&scratch, "home");
    let before = fs::read(&file).expect("the workspace");
    fs::write(view.join("README"), "mine\n").expect("a change in the view");

    let (outcome, lines) = scratch.pass(&asking_for(&[]));

    assert_eq!(outcome, Outcome::Kept);
    assert_eq!(
        lines,
        ["agentplot/flume: left: held by the view's changes in 1 file"]
    );
    assert_eq!(fs::read(&file).expect("the workspace"), before);

    scratch.git(&view, &["checkout", "--", "README"]);
    let (outcome, lines) = scratch.pass(&asking_for(&[]));

    assert_eq!(outcome, Outcome::Kept);
    assert_eq!(lines, [REMOVED.to_string(), taken(&scratch, "home")]);
    assert!(!file.exists(), "the workspace is left");
}

/// The views spec's "Its repositories moved": the drive's one repository
/// is named by another drive, so its clone records that one and the
/// departed drive's workspace goes.
#[test]
fn its_repositories_moved() {
    let scratch = Scratch::new();
    let view = kept_in_home(&scratch);
    let put = scratch.git(&view, &["rev-parse", "HEAD"]);

    let (outcome, lines) = scratch.pass(&asking_for(&[("work", &["agentplot/flume"])]));

    let work = workspace(&scratch, "work");
    assert_eq!(outcome, Outcome::Kept);
    assert_eq!(
        lines,
        [
            format!(
                "boomdrive: workspace written at {}: agentplot/flume",
                work.display()
            ),
            taken(&scratch, "home"),
        ]
    );
    assert!(
        !workspace(&scratch, "home").exists(),
        "the workspace is left"
    );
    assert_eq!(
        json(&work),
        json!({"folders": [{"path": "agentplot/flume"}], "settings": {}})
    );
    assert_eq!(scratch.git(&view, &["rev-parse", "HEAD"]), put);
}

/// The views spec's "A workspace of the person's own".
#[test]
fn a_workspace_of_the_persons_own() {
    let scratch = Scratch::new();
    kept_in_home(&scratch);
    let notes = workspace(&scratch, "notes");
    let theirs = "{\"folders\": [{\"path\": \"agentplot/flume\"}], \"settings\": {}}\n";
    fs::write(&notes, theirs).expect("a workspace of the person's");

    let (outcome, lines) = scratch.pass(&asking_for(&[]));

    assert_eq!(outcome, Outcome::Kept);
    assert_eq!(lines, [REMOVED.to_string(), taken(&scratch, "home")]);
    assert_eq!(fs::read_to_string(&notes).expect("the person's"), theirs);
}

/// The views spec's "A departed drive's workspace the runner cannot
/// read".
#[test]
fn a_departed_drives_workspace_the_runner_cannot_read() {
    let scratch = Scratch::new();
    kept_in_home(&scratch);
    let file = workspace(&scratch, "home");
    let mine = "{\n\t// mine\n\t\"folders\": []\n}\n";
    fs::write(&file, mine).expect("a comment of the person's");

    let (outcome, lines) = scratch.pass(&asking_for(&[]));

    assert_eq!(outcome, Outcome::Kept);
    assert_eq!(lines.len(), 2, "{lines:#?}");
    assert_eq!(lines[0], REMOVED);
    let left = format!(
        "boomdrive: workspace left at {}: it is not JSON the runner reads: ",
        file.display()
    );
    assert!(lines[1].starts_with(&left), "{lines:#?}");
    assert!(!lines[1].contains("mine"), "{lines:#?}");
    assert_eq!(fs::read_to_string(&file).expect("the workspace"), mine);
}

/// The views spec's "A departed drive's workspace not asked for".
#[test]
fn a_departed_drives_workspace_not_asked_for() {
    let scratch = Scratch::new();
    let view = kept_in_home(&scratch);
    let file = workspace(&scratch, "home");
    let before = fs::read(&file).expect("the workspace");

    let (outcome, lines) = scratch.pass(&drives(&[]));

    assert_eq!(outcome, Outcome::Kept);
    assert_eq!(lines, [REMOVED]);
    assert!(!view.exists(), "the view is left");
    assert!(
        !scratch.container("agentplot/flume").exists(),
        "the clone is left"
    );
    assert_eq!(fs::read(&file).expect("the workspace"), before);
}

/// The JSON the file at `path` holds.
fn json(path: &Path) -> Value {
    serde_json::from_slice(&fs::read(path).expect("the workspace")).expect("JSON")
}
