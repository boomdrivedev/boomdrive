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
use boomdrive_control::{Capacity, ToolVersion};
use common::Scratch;
use serde_json::Value;
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
            ("PATH".into(), path),
        ]);
        environment
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

/// The family-packages spec's "The set's Flume keeps the runner's watch":
/// the set's Flume, asked `--json settle --every 60s` on a scratch host
/// that holds no place as the runner asks it (record 0029), gives its
/// settled answer, read as given, and its `--json status` then gives the
/// host's watch with a bound of 60 seconds.
#[test]
fn the_sets_flume_keeps_the_runners_watch() {
    let family = Family::new();
    let cli = flume::Cli::new(family.set("flume"));
    let settle = cli.settle_every(Duration::from_secs(60));
    let Ok(flume::Reply::Given(text)) = &settle else {
        panic!("the set's Flume's settle of the scratch host: {settle:?}");
    };
    let settled: Value = serde_json::from_str(text).expect("the settle's answer is JSON");
    assert_eq!(settled["settled"]["places"], Value::Array(vec![]), "{text}");

    let status = cli.status();
    let Ok(flume::Reply::Given(text)) = &status else {
        panic!("the set's Flume's status after the settle: {status:?}");
    };
    let listing: Value = serde_json::from_str(text).expect("Flume's status is JSON");
    assert_eq!(
        listing.pointer("/status/host/watch"),
        Some(&serde_json::json!({"bound": {"seconds": 60}})),
        "{text}"
    );
    family.no_host_tool_ran();
}

/// The family-packages spec's "The set's Flume takes the runner as a
/// recipient" (design Decision 6): the set's Flume, asked on a scratch
/// host as the runner asks it, to launch in a folder that holds no
/// repository naming the runner beside another recipient, and to collect
/// for the runner a place it does not hold, refuses each on exit 1 with
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
    let collected = cli.collect("/no/such/place@1");

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
    let inbox_argv = [
        "--json",
        "inbox",
        "/no/such/place@1",
        "--recipient",
        flume::RECIPIENT,
    ];
    for (reply, argv) in [(launched, &launch_argv[..]), (collected, &inbox_argv[..])] {
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
    let reply = sluice::Cli::new(&set).entry(&container, "99");

    let printed = scratch
        .command(&set)
        .args(["status", "99", "--json"])
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
    let status = flume::Cli::new(family.set("flume")).status();
    assert!(
        matches!(status, Ok(flume::Reply::Given(_))),
        "the set's Flume answered: {status:?}"
    );
    family.no_host_tool_ran();
}
