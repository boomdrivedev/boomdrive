//! An inbox the control plane names, read at each new tip of its branch
//! in the clone and carried as references, over a scratch forge whose
//! branch holds an `inbox/` written by weir-signals' own write (design
//! Decision 14; core.5, core.11; the tool-records spec's "An inbox is
//! read at each new tip of its branch", "An inbox's records are carried
//! as references", "An inbox weir refuses keeps what was carried" and "An
//! inbox is carried only from a repository the runner keeps"; Task notes
//! 5.2).

mod common;
mod fakes;

use boomdrive::config::Address;
use boomdrive::git::{At, Git};
use boomdrive::{clone, report};
use boomdrive_control::fake::Fake;
use boomdrive_control::{Id, Inbox, InboxCarried, InboxFile, Want};
use common::{Origin, Scratch};
use fakes::Bench;
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use weir_capture::{CameIn, Capture, Key, RawFile, Taken};
use weir_signals::{InboxAt, ReadSignal, Write};

/// The repository whose branch holds the inbox.
const CREW: &str = "afterthought/crew";

const AT: &str = "2026-10-25T09:00:00Z";

/// The capture's key.
const KEY: &str = "release/mise/2026.10.10";

/// The brief of the item, after the line weir writes naming its format.
const BRIEF: &str = "Raise the shim's timeout to a minute.\n";

/// The inbox `inbox/1` on the branch `inbox` of `repository`.
fn named(repository: &str) -> Want {
    Want::Inbox(Inbox {
        id: Id::new("inbox/1"),
        repository: repository.to_string(),
        branch: "inbox".to_string(),
    })
}

/// `inbox` after `write`, which weir takes.
fn after(inbox: &InboxAt, write: Result<Write, weir_signals::Refused>) -> InboxAt {
    let write = write.unwrap_or_else(|refused| panic!("{refused}"));
    inbox
        .check(&write)
        .unwrap_or_else(|refused| panic!("{refused}"))
}

/// An inbox written by weir's own write: a capture, read into one signal,
/// and an item with a brief.
fn weirs_inbox() -> InboxAt {
    let inbox = InboxAt::read(Vec::<(String, String)>::new()).unwrap();
    let key = Key::parse(KEY).unwrap();
    let capture = Capture {
        key: key.clone(),
        source: "release".into(),
        event_at: "2026-10-10T14:02:11Z".into(),
        title: Some("mise 2026.10.10".into()),
        url: None,
        pointer: Some(format!("deneb:/data/inbox/{KEY}")),
        came_in: CameIn::Run("releases".into()),
        machine: "deneb".into(),
        taken_at: "2026-10-17T04:30:05Z".into(),
        unread: true,
    };
    let taken = Taken {
        capture,
        own_signal: None,
    };
    let inbox = after(&inbox, inbox.take(&taken));
    let line = "The shim times out after ten seconds.";
    let raw = RawFile {
        path: "notes.md".into(),
        bytes: line.as_bytes().to_vec(),
    };
    let signal = ReadSignal {
        kind: "ask".into(),
        asserted_by: "upstream".into(),
        assertion: "the shim's timeout is too short".into(),
        subjects: Vec::new(),
        argues_with: Vec::new(),
        excerpt: line.into(),
        position: "the notes".into(),
    };
    let inbox = after(&inbox, inbox.read_capture(&key, &[signal], &[raw]));
    let inbox = after(
        &inbox,
        inbox.propose_item("shim-timeout", "inquiry/toolchain#3", "operator", AT),
    );
    after(&inbox, inbox.write_brief("shim-timeout", BRIEF))
}

/// A bench keeping `afterthought/crew`, whose forge `origin` on `scratch`
/// holds the inbox `files` committed on its `inbox` branch, its clone
/// made by the runner's own `clone::make`.
struct Host<'s> {
    bench: Bench,
    container: PathBuf,
    scratch: &'s Scratch,
    origin: Origin<'s>,
}

