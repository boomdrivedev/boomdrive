//! A repository the configuration drops: never fetched again, its clone
//! and view removed once neither holds work the forge lacks, and left
//! and named until then (the clones spec's dropped repository and the
//! work the forge lacks; the views spec's owner folder, taken only when
//! empty; the running spec's "Left for the user"; design 8; design
//! Decision 10; core.43). One the runner's report holds a launch in, at
//! a place that has not ended, is drained: kept as it is until the place
//! ends (record 0021).

mod common;

use boomdrive::pass::Outcome;
use boomdrive_control::{Answer, At, Carried, Id, Launched, Lease, Outcome as Answered, Report};
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

/// The workspace an earlier runner kept in `agentplot`'s folder, under
/// the default repository root.
fn owners_workspace(scratch: &Scratch) -> PathBuf {
    scratch.home.join("Code/agentplot/agentplot.code-workspace")
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

/// The views spec's "The owner's last view".
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

/// The views spec's "The owner's folder holds more".
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

/// The views spec's "An owner's workspace from before": a workspace an
/// earlier runner kept in the owner's folder, with its partial file or
/// without, is left as it is, with its folder, where the drive's
/// workspace is kept at the root.
#[test]
fn an_owners_workspace_from_before() {
    for partial in [false, true] {
        let scratch = Scratch::new();
        scratch.origin("agentplot/flume");
        let (outcome, _) = scratch.pass(&asking(&["agentplot/flume"]));
        assert_eq!(outcome, Outcome::Kept);
        let owner = scratch.home.join("Code/agentplot");
        let file = owners_workspace(&scratch);
        let before = "{\n\t\"folders\": [{\"path\": \"flume\"}],\n\t\"settings\": {}\n}\n";
        std::fs::write(&file, before).expect("an owner's workspace from before");
        let mut held = vec!["agentplot.code-workspace", "flume"];
        if partial {
            std::fs::write(owner.join(".agentplot.code-workspace.partial"), "half a fi")
                .expect("a partial file left");
            held.insert(0, ".agentplot.code-workspace.partial");
        }
        assert_eq!(entries(&owner), held, "{partial}");

        let (outcome, lines) = scratch.pass(&asking(&[]));

        let home = scratch.home.join("Code/home.code-workspace");
        assert_eq!(outcome, Outcome::Kept);
        assert_eq!(
            lines,
            [
                REMOVED.to_string(),
                format!(
                    "boomdrive: workspace written at {}: no folders",
                    home.display()
                ),
            ],
            "{partial}"
        );
        assert_gone(&scratch);
        held.retain(|name| *name != "flume");
        assert_eq!(entries(&owner), held, "{partial}");
        assert_eq!(
            std::fs::read_to_string(&file).expect("the owner's workspace"),
            before
        );

        let (outcome, lines) = scratch.pass(&asking(&[]));
        assert_eq!(outcome, Outcome::Kept);
        assert_eq!(lines, Vec::<String>::new());
        assert_eq!(entries(&owner), held, "{partial}");
    }
}

/// The place the runner's report holds launch `a` at.
const PLACE: &str = "/places/change--a";

/// Writes the runner's report file, holding launch `a` launched at
/// [`PLACE`] in `agentplot/flume`, its carried status reading the place's
/// entry, `place` and `phase`, `listed` or not.
fn report_holding(scratch: &Scratch, place: &str, phase: Value, listed: bool) {
    let status = json!({ "status": { "places": [{ "place": place, "phase": phase }] } });
    let a = Launched {
        lease: Lease {
            number: 1,
            expires: None,
        },
        repository: Some("agentplot/flume".to_string()),
        answer: Answer {
            outcome: Answered::Launched,
            place_dir: Some(PLACE.to_string()),
            words: None,
            at: At::parse("2026-11-01T10:00:00Z").unwrap(),
        },
        status: Some(Carried {
            text: status.to_string(),
            at: At::parse("2026-11-01T10:01:00Z").unwrap(),
            listed,
        }),
        entry: None,
    };
    let report = Report {
        launches: [(Id::new("a"), a)].into(),
        ..Report::default()
    };
    boomdrive::report::save(&report_file(scratch), &report).expect("the report written");
}

/// The runner's report file in its data directory.
fn report_file(scratch: &Scratch) -> PathBuf {
    boomdrive::report::file(&boomdrive::report::control_dir(&scratch.data()))
}

const HELD_BY_THE_PLACE: &str =
    "agentplot/flume: left: held by the place /places/change--a of a, which has not ended";

/// The clones spec's "Dropped while a place is live": kept, neither
/// fetched nor moved, and named held by the place.
#[test]
fn dropped_while_a_place_is_live() {
    let scratch = Scratch::new();
    let origin = scratch.origin("agentplot/flume");
    scratch.pass(&keeping(&["agentplot/flume"]));
    let view = view(&scratch);
    let put = head(&scratch, &view);
    let bare = scratch.clone_of("agentplot/flume");
    let fetched = scratch.git(&bare, &["rev-parse", "refs/remotes/origin/main"]);
    origin.commit("main", "README", "two\n");
    report_holding(&scratch, PLACE, json!("live"), true);

    for _ in 0..2 {
        let (outcome, lines) = scratch.pass(&keeping(&[]));
        assert_eq!(outcome, Outcome::Kept);
        assert_eq!(lines, [HELD_BY_THE_PLACE]);
    }
    assert_eq!(head(&scratch, &view), put, "the view is moved");
    assert_eq!(
        scratch.git(&bare, &["rev-parse", "refs/remotes/origin/main"]),
        fetched,
        "a drained clone is fetched"
    );
    assert!(scratch.container("agentplot/flume").exists());
}

/// The clones spec's "The place tidied": its record `<place>@<at>`
/// carried tidied and landed, the next pass removes the clone and view.
#[test]
fn the_place_tidied() {
    let scratch = Scratch::new();
    kept(&scratch);
    report_holding(&scratch, PLACE, json!("live"), true);
    let (_, lines) = scratch.pass(&keeping(&[]));
    assert_eq!(lines, [HELD_BY_THE_PLACE]);

    let tidied = json!({ "tidied": {
        "over": { "landed": { "commit": "c0ffee" } },
        "at": 1793559694,
        "pending_for": ["scratch/planner"],
    } });
    report_holding(&scratch, &format!("{PLACE}@1793559694"), tidied, true);
    let (outcome, lines) = scratch.pass(&keeping(&[]));

    assert_eq!(outcome, Outcome::Kept);
    assert_eq!(lines, [REMOVED]);
    assert_gone(&scratch);
}

/// The clones spec's "The place's end held for the runner": its record
/// `<place>@<at>` carried tidied and landed, pending for the runner, the
/// next pass keeps the clone and view, named held by the place; once the
/// carried record is pending for the runner no more, the pass after
/// removes them (record 0030; Task notes 5.2).
#[test]
fn the_places_end_held_for_the_runner() {
    let scratch = Scratch::new();
    kept(&scratch);
    report_holding(&scratch, PLACE, json!("live"), true);
    let (_, lines) = scratch.pass(&keeping(&[]));
    assert_eq!(lines, [HELD_BY_THE_PLACE]);

    let tidied = |pending_for: Value| {
        json!({ "tidied": {
            "over": { "landed": { "commit": "c0ffee" } },
            "at": 1793559694,
            "pending_for": pending_for,
        } })
    };
    let record = format!("{PLACE}@1793559694");
    report_holding(&scratch, &record, tidied(json!(["boomdrive"])), true);
    let (outcome, lines) = scratch.pass(&keeping(&[]));
    assert_eq!(outcome, Outcome::Kept);
    assert_eq!(lines, [HELD_BY_THE_PLACE]);
    assert!(scratch.container("agentplot/flume").exists());

    report_holding(&scratch, &record, tidied(json!(["scratch/planner"])), true);
    let (outcome, lines) = scratch.pass(&keeping(&[]));
    assert_eq!(outcome, Outcome::Kept);
    assert_eq!(lines, [REMOVED]);
    assert_gone(&scratch);
}

/// The clones spec's "The place no longer listed": its carried status
/// standing not listed, the next pass removes the clone and view.
#[test]
fn the_place_no_longer_listed() {
    let scratch = Scratch::new();
    kept(&scratch);
    report_holding(&scratch, PLACE, json!("live"), true);
    let (_, lines) = scratch.pass(&keeping(&[]));
    assert_eq!(lines, [HELD_BY_THE_PLACE]);

    report_holding(&scratch, PLACE, json!("live"), false);
    let (outcome, lines) = scratch.pass(&keeping(&[]));

    assert_eq!(outcome, Outcome::Kept);
    assert_eq!(lines, [REMOVED]);
    assert_gone(&scratch);
}

/// The clones spec's "A report that cannot be read": no dropped clone is
/// let go, each named with the report file, and the pass fails.
#[test]
fn a_report_that_cannot_be_read() {
    let scratch = Scratch::new();
    let view = kept(&scratch);
    let file = report_file(&scratch);
    std::fs::create_dir_all(file.parent().unwrap()).unwrap();
    std::fs::write(&file, "{\"launches\": 7}").unwrap();

    let (outcome, lines) = scratch.pass(&keeping(&[]));

    assert_eq!(outcome, Outcome::Failed);
    assert_eq!(lines.len(), 1, "{lines:?}");
    let named = format!(
        "agentplot/flume: not let go: {} does not read as a report",
        file.display()
    );
    assert!(lines[0].starts_with(&named), "{lines:?}");
    assert!(view.exists(), "the view is removed");
    assert!(scratch.container("agentplot/flume").exists());
}
