//! The runner alone packs and prunes the clones it keeps: git's own
//! collection is off in each, and a keeping pass packs a kept clone once
//! what arrived passes git's own bound, dropping only what nothing of the
//! clone reaches past two weeks and touching no ref, log or worktree (the
//! clones spec; record 0051; core.14).

mod common;

use boomdrive::pass::Outcome;
use common::{keeping, Scratch};
use std::collections::BTreeSet;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Stdio;
use std::time::{Duration, SystemTime};

/// git's output from `args` in the bare repository `bare`, `input` on its
/// standard input, trimmed; panics with git's words where it fails.
fn git_with_input(scratch: &Scratch, bare: &Path, args: &[&str], input: &str) -> String {
    let mut child = scratch
        .command("git")
        .arg("--git-dir")
        .arg(bare)
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("git runs");
    let mut stdin = child.stdin.take().expect("git's standard input");
    // Written beside the read of its output, so neither pipe fills.
    let input = input.to_string();
    let writer = std::thread::spawn(move || {
        stdin
            .write_all(input.as_bytes())
            .expect("git's input written")
    });
    let output = child.wait_with_output().expect("git ends");
    writer.join().expect("git's input written");
    assert!(
        output.status.success(),
        "git {} in {}: {}",
        args.join(" "),
        bare.display(),
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8_lossy(&output.stdout).trim().to_string()
}

/// Writes `count` blobs into `bare` as loose objects, by one `git
/// hash-object -w --stdin-paths`, each a file of its own whose text names
/// `tag` and its number, and gives their ids.
fn write_loose(scratch: &Scratch, bare: &Path, tag: &str, count: usize) -> Vec<String> {
    let dir = scratch.root.join("blobs").join(tag);
    std::fs::create_dir_all(&dir).expect("the blobs' folder");
    let mut paths = String::new();
    for number in 0..count {
        let file = dir.join(number.to_string());
        std::fs::write(&file, format!("{tag} {number}\n")).expect("a blob's file");
        paths.push_str(&format!("{}\n", file.display()));
    }
    let ids = git_with_input(
        scratch,
        bare,
        &["hash-object", "-w", "--stdin-paths"],
        &paths,
    );
    ids.lines().map(str::to_string).collect()
}

/// Writes `count` packs into `bare`, each holding one blob of its own,
/// whose text names `tag` and its number, by one `git pack-objects` each.
fn write_packs(scratch: &Scratch, bare: &Path, tag: &str, count: usize) {
    let base = bare.join("objects/pack/pack");
    for id in write_loose(scratch, bare, tag, count) {
        git_with_input(
            scratch,
            bare,
            &["pack-objects", "-q", &base.to_string_lossy()],
            &format!("{id}\n"),
        );
    }
}

/// The file of the loose object `id` in `bare`.
fn loose_file(bare: &Path, id: &str) -> PathBuf {
    bare.join("objects").join(&id[..2]).join(&id[2..])
}

/// Sets the file time of the loose object `id` in `bare` back by `days`.
fn age(bare: &Path, id: &str, days: u64) {
    let when = SystemTime::now() - Duration::from_secs(days * 24 * 60 * 60);
    std::fs::File::open(loose_file(bare, id))
        .expect("the loose object's file")
        .set_modified(when)
        .expect("the loose object's time set back");
}

/// A clone's counts as `git count-objects -v` gives them: its loose
/// objects and its packs.
fn counts(scratch: &Scratch, bare: &Path) -> (u64, u64) {
    let said = scratch.git(bare, &["count-objects", "-v"]);
    let value = |key: &str| {
        said.lines()
            .find_map(|line| line.strip_prefix(key))
            .and_then(|value| value.trim().parse().ok())
            .unwrap_or_else(|| panic!("count-objects gives {key}: {said}"))
    };
    (value("count:"), value("packs:"))
}

/// The loose objects of `bare`, by id.
fn loose(bare: &Path) -> BTreeSet<String> {
    let mut ids = BTreeSet::new();
    for folder in std::fs::read_dir(bare.join("objects")).expect("the objects' folder") {
        let folder = folder.expect("an entry");
        let name = folder.file_name().to_string_lossy().into_owned();
        if name.len() != 2 || !name.chars().all(|c| c.is_ascii_hexdigit()) {
            continue;
        }
        for file in std::fs::read_dir(folder.path()).expect("a loose folder") {
            let file = file.expect("an entry");
            ids.insert(format!("{name}{}", file.file_name().to_string_lossy()));
        }
    }
    ids
}

/// The packs of `bare`, by file name.
fn packs(bare: &Path) -> BTreeSet<String> {
    std::fs::read_dir(bare.join("objects/pack"))
        .expect("the packs' folder")
        .map(|entry| {
            entry
                .expect("an entry")
                .file_name()
                .to_string_lossy()
                .into_owned()
        })
        .filter(|name| name.ends_with(".pack"))
        .collect()
}

/// Whether `bare` holds the object `id`, loose or packed.
fn holds_object(scratch: &Scratch, bare: &Path, id: &str) -> bool {
    scratch.holds(bare, &["cat-file", "-e", id])
}

/// The value of `key` in `bare`'s own config.
fn own_config(scratch: &Scratch, bare: &Path, key: &str) -> String {
    scratch.git(bare, &["config", "--local", key])
}

/// More loose objects than git's bound, `gc.auto`'s 6,700: enough that,
/// with git's collection on, a commit's `git gc --auto` finds the bound
/// passed by its sample of one folder of `objects/`.
const PAST_THE_BOUND: usize = 10_000;

/// A new clone's own config turns git's collection off.
#[test]
fn a_new_clones_collection_is_off() {
    let scratch = Scratch::new();
    scratch.origin("agentplot/flume");

    let (outcome, _) = scratch.pass(&keeping(&["agentplot/flume"]));

    assert_eq!(outcome, Outcome::Kept);
    let bare = scratch.clone_of("agentplot/flume");
    assert_eq!(own_config(&scratch, &bare, "gc.auto"), "0");
    assert_eq!(own_config(&scratch, &bare, "maintenance.auto"), "false");
}

/// A commit in a worktree of a clone past git's bound packs nothing.
#[test]
fn an_agents_commit_starts_no_collection() {
    let scratch = Scratch::new();
    scratch.origin("agentplot/flume");
    let (outcome, _) = scratch.pass(&keeping(&["agentplot/flume"]));
    assert_eq!(outcome, Outcome::Kept);
    let bare = scratch.clone_of("agentplot/flume");
    write_loose(&scratch, &bare, "arrived", PAST_THE_BOUND);
    let place = scratch.root.join("place");
    scratch.git(
        &bare,
        &[
            "worktree",
            "add",
            "--quiet",
            "-b",
            "work",
            &place.to_string_lossy(),
            "refs/remotes/origin/main",
        ],
    );
    let loose_before = loose(&bare);
    let packs_before = packs(&bare);
    assert!(loose_before.len() > 6_700);

    std::fs::write(place.join("work"), "an agent's work\n").expect("the place's file");
    scratch.git(&place, &["add", "work"]);
    scratch.git(&place, &["commit", "--quiet", "-m", "an agent's commit"]);
    let commit = scratch.git(&place, &["rev-parse", "HEAD"]);

    let loose_after = loose(&bare);
    assert!(loose_after.is_superset(&loose_before));
    assert!(loose_after.contains(&commit));
    assert_eq!(packs(&bare), packs_before);
}

/// A clone made before the runner turned git's collection off is turned
/// off at its next pass, which says nothing of it.
#[test]
fn a_clone_made_before_is_turned_off_at_its_next_pass() {
    let scratch = Scratch::new();
    scratch.origin("agentplot/flume");
    let config = keeping(&["agentplot/flume"]);
    scratch.pass(&config);
    let bare = scratch.clone_of("agentplot/flume");
    scratch.git(&bare, &["config", "--local", "gc.auto", "6700"]);
    scratch.git(&bare, &["config", "--local", "--unset", "maintenance.auto"]);

    let (outcome, lines) = scratch.pass(&config);

    assert_eq!(outcome, Outcome::Kept);
    assert_eq!(lines, [] as [String; 0]);
    assert_eq!(own_config(&scratch, &bare, "gc.auto"), "0");
    assert_eq!(own_config(&scratch, &bare, "maintenance.auto"), "false");
}

/// A clone whose config cannot be written is named and fails the pass,
/// and its repository is still fetched.
#[test]
fn a_clone_whose_config_cannot_be_written() {
    let scratch = Scratch::new();
    let origin = scratch.origin("agentplot/flume");
    let config = keeping(&["agentplot/flume"]);
    scratch.pass(&config);
    let bare = scratch.clone_of("agentplot/flume");
    scratch.git(&bare, &["config", "--local", "gc.auto", "6700"]);
    std::fs::write(bare.join("config.lock"), "").expect("the config's lock");
    let next = origin.commit("main", "next", "a second commit\n");

    let (outcome, lines) = scratch.pass(&config);

    assert_eq!(outcome, Outcome::Failed);
    let prefix = "agentplot/flume: git's own collection not turned off: ";
    assert!(
        lines.iter().any(|line| line
            .strip_prefix(prefix)
            .is_some_and(|words| !words.is_empty())),
        "{lines:?}"
    );
    assert_eq!(
        scratch.git(&bare, &["rev-parse", "refs/remotes/origin/main"]),
        next
    );
}
