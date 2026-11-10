//! Each kept repository's clone, made whole or not at all in the
//! runner's data directory, fetched over https, its HEAD on origin's
//! default branch and no branch of its own moved (the clones spec;
//! design 3 to 6; core.14).

mod common;

use boomdrive::pass::Outcome;
use common::{keeping, Scratch};
use std::path::Path;

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

/// The line a pass says when it makes `address`'s view under the default
/// repository root, at `origin/main`'s `commit`.
fn view_made(scratch: &Scratch, address: &str, commit: &str) -> String {
    format!(
        "{address}: view made at {}, at origin/main {}",
        scratch.home.join("Code").join(address).display(),
        &commit[..7]
    )
}

/// The line a pass says when it moves `address`'s view to
/// `origin/<default>`'s `commit`.
fn view_moved(address: &str, default: &str, commit: &str) -> String {
    format!(
        "{address}: view moved to origin/{default} at {}",
        &commit[..7]
    )
}

#[test]
fn a_new_repository_is_cloned() {
    let scratch = Scratch::new();
    let origin = scratch.origin("agentplot/flume");
    let fix = origin.commit("fix/one", "fix", "one\n");

    let (outcome, lines) = scratch.pass(&keeping(&["agentplot/flume"]));

    assert_eq!(outcome, Outcome::Kept);
    assert_eq!(
        lines,
        [
            "agentplot/flume: cloned from https://github.com/agentplot/flume.git".to_string(),
            view_made(&scratch, "agentplot/flume", &origin.tip("main")),
        ]
    );
    let container = scratch.container("agentplot/flume");
    let bare = scratch.clone_of("agentplot/flume");
    assert_eq!(entries(&container), [".git", "flume.git"]);
    assert_eq!(
        std::fs::read_to_string(container.join(".git")).expect("the gitfile"),
        "gitdir: ./flume.git\n"
    );
    assert_eq!(
        scratch.git(&bare, &["rev-parse", "--is-bare-repository"]),
        "true"
    );
    assert_eq!(
        scratch.git(&bare, &["config", "remote.origin.url"]),
        "https://github.com/agentplot/flume.git"
    );
    assert_eq!(
        scratch.git(
            &bare,
            &[
                "for-each-ref",
                "--format=%(refname) %(objectname)",
                "refs/remotes/origin"
            ]
        ),
        format!(
            "refs/remotes/origin/HEAD {main}\n\
             refs/remotes/origin/fix/one {fix}\n\
             refs/remotes/origin/main {main}",
            main = origin.tip("main")
        )
    );
    assert_eq!(
        scratch.git(
            &bare,
            &["for-each-ref", "--format=%(refname)", "refs/heads"]
        ),
        "refs/heads/main",
        "origin's branches are copied as the clone's own"
    );
    assert_eq!(
        scratch.git(&bare, &["symbolic-ref", "HEAD"]),
        "refs/heads/main"
    );
    assert_eq!(
        scratch.git(&bare, &["rev-parse", "main"]),
        origin.tip("main")
    );
    assert_eq!(
        entries(&scratch.data().join("repositories/agentplot")),
        ["flume"]
    );
}

