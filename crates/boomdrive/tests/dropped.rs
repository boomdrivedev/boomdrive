//! A repository the configuration drops: never fetched again, its clone
//! and view removed once neither holds work the forge lacks, and left
//! and named until then (the clones spec's dropped repository and the
//! work the forge lacks; the views spec's owner folder, its workspace
//! with it; the running spec's "Left for the user"; design 8; core.43).

mod common;

use boomdrive::pass::Outcome;
use common::{keeping, Scratch};
use serde_json::{json, Value};
use std::path::{Path, PathBuf};

/// The names of the entries in `dir`, sorted; none where it is missing.
fn entries(dir: &Path) -> Vec<String> {
    let Ok(read) = std::fs::read_dir(dir) else {
        return Vec::new();
    };
    let mut names: Vec<String> = read
        .map(|entry| {
            entry
                .expect("an entry")
                .file_name()
                .to_string_lossy()
                .into_owned()
        })
        .collect();
    names.sort();
    names
}

/// `agentplot/flume`'s view under the default repository root.
fn view(scratch: &Scratch) -> PathBuf {
    scratch.home.join("Code/agentplot/flume")
}

/// A configuration keeping `addresses`, in order, that asks for the
/// workspaces.
fn asking(addresses: &[&str]) -> String {
    format!("code_workspaces = true\n{}", keeping(addresses))
}

/// `agentplot`'s workspace under the default repository root.
fn workspace(scratch: &Scratch) -> PathBuf {
    scratch.home.join("Code/agentplot/agentplot.code-workspace")
}

/// The JSON the file at `path` holds.
fn json(path: &Path) -> Value {
    serde_json::from_slice(&std::fs::read(path).expect("the workspace")).expect("JSON")
}

/// The line a pass says when it removes the workspace `file` with its
/// folder.
fn taken(file: &Path) -> String {
    format!(
        "boomdrive: workspace removed from {}, with its folder",
        file.display()
    )
}

/// The commit HEAD is at in `dir`.
fn head(scratch: &Scratch, dir: &Path) -> String {
    scratch.git(dir, &["rev-parse", "HEAD"])
}

/// Keeps `agentplot/flume` for one pass, which clones it and makes its
/// view, and gives the view.
fn kept(scratch: &Scratch) -> PathBuf {
    scratch.origin("agentplot/flume");
    let (outcome, _) = scratch.pass(&keeping(&["agentplot/flume"]));
    assert_eq!(outcome, Outcome::Kept);
    view(scratch)
}

/// Commits on the branch `wip` in the view, then puts the view back
/// where the runner put it, so only `wip` holds what the forge lacks.
fn wip(scratch: &Scratch, view: &Path) {
    let put = head(scratch, view);
    scratch.git(view, &["switch", "--quiet", "--create", "wip"]);
    std::fs::write(view.join("wip"), "mine\n").expect("a file of the user's");
    scratch.git(view, &["add", "wip"]);
    scratch.git(view, &["commit", "--quiet", "-m", "wip"]);
    scratch.git(view, &["switch", "--quiet", "--detach", &put]);
}

const HELD_BY_WIP: &str = "agentplot/flume: left: held by branch wip, 1 commit the forge lacks";

const REMOVED: &str = "agentplot/flume: removed: clone and view";

/// Asserts that `agentplot/flume`'s clone and view are gone, and the
/// owner folders each held.
fn assert_gone(scratch: &Scratch) {
    assert!(!view(scratch).exists(), "the view is left");
    assert!(
        !scratch.container("agentplot/flume").exists(),
        "the container is left"
    );
    assert_eq!(
        entries(&scratch.data().join("repositories")),
        [] as [&str; 0]
    );
}

#[test]
fn a_stranger_in_the_data_directory() {
    let scratch = Scratch::new();
    let origin = scratch.origin("someone/else");
    let stranger = scratch.container("someone/else");
    let bare = stranger.join("else.git");
    std::fs::create_dir_all(&stranger).expect("the stranger's folder");
    scratch.git(
        &scratch.root,
        &[
            "clone",
            "--quiet",
            "--bare",
            &origin.url(),
            &bare.to_string_lossy(),
        ],
    );
    let first = origin.tip("main");
    origin.commit("main", "README", "two\n");

    let (outcome, lines) = scratch.pass(&keeping(&[]));

    assert_eq!(outcome, Outcome::Kept);
    assert_eq!(lines, Vec::<String>::new());
    assert_eq!(entries(&stranger), ["else.git"]);
    assert_eq!(scratch.git(&bare, &["rev-parse", "refs/heads/main"]), first);
}

