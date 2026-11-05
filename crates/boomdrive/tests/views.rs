//! Each kept repository's view under the repository root: made where it is
//! missing, never over a folder that is there, only ever detached, and
//! moved only while it stands where the runner last put it, whatever
//! guards the clone's own default branch (the views spec; the clones
//! spec's default branch and repository root; design 7; core.14).
//! An old root's owner folder goes with its workspace (core.43).

mod common;

use boomdrive::pass::Outcome;
use common::{keeping, Scratch};
use serde_json::json;
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

/// The commit HEAD is at in `dir`.
fn head(scratch: &Scratch, dir: &Path) -> String {
    scratch.git(dir, &["rev-parse", "HEAD"])
}

/// Whether HEAD in `dir` is detached, naming no branch.
fn detached(scratch: &Scratch, dir: &Path) -> bool {
    !scratch.holds(dir, &["symbolic-ref", "--quiet", "HEAD"])
}

fn read(path: &Path) -> String {
    std::fs::read_to_string(path).expect("the file")
}

/// The line a pass says when it makes `path`'s view at `commit`.
fn made(path: &Path, default: &str, commit: &str) -> String {
    format!(
        "agentplot/flume: view made at {}, at origin/{default} {}",
        path.display(),
        &commit[..7]
    )
}

/// The lines a pass says when origin's `main` moved to `commit`: the
/// view's move, where `view`, and nothing of the clone's own `main`,
/// which the runner never moves (core.14).
fn moved(commit: &str, view: bool) -> Vec<String> {
    let mut lines = Vec::new();
    if view {
        lines.push(format!(
            "agentplot/flume: view moved to origin/main at {}",
            &commit[..7]
        ));
    }
    lines
}

#[test]
fn the_first_view() {
    let scratch = Scratch::new();
    let origin = scratch.origin("agentplot/flume");

    let (outcome, lines) = scratch.pass(&keeping(&["agentplot/flume"]));

    let view = view(&scratch);
    let tip = origin.tip("main");
    assert_eq!(outcome, Outcome::Kept);
    assert_eq!(
        lines,
        [
            "agentplot/flume: cloned from https://github.com/agentplot/flume.git".to_string(),
            made(&view, "main", &tip),
        ]
    );
    assert_eq!(head(&scratch, &view), tip);
    assert!(detached(&scratch, &view), "the view is on a branch");
    assert_eq!(
        scratch.git(
            &view,
            &["rev-parse", "--path-format=absolute", "--git-common-dir"]
        ),
        scratch.clone_of("agentplot/flume").to_string_lossy()
    );
    assert_eq!(read(&view.join("README")), "a scratch repository\n");
    let bare = scratch.clone_of("agentplot/flume");
    assert_eq!(
        scratch.git(&bare, &["config", "boomdrive.view"]),
        view.to_string_lossy()
    );
    assert_eq!(
        scratch.git(&bare, &["rev-parse", "refs/boomdrive/view"]),
        tip
    );
}

#[test]
fn a_view_the_user_deleted() {
    let scratch = Scratch::new();
    let origin = scratch.origin("agentplot/flume");
    let config = keeping(&["agentplot/flume"]);
    scratch.pass(&config);
    std::fs::remove_dir_all(scratch.home.join("Code")).expect("the repository root removed");

    let (outcome, lines) = scratch.pass(&config);

    let view = view(&scratch);
    let tip = origin.tip("main");
    assert_eq!(outcome, Outcome::Kept);
    assert_eq!(lines, [made(&view, "main", &tip)]);
    assert_eq!(head(&scratch, &view), tip);
    assert!(detached(&scratch, &view), "the view is on a branch");
    assert_eq!(read(&view.join("README")), "a scratch repository\n");
}

