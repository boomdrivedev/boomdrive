//! Each kept repository's clone, made whole or not at all in the
//! runner's data directory (the clones spec; design 3 and 4).

mod common;

use common::{keeping, Scratch};
use flywheel::pass::Outcome;
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

#[test]
fn a_new_repository_is_cloned() {
    let scratch = Scratch::new();
    let origin = scratch.origin("agentplot/flume");
    let fix = origin.commit("fix/one", "fix", "one\n");

    let (outcome, lines) = scratch.pass(&keeping(&["agentplot/flume"]));

    assert_eq!(outcome, Outcome::Kept);
    assert_eq!(
        lines,
        ["agentplot/flume: cloned from https://github.com/agentplot/flume.git"]
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
        ["agentplot/sluice: cloned from https://github.com/agentplot/sluice.git"]
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
        ["agentplot/flume: cloned from https://github.com/agentplot/flume.git"]
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
        ["agentplot/flume: cloned from https://github.com/agentplot/flume.git"]
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
            "flywheel: removed {}, a clone left unfinished",
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
    assert_eq!(lines, Vec::<String>::new());
    assert_eq!(entries(&scratch.data().join("repositories")), ["agentplot"]);
}