#[test]
fn dropped_while_holding_work() {
    let scratch = Scratch::new();
    let origin = scratch.origin("agentplot/flume");
    scratch.pass(&keeping(&["agentplot/flume"]));
    let view = view(&scratch);
    wip(&scratch, &view);
    let bare = scratch.clone_of("agentplot/flume");
    let fetched = scratch.git(&bare, &["rev-parse", "refs/remotes/origin/main"]);
    let commit = origin.commit("main", "README", "two\n");
    assert_ne!(commit, fetched);

    let (outcome, lines) = scratch.pass(&keeping(&[]));

    assert_eq!(outcome, Outcome::Kept);
    assert_eq!(lines, [HELD_BY_WIP]);
    assert_eq!(
        scratch.git(&bare, &["rev-parse", "refs/remotes/origin/main"]),
        fetched,
        "a dropped clone is fetched"
    );
}

#[test]
fn dropped_with_nothing_of_its_own() {
    let scratch = Scratch::new();
    kept(&scratch);

    let (outcome, lines) = scratch.pass(&keeping(&[]));

    assert_eq!(outcome, Outcome::Kept);
    assert_eq!(lines, [REMOVED]);
    assert_gone(&scratch);

    let (outcome, lines) = scratch.pass(&keeping(&[]));
    assert_eq!(outcome, Outcome::Kept);
    assert_eq!(lines, Vec::<String>::new());
}

#[test]
fn emptied_later() {
    let scratch = Scratch::new();
    let view = kept(&scratch);
    wip(&scratch, &view);
    let (_, lines) = scratch.pass(&keeping(&[]));
    assert_eq!(lines, [HELD_BY_WIP]);
    scratch.git(&view, &["branch", "--quiet", "--delete", "--force", "wip"]);

    let (outcome, lines) = scratch.pass(&keeping(&[]));

    assert_eq!(outcome, Outcome::Kept);
    assert_eq!(lines, [REMOVED]);
    assert_gone(&scratch);
}

#[test]
fn the_views_folder_is_not_the_view_any_more() {
    let scratch = Scratch::new();
    let view = kept(&scratch);
    std::fs::remove_dir_all(&view).expect("the view removed");
    std::fs::create_dir_all(&view).expect("a folder of the user's");
    std::fs::write(view.join("notes"), "mine\n").expect("a file of the user's");

    let (outcome, lines) = scratch.pass(&keeping(&[]));

    assert_eq!(outcome, Outcome::Kept);
    assert_eq!(lines, ["agentplot/flume: removed: clone"]);
    assert_eq!(entries(&view), ["notes"]);
    assert_eq!(
        std::fs::read_to_string(view.join("notes")).expect("the user's file"),
        "mine\n"
    );
    assert!(!scratch.container("agentplot/flume").exists());
}

#[test]
fn a_branch_the_forge_lacks() {
    let scratch = Scratch::new();
    let view = kept(&scratch);
    wip(&scratch, &view);
    let put = head(&scratch, &view);
    let bare = scratch.clone_of("agentplot/flume");
    let wip = scratch.git(&bare, &["rev-parse", "refs/heads/wip"]);

    for _ in 0..2 {
        let (outcome, lines) = scratch.pass(&keeping(&[]));

        assert_eq!(outcome, Outcome::Kept);
        assert_eq!(lines, [HELD_BY_WIP]);
        assert_eq!(head(&scratch, &view), put);
        assert_eq!(scratch.git(&bare, &["rev-parse", "refs/heads/wip"]), wip);
        assert!(!view.join("wip").exists());
    }

    scratch.git(&view, &["push", "--quiet", "origin", "wip"]);
    let (outcome, lines) = scratch.pass(&keeping(&[]));

    assert_eq!(outcome, Outcome::Kept);
    assert_eq!(lines, [REMOVED], "a branch the forge reaches holds it");
    assert_gone(&scratch);
}

#[test]
fn a_change_in_the_view() {
    for (file, text) in [("README", "mine\n"), ("notes", "mine\n")] {
        let scratch = Scratch::new();
        let view = kept(&scratch);
        let put = head(&scratch, &view);
        std::fs::write(view.join(file), text).expect("a change in the view");

        let (outcome, lines) = scratch.pass(&keeping(&[]));

        assert_eq!(outcome, Outcome::Kept);
        assert_eq!(
            lines,
            ["agentplot/flume: left: held by the view's changes in 1 file"],
            "{file}"
        );
        assert_eq!(head(&scratch, &view), put);
        assert_eq!(
            std::fs::read_to_string(view.join(file)).expect("the change"),
            text
        );
        assert!(scratch.clone_of("agentplot/flume").exists());
    }
}