#[test]
fn a_folder_of_the_users() {
    let scratch = Scratch::new();
    let origin = scratch.origin("agentplot/flume");
    let config = keeping(&["agentplot/flume"]);
    let view = view(&scratch);
    std::fs::create_dir_all(&view).expect("a folder of the user's");
    std::fs::write(view.join("notes"), "mine\n").expect("a file of the user's");
    let left = format!(
        "agentplot/flume: left: {} is not a worktree of the clone, so no view is made",
        view.display()
    );

    let (outcome, lines) = scratch.pass(&config);

    assert_eq!(outcome, Outcome::Kept);
    assert_eq!(
        lines,
        [
            "agentplot/flume: cloned from https://github.com/agentplot/flume.git".to_string(),
            left.clone(),
        ]
    );

    let commit = origin.commit("main", "README", "two\n");
    let (outcome, lines) = scratch.pass(&config);

    assert_eq!(outcome, Outcome::Kept);
    let mut said = moved(&commit, false);
    said.push(left);
    assert_eq!(lines, said);
    assert_eq!(entries(&view), ["notes"]);
    assert_eq!(read(&view.join("notes")), "mine\n");
    let bare = scratch.clone_of("agentplot/flume");
    assert_eq!(
        scratch.git(&bare, &["rev-parse", "refs/remotes/origin/main"]),
        commit
    );
    assert_eq!(
        scratch.git(&bare, &["worktree", "list", "--porcelain"]),
        format!("worktree {}\nbare", bare.display()),
        "a view was made elsewhere"
    );
}

#[test]
fn made_and_moved_detached() {
    let scratch = Scratch::new();
    let origin = scratch.origin("agentplot/flume");
    let config = keeping(&["agentplot/flume"]);
    let view = view(&scratch);

    scratch.pass(&config);
    assert!(detached(&scratch, &view), "the view is made on a branch");
    assert_eq!(scratch.git(&view, &["branch", "--show-current"]), "");

    let commit = origin.commit("main", "README", "two\n");
    let (_, lines) = scratch.pass(&config);
    assert_eq!(lines, moved(&commit, true));
    assert_eq!(head(&scratch, &view), commit);
    assert!(detached(&scratch, &view), "the view is moved onto a branch");
    assert_eq!(scratch.git(&view, &["branch", "--show-current"]), "");
}

#[test]
fn origin_moves_on() {
    let scratch = Scratch::new();
    let origin = scratch.origin("agentplot/flume");
    let config = keeping(&["agentplot/flume"]);
    scratch.pass(&config);
    let commit = origin.commit("main", "README", "two\n");

    let (outcome, lines) = scratch.pass(&config);

    let view = view(&scratch);
    assert_eq!(outcome, Outcome::Kept);
    assert_eq!(lines, moved(&commit, true));
    assert_eq!(head(&scratch, &view), commit);
    assert!(detached(&scratch, &view), "the view is on a branch");
    assert_eq!(read(&view.join("README")), "two\n");
    assert_eq!(
        scratch.git(
            &scratch.clone_of("agentplot/flume"),
            &["rev-parse", "refs/boomdrive/view"]
        ),
        commit
    );

    let (outcome, lines) = scratch.pass(&config);
    assert_eq!(outcome, Outcome::Kept);
    assert_eq!(lines, Vec::<String>::new(), "a view at origin's is moved");
}

/// A clone a runner made before the rename records itself under
/// `flywheel` (record 0016): it is still the runner's, fetched and its
/// view moved, its records carried over to `boomdrive` (core.36).
#[test]
fn a_clone_made_before_the_rename() {
    let scratch = Scratch::new();
    let origin = scratch.origin("agentplot/flume");
    let config = keeping(&["agentplot/flume"]);
    scratch.pass(&config);
    let bare = scratch.clone_of("agentplot/flume");
    let first = scratch.git(&bare, &["rev-parse", "refs/boomdrive/view"]);
    scratch.git(
        &bare,
        &[
            "config",
            "--local",
            "--rename-section",
            "boomdrive",
            "flywheel",
        ],
    );
    scratch.git(&bare, &["update-ref", "refs/flywheel/view", &first]);
    scratch.git(&bare, &["update-ref", "-d", "refs/boomdrive/view"]);
    let commit = origin.commit("main", "README", "two\n");

    let (outcome, lines) = scratch.pass(&config);

    assert_eq!(outcome, Outcome::Kept);
    assert_eq!(lines, moved(&commit, true));
    assert_eq!(head(&scratch, &view(&scratch)), commit);
    assert_eq!(
        scratch.git(&bare, &["config", "--local", "boomdrive.address"]),
        "agentplot/flume"
    );
    assert_eq!(
        scratch.git(&bare, &["config", "--local", "boomdrive.view"]),
        view(&scratch).display().to_string()
    );
    assert!(!scratch.holds(
        &bare,
        &["config", "--local", "--get-regexp", "^flywheel\\."]
    ));
    assert_eq!(
        scratch.git(&bare, &["rev-parse", "refs/boomdrive/view"]),
        commit
    );
    assert!(!scratch.holds(
        &bare,
        &["rev-parse", "--verify", "--quiet", "refs/flywheel/view"]
    ));
}