#[test]
fn origin_cannot_be_reached() {
    let scratch = Scratch::new();

    let (outcome, lines) = scratch.pass(&keeping(&["agentplot/flume"]));
    assert_eq!(outcome, Outcome::Failed);
    assert_eq!(lines.len(), 1, "{lines:?}");
    assert!(
        lines[0].starts_with("agentplot/flume: not cloned: "),
        "{lines:?}"
    );
    assert_eq!(
        entries(&scratch.data().join("repositories")),
        Vec::<String>::new(),
        "a failed clone left something"
    );

    let sluice = scratch.origin("agentplot/sluice");
    let both = keeping(&["agentplot/flume", "agentplot/sluice"]);
    let (outcome, lines) = scratch.pass(&both);
    assert_eq!(outcome, Outcome::Failed);
    assert!(
        lines[0].starts_with("agentplot/flume: not cloned: "),
        "{lines:?}"
    );
    assert_eq!(
        lines[1..],
        [
            "agentplot/sluice: cloned from https://github.com/agentplot/sluice.git".to_string(),
            view_made(&scratch, "agentplot/sluice", &sluice.tip("main")),
        ]
    );
    assert_eq!(
        entries(&scratch.data().join("repositories/agentplot")),
        ["sluice"]
    );
    assert_eq!(
        scratch.git(
            &scratch.clone_of("agentplot/sluice"),
            &["rev-parse", "main"]
        ),
        sluice.tip("main")
    );

    let flume = scratch.origin("agentplot/flume");
    let (outcome, lines) = scratch.pass(&both);
    assert_eq!(outcome, Outcome::Kept);
    assert_eq!(
        lines,
        [
            "agentplot/flume: cloned from https://github.com/agentplot/flume.git".to_string(),
            view_made(&scratch, "agentplot/flume", &flume.tip("main")),
        ]
    );
    assert_eq!(
        scratch.git(&scratch.clone_of("agentplot/flume"), &["rev-parse", "main"]),
        flume.tip("main")
    );
}

#[test]
fn a_folder_already_there() {
    let scratch = Scratch::new();
    scratch.origin("agentplot/flume");
    let container = scratch.container("agentplot/flume");
    std::fs::create_dir_all(container.join("flume.git")).expect("a folder of someone's");
    std::fs::write(container.join("notes"), "mine\n").expect("a file of someone's");

    let (outcome, lines) = scratch.pass(&keeping(&["agentplot/flume"]));

    assert_eq!(outcome, Outcome::Kept);
    assert_eq!(
        lines,
        [format!(
            "agentplot/flume: left: {} is not a clone the runner made, so none is made",
            container.display()
        )]
    );
    assert_eq!(entries(&container), ["flume.git", "notes"]);
    assert_eq!(entries(&container.join("flume.git")), Vec::<String>::new());
    assert_eq!(
        std::fs::read_to_string(container.join("notes")).expect("the file"),
        "mine\n"
    );
    assert_eq!(
        entries(&scratch.data().join("repositories/agentplot")),
        ["flume"]
    );
    assert!(
        !scratch.home.join("Code").exists(),
        "a view was made of a repository with no clone"
    );
}

/// A bare repository that records its address under another name than
/// the runner's, `other.address`, is a folder the runner did not make: it
/// is left as it is and named, and no clone or view is made over it
/// (record 0035; core.14).
#[test]
fn a_clone_recording_its_address_under_another_name() {
    let scratch = Scratch::new();
    scratch.origin("agentplot/flume");
    let container = scratch.container("agentplot/flume");
    std::fs::create_dir_all(&container).expect("the container");
    scratch.git(&container, &["init", "--quiet", "--bare", "flume.git"]);
    let bare = container.join("flume.git");
    scratch.git(
        &bare,
        &["config", "--local", "other.address", "agentplot/flume"],
    );

    let (outcome, lines) = scratch.pass(&keeping(&["agentplot/flume"]));

    assert_eq!(outcome, Outcome::Kept);
    assert_eq!(
        lines,
        [format!(
            "agentplot/flume: left: {} is not a clone the runner made, so none is made",
            container.display()
        )]
    );
    assert_eq!(
        scratch.git(&bare, &["config", "--local", "other.address"]),
        "agentplot/flume"
    );
    assert!(!scratch.holds(&bare, &["config", "--local", "--get", "boomdrive.address"]));
    assert!(
        !scratch.home.join("Code").exists(),
        "a view was made of a repository with no clone"
    );
}