#[test]
fn a_commit_in_the_view() {
    let scratch = Scratch::new();
    let view = kept(&scratch);
    std::fs::write(view.join("README"), "mine\n").expect("a change in the view");
    scratch.git(&view, &["commit", "--quiet", "--all", "-m", "mine"]);
    let commit = head(&scratch, &view);

    let (outcome, lines) = scratch.pass(&keeping(&[]));

    assert_eq!(outcome, Outcome::Kept);
    assert_eq!(
        lines,
        ["agentplot/flume: left: held by the view's HEAD, 1 commit the forge lacks"]
    );
    assert_eq!(head(&scratch, &view), commit);
    assert!(scratch.clone_of("agentplot/flume").exists());
}

#[test]
fn a_place_in_the_clone() {
    let scratch = Scratch::new();
    let view = kept(&scratch);
    let put = head(&scratch, &view);
    let bare = scratch.clone_of("agentplot/flume");
    let place = scratch
        .container("agentplot/flume")
        .join("places/change--one");
    scratch.git(
        &bare,
        &[
            "worktree",
            "add",
            "--quiet",
            "-b",
            "change/one",
            &place.to_string_lossy(),
            "origin/main",
        ],
    );

    let (outcome, lines) = scratch.pass(&keeping(&[]));

    assert_eq!(outcome, Outcome::Kept);
    assert_eq!(
        lines,
        [format!(
            "agentplot/flume: left: held by the worktree {}",
            place.display()
        )]
    );
    assert_eq!(head(&scratch, &view), put);
    assert_eq!(head(&scratch, &place), put);
}

#[test]
fn only_ignored_files() {
    let scratch = Scratch::new();
    let origin = scratch.origin("agentplot/flume");
    origin.commit("main", ".gitignore", "build/\n");
    scratch.pass(&keeping(&["agentplot/flume"]));
    let view = view(&scratch);
    std::fs::create_dir_all(view.join("build")).expect("an ignored folder");
    std::fs::write(view.join("build/out"), "made\n").expect("an ignored file");

    let (outcome, lines) = scratch.pass(&keeping(&[]));

    assert_eq!(outcome, Outcome::Kept);
    assert_eq!(lines, [REMOVED]);
    assert_gone(&scratch);
}

#[test]
fn the_owners_last_view() {
    let scratch = Scratch::new();
    kept(&scratch);

    scratch.pass(&keeping(&[]));

    let root = scratch.home.join("Code");
    assert!(
        !root.join("agentplot").exists(),
        "the owner's folder is left"
    );
    assert!(root.is_dir(), "the repository root is removed");
    assert_eq!(entries(&root), [] as [&str; 0]);
}

#[test]
fn the_owners_folder_holds_more() {
    let scratch = Scratch::new();
    scratch.origin("agentplot/flume");
    scratch.origin("agentplot/sluice");
    scratch.pass(&keeping(&["agentplot/flume", "agentplot/sluice"]));
    let owner = scratch.home.join("Code/agentplot");
    std::fs::write(owner.join("notes"), "mine\n").expect("a file of the user's");

    let (outcome, lines) = scratch.pass(&keeping(&["agentplot/sluice"]));

    assert_eq!(outcome, Outcome::Kept);
    assert_eq!(lines, [REMOVED]);
    assert_eq!(entries(&owner), ["notes", "sluice"]);
    assert_eq!(
        entries(&scratch.data().join("repositories/agentplot")),
        ["sluice"]
    );
}

#[test]
fn left_for_the_user() {
    let scratch = Scratch::new();
    let view = kept(&scratch);
    wip(&scratch, &view);
    let file = scratch.configure(&keeping(&[]));

    let output = scratch
        .boomdrive()
        .args(["run", "--once"])
        .output()
        .expect("boomdrive runs");

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(output.status.code(), Some(0), "{stderr}");
    assert_eq!(
        stderr,
        format!(
            "boomdrive {}: keeping 0 repositories from {}; data {}; repository root {}\n{HELD_BY_WIP}\n",
            env!("CARGO_PKG_VERSION"),
            file.display(),
            scratch.data().display(),
            scratch.home.join("Code").display()
        )
    );
    assert!(scratch.clone_of("agentplot/flume").exists());
    assert!(view.exists());
}

