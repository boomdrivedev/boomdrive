//! Each kept repository's clone, made whole or not at all in the
//! runner's data directory, fetched over https, its HEAD on origin's
//! default branch and no branch of its own moved (the clones spec;
//! design 3 to 6; core.14).

mod common;
mod fakes;
mod github;

use boomdrive::forge::Forge;
use boomdrive::pass::Outcome;
use common::{keeping, Scratch};
use github::{GitHub, Key};
use std::path::Path;
use std::time::{Duration, Instant};

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

/// The words of a fetch stopped at a 500 ms limit (record 0042).
const STOPPED: &str =
    "git fetch with origin was still going after 500ms, the most a try may take, and was stopped";

/// The clones spec's "A fetch whose origin never answers": the fetch is
/// stopped at its limit and named, and the pass keeps the other
/// repository.
#[test]
fn a_fetch_whose_origin_never_answers() {
    let scratch = Scratch::new();
    scratch.origin("agentplot/flume");
    let sluice = scratch.origin("agentplot/sluice");
    let config = keeping(&["agentplot/flume", "agentplot/sluice"]);
    scratch.pass(&config);
    scratch.silent("agentplot/flume");
    let commit = sluice.commit("main", "README", "two\n");

    let started = Instant::now();
    let (outcome, lines) = scratch.pass_trying_for(&config, Duration::from_millis(500));
    let took = started.elapsed();

    assert!(took < Duration::from_secs(5), "the pass took {took:?}");
    assert_eq!(outcome, Outcome::Failed);
    assert_eq!(
        lines,
        [
            format!("agentplot/flume: not fetched: {STOPPED}"),
            view_moved("agentplot/sluice", "main", &commit),
        ]
    );
    assert_eq!(
        scratch.git(
            &scratch.clone_of("agentplot/sluice"),
            &["rev-parse", "refs/remotes/origin/main"]
        ),
        commit
    );
}

