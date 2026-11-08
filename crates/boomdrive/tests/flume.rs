//! Flume's command line against a stub `flume` written into each test's
//! own temporary directory (design Decision 21): the argv it is given,
//! its environment, the brief's file, and the reading of each answer by
//! Flume's exit, over Flume 0.4.1's own answers (Task notes 3.1).

use boomdrive::flume::{Act, Cli, Flume, LaunchArgs, Reply};
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use tempfile::TempDir;

/// A fixture of Flume 0.4.1's, as it printed it.
fn fixture(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/flume-0.4.1")
        .join(name)
}

/// The stub's program. Each call writes its arguments, NUL-terminated, to
/// `call-<n>`; whether `FLUME_RECIPIENT` was set to `recipient-<n>`; and
/// the brief its `--brief-file` names to `brief-<n>`. It then prints the
/// file `stdout` names, writes `stderr`'s on standard error, and exits
/// with `exit`'s status, each where the test wrote one.
const STUB: &str = r#"#!/bin/sh
dir=$(dirname "$0")
n=1
while [ -e "$dir/call-$n" ]; do n=$((n + 1)); done
for arg in "$@"; do printf '%s\0' "$arg"; done > "$dir/call-$n"
if [ "${FLUME_RECIPIENT+set}" = set ]; then echo set; else echo unset; fi > "$dir/recipient-$n"
previous=
for arg in "$@"; do
  if [ "$previous" = --brief-file ]; then cp "$arg" "$dir/brief-$n"; fi
  previous=$arg
done
if [ -e "$dir/kill" ]; then kill -9 $$; fi
if [ -e "$dir/stdout" ]; then cat "$(cat "$dir/stdout")"; fi
if [ -e "$dir/stderr" ]; then cat "$(cat "$dir/stderr")" >&2; fi
if [ -e "$dir/exit" ]; then exit "$(cat "$dir/exit")"; fi
exit 0
"#;

/// A stub `flume` in a temporary directory of its own, printing
/// `status.json` and exiting 0 until told otherwise.
struct Stub {
    dir: TempDir,
}

impl Stub {
    fn new() -> Stub {
        let dir = tempfile::tempdir().expect("a temporary directory");
        let program = dir.path().join("flume");
        fs::write(&program, STUB).expect("the stub");
        fs::set_permissions(&program, fs::Permissions::from_mode(0o755)).expect("its mode");
        let stub = Stub { dir };
        stub.answers(Some("status.json"), None, 0);
        stub
    }

    fn cli(&self) -> Cli {
        Cli::new(self.dir.path().join("flume"))
    }

    /// The next calls print the fixture `stdout`, write the fixture
    /// `stderr` on standard error, and exit with `exit`.
    fn answers(&self, stdout: Option<&str>, stderr: Option<&str>, exit: i32) {
        for (name, fixture_name) in [("stdout", stdout), ("stderr", stderr)] {
            let file = self.dir.path().join(name);
            match fixture_name {
                Some(fixture_name) => {
                    fs::write(&file, fixture(fixture_name).to_string_lossy().as_bytes())
                        .expect("the stub's answer")
                }
                None => {
                    let _ = fs::remove_file(&file);
                }
            }
        }
        fs::write(self.dir.path().join("exit"), exit.to_string()).expect("the stub's exit");
    }

    /// A file of the stub's, as it wrote it.
    fn read(&self, name: &str) -> Option<String> {
        fs::read_to_string(self.dir.path().join(name)).ok()
    }

    /// The arguments of every call, in order.
    fn calls(&self) -> Vec<Vec<String>> {
        (1..)
            .map_while(|n| self.read(&format!("call-{n}")))
            .map(|call| call.split_terminator('\0').map(str::to_string).collect())
            .collect()
    }

    /// A folder for briefs, inside the stub's directory and not yet made.
    fn brief_dir(&self) -> PathBuf {
        self.dir.path().join("data/control/briefs")
    }
}