#[test]
fn the_user_moved_it() {
    let scratch = Scratch::new();
    let origin = scratch.origin("agentplot/flume");
    let first = origin.tip("main");
    origin.commit("main", "README", "two\n");
    let config = keeping(&["agentplot/flume"]);
    scratch.pass(&config);
    let view = view(&scratch);
    scratch.git(&view, &["switch", "--quiet", "--detach", "HEAD~1"]);
    let commit = origin.commit("main", "README", "three\n");

    let (outcome, lines) = scratch.pass(&config);

    assert_eq!(outcome, Outcome::Kept);
    assert_eq!(lines, moved(&commit, false));
    assert_eq!(head(&scratch, &view), first);
    assert!(detached(&scratch, &view), "the view is on a branch");
}

#[test]
fn back_where_the_runner_put_it() {
    let scratch = Scratch::new();
    let origin = scratch.origin("agentplot/flume");
    let away = origin.tip("main");
    origin.commit("main", "README", "two\n");
    let config = keeping(&["agentplot/flume"]);
    scratch.pass(&config);
    let view = view(&scratch);
    let put = head(&scratch, &view);
    scratch.git(&view, &["switch", "--quiet", "--detach", "HEAD~1"]);
    let commit = origin.commit("main", "README", "three\n");
    let (_, lines) = scratch.pass(&config);
    assert_eq!(lines, moved(&commit, false));
    assert_eq!(head(&scratch, &view), away);

    scratch.git(&view, &["switch", "--quiet", "--detach", &put]);
    let (outcome, lines) = scratch.pass(&config);

    assert_eq!(outcome, Outcome::Kept);
    assert_eq!(
        lines,
        [format!(
            "agentplot/flume: view moved to origin/main at {}",
            &commit[..7]
        )]
    );
    assert_eq!(head(&scratch, &view), commit);
    assert!(detached(&scratch, &view), "the view is on a branch");
}

#[test]
fn an_uncommitted_change() {
    let scratch = Scratch::new();
    let origin = scratch.origin("agentplot/flume");
    let config = keeping(&["agentplot/flume"]);
    scratch.pass(&config);
    let view = view(&scratch);
    let put = head(&scratch, &view);
    std::fs::write(view.join("README"), "mine\n").expect("a change in the view");
    let commit = origin.commit("main", "README", "two\n");

    let (outcome, lines) = scratch.pass(&config);

    assert_eq!(outcome, Outcome::Kept);
    assert_eq!(lines, moved(&commit, false));
    assert_eq!(head(&scratch, &view), put);
    assert_eq!(read(&view.join("README")), "mine\n");
}

#[test]
fn on_a_branch() {
    let scratch = Scratch::new();
    let origin = scratch.origin("agentplot/flume");
    let config = keeping(&["agentplot/flume"]);
    scratch.pass(&config);
    let view = view(&scratch);
    let put = head(&scratch, &view);
    scratch.git(&view, &["switch", "--quiet", "--create", "mine"]);

    for text in ["two\n", "three\n"] {
        let commit = origin.commit("main", "README", text);
        let (outcome, lines) = scratch.pass(&config);

        assert_eq!(outcome, Outcome::Kept);
        assert_eq!(lines, moved(&commit, false));
        assert_eq!(
            scratch.git(&view, &["symbolic-ref", "HEAD"]),
            "refs/heads/mine"
        );
        assert_eq!(head(&scratch, &view), put);
    }
}

