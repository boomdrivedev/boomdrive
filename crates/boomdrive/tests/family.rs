//! The set's own Flume and sluice (record 0020, core.45, design Decision
//! 6): the flake's packages of its lock, which the devenv shell gives by
//! their paths as `BOOMDRIVE_SET_FLUME` and `BOOMDRIVE_SET_SLUICE`, run on
//! the scratch host and read with the runner's own readers (the
//! family-packages spec's "The set is the one the runner's tests pass
//! with"). Neither is looked for on `PATH`: where the shell gives no path,
//! the test fails, saying so.

mod common;

use boomdrive::facts::{self, Room, StreamRoom};
use boomdrive::flume::{self, Flume};
use boomdrive::hello::{self, Tools};
use boomdrive::pass::Outcome;
use boomdrive::sluice::{self, Sluice};
use boomdrive::terminal::Relays;
use boomdrive_control::fake::{Fake, Sent};
use boomdrive_control::{
    shared, Answer, At, Capacity, ControlPlane, Id, Launched, Lease, Report, ToolVersion,
};
use common::{Resident, Scratch};
use serde_json::Value;
use std::collections::BTreeMap;
use std::ffi::OsString;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::time::Duration;

/// The two tools of the set the runner runs, each with the variable the
/// devenv shell gives its program in.
const SET: [(&str, &str); 2] = [
    ("flume", "BOOMDRIVE_SET_FLUME"),
    ("sluice", "BOOMDRIVE_SET_SLUICE"),
];

/// The scratch host's Flume configuration, as the 0.4.1 fixtures' README
/// describes it: the printed command as its terminal, a pool of one
/// account, and two bounded streams beside `default`.
const FLUME_CONFIG: &str = "\
[streams.wide]
agents = 3
priority = 5
pool = \"stand-in\"

[streams.work]
agents = 1
pool = \"stand-in\"

