//! The runner alone packs and prunes the clones it keeps: git's own
//! collection is off in each, and a keeping pass packs a kept clone once
//! what arrived passes git's own bound, dropping only what nothing of the
//! clone reaches past two weeks and touching no ref, log or worktree (the
//! clones spec; record 0051; core.14).

mod common;

use boomdrive::config::{Address, Config};
use boomdrive::forge::Forge;
use boomdrive::locks::Locks;
use boomdrive::pass::Outcome;
use common::{keeping, Scratch};
use std::cell::RefCell;
use std::collections::BTreeSet;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Stdio;
use std::sync::atomic::{AtomicBool, Ordering};
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

/// Every object `bare` holds, loose or packed, by id.
fn all_objects(scratch: &Scratch, bare: &Path) -> BTreeSet<String> {
    scratch
        .git(
            bare,
            &[
                "cat-file",
                "--batch-all-objects",
                "--batch-check=%(objectname)",
            ],
        )
        .lines()
        .map(str::to_string)
        .collect()
}

/// Whether a line of `lines` says `address` was packed.
fn says_packed(lines: &[String], address: &str) -> bool {
    let packed = format!("{address}: packed ");
    lines.iter().any(|line| line.starts_with(&packed))
}

/// The index of the first line of `lines` that starts with `start`.
fn first(lines: &[String], start: &str) -> usize {
    lines
        .iter()
        .position(|line| line.starts_with(start))
        .unwrap_or_else(|| panic!("no line starts with {start:?}: {lines:?}"))
}

/// The configuration `text`, read as the default file.
fn config_of(scratch: &Scratch, text: &str) -> Config {
    boomdrive::config::parse(text, &scratch.config_file(), &scratch.home).expect("a configuration")
}

/// The value of `key` in `bare`'s own config.
fn own_config(scratch: &Scratch, bare: &Path, key: &str) -> String {
    scratch.git(bare, &["config", "--local", key])
}

/// More loose objects than git's bound, as a scratch clone held when a
/// commit in a worktree of it packed them all (record 0051).
const PAST: usize = 7_003;

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

/// A clone at the pack bound, 50 packs, is not packed.
#[test]
fn nothing_is_packed_at_the_bound() {
    let scratch = Scratch::new();
    scratch.origin("agentplot/flume");
    let config = keeping(&["agentplot/flume"]);
    scratch.pass(&config);
    let bare = scratch.clone_of("agentplot/flume");
    write_packs(&scratch, &bare, "pack", 50);
    let (loose_count, pack_count) = counts(&scratch, &bare);
    assert_eq!(pack_count, 50);
    assert!(loose_count < 6_700);
    let loose_before = loose(&bare);
    let packs_before = packs(&bare);

    let (outcome, lines) = scratch.pass(&config);

    assert_eq!(outcome, Outcome::Kept);
    assert_eq!(lines, [] as [String; 0]);
    assert_eq!(loose(&bare), loose_before);
    assert_eq!(packs(&bare), packs_before);
}

/// A clone past the bound of loose objects is packed whole, and keeps
/// every object it held.
#[test]
fn loose_objects_past_the_bound() {
    let scratch = Scratch::new();
    scratch.origin("agentplot/flume");
    let config = keeping(&["agentplot/flume"]);
    scratch.pass(&config);
    let bare = scratch.clone_of("agentplot/flume");
    write_packs(&scratch, &bare, "pack", 2);
    write_loose(&scratch, &bare, "arrived", PAST);
    let (loose_count, pack_count) = counts(&scratch, &bare);
    assert!(loose_count > 6_700);
    let held = all_objects(&scratch, &bare);

    let (outcome, lines) = scratch.pass(&config);

    assert_eq!(outcome, Outcome::Kept);
    assert_eq!(
        lines,
        [format!(
            "agentplot/flume: packed {loose_count} loose objects and {pack_count} packs"
        )]
    );
    let (loose_after, packs_after) = counts(&scratch, &bare);
    assert_eq!(loose_after, 0);
    assert!(packs_after <= 2, "{packs_after} packs");
    assert!(bare.join("objects/info/commit-graph").is_file());
    assert!(all_objects(&scratch, &bare).is_superset(&held));
}

/// A clone past the bound of packs is packed whole.
#[test]
fn packs_past_the_bound() {
    let scratch = Scratch::new();
    scratch.origin("agentplot/flume");
    let config = keeping(&["agentplot/flume"]);
    scratch.pass(&config);
    let bare = scratch.clone_of("agentplot/flume");
    write_packs(&scratch, &bare, "pack", 51);
    let (loose_count, pack_count) = counts(&scratch, &bare);
    assert_eq!(pack_count, 51);
    assert!(loose_count < 6_700);

    let (outcome, lines) = scratch.pass(&config);

    assert_eq!(outcome, Outcome::Kept);
    assert_eq!(
        lines,
        [format!(
            "agentplot/flume: packed {loose_count} loose objects and 51 packs"
        )]
    );
    let (loose_after, packs_after) = counts(&scratch, &bare);
    assert_eq!(loose_after, 0);
    assert!(packs_after <= 2, "{packs_after} packs");
}