#[test]
fn an_untracked_file() {
    let scratch = Scratch::new();
    let origin = scratch.origin("agentplot/flume");
    let config = keeping(&["agentplot/flume"]);
    scratch.pass(&config);
    let view = view(&scratch);
    std::fs::write(view.join("notes"), "mine\n").expect("an untracked file");
    let commit = origin.commit("main", "README", "two\n");

    let (outcome, lines) = scratch.pass(&config);

    assert_eq!(outcome, Outcome::Kept);
    assert_eq!(lines, moved(&commit, true));
    assert_eq!(head(&scratch, &view), commit);
    assert_eq!(read(&view.join("README")), "two\n");
    assert_eq!(read(&view.join("notes")), "mine\n");
}

#[test]
fn the_clone_is_not_under_the_repository_root() {
    let scratch = Scratch::new();
    scratch.origin("agentplot/flume");

    scratch.pass(&keeping(&["agentplot/flume"]));

    let root = scratch.home.join("Code");
    let view = view(&scratch);
    assert_eq!(entries(&root), ["agentplot"]);
    assert_eq!(entries(&root.join("agentplot")), ["flume"]);
    assert!(
        view.join(".git").is_file(),
        "the view holds a repository of its own"
    );
    assert_eq!(
        read(&view.join(".git")),
        format!(
            "gitdir: {}\n",
            scratch
                .clone_of("agentplot/flume")
                .join("worktrees/flume")
                .display()
        )
    );
    assert!(scratch
        .container("agentplot/flume")
        .starts_with(scratch.data()));
    assert!(!scratch.data().starts_with(&root));
}

#[test]
fn origins_default_is_not_main() {
    let scratch = Scratch::new();
    let origin = scratch.origin("agentplot/flume");
    let trunk = origin.commit("trunk", "trunk", "one\n");
    origin.point_head("trunk");

    let (outcome, lines) = scratch.pass(&keeping(&["agentplot/flume"]));

    let view = view(&scratch);
    assert_eq!(outcome, Outcome::Kept);
    assert_eq!(
        lines,
        [
            "agentplot/flume: cloned from https://github.com/agentplot/flume.git".to_string(),
            made(&view, "trunk", &trunk),
        ]
    );
    assert_eq!(
        scratch.git(
            &scratch.clone_of("agentplot/flume"),
            &["symbolic-ref", "HEAD"]
        ),
        "refs/heads/trunk"
    );
    assert_eq!(head(&scratch, &view), trunk);
    assert!(detached(&scratch, &view), "the view is on a branch");
}

#[test]
fn an_empty_origin() {
    let scratch = Scratch::new();
    let origin = scratch.empty_origin("agentplot/flume");
    let config = keeping(&["agentplot/flume"]);

    let (outcome, lines) = scratch.pass(&config);

    let view = view(&scratch);
    assert_eq!(outcome, Outcome::Kept);
    assert_eq!(
        lines,
        [
            "agentplot/flume: cloned from https://github.com/agentplot/flume.git",
            "agentplot/flume: origin names no default branch, so it has no view",
        ]
    );
    assert!(!scratch.home.join("Code").exists(), "a view was made");

    let (_, lines) = scratch.pass(&config);
    assert_eq!(
        lines,
        ["agentplot/flume: origin names no default branch, so it has no view"]
    );

    let commit = origin.commit("main", "README", "one\n");
    let (outcome, lines) = scratch.pass(&config);

    assert_eq!(outcome, Outcome::Kept);
    assert_eq!(
        lines,
        [
            "agentplot/flume: HEAD names main, origin's default branch".to_string(),
            made(&view, "main", &commit),
        ]
    );
    assert_eq!(head(&scratch, &view), commit);
    assert!(detached(&scratch, &view), "the view is on a branch");
}