#[test]
fn a_clone_left_unfinished_is_made_again() {
    let scratch = Scratch::new();
    let origin = scratch.origin("agentplot/flume");
    let partial = scratch.data().join("repositories/agentplot/.flume.partial");
    std::fs::create_dir_all(partial.join("flume.git")).expect("a partial clone");

    let (outcome, lines) = scratch.pass(&keeping(&["agentplot/flume"]));

    assert_eq!(outcome, Outcome::Kept);
    assert_eq!(
        lines,
        [
            "agentplot/flume: cloned from https://github.com/agentplot/flume.git".to_string(),
            view_made(&scratch, "agentplot/flume", &origin.tip("main")),
        ]
    );
    assert_eq!(
        entries(&scratch.data().join("repositories/agentplot")),
        ["flume"]
    );
    assert_eq!(
        scratch.git(&scratch.clone_of("agentplot/flume"), &["rev-parse", "main"]),
        origin.tip("main")
    );

    let stale = scratch
        .data()
        .join("repositories/agentplot/.sluice.partial");
    std::fs::create_dir_all(&stale).expect("a partial clone of another");
    let (outcome, lines) = scratch.pass(&keeping(&["agentplot/flume"]));
    assert_eq!(outcome, Outcome::Kept);
    assert_eq!(
        lines,
        [format!(
            "boomdrive: removed {}, a clone left unfinished",
            stale.display()
        )]
    );
    assert_eq!(
        entries(&scratch.data().join("repositories/agentplot")),
        ["flume"]
    );
}

#[test]
fn a_clone_is_found_by_its_record_without_case() {
    let scratch = Scratch::new();
    scratch.origin("agentplot/flume");
    scratch.pass(&keeping(&["agentplot/flume"]));

    let (outcome, lines) = scratch.pass(&keeping(&["AgentPlot/Flume"]));

    assert_eq!(outcome, Outcome::Kept);
    // A view's path is the address as written (design 7), so where the
    // file system tells case apart, the view follows the new spelling.
    assert!(
        lines
            .iter()
            .all(|line| line.starts_with("AgentPlot/Flume: view ")),
        "{lines:?}"
    );
    assert_eq!(entries(&scratch.data().join("repositories")), ["agentplot"]);
}

/// Rewrites the scratch host's git configuration as a host's that sends
/// GitHub's https addresses to ssh, the forge's own rewrite taken out.
fn rewrite_to_ssh(scratch: &Scratch) {
    let config = std::fs::read_to_string(&scratch.gitconfig).expect("the git configuration");
    let forge = format!("[url \"file://{}/\"]", scratch.forge.display());
    assert!(config.contains(&forge), "{config}");
    std::fs::write(
        &scratch.gitconfig,
        config.replace(&forge, "[url \"git@github.com:\"]"),
    )
    .expect("the git configuration written");
}

#[test]
fn the_host_rewrites_https_to_ssh() {
    let scratch = Scratch::new();
    let flume = scratch.origin("agentplot/flume");
    scratch.origin("agentplot/sluice");
    scratch.pass(&keeping(&["agentplot/flume"]));
    let fetched = flume.tip("main");
    flume.commit("main", "README", "two\n");
    rewrite_to_ssh(&scratch);

    let (outcome, lines) = scratch.pass(&keeping(&["agentplot/flume", "agentplot/sluice"]));

    assert_eq!(outcome, Outcome::Failed);
    assert_eq!(
        lines,
        [
            "agentplot/flume: not fetched: git would fetch it by ssh, \
             from git@github.com:agentplot/flume.git",
            "agentplot/sluice: not cloned: git would fetch it by ssh, \
             from git@github.com:agentplot/sluice.git",
        ]
    );
    assert_eq!(
        scratch.git(
            &scratch.clone_of("agentplot/flume"),
            &["rev-parse", "refs/remotes/origin/main"]
        ),
        fetched
    );
    assert_eq!(
        entries(&scratch.data().join("repositories/agentplot")),
        ["flume"]
    );
}

