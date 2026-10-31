//! Each kept repository's clone, made whole or not at all in the
//! runner's data directory, fetched over https, its HEAD on origin's
//! default branch and its own default branch levelled (the clones spec;
//! design 3 to 6).

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
    assert_eq!(
        lines,
        [format!(
            "agentplot/flume: main moved to origin/main at {}",
            &commit[..7]
        )]
    );
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
        lines[1],
        format!(
            "agentplot/sluice: main moved to origin/main at {}",
            &commit[..7]
        )
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
    origin.commit("main", "README", "two\n");
    let second = origin.commit("main", "README", "three\n");

    let (outcome, lines) = scratch.pass(&config);

    assert_eq!(outcome, Outcome::Kept);
    assert_eq!(
        lines,
        [format!(
            "agentplot/flume: main moved to origin/main at {}",
            &second[..7]
        )]
    );
    let bare = scratch.clone_of("agentplot/flume");
    assert_eq!(scratch.git(&bare, &["rev-parse", "main"]), second);
    assert_eq!(
        scratch.git(&bare, &["rev-parse", "refs/remotes/origin/main"]),
        second
    );
}

#[test]
fn held_by_a_worktree() {
    let scratch = Scratch::new();
    let origin = scratch.origin("agentplot/flume");
    let config = keeping(&["agentplot/flume"]);
    scratch.pass(&config);
    let bare = scratch.clone_of("agentplot/flume");
    let held = scratch.root.join("held");
    scratch.git(
        &bare,
        &[
            "worktree",
            "add",
            "--quiet",
            &held.to_string_lossy(),
            "main",
        ],
    );
    let before = scratch.git(&bare, &["rev-parse", "main"]);
    origin.commit("main", "README", "two\n");

    let (outcome, lines) = scratch.pass(&config);

    assert_eq!(outcome, Outcome::Kept);
    assert_eq!(
        lines,
        [format!(
            "agentplot/flume: main left: checked out in the worktree {}",
            held.display()
        )]
    );
    assert_eq!(scratch.git(&bare, &["rev-parse", "main"]), before);
}

#[test]
fn ahead_of_origin() {
    let scratch = Scratch::new();
    scratch.origin("agentplot/flume");
    let config = keeping(&["agentplot/flume"]);
    scratch.pass(&config);
    let bare = scratch.clone_of("agentplot/flume");
    let ahead = scratch.git(
        &bare,
        &["commit-tree", "-p", "main", "-m", "ahead", "main^{tree}"],
    );
    scratch.git(&bare, &["update-ref", "refs/heads/main", &ahead]);

    let (outcome, lines) = scratch.pass(&config);

    assert_eq!(outcome, Outcome::Kept);
    assert_eq!(
        lines,
        ["agentplot/flume: main left: 1 commit ahead of origin/main"]
    );
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
        ["agentplot/flume: HEAD names trunk, origin's default branch"]
    );
    let bare = scratch.clone_of("agentplot/flume");
    assert_eq!(
        scratch.git(&bare, &["symbolic-ref", "HEAD"]),
        "refs/heads/trunk"
    );
    assert_eq!(scratch.git(&bare, &["rev-parse", "trunk"]), trunk);
}