/// A configuration keeping `agentplot/flume` with the repository root
/// `~/src`.
fn in_src() -> String {
    format!(
        "repository_root = \"~/src\"\n{}",
        keeping(&["agentplot/flume"])
    )
}

/// `agentplot/flume`'s view under the repository root `~/src`.
fn src_view(scratch: &Scratch) -> PathBuf {
    scratch.home.join("src/agentplot/flume")
}

/// The worktrees of `agentplot/flume`'s clone other than itself.
fn worktrees(scratch: &Scratch) -> Vec<String> {
    let bare = scratch.clone_of("agentplot/flume");
    scratch
        .git(&bare, &["worktree", "list", "--porcelain"])
        .lines()
        .filter_map(|line| line.strip_prefix("worktree "))
        .filter(|path| Path::new(path) != bare)
        .map(str::to_string)
        .collect()
}

#[test]
fn a_new_root_the_old_view_clean() {
    let scratch = Scratch::new();
    let origin = scratch.origin("agentplot/flume");
    scratch.pass(&keeping(&["agentplot/flume"]));
    let old = view(&scratch);

    let (outcome, lines) = scratch.pass(&in_src());

    let new = src_view(&scratch);
    let tip = origin.tip("main");
    assert_eq!(outcome, Outcome::Kept);
    assert_eq!(
        lines,
        [
            format!("agentplot/flume: view removed from {}", old.display()),
            made(&new, "main", &tip),
        ]
    );
    assert!(!old.exists(), "the old view is left");
    assert_eq!(entries(&scratch.home.join("Code")), [] as [&str; 0]);
    assert_eq!(head(&scratch, &new), tip);
    assert!(detached(&scratch, &new), "the view is on a branch");
    assert_eq!(worktrees(&scratch), [new.to_string_lossy()]);
    assert_eq!(
        scratch.git(
            &scratch.clone_of("agentplot/flume"),
            &["config", "boomdrive.view"]
        ),
        new.to_string_lossy()
    );

    let (outcome, lines) = scratch.pass(&in_src());
    assert_eq!(outcome, Outcome::Kept);
    assert_eq!(lines, Vec::<String>::new());
}

/// A clone made before the rename, whose `main` is guarded as sluice
/// guards a branch it keeps and lags origin's, under a repository root
/// that has changed: the old view is removed and the new one made at
/// origin's tip, and nothing is written to the guarded branch.
#[test]
fn a_new_root_while_sluice_keeps_the_default_branch() {
    let scratch = Scratch::new();
    let origin = scratch.origin("agentplot/flume");
    scratch.pass(&keeping(&["agentplot/flume"]));
    let old = view(&scratch);
    let bare = scratch.clone_of("agentplot/flume");
    let put = scratch.git(&bare, &["rev-parse", "refs/boomdrive/view"]);
    scratch.git(
        &bare,
        &[
            "config",
            "--local",
            "--rename-section",
            "boomdrive",
            "flywheel",
        ],
    );
    scratch.git(&bare, &["update-ref", "refs/flywheel/view", &put]);
    scratch.git(&bare, &["update-ref", "-d", "refs/boomdrive/view"]);
    let refused = scratch.guard(&bare, "main", false);
    let tip = origin.commit("main", "README", "two\n");

    let (outcome, lines) = scratch.pass(&in_src());

    let new = src_view(&scratch);
    assert_eq!(outcome, Outcome::Kept);
    assert_eq!(
        lines,
        [
            format!("agentplot/flume: view removed from {}", old.display()),
            made(&new, "main", &tip),
        ]
    );
    assert!(!old.exists(), "the old view is left");
    assert_eq!(head(&scratch, &new), tip);
    assert_eq!(worktrees(&scratch), [new.to_string_lossy()]);
    assert_eq!(scratch.git(&bare, &["rev-parse", "main"]), put);
    assert_eq!(
        std::fs::read_to_string(&refused).expect("the guard's file"),
        "",
        "the runner wrote to the guarded branch"
    );
}