/// A launch with every flag, as design Decision 9 names them.
fn every_flag(container_dir: &Path) -> LaunchArgs {
    LaunchArgs {
        container_dir: container_dir.to_path_buf(),
        kind: "change".into(),
        name: "a".into(),
        base: Some("bolt/x".into()),
        stream: Some("home".into()),
        priority: Some(4),
        intent: Some("home/x".into()),
        recipients: vec!["home/planner".into(), "home/reviewer".into()],
        holds: vec![("code".into(), "the user reads the design".into())],
        hold_submission: true,
        brief: Some("Parse the input.\n\nAs words.".into()),
        agent_settings: vec![("model".into(), "opus".into())],
        mark: "boomdrive:home/12".into(),
    }
}

/// `words` as a call's arguments.
fn argv(words: &[&str]) -> Vec<String> {
    words.iter().map(|word| word.to_string()).collect()
}

#[test]
fn a_launch_with_every_flag() {
    let stub = Stub::new();
    stub.answers(Some("launch.json"), None, 0);
    let container = Path::new("/d/repositories/example/crew");
    let reply = stub
        .cli()
        .launch(&every_flag(container), &stub.brief_dir())
        .expect("flume answers");
    let text = fs::read_to_string(fixture("launch.json")).expect("the fixture");
    assert_eq!(reply, Reply::Given(text));
    let calls = stub.calls();
    assert_eq!(calls.len(), 1, "{calls:?}");
    let brief_file = stub.brief_dir().join("boomdrive%3Ahome%2F12.md");
    assert_eq!(
        calls[0],
        argv(&[
            "--json",
            "launch",
            "--repository",
            "/d/repositories/example/crew",
            "--kind",
            "change",
            "--name",
            "a",
            "--base",
            "bolt/x",
            "--stream",
            "home",
            "--priority",
            "4",
            "--intent",
            "home/x",
            "--recipient",
            "home/planner",
            "--recipient",
            "home/reviewer",
            "--hold",
            "code=the user reads the design",
            "--hold-submission",
            "--brief-file",
            &brief_file.to_string_lossy(),
            "--agent",
            "model=opus",
            "--mark",
            "boomdrive:home/12",
        ])
    );
    assert_eq!(
        stub.read("brief-1").as_deref(),
        Some("Parse the input.\n\nAs words."),
        "the brief, whole, as Flume read it"
    );
    assert!(!brief_file.exists(), "the brief's file is left");
}

#[test]
fn a_launch_naming_no_stream_priority_or_intent() {
    let stub = Stub::new();
    stub.answers(Some("launch.json"), None, 0);
    let args = LaunchArgs {
        container_dir: "/d/repositories/agentplot/flume".into(),
        kind: "fix".into(),
        name: "b".into(),
        mark: "boomdrive:home/13".into(),
        ..LaunchArgs::default()
    };
    stub.cli()
        .launch(&args, &stub.brief_dir())
        .expect("flume answers");
    assert_eq!(
        stub.calls(),
        [argv(&[
            "--json",
            "launch",
            "--repository",
            "/d/repositories/agentplot/flume",
            "--kind",
            "fix",
            "--name",
            "b",
            "--mark",
            "boomdrive:home/13",
        ])]
    );
    assert!(!stub.brief_dir().exists(), "a folder made for no brief");
}

#[test]
fn a_kind_named_by_its_identity_is_passed_whole() {
    let stub = Stub::new();
    stub.answers(Some("launch.json"), None, 0);
    let args = LaunchArgs {
        container_dir: "/d/repositories/example/home-blueprints".into(),
        kind: "crew/crew-planner".into(),
        name: "home".into(),
        stream: Some("home".into()),
        agent_settings: vec![("model".into(), "opus".into())],
        mark: "boomdrive:home/planner/1".into(),
        ..LaunchArgs::default()
    };
    let cli = stub.cli();
    cli.launch(&args, &stub.brief_dir()).expect("flume answers");
    cli.launch(
        &every_flag(Path::new("/d/repositories/example/crew")),
        &stub.brief_dir(),
    )
    .expect("flume answers");
    for act in acts() {
        cli.act("/p", &act).expect("flume answers");
    }
    cli.status().expect("flume answers");
    cli.status_of(Some(Path::new("/d/c")), "/p")
        .expect("flume answers");
    let calls = stub.calls();
    let kind = calls[0]
        .iter()
        .position(|arg| arg == "--kind")
        .expect("--kind");
    assert_eq!(calls[0][kind + 1], "crew/crew-planner");
    assert!(calls[0]
        .windows(2)
        .any(|pair| pair == ["--agent", "model=opus"]));
    for call in &calls {
        assert!(
            !call.iter().any(|arg| arg.starts_with("--kinds")),
            "a kinds file passed: {call:?}"
        );
        assert!(
            !call.iter().any(|arg| arg == "--force"),
            "--force passed: {call:?}"
        );
        assert_eq!(call[0], "--json", "--json is not first: {call:?}");
    }
}