#[test]
fn origin_gains_a_commit() {
    let scratch = Scratch::new();
    let origin = scratch.origin("agentplot/flume");
    let config = keeping(&["agentplot/flume"]);
    scratch.pass(&config);
    let commit = origin.commit("main", "README", "two\n");

    let (outcome, lines) = scratch.pass(&config);

    assert_eq!(outcome, Outcome::Kept);
    assert_eq!(lines, [view_moved("agentplot/flume", "main", &commit)]);
    assert_eq!(
        scratch.git(
            &scratch.clone_of("agentplot/flume"),
            &["rev-parse", "refs/remotes/origin/main"]
        ),
        commit
    );
}

#[test]
fn origin_deletes_a_branch() {
    let scratch = Scratch::new();
    let origin = scratch.origin("agentplot/flume");
    origin.commit("fix/one", "fix", "one\n");
    let config = keeping(&["agentplot/flume"]);
    scratch.pass(&config);
    let bare = scratch.clone_of("agentplot/flume");
    let fix = [
        "rev-parse",
        "--verify",
        "--quiet",
        "refs/remotes/origin/fix/one",
    ];
    assert!(scratch.holds(&bare, &fix));
    origin.delete("fix/one");

    let (outcome, lines) = scratch.pass(&config);

    assert_eq!(outcome, Outcome::Kept);
    assert_eq!(lines, Vec::<String>::new());
    assert!(
        !scratch.holds(&bare, &fix),
        "origin's deleted branch is kept"
    );
}

#[test]
fn one_fetch_fails() {
    let scratch = Scratch::new();
    let flume = scratch.origin("agentplot/flume");
    let sluice = scratch.origin("agentplot/sluice");
    let config = keeping(&["agentplot/flume", "agentplot/sluice"]);
    scratch.pass(&config);
    std::fs::rename(&flume.bare, flume.bare.with_extension("gone")).expect("origin taken away");
    let commit = sluice.commit("main", "README", "two\n");

    let (outcome, lines) = scratch.pass(&config);

    assert_eq!(outcome, Outcome::Failed);
    assert_eq!(lines.len(), 2, "{lines:?}");
    assert!(
        lines[0].starts_with("agentplot/flume: not fetched: "),
        "{lines:?}"
    );
    assert_eq!(
        lines[1..],
        [view_moved("agentplot/sluice", "main", &commit)]
    );
    assert_eq!(
        scratch.git(
            &scratch.clone_of("agentplot/sluice"),
            &["rev-parse", "refs/remotes/origin/main"]
        ),
        commit
    );
}

#[test]
fn behind_origin() {
    let scratch = Scratch::new();
    let origin = scratch.origin("agentplot/flume");
    let config = keeping(&["agentplot/flume"]);
    scratch.pass(&config);
    let bare = scratch.clone_of("agentplot/flume");
    let before = scratch.git(&bare, &["rev-parse", "main"]);
    origin.commit("main", "README", "two\n");
    let second = origin.commit("main", "README", "three\n");

    let (outcome, lines) = scratch.pass(&config);

    assert_eq!(outcome, Outcome::Kept);
    assert_eq!(lines, [view_moved("agentplot/flume", "main", &second)]);
    assert_eq!(scratch.git(&bare, &["rev-parse", "main"]), before);
    assert_eq!(
        scratch.git(&bare, &["rev-parse", "refs/remotes/origin/main"]),
        second
    );
}

/// The clone's `main` guarded as sluice guards a branch it keeps, and
/// behind origin's: the runner tries no write to it, so nothing is
/// refused, and the view still follows origin.
#[test]
fn sluice_keeps_the_default_branch() {
    let scratch = Scratch::new();
    let origin = scratch.origin("agentplot/flume");
    let config = keeping(&["agentplot/flume"]);
    scratch.pass(&config);
    let bare = scratch.clone_of("agentplot/flume");
    let refused = scratch.guard(&bare, "main", false);
    let before = scratch.git(&bare, &["rev-parse", "main"]);
    let commit = origin.commit("main", "README", "two\n");

    let (outcome, lines) = scratch.pass(&config);

    assert_eq!(outcome, Outcome::Kept);
    assert_eq!(lines, [view_moved("agentplot/flume", "main", &commit)]);
    assert_eq!(
        std::fs::read_to_string(&refused).expect("the guard's file"),
        "",
        "the runner wrote to the guarded branch"
    );
    assert_eq!(scratch.git(&bare, &["rev-parse", "main"]), before);
    assert!(
        !scratch.holds(&bare, &["update-ref", "refs/heads/main", &commit]),
        "the guard lets a move through"
    );
    assert_eq!(
        std::fs::read_to_string(&refused)
            .expect("the guard's file")
            .lines()
            .count(),
        1
    );
}