#[test]
fn a_new_root_the_old_view_holding_work() {
    let scratch = Scratch::new();
    scratch.origin("agentplot/flume");
    scratch.pass(&keeping(&["agentplot/flume"]));
    let old = view(&scratch);
    let put = head(&scratch, &old);
    std::fs::write(old.join("README"), "mine\n").expect("a change in the view");
    let new = src_view(&scratch);
    let left = format!(
        "agentplot/flume: view left at {}, so none is made at {}: \
         held by the view's changes in 1 file",
        old.display(),
        new.display()
    );

    for _ in 0..2 {
        let (outcome, lines) = scratch.pass(&in_src());

        assert_eq!(outcome, Outcome::Kept);
        assert_eq!(lines, std::slice::from_ref(&left));
        assert_eq!(head(&scratch, &old), put);
        assert_eq!(read(&old.join("README")), "mine\n");
        assert!(!new.exists(), "a view is made at the new root");
        assert!(!scratch.home.join("src").exists());
    }
}

#[test]
fn a_new_root_the_old_folder_replaced() {
    let scratch = Scratch::new();
    let origin = scratch.origin("agentplot/flume");
    scratch.pass(&keeping(&["agentplot/flume"]));
    let old = view(&scratch);
    std::fs::remove_dir_all(&old).expect("the old view removed");
    std::fs::create_dir_all(&old).expect("a folder of the user's");
    std::fs::write(old.join("notes"), "mine\n").expect("a file of the user's");

    let (outcome, lines) = scratch.pass(&in_src());

    let new = src_view(&scratch);
    let tip = origin.tip("main");
    assert_eq!(outcome, Outcome::Kept);
    assert_eq!(lines, [made(&new, "main", &tip)]);
    assert_eq!(entries(&old), ["notes"]);
    assert_eq!(read(&old.join("notes")), "mine\n");
    assert_eq!(head(&scratch, &new), tip);
    assert!(detached(&scratch, &new), "the view is on a branch");
    assert_eq!(
        scratch.git(
            &scratch.clone_of("agentplot/flume"),
            &["config", "boomdrive.view"]
        ),
        new.to_string_lossy()
    );

    let (outcome, lines) = scratch.pass(&in_src());
    assert_eq!(outcome, Outcome::Kept);
    assert_eq!(lines, Vec::<String>::new());
}

#[test]
fn a_new_root_the_old_owners_workspace() {
    let scratch = Scratch::new();
    let origin = scratch.origin("agentplot/flume");
    let asking = |root: &str| {
        format!(
            "code_workspaces = true\nrepository_root = \"{root}\"\n{}",
            keeping(&["agentplot/flume"])
        )
    };
    scratch.pass(&asking("~/Code"));
    let old = view(&scratch);
    let owner = scratch.home.join("Code/agentplot");
    let file = owner.join("agentplot.code-workspace");
    assert_eq!(entries(&owner), ["agentplot.code-workspace", "flume"]);

    let (outcome, lines) = scratch.pass(&asking("~/src"));

    let new = src_view(&scratch);
    let tip = origin.tip("main");
    let new_file = scratch.home.join("src/agentplot/agentplot.code-workspace");
    assert_eq!(outcome, Outcome::Kept);
    assert_eq!(
        lines,
        [
            format!("agentplot/flume: view removed from {}", old.display()),
            format!(
                "boomdrive: workspace removed from {}, with its folder",
                file.display()
            ),
            made(&new, "main", &tip),
            format!(
                "boomdrive: workspace written at {}: flume",
                new_file.display()
            ),
        ]
    );
    assert!(!owner.exists(), "the old owner's folder is left");
    assert_eq!(entries(&scratch.home.join("Code")), [] as [&str; 0]);
    assert_eq!(
        serde_json::from_str::<serde_json::Value>(&read(&new_file)).expect("JSON"),
        json!({"folders": [{"path": "flume"}], "settings": {}})
    );

    let (outcome, lines) = scratch.pass(&asking("~/src"));
    assert_eq!(outcome, Outcome::Kept);
    assert_eq!(lines, Vec::<String>::new());
}