/// One of each act, a retry with every flag.
fn acts() -> Vec<Act> {
    vec![
        Act::Retry {
            stage: Some("code".into()),
            words: Some("use the second option".into()),
            submission: true,
        },
        Act::Retry {
            stage: None,
            words: None,
            submission: false,
        },
        Act::Drop,
        Act::Priority(7),
        Act::Priority(-2),
        Act::Done,
    ]
}

#[test]
fn each_acts_argv() {
    let stub = Stub::new();
    stub.answers(Some("retry.json"), None, 0);
    let cli = stub.cli();
    for act in acts() {
        let reply = cli.act("/places/P", &act).expect("flume answers");
        assert!(matches!(reply, Reply::Given(_)), "{act:?}: {reply:?}");
    }
    assert_eq!(
        stub.calls(),
        [
            argv(&[
                "--json",
                "retry",
                "/places/P",
                "--stage",
                "code",
                "--words",
                "use the second option",
                "--submission",
            ]),
            argv(&["--json", "retry", "/places/P"]),
            argv(&["--json", "drop", "/places/P"]),
            argv(&["--json", "priority", "/places/P", "7"]),
            argv(&["--json", "priority", "/places/P", "-2"]),
            argv(&["--json", "done", "/places/P"]),
        ]
    );
}

#[test]
fn status_and_one_places_status() {
    let stub = Stub::new();
    let cli = stub.cli();
    let listing = fs::read_to_string(fixture("status.json")).expect("the fixture");
    assert_eq!(cli.status(), Ok(Reply::Given(listing)));
    stub.answers(Some("status-answer.json"), None, 0);
    let one = fs::read_to_string(fixture("status-answer.json")).expect("the fixture");
    assert_eq!(
        cli.status_of(Some(Path::new("/d/repositories/a/b")), "/places/P"),
        Ok(Reply::Given(one.clone()))
    );
    // A place whose repository the runner keeps no more, by its location
    // alone (record 0021).
    assert_eq!(cli.status_of(None, "/places/P"), Ok(Reply::Given(one)));
    assert_eq!(
        stub.calls(),
        [
            argv(&["--json", "status"]),
            argv(&[
                "--json",
                "status",
                "--repository",
                "/d/repositories/a/b",
                "/places/P"
            ]),
            argv(&["--json", "status", "/places/P"]),
        ]
    );
}

#[test]
fn flume_runs_with_no_recipient_of_the_runners_own() {
    // The runner's own environment may name one, as a session's does.
    std::env::set_var("FLUME_RECIPIENT", "someone/else");
    let stub = Stub::new();
    stub.answers(Some("launch.json"), None, 0);
    let cli = stub.cli();
    cli.launch(
        &every_flag(Path::new("/d/repositories/example/crew")),
        &stub.brief_dir(),
    )
    .expect("flume answers");
    cli.act("/p", &Act::Done).expect("flume answers");
    cli.status().expect("flume answers");
    cli.status_of(Some(Path::new("/d/c")), "/p")
        .expect("flume answers");
    for n in 1..=4 {
        assert_eq!(
            stub.read(&format!("recipient-{n}")).as_deref(),
            Some("unset\n"),
            "call {n} ran with FLUME_RECIPIENT set"
        );
    }
}

#[test]
fn every_answer_of_exit_zero_is_given_as_printed() {
    let stub = Stub::new();
    let cli = stub.cli();
    for name in [
        "status.json",
        "status-bounded.json",
        "status-construct.json",
        "status-code.json",
        "status-answer.json",
        "status-on-hold.json",
        "status-delivered.json",
        "status-submitted.json",
        "status-landed-kept.json",
        "status-tidied.json",
        "status-live-agent.json",
        "launch.json",
        "priority.json",
        "retry.json",
        "done.json",
        "drop.json",
    ] {
        stub.answers(Some(name), None, 0);
        let printed = fs::read_to_string(fixture(name)).expect("the fixture");
        assert_eq!(cli.status(), Ok(Reply::Given(printed)), "{name}");
    }
}