impl<'s> Host<'s> {
    fn new(scratch: &'s Scratch, files: &BTreeMap<String, String>) -> Host<'s> {
        let origin = scratch.origin(CREW);
        for (path, text) in files {
            origin.commit("inbox", path, text);
        }
        let git = scratch.runner_git();
        let container = clone::make(&git, &scratch.data(), &Address::parse(CREW).unwrap())
            .expect("the clone made");
        let mut bench = Bench::new();
        bench.git = git;
        bench.clones.ready(CREW, &container);
        Host {
            bench,
            container,
            scratch,
            origin,
        }
    }

    /// Writes `text` at `file` on the forge's `inbox` branch, fetches the
    /// clone as the keeper does, and gives the new tip.
    fn commit(&self, file: &str, text: &str) -> String {
        let tip = self.origin.commit("inbox", file, text);
        clone::fetch(&self.bench.git, &clone::bare(&self.container)).expect("the fetch");
        tip
    }

    /// The inbox the report file carries.
    fn carried(&self) -> InboxCarried {
        self.bench.report().inboxes[&Id::new("inbox/1")].clone()
    }

    /// git as the runner runs it, each call's arguments logged to the
    /// file given.
    fn logging_git(&self) -> (Git, PathBuf) {
        let found = self
            .scratch
            .command("sh")
            .args(["-c", "command -v git"])
            .output()
            .expect("sh runs");
        let real = String::from_utf8(found.stdout).unwrap().trim().to_string();
        let dir = self.scratch.root.join("logging");
        fs::create_dir_all(&dir).unwrap();
        let log = dir.join("calls");
        let program = dir.join("git");
        fs::write(
            &program,
            format!(
                "#!/bin/sh\nprintf '%s\\n' \"$*\" >> '{}'\nexec '{real}' \"$@\"\n",
                log.display()
            ),
        )
        .unwrap();
        fs::set_permissions(&program, fs::Permissions::from_mode(0o755)).unwrap();
        let mut overlay = self.scratch.overlay();
        let path = std::env::var("PATH").unwrap_or_default();
        overlay.push(("PATH".into(), format!("{}:{path}", dir.display()).into()));
        (Git::new(overlay), log)
    }
}

/// The files the carried inbox should hold for `inbox`: every record file
/// with its text, and each brief by its digest.
fn expected(inbox: &InboxAt) -> BTreeMap<String, InboxFile> {
    inbox
        .files()
        .iter()
        .map(|(path, text)| {
            let file = if path.ends_with("/brief.md") {
                InboxFile::Digest(format!("sha256:{:x}", Sha256::digest(text.as_bytes())))
            } else {
                InboxFile::Text(text.clone())
            };
            (path.clone(), file)
        })
        .collect()
}

/// The tool-records spec's "An inbox carried".
#[test]
fn an_inbox_carried() {
    let inbox = weirs_inbox();
    let scratch = Scratch::new();
    let host = Host::new(&scratch, inbox.files());
    let mut plane = Fake::holding(vec![named(CREW)]);
    let (passed, lines) = host.bench.pass(&mut plane);
    assert!(passed.reported, "{lines:?}");
    let tip = host.origin.tip("inbox");
    assert_eq!(
        lines,
        [format!(
            "inbox/1: the inbox on inbox of afterthought/crew carried at {tip}"
        )]
    );
    let carried = host.carried();
    assert_eq!(carried.tip.as_deref(), Some(tip.as_str()));
    assert_eq!(carried.words, None);
    assert_eq!(carried.files, expected(&inbox));
    let paths: Vec<&str> = carried.files.keys().map(String::as_str).collect();
    assert_eq!(
        paths,
        [
            "inbox/captures/release/mise/2026.10.10/capture.rec",
            "inbox/captures/release/mise/2026.10.10/signals.rec",
            "inbox/items/shim-timeout/brief.md",
            "inbox/items/shim-timeout/item.rec",
        ]
    );
    let brief = &carried.files["inbox/items/shim-timeout/brief.md"];
    let InboxFile::Digest(digest) = brief else {
        panic!("the brief is carried as {brief:?}");
    };
    let written = &inbox.files()["inbox/items/shim-timeout/brief.md"];
    assert!(written.ends_with(BRIEF), "{written}");
    assert_eq!(
        *digest,
        format!("sha256:{:x}", Sha256::digest(written.as_bytes()))
    );
    let file = fs::read_to_string(host.bench.report_file()).unwrap();
    assert!(
        !file.contains("shim's timeout to a minute"),
        "a brief's text is kept"
    );
    assert!(
        !file.contains("a scratch repository"),
        "a file outside inbox/ is kept"
    );
    assert_eq!(
        plane.reports().last().unwrap().inboxes[&Id::new("inbox/1")],
        carried
    );
}

/// The tool-records spec's "An unchanged tip".
#[test]
fn an_unchanged_tip() {
    let scratch = Scratch::new();
    let mut host = Host::new(&scratch, weirs_inbox().files());
    let mut plane = Fake::holding(vec![named(CREW)]);
    host.bench.pass(&mut plane);
    let held = fs::read(host.bench.report_file()).unwrap();
    let (git, log) = host.logging_git();
    host.bench.git = git;
    let (passed, lines) = host.bench.pass(&mut plane);
    assert!(passed.reported);
    assert_eq!(lines, [] as [String; 0]);
    assert_eq!(fs::read(host.bench.report_file()).unwrap(), held);
    let calls = fs::read_to_string(log).expect("git was run");
    for call in calls.lines() {
        assert!(
            call.starts_with("--git-dir ")
                && !call.contains(" ls-tree ")
                && !call.contains(" cat-file "),
            "a file is read again: {call}"
        );
    }
}

/// An inbox's tip that moves is read again, and carried anew.
#[test]
fn a_new_tip_is_read() {
    let inbox = weirs_inbox();
    let scratch = Scratch::new();
    let host = Host::new(&scratch, inbox.files());
    let mut plane = Fake::holding(vec![named(CREW)]);
    host.bench.pass(&mut plane);
    let first = host.carried();
    let more = after(
        &inbox,
        inbox.propose_item("pin-mise", "inquiry/toolchain#4", "operator", AT),
    );
    let path = "inbox/items/pin-mise/item.rec";
    let tip = host.commit(path, &more.files()[path]);
    host.bench.pass(&mut plane);
    let carried = host.carried();
    assert_eq!(carried.tip.as_deref(), Some(tip.as_str()));
    assert_eq!(carried.files, expected(&more));
    assert!(carried.at >= first.at);
    assert_ne!(carried.tip, first.tip);
}

/// The tool-records spec's "An inbox weir refuses".
#[test]
fn an_inbox_weir_refuses() {
    let inbox = weirs_inbox();
    let scratch = Scratch::new();
    let host = Host::new(&scratch, inbox.files());
    let mut plane = Fake::holding(vec![named(CREW)]);
    host.bench.pass(&mut plane);
    let before = host.carried();
    let path = "inbox/captures/release/mise/2026.10.11/capture.rec";
    let tip = host.commit(path, "no record of weir's\n");
    let mut files = inbox.files().clone();
    files.insert(path.to_string(), "no record of weir's\n".to_string());
    let words = InboxAt::read(&files).expect_err("weir refuses").to_string();
    let (_, lines) = host.bench.pass(&mut plane);
    assert_eq!(
        lines,
        [format!(
            "inbox/1: the inbox on inbox of afterthought/crew at {tip} is refused by weir: {words}"
        )]
    );
    let carried = host.carried();
    assert_eq!(carried.tip.as_deref(), Some(tip.as_str()));
    assert_eq!(carried.words.as_deref(), Some(words.as_str()));
    assert_eq!(carried.files, before.files, "what was carried stands");
    let (_, lines) = host.bench.pass(&mut plane);
    assert_eq!(lines, [] as [String; 0], "the refused tip is read again");
}

/// The tool-records spec's "A repository not kept": no git command reads
/// it, since git is not there to run.
#[test]
fn a_repository_not_kept() {
    let mut bench = Bench::new();
    let nowhere = bench.dir.path().join("no-git");
    fs::create_dir_all(&nowhere).unwrap();
    bench.git = Git::new(vec![("PATH".into(), nowhere.into())]);
    let here = bench.dir.path();
    assert!(
        bench.git.run(At::Dir(here), ["--version"]).is_err(),
        "git runs"
    );
    let mut plane = Fake::holding(vec![named("agentplot/nosuch")]);
    let (passed, lines) = bench.pass(&mut plane);
    assert!(passed.reported, "{lines:?}");
    let words = "this host's configuration does not keep agentplot/nosuch";
    assert_eq!(lines, [format!("inbox/1: not carried: {words}")]);
    let carried = bench.report().inboxes[&Id::new("inbox/1")].clone();
    assert_eq!(carried.words.as_deref(), Some(words));
    assert_eq!((carried.tip, carried.files), (None, BTreeMap::new()));
    let (_, lines) = bench.pass(&mut plane);
    assert_eq!(lines, [] as [String; 0]);
}

/// The tool-records spec's "No such branch".
#[test]
fn no_such_branch() {
    let scratch = Scratch::new();
    let host = Host::new(&scratch, &BTreeMap::new());
    let mut plane = Fake::holding(vec![named(CREW)]);
    let (passed, lines) = host.bench.pass(&mut plane);
    assert!(passed.reported, "{lines:?}");
    let words = "origin holds no branch inbox of afterthought/crew";
    assert_eq!(lines, [format!("inbox/1: not carried: {words}")]);
    let carried = host.carried();
    assert_eq!(carried.words.as_deref(), Some(words));
    assert_eq!((carried.tip, carried.files), (None, BTreeMap::new()));
}

/// An inbox whose clone is not yet made waits, answered nothing.
#[test]
fn a_clone_not_yet_made() {
    let bench = Bench::new();
    bench.clones.not_yet(CREW);
    let mut plane = Fake::holding(vec![named(CREW)]);
    let (_, lines) = bench.pass(&mut plane);
    assert_eq!(lines, [] as [String; 0]);
    let report = report::read(&bench.report_file()).unwrap();
    assert!(report.unwrap_or_default().inboxes.is_empty());
}