#[test]
fn an_old_views_folder_of_the_users_holds_nothing() {
    let scratch = Scratch::new();
    let old = kept(&scratch);
    std::fs::remove_dir_all(&old).expect("the old view removed");
    std::fs::create_dir_all(&old).expect("a folder of the user's");
    std::fs::write(old.join("notes"), "mine\n").expect("a file of the user's");
    let in_src =
        |addresses: &[&str]| format!("repository_root = \"~/src\"\n{}", keeping(addresses));
    scratch.pass(&in_src(&["agentplot/flume"]));
    assert!(scratch.home.join("src/agentplot/flume").is_dir());

    let (outcome, lines) = scratch.pass(&in_src(&[]));

    assert_eq!(outcome, Outcome::Kept);
    assert_eq!(lines, [REMOVED]);
    assert!(!scratch.home.join("src/agentplot").exists());
    assert!(!scratch.container("agentplot/flume").exists());
    assert_eq!(entries(&old), ["notes"]);
    assert_eq!(
        std::fs::read_to_string(old.join("notes")).expect("the user's file"),
        "mine\n"
    );
}

#[test]
fn the_owners_last_view_with_its_workspace() {
    for partial in [false, true] {
        let scratch = Scratch::new();
        scratch.origin("agentplot/flume");
        let (outcome, _) = scratch.pass(&asking(&["agentplot/flume"]));
        assert_eq!(outcome, Outcome::Kept);
        let owner = scratch.home.join("Code/agentplot");
        let file = workspace(&scratch);
        if partial {
            std::fs::write(owner.join(".agentplot.code-workspace.partial"), "half a fi")
                .expect("a partial file left");
        }
        assert!(file.is_file(), "no workspace is written");

        let (outcome, lines) = scratch.pass(&asking(&[]));

        assert_eq!(outcome, Outcome::Kept);
        assert_eq!(lines, [taken(&file), REMOVED.to_string()], "{partial}");
        assert!(!owner.exists(), "the owner's folder is left");
        assert_eq!(entries(&scratch.home.join("Code")), [] as [&str; 0]);
        assert_gone(&scratch);

        let (outcome, lines) = scratch.pass(&asking(&[]));
        assert_eq!(outcome, Outcome::Kept);
        assert_eq!(lines, Vec::<String>::new());
    }
}

#[test]
fn an_owner_still_kept_keeps_its_workspace() {
    let scratch = Scratch::new();
    scratch.origin("agentplot/flume");
    scratch.empty_origin("agentplot/empty");
    scratch.pass(&asking(&["agentplot/flume", "agentplot/empty"]));
    let owner = scratch.home.join("Code/agentplot");
    let file = workspace(&scratch);
    std::fs::write(
        &file,
        "{\n\t\"folders\": [{\"path\": \"flume\"}, {\"path\": \"empty\"}],\n\
         \t\"settings\": {\"editor.tabSize\": 2},\n\
         \t\"extensions\": {\"recommendations\": [\"a.b\"]}\n}\n",
    )
    .expect("the user's settings");

    let (outcome, lines) = scratch.pass(&asking(&["agentplot/empty"]));

    assert_eq!(outcome, Outcome::Kept);
    assert_eq!(
        lines,
        [
            "agentplot/empty: origin names no default branch, so it has no view".to_string(),
            REMOVED.to_string(),
            format!("boomdrive: workspace written at {}: empty", file.display()),
        ]
    );
    assert_eq!(entries(&owner), ["agentplot.code-workspace"]);
    let value = json(&file);
    assert_eq!(
        value
            .as_object()
            .expect("an object")
            .keys()
            .collect::<Vec<_>>(),
        ["folders", "settings", "extensions"]
    );
    assert_eq!(
        value,
        json!({
            "folders": [{"path": "empty"}],
            "settings": {"editor.tabSize": 2},
            "extensions": {"recommendations": ["a.b"]},
        })
    );
}

#[test]
fn a_workspace_not_asked_for() {
    let scratch = Scratch::new();
    scratch.origin("agentplot/flume");
    scratch.pass(&asking(&["agentplot/flume"]));
    let owner = scratch.home.join("Code/agentplot");
    let file = workspace(&scratch);
    let bytes = std::fs::read(&file).expect("the workspace");

    let (outcome, lines) = scratch.pass(&keeping(&[]));

    assert_eq!(outcome, Outcome::Kept);
    assert_eq!(lines, [REMOVED]);
    assert!(!view(&scratch).exists(), "the view is left");
    assert_eq!(entries(&owner), ["agentplot.code-workspace"]);
    assert_eq!(std::fs::read(&file).expect("the workspace"), bytes);
}