/// The clones spec's "A clone whose origin never answers": the new
/// clone's fetch is stopped at its limit and named, nothing of it is left,
/// and the pass keeps the other repository.
#[test]
fn a_clone_whose_origin_never_answers() {
    let scratch = Scratch::new();
    scratch.silent("agentplot/flume");
    let sluice = scratch.origin("agentplot/sluice");
    let config = keeping(&["agentplot/flume", "agentplot/sluice"]);

    let started = Instant::now();
    let (outcome, lines) = scratch.pass_trying_for(&config, Duration::from_millis(500));
    let took = started.elapsed();

    assert!(took < Duration::from_secs(5), "the pass took {took:?}");
    assert_eq!(outcome, Outcome::Failed);
    assert_eq!(
        lines,
        [
            format!("agentplot/flume: not cloned: {STOPPED}"),
            "agentplot/sluice: cloned from https://github.com/agentplot/sluice.git".to_string(),
            view_made(&scratch, "agentplot/sluice", &sluice.tip("main")),
        ]
    );
    assert_eq!(
        entries(&scratch.data().join("repositories/agentplot")),
        ["sluice"]
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

/// `config` naming the App `1`, its key at `key_file`, written as the
/// configuration names it.
fn with_the_app(config: &str, key_file: &str) -> String {
    format!("{config}\n[forge]\napp_id = 1\nkey_file = {key_file:?}\n")
}

/// The clones spec's "A repository the App is not installed on": it is
/// fetched with no credential, and no token is bought.
#[test]
fn a_repository_the_app_is_not_installed_on_is_fetched() {
    let scratch = Scratch::new();
    let origin = scratch.origin("agentplot/flume");
    let commit = origin.tip("main");
    Key::App.write(&scratch.home, 0o600);
    let github = GitHub::start();
    let forge = Forge::at(&github.url).calling_for(Duration::from_secs(5));
    let config = with_the_app(
        &keeping(&["agentplot/flume"]),
        "~/.config/boomdrive/app.pem",
    );

    let (outcome, lines) = scratch.pass_with_forge(&config, &forge);
    let landed = origin.commit("main", "README", "two\n");
    let (again, moved) = scratch.pass_with_forge(&config, &forge);

    assert_eq!((outcome, again), (Outcome::Kept, Outcome::Kept));
    assert_eq!(
        lines,
        [
            "agentplot/flume: cloned from https://github.com/agentplot/flume.git".to_string(),
            view_made(&scratch, "agentplot/flume", &commit)
        ]
    );
    assert_eq!(moved, [view_moved("agentplot/flume", "main", &landed)]);
    assert!(github.tokens().is_empty());
    assert_eq!(github.calls(), ["GET /repos/agentplot/flume/installation"]);
}

/// The clones spec's "A mint refused is named after git's words".
#[test]
fn a_mint_refused_is_named_after_gits_words() {
    let scratch = Scratch::new();
    let flume = scratch.origin("agentplot/flume");
    let github = GitHub::start();
    github.install("agentplot/flume", 7);
    let forge = Forge::at(&github.url).calling_for(Duration::from_secs(5));
    let key_file = scratch.home.join(".config/boomdrive/app.pem");
    let config = with_the_app(&keeping(&["agentplot/flume"]), &key_file.to_string_lossy());
    let (outcome, _) = scratch.pass_with_forge(&config, &forge);
    assert_eq!(outcome, Outcome::Kept);
    std::fs::rename(&flume.bare, flume.bare.with_extension("gone")).expect("origin taken away");

    let (outcome, lines) = scratch.pass_with_forge(&config, &forge);

    assert_eq!(outcome, Outcome::Failed);
    assert_eq!(lines.len(), 1, "{lines:?}");
    let refused = format!(
        "; no forge token: [forge] key_file {} cannot be read: ",
        key_file.display()
    );
    let (gits, mints) = lines[0].split_once(&refused).expect("the mint's words");
    assert!(
        gits.starts_with("agentplot/flume: not fetched: ") && gits.len() > 30,
        "{lines:?}"
    );
    assert!(!mints.is_empty(), "{lines:?}");
    assert!(github.requests().is_empty());
}

/// The forge-tokens spec's "A token kept across passes".
#[test]
fn two_passes_buy_one_token() {
    let scratch = Scratch::new();
    let origin = scratch.origin("agentplot/flume");
    Key::App.write(&scratch.home, 0o600);
    let github = GitHub::start();
    github.install("agentplot/flume", 7);
    let forge = Forge::at(&github.url).calling_for(Duration::from_secs(5));
    let config = with_the_app(
        &keeping(&["agentplot/flume"]),
        "~/.config/boomdrive/app.pem",
    );

    let (outcome, _) = scratch.pass_with_forge(&config, &forge);
    let landed = origin.commit("main", "README", "two\n");
    let (again, lines) = scratch.pass_with_forge(&config, &forge);

    assert_eq!((outcome, again), (Outcome::Kept, Outcome::Kept));
    assert_eq!(lines, [view_moved("agentplot/flume", "main", &landed)]);
    assert_eq!(github.tokens().len(), 1);
    assert_eq!(
        github.calls(),
        [
            "GET /repos/agentplot/flume/installation",
            "POST /app/installations/7/access_tokens"
        ]
    );
}

/// The forge-tokens spec's "A failed fetch drops its token": the pass
/// after a failed fetch buys a token again.
#[test]
fn a_failed_fetch_drops_its_token() {
    let scratch = Scratch::new();
    let flume = scratch.origin("agentplot/flume");
    Key::App.write(&scratch.home, 0o600);
    let github = GitHub::start();
    github.install("agentplot/flume", 7);
    let forge = Forge::at(&github.url).calling_for(Duration::from_secs(5));
    let config = with_the_app(
        &keeping(&["agentplot/flume"]),
        "~/.config/boomdrive/app.pem",
    );
    let (outcome, _) = scratch.pass_with_forge(&config, &forge);
    assert_eq!(outcome, Outcome::Kept);
    std::fs::rename(&flume.bare, flume.bare.with_extension("gone")).expect("origin taken away");

    let (failed, lines) = scratch.pass_with_forge(&config, &forge);
    let bought = github.tokens().len();
    std::fs::rename(flume.bare.with_extension("gone"), &flume.bare).expect("origin back");
    let (kept, _) = scratch.pass_with_forge(&config, &forge);

    assert_eq!((failed, kept), (Outcome::Failed, Outcome::Kept));
    assert!(
        lines[0].starts_with("agentplot/flume: not fetched: "),
        "{lines:?}"
    );
    assert!(!lines[0].contains("no forge token"), "{lines:?}");
    assert_eq!(bought, 1);
    assert_eq!(github.tokens().len(), 2);
}

/// A configuration keeping `agentplot/flume` in the drive `home`, its
/// control plane the hosted one at `url`, whose partition `home` is served
/// `agentplot/flume`: a cloud host's.
fn on_a_cloud_host(url: &str) -> String {
    format!(
        "{}\naccounts.own.control_plane = \"hosted\"\n[control_plane]\n\n[control_plane.hosted]\nurl = {url:?}\n\n\
         [[control_plane.hosted.partitions]]\nlabel = \"home\"\n\
         repositories = [\"agentplot/flume\"]\n",
        keeping(&["agentplot/flume"])
    )
}

/// A forge asking `tokens`, as a cloud host's runner makes it (design
/// Decision 8 of a-cloud-host-gets-its-github-tokens-from-the-cloud).
fn asking(tokens: &boomdrive_control::fake::Tokens) -> Forge {
    Forge::asking(Box::new(tokens.clone()))
}

/// The control plane gives `agentplot/flume` a token ending in an hour.
fn the_cloud_gives_flume(tokens: &boomdrive_control::fake::Tokens) {
    tokens.set(
        "agentplot/flume",
        boomdrive_control::TokenAsked::Token(boomdrive_control::ForgeToken::new(
            "ghs_cloud",
            time::OffsetDateTime::now_utc() + Duration::from_secs(60 * 60),
        )),
    );
}

/// The clones spec's "A cloud host's fetch given the control plane's
/// token": `agentplot/flume`, whose origin needs no credential, is cloned,
/// fetched and its view moved, through the control plane's token, which
/// the control plane was asked for.
#[test]
fn a_cloud_hosts_fetch_is_given_the_control_planes_token() {
    let scratch = Scratch::new();
    let origin = scratch.origin("agentplot/flume");
    let tokens = boomdrive_control::fake::Tokens::new();
    the_cloud_gives_flume(&tokens);
    let forge = asking(&tokens);
    let config = on_a_cloud_host("https://control.example");

    let (outcome, _) = scratch.pass_with_forge(&config, &forge);
    let landed = origin.commit("main", "README", "two\n");
    let (again, lines) = scratch.pass_with_forge(&config, &forge);

    assert_eq!((outcome, again), (Outcome::Kept, Outcome::Kept));
    assert_eq!(lines, [view_moved("agentplot/flume", "main", &landed)]);
    assert_eq!(tokens.asks("agentplot/flume"), 1);
}

/// The clones spec's "A repository the App's installation does not cover":
/// the control plane says so, and `agentplot/flume` is cloned, fetched and
/// its view moved with no credential, and no line names a forge token.
#[test]
fn a_repository_the_apps_installation_does_not_cover_on_a_cloud_host() {
    let scratch = Scratch::new();
    let origin = scratch.origin("agentplot/flume");
    let commit = origin.tip("main");
    let tokens = boomdrive_control::fake::Tokens::new();
    tokens.set(
        "agentplot/flume",
        boomdrive_control::TokenAsked::NotInstalled(
            "`home`: the installation does not cover agentplot/flume".to_string(),
        ),
    );
    let forge = asking(&tokens);
    let config = on_a_cloud_host("https://control.example");

    let (outcome, lines) = scratch.pass_with_forge(&config, &forge);
    let landed = origin.commit("main", "README", "two\n");
    let (again, moved) = scratch.pass_with_forge(&config, &forge);

    assert_eq!((outcome, again), (Outcome::Kept, Outcome::Kept));
    assert_eq!(
        lines,
        [
            "agentplot/flume: cloned from https://github.com/agentplot/flume.git".to_string(),
            view_made(&scratch, "agentplot/flume", &commit)
        ]
    );
    assert_eq!(moved, [view_moved("agentplot/flume", "main", &landed)]);
    assert!(
        lines
            .iter()
            .chain(&moved)
            .all(|line| !line.contains("forge token")),
        "{lines:?} {moved:?}"
    );
    assert_eq!(tokens.asks("agentplot/flume"), 1);
}

/// The forge-tokens spec's "A cloud host's failed fetch drops its token":
/// the pass after a failed fetch asks the control plane again.
#[test]
fn a_cloud_hosts_failed_fetch_drops_its_token() {
    let scratch = Scratch::new();
    let flume = scratch.origin("agentplot/flume");
    let tokens = boomdrive_control::fake::Tokens::new();
    the_cloud_gives_flume(&tokens);
    let forge = asking(&tokens);
    let config = on_a_cloud_host("https://control.example");
    let (outcome, _) = scratch.pass_with_forge(&config, &forge);
    assert_eq!(outcome, Outcome::Kept);
    std::fs::rename(&flume.bare, flume.bare.with_extension("gone")).expect("origin taken away");

    let (failed, lines) = scratch.pass_with_forge(&config, &forge);
    let asked = tokens.asks("agentplot/flume");
    std::fs::rename(flume.bare.with_extension("gone"), &flume.bare).expect("origin back");
    let (kept, _) = scratch.pass_with_forge(&config, &forge);

    assert_eq!((failed, kept), (Outcome::Failed, Outcome::Kept));
    assert!(
        lines[0].starts_with("agentplot/flume: not fetched: "),
        "{lines:?}"
    );
    assert!(!lines[0].contains("no forge token"), "{lines:?}");
    assert_eq!(asked, 1);
    assert_eq!(tokens.asks("agentplot/flume"), 2);
}

/// The clones spec's "The running runner asks its control plane": `boomdrive
/// run --once` on the scratch host, its stub Flume, sluice and
/// `systemd-creds` first on its `PATH`, so no real store is reached (Task
/// notes 4.3), the hosted control plane at a loopback port with nothing on
/// it, its partition `home` served `agentplot/flume` with no key of `home`
/// in the host's store, and the fetch of `agentplot/flume` failing: its line
/// gives git's words, then `; no forge token: ` and words naming `boomdrive
/// enroll --control-plane <url> --drive home --repository <each>`.
#[test]
fn the_running_runner_asks_its_control_plane() {
    let scratch = Scratch::new();
    let flume = scratch.origin("agentplot/flume");
    let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("a loopback port");
    let port = listener.local_addr().expect("its address").port();
    drop(listener);
    let config = on_a_cloud_host(&format!("http://127.0.0.1:{port}/boomdrive"));
    let tokens = boomdrive_control::fake::Tokens::new();
    let (outcome, _) = scratch.pass_with_forge(&config, &asking(&tokens));
    assert_eq!(outcome, Outcome::Kept);
    std::fs::rename(&flume.bare, flume.bare.with_extension("gone")).expect("origin taken away");
    scratch.configure(&config);
    let (flume_stub, sluice, creds) = (
        fakes::Stub::new(),
        fakes::SluiceStub::new(),
        fakes::CredsStub::new(),
    );
    let rest = std::env::var_os("PATH").unwrap_or_default();
    let path = std::env::join_paths(
        [&flume_stub.dir, &sluice.dir, &creds.dir]
            .map(|dir| dir.path().to_path_buf())
            .into_iter()
            .chain(std::env::split_paths(&rest)),
    )
    .expect("a PATH");

    let output = scratch
        .boomdrive()
        .args(["run", "--once"])
        .env("PATH", path)
        .output()
        .expect("boomdrive runs");

    let said = String::from_utf8_lossy(&output.stderr).into_owned();
    let line = said
        .lines()
        .find(|line| line.contains("agentplot/flume: not fetched: "))
        .unwrap_or_else(|| panic!("no line of the fetch: {said}"));
    let (gits, asks) = line
        .split_once("; no forge token: ")
        .unwrap_or_else(|| panic!("no ask's words: {line}"));
    assert!(
        gits.len() > "agentplot/flume: not fetched: ".len(),
        "{line}"
    );
    assert!(asks.contains("--drive home --repository "), "{line}");
    assert!(!creds.kept_or_read(), "{:?}", creds.calls());
}