#[test]
fn exit_one_is_refused_with_flumes_reason() {
    let stub = Stub::new();
    let cli = stub.cli();
    for (name, reason) in [
        (
            "launch-refused.json",
            "the stream nosuch the launch names is not declared by the host's configuration (143)",
        ),
        (
            "retry-refused.json",
            "the agent flume:2867c6a5947d:fix--lexer:code:1 is not quiet; ask again forced to end it",
        ),
    ] {
        stub.answers(Some(name), Some("usage.stderr"), 1);
        assert_eq!(
            cli.act("/p", &Act::Done),
            Ok(Reply::Refused(reason.to_string())),
            "{name}"
        );
    }
    stub.answers(Some("status-gone.json"), None, 1);
    let Ok(Reply::Refused(words)) = cli.status_of(Some(Path::new("/d/c")), "/p") else {
        panic!("the status of a place gone is not refused");
    };
    assert!(words.starts_with("no place /private/tmp/"), "{words}");
    assert!(
        words.ends_with("fix--slow-gate stands on this host"),
        "{words}"
    );
}

#[test]
fn exit_two_is_refused_with_flumes_standard_error() {
    let stub = Stub::new();
    stub.answers(None, Some("usage.stderr"), 2);
    let stderr = fs::read_to_string(fixture("usage.stderr")).expect("the fixture");
    assert!(stderr.contains("unrecognized subcommand"), "{stderr}");
    assert_eq!(stub.cli().act("/p", &Act::Done), Ok(Reply::Refused(stderr)));
}

#[test]
fn an_answer_that_is_not_json_is_refused_with_flumes_standard_error() {
    let stub = Stub::new();
    let cli = stub.cli();
    let stderr = fs::read_to_string(fixture("usage.stderr")).expect("the fixture");
    for exit in [0, 1] {
        stub.answers(Some("README.md"), Some("usage.stderr"), exit);
        assert_eq!(
            cli.status(),
            Ok(Reply::Refused(stderr.clone())),
            "exit {exit}"
        );
    }
    // A refusal Flume's JSON gives no reason for is read as one that is
    // not Flume's refused answer.
    stub.answers(Some("status.json"), Some("usage.stderr"), 1);
    assert_eq!(cli.status(), Ok(Reply::Refused(stderr)));
}

#[test]
fn a_program_that_is_not_there_gives_no_answer() {
    let stub = Stub::new();
    let cli = Cli::new(stub.dir.path().join("nosuch"));
    let failure = cli.status().expect_err("an answer from no program");
    assert!(failure.0.starts_with("flume could not run: "), "{failure}");
    let failure = cli
        .launch(
            &every_flag(Path::new("/d/repositories/example/crew")),
            &stub.brief_dir(),
        )
        .expect_err("an answer from no program");
    assert!(failure.0.starts_with("flume could not run: "), "{failure}");
    assert_eq!(
        fs::read_dir(stub.brief_dir())
            .expect("the briefs' folder")
            .count(),
        0,
        "a brief is left"
    );
}

#[test]
fn a_flume_ended_by_a_signal_gives_no_answer() {
    let stub = Stub::new();
    fs::write(stub.dir.path().join("kill"), "").expect("the stub told to die");
    let failure = stub
        .cli()
        .status()
        .expect_err("an answer from a killed flume");
    assert!(failure.0.starts_with("flume was ended: "), "{failure}");
}

#[test]
fn a_refused_launch_leaves_no_brief() {
    let stub = Stub::new();
    stub.answers(Some("launch-refused.json"), None, 1);
    let reply = stub
        .cli()
        .launch(
            &every_flag(Path::new("/d/repositories/example/crew")),
            &stub.brief_dir(),
        )
        .expect("flume answers");
    assert!(matches!(reply, Reply::Refused(_)), "{reply:?}");
    assert_eq!(
        stub.read("brief-1").as_deref(),
        Some("Parse the input.\n\nAs words.")
    );
    assert_eq!(
        fs::read_dir(stub.brief_dir())
            .expect("the briefs' folder")
            .count(),
        0,
        "a brief is left"
    );
}