#[test]
fn ahead_of_origin() {
    let scratch = Scratch::new();
    let origin = scratch.origin("agentplot/flume");
    let config = keeping(&["agentplot/flume"]);
    scratch.pass(&config);
    let bare = scratch.clone_of("agentplot/flume");
    let ahead = scratch.git(
        &bare,
        &["commit-tree", "-p", "main", "-m", "ahead", "main^{tree}"],
    );
    scratch.git(&bare, &["update-ref", "refs/heads/main", &ahead]);
    let commit = origin.commit("main", "README", "two\n");

    let (outcome, lines) = scratch.pass(&config);

    assert_eq!(outcome, Outcome::Kept);
    assert_eq!(lines, [view_moved("agentplot/flume", "main", &commit)]);
    assert_eq!(scratch.git(&bare, &["rev-parse", "main"]), ahead);
}

#[test]
fn origin_renames_its_default_branch() {
    let scratch = Scratch::new();
    let origin = scratch.origin("agentplot/flume");
    let config = keeping(&["agentplot/flume"]);
    scratch.pass(&config);
    let trunk = origin.commit("trunk", "trunk", "one\n");
    origin.point_head("trunk");

    let (outcome, lines) = scratch.pass(&config);

    assert_eq!(outcome, Outcome::Kept);
    assert_eq!(
        lines,
        [
            "agentplot/flume: HEAD names trunk, origin's default branch".to_string(),
            view_moved("agentplot/flume", "trunk", &trunk),
        ]
    );
    let bare = scratch.clone_of("agentplot/flume");
    assert_eq!(
        scratch.git(&bare, &["symbolic-ref", "HEAD"]),
        "refs/heads/trunk"
    );
    assert_eq!(scratch.git(&bare, &["rev-parse", "trunk"]), trunk);
}

/// origin's new default branch cannot be made in the clone, a hook
/// refusing it: HEAD is named as not settled and fails the pass, and the
/// view is still kept at origin's tip, as is the next repository.
#[test]
fn a_head_that_cannot_be_settled() {
    let scratch = Scratch::new();
    let origin = scratch.origin("agentplot/flume");
    let sluice = scratch.origin("agentplot/sluice");
    let config = keeping(&["agentplot/flume", "agentplot/sluice"]);
    scratch.pass(&config);
    let bare = scratch.clone_of("agentplot/flume");
    let refused = scratch.guard(&bare, "trunk", true);
    let trunk = origin.commit("trunk", "trunk", "one\n");
    origin.point_head("trunk");
    let landed = sluice.commit("main", "README", "two\n");

    let (outcome, lines) = scratch.pass(&config);

    assert_eq!(outcome, Outcome::Failed);
    assert_eq!(lines.len(), 3, "{lines:?}");
    assert!(
        lines[0].starts_with("agentplot/flume: HEAD not settled on trunk: "),
        "{lines:?}"
    );
    assert_eq!(
        lines[1..],
        [
            view_moved("agentplot/flume", "trunk", &trunk),
            view_moved("agentplot/sluice", "main", &landed),
        ]
    );
    assert!(!scratch.holds(
        &bare,
        &["rev-parse", "--verify", "--quiet", "refs/heads/trunk"]
    ));
    assert_eq!(
        std::fs::read_to_string(&refused)
            .expect("the guard's file")
            .lines()
            .count(),
        1
    );
}
