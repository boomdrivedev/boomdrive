//! Each drive's VS Code workspace as a pass keeps it beside the clones
//! that record their drives: a dropped repository's and a moved one's
//! (the views spec; core.43; record 0017 as revised; design Decision 1).
//! The rest of the workspaces' behaviour is proven by units in
//! `src/pass.rs` and `src/code_workspace.rs`, which run no git
//! (logic-is-tested-as-units-and-few-tests-run-the-binary).

mod common;

use boomdrive::pass::Outcome;
use common::{drives, keeping, Scratch};
use serde_json::{json, Value};
use std::fs;
use std::path::{Path, PathBuf};

/// A configuration keeping `addresses`, in order, in the drive `home`,
/// that asks for the workspaces.
fn asking(addresses: &[&str]) -> String {
    format!("code_workspaces = true\n{}", keeping(addresses))
}

/// A configuration keeping each of `kept`'s drives with its addresses,
/// that asks for the workspaces.
fn asking_for(kept: &[(&str, &[&str])]) -> String {
    format!("code_workspaces = true\n{}", drives(kept))
}

/// `drive`'s workspace at the default repository root.
fn workspace(scratch: &Scratch, drive: &str) -> PathBuf {
    scratch
        .home
        .join("Code")
        .join(format!("{drive}.code-workspace"))
}

/// The JSON the file at `path` holds.
fn json(path: &Path) -> Value {
    serde_json::from_slice(&fs::read(path).expect("the workspace")).expect("JSON")
}

/// The line a pass says when it writes the workspace `file` with
/// `folders`.
fn written(file: &Path, folders: &str) -> String {
    format!(
        "boomdrive: workspace written at {}: {folders}",
        file.display()
    )
}

/// The lines among `lines` that name a workspace.
fn of_workspaces(lines: &[String]) -> Vec<&String> {
    lines
        .iter()
        .filter(|line| line.starts_with("boomdrive: workspace "))
        .collect()
}

/// The views spec's "A dropped repository leaves its workspace".
#[test]
fn a_dropped_repository_leaves_its_workspace() {
    let scratch = Scratch::new();
    scratch.origin("agentplot/flume");
    scratch.origin("agentplot/sluice");
    scratch.pass(&asking(&["agentplot/flume", "agentplot/sluice"]));
    let sluice = scratch.home.join("Code/agentplot/sluice");
    let put = scratch.git(&sluice, &["rev-parse", "HEAD"]);
    fs::write(sluice.join("README"), "mine\n").expect("a change in the view");

    let (outcome, lines) = scratch.pass(&asking(&["agentplot/flume"]));

    let file = workspace(&scratch, "home");
    assert_eq!(outcome, Outcome::Kept);
    assert_eq!(
        lines,
        [
            "agentplot/sluice: left: held by the view's changes in 1 file".to_string(),
            written(&file, "agentplot/flume"),
        ]
    );
    assert_eq!(json(&file)["folders"], json!([{"path": "agentplot/flume"}]));
    assert_eq!(
        fs::read_to_string(sluice.join("README")).expect("the change"),
        "mine\n"
    );
    assert_eq!(scratch.git(&sluice, &["rev-parse", "HEAD"]), put);
}

/// The views spec's "A repository moved to another drive".
#[test]
fn a_repository_moved_to_another_drive() {
    let scratch = Scratch::new();
    scratch.origin("agentplot/flume");
    scratch.origin("agentplot/sluice");
    scratch.pass(&asking(&["agentplot/flume", "agentplot/sluice"]));
    let sluice = scratch.home.join("Code/agentplot/sluice");
    let put = scratch.git(&sluice, &["rev-parse", "HEAD"]);

    let (outcome, lines) = scratch.pass(&asking_for(&[
        ("home", &["agentplot/flume"]),
        ("work", &["agentplot/sluice"]),
    ]));

    let home = workspace(&scratch, "home");
    let work = workspace(&scratch, "work");
    assert_eq!(outcome, Outcome::Kept);
    assert_eq!(json(&home)["folders"], json!([{"path": "agentplot/flume"}]));
    assert_eq!(
        json(&work)["folders"],
        json!([{"path": "agentplot/sluice"}])
    );
    assert_eq!(
        of_workspaces(&lines),
        [
            &written(&home, "agentplot/flume"),
            &written(&work, "agentplot/sluice"),
        ]
    );
    assert_eq!(scratch.git(&sluice, &["rev-parse", "HEAD"]), put);
    assert!(scratch.clone_of("agentplot/sluice").exists());
}