[pools.stand-in]
strategy = \"fill-first\"
accounts = [{ name = \"stand-in-1\", program = \"claude\" }]
";

/// The scratch repository's policy: `main`, landed on its `origin`.
const SLUICE_POLICY: &str = "\
[branches.main]
required = [\"#check-fast\"]
remote = \"origin\"
";

/// The set's program of `tool`, by the path the devenv shell gives it.
fn set_program(tool: &str) -> PathBuf {
    let (_, variable) = SET
        .iter()
        .find(|(name, _)| *name == tool)
        .expect("a tool of the set");
    match std::env::var_os(variable) {
        Some(program) if !program.is_empty() => PathBuf::from(program),
        _ => panic!(
            "{variable} is not set: the devenv shell gives the set's {tool} by its path \
             (devenv.nix); run the tests in it, `devenv shell -- moon run boomdrive:test`"
        ),
    }
}

/// `tool`'s tag in flake.lock, without its `v`: the `original.ref` of the
/// node the root's input `tool` names.
fn locked_version(tool: &str) -> String {
    let file = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../flake.lock");
    let lock: Value = serde_json::from_str(&fs::read_to_string(&file).expect("flake.lock"))
        .expect("flake.lock is JSON");
    let root = lock["root"].as_str().expect("the lock's root");
    let node = lock["nodes"][root]["inputs"][tool]
        .as_str()
        .unwrap_or_else(|| panic!("the lock's root has no input {tool}"));
    let tag = lock["nodes"][node]["original"]["ref"]
        .as_str()
        .unwrap_or_else(|| panic!("the lock names no tag of {tool}"));
    tag.strip_prefix('v')
        .unwrap_or_else(|| panic!("{tool}'s tag {tag} is not v<version>"))
        .to_string()
}

/// `text` as one word of `sh`, single-quoted.
fn quoted(text: &OsString) -> String {
    format!("'{}'", text.to_string_lossy().replace('\'', r"'\''"))
}

/// A program at `file` that runs `body`.
fn program(file: &Path, body: &str) {
    fs::write(file, format!("#!/bin/sh\n{body}")).expect("the program");
    fs::set_permissions(file, fs::Permissions::from_mode(0o755)).expect("its mode");
}

/// The scratch host the set runs on. Its `PATH` holds first a `flume`
/// and a `sluice` of the host's own, standing in for the ones a host
/// installs, each of which notes every call and answers as no release
/// would; then the tests' own `PATH`, whose git sluice runs.
struct Family {
    scratch: Scratch,
    /// The stand-ins' folder.
    host_bin: PathBuf,
}

impl Family {
    fn new() -> Family {
        let scratch = Scratch::new();
        let host_bin = scratch.root.join("host-bin");
        fs::create_dir_all(&host_bin).expect("the host's folder");
        for (tool, _) in SET {
            program(
                &host_bin.join(tool),
                &format!(
                    "echo \"$@\" >> \"$(dirname \"$0\")/{tool}.called\"\n\
                     echo '{tool} 0.0.1-host'\n\
                     exit 3\n"
                ),
            );
        }
        fs::create_dir_all(scratch.root.join("tmp")).expect("the scratch TMPDIR");
        let flume_config = scratch.home.join(".config/flume/config.toml");
        fs::create_dir_all(flume_config.parent().expect("its folder")).expect("Flume's folder");
        fs::write(&flume_config, FLUME_CONFIG).expect("Flume's configuration");
        Family { scratch, host_bin }
    }

    /// The scratch environment, and nothing of the tests' own but its
    /// `PATH`, after the stand-ins.
    fn environment(&self) -> Vec<(OsString, OsString)> {
        let home = &self.scratch.home;
        let mut path = OsString::from(&self.host_bin);
        if let Some(own) = std::env::var_os("PATH") {
            path.push(":");
            path.push(own);
        }
        let mut environment = self.scratch.overlay();
        environment.extend([
            ("XDG_STATE_HOME".into(), home.join(".local/state").into()),
            ("XDG_CACHE_HOME".into(), home.join(".cache").into()),
            ("TMPDIR".into(), self.scratch.root.join("tmp").into()),
            // sluice's short directory of the user's, where its host
            // process binds its socket, in the scratch folder: the gate
            // runs these tests under sluice's sandbox, where a program a
            // test starts may write the scratch folder and not
            // `/tmp/sluice-<uid>`, as sluice's own tests name it.
            ("SLUICE_USER_SCRATCH".into(), self.sluice_scratch().into()),
            // Flume's short directory of the user's, where flume host
            // binds its socket, in the scratch folder for the same reason.
            (
                boomdrive::flume_host::USER_SCRATCH.into(),
                self.flume_scratch().into(),
            ),
            ("PATH".into(), path),
        ]);
        environment
    }

    /// The folder `FLUME_USER_SCRATCH` names, where flume host binds.
    fn flume_scratch(&self) -> PathBuf {
        self.scratch.root.join("f")
    }

    /// The set's `flume host` on the scratch host, whose socket Flume's
    /// status asks (the change the-runner-hears-flume-by-subscription).
    fn flume_host(&self) -> Resident {
        let mut command = std::process::Command::new(self.set("flume"));
        command.arg("host").current_dir(&self.scratch.home);
        Resident::start(command, &self.flume_scratch())
    }

    /// The set's `tool` as the runner's readers are given it: a program
    /// in the scratch folder that runs the set's by its path under the
    /// scratch environment alone, since a reader runs its program under
    /// the tests' own.
    fn set(&self, tool: &str) -> PathBuf {
        let assignments: Vec<String> = self
            .environment()
            .iter()
            .map(|(name, value)| format!("{}={}", name.to_string_lossy(), quoted(value)))
            .collect();
        let file = self.scratch.root.join(format!("set-{tool}"));
        program(
            &file,
            &format!(
                "exec env -i {} {} \"$@\"\n",
                assignments.join(" "),
                quoted(&set_program(tool).into_os_string())
            ),
        );
        file
    }

    /// The set's `tool` as [`Family::set`] gives it, noting each exit of
    /// its in the file this gives beside it, one line each.
    fn set_noting_exits(&self, tool: &str) -> (PathBuf, PathBuf) {
        let set = self.set(tool);
        let exits = self.scratch.root.join(format!("{tool}.exits"));
        let file = self.scratch.root.join(format!("noted-{tool}"));
        program(
            &file,
            &format!(
                "{} \"$@\"\ncode=$?\necho \"$code\" >> {}\nexit \"$code\"\n",
                quoted(&set.clone().into_os_string()),
                quoted(&exits.clone().into_os_string())
            ),
        );
        (file, exits)
    }

    /// The folder `SLUICE_USER_SCRATCH` names, where sluice host binds.
    fn sluice_scratch(&self) -> PathBuf {
        self.scratch.root.join("u")
    }

    /// The set's `sluice host` on the scratch host, which sluice's
    /// commands ask, and whose group, the runs it starts among it, ends
    /// with the test.
    fn sluice_host(&self) -> Resident {
        let mut command = std::process::Command::new(self.set("sluice"));
        command.arg("host").current_dir(&self.scratch.home);
        Resident::start(command, &self.sluice_scratch())
    }

    /// The calls made to the stand-in of `tool`, one line each.
    fn host_calls(&self, tool: &str) -> String {
        fs::read_to_string(self.host_bin.join(format!("{tool}.called"))).unwrap_or_default()
    }

    /// Fails where either stand-in was run.
    fn no_host_tool_ran(&self) {
        for (tool, _) in SET {
            assert_eq!(self.host_calls(tool), "", "the host's {tool} ran");
        }
    }
}

/// The family-packages spec's "Each tool's version": each program's first
/// line of `--version`, read as the hello reads it, is its tag's.
#[test]
fn each_tools_version_is_its_tags() {
    let family = Family::new();
    let tools = hello::Cli::on_path()
        .with("flume", family.set("flume"))
        .with("sluice", family.set("sluice"));
    for (tool, _) in SET {
        assert_eq!(
            tools.version(tool),
            ToolVersion::Printed(format!("{tool} {}", locked_version(tool))),
            "the set's {tool} at {}",
            set_program(tool).display()
        );
    }
    family.no_host_tool_ran();
}

/// The family-packages spec's "Flume's status of a scratch host": the
/// set's Flume's `--json status` of a host that holds no place, read by
/// `flume::Cli` and the runner's readers of the listing and the host part.
#[test]
fn flumes_status_of_a_scratch_host() {
    let family = Family::new();
    let _flume_host = family.flume_host();
    let status = flume::Cli::new(family.set("flume")).status();
    let text = match &status {
        Ok(flume::Reply::Given(text)) => text.clone(),
        other => panic!("Flume's status of the scratch host: {other:?}"),
    };
    let listing: Value = serde_json::from_str(&text).expect("Flume's status is JSON");
    assert_eq!(facts::places(&listing), &[] as &[Value], "{text}");

    let Capacity::Printed(part) = hello::capacity(&status) else {
        panic!("no host part in Flume's status: {text}");
    };
    let host = listing.pointer("/status/host").expect("the host part");
    for (field, printed) in [
        ("running", &part.running),
        ("bound", &part.bound),
        ("room", &part.room),
        ("streams", &part.streams),
    ] {
        let read: Value = serde_json::from_str(printed).expect("a field as Flume printed it");
        assert_eq!(&read, &host[field], "the host part's {field}");
        assert!(text.contains(printed.as_str()), "{field} sliced: {printed}");
    }

    assert_eq!(facts::host_room(&listing), Some(Room::Unbounded), "{text}");
    let streams = facts::streams(&listing);
    for (name, room) in [("wide", 3), ("work", 1)] {
        assert!(
            streams.contains(&StreamRoom {
                name: name.to_string(),
                room: Some(Room::Count(room)),
                queue_empty: Some(true),
            }),
            "the stream {name} with room {room}: {streams:?}"
        );
    }
    family.no_host_tool_ran();
}

/// The family-packages spec's "The set's Flume takes the runner as a
/// recipient" (design Decision 8): the set's Flume, asked on a scratch
/// host as the runner asks it, to launch in a folder that holds no
/// repository naming the runner beside another recipient, and to confirm
/// as the runner a key no end is under, refuses each on exit 1 with
/// its reason, which `flume::Cli` reads as Flume's reason, and neither is
/// a usage error; and its `--version` is no older than the least the
/// runner launches with.
#[test]
fn the_sets_flume_takes_the_runner_as_a_recipient() {
    let family = Family::new();
    let set = family.set("flume");
    let cli = flume::Cli::new(&set);
    let nowhere = family.scratch.root.join("no-repository");
    fs::create_dir_all(&nowhere).expect("a folder that holds no repository");
    let launch = flume::LaunchArgs {
        container_dir: nowhere.clone(),
        kind: "change".into(),
        name: "x".into(),
        recipients: vec!["scratch/planner".into()],
        mark: "m".into(),
        ..flume::LaunchArgs::default()
    };
    let briefs = family.scratch.root.join("briefs");
    let launched = cli.launch(&launch, &briefs);
    let key = "fix--none/t/stage/code/1/delivered";
    let confirmed = cli.confirm(flume::RECIPIENT, &[key.to_string()]);

    let launch_argv = [
        "--json",
        "launch",
        "--repository",
        nowhere.to_str().expect("a path of words"),
        "--kind",
        "change",
        "--name",
        "x",
        "--recipient",
        "scratch/planner",
        "--recipient",
        flume::RECIPIENT,
        "--mark",
        "m",
    ];
    let recipient = format!("--recipient={}", flume::RECIPIENT);
    let confirm_argv = ["--json", "confirm", &recipient, "--", key];
    for (reply, argv) in [(launched, &launch_argv[..]), (confirmed, &confirm_argv[..])] {
        let printed = family
            .scratch
            .command(&set)
            .args(argv)
            .output()
            .expect("the set's Flume runs");
        assert_eq!(printed.status.code(), Some(1), "{argv:?}: {printed:?}");
        let answer: Value =
            serde_json::from_slice(&printed.stdout).expect("Flume's refusal is JSON");
        let reason = answer
            .pointer("/refused/refusal/reason")
            .and_then(Value::as_str)
            .unwrap_or_else(|| panic!("{argv:?}: no reason in {answer}"))
            .to_string();
        assert_eq!(reply, Ok(flume::Reply::Refused(reason)), "{argv:?}");
    }

    let tools = hello::Cli::on_path().with("flume", &set);
    let ToolVersion::Printed(said) = tools.version("flume") else {
        panic!("the set's Flume says no version");
    };
    assert_eq!(flume::runs_with(&said), Ok(()), "{said}");
    family.no_host_tool_ran();
}

/// The family-packages spec's "A submission sluice has no record of": the
/// set's sluice, asked in the runner's clone of a scratch repository for
/// an entry it never recorded, refuses with its `not_found` answer, which
/// `sluice::Cli` reads as a refused entry in sluice's own words.
#[test]
fn a_submission_sluice_has_no_record_of() {
    let family = Family::new();
    let scratch = &family.scratch;
    let origin = scratch.origin("example/widgets");
    origin.commit("main", ".config/sluice.toml", SLUICE_POLICY);
    let (outcome, lines) = scratch.pass(&common::keeping(&["example/widgets"]));
    assert_eq!(outcome, Outcome::Kept, "the clone made: {lines:?}");
    let container = scratch.container("example/widgets");

    let set = family.set("sluice");
    let reply = sluice::Cli::new(&set).entry(&container, "99", "main");

    let printed = scratch
        .command(&set)
        .args(["status", "99", "--branch", "main", "--json"])
        .current_dir(&container)
        .output()
        .expect("the set's sluice runs");
    assert_eq!(printed.status.code(), Some(1), "{printed:?}");
    let answer: Value = serde_json::from_slice(&printed.stdout).expect("sluice's answer is JSON");
    assert_eq!(answer["error"]["kind"], "not_found", "{answer}");
    let message = answer["error"]["message"]
        .as_str()
        .expect("sluice's words")
        .to_string();
    assert_eq!(reply, Ok(sluice::Reply::Refused(message)));
    family.no_host_tool_ran();
}

/// The scratch repository's policy for two branches whose submissions
/// are numbered by two counts: `main`'s records on its remote, which the
/// signer `a`, whose public key is `public`, makes it keep there, and
/// `next`'s on the host. So each branch's first submission is 1, as a
/// bolt's branch and `main` came to hold one number (sluice record 0114).
fn two_counts_policy(public: &str) -> String {
    format!(
        "[signers.a]\n\
         ssh = \"a {public}\"\n\
         \n\
         [branches.main]\n\
         required = []\n\
         remote = \"origin\"\n\
         \n\
         [branches.next]\n\
         required = []\n"
    )
}

/// A new ed25519 key written at `file` in OpenSSH's form, which sluice's
/// `[signing] key` reads, readable by its owner alone, and its public key
/// as one line, `ssh-ed25519 <base64>` (core.4: made at run time).
fn signing_key(file: &Path) -> String {
    use base64::engine::general_purpose::STANDARD;
    use base64::Engine as _;
    fn string(out: &mut Vec<u8>, bytes: &[u8]) {
        out.extend_from_slice(
            &u32::try_from(bytes.len())
                .expect("a short field")
                .to_be_bytes(),
        );
        out.extend_from_slice(bytes);
    }
    let mut seed = [0u8; 32];
    std::io::Read::read_exact(
        &mut fs::File::open("/dev/urandom").expect("the host's randomness"),
        &mut seed,
    )
    .expect("32 random bytes");
    let key = ed25519_dalek::SigningKey::from_bytes(&seed);
    let public = key.verifying_key().to_bytes();
    let mut blob = Vec::new();
    string(&mut blob, b"ssh-ed25519");
    string(&mut blob, &public);
    let mut private = Vec::new();
    let check = u32::from_be_bytes([seed[0], seed[1], seed[2], seed[3]]);
    private.extend_from_slice(&check.to_be_bytes());
    private.extend_from_slice(&check.to_be_bytes());
    string(&mut private, b"ssh-ed25519");
    string(&mut private, &public);
    string(&mut private, &[seed.as_slice(), public.as_slice()].concat());
    string(&mut private, b"a");
    let mut pad = 1u8;
    while private.len() % 8 != 0 {
        private.push(pad);
        pad += 1;
    }
    let mut whole = b"openssh-key-v1\0".to_vec();
    string(&mut whole, b"none");
    string(&mut whole, b"none");
    string(&mut whole, b"");
    whole.extend_from_slice(&1u32.to_be_bytes());
    string(&mut whole, &blob);
    string(&mut whole, &private);
    let encoded = STANDARD.encode(&whole);
    let lines: Vec<&str> = encoded
        .as_bytes()
        .chunks(70)
        .map(|line| std::str::from_utf8(line).expect("base64 is ASCII"))
        .collect();
    fs::write(
        file,
        format!(
            "-----BEGIN OPENSSH PRIVATE KEY-----\n{}\n-----END OPENSSH PRIVATE KEY-----\n",
            lines.join("\n")
        ),
    )
    .expect("the signing key");
    fs::set_permissions(file, fs::Permissions::from_mode(0o600)).expect("its mode");
    format!("ssh-ed25519 {}", STANDARD.encode(&blob))
}

/// The family-packages spec's "The set's sluice takes each of the
/// runner's asks": in the runner's clone of a scratch repository whose
/// `main` and `next` each hold a submission 1, the set's sluice answers
/// each ask the runner makes, by the runner's own `sluice::Cli`, with its
/// arguments as the runner gives them: the entry of 1 on each branch, a
/// preview, the previews and a release. Each reads its command line,
/// exiting other than 64, and the entries are each branch's own. An ask
/// that names no branch is refused here, as sluice 0.92.0 began to refuse
/// it (sluice record 0114), so this fails where the runner's arguments
/// fall behind the set's sluice.
#[test]
fn the_sets_sluice_takes_each_of_the_runners_asks() {
    let family = Family::new();
    let scratch = &family.scratch;
    let key = scratch.root.join("signing");
    let public = signing_key(&key);
    let host_config = scratch.home.join(".config/sluice/config.toml");
    fs::create_dir_all(host_config.parent().expect("sluice's folder")).expect("sluice's folder");
    fs::write(
        &host_config,
        format!(
            "[signing]\nsigner = \"a\"\nkey = {:?}\n",
            key.display().to_string()
        ),
    )
    .expect("sluice's host configuration");
    let origin = scratch.origin("example/widgets");
    origin.commit("main", ".config/sluice.toml", &two_counts_policy(&public));
    origin.commit("next", "next", "the second branch\n");
    let (outcome, lines) = scratch.pass(&common::keeping(&["example/widgets"]));
    assert_eq!(outcome, Outcome::Kept, "the clone made: {lines:?}");
    let container = scratch.container("example/widgets");
    let clone = scratch.clone_of("example/widgets");
    let _host = family.sluice_host();
    let set = family.set("sluice");

    // One submission to each branch, each from a worktree of the
    // runner's clone, as a place's is. The worktrees and their commits
    // are made before the first submission, which puts sluice's branch
    // protection into the clone.
    // `next` has no remote, so sluice lands on the clone's own branch.
    scratch.git(&clone, &["branch", "next", "origin/next"]);
    let works = [("main", "a"), ("next", "b")].map(|(branch, work)| {
        let dir = scratch.root.join("places").join(work);
        scratch.git(
            &clone,
            &[
                "worktree",
                "add",
                "--quiet",
                "-b",
                work,
                &dir.to_string_lossy(),
                &format!("origin/{branch}"),
            ],
        );
        fs::write(dir.join(work), "work\n").expect("the work");
        scratch.git(&dir, &["add", "--all"]);
        scratch.git(&dir, &["commit", "--quiet", "-m", work]);
        (branch, dir)
    });
    for (branch, dir) in &works {
        let submitted = scratch
            .command(&set)
            .args(["submit", branch, "--no-wait", "--json"])
            .current_dir(dir)
            .output()
            .expect("the set's sluice runs");
        assert!(submitted.status.success(), "{submitted:?}");
        let answer: Value = serde_json::from_slice(&submitted.stdout).expect("sluice's answer");
        assert_eq!(answer["id"], 1, "{branch}: {answer}");
    }
    let unnamed = scratch
        .command(&set)
        .args(["status", "1", "--json"])
        .current_dir(&container)
        .output()
        .expect("the set's sluice runs");
    assert!(!unnamed.status.success(), "1 is on one branch: {unnamed:?}");

    let (noted, exits) = family.set_noting_exits("sluice");
    let cli = sluice::Cli::new(&noted);
    for branch in ["main", "next"] {
        let carried = match cli.entry(&container, "1", branch) {
            Ok(sluice::Reply::Given(answer)) => sluice::carried(&answer),
            other => {
                panic!("the set's sluice refused the runner's ask of 1 on {branch}: {other:?}")
            }
        };
        let carried = carried.expect("sluice's answer is a submission");
        assert_eq!(
            sluice::branch_of(&carried).as_deref(),
            Some(branch),
            "{carried}"
        );
        assert_eq!(sluice::id_of(&carried).as_deref(), Some("1"), "{carried}");
    }
    let revision = origin.tip("main");
    let previewed = cli.preview(&container, "main", &revision);
    assert!(
        matches!(previewed, Ok(sluice::Asked::Replied(_))),
        "{previewed:?}"
    );
    let listed = cli.previews(&container);
    assert!(
        matches!(&listed, Ok(sluice::Reply::Given(answer)) if sluice::listed(answer).is_some()),
        "{listed:?}"
    );
    let released = cli.release(&container, "preview/none-here");
    assert!(released.is_ok(), "{released:?}");
    let exits = fs::read_to_string(&exits).expect("each exit noted");
    let exits: Vec<&str> = exits.lines().collect();
    assert_eq!(exits.len(), 5, "{exits:?}");
    assert!(
        !exits.contains(&"64"),
        "the set's sluice could not read a command line the runner gives: {exits:?}"
    );
    family.no_host_tool_ran();
}

/// The family-packages spec's "Not the host's tools": where the `PATH`
/// the set runs under finds a `flume` and a `sluice` of the host's, the
/// readers run the set's, by their paths, and never the host's.
#[test]
fn not_the_hosts_tools() {
    let family = Family::new();
    let (_, path) = family
        .environment()
        .into_iter()
        .find(|(name, _)| name == "PATH")
        .expect("the scratch PATH");
    for (tool, _) in SET {
        let host = family
            .scratch
            .command(tool)
            .env("PATH", &path)
            .arg("--version")
            .output()
            .expect("the host's tool runs");
        assert_eq!(
            String::from_utf8_lossy(&host.stdout),
            format!("{tool} 0.0.1-host\n"),
            "the PATH finds the host's {tool}"
        );
        fs::remove_file(family.host_bin.join(format!("{tool}.called"))).expect("its note");
    }

    let tools = hello::Cli::on_path()
        .with("flume", family.set("flume"))
        .with("sluice", family.set("sluice"));
    for (tool, _) in SET {
        assert_eq!(
            tools.version(tool),
            ToolVersion::Printed(format!("{tool} {}", locked_version(tool)))
        );
    }
    let _flume_host = family.flume_host();
    let status = flume::Cli::new(family.set("flume")).status();
    assert!(
        matches!(status, Ok(flume::Reply::Given(_))),
        "the set's Flume answered: {status:?}"
    );
    family.no_host_tool_ran();
}

/// The terminal-relay spec's "The set's Flume refuses a relay of no
/// place" (design Decision 11; core.45): a terminal on a launch the
/// report holds at a place that stands nowhere on the scratch host,
/// relayed through the set's own Flume, is closed with the set's Flume's
/// refusal.
#[test]
fn the_sets_flume_answers_a_relay() {
    let family = Family::new();
    let place = family.scratch.root.join("places/nowhere");
    let launched = Launched {
        lease: Lease {
            number: 1,
            expires: None,
        },
        repository: None,
        answer: Answer {
            outcome: boomdrive_control::Outcome::Launched,
            place_dir: Some(place.to_string_lossy().into_owned()),
            words: None,
            at: At::now(),
        },
        status: None,
        entry: None,
        entry_refused: None,
    };
    let report = Report {
        launches: BTreeMap::from([(Id::new("l-1"), launched)]),
        ..Report::default()
    };
    let mut plane = Fake::holding(Vec::new());
    let far = plane.open_terminal(Id::new("t1"), Id::new("l-1"), 1);
    let pulled = plane
        .pull(&shared::hello(&report), &report)
        .expect("a pull");
    let relays = Relays::new(flume::Cli::new(family.set("flume")), |_| {});
    for terminal in pulled.terminals {
        relays.open(terminal, &report);
    }
    let sent = far
        .sent
        .recv_timeout(Duration::from_secs(30))
        .expect("the runner closes the link");
    assert_eq!(
        sent,
        Sent::Closed(format!("no place {} stands on this host", place.display()))
    );
    drop(relays);
    family.no_host_tool_ran();
}