/// A dropped repository's clone, held by a branch origin lacks, is not
/// packed.
#[test]
fn a_dropped_repository_is_not_packed() {
    let scratch = Scratch::new();
    scratch.origin("agentplot/flume");
    scratch.origin("agentplot/sluice");
    scratch.pass(&keeping(&["agentplot/flume", "agentplot/sluice"]));
    let bare = scratch.clone_of("agentplot/flume");
    let tree = scratch.git(&bare, &["rev-parse", "refs/remotes/origin/main^{tree}"]);
    let wip = scratch.git(
        &bare,
        &[
            "commit-tree",
            &tree,
            "-p",
            "refs/remotes/origin/main",
            "-m",
            "work in progress",
        ],
    );
    scratch.git(&bare, &["update-ref", "refs/heads/wip", &wip]);
    write_loose(&scratch, &bare, "arrived", PAST);
    let loose_before = loose(&bare);
    let packs_before = packs(&bare);

    let (_, lines) = scratch.pass(&keeping(&["agentplot/sluice"]));

    assert!(!says_packed(&lines, "agentplot/flume"), "{lines:?}");
    assert!(
        lines
            .iter()
            .any(|line| line.starts_with("agentplot/flume: left: held by")),
        "{lines:?}"
    );
    assert_eq!(loose(&bare), loose_before);
    assert_eq!(packs(&bare), packs_before);
}

/// A landing told while the pass packs is kept between two packs.
#[test]
fn a_landing_told_while_the_pass_packs() {
    let scratch = Scratch::new();
    scratch.origin("agentplot/flume");
    scratch.origin("agentplot/sluice");
    let timber = scratch.origin("agentplot/timber");
    let text = keeping(&["agentplot/flume", "agentplot/sluice", "agentplot/timber"]);
    scratch.pass(&text);
    for address in ["agentplot/flume", "agentplot/sluice"] {
        write_packs(&scratch, &scratch.clone_of(address), "pack", 51);
    }
    let config = config_of(&scratch, &text);
    let git = scratch.runner_git();
    let forge = Forge::new();
    let locks = Locks::new();
    let timber_address = Address::parse("agentplot/timber").expect("an address");
    let lines = RefCell::new(Vec::new());
    let mut landed = None;

    let outcome = boomdrive::pass::pass(
        &git,
        &forge,
        &scratch.data(),
        &config,
        &locks,
        &AtomicBool::new(false),
        &mut || {
            if landed.is_some() || !says_packed(&lines.borrow(), "agentplot/flume") {
                return false;
            }
            landed = Some(timber.commit("main", "landed", "a landing\n"));
            boomdrive::pass::keep_one(
                &git,
                &forge,
                &scratch.data(),
                &config,
                &locks,
                &timber_address,
                &mut |line| lines.borrow_mut().push(line),
            );
            true
        },
        &mut |line| lines.borrow_mut().push(line),
    );

    assert_eq!(outcome, Outcome::Kept);
    let lines = lines.into_inner();
    let landed = landed.expect("the landing told");
    let flume = first(&lines, "agentplot/flume: packed ");
    let moved = first(
        &lines,
        &format!(
            "agentplot/timber: view moved to origin/main at {}",
            &landed[..7]
        ),
    );
    let sluice = first(&lines, "agentplot/sluice: packed ");
    assert!(flume < moved && moved < sluice, "{lines:?}");
}

/// A stop told while the pass packs one clone packs no other.
#[test]
fn a_stop_while_the_pass_packs() {
    let scratch = Scratch::new();
    scratch.origin("agentplot/flume");
    scratch.origin("agentplot/sluice");
    let text = keeping(&["agentplot/flume", "agentplot/sluice"]);
    scratch.pass(&text);
    for address in ["agentplot/flume", "agentplot/sluice"] {
        write_packs(&scratch, &scratch.clone_of(address), "pack", 51);
    }
    let sluice = scratch.clone_of("agentplot/sluice");
    let loose_before = loose(&sluice);
    let packs_before = packs(&sluice);
    let config = config_of(&scratch, &text);
    let stop = AtomicBool::new(false);
    let lines = RefCell::new(Vec::new());

    boomdrive::pass::pass(
        &scratch.runner_git(),
        &Forge::new(),
        &scratch.data(),
        &config,
        &Locks::new(),
        &stop,
        &mut || {
            if says_packed(&lines.borrow(), "agentplot/flume") {
                stop.store(true, Ordering::SeqCst);
            }
            false
        },
        &mut |line| lines.borrow_mut().push(line),
    );

    let lines = lines.into_inner();
    assert!(says_packed(&lines, "agentplot/flume"), "{lines:?}");
    assert!(!says_packed(&lines, "agentplot/sluice"), "{lines:?}");
    assert_eq!(loose(&sluice), loose_before);
    assert_eq!(packs(&sluice), packs_before);
}
